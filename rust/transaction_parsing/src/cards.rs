use bitvec::prelude::{BitVec, Lsb0};
use hex;

use super::error::{Error};
use super::AuthorPublicKey;

pub enum Card <'a> {
    Call {pallet: &'a str, method: &'a str},
    Varname (&'a str),
    Default (&'a str),
    Id (&'a str),
    None,
    IdentityField (&'a str),
    BitVec (BitVec<Lsb0, u8>),
    Balance {number: &'a str, units: &'a str},
    FieldName (&'a str),
    FieldNumber (usize),
    EnumVariantName (&'a str),
    EraImmortalNonce (u64),
    EraMortalNonce {phase: u64, period: u64, nonce: u64},
    Tip {number: &'a str, units: &'a str},
    TipPlain (u128),
    BlockHash (&'a str),
    TxSpec {network: &'a str, version: u32, tx_version: u32},
    TxSpecPlain {gen_hash: &'a str, version: u32, tx_version: u32},
    Author {base58_author: &'a str, path: &'a str, has_pwd: bool, name: &'a str},
    AuthorPlain (&'a str),
    AuthorPublicKey (AuthorPublicKey),
    Warning (Warning),
    Error (Error),
}

pub enum Warning {
    AuthorNotFound,
    NewerVersion {used_version: u32, latest_version: u32},
}

impl Warning {
    pub fn show (&self) -> String {
        match &self {
            Warning::AuthorNotFound => String::from("\"Transaction author public key not found.\""),
            Warning::NewerVersion {used_version, latest_version} => format!("\"Transaction uses outdated runtime version {}. Latest known available version is {}.\"", used_version, latest_version),
        }
    }
}

pub fn fancy (index: u32, indent: u32, card_type: &str, decoded_string: &str) -> String {
    format!("{{\"index\":{},\"indent\":{},\"type\":\"{}\",\"payload\":{}}}", index, indent, card_type, decoded_string)
}

impl <'a> Card <'a> {
    pub fn card (&self, index: u32, indent: u32) -> String {
        match &self {
            Card::Call {pallet, method} => fancy(index, indent, "call", &format!("{{\"method\":\"{}\",\"pallet\":\"{}\"}}", method, pallet)),
            Card::Varname (varname) => fancy(index, indent, "varname", &format!("\"{}\"", varname)),
            Card::Default (decoded_string) => fancy(index, indent, "default", &format!("\"{}\"", decoded_string)),
            Card::Id (base58_id) => fancy(index, indent, "Id", &format!("\"{}\"", base58_id)),
            Card::None => fancy(index, indent, "none", "\"\""),
            Card::IdentityField (variant) => fancy(index, indent, "identity_field", &format!("\"{}\"", variant)),
            Card::BitVec (bv) => fancy(index, indent, "bitvec", &format!("\"{}\"", bv)),
            Card::Balance {number, units} => fancy(index, indent, "balance", &format!("{{\"amount\":\"{}\",\"units\":\"{}\"}}", number, units)),
            Card::FieldName (x) => fancy(index, indent, "field_name", &format!("\"{}\"", x)),
            Card::FieldNumber (i) => fancy(index, indent, "field_number", &format!("\"{}\"", i)),
            Card::EnumVariantName (name) => fancy(index, indent, "enum_variant_name", &format!("\"{}\"", name)),
            Card::EraImmortalNonce (nonce) => fancy(index, indent, "era_immortal_nonce", &format!("{{\"era\":\"Immortal\",\"nonce\":\"{}\"}}", nonce)),
            Card::EraMortalNonce {phase, period, nonce} => fancy(index, indent, "era_mortal_nonce", &format!("{{\"era\":\"Mortal\",\"phase\":\"{}\",\"period\":\"{}\",\"nonce\":\"{}\"}}", phase, period, nonce)),
            Card::Tip {number, units} => fancy(index, indent, "tip", &format!("{{\"amount\":\"{}\",\"units\":\"{}\"}}", number, units)),
            Card::TipPlain (x) => fancy(index, indent, "tip_plain", &format!("\"{}\"", x)),
            Card::BlockHash (hex_block_hash) => fancy(index, indent, "block_hash", &format!("\"{}\"", hex_block_hash)),
            Card::TxSpec {network, version, tx_version} => fancy(index, indent, "tx_spec", &format!("{{\"network\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", network, version, tx_version)),
            Card::TxSpecPlain {gen_hash, version, tx_version} => fancy(index, indent, "tx_spec_plain", &format!("{{\"network_genesis_hash\":\"{}\",\"version\":\"{}\",\"tx_version\":\"{}\"}}", gen_hash, version, tx_version)),
            Card::Author {base58_author, path, has_pwd, name} => fancy(index, indent, "author", &format!("{{\"base58\":\"{}\",\"derivation_path\":\"{}\",\"has_password\":\"{}\",\"name\":\"{}\"}}", base58_author, path, has_pwd, name)),
            Card::AuthorPlain (base58_author) => fancy(index, indent, "author_plain", &format!("{{\"base58\":\"{}\"}}", base58_author)),
            Card::AuthorPublicKey (author_pub_key) => match author_pub_key {
                AuthorPublicKey::Ed25519(x) => fancy(index, indent, "author_public_key", &format!("{{\"hex\":\"{}\",\"crypto\":\"ed25519\"}}", &hex::encode(x))),
                AuthorPublicKey::Sr25519(x) => fancy(index, indent, "author_public_key", &format!("{{\"hex\":\"{}\",\"crypto\":\"sr25519\"}}", &hex::encode(x))),
                AuthorPublicKey::Ecdsa(x) => fancy(index, indent, "author_public_key", &format!("{{\"hex\":\"{}\",\"crypto\":\"ecdsa\"}}", &hex::encode(x))),
            },
            Card::Warning (warn) => fancy(index, indent, "warning", &format!("\"{}\"", warn.show())),
            Card::Error (err) => fancy(index, indent, "error", &format!("\"{}\"", err.show())),
        }
    }
}
