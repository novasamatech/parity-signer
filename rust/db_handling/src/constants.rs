/// Default database values for initialization and factory reset

use std::convert::TryInto;
use super::chainspecs::{ChainSpecs, Verifier};

pub const SPECSTREE: &[u8] = b"chainspecs";
pub const METATREE: &[u8] = b"metadata";
pub const ADDRTREE: &[u8] = b"addresses";
pub const IDTREE: &[u8] = b"seeds";
pub const SETTREE: &[u8] = b"settings";
pub const TYPES: &[u8] = b"types";
pub const TYPESVERIFIER: &[u8] = b"types_verifier";
pub const SIGNTRANS: &[u8] = b"sign_transaction";
pub const LOADMETA: &[u8] = b"load_metadata";
pub const ADDMETAVERIFIER: &[u8] = b"add_metadata_verifier";
pub const LOADTYPES: &[u8] = b"load_types";
pub const ADDTYPESVERIFIER: &[u8] = b"add_types_verifier";

pub fn get_default_chainspecs() -> Vec<ChainSpecs> {
    vec![
        ChainSpecs {
            base58prefix: 2,
            color: String::from("#000"),
            decimals: 12,
            genesis_hash: hex::decode("b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe").unwrap().try_into().unwrap(),
            logo: String::from("kusama"),
            name: String::from("kusama"),
            order: 2,
            path_id: String::from("//kusama"),
            secondary_color: String::from("#262626"),
            title: String::from("Kusama"),
            unit: String::from("KSM"),
            verifier: Verifier::None,
    	},
	ChainSpecs {
            base58prefix: 0,
            color: String::from("#E6027A"),
            decimals: 10,
            genesis_hash: hex::decode("91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3").unwrap().try_into().unwrap(),
            logo: String::from("polkadot"),
            name: String::from("polkadot"),
            order: 1,
            path_id: String::from("//polkadot"),
            secondary_color: String::from("#262626"),
            title: String::from("Polkadot"),
            unit: String::from("DOT"),
            verifier: Verifier::None,
    	},
	ChainSpecs {
            base58prefix: 42,
            color: String::from("#6f36dc"),
            decimals: 12,
            genesis_hash: hex::decode("e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779").unwrap().try_into().unwrap(),
            logo: String::from("rococo"),
            name: String::from("rococo"),
            order: 4,
            path_id: String::from("//rococo"),
            secondary_color: String::from("#262626"),
            title: String::from("Rococo"),
            unit: String::from("ROC"),
            verifier: Verifier::None,
    	},
        ChainSpecs {
            base58prefix: 42,
            color: String::from("#660D35"),
            decimals: 12,
            genesis_hash: hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap().try_into().unwrap(),
            logo: String::from("westend"),
            name: String::from("westend"),
            order: 3,
            path_id: String::from("//westend"),
            secondary_color: String::from("#262626"),
            title: String::from("Westend"),
            unit: String::from("WND"),
            verifier: Verifier::None,
        },
    ]
}


