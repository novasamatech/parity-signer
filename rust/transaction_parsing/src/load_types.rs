use hex;
use sp_core::{Pair, ed25519, sr25519, ecdsa};
use std::convert::TryInto;
use sled::{Db, Tree, open};
use db_handling::{chainspecs::Verifier, settings::{LoadTypesDb, TypeEntry}, constants::{ADDTYPESVERIFIER, LOADTYPES, TYPESVERIFIER, SETTREE}};
use parity_scale_codec::{Decode, Encode};
use blake2_rfc::blake2b::blake2b;

use super::cards::{Card, Warning};
use super::error::{Error, BadInputData, DatabaseError, CryptoError};
use super::parse_transaction::{get_types};

pub fn load_types (data_hex: &str, dbname: &str) -> Result<String, Error> {

// loading the database and removing the previous (if any) load_types saves
    let database: Db = match open(dbname) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    
    let settings: Tree = match database.open_tree(SETTREE) {
        Ok(x) => x,
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    
    match settings.remove(LOADTYPES) {
        Ok(_) => (),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    }
    
    match database.flush() {
        Ok(_) => (),
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    
    let data = match hex::decode(&data_hex) {
        Ok(a) => a,
        Err(_) => return Err(Error::BadInputData(BadInputData::NotHex)),
    };
    
    let current_types = get_types(&settings)?;
    
    let current_verifier = match settings.get(TYPESVERIFIER) {
        Ok(reply) => {
            match reply {
                Some(a) => {
                    match <Verifier>::decode(&mut &a[..]) {
                        Ok(x) => x,
                        Err(_) => return Err(Error::DatabaseError(DatabaseError::DamagedTypesVerifier)),
                    }
                },
                None => return Err(Error::DatabaseError(DatabaseError::NoTypesVerifier)),
            }
        },
        Err(e) => return Err(Error::DatabaseError(DatabaseError::Internal(e))),
    };
    
    match &data_hex[2..4] {
        "00" => {
        // Ed25519 crypto was used by the verifier of the types data
        // minimal possible data length is 3 + 32 + 64 (prelude, public key in ed25519, signature in ed25519)
            if data.len() < 99 {return Err(Error::BadInputData(BadInputData::TooShort))}
            let into_pubkey: [u8;32] = data[3..35].try_into().expect("fixed size should fit in array");
            let pubkey = ed25519::Public::from_raw(into_pubkey);
            let message = data[35..data.len()-64].to_vec();
            let into_signature: [u8;64] = data[data.len()-64..].try_into().expect("fixed size should fit in array");
            let signature = ed25519::Signature::from_raw(into_signature);
            if ed25519::Pair::verify(&signature, &message, &pubkey) {
                let new_types = match <Vec<TypeEntry>>::decode(&mut &message[..]) {
                    Ok(x) => x,
                    Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeTypes)),
                };
                let new_verifier = Verifier::Ed25519(hex::encode(&into_pubkey));
                compare_verifiers(&current_verifier, new_verifier, &current_types, &new_types, settings, database)
            }
            else {return Err(Error::CryptoError(CryptoError::BadSignature))}
        },
        "01" => {
        // Sr25519 crypto was used by the verifier of the metadata
        // minimal possible data length is 3 + 32 + 64 (prelude, public key in sr25519, network genesis hash, signature in sr25519)
            if data.len() < 99 {return Err(Error::BadInputData(BadInputData::TooShort))}
            let into_pubkey: [u8;32] = data[3..35].try_into().expect("fixed size should fit in array");
            let pubkey = sr25519::Public::from_raw(into_pubkey);
            let message = data[35..data.len()-64].to_vec();
            let into_signature: [u8;64] = data[data.len()-64..].try_into().expect("fixed size should fit in array");
            let signature = sr25519::Signature::from_raw(into_signature);
            if sr25519::Pair::verify(&signature, &message, &pubkey) {
                let new_types = match <Vec<TypeEntry>>::decode(&mut &message[..]) {
                    Ok(x) => x,
                    Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeTypes)),
                };
                let new_verifier = Verifier::Sr25519(hex::encode(&into_pubkey));
                compare_verifiers(&current_verifier, new_verifier, &current_types, &new_types, settings, database)
            }
            else {return Err(Error::CryptoError(CryptoError::BadSignature))}
        },
        "02" => {
        // Ecdsa crypto was used by the verifier of the metadata
        // minimal possible data length is 3 + 33 + 65 (prelude, public key in ecdsa, network genesis hash, signature in ecdsa)
            if data.len() < 101 {return Err(Error::BadInputData(BadInputData::TooShort))}
            let into_pubkey: [u8;33] = data[3..36].try_into().expect("fixed size should fit in array");
            let pubkey = ecdsa::Public::from_raw(into_pubkey);
            let message = data[36..data.len()-65].to_vec();
            let into_signature: [u8;65] = data[data.len()-65..].try_into().expect("fixed size should fit in array");
            let signature = ecdsa::Signature::from_raw(into_signature);
            if ecdsa::Pair::verify(&signature, &message, &pubkey) {
                let new_types = match <Vec<TypeEntry>>::decode(&mut &message[..]) {
                    Ok(x) => x,
                    Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeTypes)),
                };
                let new_verifier = Verifier::Ecdsa(hex::encode(&into_pubkey));
                compare_verifiers(&current_verifier, new_verifier, &current_types, &new_types, settings, database)
            }
            else {return Err(Error::CryptoError(CryptoError::BadSignature))}
        },
        "ff" => {
        // Received metadata was not signed
        // minimal possible data length is 3 (prelude) - already checked on entry
            let new_types = match <Vec<TypeEntry>>::decode(&mut &data[3..]) {
                Ok(x) => x,
                Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeTypes)),
            };
            if current_verifier == Verifier::None {
                let warning_card_1 = Card::Warning(Warning::UpdatingTypes).card(0,0);
                let warning_card_2 = Card::Warning(Warning::TypesNotVerified).card(1,0);
                let index = 2;
                let upd_verifier = None;
                let (types_card, action_card) = process_received_types(&current_types, &new_types, upd_verifier, index, settings, database)?;
                Ok(format!("{{\"warning\":[{},{}],\"types_info\":[{}],{}}}", warning_card_1, warning_card_2, types_card, action_card))
            }
            else {return Err(Error::CryptoError(CryptoError::TypesVerifierDisappeared))}
        },
        _ => return Err(Error::BadInputData(BadInputData::CryptoNotSupported))
    }
}


fn compare_verifiers (current_verifier: &Verifier, new_verifier: Verifier, current_types: &Vec<TypeEntry>, new_types: &Vec<TypeEntry>, settings: Tree, database: Db) -> Result<String, Error> {
    
    if current_verifier == &new_verifier {
        let verifier_card = Card::Verifier(new_verifier.show_card()).card(0,0);
        let warning_card = Card::Warning(Warning::UpdatingTypes).card(1,0);
        let index = 2;
        let upd_verifier = None;
        let (types_card, action_card) = process_received_types(current_types, new_types, upd_verifier, index, settings, database)?;
        Ok(format!("{{\"verifier\":[{}],\"warning\":[{}],\"types_info\":[{}],{}}}", verifier_card, warning_card, types_card, action_card))
    }
    else {
        if current_verifier == &Verifier::None {
            let verifier_card = Card::Verifier(new_verifier.show_card()).card(0,0);
            let warning_card_1 = Card::Warning(Warning::UpdatingTypes).card(1,0);
            let warning_card_2 = Card::Warning(Warning::TypesVerifierAppeared).card(2,0);
            let index = 3;
            let upd_verifier = Some(new_verifier);
            let (types_card, action_card) = process_received_types(current_types, new_types, upd_verifier, index, settings, database)?;
            Ok(format!("{{\"verifier\":[{}],\"warning\":[{},{}],\"types_info\":[{}],{}}}", verifier_card, warning_card_1, warning_card_2, types_card, action_card))
        }
        else {return Err(Error::CryptoError(CryptoError::TypesVerifierChanged{old_show: current_verifier.show_error(), new_show: new_verifier.show_error()}))}
    }
}


fn process_received_types (current_types: &Vec<TypeEntry>, new_types: &Vec<TypeEntry>, upd_verifier: Option<Verifier>, index: u32, settings: Tree, database: Db) -> Result<(String, String), Error> {
    if new_types == current_types {
        match upd_verifier {
            Some(new_verifier) => {
            // adding only types verifier
                let types_card = Card::Warning(Warning::TypesAlreadyThere).card(index, 0);
            // making action entry into database
                match settings.insert(ADDTYPESVERIFIER, new_verifier.encode()) {
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
                let action_card = format!("\"action\":{{\"type\":\"add_types_verifier\",\"payload\":{{\"type\":\"add_types_verifier\",\"checksum\":\"{}\"}}}}", checksum);
                Ok((types_card, action_card))
            },
            None => return Err(Error::BadInputData(BadInputData::TypesAlreadyThere)),
        }
    }
    else {
    // loading types
        let types_card = Card::TypesInfo(&hex::encode(blake2b(32, &[], &new_types.encode()).as_bytes())).card(index, 0);
    // making action entry into database
        let action_into_db = LoadTypesDb {types_info_encoded: new_types.encode(), upd_verifier};
        
        match settings.insert(LOADTYPES, action_into_db.encode()) {
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
        let action_card = format!("\"action\":{{\"type\":\"load_types\",\"payload\":{{\"type\":\"load_types\",\"checksum\":\"{}\"}}}}", checksum);
        Ok((types_card, action_card))
    }
}

