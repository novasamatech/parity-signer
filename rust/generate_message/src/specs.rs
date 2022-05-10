//! `add_specs` payloads
//!
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
use crate::metadata_shortcut::meta_specs_shortcut;
use crate::output_prep::print_specs;
use crate::parser::{Content, Instruction, Set, TokenOverride};

/// Function to generate `add_specs` message ready for signing.
/// Exact behavior is determined by the keys used.

pub fn gen_add_specs(instruction: Instruction) -> Result<(), ErrorActive> {
    match instruction.set {
        Set::F => match instruction.content {
            Content::All => {
                if instruction.over.encryption.is_some() {
                    return Err(ErrorActive::NotSupported);
                }
                if instruction.over.token.is_some() {
                    return Err(ErrorActive::NotSupported);
                }
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
                for address_book_entry_encoded in address_book_set.into_iter() {
                    match specs_f_a_element(address_book_entry_encoded) {
                        Ok(()) => (),
                        Err(e) => error_occured(e, instruction.pass_errors)?,
                    }
                }
                Ok(())
            }
            Content::Name(name) => {
                if instruction.over.token.is_some() {
                    return Err(ErrorActive::NotSupported);
                }
                specs_f_n(&name, instruction.over.encryption)
            }
            Content::Address(address) => {
                if instruction.over.token.is_some() {
                    return Err(ErrorActive::NotSupported);
                }
                specs_f_u(&address, instruction.over.encryption)
            }
        },
        Set::D => match instruction.content {
            Content::All => Err(ErrorActive::NotSupported),
            Content::Name(_) => Err(ErrorActive::NotSupported),
            Content::Address(address) => {
                if let Some(encryption) = instruction.over.encryption {
                    specs_d_u(&address, encryption, instruction.over.token)
                } else {
                    Err(ErrorActive::NotSupported)
                }
            }
        },
        Set::K => Err(ErrorActive::NotSupported),
        Set::P => match instruction.content {
            Content::All => Err(ErrorActive::NotSupported),
            Content::Name(name) => {
                if instruction.over.token.is_some() {
                    return Err(ErrorActive::NotSupported);
                }
                if let Some(encryption) = instruction.over.encryption {
                    specs_pt_n(&name, encryption, false)
                } else {
                    Err(ErrorActive::NotSupported)
                }
            }
            Content::Address(address) => {
                if let Some(encryption) = instruction.over.encryption {
                    specs_pt_u(&address, encryption, instruction.over.token, false)
                } else {
                    Err(ErrorActive::NotSupported)
                }
            }
        },
        Set::T => match instruction.content {
            Content::All => Err(ErrorActive::NotSupported),
            Content::Name(name) => {
                if instruction.over.token.is_some() {
                    return Err(ErrorActive::NotSupported);
                }
                if let Some(encryption) = instruction.over.encryption {
                    specs_pt_n(&name, encryption, true)
                } else {
                    Err(ErrorActive::NotSupported)
                }
            }
            Content::Address(address) => {
                if let Some(encryption) = instruction.over.encryption {
                    specs_pt_u(&address, encryption, instruction.over.token, true)
                } else {
                    Err(ErrorActive::NotSupported)
                }
            }
        },
    }
}

/// Function to process individual address book entry in `add_specs -f -a` run.
/// Expected behavior:  
/// generate network key, by network key find network specs in `chainspecs` database tree, print into `sign_me` output file.  
fn specs_f_a_element(entry: (IVec, IVec)) -> Result<(), ErrorActive> {
    let network_specs = network_specs_from_entry(&AddressBookEntry::from_entry(entry)?)?;
    print_specs(&network_specs)
}

/// Function to process `add_specs -f -n title` run.
/// Here `title` is network title from the database, the key for `address_book` entry,
/// i.e. `polkadot` and `polkadot-ed25519` would be different entries
/// (since specs contain info about network encryption).
/// Expected behavior:  
/// get from `address_book` the entry corresponding to the title, generate network key,
/// with it find network specs in `chainspecs` database tree, print into `sign_me` output file.  
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

/// Function to process `add_specs -f -u url` run.
/// Expected behavior for NO encryption override:  
/// go through `address_book` database tree in search of all entries corresponding to url, generate network keys,
/// and with it find network specs in `chainspecs` database tree, print into `sign_me` output files.
/// Expected behavior for encryption override:  
/// go through `address_book` database tree in search of entry: (1) already with correct encryption,
/// (2) the one marked default, (3) any entry corresponding to url;
/// generate network key with old encryption, and with it find network specs in `chainspecs` database tree,
/// generate modified network specs (if not in case (1)) set with encryption override,
/// print into `sign_me` output file.
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

/// Function to process `add_specs -d -u url -encryption` run.
/// Expected behavior:  
/// go through address book in the database and search for given address;
/// if no entries found, do fetch (throw error if chainspecs turn up in the database), print `sign_me` file;
/// if entries found, search for appropriate network specs to modify, and print `sign_me` file.
fn specs_d_u(
    address: &str,
    encryption: Encryption,
    optional_token_override: Option<TokenOverride>,
) -> Result<(), ErrorActive> {
    let shortcut = meta_specs_shortcut(address, encryption, optional_token_override)?;
    print_specs(&shortcut.specs)
}

/// Function to process `add_specs -p -n title -encryption`, `add_specs -t -n title -encryption` and `add_specs -n title -encryption` run.
/// Expected behavior:  
/// get from address book AddressBookEntry#1 corresponding to exact title;
/// generate NetworkSpecsKey#1 using encryption from AddressBookEntry#1,  
/// search through `chainspecs` tree for network specs NetworkSpecsToSend#1,
/// if the encryption is matching, print `sign_me` file according to the key;
/// if not, generate NetworkSpecsKey#2 using override encryption,  
/// search through `chainspecs` tree for NetworkSpecsKey#2: if found, do nothing with database (network specs are already
/// in place meaning address book also should be in place and was not found only because title used in query was not exact fit),
/// print `sign_me` file according to the key;
/// if not found:
/// (1) modify NetworkSpecsToSend#1 (encryption and title fields) and insert in `chainspecs` tree with NetworkSpecsKey#2,
/// (2) modify AddressBookEntry#1 (encryption and `def = false`) and insert in `address_book` tree with encoded `name-encryption` as a key  
/// and print `sign_me` file according to the key;
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

/// Function to process `add_specs -p -u url -encryption`, `add_specs -t -u url -encryption` and `add_specs -u url -encryption` run.
/// Expected behavior:  
/// get from address book set of entries corresponding to given url address;
/// if no entries found, the network is new, and network specs are fetched;
/// if there are entries, search for appropriate network specs to modify, print `sign_me` file according to the key and update the database.
fn specs_pt_u(
    address: &str,
    encryption: Encryption,
    optional_token_override: Option<TokenOverride>,
    printing: bool,
) -> Result<(), ErrorActive> {
    let shortcut = meta_specs_shortcut(address, encryption.to_owned(), optional_token_override)?;
    if shortcut.update {
        update_db(address, &shortcut.specs)?;
        if printing {
            print_specs(&shortcut.specs)?
        }
    } else if printing {
        print_specs(&shortcut.specs)?
    } else {
        return Err(ErrorActive::Fetch(Fetch::SpecsInDb {
            name: shortcut.meta_values.name,
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
    // Argh?
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
