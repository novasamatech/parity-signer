use constants::{METATREE, SPECSTREE};
use definitions::keyring::{MetaKey, MetaKeyPrefix, NetworkSpecsKey, VerifierKey};
use blake2_rfc::blake2b::blake2b;
use hex;
use anyhow;

use crate::error::{Error, NotFound, NotDecodeable, NotHex};
use crate::helpers::{open_db, open_tree, unhex, decode_chain_specs, check_metadata, get_current_verifier, get_general_verifier};

struct MetaPrint {
    spec_version: u32,
    metadata_hash: String,
}

pub fn get_network_details_by_key (network_key: &NetworkSpecsKey, database_name: &str) -> anyhow::Result<String> {
    
    let network_specs = {
        let database = open_db(database_name)?;
        let chainspecs = open_tree(&database, SPECSTREE)?;
        match chainspecs.get(network_key.key()) {
            Ok(Some(network_specs_encoded)) => decode_chain_specs(network_specs_encoded, network_key)?,
            Ok(None) => return Err(Error::NotFound(NotFound::NetworkSpecsKey).show()),
            Err(e) => return Err(Error::InternalDatabaseError(e).show()),
        }
    };
    let verifier_key = VerifierKey::from_parts(&network_specs.genesis_hash.to_vec());
    let general_verifier = get_general_verifier(&database_name)?;
    let current_verifier = get_current_verifier(&verifier_key, &database_name)?;
    
    let database = open_db(database_name)?;
    let metadata = open_tree(&database, METATREE)?;
    let mut relevant_metadata: Vec<MetaPrint> = Vec::new();
    let meta_key_prefix = MetaKeyPrefix::from_name(&network_specs.name);
    
    for x in metadata.scan_prefix(meta_key_prefix.prefix()) {
        if let Ok((meta_key_vec, meta)) = x {
            let (name, version) = match MetaKey::from_vec(&meta_key_vec.to_vec()).name_version() {
                Ok(a) => a,
                Err(_) => return Err(Error::NotDecodeable(NotDecodeable::NameVersioned).show()),
            };
            let meta = check_metadata(meta.to_vec(), &name, version)?;
            let new = MetaPrint {
                spec_version: version,
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
    Ok(format!("{{{},\"meta\":{}}}", network_specs.show(&current_verifier, &general_verifier), metadata_print))
}


pub fn get_network_details_by_hex (network_specs_key_string: &str, database_name: &str) -> anyhow::Result<String> {
    let network_specs_key = NetworkSpecsKey::from_vec(&unhex(network_specs_key_string, NotHex::NetworkSpecsKey)?);
    get_network_details_by_key (&network_specs_key, database_name)
}


#[cfg(test)]
mod tests {
    use crate::cold_default::reset_cold_database_no_addresses;
    use super::*;
    use std::fs;
    use definitions::{crypto::Encryption, keyring::NetworkSpecsKey, network_specs::Verifier};
    
    #[test]
    fn print_westend() {
        let dbname = "tests/print_westend";
        reset_cold_database_no_addresses(dbname, Verifier(None)).unwrap();
        
        let network_specs_key_string = hex::encode(NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").expect("known value"), &Encryption::Sr25519).key());
        assert!(network_specs_key_string == "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e", "\nExpected:\n0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e\nReceived:\n{}", network_specs_key_string);
        let print = get_network_details_by_hex(&network_specs_key_string, dbname).unwrap();
        let print_expected = r##"{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"2","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","current_verifier":{"type":"general","details":{"hex":"","encryption":"none"}},"meta":[{"spec_version":"9000","meta_hash":"e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce"},{"spec_version":"9010","meta_hash":"70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf"}]}"##;
        assert!(print == print_expected, "\nExpected:\n{}\nReceived:\n{}", print_expected, print);
        
        fs::remove_dir_all(dbname).unwrap();
    }

}

