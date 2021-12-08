use std::collections::HashMap;
use definitions::{error::{DatabaseSigner, ErrorSigner}, keyring::{NetworkSpecsKey, AddressKey, print_multisigner_as_base58}, print::export_complex_vector, users::AddressDetails};
use sp_runtime::MultiSigner;

use crate::helpers::{get_network_specs, make_identicon_from_multisigner};
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
    let identities: Vec<(MultiSigner, AddressDetails)> = get_addresses_by_seed_name(database_name, seed_name)?
        .into_iter()
        .filter(|(_, address_details)| address_details.network_id.contains(network_specs_key))
        .collect();
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
    let other_print = export_complex_vector(&other_id, |(multisigner, address_details, identicon)| format!("\"address_key\":\"{}\",\"base58\":\"{}\",\"identicon\":\"{}\",\"has_pwd\":\"{}\",\"path\":\"{}\"", hex::encode(AddressKey::from_multisigner(&multisigner).key()), print_multisigner_as_base58(&multisigner, Some(network_specs.base58prefix)), hex::encode(identicon), address_details.has_pwd, address_details.path));
    
    Ok(format!("{{\"root\":{{{}}},\"set\":{},\"network\":{{\"name\":\"{}\",\"logo\":\"{}\"}}}}", root_print, other_print, network_specs.name, network_specs.logo))
}

/// Function to print all networks, with bool indicator which one is currently selected
pub fn show_all_networks_with_flag (database_name: &str, network_specs_key: &NetworkSpecsKey) -> Result<String, ErrorSigner> {
    let networks = get_all_networks(database_name)?;
    Ok(export_complex_vector(&networks, |a| 
        {
            let network_specs_key_current = NetworkSpecsKey::from_parts(&a.genesis_hash.to_vec(), &a.encryption);
            format!("\"key\":\"{}\",\"name\":\"{}\",\"logo\":\"{}\",\"selected\":\"{}\"", hex::encode(network_specs_key_current.key()), a.name, a.logo, &network_specs_key_current == network_specs_key)
        }
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use definitions::{crypto::Encryption, keyring::NetworkSpecsKey, network_specs::Verifier};
    use std::fs;
    use crate::cold_default::populate_cold;

    #[test]
    fn print_seed_names() {
        let dbname = "for_tests/print_seed_names";
        populate_cold (dbname, Verifier(None)).unwrap();
        let horrible_print = print_all_seed_names_with_identicons(dbname).unwrap();
        let expected_print = r#"[{"identicon":"89504e470d0a1a0a0000000d4948445200000065000000650806000000547c2dcf0000045949444154789cedda2d72544114c5f1b08458140b8806051852b8b00236814150280a11934db002e2a8c40414e82c00159b25843e459daacccbcd9df7bafb7eccbcfe8bf3ec54fd5cdf797277777730cad54ea0dcdede76fb918787874fca277529517a226c2b23520a144f846d65400a45c984312d1227042533c6b4081c57945dc298e689e382b2cb18d33c704c51f609639a258e19ca3e83302b1813144f900f5fafca6e76f6e975599f2c60baa2786220098479c2a09e38dd503281b05d85e982e20d8232a2a01e30cd281120282b0a6a8569428902419951500b4c354a2408ca8e826a61aa50a2419806130dc26a6016a3640161124c1610b61466118a05c8c9fbd3b29b9d7ffb58d6a7df973765377bf1e669d9be2d810945914098078c04c27ac398a07882304b180d8445c1cc42e90d82d68882e6c06c45b100416b4541db60068a524a142b10b46614a4c184a1200dc6128469309620a80ac51a8449301e204c82b106618fc134a37cb8fa5176b3b3d76fcbfa7474fcaaec66d7173fcbfad4f2a2b008a5058479c04820cc034602612d30d5281a08b384d14098258c06c2e6c0cc4299038206ca5559bd3928680a33502a7343990b8206ca5559bdb928e83ecc40a92c250ad2602c4198066309c234982520a81b0a92603c409804e301c22498a5204844a901e9d1cba3e3b29bfdbabe28eb5334eafd08138a2281300f18098445c084a36820cc12460361de3003253b8a37081a28728019284a034568a004a4c15882300d26020485a32009c6038449305120a80bcabb9387e7dcefe77ee7dce87a9fb39b512410b606180984d5c234a168206c9f613410560333501a1a28091b28091b28094b898234987d06611a4c0d086a464112cc1a409804530b82baa0b47673f9f760dad337cf0ebc8a7e5198168e2281300f18098445c184a26820cc12460361113003654b034568d528c81b66a03c0c20e5f3ff468f06cac356878234184b10a6c17883a014284882f10061124c04087a80826a604e4e3f97ddecfce397b23e45a3f6fa2f354150138a04c23c602410e6012381b0a5305d5034106609a381304b180d842d8119281d7243417361064a3f94fb2068a054e68a82e6c00c943e285310548d8234184b10a6c15882300d660e089a8d825a603c409804e301c224981610d48cd2daf3e3a3b29bfdb9b82eeb53e4397b310ab286914098078c04c2ac611e034161281a08b384d14098254c350ab28259338a0682068a525a146401b356946d2068160aea0db34694392068360af284b104611a4c14080a4541128c070893607a832033146401d352f48bc29c9680a0c528280b8c04c2b2c02c05415528281a460361d1303520a81a0545c26447a905414d28280a26334a0b086a4641113059515a41501714e40d9311a50708ea868232c1ec2a08ea8ac23c7124184f909e18cc040579c244650182cc50d03ec35881205314b64f389618cc0585ed328e0706734561bb84e389c1425058669c080c168ac232e14462b01428d33c9132204c4b8932ad27524684693b81b2b6fe01d569a07324f485730000000049454e44ae426082","seed_name":"Alice"}]"#;
        assert!(horrible_print == expected_print, "\nReceived: \n{}", horrible_print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn print_ids_seed_name_network() {
        let dbname = "for_tests/print_ids_seed_name_network";
        populate_cold (dbname, Verifier(None)).unwrap();
        let horrible_print = print_identities_for_seed_name_and_network(dbname, "Alice", &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519)).unwrap();
        let expected_print = r#"{"root":{"seed_name":"Alice","identicon":"89504e470d0a1a0a0000000d4948445200000065000000650806000000547c2dcf0000045949444154789cedda2d72544114c5f1b08458140b8806051852b8b00236814150280a11934db002e2a8c40414e82c00159b25843e459daacccbcd9df7bafb7eccbcfe8bf3ec54fd5cdf797277777730cad54ea0dcdede76fb918787874fca277529517a226c2b23520a144f846d65400a45c984312d1227042533c6b4081c57945dc298e689e382b2cb18d33c704c51f609639a258e19ca3e83302b1813144f900f5fafca6e76f6e975599f2c60baa2786220098479c2a09e38dd503281b05d85e982e20d8232a2a01e30cd281120282b0a6a8569428902419951500b4c354a2408ca8e826a61aa50a2419806130dc26a6016a3640161124c1610b61466118a05c8c9fbd3b29b9d7ffb58d6a7df973765377bf1e669d9be2d810945914098078c04c27ac398a07882304b180d8445c1cc42e90d82d68882e6c06c45b100416b4541db60068a524a142b10b46614a4c184a1200dc6128469309620a80ac51a8449301e204c82b106618fc134a37cb8fa5176b3b3d76fcbfa7474fcaaec66d7173fcbfad4f2a2b008a5058479c04820cc034602612d30d5281a08b384d14098258c06c2e6c0cc4299038206ca5559bd3928680a33502a7343990b8206ca5559bdb928e83ecc40a92c250ad2602c4198066309c234982520a81b0a92603c409804e301c22498a5204844a901e9d1cba3e3b29bfdbabe28eb5334eafd08138a2281300f18098445c084a36820cc12460361de3003253b8a37081a28728019284a034568a004a4c15882300d26020485a32009c6038449305120a80bcabb9387e7dcefe77ee7dce87a9fb39b512410b606180984d5c234a168206c9f613410560333501a1a28091b28091b28094b898234987d06611a4c0d086a464112cc1a409804530b82baa0b47673f9f760dad337cf0ebc8a7e5198168e2281300f18098445c184a26820cc12460361113003654b034568d528c81b66a03c0c20e5f3ff468f06cac356878234184b10a6c17883a014284882f10061124c04087a80826a604e4e3f97ddecfce397b23e45a3f6fa2f354150138a04c23c602410e6012381b0a5305d5034106609a381304b180d842d8119281d7243417361064a3f94fb2068a054e68a82e6c00c943e285310548d8234184b10a6c15882300d660e089a8d825a603c409804e301c224981610d48cd2daf3e3a3b29bfdb9b82eeb53e4397b310ab286914098078c04c2ac611e034161281a08b384d14098254c350ab28259338a0682068a525a146401b356946d2068160aea0db34694392068360af284b104611a4c14080a4541128c070893607a832033146401d352f48bc29c9680a0c528280b8c04c2b2c02c05415528281a460361d1303520a81a0545c26447a905414d28280a26334a0b086a4641113059515a41501714e40d9311a50708ea868232c1ec2a08ea8ac23c7124184f909e18cc040579c244650182cc50d03ec35881205314b64f389618cc0585ed328e0706734561bb84e389c1425058669c080c168ac232e14462b01428d33c9132204c4b8932ad27524684693b81b2b6fe01d569a07324f485730000000049454e44ae426082","address_key":"0146ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","base58":"5DfhGyQdFobKM8NsWvEeAKk5EQQgYe9AydgJ7rMB6E1EqRzV"},"set":[{"address_key":"013efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","base58":"5DVJWniDyUja5xnG4t5i3Rrd2Gguf1fzxPYfgZBbKcvFqk4N","identicon":"89504e470d0a1a0a0000000d4948445200000065000000650806000000547c2dcf000004b449444154789cedd92f8e144114c771406f2659b2e108e3900814174070010c928ce200080e809a20315c00c1055008246e8e40366c3259cd52bf9047a67baaababeafdab6eea2b7ae57bf53e6a33f7efeeeeeef5da6a1128373737624b5e5e5ede0f7f9aae49144984b95a446a02c51261ae16905c515ac218e789e382d232c6380f1c539425618cb3c431415932c6380b1c559435618cd3c45143593308a505a3826209b2ffbe09df61bb27c7f0b549034614c51203c540284b1824892386d21208b5541811146b10d4220a928061a37880a0565110178685e205825a46411c986a144f10d43a0aaa85a942f106a15230de20540d4c314a2b20540ca61510aa14a608450364f3711fbec38eaf76e16b93d5fc12185794d841288dc38cb39caf82620942491fe6348ff9b9305928d220c8e328a779cdcf819945d100415e47a13ce7cfc17494445af359285a20c8f328c87b7e0ac60d05a50ea37910ca737e158a3608153b8cf6414ef39c3f05c346f9b5bd08df610f0fb7e16bd3effdf97ff40f7676ffd173de5f84c201a17217e31403a12c6024de1f83a946492d44e52e56530a84d284917a7f164a0e08925aaab6b5a0a0314c47a94cf2fd49945c1024b9544d6b4241a7301da532e9f78ba0a0d462250bd59682d104a124df2f8682628b952ec4290663014249bd3f8a520322d1f6fdf9a30e6fca1f559bf7fcd308c615257610cae230def3c7b9a3a40e42691ec67b7eac8ee23c3fd600c51a04791fc57bfe5480e9288934e74fd55166d29c3f952b0a4a1dc6e220def363b9a3a0d8612c0fe23d7f9c08caf3ab97e13beccbf5a7f0b5e9eb8747e13becd9eb9fe16b93f4fbd928b18528ce62b9c540280b188df7b350520b51b58be59402a13461b4dedf511869bdbfa330d27a7f4761a4f5fe8ec248ebfd2c14945aac66a1d252309a2094c6fbd92828b658ed4235c5602c4028e9f78ba070db6f7e84efb0ddf171f8dae43d7f9c3b4aec2094c561bce7c77245491d84d23c8cf7fca93aca4c9af3a7ea283369ce9fea1f0ab286f13e8af7fc5800097ffefe468f3aca799af363b9a3a0d4612c0ee23d7f5c1328287618cb8378cf3fed0c05d5c05c6cdf86efb0dbc3bbf0b5e9db661fbec39e1e77e16b93d4fb0904b150620b51358b951603a12c6024df2f82925a882a5daca41408a50923fdfe8e2290f4fb2751502e8cf452a5ad09e514047594ca24df3f8b8272602497aa692d286310548d82528be52cc42d05a3094249bc3f1b057160721792280663014271de1f03416c146e57dbf39f53af0ff53fa796e639bf180569c3c40e42591cc673fe14087243491d84d23c8cf7fc6a14a405e37d14cff92910d4511269cd67a3200d18cfa320aff97320280b0549c3781d85f2989f0382b25190258cf4416259cecf0541ae28287618e983a4b29aaf86823460387dbed886efb017b787f06da71210548c825a81898150adc09482a02a14e40d9302a1bc616a4050350af284691da51604b15090174ccb281c10c446411e30ada2704190080ab286691145020489a1a09660960a824451284b9c188c25882406a582822c61bcd200416a2868cd305a204815855a138e26066582422d19c702833245a19684638941b9a0502de3786050ae28544b389e18541328e32c915a4018d724ca3849a41611c62d02e57feb0fbb9fbb73882a01c90000000049454e44ae426082","has_pwd":"false","path":"//westend"},{"address_key":"01d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","base58":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY","identicon":"89504e470d0a1a0a0000000d4948445200000065000000650806000000547c2dcf0000047249444154789ceddd4d8a145914c5f1762c3da835943b6868ba7a070d8d0d0edc400d1a7a410d0e72030e044570079608ee40d75003715cbe831c30226fdd7cf1debb1f59f1fe8313d384dfe89241e6a3bbbbbb5f66b93a0b94dbdbdb611ff2e2e2e25179a42e25ca48845365444a81e28970aa0c48a1289930d645e284a064c6581781e38a724e18eb3c715c50ce19639d078e29ca43c25867896386f2904198158c098a27c8bb6fafca2efbebf1b3b23e59c00c45f1c4401208f38441237186a1640261e70a3304c51b046544412360ba512240505614d40bd38512058232a3a01e9866944810941d05b5c234a14483300d261a84b5c06c46c902c224982c206c2bcc26140b908fcf3f975df6fbcb27657d7afbf979d9657f3f7959766c5b6042512410e6012381b0d13026289e20cc1246036151305528a341d01e51500dcc49140b10b45714740a66a228a544b102417b46411a4c180ad2602c4198066309829a50ac419804e301c224186b10761f4c37cae1eabfb2cbae6ffe2febd3d33f2fcb2e7bf3fe4b599fae0e87b2cb6eaeafcb9e6e134a0f08f380914098078c04c27a609a5134106609a381304b180d84d5c054a1d480a0897228ab578382d63013a53137945a1034510e65f56a51d0cf3013a5b194284883b104611a8c2508d360b680a061284882f10061128c07089360b6822011a50564442f848bfe5fc78bfe9370d1ffe674d1af234c288a04c23c6024101601138ea281304b180d8479c34c94ec28de2068a2c80166a2284d14a1891290066309c234980810148e8224180f1026c14481a021285f5f7d2bbbecd7678fcbfaf4cfd5f145fdfa66fb45ddda1f4f2fcb2efbf0e64bd9b6ba512410e6012381300f180984b5c274a16820cc1246036196301a086b8199281d4d14a1892234512ecbea4d14a1dda1200dc61284693096204c83690141dd284882f10061128c070893605a41d01094dee645bf2c1c4502611e3012088b820945d14098258c06c2226026ca89268ad0ae519037cc44390e20e5f1e33b7a34518edb1d0ad2602c419806e30d8252a02009c6038449301120e80805b5c0cc77890f65970d7b97186d45914098078c04c23c602410b61566088a06c22c6134106609a381b02d301365406e28a81666a21ccaead5a2fc0c82264a63ae28a80666a21ccaead5a0ac4150330ad2602c4198066309c234981a10548d827a603c409804e301c224981e10d48dd2dbfc15a3e3ee4541d6301208f38091409835cc7d20280c45036196301a08b38469464156307b46d140d044514a8b822c60f68a720a0455a1a0d1307b44a90141d528c813c612846930512028140549301e204c82190d82cc5090054c4fbbffff1496054602615960b682a02614140da381b068981610d48c822261b2a3b482a02e1414059319a5070475a3a00898ac28bd2068080af286c9883202040d43419960ce15040d45619e38128c27c8480c6682823c61a2b200416628e821c35881205314f690702c31980b0a3b671c0f0ce68ac2ce09c7138385a0b0cc3811182c148565c289c4602950d67922654058971265dd48a48c08ebce02656f7d07d177bb7335a728eb0000000049454e44ae426082","has_pwd":"false","path":"//Alice"}],"network":{"name":"westend","logo":"westend"}}"#;
        assert!(horrible_print == expected_print, "\nReceived: \n{}", horrible_print);
        fs::remove_dir_all(dbname).unwrap();
    }
    
    #[test]
    fn show_all_networks_flag_westend() {
        let dbname = "for_tests/show_all_networks_flag_westend";
        populate_cold (dbname, Verifier(None)).unwrap();
        let print = show_all_networks_with_flag(dbname, &NetworkSpecsKey::from_parts(&hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").unwrap(), &Encryption::Sr25519)).unwrap();
        let expected_print = r#"[{"key":"0180037f5f3c8e67b314062025fc886fcd6238ea25a4a9b45dce8d246815c9ebe770","name":"rococo","logo":"rococo","selected":"false"},{"key":"018091b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3","name":"polkadot","logo":"polkadot","selected":"false"},{"key":"0180b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe","name":"kusama","logo":"kusama","selected":"false"},{"key":"0180e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","name":"westend","logo":"westend","selected":"true"}]"#;
        assert!(print == expected_print, "\nReceived: \n{}", print);
        fs::remove_dir_all(dbname).unwrap();
    }
}



