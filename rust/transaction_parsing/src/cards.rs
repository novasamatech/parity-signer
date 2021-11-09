use hex;
use definitions::{crypto::Encryption, history::MetaValuesDisplay, keyring::VerifierKey, network_specs::{ChainSpecsToSend, Verifier}, qr_transfers::ContentLoadTypes};
use blake2_rfc::blake2b::blake2b;
use sp_runtime::generic::Era;
use parser::cards::ParserCard;

use crate::error::Error;
use crate::helpers::{GeneralHold, Hold};

pub (crate) enum Card <'a> {
    ParserCard(&'a ParserCard),
    Author {base58_author: &'a str, seed_name: &'a str, path: &'a str, has_pwd: bool, name: &'a str},
    AuthorPlain (&'a str),
    AuthorPublicKey{author_public_key: Vec<u8>, encryption: Encryption},
    Verifier(&'a Verifier),
    Meta(MetaValuesDisplay),
    TypesInfo(ContentLoadTypes),
    NewSpecs(&'a ChainSpecsToSend),
    Message(&'a str),
    NetworkName(&'a str),
    NetworkGenesisHash(&'a Vec<u8>),
    Warning (Warning <'a>),
    Error (Error),
}

pub (crate) enum Warning <'a> {
    AuthorNotFound,
    NewerVersion {used_version: u32, latest_version: u32},
    NoNetworkID,
    NotVerified,
    UpdatingTypes,
    TypesNotVerified,
    GeneralVerifierAppeared(&'a GeneralHold),
    VerifierChangingToGeneral{verifier_key: &'a VerifierKey, hold: &'a Hold},
    VerifierChangingToCustom{verifier_key: &'a VerifierKey, hold: &'a Hold},
    VerifierGeneralSuper{verifier_key: &'a VerifierKey, hold: &'a Hold},
    TypesAlreadyThere,
    NetworkSpecsAlreadyThere(&'a str), // network title
}

impl <'a> Warning <'a> {
    pub (crate) fn show (&self) -> String {
        match &self {
            Warning::AuthorNotFound => String::from("Transaction author public key not found."),
            Warning::NewerVersion {used_version, latest_version} => format!("Transaction uses outdated runtime version {}. Latest known available version is {}.", used_version, latest_version),
            Warning::NoNetworkID => String::from("Public key is on record, but not associated with the network used."),
            Warning::NotVerified => String::from("Received network information is not verified."),
            Warning::UpdatingTypes => String::from("Updating types (really rare operation)."),
            Warning::TypesNotVerified => String::from("Received types information is not verified."),
            Warning::GeneralVerifierAppeared(x) => format!("Received message is verified by a new general verifier. Currently no general verifier is set, and proceeding will update the general verifier to the received value. All previously acquired information associated with general verifier will be purged. {}", x.show()),
            Warning::VerifierChangingToGeneral{verifier_key, hold} => format!("Received message is verified by the general verifier. Current verifier for network with genesis hash {} is a custom one, and proceeding will update the network verifier to general. All previously acquired information associated with former custom verifier will be purged. {}", hex::encode(verifier_key.genesis_hash()), hold.show()),
            Warning::VerifierChangingToCustom{verifier_key, hold} => format!("Received message is verified. Currently no verifier is set for network with genesis hash {}. Proceeding will update the network verifier to custom verifier. All previously acquired network information that was received unverified will be purged. {}", hex::encode(verifier_key.genesis_hash()), hold.show()),
            Warning::VerifierGeneralSuper{verifier_key, hold} => format!("Received message is verified. Currently no verifier is set for network with genesis hash {} and no general verifier is set. Proceeding will update the network verifier to general. All previously acquired network information that was received unverified will be purged. {}", hex::encode(verifier_key.genesis_hash()), hold.show()),
            Warning::TypesAlreadyThere => String::from("Received types information is identical to the one that was in the database."),
            Warning::NetworkSpecsAlreadyThere (x) => format!("Received network specs information for {} is same as the one already in the database.", x),
        }
    }
}

fn fancy (index: &mut u32, indent: u32, card_type: &str, decoded_string: &str) -> String {
    let out = format!("{{\"index\":{},\"indent\":{},\"type\":\"{}\",\"payload\":{}}}", index, indent, card_type, decoded_string);
    *index = *index+1;
    out
}

impl <'a> Card <'a> {
    pub (crate) fn card (&self, index: &mut u32, indent: u32) -> String {
        match &self {
            Card::ParserCard (m) => match m {
                ParserCard::Pallet (pallet_name) => fancy(index, indent, "pallet", &format!("\"{}\"", pallet_name)),
                ParserCard::Method {method_name, docs} => fancy(index, indent, "method", &format!("{{\"method_name\":\"{}\",\"docs\":\"{}\"}}", method_name, hex::encode(docs.as_bytes()))),
                ParserCard::Varname (varname) => fancy(index, indent, "varname", &format!("\"{}\"", varname)),
                ParserCard::Default (decoded_string) => fancy(index, indent, "default", &format!("\"{}\"", decoded_string)),
                ParserCard::Id (base58_id) => fancy(index, indent, "Id", &format!("\"{}\"", base58_id)),
                ParserCard::None => fancy(index, indent, "none", "\"\""),
                ParserCard::IdentityField (variant) => fancy(index, indent, "identity_field", &format!("\"{}\"", variant)),
                ParserCard::BitVec (bv) => fancy(index, indent, "bitvec", &format!("\"{}\"", bv)),
                ParserCard::Balance {number, units} => fancy(index, indent, "balance", &format!("{{\"amount\":\"{}\",\"units\":\"{}\"}}", number, units)),
                ParserCard::FieldName {name, docs_field_name, path_type, docs_type} => fancy(index, indent, "field_name", &format!("{{\"name\":\"{}\",\"docs_field_name\":\"{}\",\"path_type\":\"{}\",\"docs_type\":\"{}\"}}", name, hex::encode(docs_field_name.as_bytes()), path_type, hex::encode(docs_type.as_bytes()))),
                ParserCard::FieldNumber {number, docs_field_number, path_type, docs_type} => fancy(index, indent, "field_number", &format!("{{\"number\":\"{}\",\"docs_field_number\":\"{}\",\"path_type\":\"{}\",\"docs_type\":\"{}\"}}", number, hex::encode(docs_field_number.as_bytes()), path_type, hex::encode(docs_type.as_bytes()))),
                ParserCard::EnumVariantName {name, docs_enum_variant} => fancy(index, indent, "enum_variant_name", &format!("{{\"name\":\"{}\",\"docs_enum_variant\":\"{}\"}}", name, hex::encode(docs_enum_variant.as_bytes()))),
                ParserCard::Era(era) => match era {
                    Era::Immortal => fancy(index, indent, "era", "{\"era\":\"Immortal\",\"phase\":\"\",\"period\":\"\"}"),
                    Era::Mortal(period, phase)  => fancy(index, indent, "era", &format!("{{\"era\":\"Mortal\",\"phase\":\"{}\",\"period\":\"{}\"}}", phase, period)),
                },
                ParserCard::Nonce (nonce) => fancy(index, indent, "nonce", &format!("\"{}\"", nonce)),
                ParserCard::BlockHash (block_hash) => fancy(index, indent, "block_hash", &format!("\"{}\"", hex::encode(block_hash))),
                ParserCard::Tip {number, units} => fancy(index, indent, "tip", &format!("{{\"amount\":\"{}\",\"units\":\"{}\"}}", number, units)),
                ParserCard::NetworkNameVersion {name, version} => fancy(index, indent, "name_version", &format!("{{\"name\":\"{}\",\"version\":\"{}\"}}", name, version)),
                ParserCard::TxVersion (x) => fancy(index, indent, "tx_version", &format!("\"{}\"", x)),
            },
            Card::Author {base58_author, seed_name, path, has_pwd, name} => fancy(index, indent, "author", &format!("{{\"base58\":\"{}\",\"seed\":\"{}\",\"derivation_path\":\"{}\",\"has_password\":{},\"name\":\"{}\"}}", base58_author, seed_name, path, has_pwd, name)),
            Card::AuthorPlain (base58_author) => fancy(index, indent, "author_plain", &format!("{{\"base58\":\"{}\"}}", base58_author)),
            Card::AuthorPublicKey{author_public_key, encryption} => fancy(index, indent, "author_public_key", &format!("{{\"hex\":\"{}\",\"crypto\":\"{}\"}}", hex::encode(author_public_key), encryption.show())),
            Card::Verifier(x) => fancy(index, indent, "verifier", &x.show_card()),
            Card::Meta(x) => fancy(index, indent, "meta", &format!("{{{}}}", x.show())),
            Card::TypesInfo(x) => fancy(index, indent, "types_hash", &format!("\"{}\"", hex::encode(blake2b(32, &[], &x.store()).as_bytes()))),
            Card::NewSpecs(x) => fancy(index, indent, "new_specs", &format!("{{{}}}", x.show())),
            Card::Message(x) => fancy(index, indent, "message", &format!("\"{}\"", hex::encode(x.as_bytes()))),
            Card::NetworkName(x) => fancy(index, indent, "network_name", &format!("\"{}\"", x)),
            Card::NetworkGenesisHash(x) => fancy(index, indent, "network_genesis_hash", &format!("\"{}\"", hex::encode(x))),
            Card::Warning (warn) => fancy(index, indent, "warning", &format!("\"{}\"", warn.show())),
            Card::Error (err) => fancy(index, indent, "error", &format!("\"{}\"", err.show())),
        }
    }
}


pub enum Action {
    Sign (u32),
    Stub (u32),
}

fn print_action (action: &str, checksum: &u32) -> String {
    format!("\"action\":{{\"type\":\"{}\",\"payload\":\"{}\"}}", action, checksum)
}

impl Action {
    pub fn card (&self) -> String {
        match &self {
            Action::Sign(x) => print_action("sign", x),
            Action::Stub(x) => print_action("stub", x),
        }
    }
}
