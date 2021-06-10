use sled::{Db, Tree, open};
use parity_scale_codec::Encode;
use parity_scale_codec_derive;
use regex::Regex;
use lazy_static::lazy_static;

/// Struct to store type name and description
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct TypeEntry {
    pub name: String,
    pub description: Description,
}

/// Type description
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub enum Description {
    Type(String),
    Enum(Vec<EnumVariant>),
    Struct(Vec<StructField>)
}

/// Enum variants
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct EnumVariant {
    pub variant_name: String,
    pub variant_type: EnumVariantType,
}

/// Types of enum variants
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub enum EnumVariantType {
    None,
    Type(String),
    Struct(Vec<StructField>),
}

/// Struct fields (field name is optional)
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct StructField {
    pub field_name: Option<String>,
    pub field_type: String,
}

/// Struct to store sign_transaction action information
#[derive(parity_scale_codec_derive::Decode, parity_scale_codec_derive::Encode)]
pub struct SignDb {
    pub crypto: String,
    pub path: String,
    pub name_for_seed: String,
    pub transaction: Vec<u8>,
    pub has_pwd: bool,
    pub author_base58: String,
}

lazy_static! {
    static ref REG_STRUCTS_WITH_NAMES: Regex = Regex::new(r#"(pub )?struct (?P<name>.*?)( )?\{(?P<description>(\n +(pub )?\w+: .*(,)?)*\n)\}"#).unwrap();
    static ref REG_STRUCTS_NO_NAMES: Regex = Regex::new(r#"(pub )?struct (?P<name>.*?)( )?\((pub )?(?P<description>.*)\)"#).unwrap();
    static ref REG_STRUCT_FIELDS: Regex = Regex::new(r#"(?m)^ +(pub )?(?P<field_name>[\w]+): (?P<field_type>[^ ,]*\(.*\)[^ ,]*|[^ ,]*<.*>[^ ,]*|[^ ,]*\[.*\][^ ,]*|[^ ,]+)(,)?$"#).unwrap();
    static ref REG_ENUM: Regex = Regex::new(r#"(pub )?enum (?P<name>.*?)( )?\{(?P<description>(\n +\w+( )?(\(.*\))?(\{[^}]*\})?(,)?)*)\n\}"#).unwrap();
    static ref REG_ENUM_VARIANTS: Regex = Regex::new(r#"(?m)^ +(?P<variant_name>\w+)(( )?(?P<variant_type>(\(.*\))|(\{[^}]*\})))?(,)?$"#).unwrap();
    static ref REG_ENUM_SIMPLE: Regex = Regex::new(r#"(?m)^(?P<simple_type>([^, ]*\[.*\][^, ]*)|([^, ]*<.*>[^, ]*)|([^, ]*\([^)]*\)[^, ]*)|([^, ]+))$"#).unwrap();
    static ref REG_ENUM_TUPLE: Regex = Regex::new(r#"(?P<type_in_tuple>([^, ]*\[.*\][^, ]*)|([^, ]*<.*>[^, ]*)|([^, ]*\([^)]*\)[^, ]*)|([^, ]+))(, )?"#).unwrap();
    static ref REG_ENUM_STRUCT: Regex = Regex::new(r#"(\n)?( +)?(?P<struct_field_name>\w+): (?P<struct_field_type>)(?P<fieldtype>[^ ,]*\(.*\)[^ ,]*|[^ ,]*<.*>[^ ,]*|[^ ,]*\[.*\][^ ,]*|[^ ,]+)(,)?"#).unwrap();
    static ref REG_TYPES: Regex = Regex::new(r#"(?m)(pub )?type (?P<name>.*) = (?P<description>.*);$"#).unwrap();
}

pub fn load_types (database_name: &str, type_info: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let database: Db = open(database_name)?;
    
    let settings: Tree = database.open_tree(b"settings")?;
    
    let name = String::from("types").encode();
    let mut types_prep: Vec<TypeEntry> = Vec::new();

    for caps1 in REG_STRUCTS_WITH_NAMES.captures_iter(type_info) {
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
    for caps in REG_STRUCTS_NO_NAMES.captures_iter(type_info) {
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
    for caps1 in REG_ENUM.captures_iter(type_info) {
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
    for caps in REG_TYPES.captures_iter(type_info) {
        let new_entry = TypeEntry {
            name: (&caps["name"]).to_string(),
            description: Description::Type((&caps["description"]).to_string()),
        };
        types_prep.push(new_entry);
    }
    
    let types = types_prep.encode();
    settings.insert(name, types)?;
    
    database.flush()?;
    
    Ok(())
}
