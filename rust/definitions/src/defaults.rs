use hex;
use std::convert::TryInto;
use std::fs;
use regex::Regex;
use lazy_static::lazy_static;


use super::network_specs::{ChainSpecs, ChainSpecsToSend, Verifier};
use super::types::{TypeEntry, Description, EnumVariant, EnumVariantType, StructField};
use super::metadata::{AddressBookEntry};

pub fn get_default_chainspecs() -> Vec<ChainSpecs> {
    vec![
        ChainSpecs {
            base58prefix: 2,
            color: String::from("#000"),
            decimals: 12,
            genesis_hash: hex::decode("b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe").expect("known value").try_into().expect("known value"),
            logo: String::from("kusama"),
            name: String::from("kusama"),
            order: 1,
            path_id: String::from("//kusama"),
            secondary_color: String::from("#262626"),
            title: String::from("Kusama"),
            unit: String::from("KSM"),
            verifier: Verifier::None,
    	},
	ChainSpecs {
            base58prefix: 0,
            color: String::from("#E6027A"),
            decimals: 10,
            genesis_hash: hex::decode("91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3").expect("known value").try_into().expect("known value"),
            logo: String::from("polkadot"),
            name: String::from("polkadot"),
            order: 0,
            path_id: String::from("//polkadot"),
            secondary_color: String::from("#262626"),
            title: String::from("Polkadot"),
            unit: String::from("DOT"),
            verifier: Verifier::None,
    	},
	ChainSpecs {
            base58prefix: 42,
            color: String::from("#6f36dc"),
            decimals: 12,
            genesis_hash: hex::decode("e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779").expect("known value").try_into().expect("known value"),
            logo: String::from("rococo"),
            name: String::from("rococo"),
            order: 3,
            path_id: String::from("//rococo"),
            secondary_color: String::from("#262626"),
            title: String::from("Rococo"),
            unit: String::from("ROC"),
            verifier: Verifier::None,
    	},
        ChainSpecs {
            base58prefix: 42,
            color: String::from("#660D35"),
            decimals: 12,
            genesis_hash: hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").expect("known value").try_into().expect("known value"),
            logo: String::from("westend"),
            name: String::from("westend"),
            order: 2,
            path_id: String::from("//westend"),
            secondary_color: String::from("#262626"),
            title: String::from("Westend"),
            unit: String::from("WND"),
            verifier: Verifier::None,
        },
    ]
}


pub fn get_default_chainspecs_to_send() -> Vec<ChainSpecsToSend> {
    let specs = get_default_chainspecs();
    let mut specs_to_send: Vec<ChainSpecsToSend> = Vec::new();
    
    for x in specs.iter() {
        let new = ChainSpecsToSend {
            base58prefix: x.base58prefix,
            color: x.color.to_string(),
            decimals: x.decimals,
            genesis_hash: x.genesis_hash,
            logo: x.logo.to_string(),
            name: x.name.to_string(),
            path_id: x.path_id.to_string(),
            secondary_color: x.secondary_color.to_string(),
            title: x.title.to_string(),
            unit: x.unit.to_string(),
        };
        specs_to_send.push(new);
    }
    specs_to_send
}


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

pub fn get_default_types() -> Result<Vec<TypeEntry>, String> {
    
    let filename = "../definitions/data/full_types_information";
    let type_info = match fs::read_to_string(filename) {
        Ok(x) => x,
        Err(e) => return Err(format!("Error reading datafile with default types information. {}", e)),
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
            field_type: (&caps["description"]).to_string()
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
                let caps2 = REG_ENUM_VARIANTS.captures(&line).unwrap();
                let variant_name = caps2.name("variant_name").unwrap().as_str().to_string();
                let variant_type = match caps2.name("variant_type") {
                    None => EnumVariantType::None,
                    Some(a) => {
                        let x = a.as_str().to_string();
                        if x.starts_with("(") {
                            // either a single type or a tuple
                            match REG_ENUM_SIMPLE.captures(&x[1..x.len()-1]){
                                // single type
                                Some(b) => EnumVariantType::Type(b.name("simple_type").unwrap().as_str().to_string()),
                                // tuple
                                None => EnumVariantType::Type(x),
                            }
                        }
                        else {
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
                    },
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


pub fn get_default_address_book() -> Vec<AddressBookEntry> {
    vec![
        AddressBookEntry {
            name: String::from("kusama"),
            genesis_hash: hex::decode("b0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe").expect("known value").try_into().expect("known value"),
            address: String::from("wss://kusama-rpc.polkadot.io"),
        },
        AddressBookEntry {
            name: String::from("polkadot"),
            genesis_hash: hex::decode("91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3").expect("known value").try_into().expect("known value"),
            address: String::from("wss://rpc.polkadot.io"),
        },
        AddressBookEntry {
            name: String::from("rococo"),
            genesis_hash: hex::decode("e7c3d5edde7db964317cd9b51a3a059d7cd99f81bdbce14990047354334c9779").expect("known value").try_into().expect("known value"),
            address: String::from("wss://rococo-rpc.polkadot.io"),
        },
        AddressBookEntry {
            name: String::from("westend"),
            genesis_hash: hex::decode("e143f23803ac50e8f6f8e62695d1ce9e4e1d68aa36c1cd2cfd15340213f3423e").expect("known value").try_into().expect("known value"),
            address: String::from("wss://westend-rpc.polkadot.io"),
        },
    ]
}
