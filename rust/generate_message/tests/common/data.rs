use constants::TYPES;
use defaults::{default_types_content, test_metadata};
use definitions::crypto::Encryption;
use definitions::keyring::{AddressBookKey, MetaKey, NetworkSpecsKey};
use definitions::metadata::AddressBookEntry;
use definitions::network_specs::NetworkSpecsToSend;
use parity_scale_codec::Encode;
use sled::Batch;
use sp_core::H256;
use std::str::FromStr;

fn test_specs() -> NetworkSpecsToSend {
    NetworkSpecsToSend {
        base58prefix: 0,
        color: String::from("#E6027A"),
        decimals: 10,
        encryption: Encryption::Sr25519,
        genesis_hash: H256::from_str(
            "91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3",
        )
        .unwrap(),
        logo: String::from("polkadot"),
        name: String::from("polkadot"),
        path_id: String::from("//polkadot"),
        secondary_color: String::from("#262626"),
        title: String::from("Polkadot"),
        unit: String::from("DOT"),
    }
}

pub(crate) fn metadata() -> Batch {
    let mut batch = Batch::default();
    let metadata_set = test_metadata().unwrap();
    for x in metadata_set.iter() {
        let meta_key = MetaKey::from_parts(&x.name, x.version);
        batch.insert(meta_key.key(), &x.meta[..]);
    }
    batch
}

pub(crate) fn address_book() -> Batch {
    let specs = test_specs();
    let mut batch = Batch::default();
    batch.insert(
        AddressBookKey::from_title(&specs.name).key(),
        AddressBookEntry {
            name: specs.name,
            genesis_hash: specs.genesis_hash,
            address: String::from("wss://rpc.polkadot.io"),
            encryption: specs.encryption,
            def: false,
        }
        .encode(),
    );
    batch
}

pub fn network_specs_prep() -> Batch {
    let specs = test_specs();
    let mut batch = Batch::default();
    batch.insert(
        NetworkSpecsKey::from_parts(&specs.genesis_hash, &specs.encryption).key(),
        specs.encode(),
    );
    batch
}

pub(crate) fn settings() -> Batch {
    let mut batch = Batch::default();
    let types_prep = default_types_content().unwrap();
    batch.insert(TYPES, types_prep.store());
    batch
}
