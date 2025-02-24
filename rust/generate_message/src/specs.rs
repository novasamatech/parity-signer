//! `add_specs` payloads and specs related hot database updates
//!
//! This module deals with processing command
//!
//! `$ cargo run add-specs <keys> <argument(s)>`
use definitions::{crypto::Encryption, keyring::NetworkSpecsKey, metadata::AddressBookEntry};
use std::path::Path;

use crate::error::{Error, Result};
use crate::helpers::{
    add_specs_print, address_book_content, db_upd_network, filter_address_book_by_url,
    genesis_hash_in_hot_db, get_address_book_entry, network_specs_from_entry,
    network_specs_from_title, specs_agnostic, try_get_network_specs_to_send, update_known_specs,
    update_modify_encryption_specs,
};
use crate::parser::{Content, InstructionSpecs, Override, Set, Token};

/// Process `add-specs` command according to the [`InstructionSpecs`] received
/// from the command line.
pub fn gen_add_specs(instruction: InstructionSpecs) -> Result<()> {
    match instruction.set.into() {
        // `-f` setting key: produce `add_specs` payload files from existing
        // database entries.
        Set::F => match instruction.content.clone().into() {
            // `$ cargo run add-specs -f -a`
            //
            // Make `add_specs` payloads for all specs entries in the database.
            Content::All { pass_errors: _ } => {
                // makes no sense to override encryption, or token, or title
                // for all entries at once
                if !instruction.over.all_empty() {
                    return Err(Error::NotSupported);
                }

                // collect `ADDRESS_BOOK` entries
                let database = sled::open(instruction.db)?;
                let address_book_set = address_book_content(&database)?;
                if address_book_set.is_empty() {
                    return Err(Error::AddressBookEmpty);
                }

                // process each entry
                for (_, address_book_entry) in address_book_set.iter() {
                    specs_f_a_element(&database, address_book_entry, &instruction.files_dir)?;
                }
                Ok(())
            }

            // `$ cargo run add-specs -f -n <address_book_title>
            // <optional encryption override> <optional signer title override>`
            //
            // Make `add_specs` payload for single specs entry from the
            // database, referred to by network address book title.
            //
            // Entry with encryption override and/or signer title override
            // **will not** be added to the database.
            Content::Name { s: name } => {
                // no fetch is done, there is no way to check the override is
                // allowed
                if instruction.over.token().is_some() {
                    return Err(Error::NotSupported);
                }
                let database = sled::open(instruction.db)?;
                specs_f_n(
                    &database,
                    &name,
                    instruction.over.encryption,
                    instruction.over.title,
                    instruction.files_dir,
                )
            }

            // `-u` content key is to provide the URL address for RPC calls;
            // since `-f` indicates the data is taken from the database, the
            // the combination seems of no use.
            // To address a specific network from the database, `-f -n` key
            // combination is suggested.
            Content::Address { .. } => Err(Error::NotSupported),
        },

        // `-d` setting key: produce `add_specs` payloads through RPC calls,
        // **do not** interact with the database, export payload files.
        Set::D => match instruction.content.clone().into() {
            // `-d` does not write in the database, so fetching specs for known
            // networks without checking the database seems of no use.
            Content::All { pass_errors: _ } => Err(Error::NotSupported),

            // Same as `-d -a` combination, of no use.
            Content::Name { .. } => Err(Error::NotSupported),

            // `$ cargo run add-specs -d -u network_url_address
            // <encryption override> <optional token override> <optional signer
            // title override>`
            //
            // Produce `add_specs` payload by making RPC calls at
            // `network_url_address` and print payload file.
            //
            // Database does not get checked here.
            //
            // Command line **must** contain encryption override.
            //
            // Command may contain signer title override to set up the network
            // title that is displayed in Vault.
            //
            // In some cases the command may contain token override as well.
            Content::Address { s: address } => {
                // not allowed to proceed without encryption override defined
                if let Some(encryption) = instruction.over.encryption {
                    specs_d_u(
                        &address,
                        encryption,
                        instruction.over.token(),
                        instruction.over.title.clone(),
                        &instruction.files_dir,
                    )
                } else {
                    Err(Error::NotSupported)
                }
            }
        },

        // `-k` setting key: produce payloads through RPC calls, update the
        // database, export payload files only for updated information.
        //
        // Since network specs are expected to remain constant over time,
        // threse commands seem to be of no use.
        Set::K => Err(Error::NotSupported),

        // `-p` setting key: update the database
        Set::P => match instruction.content.clone().into() {
            // Network specs are expected to remain constant over time, mass
            // override should not be possible, this command seems to be of no
            // use.
            Content::All { pass_errors: _ } => Err(Error::NotSupported),

            // `$ cargo run add-specs -p -n network_address_book_title
            // <encryption override> <optional title override>
            // <optional token override>`
            //
            // Network already has an entry in the database and could be
            // referred to by network address book title. This key combination
            // is intended to be used to:
            // - add to the hot database same network with different encryption
            // - change token (if possible for given network) or Vault
            // displayed network title
            //
            // Payload files are not created.
            Content::Name { s: name } => {
                let database = sled::open(instruction.db)?;
                // using this command makes sense only if there is some override
                specs_pt_n(
                    &database,
                    &name,
                    instruction.over,
                    false,
                    &instruction.files_dir,
                )
            }

            // `$ cargo run add-specs -p -u network_url_address
            // <encryption override> <optional token override>`
            //
            // Update the database by making RPC calls at `network_url_address`.
            //
            // This command is intended for the networks not introduced to the
            // database, and **must** contain encryption override.
            //
            // Processing known URL or a different URL for known network
            // genesis hash results in an error.
            //
            // In some cases the command may contain token override as well.
            Content::Address { s: address } => {
                // not allowed to proceed without encryption override defined
                if let Some(encryption) = instruction.over.encryption {
                    let database = sled::open(instruction.db)?;
                    specs_pt_u(
                        &database,
                        &address,
                        encryption,
                        instruction.over.token(),
                        instruction.over.title,
                        false,
                        instruction.files_dir,
                    )
                } else {
                    Err(Error::NotSupported)
                }
            }
        },

        // `-t` setting key or no setting key: produce `add_specs` payloads,
        // update the database.
        Set::T => match instruction.content.clone().into() {
            // Network specs are expected to remain constant over time,
            // this command seems to be of no use.
            Content::All { pass_errors: _ } => Err(Error::NotSupported),

            // `$ cargo run add-specs -n network_address_book_title
            // <encryption override>`
            //
            // Network already has an entry in the database and could be
            // referred to by network address book title. This key combination
            // is intended to be used to:
            // - add to the hot database same network with different encryption
            // - change token (if possible for given network) or Vault
            // displayed network title
            //
            // Payload files are created.
            Content::Name { s: name } => {
                let database = sled::open(instruction.db)?;

                specs_pt_n(
                    &database,
                    &name,
                    instruction.over,
                    true,
                    instruction.files_dir,
                )
            }

            // `$ cargo run add-specs -u network_url_address
            // <encryption override> <optional token override>`
            //
            // Update the database by making RPC calls at `network_url_address`
            // and create `add_specs` payload file.
            //
            // This command is intended for the networks not introduced to the
            // database, and **must** contain encryption override.
            //
            // Processing known URL or a different URL for known network
            // genesis hash results in an error.
            //
            // In some cases the command may contain token override as well.
            Content::Address { s: address } => {
                // not allowed to proceed without encryption override defined
                if let Some(encryption) = instruction.over.encryption {
                    let database = sled::open(&instruction.db)?;
                    specs_pt_u(
                        &database,
                        &address,
                        encryption,
                        instruction.over.token(),
                        instruction.over.title,
                        true,
                        instruction.files_dir,
                    )
                } else {
                    Err(Error::NotSupported)
                }
            }
        },
    }
}

/// `add-specs -f -a` for individual address book entry.
///
/// - Get network specs
///   [`NetworkSpecs`](definitions::network_specs::NetworkSpecs) from
///   the database using information in address book entry
/// - Output raw bytes payload file
fn specs_f_a_element<P>(database: &sled::Db, entry: &AddressBookEntry, files_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let network_specs = network_specs_from_entry(database, entry)?;
    add_specs_print(&network_specs, files_dir)
}

/// `add-specs -f -n <address_book_title> <override(s)>`
///
/// Token override is not allowed. Encryption and title override are optional.
/// Overrides are used to modify the entry for specified address book title.
///
/// - Get address book entry for the network using network address book `title`
/// - Get network specs
///   [`NetworkSpecs`](definitions::network_specs::NetworkSpecs) from
///   the database using information in address book entry
/// - Output raw bytes payload file
fn specs_f_n<P>(
    database: &sled::Db,
    title: &str,
    optional_encryption_override: Option<Encryption>,
    optional_signer_title_override: Option<String>,
    files_dir: P,
) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut network_specs = network_specs_from_title(database, title)?;
    match optional_encryption_override {
        Some(encryption) => {
            if network_specs.encryption == encryption {
                if let Some(new_title) = optional_signer_title_override {
                    network_specs.title = new_title
                }
                add_specs_print(&network_specs, &files_dir)
            } else {
                network_specs.title = optional_signer_title_override.unwrap_or(format!(
                    "{}-{}",
                    network_specs.name,
                    encryption.show()
                ));
                network_specs.encryption = encryption;
                add_specs_print(&network_specs, &files_dir)
            }
        }
        None => add_specs_print(&network_specs, &files_dir),
    }
}

/// `add-specs -d -u <url_address> <override(s)>`
///
/// Encryption override is mandatory. Title override is optional. Token override
/// is possible if token set is fetched.
///
/// - Fetch network information using RPC calls and interpret it
/// - Construct
///   [`NetworkSpecs`](definitions::network_specs::NetworkSpecs) with
///   fetched values, user overrides and defaults
/// - Output raw bytes payload file
fn specs_d_u<P>(
    address: &str,
    encryption: Encryption,
    optional_token_override: Option<Token>,
    optional_signer_title_override: Option<String>,
    files_dir: P,
) -> Result<()>
where
    P: AsRef<Path>,
{
    let specs = specs_agnostic(
        address,
        encryption,
        optional_token_override,
        optional_signer_title_override,
    )?;
    add_specs_print(&specs, &files_dir)
}

/// `add-specs <-p/-t> -n <address_book_title> <override(s)>`
///
/// Encryption and title overrides are possible. Token override is possible if
/// network has token set.
///
/// Function inputs network address book title, override set [`Override`], and
/// `printing` flag indicating if payload file should be made.
///
/// - Search for an address book entry by address book title and get
///   corresponding
///   [`NetworkSpecs`](definitions::network_specs::NetworkSpecs)
/// - Fetch network specs through RPC calls and check that the network specs
///   from the database are still valid
/// - Modify network specs according to the overrides requested
/// - Update database as needed: [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) and
///   [`SPECSTREEPREP`](constants::SPECSTREEPREP) are updated if the encryption
///   was not previously in the database for this network,
///   [`SPECSTREEPREP`](constants::SPECSTREEPREP) alone is updated if the
///   overrides modified network specs entry
/// - Print payload files if requested
///
/// Network address book title for new address book entries is constructed as
/// `<network_name>-<encryption>`. Field `title` in network specs
/// [`NetworkSpecs`](definitions::network_specs::NetworkSpecs), i.e.
/// the title under which Vault displays the network, is also constructed as
/// `<network_name>-<encryption>` for non-default networks, unless overridden by
/// the user.
fn specs_pt_n<P>(
    database: &sled::Db,
    title: &str,
    over: Override,
    printing: bool,
    files_dir: P,
) -> Result<()>
where
    P: AsRef<Path>,
{
    // address book entry for `title`
    let address_book_entry = get_address_book_entry(database, title)?;
    let mut network_specs_to_change = network_specs_from_entry(database, &address_book_entry)?;
    let make_update = match over.encryption {
        // requested encryption override
        Some(ref encryption) => {
            // encryption is already correct in title entered by user
            if address_book_entry.encryption == *encryption {
                update_known_specs(
                    &address_book_entry.address,
                    &mut network_specs_to_change,
                    over.title.clone(),
                    over.token(),
                )?
            }
            // encryption in override is different from encryption in title
            else {
                // construct `NetworkSpecsKey` with encryption from override and
                // known genesis hash
                let network_specs_key_possible =
                    NetworkSpecsKey::from_parts(&address_book_entry.genesis_hash, encryption);

                // check if this new network specs key has an entry in the
                // database
                match try_get_network_specs_to_send(database, &network_specs_key_possible)? {
                    // user entered encryption override that already has an
                    // entry in the database, only with wrong address book title
                    //
                    // try applying other overrides
                    Some(network_specs_found) => {
                        network_specs_to_change = network_specs_found;
                        update_known_specs(
                            &address_book_entry.address,
                            &mut network_specs_to_change,
                            over.title.clone(),
                            over.token(),
                        )?
                    }

                    // user has actually entered encryption override that is new
                    // to the database
                    None => {
                        update_modify_encryption_specs(
                            &address_book_entry.address,
                            &mut network_specs_to_change,
                            encryption,
                            over.title.clone(),
                            over.token(),
                        )?;
                        true
                    }
                }
            }
        }
        None => update_known_specs(
            &address_book_entry.address,
            &mut network_specs_to_change,
            over.title.clone(),
            over.token(),
        )?,
    };

    if make_update {
        db_upd_network(
            database,
            &address_book_entry.address,
            &network_specs_to_change,
        )?;
        if printing {
            add_specs_print(&network_specs_to_change, &files_dir)
        } else {
            Ok(())
        }
    } else if printing {
        add_specs_print(&network_specs_to_change, &files_dir)
    } else {
        Err(Error::SpecsInDb {
            name: address_book_entry.name,
            encryption: network_specs_to_change.encryption,
        })
    }
}

/// `add-specs <-p/-t> -u <url_address> <override(s)>`
///
/// Encryption override is mandatory. Title override is optional. Token override
/// is possible if token set is fetched.
///
/// Function inputs `&str` URL address that could be used for RPC calls,
/// encryption supported by the network [`Encryption`], optional token and
/// title overrides and `printing` flag indicating if payload file should be
/// made.
///
/// - Check that the URL address is unknown to the database
/// - Fetch network information using RPC calls and interpret it
/// - Check that there is no entries with same genesis hash as was just fetched
///   in the database
/// - Construct
///   [`NetworkSpecs`](definitions::network_specs::NetworkSpecs) with
///   fetched values, user overrides and defaults
/// - Construct `AddressBookEntry`
/// - Update the database (network specs and address book)
/// - Output raw bytes payload files if requested
fn specs_pt_u<P>(
    database: &sled::Db,
    address: &str,
    encryption: Encryption,
    optional_token_override: Option<Token>,
    optional_signer_title_override: Option<String>,
    printing: bool,
    files_dir: P,
) -> Result<()>
where
    P: AsRef<Path>,
{
    let known_address_set = filter_address_book_by_url(database, address)?;

    if !known_address_set.is_empty() {
        return Err(Error::UKeyUrlInDb {
            title: known_address_set[0].0.to_string(),
            url: address.to_string(),
        });
    }

    let specs = specs_agnostic(
        address,
        encryption,
        optional_token_override,
        optional_signer_title_override,
    )?;

    match genesis_hash_in_hot_db(database, specs.genesis_hash)? {
        Some(address_book_entry) => Err(Error::UKeyHashInDb {
            address_book_entry,
            url: address.to_string(),
        }),
        None => {
            db_upd_network(database, address, &specs)?;
            if printing {
                add_specs_print(&specs, &files_dir)?
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{ContentArgs, Override, SetFlags};
    use constants::{FOLDER, HOT_DB_NAME};

    // The aim is to check that RPC calls are going through for "officially
    // approved" networks. Although the blanket fetch test was nice, not all
    // networks could be reached at all the times, therefore this is currently
    // limited to three default networks that must be working always.
    #[test]
    #[ignore]
    fn mass_fetch() {
        let address_set = [
            "wss://rpc.polkadot.io",
            "wss://kusama-rpc.polkadot.io",
            "wss://westend-rpc.polkadot.io",
        ];
        let mut all_clear = true;
        for address in address_set {
            let instruction = InstructionSpecs {
                set: SetFlags {
                    d: true,
                    ..Default::default()
                },
                content: ContentArgs {
                    address: Some(address.to_string()),
                    ..Default::default()
                },
                over: Override {
                    encryption: Some(Encryption::Sr25519),
                    title: None,
                    token_unit: None,
                    token_decimals: None,
                },
                db: HOT_DB_NAME.into(),
                files_dir: FOLDER.into(),
            };
            match gen_add_specs(instruction) {
                Ok(()) => (),
                Err(e) => {
                    println!("Error: \n{e}");
                    all_clear = false;
                }
            };
        }
        let path_set = std::fs::read_dir(FOLDER).unwrap();
        for x in path_set.flatten() {
            if let Some(filename) = x.path().to_str() {
                if filename.contains("sign_me_add_specs") {
                    std::fs::remove_file(x.path()).unwrap()
                }
            }
        }
        assert!(all_clear, "Errors were encountered");
    }
}
