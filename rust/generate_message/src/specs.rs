//! `add_specs` payloads and updating
//! [`SPECSTREEPREP`](constants::SPECSTREEPREP) tree of the hot database
//!
//! This module deals with processing command:
//!
//! `$ cargo run add_specs <keys> <argument(s)>`
//!
// TODO add direct link to keys and agruments so that they are not repeated
// here, again
//!
//! Data could be either from the rpc calls or from the hot database itself.
//!
//! Payload `add_specs` is exported in dedicated [`FOLDER`](constants::FOLDER)
//! to (optionally) be signed and later be transformed into `add_specs` update
//! QR. Output file name is `sign_me_add_specs_<name>_<encryption>`.
use constants::{ADDRESS_BOOK, HOT_DB_NAME};
use db_handling::helpers::{open_db, open_tree};
use definitions::{
    crypto::Encryption,
    error_active::{Active, DatabaseActive, ErrorActive, Fetch, NotFoundActive},
    keyring::NetworkSpecsKey,
    metadata::AddressBookEntry,
};
use sled::IVec;

use crate::helpers::{
    error_occured, filter_address_book_by_url, get_address_book_entry, get_network_specs_to_send,
    network_specs_from_entry, network_specs_from_title, process_indices,
    try_get_network_specs_to_send, update_db,
};
use crate::metadata_shortcut::specs_shortcut;
use crate::output_prep::print_specs;
use crate::parser::{Content, Instruction, Set, TokenOverride};

/// Process `add_specs` command according to the [`Instruction`] received from
/// the command line
pub fn gen_add_specs(instruction: Instruction) -> Result<(), ErrorActive> {
    match instruction.set {
        // `-f` setting key: produce `add_specs` payload files from existing
        // database entries.
        //
        // Note that `-f` key processed with an encryption override **will not**
        // add entry with a new encryption in the database.
        Set::F => match instruction.content {
            // `$ cargo run add_specs -f -a`
            //
            // Make `add_specs` payloads for all specs entries in the database.
            Content::All => {
                // makes no sense to override encryption for all entries at once
                if instruction.over.encryption.is_some() {
                    return Err(ErrorActive::NotSupported);
                }

                // ...or to override token for all entries at once
                if instruction.over.token.is_some() {
                    return Err(ErrorActive::NotSupported);
                }

                // collect `ADDRESS_BOOK` entries
                let mut address_book_set: Vec<(IVec, IVec)> = Vec::new();
                {
                    let database = open_db::<Active>(HOT_DB_NAME)?;
                    let address_book = open_tree::<Active>(&database, ADDRESS_BOOK)?;
                    if address_book.is_empty() {
                        return Err(ErrorActive::Database(DatabaseActive::AddressBookEmpty));
                    }
                    for x in address_book.iter().flatten() {
                        address_book_set.push(x)
                    }
                }

                // process each entry
                for address_book_entry_encoded in address_book_set.into_iter() {
                    match specs_f_a_element(address_book_entry_encoded) {
                        Ok(()) => (),
                        Err(e) => error_occured(e, instruction.pass_errors)?,
                    }
                }
                Ok(())
            }

            // `$ cargo run add_specs -f -n network_address_book_title
            // <optional encryption override>`
            //
            // Make `add_specs` payload for single specs entry **already in the
            // database**, referred to by network address book title.
            Content::Name(name) => {
                // entry is expected to be in the database, meaning the token is
                // already set up
                //
                // should not change it
                if instruction.over.token.is_some() {
                    return Err(ErrorActive::NotSupported);
                }
                specs_f_n(&name, instruction.over.encryption)
            }

            // `$ cargo run add_specs -f -u network_url_address
            // <optional encryption override>`
            //
            // Make `add_specs` payload(s) for specs entries **already in the
            // database**, referred to by network url address.
            //
            // Multiple payloads will be produced if multiple encryptions are
            // in the database for the same network and no encryption override
            // is invoked.
            Content::Address(address) => {
                // key `-f` implies that the entry is in the database, even
                // though it is searched for by url address
                //
                // token is set up, changing it is not allowed
                if instruction.over.token.is_some() {
                    return Err(ErrorActive::NotSupported);
                }
                specs_f_u(&address, instruction.over.encryption)
            }
        },

        // `-d` setting key: produce `add_specs` payloads through rpc calls,
        // **do not** update the database, export payload files.
        Set::D => match instruction.content {
            // `-d` key implies that the database does not get updated (only
            // the payload files are produced from rpc calls), `-a` key implies
            // that the action is done for all entries in database address book;
            //
            // Network specs are expected to remain constant over time,
            // therefore this command seems to be of no use.
            Content::All => Err(ErrorActive::NotSupported),

            // `-n` key implies that the action is done for the network already
            // having an address book title in the database;
            //
            // Same as `-d -a` combination, of no use.
            Content::Name(_) => Err(ErrorActive::NotSupported),

            // `$ cargo run add_specs -d -u network_url_address
            // <encryption override> <optional token override>`
            //
            // Produce `add_specs` payload by making rpc calls at
            // `network_url_address`, print payload file, do not update the
            // database.
            //
            // Note that if the network genesis hash has a record in the
            // database, and url used for the rpc call is different, an error is
            // produced, even though no writing in the database is expected.
            // Only one url address is allowed at any time as a source of
            // network information.
            //
            // This command is expected to be used mostly for truly unknown
            // networks, and **must** contain encryption override.
            //
            // In some cases the command may contain token override as well.
            Content::Address(address) => {
                // not allowed to proceed without encryption override defined
                if let Some(encryption) = instruction.over.encryption {
                    specs_d_u(&address, encryption, instruction.over.token)
                } else {
                    Err(ErrorActive::NotSupported)
                }
            }
        },

        // `-k` setting key: produce payloads through rpc calls, update the
        // database, export payload files only for updated information.
        //
        // Since network specs are expected to remain constant over time,
        // these commands seem to be of no use.
        Set::K => Err(ErrorActive::NotSupported),

        // `-p` setting key: update the database through rpc calls.
        Set::P => match instruction.content {
            // Network specs are expected to remain constant over time,
            // this command seems to be of no use.
            Content::All => Err(ErrorActive::NotSupported),

            // `$ cargo run add_specs -p -n network_address_book_title
            // <encryption override>`
            //
            // Network already has an entry in the database and could be
            // referred to by network address book title. This key combination
            // is intended to be used to add to the hot database same network
            // with different encryption, without creating payload file.
            Content::Name(name) => {
                // network has entry in the database, token is set up and could
                // not be changed
                if instruction.over.token.is_some() {
                    return Err(ErrorActive::NotSupported);
                }

                // using this command makes sense only if the encryption
                // override is set
                if let Some(encryption) = instruction.over.encryption {
                    specs_pt_n(&name, encryption, false)
                } else {
                    Err(ErrorActive::NotSupported)
                }
            }

            // `$ cargo run add_specs -p -u network_url_address
            // <encryption override> <optional token override>`
            //
            // Update the database by making rpc calls at `network_url_address`.
            //
            // Note that if the network genesis hash has a record in the
            // database, and url used for the rpc call is different, an error is
            // produced. Only one url address is allowed at any time as a source
            // of network information.
            //
            // This command is expected to be used mostly for truly unknown
            // networks, and **must** contain encryption override.
            //
            // In some cases the command may contain token override as well.
            Content::Address(address) => {
                // not allowed to proceed without encryption override defined
                if let Some(encryption) = instruction.over.encryption {
                    specs_pt_u(&address, encryption, instruction.over.token, false)
                } else {
                    Err(ErrorActive::NotSupported)
                }
            }
        },

        // `-t` setting key or no setting key: produce `add_specs` payloads
        // through rpc calls, even if the specs in the payload are already in
        // the database, update the database.
        Set::T => match instruction.content {
            // Network specs are expected to remain constant over time,
            // this command seems to be of no use.
            Content::All => Err(ErrorActive::NotSupported),

            // `$ cargo run add_specs -n network_address_book_title
            // <encryption override>`
            //
            // Network already has an entry in the database and could be
            // referred to by network address book title. This key combination
            // is intended to be used to add to the database same network with
            // different encryption and create `add_specs` payload file at the
            // same time.
            Content::Name(name) => {
                // network has entry in the database, token is set up and could
                // not be changed
                if instruction.over.token.is_some() {
                    return Err(ErrorActive::NotSupported);
                }

                // using this command makes sense only if the encryption
                // override is set
                if let Some(encryption) = instruction.over.encryption {
                    specs_pt_n(&name, encryption, true)
                } else {
                    Err(ErrorActive::NotSupported)
                }
            }

            // `$ cargo run add_specs -u network_url_address
            // <encryption override> <optional token override>`
            //
            // Update the database by making rpc calls at `network_url_address`
            // and create `add_specs` payload file.
            //
            // Note that if the network genesis hash has a record in the
            // database, and url used for the rpc call is different, an error is
            // produced. Only one url address is allowed at any time as a source
            // of network information.
            //
            // This command is expected to be used mostly for truly unknown
            // networks, and **must** contain encryption override.
            //
            // In some cases the command may contain token override as well.
            Content::Address(address) => {
                // not allowed to proceed without encryption override defined
                if let Some(encryption) = instruction.over.encryption {
                    specs_pt_u(&address, encryption, instruction.over.token, true)
                } else {
                    Err(ErrorActive::NotSupported)
                }
            }
        },
    }
}

/// `add_specs -f -a` for individual address book entry.
///
/// - Get network specs
/// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend) from
/// the database using information in address book entry
/// - Output raw bytes payload file
fn specs_f_a_element(entry: (IVec, IVec)) -> Result<(), ErrorActive> {
    let network_specs = network_specs_from_entry(&AddressBookEntry::from_entry(entry)?)?;
    print_specs(&network_specs)
}

/// `add_specs -f -n network_address_book_title <optional encryption override>`
///
/// - Get address book entry for the network using `network_address_book_title`
/// - Get network specs
/// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend) from
/// the database using information in address book entry
/// - Output raw bytes payload file
fn specs_f_n(title: &str, encryption_override: Option<Encryption>) -> Result<(), ErrorActive> {
    let mut network_specs = network_specs_from_title(title)?;
    match encryption_override {
        Some(encryption) => {
            network_specs.encryption = encryption.clone();
            network_specs.title = format!("{}-{}", network_specs.name, encryption.show());
            print_specs(&network_specs)
        }
        None => print_specs(&network_specs),
    }
}

/// `add_specs -f -u network_url_address <optional encryption override>`
///
/// Note that no database updates happen here.
///
/// ## Encryption override **is not** invoked:
///
/// - Get all address book entries corresponding to `network_url_address`
/// (corresponding address book titles could be, for example, `westend`,
/// `westend-ed25519`, and `westend-ecdsa`)
/// - Get network specs
/// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend) from
/// the database for each of these using information in address book entry
/// - Output raw bytes payload files
///
/// ## Encryption override **is** invoked:
///
/// - Get all address book entries corresponding to `network_url_address`
/// - Get existing or construct new network specs
/// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend) with
/// correct encryption and title, using or modifying data for the most
/// preferable address book entry
/// - Output raw bytes payload file
///
/// Address book entries in order of preference:
///
/// 1. with encryption matching the override
/// 2. the one marked default
/// 3. any entry corresponding to url provided
fn specs_f_u(address: &str, encryption_override: Option<Encryption>) -> Result<(), ErrorActive> {
    let entries = filter_address_book_by_url(address)?;
    if entries.is_empty() {
        return Err(ErrorActive::NotFound(
            NotFoundActive::AddressBookEntryWithUrl {
                url: address.to_string(),
            },
        ));
    }
    match encryption_override {
        Some(encryption) => {
            let network_specs = process_indices(&entries, encryption)?.0;
            print_specs(&network_specs)
        }
        None => {
            for x in entries.iter() {
                let network_specs_key = NetworkSpecsKey::from_parts(&x.genesis_hash, &x.encryption);
                let network_specs = get_network_specs_to_send(&network_specs_key)?;
                print_specs(&network_specs)?;
            }
            Ok(())
        }
    }
}

/// `add_specs -d -u network_url_address <encryption override> <optional token
/// override>`
///
/// - Fetch network information using rpc calls and interpret it
/// - If there are entries for the network in the database, as filtered by url
/// in address book, check that no **important** specs have changed: base58
/// prefix, decimals, genesis hash, name, or unit
/// - Use non-important specs from database entry or default ones to construct
/// `NetworkSpecsToSend` entry
/// - Output raw bytes payload file
fn specs_d_u(
    address: &str,
    encryption: Encryption,
    optional_token_override: Option<TokenOverride>,
) -> Result<(), ErrorActive> {
    let shortcut = specs_shortcut(address, encryption, optional_token_override)?;
    print_specs(&shortcut.specs)
}

/// `add_specs <-p/-t> -n network_address_book_title <encryption override>`
///
/// Inputs network address book title, encryption [`Encryption`] requested in
/// the override, and `printing` flag indicating if payload file should be made.
///
/// - Search for an address book entry with exactly same address book title and
/// get corresponding
/// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
/// - If encryption in specs matches with the one in encryption override, **do
/// not** update the database and print the raw bytes payload file if requested
/// - If encryption in specs does not match the one in encryption override,
/// construct new [`NetworkSpecsKey`] and new
/// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend)
/// entry with correct `encryption` and `title`. If new network specs key is
/// known to the database (unlikely), **do not** update the database and print
/// the raw bytes payload file if requested. If new network specs key is not
/// known to the database, construct also
/// [`AddressBookEntry`](definitions::metadata::AddressBookEntry) and add
/// `NetworkSpecsToSend` and `AddressBookEntry` into the database and print the
/// raw bytes payload file if requested.
///
/// Note that no fetch is done while processing this command.
///
/// Network address book title is the key in [`ADDRESS_BOOK`] tree, it is
/// built as `<network name>-<encryption>` for non-default networks. Default
/// networks have `<network name>` as an address book title. Field `title` in
/// network specs
/// [`NetworkSpecsToSend`](definitions::network_specs::NetworkSpecsToSend) is
/// also `<network name>-<encryption>` for non-default networks. Default
/// networks have `<Network name>` as `title` field.
fn specs_pt_n(title: &str, encryption: Encryption, printing: bool) -> Result<(), ErrorActive> {
    let address_book_entry = get_address_book_entry(title)?;
    let network_specs_key_existing = NetworkSpecsKey::from_parts(
        &address_book_entry.genesis_hash,
        &address_book_entry.encryption,
    );
    let network_specs_existing = get_network_specs_to_send(&network_specs_key_existing)?;
    if address_book_entry.encryption == encryption {
        if printing {
            print_specs(&network_specs_existing)
        } else {
            Err(ErrorActive::Fetch(Fetch::SpecsInDb {
                name: address_book_entry.name,
                encryption,
            }))
        }
    } else {
        let network_specs_key_possible =
            NetworkSpecsKey::from_parts(&address_book_entry.genesis_hash, &encryption);
        match try_get_network_specs_to_send(&network_specs_key_possible)? {
            Some(network_specs_found) => {
                if printing {
                    print_specs(&network_specs_found)
                } else {
                    Err(ErrorActive::Fetch(Fetch::SpecsInDb {
                        name: address_book_entry.name,
                        encryption,
                    }))
                }
            }
            None => {
                // this encryption is not on record
                let mut network_specs = network_specs_existing;
                network_specs.encryption = encryption.clone();
                network_specs.title = format!("{}-{}", network_specs.name, encryption.show());
                update_db(&address_book_entry.address, &network_specs)?;
                if printing {
                    print_specs(&network_specs)
                } else {
                    Ok(())
                }
            }
        }
    }
}

/// `add_specs <-p/-t> -u network_url_address <encryption override> <optional
/// token override>`
///
/// Inputs `&str` url address that could be used for rpc calls in given network,
/// encryption [`Encryption`] requested in the override, and `printing` flag
/// indicating if payload file should be made.
///
/// - Fetch network information using rpc calls and interpret it
/// - If there are entries for the network in the database, as filtered by url
/// in address book, check that:
///     - exactly same entry is not yet in the database
///     - no **important** specs have changed: base58 prefix, decimals, genesis
/// hash, name, or unit
/// - Use non-important specs from database entry or default non-important specs
/// from [`constants`] to complete the remaining fields in `NetworkSpecsToSend`
/// entry
/// - Construct `AddressBookEntry`
/// - Update the database (network specs and address book)
/// - Output raw bytes payload files if requested
fn specs_pt_u(
    address: &str,
    encryption: Encryption,
    optional_token_override: Option<TokenOverride>,
    printing: bool,
) -> Result<(), ErrorActive> {
    let shortcut = specs_shortcut(address, encryption.to_owned(), optional_token_override)?;
    if shortcut.update {
        update_db(address, &shortcut.specs)?;
        if printing {
            print_specs(&shortcut.specs)?
        }
    } else if printing {
        print_specs(&shortcut.specs)?
    } else {
        return Err(ErrorActive::Fetch(Fetch::SpecsInDb {
            name: shortcut.specs.name,
            encryption,
        }));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Override;
    use constants::FOLDER;

    // The aim is to check that rpc calls are going through for "officially
    // approved" networks.
    // Also, mass fetcher is neat, and it is nice to have all addresses in one
    // place.
    // However, the fetching results are constantly changing (some networks at
    // times could not be called).
    // So, this is currently limited to three default networks that must be
    // working always.
    #[test]
    fn mass_fetch() {
        let address_set = [
            "wss://rpc.polkadot.io",
            /*            "wss://statemint-rpc.polkadot.io",
                        "wss://acala-polkadot.api.onfinality.io/public-ws",
            //            "wss://wss.odyssey.aresprotocol.io", // error502
                        "wss://rpc.astar.network",
                        "wss://fullnode.parachain.centrifuge.io",
                        "wss://rpc-para.clover.finance",
                        "wss://rpc.efinity.io",
                        "wss://rpc-01.hydradx.io",
            //            "wss://api.interlay.io/parachain", // Base58PrefixMismatch { specs: 2032, meta: 42 }
                        "wss://wss.api.moonbeam.network",
            //            "wss://node-6907995778982338560.sz.onfinality.io/ws?apikey=b5324589-1447-4699-92a6-025bc2cc2ac1", // error500
                        "wss://rpc.parallel.fi",
                        "wss://mainnet.polkadex.trade",
                        "wss://ws.unique.network/",
            */
            "wss://kusama-rpc.polkadot.io",
            /*            "wss://statemine-rpc.polkadot.io",
            //            "wss://encointer.api.onfinality.io/public-ws", // Base58PrefixMismatch { specs: 2, meta: 42 }
                        "wss://fullnode.altair.centrifuge.io",
                        "wss://rpc-01.basilisk.hydradx.io",
                        "wss://bifrost-rpc.liebi.com/ws",
                        "wss://pioneer-1-rpc.bit.country",
            //            "wss://falafel.calamari.systems", // error502
            //            "wss://rpc-shadow.crust.network", //error500
                        "wss://crab-parachain-rpc.darwinia.network/",
                        "wss://kusama.api.integritee.network",
                        "wss://karura.api.onfinality.io/public-ws",
                        "wss://khala-api.phala.network/ws",
            //            "wss://rpc.api.kico.dico.io", // Base58PrefixMismatch { specs: 51, meta: 42 }
                        "wss://spiritnet.kilt.io",
                        "wss://kintsugi.api.onfinality.io/public-ws",
                        "wss://rpc.litmus-parachain.litentry.io",
            //            "wss://wss.mars.aresprotocol.io", // error502
                        "wss://wss.moonriver.moonbeam.network",
                        "wss://heiko-rpc.parallel.fi",
                        "wss://picasso-rpc.composable.finance",
            //            "wss://kusama.kylin-node.co.uk", // Networking or low-level protocol error: Error in the WebSocket handshake: i/o error: unexpected end of file
                        "wss://quartz.unique.network",
                        "wss://kusama.rpc.robonomics.network/",
                        "wss://rpc.shiden.astar.network",
                        "wss://ws.parachain-collator-1.c1.sora2.soramitsu.co.jp",
            //            "wss://gamma.subgame.org/", // error502
                        "wss://para.subsocial.network",
                        "wss://rpc.kusama.standard.tech",
                        "wss://rpc-0.zeitgeist.pm",
            */
            "wss://westend-rpc.polkadot.io",
            /*            "wss://westmint-rpc.polkadot.io",
                        "wss://fullnode-collator.charcoal.centrifuge.io",
                        "wss://teerw1.integritee.network",
                        "wss://westend.kylin-node.co.uk",
                        "wss://rpc.westend.standard.tech",
                        "wss://westend.kilt.io:9977",
                        "wss://rococo-rpc.polkadot.io",
                        "wss://rococo-statemint-rpc.polkadot.io",
                        "wss://rococo-canvas-rpc.polkadot.io",
                        "wss://rococo.api.encointer.org",
                        "wss://rpc-01.basilisk-rococo.hydradx.io",
                        "wss://rpc.rococo.efinity.io",
                        "wss://moonsama-testnet-rpc.moonsama.com",
                        "wss://rococo.kilt.io",
                        "wss://spreehafen.datahighway.com",
                        "wss://ws.azero.dev",
                        "wss://api.ata.network",
            //            "wss://fullnode.centrifuge.io", // metadata below V12
                        "wss://mainnet.chainx.org/ws",
                        "wss://node0.competitors.club/wss",
                        "wss://blockchain.crownsterling.io",
                        "wss://crust.api.onfinality.io/public-ws",
                        "wss://rpc.darwinia.network",
                        "wss://crab-rpc.darwinia.network",
                        "wss://mainnet-node.dock.io",
            //            "wss://edgeware.api.onfinality.io/public-ws", // no version block in metadata
            //            "wss://node.equilibrium.io", // decimals [9,9,9,9,9,9,9] vs units ["Unknown","USD","EQ","ETH","BTC","EOS","DOT","CRV"]
                        "wss://node.genshiro.io",
                        "wss://rpc-01.snakenet.hydradx.io",
                        "wss://api.solo.integritee.io",
                        "wss://rpc.kulupu.corepaper.org/ws",
                        "wss://ws.kusari.network",
                        "wss://mathchain-asia.maiziqianbao.net/ws",
                        "wss://rpc.neatcoin.org/ws",
                        "wss://mainnet.nftmart.io/rpc/ws",
                        "wss://main3.nodleprotocol.io",
            //            "wss://rpc.plasmnet.io", // metadata below V12
                        "wss://mainnet.polkadex.trade",
                        "wss://mainnet-rpc.polymesh.network",
            //            "wss://node.v1.riochain.io", // no version block in metadata
                        "wss://mainnet.sherpax.io",
                        "wss://mof2.sora.org",
                        "wss://mainnet.subgame.org/",
                        "wss://rpc.subsocial.network",
                        "wss://ws.swapdex.network",
            //            "wss://mainnet.uniarts.vip:9443", // Base58PrefixMismatch { specs: 2, meta: 42 }
            //            "wss://westlake.datahighway.com", // error502
                        "wss://rpc-test.ajuna.network",
            //            "wss://ws.test.azero.dev", // units TZERO, no decimals
            //            "wss://fullnode.amber.centrifuge.io", // metadata below 12
                        "wss://arcadia1.nodleprotocol.io",
                        "wss://gladios.aresprotocol.io",
                        "wss://cf-api.ata.network",
            //            "wss://beresheet.edgewa.re", // Base58PrefixMismatch { specs: 42, meta: 7 }
            //            "wss://asgard-rpc.liebi.com/ws", // error502
            //            "wss://tewai-rpc.bit.country", // Base58PrefixMismatch { specs: 28, meta: 42 }
                        "wss://api.crust.network/",
                        "wss://trillian.dolphin.red",
                        "wss://mogiway-01.dotmog.com",
                        "wss://gesell.encointer.org",
                        "wss://galois-hk.maiziqianbao.net/ws",
            //            "wss://gamepower.io", // Networking or low-level protocol error: Connection timeout exceeded: 10s
                        "wss://testnet.geekcash.org",
            //            "wss://api.interlay.io/parachain", // Base58PrefixMismatch { specs: 2032, meta: 42 }
            //            "wss://ws.jupiter-poa.patract.cn", // Error when opening the TCP socket: Connection reset by peer (os error 104)
                        "wss://full-nodes.kilt.io:9944/",
                        "wss://peregrine.kilt.io/parachain-public-ws/",
                        "wss://klugdossier.net/",
            //            "wss://testnet.litentry.io", // error502
                        "wss://mandala.polkawallet.io",
                        "wss://minichain.coming.chat/ws",
                        "wss://wss.api.moonbase.moonbeam.network",
                        "wss://neumann.api.onfinality.io/public-ws",
            //            "wss://staging-ws.nftmart.io", // Error when opening the TCP socket: invalid peer certificate contents: invalid peer certificate: CertExpired
                        "wss://opal.unique.network",
                        "wss://rpc.opportunity.standard.tech",
            //            "wss://parachain-rpc.origin-trail.network", // decimals 18, no units
                        "wss://pangolin-rpc.darwinia.network",
                        "wss://pangoro-rpc.darwinia.network",
                        "wss://poc5.phala.network/ws",
            //            "wss://blockchain.polkadex.trade", // Error when opening the TCP socket: invalid peer certificate contents: invalid peer certificate: CertExpired
                        "wss://testnet-rpc.polymesh.live",
                        "wss://testnet.pontem.network/ws",
            //            "wss://testnet.psm.link", // error502
                        "wss://rpc.realis.network/",
                        "wss://sherpax-testnet.chainx.org",
                        "wss://rpc.shibuya.astar.network",
                        "wss://parachain-rpc.snowbridge.network",
                        "wss://alpha.subdao.org",
                        "wss://staging.subgame.org",
                        "wss://farm-rpc.subspace.network",
                        "wss://test-rpc.subspace.network",
                        "wss://testnet.ternoa.com/",
            //            "wss://testnet-node-1.laminar-chain.laminar.one/ws", // no version block in metadata
            //            "wss://testnet.uniarts.network", // Base58PrefixMismatch { specs: 2, meta: 45 }
                        "wss://testnet2.unique.network",
                        "wss://vodka.rpc.neatcoin.org/ws",
                        "wss://testnet.web3games.org",
            //            "wss://test1.zcloak.network", // error502
                        "wss://bsr.zeitgeist.pm",
            //            "wss://alphaville.zero.io", // error502*/
        ];
        let mut all_clear = true;
        for address in address_set {
            let instruction = Instruction {
                set: Set::D,
                content: Content::Address(address.to_string()),
                pass_errors: true,
                over: Override {
                    encryption: Some(Encryption::Sr25519),
                    token: None,
                },
            };
            match gen_add_specs(instruction) {
                Ok(()) => (),
                Err(e) => {
                    println!("Error: \n{}", e);
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
