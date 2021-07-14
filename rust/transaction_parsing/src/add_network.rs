use hex;
use sled::{Db, Tree, open};
use definitions::{constants::{ADDNETWORK, METATREE, SETTREE, SPECSTREE, TRANSACTION}, metadata::{NameVersioned, VersionDecoded}, network_specs::{ChainSpecsToSend, Verifier, generate_network_key}, transactions::{AddNetworkDb}};
use meta_reading::decode_metadata::{get_meta_const_light};
use parity_scale_codec::{Decode, Encode};
use blake2_rfc::blake2b::blake2b;
use frame_metadata::RuntimeMetadataV12;

use super::cards::{Action, Card, Warning};
use super::error::{Error, BadInputData, DatabaseError, CryptoError};
use super::check_signature::pass_crypto;
use super::load_metadata::process_received_metadata;
use super::utils::{get_chainspecs, get_general_verifier};

pub fn add_network (data_hex: &str, dbname: &str) -> Result<String, Error> {

// loading the database and its trees: chainspecs, metadata, settings, transaction;

    let database: Db = match open(dbname) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    
    let chainspecs: Tree = match database.open_tree(SPECSTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    let metadata: Tree = match database.open_tree(METATREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    let settings: Tree = match database.open_tree(SETTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    let transaction: Tree = match database.open_tree(TRANSACTION) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    
    let current_verifier = get_general_verifier(&settings)?;
    
    let checked_info = pass_crypto(&data_hex)?;
    
    let (new_meta_vec, new_chain_specs) = match <(Vec<u8>, ChainSpecsToSend)>::decode(&mut &checked_info.message[..]) {
        Ok(x) => x,
        Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeAddNetworkMessage)),
    };
    
    let verifier = checked_info.verifier;
    
    let new_network_key = generate_network_key(&new_chain_specs.genesis_hash.to_vec());
    
    match get_chainspecs (&new_network_key, &chainspecs) {
        Ok(x) => {
        
        // network is already in the system,
        // need to warn about that and proceed to check the received metadata for version
        // can offer to add the metadata and/or update the network verifier and/or update the general verifier
        
        // x.verifier - known verifier for this newtork
        // current_verifier - current general verifier for adding networks and importing types
        // verifier - verifier of this particular message
        
        // first check if the important specs have changed: base58prefix, decimals, name, and unit
            if (x.base58prefix != new_chain_specs.base58prefix)|(x.decimals != new_chain_specs.decimals)|(x.name != new_chain_specs.name)|(x.unit != new_chain_specs.unit) {return Err(Error::BadInputData(BadInputData::ImportantSpecsChanged))}
        
        // need to check that verifier of message is not "worse" than current general verifier and the verifier of the network
            match verifier {
                Verifier::None => {
                // to proceed, both the general verifier and the verifier for this particular network also should be Verifier::None
                    if current_verifier == Verifier::None {
                        if x.verifier == Verifier::None {
                        // action appears only if the metadata is actually uploaded
                        // "only verifier" warning is not possible
                            let warning_card_1 = Card::Warning(Warning::NotVerified).card(0,0);
                            let warning_card_2 = Card::Warning(Warning::NetworkAlreadyHasEntries).card(1,0);
                            let index = 2;
                            let upd_network = None;
                            let upd_general = false;
                            let (meta_card, action_card) = process_received_metadata(new_meta_vec, &new_chain_specs.name, index, upd_network, upd_general, verifier, metadata, transaction, database)?;
                            Ok(format!("{{\"warning\":[{},{}],\"meta\":[{}],{}}}", warning_card_1, warning_card_2, meta_card, action_card))
                        }
                        else {return Err(Error::CryptoError(CryptoError::NetworkExistsVerifierDisappeared))}
                    }
                    else {return Err(Error::CryptoError(CryptoError::GeneralVerifierDisappeared))}
                },
                _ => {
                let verifier_card = Card::Verifier(verifier.show_card()).card(0,0);
                // message has a verifier
                    if current_verifier == verifier {
                        if x.verifier == verifier {
                        // all verifiers are equal, can only update metadata if the version is newer
                        // action appears only if the metadata is actually uploaded
                        // "only verifier" warning is not possible
                            let warning_card_1 = Card::Warning(Warning::NetworkAlreadyHasEntries).card(1,0);
                            let index = 2;
                            let upd_network = None;
                            let upd_general = false;
                            let (meta_card, action_card) = process_received_metadata(new_meta_vec, &new_chain_specs.name, index, upd_network, upd_general, verifier, metadata, transaction, database)?;
                            Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"meta\":[{}],{}}}", verifier_card, warning_card_1, meta_card, action_card))
                        }
                        else {
                            if x.verifier == Verifier::None {
                            // update metadata if version is newer and update network verifier
                                let warning_card_1 = Card::Warning(Warning::NetworkAlreadyHasEntries).card(1,0);
                                let warning_card_2 = Card::Warning(Warning::VerifierAppeared).card(2,0);
                                let possible_warning = Card::Warning(Warning::MetaAlreadyThereUpdMetaVerifier).card(3, 0);
                                let index = 3;
                                let upd_network = Some(new_network_key);
                                let upd_general = false;
                                let (meta_card, action_card) = process_received_metadata(new_meta_vec, &new_chain_specs.name, index, upd_network, upd_general, verifier, metadata, transaction, database)?;
                                if meta_card == possible_warning {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{},{}],{}}}", verifier_card, warning_card_1, warning_card_2, meta_card, action_card))}
                                else {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"meta\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, meta_card, action_card))}
                            }
                            else {return Err(Error::CryptoError(CryptoError::NetworkExistsVerifierDisappeared))}
                        }
                    }
                    else {
                        if current_verifier == Verifier::None {
                        // need to update the general verifier if the message is ok
                            if x.verifier == verifier {
                                let warning_card_1 = Card::Warning(Warning::NetworkAlreadyHasEntries).card(1,0);
                                let warning_card_2 = Card::Warning(Warning::GeneralVerifierAppeared).card(2,0);
                                let possible_warning = Card::Warning(Warning::MetaAlreadyThereUpdGeneralVerifier).card(3, 0);
                                let index = 3;
                                let upd_network = None;
                                let upd_general = true;
                                let (meta_card, action_card) = process_received_metadata(new_meta_vec, &new_chain_specs.name, index, upd_network, upd_general, verifier, metadata, transaction, database)?;
                                if meta_card == possible_warning {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{},{}],{}}}", verifier_card, warning_card_1, warning_card_2, meta_card, action_card))}
                                else {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"meta\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, meta_card, action_card))}
                            }
                            else {
                                if x.verifier == Verifier::None {
                                // need to update both the general verifier and the network verifier
                                    let warning_card_1 = Card::Warning(Warning::NetworkAlreadyHasEntries).card(1,0);
                                    let warning_card_2 = Card::Warning(Warning::GeneralVerifierAppeared).card(2,0);
                                    let warning_card_3 = Card::Warning(Warning::VerifierAppeared).card(3,0);
                                    let possible_warning = Card::Warning(Warning::MetaAlreadyThereUpdBothVerifiers).card(4, 0);
                                    let index = 4;
                                    let upd_network = Some(new_network_key);
                                    let upd_general = true;
                                    let (meta_card, action_card) = process_received_metadata(new_meta_vec, &new_chain_specs.name, index, upd_network, upd_general, verifier, metadata, transaction, database)?;
                                    if meta_card == possible_warning {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{},{},{}],{}}}", verifier_card, warning_card_1, warning_card_2, warning_card_3, meta_card, action_card))}
                                    else {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{},{}],\"meta\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, warning_card_3, meta_card, action_card))}
                                    }
                                else {return Err(Error::CryptoError(CryptoError::VerifierChanged{old_show: x.verifier.show_error(), new_show: verifier.show_error()}))}
                            }
                        }
                        else {return Err(Error::CryptoError(CryptoError::GeneralVerifierChanged{old_show: current_verifier.show_error(), new_show: verifier.show_error()}))}
                    }
                },
            }
        },
        Err(Error::DatabaseError(DatabaseError::NoNetwork)) => {
        
        // network genesis hash is not on record, this is the most likely variant of add_network procedure

        // TODO it could be possible that the network did change genesis hash,
        // and there are networks with same name in metadata tree of the database;
        // also so far there was no way to ensure network name corresponds uniquely to genesis hash,
        // i.e. in chainspecs tree of the database each name is encountered only once;
        // this possibilities should be looked closer into later, maybe
        
            match verifier {
                Verifier::None => {
                    if current_verifier == Verifier::None {
                        let warning_card = Card::Warning(Warning::AddNetworkNotVerified).card(0,0);
                        let index = 1;
                        let upd = false;
                        let (new_network_card, action_card) = process_received_network_info (new_meta_vec, new_chain_specs, index, verifier, upd, transaction, database)?;
                        Ok(format!("{{\"warning\":[{}],\"new_network\":[{}],{}}}", warning_card, new_network_card, action_card))
                    }
                    else {return Err(Error::CryptoError(CryptoError::GeneralVerifierDisappeared))}
                },
                _ => {
                    let verifier_card = Card::Verifier(verifier.show_card()).card(0,0);
                    if current_verifier == verifier {
                        let index = 1;
                        let upd = false;
                        let (new_network_card, action_card) = process_received_network_info (new_meta_vec, new_chain_specs, index, verifier, upd, transaction, database)?;
                        Ok(format!("{{\"verifier\":[{}],\"new_network\":[{}],{}}}", verifier_card, new_network_card, action_card))
                    }
                    else {
                        if current_verifier == Verifier::None {
                            let warning_card = Card::Warning(Warning::GeneralVerifierAppeared).card(1,0);
                            let index = 2;
                            let upd = true;
                            let (new_network_card, action_card) = process_received_network_info (new_meta_vec, new_chain_specs, index, verifier, upd, transaction, database)?;
                            Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"new_network\":[{}],{}}}", verifier_card, warning_card, new_network_card, action_card))
                        }
                        else {return Err(Error::CryptoError(CryptoError::GeneralVerifierChanged{old_show: current_verifier.show_error(), new_show: verifier.show_error()}))}
                    }
                },
            }
        },
        Err(e) => {
        // damaged database, generally unexpected outcome
            return Err(e)
        },
    }
    
}


fn process_received_network_info (meta: Vec<u8>, new_chain_specs: ChainSpecsToSend, index: u32, verifier: Verifier, upd: bool, transaction: Tree, database: Db) -> Result<(String, String), Error> {
    if !meta.starts_with(&vec![109, 101, 116, 97]) {return Err(Error::BadInputData(BadInputData::NotMeta))}
    if meta[4] < 12 {return Err(Error::BadInputData(BadInputData::MetaVersionBelow12))}
    match RuntimeMetadataV12::decode(&mut &meta[5..]) {
        Ok(received_metadata) => {
            match get_meta_const_light(&received_metadata) {
                Ok(x) => {
                    match VersionDecoded::decode(&mut &x[..]) {
                        Ok(y) => {
                            if y.specname != new_chain_specs.name {return Err(Error::BadInputData(BadInputData::MetaMismatch))}
                            
                            let new_network_card = Card::NewNetwork{specname: &y.specname, spec_version: y.spec_version, meta_hash: &hex::encode(blake2b(32, &[], &meta).as_bytes()), chain_specs: &new_chain_specs, verifier_line: verifier.show_card()}.card(index, 0);
                            
                            let received_versioned_name = NameVersioned {
                                name: y.specname.to_string(),
                                version: y.spec_version,
                            };
                            let add_network = AddNetworkDb {
                                versioned_name: received_versioned_name.encode(),
                                meta,
                                chainspecs: new_chain_specs,
                                verifier,
                            };
                            match transaction.insert(ADDNETWORK, add_network.encode()) {
                                Ok(_) => (),
                                Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
                            };
                            match database.flush() {
                                Ok(_) => (),
                                Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
                            };
                            let checksum = match database.checksum() {
                                Ok(x) => x,
                                Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
                            };
                            let action_card = {
                                if upd {Action::AddNetworkAndAddGeneralVerifier(checksum).card()}
                                else {Action::AddNetwork(checksum).card()}
                            };
                            Ok((new_network_card, action_card))
                        },
                        Err(_) => return Err(Error::BadInputData(BadInputData::VersionNotDecodeable)),
                    }
                },
                Err(_) => return Err(Error::BadInputData(BadInputData::NoMetaVersion)),
            }
        },
        Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeMeta)),
    }
}
