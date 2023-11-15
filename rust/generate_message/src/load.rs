//! `load_metadata` payloads and metadata related hot database updates
//!
//! This module deals with processing commands:
//!
//! - `$ cargo run load-metadata <key(s)> <(argument)>` to produce
//! `load_metadata` update payloads from the database entries and through RPC
//! calls and update the hot database
//!
//! - `$ cargo run unwasm -payload <wasm_file_path> <optional -d key>` to
//! produce `load_metadata` update payloads from `.wasm` files and update the
//! hot database
//!
//! - `$ cargo run meta_default_file -name <network_name> -version
//! <network_version>` to generates metadata files for `defaults` crate from
//! hot database entries
use sp_core::H256;
use std::path::Path;

use constants::METATREE;
use db_handling::helpers::{get_meta_values_by_name_version, open_tree};
use definitions::{error::MetadataError, keyring::MetaKeyPrefix, metadata::MetaValues};

use crate::error::{Changed, Error, Result};
use crate::helpers::{
    add_new_metadata, address_book_content, db_upd_metadata, error_occured, load_metadata_print,
    meta_fetch, network_specs_from_entry, prepare_metadata, MetaFetched, MetaShortCut,
    MetaValuesStamped, SortedMetaValues, Write,
};
use crate::parser::{Content, InstructionMeta, Set};

/// Process `load-metadata` command according to the [`InstructionMeta`]
/// received from the command line.
pub fn gen_load_meta(instruction: InstructionMeta) -> Result<()> {
    match instruction.set.into() {
        // `-f` setting key: produce payload files from existing database
        // entries.
        Set::F => match instruction.content.into() {
            // `$ cargo run load-metadata -f -a`
            //
            // Make payloads for all metadata entries in the database.
            Content::All { pass_errors } => {
                // Get `AddressSpecs` for each network in `ADDRESS_BOOK`
                let database = sled::open(&instruction.db)?;
                let set = address_specs_set(&database)?;

                // Process each entry
                for x in set.iter() {
                    match meta_f_a_element(&database, x, &instruction.files_dir) {
                        Ok(()) => (),
                        Err(e) => error_occured(e, pass_errors)?,
                    }
                }
                Ok(())
            }

            // `$ cargo run load-metadata -f -n <network_name>`
            //
            // Make payload(s) for all metadata entries in the database for
            // network with user-entered name.
            Content::Name { s: name } => {
                let database = sled::open(&instruction.db)?;
                meta_f_n(&database, &name, &instruction.files_dir)
            }

            // `-u` content key is to provide the URL address for RPC calls;
            // since `-f` indicates the data is taken from the database, the
            // the combination seems of no use.
            Content::Address { .. } => Err(Error::NotSupported),
        },

        // `-d` setting key: get network data using RPC calls, **do not**
        // update the database, export payload files.
        Set::D => match instruction.content.into() {
            // `$ cargo run load-metadata -d -a`
            //
            // Make RPC calls for all networks in `ADDRESS_BOOK`, produce
            // `load_metadata` payload files.
            Content::All { pass_errors } => {
                // Collect `AddressSpecs` for each network in `ADDRESS_BOOK`
                let database = sled::open(&instruction.db)?;
                let set = address_specs_set(&database)?;

                // Process each entry
                for x in set.iter() {
                    match meta_d_a_element(x, &instruction.files_dir) {
                        Ok(()) => (),
                        Err(e) => error_occured(e, pass_errors)?,
                    }
                }
                Ok(())
            }

            // `$ cargo run load-metadata -d -n <network_name>`
            //
            // Make RPC calls for network with user-entered name and produce
            // `load_metadata` payload file.
            //
            // Network here must already have an entry in `ADDRESS_BOOK`, so
            // so that the URL address at which to make RPC call is made could
            // be found.
            Content::Name { s: name } => {
                let database = sled::open(&instruction.db)?;
                meta_d_n(&database, &name, &instruction.files_dir)
            }

            // `$ cargo run load-metadata -d -u <url_address>`
            //
            // Make RPC calls for network at user-entered URL address and
            // produce `load_metadata` payload file.
            //
            // This is intended for the networks that do not have yet entries in
            // `ADDRESS_BOOK`.
            //
            // This key combination is completely agnostic and will not address
            // the database at all. If there are changes in the base58 prefix or
            // genesis hash, this will not be found here.
            Content::Address { s: address } => meta_d_u(&address, &instruction.files_dir),
        },

        // `-k` setting key: get network data using RPC calls, update the
        // database, produce `load_metadata` payload files only if new metadata
        // was fetched.
        Set::K => {
            let write = Write::OnlyNew;
            match instruction.content.into() {
                // `$ cargo run load-metadata -k -a`
                //
                // Make RPC calls, update the database as needed and produce
                // payload files if new data is fetched for all networks in
                // address book.
                //
                // If there are two entries for the same network with different
                // encryption, fetch and (possibly) payload export is done only
                // once: `load_metadata` payloads do not specify encryption.
                Content::All { pass_errors } => {
                    let database = sled::open(instruction.db)?;
                    meta_kpt_a(&database, &write, pass_errors, &instruction.files_dir)
                }

                // `$ cargo run load-metadata -k -n <network_name>`
                //
                // Make RPC calls, update the database as needed and produce
                // payload file if new data is fetched for network with
                // specified name.
                //
                // This command is for networks already having at least one
                // entry in the `ADDRESS_BOOK` and `SPECSTREEPREP` of the hot
                // database.
                //
                // Regardless of how many entries with different encryptions are
                // there, fetch and (possibly) payload export is done only once.
                Content::Name { s: name } => {
                    let database = sled::open(instruction.db)?;
                    meta_kpt_n(&database, &name, &write, &instruction.files_dir)
                }

                // Key `-u` is for URL addresses. If network has no entry in the
                // database, its metadata can not be added before its specs. If
                // network has an entry in the database, it is simpler to
                // address it with `-n <network_name>` combination.
                Content::Address { .. } => Err(Error::NotSupported),
            }
        }

        // `-p` setting key: get network data using RPC calls and update the
        // database.
        Set::P => {
            let write = Write::None;
            match instruction.content.into() {
                // `$ cargo run load-metadata -p -a`
                //
                // Make RPC calls and update the database as needed for all
                // networks in address book.
                //
                // One fetch for each address.
                Content::All { pass_errors } => {
                    let database = sled::open(instruction.db)?;
                    meta_kpt_a(&database, &write, pass_errors, &instruction.files_dir)
                }

                // `$ cargo run load-metadata -p -n <network_name>`
                //
                // Make RPC calls and update the database as needed for network
                // with specified name.
                //
                // This command is for networks already having at least one
                // entry in the `ADDRESS_BOOK` and `SPECSTREEPREP` of the hot
                // database.
                //
                // One fetch only.
                Content::Name { s: name } => {
                    let database = sled::open(instruction.db)?;
                    meta_kpt_n(&database, &name, &write, &instruction.files_dir)
                }

                // Key `-u` is for URL addresses. If network has no entry in the
                // database, its metadata can not be added before its specs. If
                // network has an entry in the database, it is simpler to
                // address it with `-n <network_name>` combination.
                Content::Address { .. } => Err(Error::NotSupported),
            }
        }

        // `-t` setting key or no setting key: get network data using RPC calls
        // and update the database.
        Set::T => {
            let write = Write::All;
            match instruction.content.into() {
                // `$ cargo run load-metadata -a`
                //
                // Make RPC calls, update the database as needed and produce
                // payload files for all networks in address book.
                //
                // One fetch and one payload print for each address.
                Content::All { pass_errors } => {
                    let database = sled::open(instruction.db)?;
                    meta_kpt_a(&database, &write, pass_errors, &instruction.files_dir)
                }

                // `$ cargo run load-metadata -n <network_name>`
                //
                // Make RPC calls, update the database as needed and produce
                // payload file for network with specified name.
                //
                // This command is for networks already having at least one
                // entry in the `ADDRESS_BOOK` and `SPECSTREEPREP` of the hot
                // database.
                //
                // One fetch and one payload print only.
                Content::Name { s: name } => {
                    let database = sled::open(instruction.db)?;
                    meta_kpt_n(&database, &name, &write, &instruction.files_dir)
                }

                // Key `-u` is for URL addresses. If network has no entry in the
                // database, its metadata can not be added before its specs. If
                // network has an entry in the database, it is simpler to
                // address it with `-n <network_name>` combination.
                Content::Address { .. } => Err(Error::NotSupported),
            }
        }
    }
}

/// `load-metadata-f -a` for individual [`AddressSpecs`] value.
///
/// - Get metadata entries from database [`METATREE`] by [`MetaKeyPrefix`]
/// generated with network name. At most two entries are expected.
/// - Check the metadata integrity
/// - Output raw bytes payload file
fn meta_f_a_element<P>(database: &sled::Db, set_element: &AddressSpecs, files_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let meta_key_prefix = MetaKeyPrefix::from_name(&set_element.name);
    let metadata = open_tree(database, METATREE)?;
    for x in metadata.scan_prefix(meta_key_prefix.prefix()).flatten() {
        let meta_values = MetaValues::from_entry_checked(x)?;
        if meta_values.warn_incomplete_extensions {
            warn(&meta_values.name, meta_values.version);
        }
        if let Some(prefix_from_meta) = meta_values.optional_base58prefix {
            if prefix_from_meta != set_element.base58prefix {
                Err(MetadataError::Base58PrefixSpecsMismatch {
                    specs: set_element.base58prefix,
                    meta: prefix_from_meta,
                })?;
            }
        }
        let shortcut = MetaShortCut {
            meta_values,
            genesis_hash: set_element.genesis_hash,
        };
        load_metadata_print(&shortcut, &files_dir)?;
    }
    Ok(())
}

/// `load-metadata-f -n <network_name>`
///
/// - Get all available [`AddressSpecs`] from the database and search for the
/// one with user-entered network name
/// - Get metadata entries from database [`METATREE`] by [`MetaKeyPrefix`]
/// generated with `name`. At most two entries are expected.
/// - Check the metadata integrity
/// - Output raw bytes payload file
fn meta_f_n<P>(database: &sled::Db, name: &str, files_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    meta_f_a_element(database, &search_name(database, name)?, &files_dir)
}

/// `load-metadata-d -a` for individual [`AddressSpecs`] value.
///
/// - Fetch network information using RPC calls at `address` in [`AddressSpecs`]
/// and interpret it
/// - Check the metadata integrity with the data on record in the database
/// - Output raw bytes payload file
fn meta_d_a_element<P>(set_element: &AddressSpecs, files_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let meta_fetch = fetch_set_element(set_element)?;
    load_metadata_print(&meta_fetch.cut(), files_dir)
}

/// `load-metadata-d -n <network_name>`
///
/// - Get all available [`AddressSpecs`] from the database and search for the
/// one with user-entered network name
/// - Fetch network information using RPC calls at `address` in [`AddressSpecs`]
/// and interpret it
/// - Check the metadata integrity with the data on record in the database
/// - Output raw bytes payload file
fn meta_d_n<P>(database: &sled::Db, name: &str, files_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    meta_d_a_element(&search_name(database, name)?, files_dir)
}

/// `load-metadata-d -u <url_address>`
///
/// - Fetch network information using RPC calls at user-entered `address` and
/// interpret it
/// - Output raw bytes payload file
///
/// The command is intended to be used with unknown networks that do not have
/// yet an entry in the database. Known networks are better addressed by the
/// network name.
///
/// No checking of metadata and network specs integrity is done here, as there
/// are no network specs. Base58 prefix change in the metadata would cause no
/// error here. The Vault, if such contradicting metadata update is scanned,
/// will produce an error, since the Vault must have matching network specs to
/// accept the metadata.
fn meta_d_u<P>(address: &str, files_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let meta_fetched = meta_fetch(address)?;
    if meta_fetched.meta_values.warn_incomplete_extensions {
        warn(
            &meta_fetched.meta_values.name,
            meta_fetched.meta_values.version,
        );
    }
    load_metadata_print(&meta_fetched.cut(), files_dir)
}

/// `load-metadata<-k/-p/-t> -a`
///
/// - Get all available [`AddressSpecs`] from the database
/// - Get and sort existing metadata entries from [`METATREE`], with block
/// data from [`META_HISTORY`](constants::META_HISTORY) if available
/// - Process each [`AddressSpecs`] and update sorted metadata entries in the
/// process. Input [`Write`] indicates if the payload file should be created.
/// - Rewrite the database [`METATREE`] with updated metadata set and update
/// [`META_HISTORY`](constants::META_HISTORY)
fn meta_kpt_a<P>(database: &sled::Db, write: &Write, pass_errors: bool, files_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let set = address_specs_set(database)?;
    let mut sorted_meta_values = prepare_metadata(database)?;
    for x in set.iter() {
        match meta_kpt_a_element(x, write, &mut sorted_meta_values, &files_dir) {
            Ok(_) => (),
            Err(e) => error_occured(e, pass_errors)?,
        };
    }
    db_upd_metadata(database, sorted_meta_values)
}

/// `load-metadata<-k/-p/-t> -a` for individual [`AddressSpecs`] value.
///
/// - Fetch network information using RPC calls at `address` in [`AddressSpecs`]
/// and interpret it
/// - Check the metadata integrity with the data on record in the database,
/// insert it into received [`SortedMetaValues`]
/// - Output raw bytes payload file, if requested by input [`Write`]
///
/// Inputs [`AddressSpecs`] for the network currently processed, [`Write`]
/// indicating if the `load_metadata` payload should be created, and
/// [`SortedMetaValues`] to be updated.
fn meta_kpt_a_element<P>(
    set_element: &AddressSpecs,
    write: &Write,
    sorted_meta_values: &mut SortedMetaValues,
    files_dir: P,
) -> Result<()>
where
    P: AsRef<Path>,
{
    let meta_fetched = fetch_set_element(set_element)?;
    let got_meta_update = add_new_metadata(&meta_fetched.stamped(), sorted_meta_values)?;
    match write {
        Write::All => load_metadata_print(&meta_fetched.cut(), files_dir)?,
        Write::OnlyNew => {
            if got_meta_update {
                load_metadata_print(&meta_fetched.cut(), files_dir)?
            }
        }
        Write::None => (),
    }
    if got_meta_update {
        println!(
            "Fetched new metadata {}{} at block hash {}",
            meta_fetched.meta_values.name,
            meta_fetched.meta_values.version,
            hex::encode(meta_fetched.block_hash)
        )
    } else {
        println!(
            "Fetched previously known metadata {}{}",
            meta_fetched.meta_values.name, meta_fetched.meta_values.version,
        )
    }
    Ok(())
}

/// `load-metadata<-k/-p/-t> -n <network_name>`
///
/// - Get and sort existing metadata entries from [`METATREE`], with block
/// data from [`META_HISTORY`](constants::META_HISTORY) if available
/// - Get all available [`AddressSpecs`] from the database and search for the
/// one with user-entered network name
/// - Fetch network information using RPC calls at `address` in [`AddressSpecs`]
/// and interpret it
/// - Check the metadata integrity with the data on record in the database,
/// insert it into [`SortedMetaValues`]
/// - Output raw bytes payload file, if requested by input [`Write`]
/// - Rewrite the database [`METATREE`] with updated metadata set and update
/// [`META_HISTORY`](constants::META_HISTORY)
///
/// Inputs user-entered network name and [`Write`] indicating if the
/// `load_metadata` payload should be created.
fn meta_kpt_n<P>(database: &sled::Db, name: &str, write: &Write, files_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut sorted_meta_values = prepare_metadata(database)?;
    meta_kpt_a_element(
        &search_name(database, name)?,
        write,
        &mut sorted_meta_values,
        files_dir,
    )?;
    db_upd_metadata(database, sorted_meta_values)
}

/// Network information from [`ADDRESS_BOOK`](constants::ADDRESS_BOOK) and
/// [`SPECSTREEPREP`](constants::SPECSTREEPREP).
///
/// This data is sufficient to make RPC calls and check that the metadata is
/// consistent with existing database content.
#[derive(PartialEq)]
struct AddressSpecs {
    address: String,
    base58prefix: u16,
    genesis_hash: H256,
    name: String,
}

/// Collect all unique [`AddressSpecs`] from the hot database.
fn address_specs_set(database: &sled::Db) -> Result<Vec<AddressSpecs>> {
    let set = address_book_content(database)?;
    if set.is_empty() {
        return Err(Error::AddressBookEmpty);
    }
    let mut out: Vec<AddressSpecs> = Vec::new();
    for (_, x) in set.iter() {
        let specs = network_specs_from_entry(database, x)?;
        for y in out.iter() {
            if y.name == specs.name {
                if y.genesis_hash != specs.genesis_hash {
                    return Err(Error::TwoGenesisHashVariantsForName {
                        name: x.name.to_string(),
                    });
                }
                if y.address != x.address {
                    return Err(Error::TwoUrlVariantsForName {
                        name: x.name.to_string(),
                    });
                }
                if y.base58prefix != specs.base58prefix {
                    return Err(Error::TwoBase58ForName {
                        name: x.name.to_string(),
                    });
                }
            }
        }
        let new = AddressSpecs {
            address: x.address.to_string(),
            base58prefix: specs.base58prefix,
            genesis_hash: specs.genesis_hash,
            name: specs.name.to_string(),
        };
        if !out.contains(&new) {
            out.push(new)
        }
    }
    Ok(out)
}

/// Find [`AddressSpecs`] with certain `name`.
fn search_name(database: &sled::Db, name: &str) -> Result<AddressSpecs> {
    let set = address_specs_set(database)?;
    let mut found = None;
    for x in set.into_iter() {
        if x.name == name {
            found = Some(x);
            break;
        }
    }
    match found {
        Some(a) => Ok(a),
        None => Err(Error::AddressBookEntryWithName {
            name: name.to_string(),
        }),
    }
}

/// Make RPC calls and check the received information for given
/// [`AddressSpecs`].
///
/// Checks that the network name, genesis hash, and base58 prefix did not
/// change compared to what is on record in the database. Warns if the metadata
/// (`v14`) has incomplete set of signed extensions.
///
/// Outputs [`MetaFetched`], the data sufficient to produce `load_metadata`
/// payload and update the database.
fn fetch_set_element(set_element: &AddressSpecs) -> Result<MetaFetched> {
    let meta_fetched = meta_fetch(&set_element.address)?;
    if meta_fetched.meta_values.name != set_element.name {
        return Err(Error::ValuesChanged {
            url: set_element.address.to_string(),
            what: Changed::Name {
                old: set_element.name.to_string(),
                new: meta_fetched.meta_values.name,
            },
        });
    }
    if meta_fetched.genesis_hash != set_element.genesis_hash {
        return Err(Error::ValuesChanged {
            url: set_element.address.to_string(),
            what: Changed::GenesisHash {
                old: set_element.genesis_hash,
                new: meta_fetched.genesis_hash,
            },
        });
    }
    if let Some(prefix_from_meta) = meta_fetched.meta_values.optional_base58prefix {
        if prefix_from_meta != set_element.base58prefix {
            Err(MetadataError::Base58PrefixSpecsMismatch {
                specs: set_element.base58prefix,
                meta: prefix_from_meta,
            })?;
        }
    }
    if meta_fetched.meta_values.warn_incomplete_extensions {
        warn(
            &meta_fetched.meta_values.name,
            meta_fetched.meta_values.version,
        );
    }
    Ok(meta_fetched)
}

/// Show warning if the metadata (`v14`) has incomplete set of signed extensions.
fn warn(name: &str, version: u32) {
    println!("Warning. Metadata {name}{version} has incomplete set of signed extensions, and could cause Vault to fail in parsing signable transactions using this metadata.");
}

/// `unwasm -payload <wasm_file_path> <optional -d key>`
///
/// Generate `load_metadata` payload from `.wasm` files.
///
/// Function is intended to be used for metadata not yet published on a node and
/// only for the networks that have network specs on record in the hot database.
///
/// Metadata is retrieved from `.wasm` file itself. To get genesis hash needed
/// to complete `load_metadata` payload and to check the metadata for
/// consistency, network name found in the metadata is used to retrieve
/// [`NetworkSpecs`](definitions::network_specs::NetworkSpecs).
///
/// Optional key `-d`, if used, indicates that the metadata entry should **not**
/// be added to the [`METATREE`] of the hot database.
pub fn unwasm<P>(database: &sled::Db, filename: &str, update_db: bool, files_dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let meta_values = MetaValues::from_wasm_file(filename)?;
    let set_element = search_name(database, &meta_values.name)?;
    if let Some(prefix_from_meta) = meta_values.optional_base58prefix {
        if prefix_from_meta != set_element.base58prefix {
            Err(MetadataError::Base58PrefixSpecsMismatch {
                specs: set_element.base58prefix,
                meta: prefix_from_meta,
            })?;
        }
    }
    let genesis_hash = set_element.genesis_hash;
    if update_db {
        let meta_values_stamped = MetaValuesStamped {
            meta_values: meta_values.to_owned(),
            at_block_hash: None,
        };
        let mut sorted_meta_values = prepare_metadata(database)?;
        let got_meta_update = add_new_metadata(&meta_values_stamped, &mut sorted_meta_values)?;
        if got_meta_update {
            println!(
                "Unwasmed new metadata {}{}",
                meta_values.name, meta_values.version
            )
        } else {
            println!(
                "Unwasmed previously known metadata {}{}",
                meta_values.name, meta_values.version
            )
        }
        db_upd_metadata(database, sorted_meta_values)?;
    }
    let shortcut = MetaShortCut {
        meta_values,
        genesis_hash,
    };
    load_metadata_print(&shortcut, files_dir)
}

/// `meta_default_file -name <network_name> -version <metadata_version>`
///
/// Generate text file with hex string metadata, from a hot database
/// [`METATREE`] entry, for `defaults` crate.
pub fn meta_default_file<P>(
    database: &sled::Db,
    name: &str,
    version: u32,
    export_dir: P,
) -> Result<()>
where
    P: AsRef<Path>,
{
    let meta_values = get_meta_values_by_name_version(database, name, version)?;
    let file_path = export_dir.as_ref().join(format!("{name}{version}"));
    std::fs::write(file_path, hex::encode(meta_values.meta))?;
    Ok(())
}
