use constants::{METATREE, SPECSTREE, VERIFIERS};
use definitions::{metadata::NameVersioned, network_specs::NetworkKey};
use parity_scale_codec::{Decode, Encode};
use blake2_rfc::blake2b::blake2b;
use hex;
use anyhow;

use crate::error::{Error, NotFound, NotDecodeable, NotHex};
use crate::helpers::{open_db, open_tree, unhex, decode_chain_specs, check_metadata, get_verifier};

struct MetaPrint {
    spec_version: u32,
    metadata_hash: String,
}

pub fn get_network_details_by_key (network_key: &NetworkKey, database_name: &str) -> anyhow::Result<String> {
    
    let database = open_db(database_name)?;
    let metadata = open_tree(&database, METATREE)?;
    let chainspecs = open_tree(&database, SPECSTREE)?;
    let verifiers = open_tree(&database, VERIFIERS)?;
        
    match chainspecs.get(&network_key) {
        Ok(Some(network_specs_encoded)) => {
            let network_specs = decode_chain_specs(network_specs_encoded, &network_key.to_vec())?;
            let network_verifier = get_verifier(network_specs.genesis_hash, &verifiers)?;
            let mut relevant_metadata: Vec<MetaPrint> = Vec::new();
            for x in metadata.scan_prefix(network_specs.name.encode()) {
                if let Ok((versioned_name_encoded, meta)) = x {
                    let versioned_name = match <NameVersioned>::decode(&mut &versioned_name_encoded[..]) {
                        Ok(a) => a,
                        Err(_) => return Err(Error::NotDecodeable(NotDecodeable::NameVersioned).show()),
                    };
                    let meta = check_metadata(meta.to_vec(), &versioned_name)?;
                    let new = MetaPrint {
                        spec_version: versioned_name.version,
                        metadata_hash: hex::encode(blake2b(32, &[], &meta).as_bytes()),
                    };
                    relevant_metadata.push(new);
                }
            }
            let mut metadata_print = String::from("[");
            for (i,x) in relevant_metadata.iter().enumerate() {
                if i > 0 {metadata_print.push_str(",")}
                metadata_print.push_str(&format!("{{\"spec_version\":\"{}\",\"meta_hash\":\"{}\"}}", x.spec_version, x.metadata_hash));
            }
            metadata_print.push_str("]");
            Ok(format!("{{{},\"meta\":{}}}", network_specs.show(&network_verifier), metadata_print))
        },
        Ok(None) => return Err(Error::NotFound(NotFound::NetworkKey).show()),
        Err(e) => return Err(Error::InternalDatabaseError(e).show()),
    }
}


pub fn get_network_details_by_hex (network_key_string: &str, database_name: &str) -> anyhow::Result<String> {
    
    let network_key = unhex(network_key_string, NotHex::NetworkKey)?;
    get_network_details_by_key (&network_key, database_name)
    
}


#[cfg(test)]
mod tests {
    use crate::{populate_cold};
    use super::*;
    use std::fs;
    use definitions::{crypto::Encryption, network_specs::generate_network_key};
    
    const METADATA_FILE: &str = "metadata_database.ts";

    #[test]
    fn print_westend() {
        let dbname = "tests/print_westend";
        populate_cold(dbname, METADATA_FILE, true).unwrap();
        
        let network_key_string = hex::encode(generate_network_key(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").expect("known value"), Encryption::Sr25519));
        let print = get_network_details_by_hex(&network_key_string, dbname).unwrap();
        let print_expected = r##"{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"2","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","verifier":{"hex":"","encryption":"none"},"meta":[{"spec_version":"9000","meta_hash":"e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce"},{"spec_version":"9010","meta_hash":"70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf"}]}"##;
        assert!(print == print_expected, "\nExpected:\n{}\nReceived:\n{}", print_expected, print);
        
        fs::remove_dir_all(dbname).unwrap();
    }

}

