use db_handling::db_transactions::TrDbColdStub;
use definitions::{keyring::VerifierKey, network_specs::{CurrentVerifier, Verifier}, history::{Event, MetaValuesDisplay}, qr_transfers::ContentLoadMeta};

use crate::cards::{Action, Card, Warning};
use crate::check_signature::pass_crypto;
use crate::error::{Error, BadInputData, CryptoError, DatabaseError};
use crate::helpers::{get_general_verifier, get_current_verifier, decode_input_metadata, accept_meta_values, stub_store_and_get_checksum};

enum FirstCard {
    WarningCard(String),
    VerifierCard(String),
}

pub fn load_metadata(data_hex: &str, database_name: &str) -> Result<String, Error> {
    let checked_info = pass_crypto(&data_hex)?;
    let (meta, genesis_hash) = match ContentLoadMeta::from_vec(&checked_info.message).meta_genhash() {
        Ok(x) => x,
        Err(_) => return Err(Error::BadInputData(BadInputData::UnableToDecodeLoadMetadataMessage)),
    };
    let meta_values = decode_input_metadata(meta)?;
    let current_verifier = match get_current_verifier (&VerifierKey::from_parts(&genesis_hash.to_vec()), &database_name)? {
        Some(a) => a,
        None => return Err(Error::BadInputData(BadInputData::LoadMetaUnknownNetwork(meta_values.name))),
    };
    let general_verifier = get_general_verifier(&database_name)?;
    let mut stub = TrDbColdStub::new();
    let mut index = 0;
    
    let first_card = {
        if checked_info.verifier == Verifier(None) {
            stub = stub.new_history_entry(Event::Warning(Warning::NotVerified.show()));
            match current_verifier {
                CurrentVerifier::Custom(Verifier(None)) => (),
                CurrentVerifier::Custom(_) => return Err(Error::CryptoError(CryptoError::VerifierDisappeared)),
                CurrentVerifier::General => if general_verifier != Verifier(None) {return Err(Error::CryptoError(CryptoError::GeneralVerifierDisappeared))},
                CurrentVerifier::Dead => return Err(Error::DatabaseError(DatabaseError::DeadVerifier(meta_values.name))),
            }
            FirstCard::WarningCard(Card::Warning(Warning::NotVerified).card(&mut index,0))
        }
        else {
            match current_verifier {
                CurrentVerifier::Custom(a) => {
                    if checked_info.verifier != a {
                        if a == Verifier(None) {return Err(Error::CryptoError(CryptoError::LoadMetaUpdVerifier{network_name: meta_values.name, new_verifier: checked_info.verifier}))}
                        else {return Err(Error::CryptoError(CryptoError::LoadMetaVerifierChanged{network_name: meta_values.name, old: a, new: checked_info.verifier}))}
                    }
                },
                CurrentVerifier::General => {
                    if checked_info.verifier != general_verifier {
                        if general_verifier == Verifier(None) {return Err(Error::CryptoError(CryptoError::LoadMetaUpdGeneralVerifier{network_name: meta_values.name, new_verifier: checked_info.verifier}))}
                        else {return Err(Error::CryptoError(CryptoError::LoadMetaGeneralVerifierChanged{network_name: meta_values.name, old: general_verifier, new: checked_info.verifier}))}
                    }
                },
                CurrentVerifier::Dead => return Err(Error::DatabaseError(DatabaseError::DeadVerifier(meta_values.name))),
            }
            FirstCard::VerifierCard(Card::Verifier(&checked_info.verifier).card(&mut index,0))
        }
    };
    if accept_meta_values(&meta_values, &database_name)? {
        stub = stub.add_metadata(&meta_values);
        let checksum = stub_store_and_get_checksum(stub, &database_name)?;
        let meta_display = MetaValuesDisplay::get(&meta_values);
        let meta_card = Card::Meta(meta_display).card(&mut index, 0);
        let action_card = Action::Stub(checksum).card();
        match first_card {
            FirstCard::WarningCard(warning_card) => Ok(format!("{{\"warning\":[{}],\"meta\":[{}],{}}}", warning_card, meta_card, action_card)),
            FirstCard::VerifierCard(verifier_card) => Ok(format!("{{\"verifier\":[{}],\"meta\":[{}],{}}}", verifier_card, meta_card, action_card)),
        }
    }
    else {return Err(Error::BadInputData(BadInputData::MetaAlreadyThere))}
}
