use hex;
use sled::{Db, Tree, open};
use definitions::{network_specs::{Verifier, generate_network_key}, transactions::{LoadMetaDb, UpdSpecs}, metadata::{NameVersioned, VersionDecoded}, constants::{ADDGENERALVERIFIER, ADDMETAVERIFIER, LOADMETA, METATREE, SPECSTREE, TRANSACTION}};
use meta_reading::decode_metadata::get_meta_const_light;
use parity_scale_codec::{Decode, Encode};
use blake2_rfc::blake2b::blake2b;
use frame_metadata::RuntimeMetadataV12;

use super::cards::{Action, Card, Warning};
use super::error::{Error, BadInputData, DatabaseError, CryptoError};
use super::check_signature::pass_crypto;
use super::utils::get_chainspecs;


/// Function to check incoming metadata, and prepare info card and database entry
pub fn process_received_metadata (meta: Vec<u8>, name: &str, index: u32, upd_network: Option<Vec<u8>>, upd_general: bool, verifier: Verifier, metadata: Tree, transaction: Tree, database: Db) -> Result<(String, String), Error> {
    if !meta.starts_with(&vec![109, 101, 116, 97]) {return Err(Error::BadInputData(BadInputData::NotMeta))}
    if meta[4] < 12 {return Err(Error::BadInputData(BadInputData::MetaVersionBelow12))}
    match RuntimeMetadataV12::decode(&mut &meta[5..]) {
        Ok(received_metadata) => {
            match get_meta_const_light(&received_metadata) {
                Ok(x) => {
                    match VersionDecoded::decode(&mut &x[..]) {
                        Ok(y) => {
                            if y.specname != name {return Err(Error::BadInputData(BadInputData::MetaMismatch))}
                            let received_versioned_name = NameVersioned {
                                name: y.specname.to_string(),
                                version: y.spec_version,
                            };
                        // search through the database to check if the metadata is already there
                            match metadata.get(received_versioned_name.encode()) {
                                Ok(z) => {
                                    match z {
                                        Some(a) => {
                                        // same versioned name found
                                            if a[..] == meta[..] {
                                            // same versioned name found, and metadata equal
                                                match upd_network {
                                                    Some(network_key) => {
                                                        // preparing action entry
                                                            let upd = UpdSpecs {
                                                                network_key,
                                                                verifier,
                                                            };
                                                        // selecting correct action card type
                                                        if upd_general {
                                                        // need to update both verifiers
                                                            let meta_card = Card::Warning(Warning::MetaAlreadyThereUpdBothVerifiers).card(index, 0);
                                                        // making action entry into database
                                                            match transaction.insert(ADDMETAVERIFIER, upd.encode()) {
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
                                                        // action card
                                                            let action_card = Action::AddTwoVerifiers(checksum).card();
                                                            Ok((meta_card, action_card))
                                                        }
                                                        else {
                                                        // need to update only metadata verifier
                                                            let meta_card = Card::Warning(Warning::MetaAlreadyThereUpdMetaVerifier).card(index, 0);
                                                        // making action entry into database
                                                            match transaction.insert(ADDMETAVERIFIER, upd.encode()) {
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
                                                        // action card
                                                            let action_card = Action::AddMetadataVerifier(checksum).card();
                                                            Ok((meta_card, action_card))
                                                        }
                                                    },
                                                    None => {
                                                        let meta_card = Card::Warning(Warning::MetaAlreadyThereUpdGeneralVerifier).card(index, 0);
                                                        if upd_general {
                                                            match transaction.insert(ADDGENERALVERIFIER, verifier.encode()) {
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
                                                        // action card
                                                            let action_card = Action::AddGeneralVerifier(checksum).card();
                                                            Ok((meta_card, action_card))
                                                        }
                                                        else {return Err(Error::BadInputData(BadInputData::MetaAlreadyThere))}
                                                    },
                                                }
                                            }
                                            else {return Err(Error::BadInputData(BadInputData::MetaTotalMismatch))}
                                        },
                                        None => {
                                        // same versioned name NOT found
                                            let meta_card = Card::Meta{specname: name, spec_version: y.spec_version, meta_hash: &hex::encode(blake2b(32, &[], &meta).as_bytes())}.card(index, 0);
                                        // making action entry into database
                                            let action_into_db = LoadMetaDb {
                                                versioned_name: received_versioned_name.encode(),
                                                meta,
                                                upd_network,
                                                verifier,
                                            };
                                            let action_card = {
                                                if upd_general {
                                                // updating general verifier
                                                    match transaction.insert(LOADMETA, action_into_db.encode()) {
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
                                                // action card
                                                    Action::LoadMetadataAndAddGeneralVerifier(checksum).card()
                                                }
                                                else {
                                                // NOT updating general verifier
                                                    match transaction.insert(LOADMETA, action_into_db.encode()) {
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
                                                // action card
                                                    Action::LoadMetadata(checksum).card()
                                                }
                                            };
                                            Ok((meta_card, action_card))
                                        },
                                    }
                                },
                                Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
                            }
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


pub fn load_metadata (data_hex: &str, dbname: &str) -> Result<String, Error> {

// loading the database and removing the previous (if any) load_metadata saves
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
    let transaction: Tree = match database.open_tree(TRANSACTION) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    
    let checked_info = pass_crypto(&data_hex)?;
    
// minimal length is 32 - the length of genesis hash
    if checked_info.message.len() < 32 {return Err(Error::BadInputData(BadInputData::TooShort))}
    let meta = checked_info.message[..checked_info.message.len()-32].to_vec();
    let gen_hash = checked_info.message[checked_info.message.len()-32..].to_vec();
    
    let network_key = generate_network_key(&gen_hash);
    
    let chain_specs_found = get_chainspecs(&network_key, &chainspecs)?;
    
    let verifier = checked_info.verifier;
    
    match verifier {
        Verifier::None => {
            if chain_specs_found.verifier == Verifier::None {
            // action appears only if the metadata is actually uploaded
            // "only verifier" warning is not possible
                let index = 1;
                let upd_network = None;
                let upd_general = false;
                let (meta_card, action_card) = process_received_metadata(meta, &chain_specs_found.name, index, upd_network, upd_general, verifier, metadata, transaction, database)?;
                Ok(format!("{{\"warning\":[{}],\"meta\":[{}],{}}}", Card::Warning(Warning::NotVerified).card(0,0), meta_card, action_card))
            }
            else {return Err(Error::CryptoError(CryptoError::VerifierDisappeared))}
        },
        _ => {
            let verifier_card = Card::Verifier(verifier.show_card()).card(0,0);
            if chain_specs_found.verifier == verifier {
            // action appears only if the metadata is actually uploaded
            // "only verifier" warning is not possible
                let index = 1;
                let upd_network = None;
                let upd_general = false;
                let (meta_card, action_card) = process_received_metadata(meta, &chain_specs_found.name, index, upd_network, upd_general, verifier, metadata, transaction, database)?;
                Ok(format!("{{\"verifier\":[{}],\"meta\":[{}],{}}}", verifier_card, meta_card, action_card))
            }
            else {
                if chain_specs_found.verifier == Verifier::None {
                // action could be either uploading of metadata or only updating of the network verifier
                    let warning_card = Card::Warning(Warning::VerifierAppeared).card(1,0);
                    let possible_warning = Card::Warning(Warning::MetaAlreadyThereUpdMetaVerifier).card(2, 0);
                    let index = 2;
                    let upd_network = Some(network_key);
                    let upd_general = false;
                    let (meta_card, action_card) = process_received_metadata(meta, &chain_specs_found.name, index, upd_network, upd_general, verifier, metadata, transaction, database)?;
                    if meta_card == possible_warning {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],{}}}", verifier_card, warning_card, meta_card, action_card))}
                    else {Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"meta\":[{}],{}}}", verifier_card, warning_card, meta_card, action_card))}
                }
                else {return Err(Error::CryptoError(CryptoError::VerifierChanged{old_show: chain_specs_found.verifier.show_error(), new_show: verifier.show_error()}))}
            }
        },
        
    }
}
