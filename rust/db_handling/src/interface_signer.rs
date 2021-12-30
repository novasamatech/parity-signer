use blake2_rfc::blake2b::blake2b;
use hex;
use parity_scale_codec::Encode;
use sp_core::{Pair, sr25519};
use sp_runtime::MultiSigner;
use std::collections::HashMap;

use constants::HISTORY;
use definitions::{error::{AddressGenerationCommon, DatabaseSigner, ErrorSigner, ErrorSource, InterfaceSigner, NotFoundSigner, Signer}, helpers::{make_identicon_from_multisigner, multisigner_to_encryption, multisigner_to_public, pic_meta}, keyring::{AddressKey, NetworkSpecsKey, print_multisigner_as_base58, VerifierKey}, network_specs::NetworkSpecs, print::export_complex_vector, qr_transfers::ContentLoadTypes, users::AddressDetails};
use qrcode_static::png_qr_from_string;

use crate::helpers::{get_address_details, get_general_verifier, get_meta_values_by_name, get_meta_values_by_name_version, get_network_specs, get_valid_current_verifier, open_db, open_tree, try_get_types};
use crate::identities::{get_all_addresses, get_addresses_by_seed_name, generate_random_phrase};
use crate::network_details::get_all_networks;

/// Function to print all seed names with identicons
/// Gets used only on the Signer side, interacts with the user interface.
pub fn print_all_seed_names_with_identicons (database_name: &str, names_phone_knows: &Vec<String>) -> Result<String, ErrorSigner> {
    let mut data_set: HashMap<String, Vec<MultiSigner>> = HashMap::new();
    for (multisigner, address_details) in get_all_addresses(database_name)?.into_iter() {
        if (address_details.path == "")&&(!address_details.has_pwd) {
        // found a root; could be any of the supported encryptions;
            match data_set.get(&address_details.seed_name) {
                Some(root_set) => {
                    for id in root_set.iter() {
                        if multisigner_to_encryption(id) == address_details.encryption {
                            return Err(ErrorSigner::Database(DatabaseSigner::TwoRootKeys{seed_name: address_details.seed_name.to_string(), encryption: address_details.encryption.to_owned()}))
                        }
                    }
                    let mut new_root_set = root_set.to_vec();
                    new_root_set.push(multisigner);
                    data_set.insert(address_details.seed_name.to_string(), new_root_set);
                },
                None => {data_set.insert(address_details.seed_name.to_string(), vec![multisigner]);},
            }
        }
        else {if let None = data_set.get(&address_details.seed_name) {data_set.insert(address_details.seed_name.to_string(), Vec::new());}}
    }
    for x in names_phone_knows.iter() {if let None = data_set.get(x) {data_set.insert(x.to_string(), Vec::new());}}
    let mut print_set: Vec<(String, String)> = Vec::new();
    for (seed_name, multisigner_set) in data_set.into_iter() {
        let identicon_string = preferred_multisigner_identicon(&multisigner_set);
        print_set.push((identicon_string, seed_name))
    }
    print_set.sort_by(|a, b| a.1.cmp(&b.1));
    Ok(export_complex_vector(&print_set, |(identicon_string, seed_name)| format!("\"identicon\":\"{}\",\"seed_name\":\"{}\"", identicon_string, seed_name)))
}

fn preferred_multisigner_identicon(multisigner_set: &Vec<MultiSigner>) -> String {
    if multisigner_set.len() == 0 {String::new()}
    else {
        let mut got_sr25519 = None;
        let mut got_ed25519 = None;
        let mut got_ecdsa = None;
        for x in multisigner_set.iter() {
            match x {
                MultiSigner::Ed25519(_) => got_ed25519 = Some(x.to_owned()),
                MultiSigner::Sr25519(_) => got_sr25519 = Some(x.to_owned()),
                MultiSigner::Ecdsa(_) => got_ecdsa = Some(x.to_owned()),
            }
        }
        if let Some(a) = got_sr25519 {
            match make_identicon_from_multisigner(&a) {
                Ok(b) => hex::encode(b),
                Err(_) => String::new()
            }
        }
        else {
            if let Some(a) = got_ed25519 {
                match make_identicon_from_multisigner(&a) {
                    Ok(b) => hex::encode(b),
                    Err(_) => String::new()
                }
            }
            else {
                if let Some(a) = got_ecdsa {
                    match make_identicon_from_multisigner(&a) {
                        Ok(b) => hex::encode(b),
                        Err(_) => String::new()
                    }
                }
                else {String::new()}
            }
        }
    }
}

/// Function to print all identities (seed names AND derication paths) with identicons
/// Gets used only on the Signer side, interacts with the user interface.
pub fn print_all_identities (database_name: &str) -> Result<String, ErrorSigner> {
    Ok(export_complex_vector(&get_all_addresses(database_name)?, |(multisigner, address_details)| {
        let address_key = AddressKey::from_multisigner(&multisigner); // to click
        let public_key = multisigner_to_public(&multisigner); // to display
        let hex_identicon = match make_identicon_from_multisigner(&multisigner) {
            Ok(a) => hex::encode(a),
            Err(_) => String::new(),
        };
        format!("\"seed_name\":\"{}\",\"address_key\":\"{}\",\"public_key\":\"{}\",\"identicon\":\"{}\",\"has_pwd\":{},\"path\":\"{}\"", address_details.seed_name, hex::encode(address_key.key()), hex::encode(public_key), hex_identicon, address_details.has_pwd, address_details.path)
    }))
}

/// Function to print separately root identity and derived identities for given seed name and network specs key.
/// Is used only on the Signer side, interacts only with navigation.
pub fn print_identities_for_seed_name_and_network (database_name: &str, seed_name: &str, network_specs_key: &NetworkSpecsKey, swiped_key: Option<MultiSigner>, multiselect: Vec<MultiSigner>) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let identities = addresses_set_seed_name_network (database_name, seed_name, network_specs_key)?;
    let mut root_id = None;
    let mut other_id: Vec<(MultiSigner, AddressDetails, Vec<u8>, bool, bool)> = Vec::new();
    for (multisigner, address_details) in identities.into_iter() {
        let identicon = make_identicon_from_multisigner(&multisigner)?;
        let base58 = print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix));
        let address_key = AddressKey::from_multisigner(&multisigner);
        let swiped = {
            if let Some(ref swiped_multisigner) = swiped_key {
                if swiped_multisigner == &multisigner {true}
                else {false}
            }
            else {false}
        };
        let multiselect = {
            if multiselect.contains(&multisigner) {true}
            else {false}
        };
        if (address_details.path == "")&&(!address_details.has_pwd) {
            if let Some(_) = root_id {return Err(ErrorSigner::Database(DatabaseSigner::TwoRootKeys{seed_name: seed_name.to_string(), encryption: network_specs.encryption.to_owned()}))}
            root_id = Some(format!("\"seed_name\":\"{}\",\"identicon\":\"{}\",\"address_key\":\"{}\",\"base58\":\"{}\",\"swiped\":{},\"multiselect\":{}", seed_name, hex::encode(identicon), hex::encode(address_key.key()), base58, swiped, multiselect));
        }
        else {other_id.push((multisigner, address_details, identicon, swiped, multiselect))}
    }
    let root_print = match root_id {
        Some(a) => a,
        None => format!("\"seed_name\":\"{}\",\"identicon\":\"\",\"address_key\":\"\",\"base58\":\"\",\"swiped\":false,\"multiselect\":false", seed_name),
    };
    let other_print = export_complex_vector(&other_id, |(multisigner, address_details, identicon, swiped, multiselect)| format!("\"address_key\":\"{}\",\"base58\":\"{}\",\"identicon\":\"{}\",\"has_pwd\":{},\"path\":\"{}\",\"swiped\":{},\"multiselect\":{}", hex::encode(AddressKey::from_multisigner(&multisigner).key()), print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix)), hex::encode(identicon), address_details.has_pwd, address_details.path, swiped, multiselect));
    
    Ok(format!("\"root\":{{{}}},\"set\":{},\"network\":{{\"title\":\"{}\",\"logo\":\"{}\"}}", root_print, other_print, network_specs.title, network_specs.logo))
}

/// Function to get addresses for given seed name and network specs key
pub fn addresses_set_seed_name_network (database_name: &str, seed_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<Vec<(MultiSigner, AddressDetails)>, ErrorSigner> {
    Ok(get_addresses_by_seed_name(database_name, seed_name)?
        .into_iter()
        .filter(|(_, address_details)| address_details.network_id.contains(network_specs_key))
        .collect())
}

/// Function to print all networks, with bool indicator which one is currently selected
pub fn show_all_networks_with_flag (database_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<String, ErrorSigner> {
    let mut networks = get_all_networks(database_name)?;
    networks.sort_by(|a, b| a.order.cmp(&b.order));
    Ok(format!("\"networks\":{}", export_complex_vector(&networks, |a| 
        {
            let network_specs_key_current = NetworkSpecsKey::from_parts(&a.genesis_hash.to_vec(), &a.encryption);
            format!("\"key\":\"{}\",\"title\":\"{}\",\"logo\":\"{}\",\"order\":{},\"selected\":{}", hex::encode(network_specs_key_current.key()), a.title, a.logo, a.order, &network_specs_key_current == network_specs_key)
        }
    )))
}

/// Function to print all networks without any selection
pub fn show_all_networks (database_name: &str) -> Result<String, ErrorSigner> {
    let mut networks = get_all_networks(database_name)?;
    networks.sort_by(|a, b| a.order.cmp(&b.order));
    Ok(format!("\"networks\":{}", export_complex_vector(&networks, |a| 
        {
            let network_specs_key_current = NetworkSpecsKey::from_parts(&a.genesis_hash.to_vec(), &a.encryption);
            format!("\"key\":\"{}\",\"title\":\"{}\",\"logo\":\"{}\",\"order\":{}", hex::encode(network_specs_key_current.key()), a.title, a.logo, a.order)
        }
    )))
}

/// Function to sort networks by the order and get the network specs for the first network on the list.
/// If no networks in the system, throws error
pub fn first_network (database_name: &str) -> Result<NetworkSpecs, ErrorSigner> {
    let mut networks = get_all_networks(database_name)?;
    if networks.len() == 0 {return Err(ErrorSigner::NoNetworksAvailable)}
    networks.sort_by(|a, b| a.order.cmp(&b.order));
    Ok(networks.remove(0))
}

/// Function to prepare the export key screen.
/// Contains among else the QR code with identity information, in format 
/// `substrate:{public_key as as_base58}:0x{network_key}`,
/// this string is transformed into bytes, then into png qr code, then qr code
/// content is hexed so that it could be transferred into app.
pub fn export_key (database_name: &str, multisigner: &MultiSigner, expected_seed_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, &network_specs_key)?;
    let address_key = AddressKey::from_multisigner(&multisigner);
    let address_details = get_address_details(database_name, &address_key)?;
    if address_details.seed_name != expected_seed_name {return Err(ErrorSigner::Interface(InterfaceSigner::SeedNameNotMatching{address_key, expected_seed_name: expected_seed_name.to_string(), real_seed_name: address_details.seed_name}))}
    let address_base58 = print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix));
    let public_key = multisigner_to_public(&multisigner);
    let identicon = make_identicon_from_multisigner(&multisigner)?;
    let qr_prep = {
        if address_details.network_id.contains(&network_specs_key) {
            match png_qr_from_string(&format!("substrate:{}:0x{}", address_base58, hex::encode(&network_specs.genesis_hash))) {
                Ok(a) => a,
                Err(e) => return Err(ErrorSigner::Qr(e.to_string())),
            }
        }
        else {return Err(ErrorSigner::NotFound(NotFoundSigner::NetworkSpecsKeyForAddress{network_specs_key: network_specs_key.to_owned(), address_key}))}
    };
    Ok(format!("\"qr\":\"{}\",\"pubkey\":\"{}\",\"base58\":\"{}\",\"identicon\":\"{}\",\"seed_name\":\"{}\",\"path\":\"{}\",\"network_title\":\"{}\",\"network_logo\":\"{}\"", hex::encode(qr_prep), hex::encode(public_key), address_base58, hex::encode(identicon), address_details.seed_name, address_details.path, network_specs.title, network_specs.logo))
}

/// Function to prepare seed backup screen.
/// Gets seed name, outputs all known derivations in all networks.
pub fn backup_prep (database_name: &str, seed_name: &str) -> Result<String, ErrorSigner> {
    let networks = get_all_networks(database_name)?;
    if networks.len() == 0 {return Err(ErrorSigner::NoNetworksAvailable)}
    let mut export: Vec<(NetworkSpecs, Vec<AddressDetails>)> = Vec::new();
    for x in networks.into_iter() {
        let id_set = addresses_set_seed_name_network (database_name, seed_name, &NetworkSpecsKey::from_parts(&x.genesis_hash.to_vec(), &x.encryption))?;
        if id_set.len() != 0 {export.push((x, id_set.into_iter().map(|(_, a)| a).collect()))}
    }
    export.sort_by(|(a, _), (b, _)| a.order.cmp(&b.order));
    Ok(format!("\"seed_name\":\"{}\",\"derivations\":{}", seed_name, export_complex_vector(&export, |(specs, id_set)| format!("\"network_title\":\"{}\",\"network_logo\":\"{}\",\"network_order\":{},\"id_set\":{}", specs.title, specs.logo, specs.order, export_complex_vector(&id_set, |a| format!("\"path\":\"{}\",\"has_pwd\":{}", a.path, a.has_pwd))))))
}

/// Function to prepare key derivation screen.
/// Gets seed name, network specs key and suggested derivation
pub fn derive_prep (database_name: &str, seed_name: &str, network_specs_key: &NetworkSpecsKey, suggest: &str) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    Ok(format!("\"seed_name\":\"{}\",\"network_title\":\"{}\",\"network_logo\":\"{}\",\"suggested_derivation\":\"{}\"", seed_name, network_specs.title, network_specs.logo, suggest))
}

/// Print network specs and metadata set information for network with given network specs key.
pub fn network_details_by_key (database_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let verifier_key = VerifierKey::from_parts(&network_specs.genesis_hash.to_vec());
    let general_verifier = get_general_verifier(&database_name)?;
    let valid_current_verifier = get_valid_current_verifier(&verifier_key, &database_name)?;
    let relevant_meta = get_meta_values_by_name::<Signer>(database_name, &network_specs.name)?;
    let metadata_print = export_complex_vector(&relevant_meta, |a| {
        let meta_hash = blake2b(32, &[], &a.meta).as_bytes().to_vec();
        let hex_id_pic = match pic_meta(&meta_hash) {
            Ok(a) => hex::encode(a),
            Err(_) => String::new(),
        };
        format!("\"spec_version\":\"{}\",\"meta_hash\":\"{}\",\"meta_id_pic\":\"{}\"", a.version, hex::encode(meta_hash), hex_id_pic)
    });
    Ok(format!("{},\"meta\":{}", network_specs.show(&valid_current_verifier, &general_verifier), metadata_print))
}

/// Print metadata details for given network specs key and version.
pub fn metadata_details (database_name: &str, network_specs_key: &NetworkSpecsKey, network_version: u32) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let meta_values = get_meta_values_by_name_version::<Signer>(database_name, &network_specs.name, network_version)?;
    let relevant_networks = get_all_networks(database_name)?
        .into_iter()
        .filter(|a| a.name == network_specs.name)
        .collect()
    ;
    let relevant_networks_print = export_complex_vector(&relevant_networks, |a| {
        format!("\"title\":\"{}\",\"logo\":\"{}\",\"order\":{},\"current_on_screen\":{}", a.title, a.logo, a.order, &NetworkSpecsKey::from_parts(&a.genesis_hash.to_vec(), &a.encryption) == network_specs_key)
    });
    let meta_hash = blake2b(32, &[], &meta_values.meta).as_bytes().to_vec();
    let hex_id_pic = match pic_meta(&meta_hash) {
        Ok(a) => hex::encode(a),
        Err(_) => String::new(),
    };
    Ok(format!("\"name\":\"{}\",\"version\":\"{}\",\"meta_hash\":\"{}\",\"meta_id_pic\":\"{}\",\"networks\":{}", network_specs.name, network_version, hex::encode(meta_hash), hex_id_pic, relevant_networks_print))
}

/// Display types status
pub fn show_types_status (database_name: &str) -> Result<String, ErrorSigner> {
    match try_get_types::<Signer>(database_name)? {
        Some(a) => Ok(format!("\"types_on_file\":true,{}", ContentLoadTypes::generate(&a).show())),
        None => Ok(String::from("\"types_on_file\":false")),
    }
}

/// Function to generate new random seed phrase, make identicon for sr25519 public key,
/// and send to Signer screen
pub fn print_new_seed (seed_name: &str) -> Result<String, ErrorSigner> {
    let seed_phrase = generate_random_phrase(24)?;
    let sr25519_pair = match sr25519::Pair::from_string(&seed_phrase, None) {
        Ok(x) => x,
        Err(e) => return Err(<Signer>::address_generation_common(AddressGenerationCommon::SecretString(e))),
    };
    let hex_identicon = match make_identicon_from_multisigner(&MultiSigner::Sr25519(sr25519_pair.public())) {
        Ok(a) => hex::encode(a),
        Err(_) => String::new(),
    };
    Ok(format!("\"seed\":\"{}\",\"seed_phrase\":\"{}\",\"identicon\":\"{}\"", seed_name, seed_phrase, hex_identicon))
}


/// Function to get database history tree checksum to be displayed in log screen
pub fn history_hex_checksum (database_name: &str) -> Result<String, ErrorSigner> {
    let database = open_db::<Signer>(database_name)?;
    let history = open_tree::<Signer>(&database, HISTORY)?;
    let checksum = history.checksum().map_err(|e| ErrorSigner::Database(DatabaseSigner::Internal(e)))?;
    Ok(format!("\"checksum\":\"{}\"", hex::encode(checksum.encode()).to_uppercase()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use definitions::{crypto::Encryption, keyring::NetworkSpecsKey, network_specs::Verifier};
    use sp_core::sr25519::Public;
    use std::fs;
    use std::convert::TryInto;
    use crate::cold_default::populate_cold;
    use crate::manage_history::print_history;
    use crate::remove_types::remove_types_info;

    #[test]
    fn print_seed_names() {
        let dbname = "for_tests/print_seed_names";
        populate_cold (dbname, Verifier(None)).unwrap();
        let horrible_print = print_all_seed_names_with_identicons(dbname, &vec![String::from("Alice")]).unwrap();
        let expected_print = r#"[{"identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082","seed_name":"Alice"}]"#;
        assert!(horrible_print == expected_print, "\nReceived: \n{}", horrible_print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn print_seed_names_with_orphan() {
        let dbname = "for_tests/print_seed_names_with_orphan";
        populate_cold (dbname, Verifier(None)).unwrap();
        let horrible_print = print_all_seed_names_with_identicons(dbname, &vec![String::from("Alice"), String::from("BobGhost")]).unwrap();
        let expected_print = r#"[{"identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082","seed_name":"Alice"},{"identicon":"","seed_name":"BobGhost"}]"#;
        assert!(horrible_print == expected_print, "\nReceived: \n{}", horrible_print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn print_all_ids() {
        let dbname = "for_tests/print_all_ids";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = print_all_identities(dbname).unwrap();
        let expected_print = r#"[{"seed_name":"Alice","address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c945000003ac49444154789cedd83d6e14411086614c6cad64647184cd080988b80001172021441b7100020e40b42224e102045c808880906c8f802c2cad1c63ba34faec9ad99afae9ae5eade479832600757ff30496c5d9edededa387de5110aeafafab1fb9b8b8382b7f74ad0b42cb475bf5404945e8f9f1d3323152106a3e7efb6b55ce719be7fb72c6cac06842a8f9784a02403510540b4635420f00746c882a845a00aa27025503114268f978d41b014530dc081900d4b110282f840b2102b0fab22de7b8fddb4d3987a208d67d561e0813a11500f1e11a840580f87d5616441a823618f1e112841700f1fbb49a10bc0054e6682afb3e0d6216210240658fcebe8f9a831011a20054f6e8ecfb9004e146f8bb3e2fe7b827bb9b720e4547ffdb1efe4c78bca9ff9960ed432e042f00e20f69c3f96009007921f87dde7d680a6122680f20fe90349c0fd6009005c1ef8beea35484290055f3885614c1aa761f8758104a7708120055fbc85ca7824001221d61fde9f0dfefdedfff7d14c1ba2fba8fe746a0b487f803d260c4876b101600e2f779f74d0b2150d243fc016d30e2c325082f00e2f759fba446081680a7e868abecfbe622880561410822bcba7c53ce71dfafbe9673283afac7e7a7e51cf7f2dd9f720e45efb3f6cde546901e40fc216d381f2c01202f04bfcfbb4fca85a03d80f843d2703e5803401604bf2fba6f5a1704ab288255ebbe05a1b42094ba206c57bfcb396eb37f56cea12882755f74df341702a53dc41f9006233e5c83b00010bfcfbb4fca8d40490ff107b4c1880f9720bc0088df67ed9b2b8460151d6d957ddf5c0b42694128dd215016c4f9fa4339c7ddec3e9673283afae76a5bce712ff69b720e45efb3f6491140f963f8ff044a43901e40fc216d381f2c01202f04bfcfbb6f9a1b417b00f187a4e17cb006802c087e5f741faf1b825514c1aa65df01022541b43c22752a0800a01684d208819a42441fb95c1ffee676b5bbffcd2d8a60dd17dd477100ca44a0b487f803d260c4876b101600e2f779f7211381f242f007b4c1880f9720bc0088df67ed435300ca8d60151d6d957d1f7223505188ecd1d9f7511200358b404520b24767df370740a9089417223afadbf9ba9ce35edfecca3914bd4f4b03a0d210286d381f2c01202f04bfcfaa19816a85e08335006441f0fbac2c00ca85404520b4a2082d79002837029501712c042f001542402d18bd11221f8faa10a85a889e083500543502d503e2d800541302aac190206a005a3e1ea520a01a8cda323e1ea522a09e18991f8fba204c6b41e9f1d1d38e8270eafd078fa8bfac68d834b50000000049454e44ae426082","has_pwd":false,"path":"//westend"},{"seed_name":"Alice","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082","has_pwd":false,"path":""},{"seed_name":"Alice","address_key":"0164a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","public_key":"64a31235d4bf9b37cfed3afa8aa60754675f9c4915430454d365c05112784d05","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030b49444154789cedda3f8e527114c5712929a95885c524b666626baf050b60092ec225b0000aed6dcdc4d6640a5741454989f7849cd1f77cef77cffdfd79017cdfe24c3124e47e0a4280c5f97c7ef5bf3709c2f178cc7e92d56ab5b03f4d6b825072b4570b94aa082d8fef5713a30a42cef1bbe52fdb6edbd36bdb5835308a10728e4743002c070295606423b4006053436421e402a096082807228450723c6b8dc0221832420d003415025221248408c07af3c6b6db61ffd3f6521461f3616ddb6dfff560aba540b808a5004c85f000584d886a082900e641a8004c85284250015014c1ab26024a418c224400d0b523a0318841842800ba050434042123acdf6d6cbb1dbeef6d2f45111e169f6cbb3d9f3fdb5e8a223c6efe7dfcd3fecfff9984a0023015c203602a8407c01408172105c03c08158079102a00eb432411fa00288ae01545f0ca41407f43cc08d60bc21000ba57044488ea083f160fb6ddde9e9f6d2f4511764f4bdb6edbc793eda54910500ac203602a8407c05488310014424043102a00f3205400e641a4005007c103508a227845117203c48c3023041176cb2fb6ddb6a78fb697a208de3bcb28c26ef9cdb6dbf6f4de369d8c3004c054080f80a9101e00f32024841400f3205400e641a8002c05d104c12b8ae0352358338275950877f99a8052101e0053213c00a642a400908c8086205400e641a800cc83f0005008c12b8ae01545c86d46b06604eb0501791077ff79024a210c013015c203602a8407c0521032420a8079102a00f32054003606d10cc12b8ae05515010d41dc2b0201d08c607510501f228ae0bd138c224cfe5d24ea23a0148407c054080f80a9107d00e42220154205601e840ac03c080500c9085e5104af28829a8c80a210b78030048046115004e2da11c600501201a91051849bf91d235211500ac203602a840a808a115029840ac03c889a0048424011885451849214002423a01a105321a8002884c04a305a23448e6759082817a225420e00ca46402d20a6064045082c0763082207a0e478560581e560e456e378561581b5c4a8793c6b82d0af04a5c5d1fd2641b8f67e0340babfaccd8d3a340000000049454e44ae426082","has_pwd":false,"path":"//kusama"},{"seed_name":"Alice","address_key":"0196129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04","public_key":"96129dcebc2e10f644e81fcf4269a663e521330084b1e447369087dec8017e04","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000034e49444154789cedd93b6e1541108561bc849bb106522422c74ebc142467160944902067482cc589634748a4ac81ec2ee1d24757c7d48c6bead18fd1589e3f6807a0aeea2f1ad917a7d3e9cd6b6f1584e3f1583de470385c941f431b82d0f268af11285d11463e7e5e4f8c2e08358f7ff8f5b79cd3ae3ebc2d67ae1e184d08358f471a00ab81402d18d5082300d8da105508b500682402aa814821b43c9e8d4660198c30420f00b416028a4284103200378f8fe59cf6e3f2b29ce7b20837df94fb3effbfcf2b02e122b402b0288407c07a427443b0009807110560518826842800ca2278f5444016c4224206006d1d012d41a8085900f4121090061146f87a7d57ce695fee6fcb792e8bf0f1eeba9cd37edede97f35c16c1db8f8510a2004c0eb2203c001685f00098dc8fcd215c046b00938334882800f320a2004cee874c843900aa19629545f0aadd4f42ec08a527040d00d50e596a2b088810dd11be2bffff93f8f72c82775f763f5918015983e4006d612617b7203c0026ef8bee372f8580b4417280b530938b6b10510026eff3f6d39a20780091b24b7bf5be6f2940ec083b4212c1fb72cb2efdfeea5d39a7fd7ef853ce73d9fbbc2fd5a5c2081a008b42c8853500168590f76900cc8308215800cc83900b5b00cc8390f75900cc821882e09545f0da114a3b42699308de976016c1bb6f1504644178004c2e6e4178004cde67415800288c8034882800938b6b10510026efd3203c009442f0ca2eedd5fbbea57684d28e507a42401e84f7b7c3ecd2de9760f63e6f3f2d00941fe7df27200b411bc0e4206b71b9b006c0a210f2bee87ef3c208d6002607698bcb852d00e641c8fbb2fbc98621786511bc5af67b8680348896215a5b412000da114a13043487c80ef1bedcb208de976a763f2401908b80ac41728006c0a2101e008b42c8fd988b80a210728005c03c882800f320e47e6c0e80c2085e5904af2c42b43002ca42bc04040d002d22a00cc4d6119600908980a2105904ef6f873d112c00d40d0159101e008b4244015033026a858802300fa227000a21a00c845516a1a508000a23a01e106b214401500a81b5608c46c83c9e5521a05a8891083500a81a018d80581b003521b01a0c0da206a0e5f1ac0b02abc1a8adc7e35957043612a3e7e3d91084792d28231e3d6f1584adf70f48c8bfac13b313dd0000000049454e44ae426082","has_pwd":false,"path":"//rococo"},{"seed_name":"Alice","address_key":"01c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27","public_key":"c81ebbec0559a6acf184535eb19da51ed3ed8c4ac65323999482aaf9b6696e27","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000036d49444154789cedd93d8e134d148561266001b644320b1989e5109082c442902025603948b3904990ec0510983ab2cee87673fbfe54ddb22cd16fd05f76ebf889461f0f97cbe5cdffde4d10cee773f72387c3e1a1fd676a5310467eb4d70c945284993f7e5d25460942cf8f7f3efe68df654fa78fed9bab026308a1e7c7230d80f540a0118c6e841900ecd6105d08bd00682602ea8148218cfc78361b816530c2081500e85608280a1142c8001cdf7d68df65a7df3fdbf75a16c1bbe7158170114601981c6e4178004cdef3f220ca10acc14c0ed720a2004cdeb31a428802a0cad1a8fa9e05b189900140d5a3abefa12d0815210b80aa4757df631a4418e1f1f0b97d97bd9cbfb5efb5ece8ef87afedbbecd3f94bfb5ecbdef3f6b110421480c987ace172b006c0a210f25e741f5b43b808d6034c3ea40d97832d00e641c87bd97dc8445803a09e47acb2085ebdfb24c48ed07a45d00050ef235bdd0b02224439c29fc363fb2e7b7b7e69df6b5904ef5e769f2c8c80ac87e403da6026875b101e0093f7a2fbd6a51090f6907cc01acce4700d220ac0e43d6f9fd602c10388941ded557d6f2b40ec083b4212c1fbcb2d3bfafdf3b17d97fd7a3ab5efb5ec3d6fdf566104ed01261fb286cbc11a008b42c87bd17d5a2104eb01261fd286cbc11600f320e4bdecbe755310bcb2085ea3fb7684d68ed09a82e0fd259845f0ee65f7ad0b2120eb21f9803698c9e1168407c0e4bde83ead3002d21e920f5883991cae41440198bce7eddb2a85e0951ded557d6fab1da1b523b45e119007e1fddb6176b4f79760f69eb74f0b00ed3fd7ff9f802c04ed01261fb286cbc11a008b42c87bd17debc208d6034c3ea40d97832d00e641c87bd97db269085e5904af917dff20200d62e411ad7b412000da115a0b04b486c83ee2fde59645f0ee65f72109805c04643d241fd0063339dc82f00098bc17ddc75c041485900f5883991cae41440198bce7ed636b001446f0ca8ef6aabec7c208280b513dbafa1ed200d02602ca40548faebeb705804c041485c88ef6feed307bcfca02406508c81a2e076b002c0a21ef790d23a0510839d802601e84bce7e501a01002ca40586511468a00a03002aa80b815421400a510d808c66c84cc8f675d08a8176226420f00ea464033206e0d808610580f8606d10330f2e3590902ebc1e8ade2c7b352043613a3f2c7b32908eb465066fce8753741b8f7fe02e3f3bfac1f06b6e60000000049454e44ae426082","has_pwd":false,"path":"//alice"},{"seed_name":"Alice","address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","has_pwd":false,"path":"//Alice"},{"seed_name":"Alice","address_key":"01f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","public_key":"f606519cb8726753885cd4d0f518804a69a5e0badf36fee70feadd8044081730","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c945000003a849444154789cedd82f6e1b5110c7f1865a0ab054c9a8dc472829300a2c28aa941ba46728e8199a1b442a2a280c3228e911cc8b2c55328864eace6835c9ec7a76febc376f6529fb052fecbddf7ea4105f9d4ea737afbd49100e8743f123cbe5f20afe34ad0942cd475bb540494568f9f1c3323152104a3e7eb1dec2d9efb8dbc0192b03a30aa1e4e33109802a81c06a308a115a0050534314219402602d11b012881042cdc753ad11a808861b2103009b0a01f342b8102200abed4738fbed37bfe0ec8a2258f75979204c845a008a0fd7202c008adf676541a4216883293e5c82f00250fc3ead2a042f0096391acbbe4f831845880060d9a3b3efc3c62044842800963d3afb3e4a8270232c1ed670f63bdeeee0ec8a8e7ebcbf86b3dfcddd139c5dd1fbac7d940bc10b40f187b4e17cb004407921f87dde7dd410c244d01ea0f843d2703e5803a02c087e5f741fa6220c01b09247b4a20856a5fb38c48c003d23480058e923635d0a024610e908ebf70b38fbedfe1ce1ec8a2258f745f7f1dc0898f6107f401a4cf1e11a840540f1fbbcfb86851030e921fe803698e2c325082f00c5efb3f649f5102c004fd1d156d9f78d851033c28c104478b75dc1d9efef660f675774f4f5fd4f38fb3ddd7d82b32b7a9fb56f2c3782f400c51fd286f3c11200e585e0f779f749b910b40728fe90349c0fd600280b82df17dd37ac09825514c1aa76df8c00cd085013842febf3ffe1efbb97ffe12882755f74df301702a63dc41f9006537cb806610150fc3eef3e293702263dc41fd006537cb804e105a0f87dd6beb1420856d1d156d9f78d3523403302f48c8059109f85513fd888e8e8d5b72d9cfdf65f37707645efb3f6492100fce97e4fc03404e9018a3fa40de7832500ca0bc1eff3ee1be646d01ea0f843d2703e5803a02c087e5f741faf19825514c1aa66df19022641d43c227529080480cd08500f011b42441fb9599cffeafb787cf9d5378a60dd17dd877100cc44c0b487f803d2608a0fd7202c008adfe7dd4799089817823fa00da6f87009c20b40f1fbac7dd4100073235845475b65df47b911b02844f6e8ecfb3009001b45c02210d9a3b3ef1b03c05404cc0b111d7dfbf6039cfd1efefd86b32b7a9f960680a52160da703e5802a0bc10fc3eab6a04ac16820fd600280b82df676501602e042c02a11545a8c90380b911b00c88a910bc00580881aac1688d10f978aa08012b8568895002801523602d20a606c0aa10a8120c09a204a0e6e3a91404aa04a3b48c8fa75211a89618991f4f3541185683d2e2a3874d8270e9fd07d4a1bface9299ffb0000000049454e44ae426082","has_pwd":false,"path":"//polkadot"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn print_ids_seed_name_network() {
        let dbname = "for_tests/print_ids_seed_name_network";
        populate_cold (dbname, Verifier(None)).unwrap();
        let horrible_print = print_identities_for_seed_name_and_network(dbname, "Alice", &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519), None, Vec::new()).unwrap();
        let expected_print = r#""root":{"seed_name":"Alice","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","swiped":false,"multiselect":false},"set":[{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c945000003ac49444154789cedd83d6e14411086614c6cad64647184cd080988b80001172021441b7100020e40b42224e102045c808880906c8f802c2cad1c63ba34faec9ad99afae9ae5eade479832600757ff30496c5d9edededa387de5110aeafafab1fb9b8b8382b7f74ad0b42cb475bf5404945e8f9f1d3323152106a3e7efb6b55ce719be7fb72c6cac06842a8f9784a02403510540b4635420f00746c882a845a00aa27025503114268f978d41b014530dc081900d4b110282f840b2102b0fab22de7b8fddb4d3987a208d67d561e0813a11500f1e11a840580f87d5616441a823618f1e112841700f1fbb49a10bc0054e6682afb3e0d6216210240658fcebe8f9a831011a20054f6e8ecfb9004e146f8bb3e2fe7b827bb9b720e4547ffdb1efe4c78bca9ff9960ed432e042f00e20f69c3f96009007921f87dde7d680a6122680f20fe90349c0fd6009005c1ef8beea35484290055f3885614c1aa761f8758104a7708120055fbc85ca7824001221d61fde9f0dfefdedfff7d14c1ba2fba8fe746a0b487f803d260c4876b101600e2f779f74d0b2150d243fc016d30e2c325082f00e2f759fba446081680a7e868abecfbe622880561410822bcba7c53ce71dfafbe9673283afac7e7a7e51cf7f2dd9f720e45efb3f6cde546901e40fc216d381f2c01202f04bfcfbb4fca85a03d80f843d2703e5803401604bf2fba6f5a1704ab288255ebbe05a1b42094ba206c57bfcb396eb37f56cea12882755f74df341702a53dc41f9006233e5c83b00010bfcfbb4fca8d40490ff107b4c1880f9720bc0088df67ed9b2b8460151d6d957ddf5c0b42694128dd215016c4f9fa4339c7ddec3e9673283afae76a5bce712ff69b720e45efb3f6491140f963f8ff044a43901e40fc216d381f2c01202f04bfcfbb6f9a1b417b00f187a4e17cb006802c087e5f741faf1b825514c1aa65df01022541b43c22752a0800a01684d208819a42441fb95c1ffee676b5bbffcd2d8a60dd17dd477100ca44a0b487f803d260c4876b101600e2f779f7211381f242f007b4c1880f9720bc0088df67ed435300ca8d60151d6d957d1f7223505188ecd1d9f7511200358b404520b24767df370740a9089417223afadbf9ba9ce35edfecca3914bd4f4b03a0d210286d381f2c01202f04bfcfaa19816a85e08335006441f0fbac2c00ca85404520b4a2082d79002837029501712c042f001542402d18bd11221f8faa10a85a889e083500543502d503e2d800541302aac190206a005a3e1ea520a01a8cda323e1ea522a09e18991f8fba204c6b41e9f1d1d38e8270eafd078fa8bfac68d834b50000000049454e44ae426082","has_pwd":false,"path":"//westend","swiped":false,"multiselect":false},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","has_pwd":false,"path":"//Alice","swiped":false,"multiselect":false}],"network":{"title":"Westend","logo":"westend"}"#;
        assert!(horrible_print == expected_print, "\nReceived: \n{}", horrible_print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn show_all_networks_flag_westend() {
        let dbname = "for_tests/show_all_networks_flag_westend";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = show_all_networks_with_flag(dbname, &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519)).unwrap();
        let expected_print = r#""networks":[{"key":"018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3","title":"Polkadot","logo":"polkadot","order":0,"selected":false},{"key":"0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe","title":"Kusama","logo":"kusama","order":1,"selected":false},{"key":"0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","title":"Westend","logo":"westend","order":2,"selected":true},{"key":"0180aaf2cd1b74b5f726895921259421b534124726263982522174147046b8827897","title":"Rococo","logo":"rococo","order":3,"selected":false}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn show_all_networks_no_flag() {
        let dbname = "for_tests/show_all_networks_no_flag";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = show_all_networks(dbname).unwrap();
        let expected_print = r#""networks":[{"key":"018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3","title":"Polkadot","logo":"polkadot","order":0},{"key":"0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe","title":"Kusama","logo":"kusama","order":1},{"key":"0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","title":"Westend","logo":"westend","order":2},{"key":"0180aaf2cd1b74b5f726895921259421b534124726263982522174147046b8827897","title":"Rococo","logo":"rococo","order":3}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn first_standard_network() {
        let dbname = "for_tests/first_standard_network";
        populate_cold (dbname, Verifier(None)).unwrap();
        let specs = first_network(dbname).unwrap();
        assert!(specs.name == "polkadot", "\nReceived: \n{:?}", specs);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn export_alice_westend() {
        let dbname = "for_tests/export_alice_westend";
        populate_cold (dbname, Verifier(None)).unwrap();
        let public: [u8;32] = hex::decode("46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a").unwrap().try_into().unwrap();
        let print = export_key (dbname, &MultiSigner::Sr25519(Public::from_raw(public)), "Alice", &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519)).unwrap();
        let expected_print = r#""qr":"89504e470d0a1a0a0000000d49484452000000c4000000c40800000000e5cdd1b7000006ac49444154789cedcf416e1cdb1504517bff8bb6790601245e574be4c046ff6205406466bc6a09f7dffff9d73f9fe7884fe139e253788ef8149e233e85e7884fe139e253788ef814ee79c4bfbffedee1db7db7176f391dbbb7c3ae0787d32f7d13cf11f6e22da763f776d8f5e070faa56fe2be479c0e7979c29f9cdfed37bd5d39f0b63cb9f2cf11c19f9cdfed37bd5d39f0b63cb9f2f73e42467b336c9cce3e13f5123a6cfdcc68cbe53902364e679f897a091db67e66b4e5f27b8f083bfee4db7519f6e28ddb8cb65c9e23ecf8936fd765d88b376e33da72f99d474087ad6fe2ecb0f532ecc51bb7196db93c47d8fa26ce0e5b2fc35ebc719bd196cbbd8f38c97f37a1a31d7c4e877d767972e59f23fe94d0d10e3ea7c33ebb3cb9f2f73de21dbef5feffca77785f9e23fed7f90eefcb3d8ff80efe03bf93d061eb12575d4287ad97d07fc27384840e5b97b8ea123a6cbd84fe13ee79c4f90fb5b14e97a86fa22e51976183db0e5b970bb73c4748d4375197a8cbb0c16d87adcb855bee7d449cefe89bdeecb3974b4e2e39091d767de197e788deecb3974b4e2e39091d767de197df73c4e9c09ff4ddbeadfb6e2fe36a2fcf11277db76febbedbcbb8dacb3d8f083ff256868d9cad6f62fbc29ff45d6fb62e63b7be3c47c0d637b17de14ffaae375b97b15b5fee7f04bcbfeb6807bfae2d9775f5f2a73c47a01dfcbab65cd6d5cb9f72cf23fc438b774ec6bb2da10767cbe5bb6ee95d2ecf11d196d083b3e5f25db7f42e977b1eb1f8c1e25b6e7379e7c05f75b465d8e0ce2e97e708b9bc73e0af3ada326c706797cb3d8ff011f2bbcfbe1936387d73e1c0d761631dd6d7e33962336c70fae6c281afc3c63aacafc73d8f800fd1db6e5d627b708b774ec66e1db6fe9d3c798ed81edce29d93b15b87ad7f274fee79c4d587ef5cf4966b83b34be8b0f512fa925f5ebe79115f7fdf75d15bae0dce2ea1c3d64be84b7e79f9e6457cfd7dd7456fb93638bb840e5b2fa12ff9e5e59b17f1f5f75d17bde5dae0ec123a6cbd84bee497976f4e013fe437a1a30d6e77f0cbd537f05d6ff54de84b3e9e23c0ed0e7eb9fa06beebadbe097dc9c73d8ff0034ea22e51df848e76f0ebeca5b7f5dcd596d097e788123adac1afb397ded673575b425fee7d0474d8f5e0c25b5b8fd3d9fa996183d3cbe5d2bd88afbf9c0ebb1e5c786beb713a5b3f336c707ab95cba17f1f597d361d7830b6f6d3d4e67eb67860d4e2f974bf722befe723aec7a70e1adadc7e96cfdccb0c1e9e572e95ec4d71f27176ef1ce6d425fd6eb57b9e4364ff8e53982db84beacd7af72c96d9ef0cb3d8f801fe6f5c8456ff9dd7ab970e16df7b26fef7a3c47f496dfad970b17de762ffbf6aec7bd8f887dcf73db97d35f6d5da25e2e1cf8fa15cf11d097d35f6d5da25e2e1cf8fa15f73cc20f4e7c93bfea3276ebb07589bff532ce7df21c5197b15b87ad4bfcad9771ee93fb1ec16d62fbb2be5e42bfa277f8c69661077feee53902ebeb25f42b7a876f6c1976f0e75eee7904fc8097273cf66d5d3d72326cbc73ba841e570ecf11b1ae1e391936de395d428f2b877b1ee1436e133a6cbd0cfb4ff8b66ff425bfec37de779f3c47c8b0ff846ffb465ff2cb7ee37df7c9ef38027ae4e27c6bd74be80bcf9df9539e2370beb5eb25f485e7cefc29f73d02bcbeb9acab97a897d061d79777fec477cb7344ae5ea25e42875d5fdef913df2df73c22fc63de36a15fd1fbe2db2b8fdee49293b8ea72798e587a5f7c7be5d19b5c7212575d2ebfeb88b091b37589b3c3ae0777e29b7c5d2e97ee14d1c765d8c8d9bac4d961d7833bf14dbe2e974b778ae8e3326ce46c5de2ecb0ebc19df8265f97cba53b45f471193672b62e7176d8f5e04e7c93afcbe5d2bd88af3ff07a89bac45597efe85d623bec2bfaa6f7763c475c75f98ede25b6c3bea26f7a6fc73d8ff81bfd434bff466fedc55b5e87ad9761c7fad8773c4744ff466fedc55b5e87ad9761c7fad877dcf388ab1f45dffa46df0c1b5c3d38e4cf0dcebecab097e788cdb0c1d58343fedce0ecab0c7bb9ef11a743be841e9cbd09fdc4db950fefd1779c2e4f9e23f4e0ec4de827deae7c788fbee3747972ef2364b4379775fac2e7f4250f6fbb4fbcbfe33922d6e90b9fd3973cbced3ef1fe8edf7904cebee4a377fe6f7dc94be8918be708e84b3e7ae7ffd697bc841eb9f8dd472cebf532ec13ef7c89b307d7de1ecf11b15e2fc33ef1ce97387b70eded71ef234ef225aeba8cdd75091df6bb7ef2eeed39e2aacbd85d97d061bfeb27efdeee7bc43b7ceb5d9e5c796ed9776fb68cb68cdddb97e788e5ca73cbbe7bb365b465ecdebedcf3887f22cf119fc273c4a7f01cf1293c477c0acf119fc273c4a7f01cf129dce288ff0239d9b86ac67989a70000000049454e44ae426082","pubkey":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082","seed_name":"Alice","path":"","network_title":"Westend","network_logo":"westend""#;
        assert!(print == expected_print, "\nReceived: \n{:?}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn backup_prep_alice() {
        let dbname = "for_tests/backup_prep_alice";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = backup_prep(dbname, "Alice").unwrap();
        let expected_print = r#""seed_name":"Alice","derivations":[{"network_title":"Polkadot","network_logo":"polkadot","network_order":0,"id_set":[{"path":"","has_pwd":false},{"path":"//polkadot","has_pwd":false}]},{"network_title":"Kusama","network_logo":"kusama","network_order":1,"id_set":[{"path":"","has_pwd":false},{"path":"//kusama","has_pwd":false}]},{"network_title":"Westend","network_logo":"westend","network_order":2,"id_set":[{"path":"//westend","has_pwd":false},{"path":"","has_pwd":false},{"path":"//Alice","has_pwd":false}]},{"network_title":"Rococo","network_logo":"rococo","network_order":3,"id_set":[{"path":"","has_pwd":false},{"path":"//rococo","has_pwd":false},{"path":"//alice","has_pwd":false}]}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn derive_prep_alice() {
        let dbname = "for_tests/derive_prep_alice";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = derive_prep(dbname, "Alice", &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519), "//secret//derive").unwrap();
        let expected_print = r#""seed_name":"Alice","network_title":"Westend","network_logo":"westend","suggested_derivation":"//secret//derive""#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn westend_network_details() {
        let dbname = "for_tests/westend_network_details";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = network_details_by_key(dbname, &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519)).unwrap();
        let expected_print = r##""base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"2","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","current_verifier":{"type":"general","details":{"hex":"","identicon":"","encryption":"none"}},"meta":[{"spec_version":"9000","meta_hash":"e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce","meta_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000032f49444154789cedda3d6e534114c571a7a0414a6109c95b70c1265c221640431189928e85d05122a5a0610188d29ba0f0162c21b988444361e6c83af1f8793cf7dcf9b06cf3fec5a4784f37777e456425b9db6eb793ffbdb3206c369be26f329d4eefc297ae7541a8b9b4550f94a6083d2f3fac2546138492cbcf3fbf0ce761ab4f7fc2e9ab05461542c9e5510a809540a01a8c62841e00ecdc104508a500a827022a817021d45c9ef546601e0c19a105003a1702522124040fc0ecc3329c87adbf2ec2b9cb8bf07a7efcfeafd5feb995026122d4023015c202602d219a21e4009805a1023015a20a4105405e04ab960828077112c103802e1d019d8248227801d03520a014848c70ffe24b380f7bfafb319cbbbc08b3c7e3f7d70ffbe75e046b1e93105400a64258002c5e3c076101b0781e1b42980839006641a8002c5e3c05a102b0781eca220c019017c1aa64e95ca5f362881121f48c900240b78a8008d11ce1fec7f1fb4f6ff7cfbd4bcf96c7efaf17d173e7bc381901e5202c00a642c40ba700980a11cf1be64240290815805910f1c239006641c4f3521d2058004a5e042b2f4269801811460427c26cf93019b65e3c4e9817e1fdbb57e13cecdbf7dfe1dce54598ff3cfe64b97ab3ff64792a192105c054080b80a9101600b32024841c00b32054006641a8002c07d105c1ca8b603522844684d04522dce4cf049483b000980a610130152207806404948250019805a102300bc202402e042b2f829517a1b411213422849e11900571f3bf4f403984140053212c00162f9e83b000583c6f988c90036016840ac0e2c553102a008be7c57543b0aa593a55cdbc23049482b8550402a0112174808086105e84abff5b241a22a01c8405c0548878e114005321e279cc44402a840ac02c8878e11c00b320e2796c088064042b2f829517414d46405e886b404801a09308c80371e908a700501601a9105e84abf93f46a422a01c8405c0540815005523a05a08158059102d019084803c10b9bc083529004846402d20ce85a002201702abc1e88de0b93c2b4240a5103d114a00503102ea01716e005485c04a30521025003597674d10580946692d2ecf9a22b09e182d2fcfba200cab41e971e9616741b8f4fe0136d3bfacd01443e90000000049454e44ae426082"},{"spec_version":"9010","meta_hash":"70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf","meta_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033449444154789cedd93d6e135114c571b284c83414d429116e690129ca365c2121b6902d20242a6f238a04b4b44694a92968b0bc04f38eac1b7bc66fde39f77d8c9c64fec59d6246b2eeaf183d79ceb6dbedb3a7de28089bcd26fb47cecfcfcfc2a5694d104a9666b540a98ad072f97e3531aa20e42cbffe761366b7d9fbab307dd5c02842c8591ec500ac1c085482918dd002c01a1b220b211700b5444039102e8492e5add60896074346a80180c642402a8484e001587d5d84d96dfe6119e62e2fc2fad7f1f3b3d7fbfb2c0582229402582a0403b06a4254434801580c4205b05488220415007911583511500a6210c103804e1d010d414411bc00e82120a018848c70f3671d66b7ab97b330777911fead8f977c3edb2fe54558fc5c85d96df9661e6637094105b054080660a9100cc0522028420ac062102a80c5205400ab0f9144e803202f02cb8bc0ca414087101342e81e2106801e2b023288ea08abc5a730bbcd979fc3dce545b8bcb808b3dbeddd5d98bb46414029080660a9100cc052218600900b01c52054008b41a80016834801a00e020350f222b0bc08b901624298109c08ec24e845787779bce4f7dbfd525e84bf9177cc8b8377cc5032420cc052211880a54230008b41480829008b41a80016835001ac144413049617813521842684d04922b093a017e155e4f9df07f747414029080660a9100cc052215200484640310815c062102a80c52018007221b0bc082c2f426e1342684208dd232006c1be1d7a11d849d08b70fdf64b98ddae7f7c0c73380084cbeeff0494428801582a0403b054080660a52064841480c52054008b41a800d610443304961781551501c5201e2b8201a00921d441407d082f02fb76e8456027c11c8443004411500a8201582a0403b054883e00a208488550012c06a102580c4201403202cb8bc0f222a8c908c80bf11010620068100179204e1d6108002511900ae14560df0e6b22a40050350494826000960aa102a06204540aa102580ca2260092109007229517a1240500c908a806c458082a007221582518ad113ccb5b59082817a225420e00ca46402d20c606404508560e460c2207a06479ab0a829583915b8de5adaa08564b8c9acb5b4d10fa95a0b458badf2808a7de7f10bfbfacc65e85560000000049454e44ae426082"}]"##;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn westend_9010_metadata_details() {
        let dbname = "for_tests/westend_9010_metadata_details";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = metadata_details(dbname, &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519), 9010).unwrap();
        let expected_print = r#""name":"westend","version":"9010","meta_hash":"70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf","meta_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033449444154789cedd93d6e135114c571b284c83414d429116e690129ca365c2121b6902d20242a6f238a04b4b44694a92968b0bc04f38eac1b7bc66fde39f77d8c9c64fec59d6246b2eeaf183d79ceb6dbedb3a7de28089bcd26fb47cecfcfcfc2a5694d104a9666b540a98ad072f97e3531aa20e42cbffe761366b7d9fbab307dd5c02842c8591ec500ac1c085482918dd002c01a1b220b211700b5444039102e8492e5add60896074346a80180c642402a8484e001587d5d84d96dfe6119e62e2fc2fad7f1f3b3d7fbfb2c0582229402582a0403b06a4254434801580c4205b05488220415007911583511500a6210c103804e1d010d414411bc00e82120a018848c70f3671d66b7ab97b330777911fead8f977c3edb2fe54558fc5c85d96df9661e6637094105b054080660a9100cc0522028420ac062102a80c5205400ab0f9144e803202f02cb8bc0ca414087101342e81e2106801e2b023288ea08abc5a730bbcd979fc3dce545b8bcb808b3dbeddd5d98bb46414029080660a9100cc052218600900b01c52054008b41a80016834801a00e020350f222b0bc08b901624298109c08ec24e845787779bce4f7dbfd525e84bf9177cc8b8377cc5032420cc052211880a54230008b41480829008b41a80016835001ac144413049617813521842684d04922b093a017e155e4f9df07f747414029080660a9100cc052215200484640310815c062102a80c52018007221b0bc082c2f426e1342684208dd232006c1be1d7a11d849d08b70fdf64b98ddae7f7c0c73380084cbeeff0494428801582a0403b054080660a52064841480c52054008b41a800d610443304961781551501c5201e2b8201a00921d441407d082f02fb76e8456027c11c8443004411500a8201582a0403b054883e00a208488550012c06a102580c4201403202cb8bc0f222a8c908c80bf11010620068100179204e1d6108002511900ae14560df0e6b22a40050350494826000960aa102a06204540aa102580ca2260092109007229517a1240500c908a806c458082a007221582518ad113ccb5b59082817a225420e00ca46402d20c606404508560e460c2207a06479ab0a829583915b8de5adaa08564b8c9acb5b4d10fa95a0b458badf2808a7de7f10bfbfacc65e85560000000049454e44ae426082","networks":[{"title":"Westend","logo":"westend","order":2,"current_on_screen":true}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn types_status_and_history() {
        let dbname = "for_tests/types_status_and_history";
        populate_cold (dbname, Verifier(None)).unwrap();
        
        let print = show_types_status(dbname).unwrap();
        let expected_print = r#""types_on_file":true,"types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035549444154789cedd8dd89144114c57117049fdb140cc18c8c61c3d818ccc8104cc17e1684b50ec35dcfd4debe1f55b79a01fbff30fb304dd5e91f2cac3ebdbebe7ef8df3b0561dff7e14bb66d7b6a3f96b60461e6a5bd56a09422ac7cf9be4a8c12849197fffce55bfbbcefd7cfefed335705c614c2c8cb230d401a81403318c3082b00a4b32186104601d04a043402914298797969358294c108235400a0b310501422849001d87ebcb4cffbf6afcfedf35616c13bcf2b02e122cc02483cdc82f000243ecfcb832843b0064b3c5c838802487c9ed514421400558e46e5e7191087081900543ebaf83c7404a122640150f5e8eaf3240d228c50fd3bfc697bfffceffddff7d9f3bc7d5208210a20f145d6701eac014851083e2fba4fea215c04eb02892fd286f3600b40f220f8bcec3e6422f40068e412ab2c82d7e83e86b8105a6f081a001abde4a847414002518ea0bd24bf94f67d9ff73c7f9fddc785119075115fa00d967878f573d17d7d2904a45dc4175883251eae3def7ddfc7cf7bfbb4ee103c8048d9d15ed5e71d05880be1424822787fb965477f7cd9dae77d7f9ef7f6792b7b9eb7efa830827681c41759c379b006204521f8bce83ead10827581c41769c379b0052079107c5e765fdf1204af2c82d7ecbe0ba17521b4962078bfc35904efbcecbebe1002b22ee20bb4c1120fb7203c0089cf8beed30a2320ed22bec01a2cf1700d220a20f179debea352085ed9d15ed5e71d7521b42e84d61b02f220bc7faf67476bcf7bdff7f1f3de3e2d00b41fb7ff4f4016827681c41759c37970f573d17d7d6104eb02892fd286f360edfb3eef79fe3ebb8f5b86e0a5bd541fbfa4d7ccbe7708488398b944eb511004005d08ad3b04d443642ff1fe72cb2278e765f72106402e02b22ee20bb4c1120fb7203c0089cf8bee935c041485e00bacc1120fd720a200129fe7ed937a001446f0ca8ef6aa3e4f0a23a02c44f5e8eaf39006800e115006a27a74f9790700c844405188ec68ef77387b9e950580ca1090359c076b00521482cff39a4640b3103cd802903c083ecfcb034021049481b0ca22cc140140610454017116421400a510a4198cd5089997978610d028c44a841100348c8056409c0d80a610a4110c0d620460e6e5a51204690463b48a97974a11a49518952f2f2d41e89b4159f1d27da7203c7a7f01d20cbfac205d21400000000049454e44ae426082""#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        
        remove_types_info(dbname).unwrap();
        let print = show_types_status(dbname).unwrap();
        let expected_print = r#""types_on_file":false"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        
        let history_printed = print_history(dbname).unwrap();
        let expected_element = r#"{"event":"types_removed","payload":{"types_hash":"d091a5a24a97e18dfe298b167d8fd5a2add10098c8792cba21c39029a9ee0aeb","types_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000035549444154789cedd8dd89144114c57117049fdb140cc18c8c61c3d818ccc8104cc17e1684b50ec35dcfd4debe1f55b79a01fbff30fb304dd5e91f2cac3ebdbebe7ef8df3b0561dff7e14bb66d7b6a3f96b60461e6a5bd56a09422ac7cf9be4a8c12849197fffce55bfbbcefd7cfefed335705c614c2c8cb230d401a81403318c3082b00a4b32186104601d04a043402914298797969358294c108235400a0b310501422849001d87ebcb4cffbf6afcfedf35616c13bcf2b02e122cc02483cdc82f000243ecfcb832843b0064b3c5c838802487c9ed514421400558e46e5e7191087081900543ebaf83c7404a122640150f5e8eaf3240d228c50fd3bfc697bfffceffddff7d9f3bc7d5208210a20f145d6701eac014851083e2fba4fea215c04eb02892fd286f3600b40f220f8bcec3e6422f40068e412ab2c82d7e83e86b8105a6f081a001abde4a847414002518ea0bd24bf94f67d9ff73c7f9fddc785119075115fa00d967878f573d17d7d2904a45dc4175883251eae3def7ddfc7cf7bfbb4ee103c8048d9d15ed5e71d05880be1424822787fb965477f7cd9dae77d7f9ef7f6792b7b9eb7efa830827681c41759c379b006204521f8bce83ead10827581c41769c379b0052079107c5e765fdf1204af2c82d7ecbe0ba17521b4962078bfc35904efbcecbebe1002b22ee20bb4c1120fb7203c0089cf8beed30a2320ed22bec01a2cf1700d220a20f179debea352085ed9d15ed5e71d7521b42e84d61b02f220bc7faf67476bcf7bdff7f1f3de3e2d00b41fb7ff4f4016827681c41759c37970f573d17d7d6104eb02892fd286f360edfb3eef79fe3ebb8f5b86e0a5bd541fbfa4d7ccbe7708488398b944eb511004005d08ad3b04d443642ff1fe72cb2278e765f72106402e02b22ee20bb4c1120fb7203c0089cf8bee935c041485e00bacc1120fd720a200129fe7ed937a001446f0ca8ef6aa3e4f0a23a02c44f5e8eaf39006800e115006a27a74f9790700c844405188ec68ef77387b9e950580ca1090359c076b00521482cff39a4640b3103cd802903c083ecfcb034021049481b0ca22cc140140610454017116421400a510a4198cd5089997978610d028c44a841100348c8056409c0d80a610a4110c0d620460e6e5a51204690463b48a97974a11a49518952f2f2d41e89b4159f1d27da7203c7a7f01d20cbfac205d21400000000049454e44ae426082","verifier":{"hex":"","identicon":"","encryption":"none"}}}"#;
        assert!(history_printed.contains(expected_element), "\nReceived history: \n{}", history_printed);
        
        fs::remove_dir_all(dbname).unwrap();
    }
    
}



