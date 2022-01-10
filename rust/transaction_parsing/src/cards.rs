use hex;
use sp_core::crypto::{Ss58Codec, Ss58AddressFormat};
use sp_runtime::{generic::Era, MultiSigner};

use constants::HALFSIZE;
use definitions::{crypto::Encryption, error::{ErrorSigner, ErrorSource, Signer}, helpers::make_identicon_from_multisigner, history::MetaValuesDisplay, keyring::{print_multisigner_as_base58, VerifierKey}, network_specs::{NetworkSpecs, NetworkSpecsToSend, VerifierValue}, print::{export_complex_single, export_plain_vector}, qr_transfers::ContentLoadTypes, users::AddressDetails};
use parser::cards::ParserCard;
use plot_icon::png_data_from_vec;

use crate::holds::{GeneralHold, Hold};

pub (crate) enum Card <'a> {
    ParserCard(&'a ParserCard),
    Author {author: &'a MultiSigner, base58prefix: u16, address_details: &'a AddressDetails},
    AuthorPlain {author: &'a MultiSigner, base58prefix: u16},
    AuthorPublicKey(&'a MultiSigner),
    Verifier(&'a VerifierValue),
    Meta(MetaValuesDisplay),
    TypesInfo(ContentLoadTypes),
    NewSpecs(&'a NetworkSpecsToSend),
    NetworkInfo(&'a NetworkSpecs),
    NetworkGenesisHash(&'a Vec<u8>),
    Derivations(&'a Vec<String>),
    Warning (Warning <'a>),
    Error (ErrorSigner),
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
                ParserCard::Text (decoded_text) => fancy(index, indent, "text", &format!("\"{}\"", hex::encode(decoded_text.as_bytes()))),
                ParserCard::Id {id, base58prefix} => {
                    let hex_identicon = match png_data_from_vec(&<[u8;32]>::from(id.to_owned()).to_vec(), HALFSIZE) {
                        Ok(a) => hex::encode(a),
                        Err(_) => "".to_string(),
                    };
                    fancy(index, indent, "Id", &format!("{{\"base58\":\"{}\",\"identicon\":\"{}\"}}", id.to_ss58check_with_version(Ss58AddressFormat::Custom(*base58prefix)), hex_identicon))
                },
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
            Card::Author {author, base58prefix, address_details} => fancy(index, indent, "author", &format!("{{{}}}", make_author_info(author, *base58prefix, address_details))),
            Card::AuthorPlain {author, base58prefix} => {
                let hex_identicon = match make_identicon_from_multisigner(author) {
                    Ok(a) => hex::encode(a),
                    Err(_) => "".to_string(),
                };
                fancy(index, indent, "author_plain", &format!("{{\"base58\":\"{}\",\"identicon\":\"{}\"}}", print_multisigner_as_base58(&author, Some(*base58prefix)), hex_identicon))
            },
            Card::AuthorPublicKey(author) => {
                let hex_identicon = match make_identicon_from_multisigner(author) {
                    Ok(a) => hex::encode(a),
                    Err(_) => "".to_string(),
                };
                let insert = match author {
                    MultiSigner::Ed25519(p) => format!("{{\"hex\":\"{}\",\"crypto\":\"{}\",\"identicon\":\"{}\"}}", hex::encode(p.to_vec()), Encryption::Ed25519.show(), hex_identicon),
                    MultiSigner::Sr25519(p) => format!("{{\"hex\":\"{}\",\"crypto\":\"{}\",\"identicon\":\"{}\"}}", hex::encode(p.to_vec()), Encryption::Sr25519.show(), hex_identicon),
                    MultiSigner::Ecdsa(p) => format!("{{\"hex\":\"{}\",\"crypto\":\"{}\",\"identicon\":\"{}\"}}", hex::encode(p.0.to_vec()), Encryption::Ecdsa.show(), hex_identicon),
                };
                fancy(index, indent, "author_public_key", &insert)
            },
            Card::Verifier(x) => fancy(index, indent, "verifier", &export_complex_single(x, |a| a.show_card())),
            Card::Meta(x) => fancy(index, indent, "meta", &format!("{{{}}}", x.show())),
            Card::TypesInfo(x) => fancy(index, indent, "types", &format!("{{{}}}", x.show())),
            Card::NewSpecs(x) => fancy(index, indent, "new_specs", &format!("{{{}}}", x.show())),
            Card::NetworkInfo(x) => fancy(index, indent, "network_info", &format!("{{\"network_title\":\"{}\",\"network_logo\":\"{}\"}}", x.title, x.logo)),
            Card::NetworkGenesisHash(x) => fancy(index, indent, "network_genesis_hash", &format!("\"{}\"", hex::encode(x))),
            Card::Derivations(x) => fancy(index, indent, "derivations", &export_plain_vector(x)),
            Card::Warning (warn) => fancy(index, indent, "warning", &format!("\"{}\"", warn.show())),
            Card::Error (err) => fancy(index, indent, "error", &format!("\"{}\"", <Signer>::show(&err))),
        }
    }
}


pub (crate) fn make_author_info (author: &MultiSigner, base58prefix: u16, address_details: &AddressDetails) -> String {
    let hex_identicon = match make_identicon_from_multisigner(author) {
        Ok(a) => hex::encode(a),
        Err(_) => "".to_string(),
    };
    format!("\"base58\":\"{}\",\"identicon\":\"{}\",\"seed\":\"{}\",\"derivation_path\":\"{}\",\"has_pwd\":{}", print_multisigner_as_base58(&author, Some(base58prefix)), hex_identicon, address_details.seed_name, address_details.path, address_details.has_pwd)
}
