//! Types description needed to decode transactions generated with
//! [`RuntimeMetadata`](https://docs.rs/frame-metadata/15.0.0/frame_metadata/enum.RuntimeMetadata.html)
//! V12 and V13  
//!
//! Prior to V14, metadata did not contain information on *what* the type used
//! in the transaction actually is, it had only the type name.
//! For common types (i.e the ones encountered in network Westend, Polkadot,
//! Kusama, and Rococo) the type information was found elsewhere, and is a part of
//! existing Signer defaults.  
//!
//! [`TypeEntry`] is used to store the type information. Default types set contains [`TypeEntry`]
//! data for common types, and is stored in `SETTREE` tree of both hot and cold databases
//! with key `TYPES`.  
//!
//! For transaction with runtime metadata V12 or V13 to be decoded each of the
//! encountered types must have a corresponding [`TypeEntry`] or be transformable into a type
//! having [`TypeEntry`].  
//!
//! With V14 release this module became obsolete, however, since not all networks are yet on V14
//! metadata, it remains here.

use parity_scale_codec::{Decode, Encode};

/// Information each type has to have: type name and description
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct TypeEntry {
    /// type name, as it appears in the metadata
    pub name: String,

    /// type description
    pub description: Description,
}

/// Type description
///
/// Must be available for all types
#[derive(Decode, Encode, PartialEq, Clone)]
pub enum Description {
    /// type is a type alias
    Type(String),

    /// type is an enum
    Enum(Vec<EnumVariant>),

    /// type is a struct
    Struct(Vec<StructField>),
}

/// Enum variant description
///
/// Must be available for each variant of the enum.
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct EnumVariant {
    /// enum variant name, will be displayed during the parsing if the variant is encountered
    pub variant_name: String,

    /// variant type
    pub variant_type: EnumVariantType,
}

/// Types of enum variants
#[derive(Decode, Encode, PartialEq, Clone)]
pub enum EnumVariantType {
    None,
    Type(String),
    Struct(Vec<StructField>),
}

/// Struct field description
///
/// Must be available for each field if the struct.
#[derive(Decode, Encode, PartialEq, Clone)]
pub struct StructField {
    /// struct field name, if it exists, it will be displayed during the parsing if the struct is encountered
    pub field_name: Option<String>,

    /// field type
    pub field_type: String,
}
