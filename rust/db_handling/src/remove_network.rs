use parity_scale_codec::Encode;
use sled::Batch;

use constants::{ADDRTREE, METATREE, SPECSTREE};
use definitions::{error::{ErrorSigner, ErrorSource, NotFoundSigner, Signer}, helpers::multisigner_to_public, history::{Event, IdentityHistory, MetaValuesDisplay, NetworkSpecsDisplay}, keyring::{AddressKey, NetworkSpecsKey, VerifierKey, MetaKeyPrefix, MetaKey}, network_specs::{CurrentVerifier, NetworkSpecs, ValidCurrentVerifier, Verifier}, users::AddressDetails};

use crate::db_transactions::TrDbCold;
use crate::helpers::{open_db, open_tree, get_network_specs, get_general_verifier, get_valid_current_verifier};
use crate::manage_history::events_to_batch;

/// Function to remove the network with given NetworkSpecsKey from the database.
/// Removes network specs for all entries with same genesis hash.
/// Removes all metadata entries for corresponding network specname.
/// Removes all addresses corresponding to the networks removed (all encryptions).
/// If ValidCurrentVerifier is Custom(Verifier(None)), leaves it as that. If ValidCurrentVerifier is General, leaves it as General.
/// If ValidCurrentVerifier is Custom with some Verifier set, transforms CurrentVerifier from Valid into Dead to disable the network
/// permanently until Signer is wiped altogether.
/// Function is used only on the Signer side.
pub fn remove_network (network_specs_key: &NetworkSpecsKey, database_name: &str) -> Result<(), ErrorSigner> {
    let mut address_batch = Batch::default();
    let mut meta_batch = Batch::default();
    let mut network_specs_batch = Batch::default();
    let mut verifiers_batch = Batch::default();
    let mut events: Vec<Event> = Vec::new();
    
    let general_verifier = get_general_verifier(database_name)?;
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    
    let verifier_key = VerifierKey::from_parts(&network_specs.genesis_hash);
    let valid_current_verifier = get_valid_current_verifier(&verifier_key, database_name)?;

// modify verifier as needed    
    if let ValidCurrentVerifier::Custom(ref a) = valid_current_verifier {
        match a {
            Verifier(None) => (),
            _ => {
                verifiers_batch.remove(verifier_key.key());
                verifiers_batch.insert(verifier_key.key(), (CurrentVerifier::Dead).encode());
            },
        }
    }

    {
        let database = open_db::<Signer>(database_name)?;
        let metadata = open_tree::<Signer>(&database, METATREE)?;
        let chainspecs = open_tree::<Signer>(&database, SPECSTREE)?;
        let identities = open_tree::<Signer>(&database, ADDRTREE)?;

    // scan through chainspecs tree to mark for removal all networks with target genesis hash
        let mut keys_to_wipe: Vec<NetworkSpecsKey> = Vec::new();
        for (network_specs_key_vec, entry) in chainspecs.iter().flatten() {
            let x_network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
            let mut x_network_specs = NetworkSpecs::from_entry_with_key_checked::<Signer>(&x_network_specs_key, entry)?;
            if x_network_specs.genesis_hash == network_specs.genesis_hash {
                network_specs_batch.remove(x_network_specs_key.key());
                events.push(Event::NetworkSpecsRemoved(NetworkSpecsDisplay::get(&x_network_specs, &valid_current_verifier, &general_verifier)));
                keys_to_wipe.push(x_network_specs_key);
            }
            else if x_network_specs.order > network_specs.order {
                x_network_specs.order -= 1;
                network_specs_batch.insert(x_network_specs_key.key(), x_network_specs.encode());
            }
        }
    // scan through metadata tree to mark for removal all networks with target name
        let meta_key_prefix = MetaKeyPrefix::from_name(&network_specs.name);
        for (meta_key_vec, meta_stored) in metadata.scan_prefix(meta_key_prefix.prefix()).flatten() {
            let meta_key = MetaKey::from_ivec(&meta_key_vec);
            meta_batch.remove(meta_key.key());
            if let Ok((name, version)) = meta_key.name_version::<Signer>() {
                let meta_values_display = MetaValuesDisplay::from_storage(&name, version, meta_stored);
                events.push(Event::MetadataRemoved(meta_values_display));
            }
        }
    // scan through address tree to clean up the network_key(s) from identities
        for (address_key_vec, entry) in identities.iter().flatten() {
            let address_key = AddressKey::from_ivec(&address_key_vec);
            let (multisigner, mut address_details) = AddressDetails::process_entry_checked::<Signer>((address_key_vec, entry))?;
            for key in keys_to_wipe.iter() {
                if address_details.network_id.contains(key) {
                    let identity_history = IdentityHistory::get(&address_details.seed_name, &address_details.encryption, &multisigner_to_public(&multisigner), &address_details.path, &network_specs.genesis_hash);
                    events.push(Event::IdentityRemoved(identity_history));
                    address_details.network_id = address_details.network_id.into_iter().filter(|id| id != key).collect();
                }
            }
            if address_details.network_id.is_empty() {address_batch.remove(address_key.key())}
            else {address_batch.insert(address_key.key(), address_details.encode())}
        }
    }
    TrDbCold::new()
        .set_addresses(address_batch) // upd addresses
        .set_history(events_to_batch::<Signer>(database_name, events)?) // add corresponding history
        .set_metadata(meta_batch) // upd metadata
        .set_network_specs(network_specs_batch) // upd network_specs
        .set_verifiers(verifiers_batch) // upd network_verifiers
        .apply::<Signer>(database_name)
}

pub fn remove_metadata (network_specs_key: &NetworkSpecsKey, network_version: u32, database_name: &str) -> Result<(), ErrorSigner> {
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    let meta_key = MetaKey::from_parts(&network_specs.name, network_version);
    let mut meta_batch = Batch::default();
    meta_batch.remove(meta_key.key());
    let history_batch = get_batch_remove_unchecked_meta(database_name, &network_specs.name, network_version)?;
    TrDbCold::new()
        .set_metadata(meta_batch) // remove metadata
        .set_history(history_batch) // add corresponding history
        .apply::<Signer>(database_name)
}


fn get_batch_remove_unchecked_meta (database_name: &str, network_name: &str, network_version: u32) -> Result<Batch, ErrorSigner> {
    let events = {
        let meta_key = MetaKey::from_parts(network_name, network_version);
        let database = open_db::<Signer>(database_name)?;
        let metadata = open_tree::<Signer>(&database, METATREE)?;
        match metadata.get(meta_key.key()) {
            Ok(Some(meta_stored)) => {
                let meta_values_display = MetaValuesDisplay::from_storage(network_name, network_version, meta_stored);
                vec![Event::MetadataRemoved(meta_values_display)]
            },
            Ok(None) => return Err(ErrorSigner::NotFound(NotFoundSigner::Metadata{name: network_name.to_string(), version: network_version})),
            Err(e) => return Err(<Signer>::db_internal(e)),
        }
    };
    events_to_batch::<Signer>(database_name, events)
}

#[cfg(test)]
mod tests {
    use crate::{cold_default::{populate_cold}, manage_history::{print_history}};
    use super::*;
    use std::fs;
    use sled::{Db, Tree, open};
    use definitions::{crypto::Encryption, keyring::{MetaKey, NetworkSpecsKey}, network_specs::Verifier, users::AddressDetails};
    
    fn check_for_network (name: &str, version: u32, dbname: &str) -> bool {
        let database: Db = open(dbname).unwrap();
        let metadata: Tree = database.open_tree(METATREE).unwrap();
        let meta_key = MetaKey::from_parts(name, version);
        metadata.contains_key(meta_key.key()).unwrap()
    }

    #[test]
    fn remove_all_westend() {
        let dbname = "for_tests/remove_all_westend";
        populate_cold (dbname, Verifier(None)).unwrap();
        
        let genesis_hash = "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let network_specs_key = NetworkSpecsKey::from_parts(&hex::decode(genesis_hash).unwrap(), &Encryption::Sr25519);
        remove_network (&network_specs_key, dbname).unwrap();
        
        {
            let database: Db = open(dbname).unwrap();
            let chainspecs: Tree = database.open_tree(SPECSTREE).unwrap();
            assert!(chainspecs.get(&network_specs_key.key()).unwrap() == None, "Westend network specs were not deleted");
            let metadata: Tree = database.open_tree(METATREE).unwrap();
            let prefix_meta = String::from("westend").encode();
            assert!(metadata.scan_prefix(&prefix_meta).next() == None, "Some westend metadata was not deleted");
            let identities: Tree = database.open_tree(ADDRTREE).unwrap();
            for x in identities.iter() {
                if let Ok(a) = x {
                    let (_, address_details) = AddressDetails::process_entry_checked::<Signer>(a).unwrap();
                    assert!(!address_details.network_id.contains(&network_specs_key), "Some westend identities still remain.");
                    assert!(address_details.network_id.len() != 0, "Did not remove address key entried with no network keys associated");
                }
            }
        }
        let history_printed = print_history(dbname).unwrap();
        assert!(history_printed.contains(r#"{"event":"database_initiated"}"#) && history_printed.contains(r##"{"event":"network_removed","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"2","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","current_verifier":{"type":"general","details":{"public_key":"","identicon":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea20000002e49444154789cedcd410100200c0021ed1f7ab6381f8302dc99393f8833e28c3823ce8833e28c3823ce8833fbe20724cf59c50a861d5c0000000049454e44ae426082","encryption":"none"}}}}"##) && history_printed.contains(r#"{"event":"metadata_removed","payload":{"specname":"westend","spec_version":"9000","meta_hash":"e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce","meta_id_pic":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a7b49444154789c7557797454e515ff7d6f9b7933938490b0c4844d88249ab21920090812a0543d08882d52e45841ed1f504e59542a15b5156dcf11bb0922682d88d6a341b11e721009a8c510031494352cb22621404226b3bc99795b7f6f222db57a73be33f3de7cdffdddfbbbcb77235cd7c5f788cc6573a5c5b6ed5b4dd3bc2b659a6385eb1464f8f99b00e249c8a623b56aaa52abaa6ab5a2289f7adbb93c51b82caeff93ef03fecf81542a3533994c2eb02cabcc715c68fcc53005aa1be4f4e7c4420bf9992e62295a2a09c8b27c4cd3b4753e9f6fb5102241159e030ed7ff007d173055c3b26cbb341e8bbd4e2f4b6c6e11fca35e5b928498bbc927de3fa2b87cc6a09e8ea89a95707375d74d5ade1610cf85aa4a97743d308f4654511f5fa7859a3ae5dbc069507af8503c1e5fe779e853252be45704bfca92e2a0e6a88bbbdfd091adf305d5357608bc7867120bc69b3049b022c1310c611b0654cf005dd75fe45a4cbd9ef04427f8f5c06950c3309ee55ae6728f9fa02ded4965cdb6f338772581e923bb6150611eca5669b402d048e245027f30378182a88b575e5790110466ceb2507cb3e3c4e370092ed3eb0f42a1d054eaf7240d7e0d380dea791a8b45d709213126a44b96a43b56ecc3ae2f2e017ea270ebf615a5d87b350fcfd608b88ec0bd432cfc6a98893b27e9686af2de01030a1d6cd99a4056b6e3dab6b05cd751fd7e7d552818984f1c2a82ed01a7bfd8b6551aee88edf1debbaeed047c9af4d59936542cab834ed714462f1c4e62d6f8026c5c3204074f5b882505ca8a1cac592d61f1421ff2f25ccf365c6a1158b32e8159f70391ab5e420acb711c45f8f5397e9fef756e513c607ebae8e8881f74ecd6125df9d4e276c5457fb4c54760c4d2cfd1d814831654916a4be0d54787616049084f7c7e142603fff321052809f7c6b8892a74bfe701d591cc5d9f98a893cfe14f7b2f20a0cacef2d21ba5f105b909a107fb2ab2d492064ea5923323d1c45b21f53d4b911a14d70dd0eb38146d32aaffd5170bd77f89e6b0859f8dee8905f7f4c1f0aa3a84a3ac142fd004df71df709cdcd2032bff20339b5d2c7dcc46c1844bb87d2309546885e5202f3b68d64d1ba1f6c8ca78c1e7d71f4d03774462bb6d2b5c96a16ea0bd8eec42625d18485a8510be9948c4dad01636d0bf5f1eb69e68c21d9bbf40a6aed131d21f4be0f9d1c5583aba10874f47a1a8c0c082107ebdf318567c710259411f8bd8452461bad5934bc5c45edd5aa1078a846599b7767474eca56908289b5822c7e884ce48c76148d36036e522f4e587501c0bc96e37e272f1188cdc5c8fe6d68e4e8f65197b7e5c81eaf62b78e9c2593ae7e2f7850351a4043166532d20714fa7c7a89f5e6667a8b2ec0b041e112c9de5f178ec192154f689b09ca5d6535f0409bb0f628912647eb21122da0657f5b13f4620954fc6671903b072ff19842d174b8a7b203b4bc6d8da3a1ac1e22083240207468fc2a1e608d61e3d830c55c5e221fd51deb38b134d5992dfa76d11f4b6c634ad4a08c709c921e954fc0a4ec71b3128bb18dde361a0e635488ac6c223fd892892034aa18d9a82338777219a4a62484905569fb9807907f621d3af33b3808e44021b860ec6ecde7db0abc54277bfc04d19365a138ed7ed04e58a0887c30d29cbbca98b1a743734ee10f38fae8561c57153a817de2f5d8ea2afeae09cda0be1814b32cc0973b0e6bd95f8c7bb2fa59d2b2f1b8f398bd7e3b67dfbd11289d063c638bb0bb60f1f8105fb74545f94d32d6041610acb8a2d4459330487680fb71f716db738e9984ee9ee25d205e322a9c940c4b88cb937de83578b1e41f8c42e2809034abf4138dada8425f347c1e797a030932e5f4ee2a927d7a0efd8d95877fc109951b0884958d39c8907ea15e407c0b0316569d0f6db1328ca7060d8dcd6dede7e4438280edb716768ed22e9aa194586a2239c68c58c8209786be86308371e869c8843eb5b82830df558ba703c42193ee68d42e018162f7e11774c5b884da78e921505b3f273f097e31a161ed0501070bddc42849e56df964069570731cb030eb7379869aa43eed327df162b8eaf6794d821e9f596912b30eefc455807b71384dd3f988df898fbb0e28f0fe1b31d1f9333d25ad8074fadd88619a71ab1b7a59927057e94df136b8a8761ca2e0d5fb6731365561f0baf94a690a41182cf5e72d5f2ea2b67c01d9fa44935ad0d3814398771dd06a1540ec0d9b69631619058368877c02e198bf8cd1538f4cfbf73083030b67c2ade8983b4d622f44d72459329bc3f7c28c677cdc7e666811e3ea0222749cf5dd634ed15222278fdbdc0925acc8b81e564c95dd513046947caee8b48b42bb276fe1530537015360c023b83c7e17cff726c387611572d07f30b7371d4b88aa9f5fbc032612cd9094d0b3b2bca90edc8f85bc33974f1a9b87f606ff4d07d4ed2b6254d553f13f4b63212e9a8612b777579ab50a47a1e56c9a28b843413ee0903fe03d5a4876675c943a27c0a2a3f3a8c03679a69ba84accc20eaa697e3c9af4fa2aaa98959ede297fdfb636ecf1b30ac6a374cc364663918da2b07dbef1eeedd4ab21e083cce4f47e6ad74c8b1234519ea7a876d5ee2154f3ae26c994570d41908188d48c4e208dcd00bdb4eb761d2e63a84e8852c44ba65ae1c538245157d51db72c5b30565ddbae1e94f4ee099ba636c991cce684c3469ba5bef1e2e2af3730cb6cc1f10989e25128b48f9ca90f6b629a3896d9eb1723b58bb95387ca114cbab8ee1ebcb29fca2320fa38665e196b73e67a41811a2d8f114aa7e320cddcfe663e50b0acbcc6596db38dff302ee7d673f94a0e6cd510868aab567fa48a530a7cb4655d767a781797bfbdbc3c6590967bbebca36f67443b2d19bf53709954fedc7feaf2e03bc16f9021f3e3d021743261edfe5f574170f96e4e3e1dc81183fc1c7cb867cb166f37abad8b923853f9f6bc0ba834d9c5484fb9be103f0f0cd05c2f5076ed114e58807cc060b8bd3e4bdbc1adf958449669252c09729f69feec0a865b508fa954e5a3908fcb4b2006f3e3a04271a63a0b318dc4fc5cbab142c59a4c11b043c69e120f0ca5aee9dede064638a838430737c8aeaa8bee702babe8c5bae0d020c2a8d358cf84ac3482c82904d4938aa653b287fa21e274fb26707691fbbc09bcb0643cfe98327b74acc7c090f552431216463e2449dde0a2f8f100ab9f8a8c640df7e24cf94532c228d6d6e7b5646c644e278c251ac13d8ab694fdc6834ba39954a4ee19d6c057db2b4f75458fa6dd52934b525316d4437ccf9e10054bc4c5a3907f8684bd810d839cfc0c5dd02abd6aa08b245ce7d9833d85db61b8b713617aeaac8f2f1ccccccc1a273ce66fac1b906ec89079e7e60a2bdc4849be73d047d8ac96b57314c576486043e3e024c7dc3875c02907d3447047e37298985134c74440538f0bb5ece45a31e8bae50556d7b28149cfc0d28df75fe97713db02754d5099e4ca61e24f5ab4dcbf6f3104104897545dc1262ea1bbad87b81d6504d6ec0c5870fb0f9e7380e73cfa502d99bc7158e3c7ebfff39ced45e4c3de1ee4e504fbe0dec09cfa6e9b03919f6e0c8bb84eb417ecfa13eb03a7086fdf7b53d2aff8501660cb250d6db4e539fbeee8464689aba89a0cfcbb24c7ed2e2e963f4ff2bdf057c4d1841c688c23db9ccfa69ec725338348c54653737a4751e4c98eca49614d11479bfa2aa5b38bc6fe285728a3f79e2e9b0b9d27baf977f03ce7bfeaa1a7a59e70000000049454e44ae426082"}}"#) && history_printed.contains(r#"{"event":"metadata_removed","payload":{"specname":"westend","spec_version":"9010","meta_hash":"70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf","meta_id_pic":"89504e470d0a1a0a0000000d494844520000001e0000001e08060000003b30aea200000a5c49444154789c7557097094e5197efe6b77ffddcd6e0e12926042141209b7805c2d62b0161404416c45eb50aa838822d3227514eb206391ce20d54e2b531c04045b6ba1141c4530448e1a8d342a060202f180841c24d9237bfe679fffa7d86aedbbfbcdfec7f7bdcff73eeff71e2bd8b68dff231287c9e18a699a63755d9fc931d5b6edab0445e13b01b6a14b54d2232b4abda2286fc9b27cd899cee188cc6170fc8f70cd77027fbd40d3b405d96cf611c330263a730551846d9ad03a2ebabf9efec5107d2aaf0d08820849924e7b3c9e97bc5eef8b822064a8c231c0e2f806d07701bba0b4705c3299dc62e8fa70678e2070b220984416e28d0d42f662abfb500ee70ab913be6f0b5eaf0dcb14a98d3400a22c77f955f5216e6227f571b52b7c7359be0dec82d2c2fb53a9d44bb66d41941443f2f8380f1231916e3f8f58fd51081e0f710598e93472465e077fe510d83aad1645cbb64cd3320d85baa0aaea068e15bc74c4d9800b4885eeaf232e683a9d7e86639533439015434f46e5aee3ef408b7723aff27a844a87a3a7ee2d828a344b8495c92077d2148821959b3a074152e0eb3710b23fec6c80ea6d8956ef090683b753a5238e6afb0ab00bea589a48265f1205c1729e8b922c7eb66b2d7acf3640527c9c02542f781a725641a2f938ef00ef803204aa8722da7c08a696e61352e3cb41ded01be95d0f95db8665590aadfe43c0ef7f187cca613ac0ee050fcfb8645fdf3182825bb5648f4f8c777e8993afae82a838b48ad05331148dbc098367af40a6a395478687abb004c9b66624bef808a247a52a3ed633c819341eb9fdaf86a16b244620b82dcbaafa339fd7bb8553640798bf40a2afaf2996c90e3f194d1969c3948bfd3e540544346d7f1ce9de8b907d0168895e54deb60279c3c6201efdd8254df55d0d590f22d27c90f78e0d363f024ab8c123310defb4b6c343bfcfaf18208ec80b673cc16085248a9d2eb01332a964e24fef5fea33da925999139165788c2f2d4251cf199c7b771bcc4c1cf9832763c00df31189d431aed3544f3f13262fbf065a7784e31cef44848b2bd1e22dc4eac64f411068a68552bfaaff7a4cb552100caef7aaea4a1798d6be9fc86a13df6d8f9a74aee4d0ad599cec533065501992915ee8b15ee4545c8b54ec4b447a0ed1ff5e82d2472669cd1985a2e231e83cff052cbaa4ac7c20b67fda8c573f6f45be570121d0a71bf6af460f1146e5e7f62881c01041378cb17df1f83f1db0face185a931978193659ee724259110abf68c1994dafc0ce6a088f1e8ef28577229a380a2d1ba5df250e11c5c533b0e3779bb17bdb566ec4c4b2c71f47e5dc3bf0e89106289274d908bf8a7563879b3e499454bf7fb1c0d0798a31fb34293153ba2e9deed391b40414fb445c23db38f1c433c8b47742e242ad378241cb96a060fa3868b166c6848940b00a273e6cc1e25b67c01f088027987403af1eac453b0fde9b2dad087864cc2bef8fca508e95314dd1ebf1bc29c4e3f183ccbfd338d752bcaaa8c7ba908e74205456855447044d2b9f644af4f2dc70e791284a6ffd21aa962d46d3b10f90c9a631e97b35d8bd650b9e5af2000a4b4ae8721bdd1d1d58bb651be6dcb3003d5d2df0282a5369081953b7791e9977846e21168b7dc6b458c5536b5f3a7948f8b27633cc6c0a6a411986fc78152e6cde89ce03b504f731a1c818bd6e35361f78053bb6bec0b0b3515333138f3ef22c96cc9c85cfcf9c71ad1d39761c36beb11749a309a9741b9124e4f8ab100e0ce5be9c1220408845a3cd3c51d5b6a95b27b63f266669b1ac06918d5d42e9c4b91878e322b4eddc836c248eab6e9a820b521af7de3315aa3f005956d0d979111b9edb81b14327e16f2fbfcc981531ebde85c829d4d17ee92814394030931bb251945bc3fb30ef0dfb32b08d6a5a699d7865a5686493cc52f42763b670c43454ce66b2d0cec3b4b3c80d56e0a38606dcbfe416844279a44f4247672bd63cb511f3efbc0f7b6a77317c24fc60d22db4f42cba638d90253f81e8775a5a983b95b4e73bd7048e4549b5e152dd5affba70e1f00e52c1d4a2e6a0faced540288b78a491ec88b43088bcbca978e2c987f1f681dda410183674145ef8ed5ff0c8d34b51f75e1de907e6ce988b3faedd848eee43c8ea11875804fc1528085f4f50a742521d0f573d0fd7243adc126545ecbb781aa9ee0b08978d80279c8beeae7d9cc5a5f49369a610ca194e9a87a1be613fad4a62fab4dbb1fff0db58b4f26e14f3143b79e1526f17b6ad7f1db367dc829ec43986941fa2990f7a93baf815843e81a1b49e21b582e5cc646197749e482b95825c500829cf8fde9e5a5a61d077320c23498bc7424bf7c7be3fef4022de873b7eba0827db4ee3470fce46617e11e75a88f7c5b077eb3e94fbaec5e1d78f21105631f18e11c8e917b08cacc1d4af1c1168ed345a7d505414bbef934621d5f219434726193672274c8595a791ea63bc0714fa27141a8f65f3eec207470ebb25ada4bc1c2f1f3880df6c5d87ddfbfe4a564c3cb4683916cf5c8ef5f76c422a9a66a1b05035f16a2cd978178f932df9fdfec7041622299e489cd053a921d1a37516ad169d98b5b259784b0710fc06e8995e5a9b466efe40341e398aa5736621949b47164474b5b7e389e79fc74f962ec57b0dff208d12268f9f843736d662ef863a848b72400ca6da8cbd64d302a1f2fa81698fe81de1e6ea4c26f30b52fe5cecfda3ba11e951048f17563a85e0b051f05d53c192d7c4d84ed3ef9568efcd62e18d6482ca647621b19e6e6c786d27a6cc9a8c68e404816504c38371aaae1b9b97bf86406e8085de802fe831966f5f28f71f58b8c3e7f5ddeb0273f8e2c9e457d9aecea244d3c796ad69a2421fe78c1a83d8d97aa6c70ed2ef8165682819331d6fec39802deb9e758a3866dc75371e58f573b475ee87696924df7643a8b4e866ec7fb101c7f6b242c9a23dfdc12998386fb4406b87c98adcec003bae32581ae727d369d74996ae89923f2818c928a227eb18499cc2936de9a49f6d4df97535387fe102e35b4355c5205cbc741cbdf146c6f5e546c0a95805e1f108d3f268572f08a4ab61af224359abfad5559cf275232071980edd0eeda224e9f48b5bcfa2a70e414f44d8352a3c241a8a874cc607ba1f3bbf6a85cef7d38afa61f65501b4f5becb48217bfc8882c2645103590c709dc0e2647998586a43a1d0cdc47144b802cce872c54e24127f67ef35c7695768a9a82723628ab16d6a1904f24b902eb806bffcf05324d9512a2c9f699ee235634662b03f8aeec459b6a23202ea20a8de52aa36d9f2580a9bfc33041dc5f875fa6c91c3ba02ec8803eeded0f2dfd3f2879c070c2d9dfb932dcb14fcecbd8e77f760cd27a71052143e067a58a7efabacc0bc8a72c448bd445b986e98fe0d89ba058f47a90d0482b7fd1bd46596e31bedad235475199c3e5fc40dbc4897fbb8886f0493ab84b46909ab3f39259c8d271c106e40c69aeb86a23ca85ae494a07000395d84cfe75bcbeed2f1a9235c7e19d4916f033bc2b52e1d2669ea4fda1fe558c4eb0267a69731de95c9e2edd64e7616266e282e4475388894c13f197ccfd84ef33fd42e823ecb22d2cc478e38fa982fff23df057c45640ea778f28cd9fdc8c05c66b9396c9526f0453f9f2cb90bd9c80959cbeaa3233f26e09b6cde7711bc85af1ce154d74a77ee7fcbbf00a20127bd2177af5c0000000049454e44ae426082"}}"#) && history_printed.contains(r#"{"event":"identity_removed","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","path":"//westend","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}"#) && history_printed.contains(r#"{"event":"identity_removed","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","path":"","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}"#) && history_printed.contains(r#"{"event":"identity_removed","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","path":"//Alice","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}"#), "Expected different history:\n{}", history_printed);
        fs::remove_dir_all(dbname).unwrap();
    }

    #[test]
    fn remove_westend_9010() {
        let dbname = "for_tests/remove_westend_9010";
        populate_cold(dbname, Verifier(None)).unwrap();
        let genesis_hash = "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e";
        let network_specs_key = NetworkSpecsKey::from_parts(&hex::decode(genesis_hash).unwrap(), &Encryption::Sr25519);
        let network_version = 9010;
        assert!(check_for_network("westend", network_version, dbname), "No westend 9010 to begin with.");
        remove_metadata (&network_specs_key, network_version, dbname).unwrap();
        assert!(!check_for_network("westend", network_version, dbname), "Westend 9010 not removed.");
        fs::remove_dir_all(dbname).unwrap();
    }
}
