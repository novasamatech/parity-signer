use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use std::convert::TryInto;
use sled::{Db, Tree, open};
use db_handling::{chainspecs::{ChainSpecs, Verifier}, settings::{LoadMetaDb, UpdSpecs}, metadata::NameVersioned};
use meta_reading::{get_meta_const_light, VersionDecoded};
use parity_scale_codec::{Decode, Encode};
use blake2_rfc::blake2b::blake2b;
use frame_metadata::RuntimeMetadataV12;

use super::cards::{Card, Warning};
use super::error::{Error, BadInputData, DatabaseError, CryptoError};
use super::utils_base58::vec_to_base;



/// Function to search for genesis_hash in chainspecs database tree
fn get_chainspecs (gen_hash: &Vec<u8>, chainspecs: Tree) -> Result<ChainSpecs, Error> {

    let chainspecs_db_reply = match chainspecs.get(gen_hash) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    match chainspecs_db_reply {
        Some(x) => {
        // some entry found for this genesis hash
            match <ChainSpecs>::decode(&mut &x[..]) {
                Ok(y) => Ok(y),
                Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedChainSpecs)),
            }
        },
        None => {
        // no entry exists
            return Err(Error::DatabaseError(DatabaseError::NoNetwork))
        },
    }
}

/// Function to check incoming metadata, and prepare info card and database entry
fn process_received_metadata (meta: Vec<u8>, name: &str, index: u32, upd_specs: Option<UpdSpecs>, metadata: Tree, settings: Tree, database: Db) -> Result<(String, String), Error> {
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
                                            if a[..] == meta[4..] {return Err(Error::BadInputData(BadInputData::MetaAlreadyThere))}
                                            else {return Err(Error::BadInputData(BadInputData::MetaTotalMismatch))}
                                        },
                                        None => {
                                            let meta_card = Card::Meta{specname: name, spec_version: y.spec_version, meta_hash: &hex::encode(blake2b(32, &[], &meta).as_bytes())}.card(index, 0);
                                        // making action entry into database
                                            let action_into_db = LoadMetaDb {
                                                name: y.specname.to_string(),
                                                version: y.spec_version,
                                                meta: meta[4..].to_vec(),
                                                upd_specs,
                                            };
                                            match settings.insert(b"load_metadata", action_into_db.encode()) {
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
                                            let action_card = format!("\"action\":{{\"type\":\"load_metadata\",\"payload\":{{\"checksum\":\"{}\"}}}}", checksum);
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


/// Function to create output card based on verifier info
fn check_verifier (old: &Verifier, new: Verifier, meta: Vec<u8>, gen_hash: Vec<u8>, name: &str, metadata: Tree, settings: Tree, database: Db) -> Result <String, Error> {
    if old == &new {
        let verifier_card = Card::Verifier(new.show_card()).card(0,0);
        let index = 1;
        let upd_specs = None;
        let (meta_card, action_card) = process_received_metadata(meta, name, index, upd_specs, metadata, settings, database)?;
        Ok(format!("{{\"verifier\":[{}],\"meta\":[{}],{}}}", verifier_card, meta_card, action_card))
    }
    else {
        if old == &Verifier::None {
            let verifier_card = Card::Verifier(new.show_card()).card(0,0);
            let warning_card = Card::Warning(Warning::VerifierAppeared).card(1,0);
            let index = 2;
            let upd_specs = Some(UpdSpecs{gen_hash, verifier: new});
            let (meta_card, action_card) = process_received_metadata(meta, name, index, upd_specs, metadata, settings, database)?;
            Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"meta\":[{}],{}}}", verifier_card, warning_card, meta_card, action_card))
        }
        else {return Err(Error::CryptoError(CryptoError::VerifierChanged{old_show: old.show_error(), new_show: new.show_error()}))}
    }
}

pub fn load_metadata (data_hex: &str, dbname: &str) -> Result<String, Error> {

// loading the database and removing the previous (if any) load_metadata saves
    let database: Db = match open(dbname) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    let chainspecs: Tree = match database.open_tree(b"chainspecs") {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    let metadata: Tree = match database.open_tree(b"metadata") {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    let settings: Tree = match database.open_tree(b"settings") {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    
    match settings.remove(b"load_metadata") {
        Ok(_) => (),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
    
    let data = match hex::decode(&data_hex) {
        Ok(a) => a,
        Err(_) => return Err(Error::BadInputData(BadInputData::NotHex)),
    };
    
    match &data_hex[2..4] {
        "00" => {
        // Ed25519 crypto was used by the verifier of the metadata
        // minimal possible data length is 3 + 32 + 32 + 64 (prelude, public key in ed25519, network genesis hash, signature in ed25519)
            if data.len() < 131 {return Err(Error::BadInputData(BadInputData::TooShort))}
            let into_pubkey: [u8;32] = data[3..35].try_into().expect("fixed size should fit in array");
            let pubkey = ed25519::Public::from_raw(into_pubkey);
            let message = data[35..data.len()-64].to_vec();
            let into_signature: [u8;64] = data[data.len()-64..].try_into().expect("fixed size should fit in array");
            let signature = ed25519::Signature::from_raw(into_signature);
            if ed25519::Pair::verify(&signature, &message, &pubkey) {
                let meta = message[..message.len()-32].to_vec();
                let gen_hash = message[message.len()-32..].to_vec();
                let chain_specs_found = get_chainspecs(&gen_hash, chainspecs)?;
                let author = vec_to_base(&(into_pubkey.to_vec()), chain_specs_found.base58prefix);
                let ver_author = Verifier::Ed25519(author);
                check_verifier (&chain_specs_found.verifier, ver_author, meta, gen_hash, &chain_specs_found.name, metadata, settings, database)
            }
            else {return Err(Error::CryptoError(CryptoError::BadSignature))}
        },
        "01" => {
        // Sr25519 crypto was used by the verifier of the metadata
        // minimal possible data length is 3 + 32 + 32 + 64 (prelude, public key in sr25519, network genesis hash, signature in sr25519)
            if data.len() < 131 {return Err(Error::BadInputData(BadInputData::TooShort))}
            let into_pubkey: [u8;32] = data[3..35].try_into().expect("fixed size should fit in array");
            let pubkey = sr25519::Public::from_raw(into_pubkey);
            let message = data[35..data.len()-64].to_vec();
            let into_signature: [u8;64] = data[data.len()-64..].try_into().expect("fixed size should fit in array");
            let signature = sr25519::Signature::from_raw(into_signature);
            if sr25519::Pair::verify(&signature, &message, &pubkey) {
                let meta = message[..message.len()-32].to_vec();
                let gen_hash = message[message.len()-32..].to_vec();
                let chain_specs_found = get_chainspecs(&gen_hash, chainspecs)?;
                let author = vec_to_base(&(into_pubkey.to_vec()), chain_specs_found.base58prefix);
                let ver_author = Verifier::Sr25519(author);
                check_verifier (&chain_specs_found.verifier, ver_author, meta, gen_hash, &chain_specs_found.name, metadata, settings, database)
            }
            else {return Err(Error::CryptoError(CryptoError::BadSignature))}
        },
        "02" => {
        // Ecdsa crypto was used by the verifier of the metadata
        // minimal possible data length is 3 + 33 + 32 + 65 (prelude, public key in ecdsa, network genesis hash, signature in ecdsa)
            if data.len() < 133 {return Err(Error::BadInputData(BadInputData::TooShort))}
            let into_pubkey: [u8;33] = data[3..36].try_into().expect("fixed size should fit in array");
            let pubkey = ecdsa::Public::from_raw(into_pubkey);
            let message = data[36..data.len()-65].to_vec();
            let into_signature: [u8;65] = data[data.len()-65..].try_into().expect("fixed size should fit in array");
            let signature = ecdsa::Signature::from_raw(into_signature);
            if ecdsa::Pair::verify(&signature, &message, &pubkey) {
                let meta = message[..message.len()-32].to_vec();
                let gen_hash = message[message.len()-32..].to_vec();
                let chain_specs_found = get_chainspecs(&gen_hash, chainspecs)?;
                let author = vec_to_base(&(into_pubkey.to_vec()), chain_specs_found.base58prefix);
                let ver_author = Verifier::Ecdsa(author);
                check_verifier (&chain_specs_found.verifier, ver_author, meta, gen_hash, &chain_specs_found.name, metadata, settings, database)
            }
            else {return Err(Error::CryptoError(CryptoError::BadSignature))}
        },
        "ff" => {
        // Received metadata was not signed
        // minimal possible data length is 3 + 32 (prelude, network genesis hash)
            if data.len() < 35 {return Err(Error::BadInputData(BadInputData::TooShort))}
            let meta = data[3..data.len()-32].to_vec();
            let gen_hash = data[data.len()-32..].to_vec();
            let chain_specs_found = get_chainspecs(&gen_hash, chainspecs)?;
            if chain_specs_found.verifier == Verifier::None {
                let index = 1;
                let upd_specs = None;
                let (meta_card, action_card) = process_received_metadata(meta, &chain_specs_found.name, index, upd_specs, metadata, settings, database)?;
                Ok(format!("{{\"warning\":[{}],\"meta\":[{}],{}}}", Card::Warning(Warning::NotVerified).card(0,0), meta_card, action_card))
            }
            else {return Err(Error::CryptoError(CryptoError::VerifierDisappeared))}
        },
        _ => return Err(Error::BadInputData(BadInputData::CryptoNotSupported))
    }    

}
