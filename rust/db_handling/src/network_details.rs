use anyhow;
use blake2_rfc::blake2b::blake2b;
use constants::SPECSTREE;
use definitions::{error::{ErrorSigner, Signer}, keyring::{NetworkSpecsKey, VerifierKey}, network_specs::{NetworkSpecs}, print::{export_complex_single, export_complex_vector}};
use hex;

use crate::helpers::{get_valid_current_verifier, get_general_verifier, get_meta_values_by_name, get_network_specs, open_db, open_tree};

/// Fetch NetworkSpecs for 1 network from Signer database by network key (genesis hash and encryption)
/// Applicable only to the Signer side.
pub fn get_network_specs_by_hex_key (database_name: &str, network_specs_key_string: &str) -> Result<NetworkSpecs, ErrorSigner> {
    let network_specs_key = NetworkSpecsKey::from_hex(network_specs_key_string)?;
    get_network_specs(database_name, &network_specs_key)
}

/// Print network details for 1 network from Signer database by genesis hash.
/// Applicable only to the Signer side.
/// Function gets called from user interface.
pub fn print_network (database_name: &str, network_specs_key_string: &str) -> anyhow::Result<String> {
    let network_specs = get_network_specs_by_hex_key(database_name, network_specs_key_string).map_err(|e| e.anyhow())?;
    Ok(export_complex_single(&network_specs, |a| a.print_single()))
}

/// Function to get network specs for all networks in Signer database.
/// Applicable only to the Signer side.
pub fn get_all_networks (database_name: &str) -> Result<Vec<NetworkSpecs>, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
    let mut out: Vec<NetworkSpecs> = Vec::new();
    for x in chainspecs.iter() {if let Ok(a) = x {out.push(NetworkSpecs::from_entry_checked::<Signer>(a)?)}}
    Ok(out)
}

/// Print details for all networks in Signer database.
/// Applicable only to the Signer side.
/// Function gets called from user interface.
pub fn print_all_networks (database_name: &str) -> anyhow::Result<String> {
    let mut network_specs_vec = get_all_networks(database_name).map_err(|e| e.anyhow())?;
    network_specs_vec.sort_by(|a, b| a.order.cmp(&b.order));
    Ok(export_complex_vector(&network_specs_vec, |a| a.print_as_set_part()))
}

/// Print network specs and metadata set information for network with given network specs key.
/// Applicable only to the Signer side.
/// Function gets called from user interface.
pub fn get_network_details_by_hex (database_name: &str, network_specs_key_string: &str) -> anyhow::Result<String> {
    let network_specs = get_network_specs_by_hex_key(database_name, network_specs_key_string).map_err(|e| e.anyhow())?;
    let verifier_key = VerifierKey::from_parts(&network_specs.genesis_hash.to_vec());
    let general_verifier = get_general_verifier(&database_name).map_err(|e| e.anyhow())?;
    let valid_current_verifier = get_valid_current_verifier(&verifier_key, &database_name).map_err(|e| e.anyhow())?;
    let relevant_meta = get_meta_values_by_name::<Signer>(database_name, &network_specs.name).map_err(|e| e.anyhow())?;
    let metadata_print = export_complex_vector(&relevant_meta, |a| format!("\"spec_version\":\"{}\",\"meta_hash\":\"{}\"", a.version, hex::encode(blake2b(32, &[], &a.meta).as_bytes())));
    Ok(format!("{{{},\"meta\":{}}}", network_specs.show(&valid_current_verifier, &general_verifier), metadata_print))
}

#[cfg(test)]
mod tests {
    
    use super::*;
    
    use crate::cold_default::{populate_cold, populate_cold_no_metadata};
    use defaults::get_default_chainspecs;
    use definitions::{crypto::Encryption, keyring::NetworkSpecsKey, network_specs::Verifier};
    use std::fs;
    
    #[test]
    fn print_westend_network_specs() {
        let dbname = "for_tests/print_westend_network_specs";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let network_specs_default = get_default_chainspecs();
        let mut westend_network_specs_key = None;
        for x in network_specs_default.iter() {
            if x.name == "westend" {
                westend_network_specs_key = Some(NetworkSpecsKey::from_parts(&x.genesis_hash.to_vec(), &x.encryption));
                break;
            }
        }
        let westend_network_specs_key_hex = hex::encode(westend_network_specs_key.unwrap().key());
        let print = print_network(dbname, &westend_network_specs_key_hex).unwrap();
        let print_expected = r##"{"color":"#660D35","logo":"westend","secondary_color":"#262626","title":"Westend"}"##;
        assert!(print == print_expected, "Received print: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn print_all_network_specs() {
        let dbname = "for_tests/print_all_network_specs";
        populate_cold_no_metadata(dbname, Verifier(None)).unwrap();
        let print = print_all_networks(dbname).unwrap();
        let print_expected = r##"[{"key":"018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3","color":"#E6027A","logo":"polkadot","order":"0","secondary_color":"#262626","title":"Polkadot"},{"key":"0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe","color":"#000","logo":"kusama","order":"1","secondary_color":"#262626","title":"Kusama"},{"key":"0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","color":"#660D35","logo":"westend","order":"2","secondary_color":"#262626","title":"Westend"}]"##;
        assert!(print == print_expected, "Received print: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn print_westend() {
        let dbname = "for_tests/print_westend";
        populate_cold(dbname, Verifier(None)).unwrap();
        let network_specs_key_string = hex::encode(NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").expect("known value"), &Encryption::Sr25519).key());
        assert!(network_specs_key_string == "0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e", "\nExpected:\n0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e\nReceived:\n{}", network_specs_key_string);
        let print = get_network_details_by_hex(dbname, &network_specs_key_string).unwrap();
        let print_expected = r##"{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"2","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","current_verifier":{"type":"general","details":{"hex":"","identicon":"","encryption":"none"}},"meta":[{"spec_version":"9000","meta_hash":"e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce"},{"spec_version":"9010","meta_hash":"70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf"}]}"##;
        assert!(print == print_expected, "\nExpected:\n{}\nReceived:\n{}", print_expected, print);
        fs::remove_dir_all(dbname).unwrap();
    }
}
