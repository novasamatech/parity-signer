//! Default data for [Signer](https://github.com/paritytech/parity-signer)
//! and Signer-supporting ecosystem.  
//!
//! This crate deals with data loaded into Signer on build and retained on reset.
//!
//! Signer supports by default three networks: Polkadot, Kusama, and Westend,
//! both on the cold and the hot side. More default networks could be considered
//! in the future.
//!
//! # Defaults in the cold database
//!
//! Default build of the cold database for Signer includes:
//!
//! - general verifier `Verifier` set to `Some(VerifierValue)` with Parity
//! public key as a verifier value
//! - network specs `NetworkSpecs` for default networks
//! - verifiers for default networks, set to
//! `CurrentVerifier::Valid(ValidCurrentVerifier::General)`
//! - two latest metadata versions for each of the default networks
//! - default types information
//!
//! Latest metadata entries get updated as soon as they are published and could
//! be fetched via rpc calls. For this, new metadata entry is added into
//! `release_metadata` folder, and older one is removed.
//!
//! # Defaults in the hot database
//!
//! Default hot database contains:
//!
//! - network specs `NetworkSpecsToSend` for default networks
//! - address book containing the data for default networks
//! - default types information
//!
//! # Features
//! Feature `"signer"` corresponds to everything related to Signer air-gapped
//! device. Currently the only Signer-related default is `Verifier` with Parity
//! public key, that could be set up as a general verifier after Signer wipe by
//! the user.
//!
//! Feature `"active"` corresponds to all Signer-related things happening
//! **without** air-gap. This includes generating default:
//! - network specs sets for cold and hot databases
//! - verifiers for cold databases
//! - metadata sets for cold databases
//! - address book for hot database
//! - types information for cold and hot databases
//!
//! Feature `"test"` includes both `"signer"` and `"active"` features, along
//! with some testing, and is the default one.  

#![deny(unused_crate_dependencies)]

#[cfg(feature = "signer")]
use sp_runtime::MultiSigner;

#[cfg(feature = "active")]
use std::convert::TryInto;

#[cfg(feature = "active")]
use std::fs;

#[cfg(feature = "active")]
use regex::Regex;

#[cfg(feature = "active")]
use lazy_static::lazy_static;

#[cfg(feature = "active")]
use definitions::{
    crypto::Encryption,
    error::{ErrorSource, MetadataError, MetadataSource},
    error_active::{
        Active, DefaultLoading, ErrorActive, IncomingMetadataSourceActive,
        IncomingMetadataSourceActiveStr,
    },
    keyring::VerifierKey,
    metadata::{AddressBookEntry, MetaValues},
    network_specs::{CurrentVerifier, NetworkSpecs, NetworkSpecsToSend, ValidCurrentVerifier},
    qr_transfers::ContentLoadTypes,
    types::{Description, EnumVariant, EnumVariantType, StructField, TypeEntry},
};

#[cfg(feature = "signer")]
use definitions::network_specs::{Verifier, VerifierValue};

/// Real Parity public key, with Sr25519 encryption
///
/// To be used in [`VerifierValue`] for general verifier in default cold
/// database
#[cfg(feature = "signer")]
pub const DEFAULT_VERIFIER_PUBLIC: [u8; 32] = [
    0xc4, 0x6a, 0x22, 0xb9, 0xda, 0x19, 0x54, 0x0a, 0x77, 0xcb, 0xde, 0x23, 0x19, 0x7e, 0x5f, 0xd9,
    0x04, 0x85, 0xc7, 0x2b, 0x4e, 0xcf, 0x3c, 0x59, 0x9e, 0xcc, 0xa6, 0x99, 0x8f, 0x39, 0xbd, 0x57,
];

/// Generate default general verifier [`Verifier`], with Parity public key
/// inside.
#[cfg(feature = "signer")]
pub fn default_general_verifier() -> Verifier {
    Verifier(Some(VerifierValue::Standard(MultiSigner::Sr25519(
        sp_core::sr25519::Public::from_raw(DEFAULT_VERIFIER_PUBLIC),
    ))))
}

/// Network information that is not expected to change, source for network specs
/// entries in cold and hot databases and address book entries in hot database
#[cfg(feature = "active")]
struct DefaultNetworkInfo {
    address: String,
    base58prefix: u16,
    color: String,
    decimals: u8,
    encryption: Encryption,
    genesis_hash: [u8; 32],
    logo: String,
    name: String,
    order: u8,
    path_id: String,
    secondary_color: String,
    title: String,
    unit: String,
}

/// Populate `DefaultNetworkInfo` for default networks
#[cfg(feature = "active")]
fn default_network_info() -> [DefaultNetworkInfo; 3] {
    [
        DefaultNetworkInfo {
            address: String::from("wss://kusama-rpc.polkadot.io"),
            base58prefix: 2,
            color: String::from("#000"),
            decimals: 12,
            encryption: Encryption::Sr25519,
            genesis_hash: hex::decode(
                "b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe",
            )
            .expect("known value")
            .try_into()
            .expect("known value"),
            logo: String::from("kusama"),
            name: String::from("kusama"),
            order: 1,
            path_id: String::from("//kusama"),
            secondary_color: String::from("#262626"),
            title: String::from("Kusama"),
            unit: String::from("KSM"),
        },
        DefaultNetworkInfo {
            address: String::from("wss://rpc.polkadot.io"),
            base58prefix: 0,
            color: String::from("#E6027A"),
            decimals: 10,
            encryption: Encryption::Sr25519,
            genesis_hash: hex::decode(
                "91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3",
            )
            .expect("known value")
            .try_into()
            .expect("known value"),
            logo: String::from("polkadot"),
            name: String::from("polkadot"),
            order: 0,
            path_id: String::from("//polkadot"),
            secondary_color: String::from("#262626"),
            title: String::from("Polkadot"),
            unit: String::from("DOT"),
        },
        DefaultNetworkInfo {
            address: String::from("wss://westend-rpc.polkadot.io"),
            base58prefix: 42,
            color: String::from("#660D35"),
            decimals: 12,
            encryption: Encryption::Sr25519,
            genesis_hash: hex::decode(
                "e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e",
            )
            .expect("known value")
            .try_into()
            .expect("known value"),
            logo: String::from("westend"),
            name: String::from("westend"),
            order: 2,
            path_id: String::from("//westend"),
            secondary_color: String::from("#262626"),
            title: String::from("Westend"),
            unit: String::from("WND"),
        },
    ]
}

/// Generate network specs [`NetworkSpecs`] set for the default networks, for
/// cold database
#[cfg(feature = "active")]
pub fn default_chainspecs() -> Vec<NetworkSpecs> {
    let mut out: Vec<NetworkSpecs> = Vec::new();
    for x in default_network_info() {
        let new = NetworkSpecs {
            base58prefix: x.base58prefix,
            color: x.color.to_string(),
            decimals: x.decimals,
            encryption: x.encryption.clone(),
            genesis_hash: x.genesis_hash,
            logo: x.logo.to_string(),
            name: x.name.to_string(),
            order: x.order,
            path_id: x.path_id.to_string(),
            secondary_color: x.secondary_color.to_string(),
            title: x.title.to_string(),
            unit: x.unit.to_string(),
        };
        out.push(new);
    }
    out
}

/// Generate verifiers set for the default networks, for cold database
#[cfg(feature = "active")]
pub fn default_verifiers() -> Vec<(VerifierKey, CurrentVerifier)> {
    let mut out: Vec<(VerifierKey, CurrentVerifier)> = Vec::new();
    for x in default_network_info() {
        out.push((
            VerifierKey::from_parts(&x.genesis_hash),
            CurrentVerifier::Valid(ValidCurrentVerifier::General),
        ));
    }
    out
}

/// Generate network specs [`NetworkSpecsToSend`] set for the default networks,
/// for hot database
#[cfg(feature = "active")]
pub fn default_chainspecs_to_send() -> Vec<NetworkSpecsToSend> {
    let mut out: Vec<NetworkSpecsToSend> = Vec::new();
    for x in default_network_info() {
        let new = NetworkSpecsToSend {
            base58prefix: x.base58prefix,
            color: x.color.to_string(),
            decimals: x.decimals,
            encryption: x.encryption.clone(),
            genesis_hash: x.genesis_hash,
            logo: x.logo.to_string(),
            name: x.name.to_string(),
            path_id: x.path_id.to_string(),
            secondary_color: x.secondary_color.to_string(),
            title: x.title.to_string(),
            unit: x.unit.to_string(),
        };
        out.push(new);
    }
    out
}

/// Generate address book set for the default networks, for hot database
#[cfg(feature = "active")]
pub fn default_address_book() -> Vec<AddressBookEntry> {
    let mut out: Vec<AddressBookEntry> = Vec::new();
    for x in default_network_info() {
        let new = AddressBookEntry {
            name: x.name.to_string(),
            genesis_hash: x.genesis_hash,
            address: x.address.to_string(),
            encryption: x.encryption.clone(),
            def: true,
        };
        out.push(new);
    }
    out
}

/// Read metadata files from given directory. Is used to populate different
/// variants of cold database (release and tests)
#[cfg(feature = "active")]
fn metadata(dir: &str) -> Result<Vec<MetaValues>, ErrorActive> {
    let mut out: Vec<MetaValues> = Vec::new();
    let path_set = match std::fs::read_dir(dir) {
        Ok(a) => a,
        Err(e) => {
            return Err(ErrorActive::DefaultLoading(DefaultLoading::MetadataFolder(
                e,
            )))
        }
    };
    for x in path_set.flatten() {
        if let Some(filename) = x.path().to_str() {
            let meta_str = match std::fs::read_to_string(x.path()) {
                Ok(a) => a,
                Err(e) => return Err(ErrorActive::DefaultLoading(DefaultLoading::MetadataFile(e))),
            };
            let new = MetaValues::from_str_metadata(
                meta_str.trim(),
                IncomingMetadataSourceActiveStr::Default {
                    filename: filename.to_string(),
                },
            )?;
            let mut found = false;
            for a in default_network_info() {
                if new.name == a.name {
                    found = true;
                    if let Some(prefix_from_meta) = new.optional_base58prefix {
                        if prefix_from_meta != a.base58prefix {
                            return Err(<Active>::faulty_metadata(
                                MetadataError::Base58PrefixSpecsMismatch {
                                    specs: a.base58prefix,
                                    meta: prefix_from_meta,
                                },
                                MetadataSource::Incoming(IncomingMetadataSourceActive::Str(
                                    IncomingMetadataSourceActiveStr::Default {
                                        filename: filename.to_string(),
                                    },
                                )),
                            ));
                        }
                    }
                    break;
                }
            }
            if !found {
                return Err(ErrorActive::DefaultLoading(
                    DefaultLoading::OrphanMetadata {
                        name: new.name,
                        filename: filename.to_string(),
                    },
                ));
            }
            out.push(new)
        }
    }
    Ok(out)
}

/// Read metadata set for test cold database from `test_metadata` folder
#[cfg(feature = "active")]
pub fn test_metadata() -> Result<Vec<MetaValues>, ErrorActive> {
    metadata("../defaults/test_metadata")
}

/// Read metadata set for navigation test cold database from `nav_test_metadata`
/// folder
#[cfg(feature = "active")]
pub fn nav_test_metadata() -> Result<Vec<MetaValues>, ErrorActive> {
    metadata("../defaults/nav_test_metadata")
}

/// Read metadata set for release cold database from `release_metadata` folder
#[cfg(feature = "active")]
pub fn release_metadata() -> Result<Vec<MetaValues>, ErrorActive> {
    metadata("../defaults/release_metadata")
}

#[cfg(feature = "active")]
lazy_static! {
    static ref REG_STRUCTS_WITH_NAMES: Regex = Regex::new(r#"(pub )?struct (?P<name>.*?)( )?\{(?P<description>(\n +(pub )?\w+: .*(,)?)*\n)\}"#).expect("checked construction");
    static ref REG_STRUCTS_NO_NAMES: Regex = Regex::new(r#"(pub )?struct (?P<name>.*?)( )?\((pub )?(?P<description>.*)\)"#).expect("checked construction");
    static ref REG_STRUCT_FIELDS: Regex = Regex::new(r#"(?m)^ +(pub )?(?P<field_name>[\w]+): (?P<field_type>[^ ,]*\(.*\)[^ ,]*|[^ ,]*<.*>[^ ,]*|[^ ,]*\[.*\][^ ,]*|[^ ,]+)(,)?$"#).expect("checked construction");
    static ref REG_ENUM: Regex = Regex::new(r#"(pub )?enum (?P<name>.*?)( )?\{(?P<description>(\n +\w+( )?(\(.*\))?(\{[^}]*\})?(,)?)*)\n\}"#).expect("checked construction");
    static ref REG_ENUM_VARIANTS: Regex = Regex::new(r#"(?m)^ +(?P<variant_name>\w+)(( )?(?P<variant_type>(\(.*\))|(\{[^}]*\})))?(,)?$"#).expect("checked construction");
    static ref REG_ENUM_SIMPLE: Regex = Regex::new(r#"(?m)^(?P<simple_type>([^, ]*\[.*\][^, ]*)|([^, ]*<.*>[^, ]*)|([^, ]*\([^)]*\)[^, ]*)|([^, ]+))$"#).expect("checked construction");
    static ref REG_ENUM_TUPLE: Regex = Regex::new(r#"(?P<type_in_tuple>([^, ]*\[.*\][^, ]*)|([^, ]*<.*>[^, ]*)|([^, ]*\([^)]*\)[^, ]*)|([^, ]+))(, )?"#).expect("checked construction");
    static ref REG_ENUM_STRUCT: Regex = Regex::new(r#"(\n)?( +)?(?P<struct_field_name>\w+): (?P<struct_field_type>)(?P<fieldtype>[^ ,]*\(.*\)[^ ,]*|[^ ,]*<.*>[^ ,]*|[^ ,]*\[.*\][^ ,]*|[^ ,]+)(,)?"#).expect("checked construction");
    static ref REG_TYPES: Regex = Regex::new(r#"(?m)(pub )?type (?P<name>.*) = (?P<description>.*);$"#).expect("checked construction");
}

/// Generate default types as [`TypeEntry`] set
///
/// Type definitions stored in `../defaults/default_types/full_types_information`
/// are recorded with standard Rust syntax, processed with regular expressions
/// to categorize, and collected as `TypeEntry` set.
///
/// Types information is necessary only to parse transactions produced with
/// `RuntimeVersion` of the network metadata below V14. Therefore, it is
/// obsolete for default networks.
///
/// Types information currently on file was collected for older metadata
/// versions of Westend, Polkadot, Kusama, and Rococo, when they were still
/// using metadata runtime version V12 and V13. Type definitions were gathered
/// (mostly) from substrate crates and in some cases from js client code, when
/// it was not possible to find explicit descriptions in substrate crates.
///
/// This types collection is totally not expected to be exhaustive for any
/// network a user may encounter, although the most common types are likely
/// described. If user gets an error that certain type is unknown, this type
/// description could be added into types file by user, and types update could
/// be generated by user.
#[cfg(feature = "active")]
pub fn default_types_vec() -> Result<Vec<TypeEntry>, ErrorActive> {
    let filename = "../defaults/default_types/full_types_information";
    let type_info = match fs::read_to_string(filename) {
        Ok(x) => x,
        Err(e) => return Err(ErrorActive::DefaultLoading(DefaultLoading::TypesFile(e))),
    };

    let mut types_prep: Vec<TypeEntry> = Vec::new();

    for caps1 in REG_STRUCTS_WITH_NAMES.captures_iter(&type_info) {
        let struct_name = (&caps1["name"]).to_string();
        let struct_description = (&caps1["description"]).to_string();
        let mut struct_fields: Vec<StructField> = Vec::new();
        for caps2 in REG_STRUCT_FIELDS.captures_iter(&struct_description) {
            let new = StructField {
                field_name: Some((&caps2["field_name"]).to_string()),
                field_type: (&caps2["field_type"]).to_string(),
            };
            struct_fields.push(new);
        }
        let new_entry = TypeEntry {
            name: struct_name,
            description: Description::Struct(struct_fields),
        };
        types_prep.push(new_entry);
    }
    for caps in REG_STRUCTS_NO_NAMES.captures_iter(&type_info) {
        let only_field = StructField {
            field_name: None,
            field_type: (&caps["description"]).to_string(),
        };
        let new_entry = TypeEntry {
            name: (&caps["name"]).to_string(),
            description: Description::Struct(vec![only_field]),
        };
        types_prep.push(new_entry);
    }
    for caps1 in REG_ENUM.captures_iter(&type_info) {
        let enum_name = (&caps1["name"]).to_string();
        let enum_description = (&caps1["description"]).to_string();
        let enum_variants = enum_description
            .lines()
            .filter(|line| REG_ENUM_VARIANTS.is_match(line))
            .map(|line| {
                let caps2 = REG_ENUM_VARIANTS
                    .captures(line)
                    .expect("just checked it is match");
                let variant_name = (&caps2["variant_name"]).to_string();
                let variant_type = match caps2.name("variant_type") {
                    None => EnumVariantType::None,
                    Some(a) => {
                        let x = a.as_str().to_string();
                        if x.starts_with('(') {
                            // either a single type or a tuple
                            match REG_ENUM_SIMPLE.captures(&x[1..x.len() - 1]) {
                                // single type
                                Some(b) => EnumVariantType::Type((&b["simple_type"]).to_string()),
                                // tuple
                                None => EnumVariantType::Type(x),
                            }
                        } else {
                            // struct with struct-like fields
                            let mut type_is_struct: Vec<StructField> = Vec::new();
                            for caps3 in REG_ENUM_STRUCT.captures_iter(&x) {
                                let new = StructField {
                                    field_name: Some((&caps3["field_name"]).to_string()),
                                    field_type: (&caps3["field_type"]).to_string(),
                                };
                                type_is_struct.push(new);
                            }
                            EnumVariantType::Struct(type_is_struct)
                        }
                    }
                };
                EnumVariant {
                    variant_name,
                    variant_type,
                }
            })
            .collect();
        let new_entry = TypeEntry {
            name: enum_name,
            description: Description::Enum(enum_variants),
        };
        types_prep.push(new_entry);
    }
    for caps in REG_TYPES.captures_iter(&type_info) {
        let new_entry = TypeEntry {
            name: (&caps["name"]).to_string(),
            description: Description::Type((&caps["description"]).to_string()),
        };
        types_prep.push(new_entry);
    }

    Ok(types_prep)
}

/// Generate default types as [`ContentLoadTypes`]
#[cfg(feature = "active")]
pub fn default_types_content() -> Result<ContentLoadTypes, ErrorActive> {
    Ok(ContentLoadTypes::generate(&default_types_vec()?))
}

#[cfg(feature = "test")]
#[cfg(test)]
mod tests {
    use super::*;

    /// Check that can generate test metadata set
    #[test]
    fn can_get_test_metadata() {
        assert!(test_metadata().is_ok());
    }

    /// Check that can generate test metadata set for `navigator` crate testing
    #[test]
    fn can_get_nav_test_metadata() {
        assert!(nav_test_metadata().is_ok());
    }

    /// Check that can generate release metadata set
    #[test]
    fn can_get_release_metadata() {
        assert!(release_metadata().is_ok());
    }

    /// Check that can generate types information set
    #[test]
    fn can_get_default_types_vec() {
        assert!(default_types_vec().is_ok());
    }
}
