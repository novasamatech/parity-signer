use std::convert::TryInto;

pub struct AddressBookEntry <'a> {
    pub name: &'a str,
    pub genesis_hash: [u8; 32],
    pub address: &'a str,
}

pub fn get_default_address_book() -> Vec<AddressBookEntry <'static>> {
    vec![
        AddressBookEntry {
            name: "kusama",
            genesis_hash: hex::decode("b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe").expect("known value").try_into().expect("known value"),
            address: "wss://kusama-rpc.polkadot.io",
        },
        AddressBookEntry {
            name: "polkadot",
            genesis_hash: hex::decode("91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3").expect("known value").try_into().expect("known value"),
            address: "wss://rpc.polkadot.io",
        },
        AddressBookEntry {
            name: "rococo",
            genesis_hash: hex::decode("e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779").expect("known value").try_into().expect("known value"),
            address: "wss://rococo-rpc.polkadot.io",
        },
        AddressBookEntry {
            name: "westend",
            genesis_hash: hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").expect("known value").try_into().expect("known value"),
            address: "wss://westend-rpc.polkadot.io",
        },
    ]
}
