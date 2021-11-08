use hex;
use sled::{Db, Tree};
use constants::{ADDGENERALVERIFIER, ADDMETAVERIFIER, LOADMETA, METATREE, TRANSACTION, VERIFIERS};
use definitions::{network_specs::{Verifier, generate_verifier_key, VerifierKey}, transactions::{Transaction, LoadMeta, UpdMetaVerifier, UpdGeneralVerifier}, metadata::{MetaValuesDisplay, NameVersioned, VersionDecoded}, history::Event, qr_transfers::ContentLoadMeta};
use meta_reading::decode_metadata::get_meta_const_light;
use parity_scale_codec::{Decode, Encode};
use blake2_rfc::blake2b::blake2b;
use frame_metadata::RuntimeMetadata;

use crate::cards::{Action, Card, Warning};
use crate::check_signature::pass_crypto;
use crate::error::{Error, BadInputData, CryptoError};
use crate::helpers::{open_db, open_tree, flush_db, insert_into_tree, get_checksum, get_from_tree, get_verifier};


pub fn load_metadata (data_hex: &str, dbname: &str) -> Result<String, Error> {

// loading the database and removing the previous (if any) load_metadata saves
    let database = open_db(dbname)?;
    let metadata = open_tree(&database, METATREE)?;
    let transaction = open_tree(&database, TRANSACTION)?;
    let verifiers = open_tree(&database, VERIFIERS)?;
    
    let checked_info = pass_crypto(&data_hex)?;
    
    let (meta, gen_hash) = match ContentLoadMeta::from_vec(&checked_info.message).meta_genhash() {
        Ok(x) => x,
        Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeLoadMetadataMessage)),
    };
    
    let verifier = checked_info.verifier;
    let current_verifier = get_verifier (gen_hash, &verifiers)?;
    
    match verifier {
        Verifier::None => {
            if current_verifier == Verifier::None {
            // action appears only if the metadata is actually uploaded
            // "only verifier" warning is not possible
                let index = 1;
                let upd_network = None;
                let upd_general = false;
                let history: Vec<Event> = vec![Event::Warning(Warning::NotVerified.show())];
                let (meta_card, action_card) = process_received_metadata(meta, None, history, index, upd_network, upd_general, verifier, &metadata, &transaction, &database)?;
                Ok(format!("{{\"warning\":[{}],\"meta\":[{}],{}}}", Card::Warning(Warning::NotVerified).card(0,0), meta_card, action_card))
            }
            else {return Err(Error::CryptoError(CryptoError::VerifierDisappeared))}
        },
        _ => {
            let verifier_card = Card::Verifier(verifier.show_card()).card(0,0);
            if current_verifier == verifier {
            // action appears only if the metadata is actually uploaded
            // "only verifier" warning is not possible
                let index = 1;
                let upd_network = None;
                let upd_general = false;
                let history: Vec<Event> = Vec::new();
                let (meta_card, action_card) = process_received_metadata(meta, None, history, index, upd_network, upd_general, verifier, &metadata, &transaction, &database)?;
                Ok(format!("{{\"verifier\":[{}],\"meta\":[{}],{}}}", verifier_card, meta_card, action_card))
            }
            else {
                if current_verifier == Verifier::None {
                // action could be either uploading of metadata or only updating of the network verifier
                    let warning_card = Card::Warning(Warning::VerifierAppeared).card(1,0);
                    let possible_warning = Card::Warning(Warning::MetaAlreadyThereUpdMetaVerifier).card(2, 0);
                    let index = 2;
                    let upd_network = Some(generate_verifier_key(&gen_hash.to_vec()));
                    let upd_general = false;
                    let history: Vec<Event> = vec![Event::Warning(Warning::VerifierAppeared.show())];
                    let (meta_card, action_card) = process_received_metadata(meta, None, history, index, upd_network, upd_general, verifier, &metadata, &transaction, &database)?;
                    if meta_card == possible_warning {Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],{}}}", verifier_card, warning_card, meta_card, action_card))}
                    else {Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"meta\":[{}],{}}}", verifier_card, warning_card, meta_card, action_card))}
                }
                else {return Err(Error::CryptoError(CryptoError::VerifierChanged{old_show: current_verifier.show_error(), new_show: verifier.show_error()}))}
            }
        },
        
    }
}


/// Function to check incoming metadata, and prepare info card and database entry
pub fn process_received_metadata (meta: Vec<u8>, name_to_check: Option<&str>, history: Vec<Event>, index: u32, upd_network: Option<VerifierKey>, upd_general: bool, verifier: Verifier, metadata: &Tree, transaction: &Tree, database: &Db) -> Result<(String, String), Error> {
    if !meta.starts_with(&vec![109, 101, 116, 97]) {return Err(Error::BadInputData(BadInputData::NotMeta))}
    if meta[4] < 12 {return Err(Error::BadInputData(BadInputData::MetaVersionBelow12))}
    match RuntimeMetadata::decode(&mut &meta[4..]) {
        Ok(received_metadata) => {
            match get_meta_const_light(&received_metadata) {
                Ok(x) => {
                    match VersionDecoded::decode(&mut &x[..]) {
                        Ok(y) => {
                            if let Some(name) = name_to_check {
                                if y.specname != name {return Err(Error::BadInputData(BadInputData::MetaMismatch))}
                            }
                            let received_versioned_name = NameVersioned {
                                name: y.specname.to_string(),
                                version: y.spec_version,
                            };
                        // search through the database to check if the metadata is already there
                            match get_from_tree(&received_versioned_name.encode(), metadata)? {
                                Some(a) => {
                                // same versioned name found
                                    if a[..] == meta[..] {
                                    // same versioned name found, and metadata equal
                                        match upd_network {
                                            Some(verifier_key) => {
                                            // preparing action entry
                                                let mut upd_meta_verifier = UpdMetaVerifier {
                                                    verifier_key,
                                                    verifier,
                                                    history,
                                                };
                                            // selecting correct action card type
                                                if upd_general {
                                                // need to update both verifiers
                                                    let meta_card = Card::Warning(Warning::MetaAlreadyThereUpdBothVerifiers).card(index, 0);
                                                    upd_meta_verifier.history.push(Event::Warning(Warning::MetaAlreadyThereUpdBothVerifiers.show()));
                                                // making action entry into database
                                                    let add_meta_verifier = Transaction::UpdMetaVerifier(upd_meta_verifier);
                                                    insert_into_tree(ADDMETAVERIFIER.to_vec(), add_meta_verifier.encode(), transaction)?;
                                                    flush_db(database)?;
                                                    let checksum = get_checksum(database)?;
                                                // action card
                                                    let action_card = Action::AddTwoVerifiers(checksum).card();
                                                    Ok((meta_card, action_card))
                                                }
                                                else {
                                                // need to update only metadata verifier
                                                    let meta_card = Card::Warning(Warning::MetaAlreadyThereUpdMetaVerifier).card(index, 0);
                                                    upd_meta_verifier.history.push(Event::Warning(Warning::MetaAlreadyThereUpdMetaVerifier.show()));
                                                // making action entry into database
                                                    let add_meta_verifier = Transaction::UpdMetaVerifier(upd_meta_verifier);
                                                    insert_into_tree(ADDMETAVERIFIER.to_vec(), add_meta_verifier.encode(), transaction)?;
                                                    flush_db(database)?;
                                                    let checksum = get_checksum(database)?;
                                                // action card
                                                    let action_card = Action::AddMetadataVerifier(checksum).card();
                                                    Ok((meta_card, action_card))
                                                }
                                            },
                                            None => {
                                            // preparing action entry
                                                let mut upd_general_verifier = UpdGeneralVerifier {
                                                    verifier,
                                                    history,
                                                };
                                                let meta_card = Card::Warning(Warning::MetaAlreadyThereUpdGeneralVerifier).card(index, 0);
                                                upd_general_verifier.history.push(Event::Warning(Warning::MetaAlreadyThereUpdGeneralVerifier.show()));
                                                if upd_general {
                                                    let add_gen_verifier = Transaction::UpdGeneralVerifier(upd_general_verifier);
                                                    insert_into_tree(ADDGENERALVERIFIER.to_vec(), add_gen_verifier.encode(), transaction)?;
                                                    flush_db(database)?;
                                                    let checksum = get_checksum(database)?;
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
                                    let new_meta = MetaValuesDisplay {
                                        name: &y.specname,
                                        version: y.spec_version,
                                        meta_hash: &hex::encode(blake2b(32, &[], &meta).as_bytes()),
                                    }.show();
                                    let meta_card = Card::Meta(new_meta).card(index, 0);
                                // making action entry into database
                                    let load_meta = Transaction::LoadMeta(LoadMeta{
                                        versioned_name: received_versioned_name,
                                        meta,
                                        upd_network,
                                        verifier,
                                        history,
                                    });
                                    insert_into_tree(LOADMETA.to_vec(), load_meta.encode(), transaction)?;
                                    flush_db(database)?;
                                    let checksum = get_checksum(database)?;
                                    let action_card = {
                                        if upd_general {Action::LoadMetadataAndAddGeneralVerifier(checksum).card()}
                                        else {Action::LoadMetadata(checksum).card()}
                                    };
                                    Ok((meta_card, action_card))
                                },
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

