use parity_scale_codec::Encode;
use sled::Batch;

use constants::{ADDRTREE, METATREE, SPECSTREE};
use definitions::{error::{ErrorSigner, ErrorSource, NotFoundSigner, Signer}, helpers::multisigner_to_public, history::{Event, IdentityHistory, MetaValuesDisplay, NetworkSpecsDisplay}, keyring::{AddressKey, NetworkSpecsKey, VerifierKey, MetaKeyPrefix, MetaKey}, metadata::MetaValues, network_specs::{CurrentVerifier, NetworkSpecs, ValidCurrentVerifier, Verifier}, users::AddressDetails};

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
    
    let general_verifier = get_general_verifier(&database_name)?;
    let network_specs = get_network_specs(database_name, network_specs_key)?;
    
    let verifier_key = VerifierKey::from_parts(&network_specs.genesis_hash.to_vec());
    let valid_current_verifier = get_valid_current_verifier(&verifier_key, &database_name)?;

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
        for x in chainspecs.iter() {
            if let Ok((network_specs_key_vec, entry)) = x {
                let x_network_specs_key = NetworkSpecsKey::from_ivec(&network_specs_key_vec);
                let x_network_specs = NetworkSpecs::from_entry_with_key_checked::<Signer>(&x_network_specs_key, entry)?;
                if x_network_specs.genesis_hash == network_specs.genesis_hash {
                    network_specs_batch.remove(x_network_specs_key.key());
                    events.push(Event::NetworkSpecsRemoved(NetworkSpecsDisplay::get(&x_network_specs, &valid_current_verifier, &general_verifier)));
                    keys_to_wipe.push(x_network_specs_key);
                }
            }
        }
    // scan through metadata tree to mark for removal all networks with target name
        let meta_key_prefix = MetaKeyPrefix::from_name(&network_specs.name);
        for x in metadata.scan_prefix(meta_key_prefix.prefix()) {
            if let Ok((meta_key_vec, meta_stored)) = x {
                let meta_key = MetaKey::from_ivec(&meta_key_vec);
                meta_batch.remove(meta_key.key());
                if let Ok((name, version)) = meta_key.name_version::<Signer>() {
                    let meta_values_display = MetaValuesDisplay::get(&MetaValues{name, version, meta: meta_stored.to_vec()});
                    events.push(Event::MetadataRemoved(meta_values_display));
                }
            }
        }
    // scan through address tree to clean up the network_key(s) from identities
        for x in identities.iter() {
            if let Ok((address_key_vec, entry)) = x {
                let address_key = AddressKey::from_ivec(&address_key_vec);
                let (multisigner, mut address_details) = AddressDetails::process_entry_checked::<Signer>((address_key_vec, entry))?;
                for key in keys_to_wipe.iter() {
                    if address_details.network_id.contains(key) {
                        let identity_history = IdentityHistory::get(&address_details.seed_name, &address_details.encryption, &multisigner_to_public(&multisigner), &address_details.path, &network_specs.genesis_hash.to_vec());
                        events.push(Event::IdentityRemoved(identity_history));
                        address_details.network_id = address_details.network_id.into_iter().filter(|id| id != key).collect();
                    }
                }
                if address_details.network_id.is_empty() {address_batch.remove(address_key.key())}
                else {address_batch.insert(address_key.key(), address_details.encode())}
            }
        }
    }
    TrDbCold::new()
        .set_addresses(address_batch) // upd addresses
        .set_history(events_to_batch::<Signer>(&database_name, events)?) // add corresponding history
        .set_metadata(meta_batch) // upd metadata
        .set_network_specs(network_specs_batch) // upd network_specs
        .set_verifiers(verifiers_batch) // upd network_verifiers
        .apply::<Signer>(&database_name)
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
        .apply::<Signer>(&database_name)
}


fn get_batch_remove_unchecked_meta (database_name: &str, network_name: &str, network_version: u32) -> Result<Batch, ErrorSigner> {
    let events = {
        let meta_key = MetaKey::from_parts(network_name, network_version);
        let database = open_db::<Signer>(database_name)?;
        let metadata = open_tree::<Signer>(&database, METATREE)?;
        match metadata.get(meta_key.key()) {
            Ok(Some(meta_stored)) => {
                let meta_values_display = MetaValuesDisplay::get(&MetaValues{name: network_name.to_string(), version: network_version, meta: meta_stored.to_vec()});
                vec![Event::MetadataRemoved(meta_values_display)]
            },
            Ok(None) => return Err(ErrorSigner::NotFound(NotFoundSigner::Metadata{name: network_name.to_string(), version: network_version})),
            Err(e) => return Err(<Signer>::db_internal(e)),
        }
    };
    events_to_batch::<Signer>(&database_name, events)
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
        assert!(history_printed.contains(r#"{"event":"database_initiated"}"#) && history_printed.contains(r##"{"event":"network_removed","payload":{"base58prefix":"42","color":"#660D35","decimals":"12","encryption":"sr25519","genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e","logo":"westend","name":"westend","order":"2","path_id":"//westend","secondary_color":"#262626","title":"Westend","unit":"WND","current_verifier":{"type":"general","details":{"hex":"","identicon":"","encryption":"none"}}}}"##) && history_printed.contains(r#"{"event":"metadata_removed","payload":{"specname":"westend","spec_version":"9000","meta_hash":"e80237ad8b2e92b72fcf6beb8f0e4ba4a21043a7115c844d91d6c4f981e469ce","meta_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000032f49444154789cedda3d6e534114c571a7a0414a6109c95b70c1265c221640431189928e85d05122a5a0610188d29ba0f0162c21b988444361e6c83af1f8793cf7dcf9b06cf3fec5a4784f37777e456425b9db6eb793ffbdb3206c369be26f329d4eefc297ae7541a8b9b4550f94a6083d2f3fac2546138492cbcf3fbf0ce761ab4f7fc2e9ab05461542c9e5510a809540a01a8c62841e00ecdc104508a500a827022a817021d45c9ef546601e0c19a105003a1702522124040fc0ecc3329c87adbf2ec2b9cb8bf07a7efcfeafd5feb995026122d4023015c202602d219a21e4009805a1023015a20a4105405e04ab960828077112c103802e1d019d8248227801d03520a014848c70ffe24b380f7bfafb319cbbbc08b3c7e3f7d70ffbe75e046b1e93105400a64258002c5e3c076101b0781e1b42980839006641a8002c5e3c05a102b0781eca220c019017c1aa64e95ca5f362881121f48c900240b78a8008d11ce1fec7f1fb4f6ff7cfbd4bcf96c7efaf17d173e7bc381901e5202c00a642c40ba700980a11cf1be64240290815805910f1c239006641c4f3521d2058004a5e042b2f4269801811460427c26cf93019b65e3c4e9817e1fdbb57e13cecdbf7dfe1dce54598ff3cfe64b97ab3ff64792a192105c054080b80a9101600b32024841c00b32054006641a8002c07d105c1ca8b603522844684d04522dce4cf049483b000980a610130152207806404948250019805a102300bc202402e042b2f829517a1b411213422849e11900571f3bf4f403984140053212c00162f9e83b000583c6f988c90036016840ac0e2c553102a008be7c57543b0aa593a55cdbc23049482b8550402a0112174808086105e84abff5b241a22a01c8405c0548878e114005321e279cc44402a840ac02c8878e11c00b320e2796c088064042b2f829517414d46405e886b404801a09308c80371e908a700501601a9105e84abf93f46a422a01c8405c0540815005523a05a08158059102d019084803c10b9bc083529004846402d20ce85a002201702abc1e88de0b93c2b4240a5103d114a00503102ea01716e005485c04a30521025003597674d10580946692d2ecf9a22b09e182d2fcfba200cab41e971e9616741b8f4fe0136d3bfacd01443e90000000049454e44ae426082"}}"#) && history_printed.contains(r#"{"event":"metadata_removed","payload":{"specname":"westend","spec_version":"9010","meta_hash":"70c99738c27fb32c87883f1c9c94ee454bf0b3d88e4a431a2bbfe1222b46ebdf","meta_id_pic":"89504e470d0a1a0a0000000d49484452000000410000004108060000008ef7c9450000033449444154789cedd93d6e135114c571b284c83414d429116e690129ca365c2121b6902d20242a6f238a04b4b44694a92968b0bc04f38eac1b7bc66fde39f77d8c9c64fec59d6246b2eeaf183d79ceb6dbedb3a7de28089bcd26fb47cecfcfcfc2a5694d104a9666b540a98ad072f97e3531aa20e42cbffe761366b7d9fbab307dd5c02842c8591ec500ac1c085482918dd002c01a1b220b211700b5444039102e8492e5add60896074346a80180c642402a8484e001587d5d84d96dfe6119e62e2fc2fad7f1f3b3d7fbfb2c0582229402582a0403b06a4254434801580c4205b05488220415007911583511500a6210c103804e1d010d414411bc00e82120a018848c70f3671d66b7ab97b330777911fead8f977c3edb2fe54558fc5c85d96df9661e6637094105b054080660a9100cc0522028420ac062102a80c5205400ab0f9144e803202f02cb8bc0ca414087101342e81e2106801e2b023288ea08abc5a730bbcd979fc3dce545b8bcb808b3dbeddd5d98bb46414029080660a9100cc052218600900b01c52054008b41a80016834801a00e020350f222b0bc08b901624298109c08ec24e845787779bce4f7dbfd525e84bf9177cc8b8377cc5032420cc052211880a54230008b41480829008b41a80016835001ac144413049617813521842684d04922b093a017e155e4f9df07f747414029080660a9100cc052215200484640310815c062102a80c52018007221b0bc082c2f426e1342684208dd232006c1be1d7a11d849d08b70fdf64b98ddae7f7c0c73380084cbeeff0494428801582a0403b054080660a52064841480c52054008b41a800d610443304961781551501c5201e2b8201a00921d441407d082f02fb76e8456027c11c8443004411500a8201582a0403b054883e00a208488550012c06a102580c4201403202cb8bc0f222a8c908c80bf11010620068100179204e1d6108002511900ae14560df0e6b22a40050350494826000960aa102a06204540aa102580ca2260092109007229517a1240500c908a806c458082a007221582518ad113ccb5b59082817a225420e00ca46402d20c606404508560e460c2207a06479ab0a829583915b8de5adaa08564b8c9acb5b4d10fa95a0b458badf2808a7de7f10bfbfacc65e85560000000049454e44ae426082"}}"#) && history_printed.contains(r#"{"event":"identity_removed","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"3efeca331d646d8a2986374bb3bb8d6e9e3cfcdd7c45c2b69104fab5d61d3f34","path":"//westend","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}"#) && history_printed.contains(r#"{"event":"identity_removed","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"46ebddef8cd9bb167dc30878d7113b7e168e6f0646beffd77d69d39bad76b47a","path":"","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}"#) && history_printed.contains(r#"{"event":"identity_removed","payload":{"seed_name":"Alice","encryption":"sr25519","public_key":"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d","path":"//Alice","network_genesis_hash":"e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e"}}"#), "Expected different history:\n{}", history_printed);
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
