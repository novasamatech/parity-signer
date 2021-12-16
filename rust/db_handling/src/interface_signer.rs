use sp_runtime::MultiSigner;
use std::collections::HashMap;

use definitions::{error::{DatabaseSigner, ErrorSigner, InterfaceSigner, NotFoundSigner}, helpers::{multisigner_to_public, make_identicon_from_multisigner}, keyring::{NetworkSpecsKey, AddressKey, print_multisigner_as_base58}, network_specs::NetworkSpecs, print::export_complex_vector, users::AddressDetails};
use qrcode_static::png_qr_from_string;

use crate::helpers::{get_address_details, get_network_specs};
use crate::identities::{get_all_addresses, get_addresses_by_seed_name};
use crate::network_details::get_all_networks;

/// Function to print all seed names with identicons
/// Gets used only on the Signer side, interacts with the user interface.
pub fn print_all_seed_names_with_identicons (database_name: &str) -> Result<String, ErrorSigner> {
    let mut data_set: HashMap<String, Option<MultiSigner>> = HashMap::new();
    for (multisigner, address_details) in get_all_addresses(database_name)?.into_iter() {
        if (address_details.path == "")&&(!address_details.has_pwd) {
            match data_set.get(&address_details.seed_name) {
                Some(Some(_)) => return Err(ErrorSigner::Database(DatabaseSigner::TwoRootKeys{seed_name: address_details.seed_name.to_string()})),
                _ => {data_set.insert(address_details.seed_name.to_string(), Some(multisigner));},
            }
        }
        else {if let None = data_set.get(&address_details.seed_name) {data_set.insert(address_details.seed_name.to_string(), None);}}
    }
    let mut print_set: Vec<(String, String)> = Vec::new();
    for (seed_name, possible_multisigner) in data_set.into_iter() {
        let identicon_string = match possible_multisigner {
            Some(multisigner) => hex::encode(make_identicon_from_multisigner(&multisigner)?),
            None => String::new(),
        };
        print_set.push((identicon_string, seed_name))
    }
    Ok(export_complex_vector(&print_set, |(identicon_string, seed_name)| format!("\"identicon\":\"{}\",\"seed_name\":\"{}\"", identicon_string, seed_name)))
}

/// Function to print separately root identity and derived identities for given seed name and network specs key.
/// Is used only on the Signer side, interacts only with navigation.
pub fn print_identities_for_seed_name_and_network (database_name: &str, seed_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let identities = addresses_set_seed_name_network (database_name, seed_name, network_specs_key)?;
    let mut root_id = None;
    let mut other_id: Vec<(MultiSigner, AddressDetails, Vec<u8>)> = Vec::new();
    for (multisigner, address_details) in identities.into_iter() {
        let identicon = make_identicon_from_multisigner(&multisigner)?;
        let base58 = print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix));
        let address_key = AddressKey::from_multisigner(&multisigner);
        if (address_details.path == "")&&(!address_details.has_pwd) {
            if let Some(_) = root_id {return Err(ErrorSigner::Database(DatabaseSigner::TwoRootKeys{seed_name: seed_name.to_string()}))}
            root_id = Some(format!("\"seed_name\":\"{}\",\"identicon\":\"{}\",\"address_key\":\"{}\",\"base58\":\"{}\"", seed_name, hex::encode(identicon), hex::encode(address_key.key()), base58));
        }
        else {other_id.push((multisigner, address_details, identicon))}
    }
    let root_print = match root_id {
        Some(a) => a,
        None => format!("\"seed_name\":\"{}\",\"identicon\":\"\",\"address_key\":\"\",\"base58\":\"\"", seed_name),
    };
    let other_print = export_complex_vector(&other_id, |(multisigner, address_details, identicon)| format!("\"address_key\":\"{}\",\"base58\":\"{}\",\"identicon\":\"{}\",\"has_pwd\":{},\"path\":\"{}\"", hex::encode(AddressKey::from_multisigner(&multisigner).key()), print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix)), hex::encode(identicon), address_details.has_pwd, address_details.path));
    
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
    let networks = get_all_networks(database_name)?;
    Ok(format!("\"networks\":{}", export_complex_vector(&networks, |a| 
        {
            let network_specs_key_current = NetworkSpecsKey::from_parts(&a.genesis_hash.to_vec(), &a.encryption);
            format!("\"key\":\"{}\",\"title\":\"{}\",\"logo\":\"{}\",\"order\":{},\"selected\":{}", hex::encode(network_specs_key_current.key()), a.title, a.logo, a.order, &network_specs_key_current == network_specs_key)
        }
    )))
}

/// Function to print all networks without any selection
pub fn show_all_networks (database_name: &str) -> Result<String, ErrorSigner> {
    let networks = get_all_networks(database_name)?;
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
    Ok(format!("\"seed_name\":\"{}\",\"derivations\":{}", seed_name, export_complex_vector(&export, |(specs, id_set)| format!("\"network_title\":\"{}\",\"network_logo\":\"{}\",\"network_order\":{},\"id_set\":{}", specs.title, specs.logo, specs.order, export_complex_vector(&id_set, |a| format!("\"path\":\"{}\",\"has_pwd\":{}", a.path, a.has_pwd))))))
}

/// Function to prepare key derivation screen.
/// Gets seed name, network specs key and suggested derivation
pub fn derive_prep (database_name: &str, seed_name: &str, network_specs_key: &NetworkSpecsKey, suggest: &str) -> Result<String, ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    Ok(format!("\"seed_name\":\"{}\",\"network_title\":\"{}\",\"network_logo\":\"{}\",\"suggested_derivation\":\"{}\"", seed_name, network_specs.title, network_specs.logo, suggest))
}

#[cfg(test)]
mod tests {
    use super::*;
    use definitions::{crypto::Encryption, keyring::NetworkSpecsKey, network_specs::Verifier};
    use sp_core::sr25519::Public;
    use std::fs;
    use std::convert::TryInto;
    use crate::cold_default::populate_cold;

    #[test]
    fn print_seed_names() {
        let dbname = "for_tests/print_seed_names";
        populate_cold (dbname, Verifier(None)).unwrap();
        let horrible_print = print_all_seed_names_with_identicons(dbname).unwrap();
        let expected_print = r#"[{"identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082","seed_name":"Alice"}]"#;
        assert!(horrible_print == expected_print, "\nReceived: \n{}", horrible_print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn print_ids_seed_name_network() {
        let dbname = "for_tests/print_ids_seed_name_network";
        populate_cold (dbname, Verifier(None)).unwrap();
        let horrible_print = print_identities_for_seed_name_and_network(dbname, "Alice", &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519)).unwrap();
        let expected_print = r#""root":{"seed_name":"Alice","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033a49444154789cedd8316e1441108561fb084e1d7100c71081135b64e6045cc289034484089cf8129c80cdd03a314410ef01889cee11967a1a9599197aba5e55778fd766fea036989656f505a3d61cee76bb83ffbd5910b6db6df84f8e8e8e0ee5a7694d104a96b66a815215a1e5f2e36a625441882c7ff9f94ee6b09b0fa7327dd5c02842882c8f52005a0402956084115a006873438410a200a825028a40b8104a96d75a23681e0c1aa106009a0b01b110148207e0e2fdb5cc61ab2f5732bbbc083f6fef650e7b75762c938b8130114a013416c202d06a425443c8016816040ba0b11045082c00f22258d54440398849040f00da770434059144f002a0a78080521034c2e5dd3799c36e4edfcaecf2229c9cbf91396cb3fe2eb3cb8b60bd63340a8105d058080b4063212c008d81301172009a05c1026816040ba08d21b2086300e445b0f222584510501f6241901e105200e8b9222085a88ef0fae45ce6b01f9bb5cc2e2f42ea7cfff92c0828076101682c447f41f65c0e620a00b910500a8205d02c88fe62a9e7e3fae7531039003440b00098bc08565e8468805810160427c2bb8b7f6f825f577f6f828f8d60dd54a7a21152001a0be101d072101680664150083900cd8288006829081640cb41344198bb05415a10a4bd44b8bffd7d30eef8ecc54134eb1d330b02ca4158005a042205a0b11039004423a014040ba0792072009a056101201782d56323445b10a405417a404016c4c5f54799c356579f6476791152e7fbcfbd08d6f78e5400909fee7b02ca21a4003416a2bf207b2e0761016839081a2107a05910fdc552cfc7f5cfa72058006d0aa219829517c1aa2a024a413c570405400b8234404063082fc2cbf31399c37ead3732bbbc08d64d3582d007402602ca4158001a0b6101682cc41800990888856001340b8205d02c080600d108565e042b2f021b8d80bc104f012105802611900762df11a600501601b1105e04eb1d5313210780aa21a01c8405a0b1102c002a4640a5102c806641d404401402f240e4f22294c400201a01d580980b8105402e04ad04a3358267792d8480a2102d112200288c805a40cc0d808a10b408460a220250b2bc5605418b6044abb1bc5615416b89517379ad09c2b81294164b8f9b0561dffb036973a4ac27dc44b20000000049454e44ae426082","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV"},"set":[{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c945000003ac49444154789cedd83d6e14411086614c6cad64647184cd080988b80001172021441b7100020e40b42224e102045c808880906c8f802c2cad1c63ba34faec9ad99afae9ae5eade479832600757ff30496c5d9edededa387de5110aeafafab1fb9b8b8382b7f74ad0b42cb475bf5404945e8f9f1d3323152106a3e7efb6b55ce719be7fb72c6cac06842a8f9784a02403510540b4635420f00746c882a845a00aa27025503114268f978d41b014530dc081900d4b110282f840b2102b0fab22de7b8fddb4d3987a208d67d561e0813a11500f1e11a840580f87d5616441a823618f1e112841700f1fbb49a10bc0054e6682afb3e0d6216210240658fcebe8f9a831011a20054f6e8ecfb9004e146f8bb3e2fe7b827bb9b720e4547ffdb1efe4c78bca9ff9960ed432e042f00e20f69c3f96009007921f87dde7d680a6122680f20fe90349c0fd6009005c1ef8beea35484290055f3885614c1aa761f8758104a7708120055fbc85ca7824001221d61fde9f0dfefdedfff7d14c1ba2fba8fe746a0b487f803d260c4876b101600e2f779f74d0b2150d243fc016d30e2c325082f00e2f759fba446081680a7e868abecfbe622880561410822bcba7c53ce71dfafbe9673283afac7e7a7e51cf7f2dd9f720e45efb3f6cde546901e40fc216d381f2c01202f04bfcfbb4fca85a03d80f843d2703e5803401604bf2fba6f5a1704ab288255ebbe05a1b42094ba206c57bfcb396eb37f56cea12882755f74df341702a53dc41f9006233e5c83b00010bfcfbb4fca8d40490ff107b4c1880f9720bc0088df67ed9b2b8460151d6d957ddf5c0b42694128dd215016c4f9fa4339c7ddec3e9673283afae76a5bce712ff69b720e45efb3f6491140f963f8ff044a43901e40fc216d381f2c01202f04bfcfbb6f9a1b417b00f187a4e17cb006802c087e5f741faf1b825514c1aa65df01022541b43c22752a0800a01684d208819a42441fb95c1ffee676b5bbffcd2d8a60dd17dd477100ca44a0b487f803d260c4876b101600e2f779f7211381f242f007b4c1880f9720bc0088df67ed435300ca8d60151d6d957d1f7223505188ecd1d9f7511200358b404520b24767df370740a9089417223afadbf9ba9ce35edfecca3914bd4f4b03a0d210286d381f2c01202f04bfcfaa19816a85e08335006441f0fbac2c00ca85404520b4a2082d79002837029501712c042f001542402d18bd11221f8faa10a85a889e083500543502d503e2d800541302aac190206a005a3e1ea520a01a8cda323e1ea522a09e18991f8fba204c6b41e9f1d1d38e8270eafd078fa8bfac68d834b50000000049454e44ae426082","has_pwd":false,"path":"//westend"},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000030e49444154789cedda4d6a544114c571330e0eb286640782d8ee4090081964033d105c90e0a0379041c020b8035b047790aca10792715b87e644eb59afeeb9f5f1e86edf7f70320cf7075d83a44fb6dbedb3ffbd4910369b4df12f393b3b3b093fbad605a1e668ab1e284d117a1e3fac2546138492e3bf3ede868d7b737a15d6570b8c2a8492e3510a809540a01a8c62841e006c6a8822845200d413019540b8106a8e67bd1198074346680180a642402a8484e001f8717d1f36eee5cd45d85d5e842ff7d761e3de5edc84d552204c845a00a6425800ac254433841c00b3205400a6425421a800c88b60d51201e52046113c0068df11d0184412c10b800e0101a5206484d5e243d8b8e5fa63d85d5e84cbd7e761e3eebe3d84dde54558ac5661e3d6cb65d838094105602a8405c054080b80291026420e8059102a00b32054003684c8220c019017c1ca8b60558280fe869811424f08290074ac088810cd113e25de84f7156fc2cfc49bf0a2f24d603202ca4158004c85b000980a3106805c082805a102300b42056016440e00450816809217c1ca8b501a20668419c189f0ebf6316cdcf3abd3b0bbbc08ef16ff7e863fafff7c86bd08af2ecfc3c67dbf7b089b4f4648013015c202602a8405c02c08092107c02c08158059102a00cb417441b0f22258cd08a11921b4970847f926a01c8405c054080b80a91039002423a014840ac02c081580591016007221587911acbc08a5cd08a11921f484802c88a3ff7b02ca21a400980a61013015c202603908192107c02c08158059102a001b83e88660e545b06a8a805210c78a400034238422043484f0221cfcff22d11001e5202c00a64258004c851802201301a9102a00b3205400664128004846b0f222587911d46404e48538048414001a45401e887d471803405904a44278110ee67b8c48454039080b80a9102a00aa4640b5102a00b3205a0220090179207279116a5200908c805a404c85a002201702abc1e88de0399e1521a052889e082500a81801f580981a005521b0128c14440940cdf1ac09022bc128adc5f1ac2902eb89d1f278d60561580d4a8fa3874d82b0effd06a580bfac7347ad900000000049454e44ae426082","has_pwd":false,"path":"//Alice"}],"network":{"title":"Westend","logo":"westend"}"#;
        assert!(horrible_print == expected_print, "\nReceived: \n{}", horrible_print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn show_all_networks_flag_westend() {
        let dbname = "for_tests/show_all_networks_flag_westend";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = show_all_networks_with_flag(dbname, &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519)).unwrap();
        let expected_print = r#""networks":[{"key":"0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770","title":"Rococo","logo":"rococo","order":3,"selected":false},{"key":"018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3","title":"Polkadot","logo":"polkadot","order":0,"selected":false},{"key":"0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe","title":"Kusama","logo":"kusama","order":1,"selected":false},{"key":"0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","title":"Westend","logo":"westend","order":2,"selected":true}]"#;
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
        let expected_print = r#""seed_name":"Alice","derivations":[{"network_title":"Rococo","network_logo":"rococo","network_order":3,"id_set":[{"path":"","has_pwd":false},{"path":"//rococo","has_pwd":false},{"path":"//alice","has_pwd":false}]},{"network_title":"Polkadot","network_logo":"polkadot","network_order":0,"id_set":[{"path":"","has_pwd":false},{"path":"//polkadot","has_pwd":false}]},{"network_title":"Kusama","network_logo":"kusama","network_order":1,"id_set":[{"path":"","has_pwd":false},{"path":"//kusama","has_pwd":false}]},{"network_title":"Westend","network_logo":"westend","network_order":2,"id_set":[{"path":"//westend","has_pwd":false},{"path":"","has_pwd":false},{"path":"//Alice","has_pwd":false}]}]"#;
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
}



