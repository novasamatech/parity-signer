use hex;
use sp_runtime::generic::Era;


#[derive(Clone)]
pub enum MethodCard {
    Pallet (String), // pallet name
    Method {method_name: String, docs: String},
    Varname (String),
    Default (String),
    Id (String),
    None,
    IdentityField (String),
    BitVec (String), // String from printing BitVec
    Balance {number: String, units: String},
    FieldName {name: String, docs_field_name: String, path_type: String, docs_type: String},
    FieldNumber {number: usize, docs_field_number: String, path_type: String, docs_type: String},
    EnumVariantName {name: String, docs_enum_variant: String},
    Era(Era),
    Nonce (String),
    NetworkName (String),
    BlockHash ([u8; 32]),
    Tip {number: String, units: String},
    SpecVersion (String),
    TxVersion (String),
}

impl MethodCard {
/*
    pub fn show(&self, indent: u32) -> String {
        match &self {
            MethodCard::Pallet (pallet_name) => readable(indent, "pallet", &pallet_name),
            MethodCard::Method {method_name, docs} => readable(indent, "method", &format!("{{method_name: {}, docs: {}}}", method_name, docs)),
            MethodCard::Varname (varname) => readable(indent, "varname", &varname),
            MethodCard::Default (decoded_string) => readable(indent, "default", &decoded_string),
            MethodCard::Id (base58_id) => readable(indent, "Id", &base58_id),
            MethodCard::None => readable(indent, "none", ""),
            MethodCard::IdentityField (variant) => readable(indent, "identity_field", &variant),
            MethodCard::BitVec (bv) => readable(indent, "bitvec", &bv),
            MethodCard::Balance {number, units} => readable(indent, "balance", &format!("{{amount: {}, units: {}}}", number, units)),
            MethodCard::FieldName {name, docs_field_name, path_type, docs_type} => readable(indent, "field_name", &format!("{{name: {}, docs_field_name: {}, path_type: {}, docs_type: {}}}", name, docs_field_name, path_type, docs_type)),
            MethodCard::FieldNumber {number, docs_field_number, path_type, docs_type} => readable(indent, "field_number", &format!("{{number: {}, docs_field_number: {}, path_type: {}, docs_type: {}}}", number, docs_field_number, path_type, docs_type)),
            MethodCard::EnumVariantName {name, docs_enum_variant} => readable(indent, "enum_variant_name", &format!("{{name: {}, docs_enum_variant: {}}}", name, docs_enum_variant)),
            MethodCard::Era(era) => match era {
                Era::Immortal => readable(indent, "era", "era: Immortal"),
                Era::Mortal(phase, period)  => readable(indent, "era", &format!("{{era: Mortal, phase: {}, period: {}}}", phase, period)),
            },
            MethodCard::Nonce (nonce) => readable(indent, "nonce", &nonce),
            MethodCard::NetworkName (network_name) => readable(indent, "network", &network_name),
            MethodCard::BlockHash (block_hash) => readable(indent, "block_hash", &hex::encode(block_hash)),
            MethodCard::Tip {number, units} => readable(indent, "tip", &format!("{{amount: {}, units: {}}}", number, units)),
            MethodCard::SpecVersion (x) => readable(indent, "version", &x),
            MethodCard::TxVersion (x) => readable(indent, "tx_version", &x),
        }
    }
*/
    pub fn show_no_docs(&self, indent: u32) -> String {
        match &self {
            MethodCard::Pallet (pallet_name) => readable(indent, "pallet", &pallet_name),
            MethodCard::Method {method_name, docs: _} => readable(indent, "method", method_name),
            MethodCard::Varname (varname) => readable(indent, "varname", &varname),
            MethodCard::Default (decoded_string) => readable(indent, "default", &decoded_string),
            MethodCard::Id (base58_id) => readable(indent, "Id", &base58_id),
            MethodCard::None => readable(indent, "none", ""),
            MethodCard::IdentityField (variant) => readable(indent, "identity_field", &variant),
            MethodCard::BitVec (bv) => readable(indent, "bitvec", &bv),
            MethodCard::Balance {number, units} => readable(indent, "balance", &format!("{} {}", number, units)),
            MethodCard::FieldName {name, docs_field_name: _, path_type: _, docs_type: _} => readable(indent, "field_name", &name),
            MethodCard::FieldNumber {number, docs_field_number: _, path_type: _, docs_type: _} => readable(indent, "field_number", &number.to_string()),
            MethodCard::EnumVariantName {name, docs_enum_variant: _} => readable(indent, "enum_variant_name", &name),
            MethodCard::Era(era) => match era {
                Era::Immortal => readable(indent, "era", "Immortal"),
                Era::Mortal(phase, period)  => readable(indent, "era", &format!("Mortal, phase: {}, period: {}", phase, period)),
            },
            MethodCard::Nonce (nonce) => readable(indent, "nonce", &nonce),
            MethodCard::NetworkName (network_name) => readable(indent, "network", &network_name),
            MethodCard::BlockHash (block_hash) => readable(indent, "block_hash", &hex::encode(block_hash)),
            MethodCard::Tip {number, units} => readable(indent, "tip", &format!("{} {}", number, units)),
            MethodCard::SpecVersion (x) => readable(indent, "version", &x),
            MethodCard::TxVersion (x) => readable(indent, "tx_version", &x),
        }
    }
}

fn readable (indent: u32, card_type: &str, card_payload: &str) -> String {
    format!("{}{}: {}", "  ".repeat(indent as usize), card_type, card_payload)
}
