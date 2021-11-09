use db_handling::db_transactions::{TrDbColdSign, SignContent};
use definitions::keyring::AddressKey;
use parity_scale_codec::Decode;

use crate::cards::{Action, Card, Warning};
use crate::error::{Error, BadInputData, DatabaseError};
use crate::helpers::{author_encryption_msg_genesis, checked_address_details, checked_network_specs, sign_store_and_get_checksum};

pub fn process_message (data_hex: &str, dbname: &str) -> Result<String, Error> {

    let (author_public_key, encryption, message_vec, network_specs_key) = author_encryption_msg_genesis(data_hex)?;
    
// this is a standard decoding of String, with utf8 conversion;
// processing input vec![20, 104, 101, 3, 108, 111] will not throw error at element `3`,
// it will result in output `helo` instead, length, however, is still correct, 5.
// note that some invisible symbols may thus sneak into the message;
    let message = match String::decode(&mut &message_vec[..]) {
        Ok(a) => a,
        Err(_) => return Err(Error::BadInputData(BadInputData::MessageNotReadable))
    };
    
// initialize index and indent
    let mut index: u32 = 0;
    let indent: u32 = 0;
    
    match checked_network_specs(&network_specs_key, &dbname)? {
        Some(network_specs) => {
            let address_key = AddressKey::from_parts(&author_public_key, &encryption).expect("already matched encryption type and author public key length, should always work");
            let author = address_key.print_as_base58(&encryption, Some(network_specs.base58prefix)).expect("just generated address_key, should always work");
            match checked_address_details(&address_key, &dbname)? {
                Some(address_details) => {
                    let author_card = Card::Author{base58_author: &author, seed_name: &address_details.seed_name, path: &address_details.path, has_pwd: address_details.has_pwd, name: &address_details.name}.card(&mut index, indent);
                    if address_details.network_id.contains(&network_specs_key) {
                        let message_card = Card::Message(&message).card(&mut index, indent);
                        let sign = TrDbColdSign::generate(SignContent::Message(message), &network_specs.name, &address_details.path, address_details.has_pwd, &address_key, Vec::new());
                        let checksum = sign_store_and_get_checksum (sign, &dbname)?;
                        let action_card = Action::Sign(checksum).card();
                        Ok(format!("{{\"author\":[{}],\"message\":[{}],{}}}", author_card, message_card, action_card))
                    }
                    else {
                        let warning_card = Card::Warning(Warning::NoNetworkID).card(&mut index, indent);
                        let message_card = Card::Message(&message).card(&mut index, indent);
                        Ok(format!("{{\"author\":[{}],\"warning\":[{}],\"message\":[{}]}}", author_card, warning_card, message_card))
                    }
                }
                None => {
                    let author_card = Card::AuthorPlain(&author).card(&mut index, indent);
                    let warning_card = Card::Warning(Warning::AuthorNotFound).card(&mut index, indent);
                    let message_card = Card::Message(&message).card(&mut index, indent);
                    Ok(format!("{{\"author\":[{}],\"warning\":[{}],\"message\":[{}]}}", author_card, warning_card, message_card))
                }
            }
        },
        None => {
            let author_card = Card::AuthorPublicKey{author_public_key, encryption}.card(&mut index, indent);
            let error_card = Card::Error(Error::DatabaseError(DatabaseError::NoNetwork)).card(&mut index, indent);
            let message_card = Card::Message(&message).card(&mut index, indent);
            Ok(format!("{{\"author\":[{}],\"error\":[{}],\"message\":[{}]}}", author_card, error_card, message_card))
        },
    }
}
