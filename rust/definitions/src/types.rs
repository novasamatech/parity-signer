use parity_scale_codec_derive::{Decode, Encode};

/// Struct to store type name and description
#[derive(Decode, Encode, PartialEq)]
pub struct TypeEntry {
    pub name: String,
    pub description: Description,
}

/// Type description
#[derive(Decode, Encode, PartialEq)]
pub enum Description {
    Type(String),
    Enum(Vec<EnumVariant>),
    Struct(Vec<StructField>)
}

/// Enum variants
#[derive(Decode, Encode, PartialEq)]
pub struct EnumVariant {
    pub variant_name: String,
    pub variant_type: EnumVariantType,
}

/// Types of enum variants
#[derive(Decode, Encode, PartialEq)]
pub enum EnumVariantType {
    None,
    Type(String),
    Struct(Vec<StructField>),
}

/// Struct fields (field name is optional)
#[derive(Decode, Encode, PartialEq)]
pub struct StructField {
    pub field_name: Option<String>,
    pub field_type: String,
}

