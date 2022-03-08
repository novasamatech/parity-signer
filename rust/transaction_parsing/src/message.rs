use db_handling::{db_transactions::{TrDbColdSign, SignContent}, helpers::{try_get_network_specs, try_get_address_details}};
use definitions::{error_signer::{ErrorSigner, InputSigner}, keyring::{AddressKey, NetworkSpecsKey}};
use parity_scale_codec::Decode;
use parser::cards::ParserCard;

use crate::Action;
use crate::cards::{Card, make_author_info, Warning};
use crate::helpers::multisigner_msg_genesis_encryption;

pub fn process_message (data_hex: &str, database_name: &str) -> Result<Action, ErrorSigner> {

    let (author_multi_signer, message_vec, genesis_hash_vec, encryption) = multisigner_msg_genesis_encryption(data_hex)?;
    let network_specs_key = NetworkSpecsKey::from_parts(&genesis_hash_vec, &encryption);
    
// this is a standard decoding of String, with utf8 conversion;
// processing input vec![20, 104, 101, 3, 108, 111] will not throw error at element `3`,
// it will result in output `helo` instead, length, however, is still correct, 5.
// note that some invisible symbols may thus sneak into the message;
    let message = match String::decode(&mut &message_vec[..]) {
        Ok(a) => a,
        Err(_) => return Err(ErrorSigner::Input(InputSigner::MessageNotReadable)),
    };
    
// initialize index and indent
    let mut index: u32 = 0;
    let indent: u32 = 0;
    
    match try_get_network_specs(database_name, &network_specs_key)? {
        Some(network_specs) => {
            let address_key = AddressKey::from_multisigner(&author_multi_signer);
            match try_get_address_details(database_name, &address_key)? {
                Some(address_details) => {
                    if address_details.network_id.contains(&network_specs_key) {
                        let message_card = Card::ParserCard(&ParserCard::Text(message.to_string())).card(&mut index, indent);
                        let sign = TrDbColdSign::generate(SignContent::Message(message), &network_specs.name, &address_details.path, address_details.has_pwd, &author_multi_signer, Vec::new());
                        let checksum = sign.store_and_get_checksum (database_name)?;
                        let author_info = make_author_info(&author_multi_signer, network_specs.base58prefix, &address_details);
                        let network_info = format!("\"network_title\":\"{}\",\"network_logo\":\"{}\"", network_specs.title, network_specs.logo);
                        Ok(Action::Sign{content: format!("\"message\":[{}]", message_card), checksum, has_pwd: address_details.has_pwd, author_info, network_info})
                    }
                    else {
                        let author_card = Card::Author{author: &author_multi_signer, base58prefix: network_specs.base58prefix, address_details: &address_details}.card(&mut index, indent);
                        let warning_card = Card::Warning(Warning::NoNetworkID).card(&mut index, indent);
                        let message_card = Card::ParserCard(&ParserCard::Text(message)).card(&mut index, indent);
                        let network_card = Card::NetworkInfo(&network_specs).card(&mut index, indent);
                        Ok(Action::Read(format!("\"author\":[{}],\"warning\":[{}],\"message\":[{},{}]", author_card, warning_card, message_card, network_card)))
                    }
                }
                None => {
                    let author_card = Card::AuthorPlain{author: &author_multi_signer, base58prefix: network_specs.base58prefix}.card(&mut index, indent);
                    let warning_card = Card::Warning(Warning::AuthorNotFound).card(&mut index, indent);
                    let message_card = Card::ParserCard(&ParserCard::Text(message)).card(&mut index, indent);
                    let network_card = Card::NetworkInfo(&network_specs).card(&mut index, indent);
                    Ok(Action::Read(format!("\"author\":[{}],\"warning\":[{}],\"message\":[{},{}]", author_card, warning_card, message_card, network_card)))
                }
            }
        },
        None => {
            let author_card = Card::AuthorPublicKey(&author_multi_signer).card(&mut index, indent);
            let error_card = Card::Error(ErrorSigner::Input(InputSigner::UnknownNetwork{genesis_hash: genesis_hash_vec.to_vec(), encryption})).card(&mut index, indent);
            let message_card = Card::ParserCard(&ParserCard::Text(message)).card(&mut index, indent);
            let network_card = Card::NetworkGenesisHash(&genesis_hash_vec).card(&mut index, indent);
            Ok(Action::Read(format!("\"author\":[{}],\"error\":[{}],\"message\":[{},{}]", author_card, error_card, message_card, network_card)))
        },
    }
}
