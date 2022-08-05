use db_handling::{
    db_transactions::{SignContent, TrDbColdSign},
    helpers::{try_get_address_details, try_get_network_specs},
};
use definitions::{
    error_signer::{ErrorSigner, InputSigner},
    keyring::{AddressKey, NetworkSpecsKey},
    navigation::TransactionCardSet,
};
use parser::cards::ParserCard;

use crate::cards::{make_author_info, Card, Warning};
use crate::helpers::multisigner_msg_genesis_encryption;
use crate::TransactionAction;

pub fn process_message(
    data_hex: &str,
    database_name: &str,
) -> Result<TransactionAction, ErrorSigner> {
    let (author_multi_signer, message_vec, genesis_hash, encryption) =
        multisigner_msg_genesis_encryption(data_hex)?;
    let network_specs_key = NetworkSpecsKey::from_parts(&genesis_hash, &encryption);

    // Message starts with `<Bytes>` and ends with `</Bytes>`. Data in the
    // middle is the message itself represented as bytes.
    //
    // Printed part is message in the middle. Signed part will be whole thing
    // with wrapper.
    let message_cut_vec = match message_vec.strip_prefix(br#"<Bytes>"#) {
        Some(a) => match a.strip_suffix(br#"</Bytes>"#) {
            Some(b) => b,
            None => return Err(ErrorSigner::Input(InputSigner::MessageNoWrapper)),
        },
        None => return Err(ErrorSigner::Input(InputSigner::MessageNoWrapper)),
    };

    let message = String::from_utf8(message_cut_vec.to_vec())
        .map_err(|_| ErrorSigner::Input(InputSigner::MessageNotValidUtf8))?;

    // initialize index and indent
    let mut index: u32 = 0;
    let indent: u32 = 0;

    match try_get_network_specs(database_name, &network_specs_key)? {
        Some(network_specs) => {
            let address_key = AddressKey::from_multisigner(&author_multi_signer);
            match try_get_address_details(database_name, &address_key)? {
                Some(address_details) => {
                    if address_details.network_id.contains(&network_specs_key) {
                        let message_card = Card::ParserCard(&ParserCard::Text(message.to_string()))
                            .card(&mut index, indent);
                        let sign = TrDbColdSign::generate(
                            SignContent::Message(message),
                            &network_specs.name,
                            &address_details.path,
                            address_details.has_pwd,
                            &author_multi_signer,
                            Vec::new(),
                        );
                        let checksum = sign.store_and_get_checksum(database_name)?;
                        let author_info = make_author_info(
                            &author_multi_signer,
                            network_specs.base58prefix,
                            &address_details,
                        );
                        let network_info = network_specs;
                        Ok(TransactionAction::Sign {
                            content: TransactionCardSet {
                                message: Some(vec![message_card]),
                                ..Default::default()
                            },
                            checksum,
                            has_pwd: address_details.has_pwd,
                            author_info,
                            network_info,
                        })
                    } else {
                        let author_card = Card::Author {
                            author: &author_multi_signer,
                            base58prefix: network_specs.base58prefix,
                            address_details: &address_details,
                        }
                        .card(&mut index, indent);
                        let warning_card =
                            Card::Warning(Warning::NoNetworkID).card(&mut index, indent);
                        let message_card =
                            Card::ParserCard(&ParserCard::Text(message)).card(&mut index, indent);
                        let network_card =
                            Card::NetworkInfo(&network_specs).card(&mut index, indent);
                        Ok(TransactionAction::Read {
                            r: TransactionCardSet {
                                author: Some(vec![author_card]),
                                warning: Some(vec![warning_card]),
                                message: Some(vec![message_card]),
                                new_specs: Some(vec![network_card]),
                                ..Default::default()
                            },
                        })
                    }
                }
                None => {
                    let author_card = Card::AuthorPlain {
                        author: &author_multi_signer,
                        base58prefix: network_specs.base58prefix,
                    }
                    .card(&mut index, indent);
                    let warning_card =
                        Card::Warning(Warning::AuthorNotFound).card(&mut index, indent);
                    let message_card =
                        Card::ParserCard(&ParserCard::Text(message)).card(&mut index, indent);
                    let network_card = Card::NetworkInfo(&network_specs).card(&mut index, indent);
                    Ok(TransactionAction::Read {
                        r: TransactionCardSet {
                            author: Some(vec![author_card]),
                            warning: Some(vec![warning_card]),
                            message: Some(vec![message_card]),
                            new_specs: Some(vec![network_card]),
                            ..Default::default()
                        },
                    })
                }
            }
        }
        None => {
            let author_card = Card::AuthorPublicKey(&author_multi_signer).card(&mut index, indent);
            let error_card = Card::Error(ErrorSigner::Input(InputSigner::UnknownNetwork {
                genesis_hash,
                encryption,
            }))
            .card(&mut index, indent);
            let message_card =
                Card::ParserCard(&ParserCard::Text(message)).card(&mut index, indent);
            let network_card =
                Card::NetworkGenesisHash(genesis_hash.as_ref()).card(&mut index, indent);
            Ok(TransactionAction::Read {
                r: TransactionCardSet {
                    author: Some(vec![author_card]),
                    error: Some(vec![error_card]),
                    message: Some(vec![message_card]),
                    new_specs: Some(vec![network_card]),
                    ..Default::default()
                },
            })
        }
    }
}
