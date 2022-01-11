use db_handling::{db_transactions::TrDbColdStub, helpers::{try_get_types, get_general_verifier}};
use definitions::{error::{ErrorSigner, GeneralVerifierForContent, InputSigner, Signer, TransferContent}, network_specs::Verifier, history::Event, qr_transfers::ContentLoadTypes, types::TypeEntry};

use crate::{Action, StubNav};
use crate::cards::{Card, Warning};
use crate::check_signature::pass_crypto;
use crate::holds::{GeneralHold};


pub fn load_types(data_hex: &str, database_name: &str) -> Result<Action, ErrorSigner> {
    let checked_info = pass_crypto(&data_hex, TransferContent::LoadTypes)?;
    let content_new_types = ContentLoadTypes::from_vec(&checked_info.message);
    let new_types = content_new_types.types::<Signer>()?;
    let old_types: Vec<TypeEntry> = match try_get_types::<Signer>(&database_name)? {
        Some(a) => a,
        None => Vec::new(),
    };
    let general_verifier = get_general_verifier(&database_name)?;
    let mut stub = TrDbColdStub::new();
    let mut index = 0;
    match checked_info.verifier {
        Verifier(None) => {
            match general_verifier {
                Verifier(None) => {
                    if new_types == old_types {return Err(ErrorSigner::Input(InputSigner::TypesKnown))}
                    else {
                        stub = stub.new_history_entry(Event::Warning(Warning::TypesNotVerified.show()));
                        stub = stub.new_history_entry(Event::Warning(Warning::UpdatingTypes.show()));
                        stub = stub.add_types(&content_new_types, &checked_info.verifier);
                        let checksum = stub.store_and_get_checksum(&database_name)?;
                        let warning_card_1 = Card::Warning(Warning::TypesNotVerified).card(&mut index,0);
                        let warning_card_2 = Card::Warning(Warning::UpdatingTypes).card(&mut index,0);
                        let types_card = Card::TypesInfo(content_new_types).card(&mut index, 0);
                        Ok(Action::Stub(format!("\"warning\":[{},{}],\"types_info\":[{}]", warning_card_1, warning_card_2, types_card), checksum, StubNav::LoadTypes))
                    }
                },
                Verifier(Some(old_general_verifier_value)) => {
                    return Err(ErrorSigner::Input(InputSigner::NeedGeneralVerifier{content: GeneralVerifierForContent::Types, verifier_value: old_general_verifier_value.to_owned()}))
                },
            }
        },
        Verifier(Some(ref new_general_verifier_value)) => {
            let verifier_card = Card::Verifier(new_general_verifier_value).card(&mut index,0);
            if general_verifier == checked_info.verifier {
                if new_types == old_types {return Err(ErrorSigner::Input(InputSigner::TypesKnown))}
                else {
                    stub = stub.new_history_entry(Event::Warning(Warning::UpdatingTypes.show()));
                    stub = stub.add_types(&content_new_types, &checked_info.verifier);
                    let checksum = stub.store_and_get_checksum(&database_name)?;
                    let warning_card = Card::Warning(Warning::UpdatingTypes).card(&mut index,0);
                    let types_card = Card::TypesInfo(content_new_types).card(&mut index, 0);
                    Ok(Action::Stub(format!("\"verifier\":[{}],\"warning\":[{}],\"types_info\":[{}]", verifier_card, warning_card, types_card), checksum, StubNav::LoadTypes))
                }
            }
            else {
                match general_verifier {
                    Verifier(None) => {
                        let new_general_verifier = checked_info.verifier;
                        let general_hold = GeneralHold::get(&database_name)?;
                        stub = general_hold.upd_stub(stub, &new_general_verifier, &database_name)?;
                        stub = stub.add_types(&content_new_types, &new_general_verifier);
                        let warning_card_1 = Card::Warning(Warning::GeneralVerifierAppeared(&general_hold)).card(&mut index,0);
                        let warning_card_2 = {
                            if new_types == old_types {
                                stub = stub.new_history_entry(Event::Warning(Warning::TypesAlreadyThere.show()));
                                Card::Warning(Warning::TypesAlreadyThere).card(&mut index,0)
                            }
                            else {
                                stub = stub.new_history_entry(Event::Warning(Warning::UpdatingTypes.show()));
                                Card::Warning(Warning::UpdatingTypes).card(&mut index,0)
                            }
                        };
                        let types_card = Card::TypesInfo(content_new_types).card(&mut index, 0);
                        let checksum = stub.store_and_get_checksum(&database_name)?;
                        Ok(Action::Stub(format!("\"verifier\":[{}],\"warning\":[{},{}],\"types_info\":[{}]", verifier_card, warning_card_1, warning_card_2, types_card), checksum, StubNav::LoadTypes))
                    },
                    Verifier(Some(old_general_verifier_value)) => {
                        return Err(ErrorSigner::Input(InputSigner::GeneralVerifierChanged{content: GeneralVerifierForContent::Types, old_general_verifier_value: old_general_verifier_value.to_owned(), new_general_verifier_value: new_general_verifier_value.to_owned()}))
                    },
                }
            }
        },
    }
}
