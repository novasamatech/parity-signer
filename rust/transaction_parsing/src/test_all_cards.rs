use bitvec::prelude::{BitVec, Lsb0};
use definitions::{
    crypto::Encryption,
    history::MetaValuesDisplay,
    keyring::VerifierKey,
    metadata::MetaValues,
    navigation::{TransactionCard, TransactionCardSet},
    network_specs::{NetworkSpecs, VerifierValue},
    qr_transfers::ContentLoadTypes,
    test_all_errors_signer::error_signer,
    users::AddressDetails,
};
use parser::cards::ParserCard;
use sp_core::{crypto::AccountId32, H256};
use sp_runtime::{generic::Era, MultiSigner};
use std::str::FromStr;

use crate::cards::{Card, Warning};
use crate::holds::{GeneralHold, Hold};
use crate::TransactionAction;

const PUBLIC: [u8; 32] = [
    142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201,
    18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
];

fn verifier_value_sr25519() -> VerifierValue {
    VerifierValue::Standard {
        m: MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(PUBLIC)),
    }
}

/// Function to pring all types of cards.
/// Should be used to check how the cards are printed in the app.

pub fn make_all_cards() -> TransactionAction {
    let mut index = 0;
    let mut all_cards: Vec<TransactionCard> = Vec::new();
    let mut warning: Vec<TransactionCard> = Vec::new();
    let network_specs_westend = NetworkSpecs {
        base58prefix: 42,
        color: String::from("#660D35"),
        decimals: 12,
        encryption: Encryption::Sr25519,
        genesis_hash: H256::from_str(
            "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
        )
        .expect("known value"),
        logo: String::from("westend"),
        name: String::from("westend"),
        order: 3,
        path_id: String::from("//westend"),
        secondary_color: String::from("#262626"),
        title: String::from("Westend"),
        unit: String::from("WND"),
    };
    let address_details = AddressDetails {
        seed_name: String::from("Alice"),
        path: String::from("//Bob"),
        has_pwd: false,
        network_id: Vec::new(),
        encryption: Encryption::Sr25519,
    };
    let bv: BitVec<u8, Lsb0> = BitVec::from_vec(vec![32, 4, 155]);

    all_cards
        .push(Card::ParserCard(&ParserCard::Pallet("test_pallet".to_string())).card(&mut index, 0));
    all_cards.push(
        Card::ParserCard(&ParserCard::Method {
            method_name: "test_method".to_string(),
            docs: "verbose \ndescription \nof \nthe \nmethod".to_string(),
        })
        .card(&mut index, 0),
    );
    all_cards.push(
        Card::ParserCard(&ParserCard::Varname("test_Varname".to_string())).card(&mut index, 0),
    );
    all_cards.push(Card::ParserCard(&ParserCard::Default("12345".to_string())).card(&mut index, 0));
    all_cards.push(Card::ParserCard(&ParserCard::Text("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.
Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string())).card(&mut index,0));
    all_cards.push(
        Card::ParserCard(&ParserCard::Id {
            id: AccountId32::new(PUBLIC),
            base58prefix: 42,
        })
        .card(&mut index, 0),
    );
    all_cards.push(Card::ParserCard(&ParserCard::None).card(&mut index, 0));
    all_cards.push(
        Card::ParserCard(&ParserCard::IdentityField("Twitter".to_string())).card(&mut index, 0),
    );
    all_cards.push(Card::ParserCard(&ParserCard::BitVec(bv.to_string())).card(&mut index, 0));
    all_cards.push(
        Card::ParserCard(&ParserCard::Balance {
            number: "300.000000".to_string(),
            units: "KULU".to_string(),
        })
        .card(&mut index, 0),
    );
    all_cards.push(
        Card::ParserCard(&ParserCard::FieldName {
            name: "test_FieldName".to_string(),
            docs_field_name: "a very special field".to_string(),
            path_type: "field >> path >> TypePath".to_string(),
            docs_type: "type is difficult to describe".to_string(),
        })
        .card(&mut index, 0),
    );
    all_cards.push(
        Card::ParserCard(&ParserCard::FieldNumber {
            number: 1,
            docs_field_number: "less special field".to_string(),
            path_type: "field >> path >> TypePath".to_string(),
            docs_type: "type is just as difficult to describe".to_string(),
        })
        .card(&mut index, 0),
    );
    all_cards.push(
        Card::ParserCard(&ParserCard::EnumVariantName {
            name: "test_EnumVariantName".to_string(),
            docs_enum_variant: "".to_string(),
        })
        .card(&mut index, 0),
    );
    all_cards.push(Card::ParserCard(&ParserCard::Era(Era::Immortal)).card(&mut index, 0));
    all_cards.push(Card::ParserCard(&ParserCard::Era(Era::Mortal(64, 31))).card(&mut index, 0));
    all_cards.push(Card::ParserCard(&ParserCard::Nonce("15".to_string())).card(&mut index, 0));
    all_cards.push(
        Card::ParserCard(&ParserCard::BlockHash(
            H256::from_str("a8dfb73a4b44e6bf84affe258954c12db1fe8e8cf00b965df2af2f49c1ec11cd")
                .expect("checked value"),
        ))
        .card(&mut index, 0),
    );
    all_cards.push(
        Card::ParserCard(&ParserCard::Tip {
            number: "0".to_string(),
            units: "pWND".to_string(),
        })
        .card(&mut index, 0),
    );
    all_cards.push(
        Card::ParserCard(&ParserCard::NetworkNameVersion {
            name: "westend".to_string(),
            version: "9110".to_string(),
        })
        .card(&mut index, 0),
    );
    all_cards.push(Card::ParserCard(&ParserCard::TxVersion("5".to_string())).card(&mut index, 0));

    all_cards.push(
        Card::Author {
            author: &MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(PUBLIC)),
            base58prefix: 42,
            address_details: &address_details,
        }
        .card(&mut index, 0),
    );
    all_cards.push(
        Card::AuthorPlain {
            author: &MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(PUBLIC)),
            base58prefix: 42,
        }
        .card(&mut index, 0),
    );
    all_cards.push(
        Card::AuthorPublicKey(&MultiSigner::Sr25519(sp_core::sr25519::Public::from_raw(
            PUBLIC,
        )))
        .card(&mut index, 0),
    );
    all_cards.push(Card::Verifier(&verifier_value_sr25519()).card(&mut index, 0));
    all_cards.push(
        Card::Meta(MetaValuesDisplay::get(&MetaValues {
            name: String::from("westend"),
            version: 9100,
            optional_base58prefix: Some(42),
            warn_incomplete_extensions: false,
            meta: Vec::new(),
        }))
        .card(&mut index, 0),
    );
    all_cards.push(Card::TypesInfo(ContentLoadTypes::from_slice(&[])).card(&mut index, 0));
    all_cards.push(Card::NewSpecs(&network_specs_westend.to_send()).card(&mut index, 0));
    all_cards.push(Card::NetworkInfo(&network_specs_westend).card(&mut index, 0));
    all_cards.push(
        Card::NetworkGenesisHash(network_specs_westend.genesis_hash.as_bytes()).card(&mut index, 0),
    );
    all_cards.push(
        Card::Derivations(&[
            "//Alice".to_string(),
            "//Alice/2/1".to_string(),
            "//secret//westend".to_string(),
        ])
        .card(&mut index, 0),
    );

    warning.push(Card::Warning(Warning::AuthorNotFound).card(&mut index, 0));
    warning.push(
        Card::Warning(Warning::NewerVersion {
            used_version: 50,
            latest_version: 9010,
        })
        .card(&mut index, 0),
    );
    warning.push(Card::Warning(Warning::NoNetworkID).card(&mut index, 0));
    warning.push(Card::Warning(Warning::NotVerified).card(&mut index, 0));
    warning.push(Card::Warning(Warning::UpdatingTypes).card(&mut index, 0));
    warning.push(Card::Warning(Warning::TypesNotVerified).card(&mut index, 0));
    warning.push(
        Card::Warning(Warning::GeneralVerifierAppeared(&GeneralHold {
            metadata_set: Vec::new(),
            network_specs_set: Vec::new(),
            types: true,
        }))
        .card(&mut index, 0),
    );
    warning.push(
        Card::Warning(Warning::VerifierChangingToGeneral {
            verifier_key: &VerifierKey::from_parts(network_specs_westend.genesis_hash.as_bytes()),
            hold: &Hold {
                metadata_set: Vec::new(),
                network_specs_set: Vec::new(),
            },
        })
        .card(&mut index, 0),
    );
    warning.push(
        Card::Warning(Warning::VerifierChangingToCustom {
            verifier_key: &VerifierKey::from_parts(network_specs_westend.genesis_hash.as_bytes()),
            hold: &Hold {
                metadata_set: Vec::new(),
                network_specs_set: Vec::new(),
            },
        })
        .card(&mut index, 0),
    );
    warning.push(
        Card::Warning(Warning::VerifierGeneralSuper {
            verifier_key: &VerifierKey::from_parts(network_specs_westend.genesis_hash.as_bytes()),
            hold: &Hold {
                metadata_set: Vec::new(),
                network_specs_set: Vec::new(),
            },
        })
        .card(&mut index, 0),
    );
    warning.push(Card::Warning(Warning::TypesAlreadyThere).card(&mut index, 0));
    warning.push(
        Card::Warning(Warning::NetworkSpecsAlreadyThere(
            &network_specs_westend.title,
        ))
        .card(&mut index, 0),
    );
    warning.push(Card::Warning(Warning::MetadataExtensionsIncomplete).card(&mut index, 0));

    let errors = error_signer()
        .into_iter()
        .map(|e| Card::Error(e).card(&mut index, 0))
        .collect();

    TransactionAction::Read {
        r: TransactionCardSet {
            meta: Some(all_cards),
            error: Some(errors),
            warning: Some(warning),
            ..Default::default()
        },
    }
}
