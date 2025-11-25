use alloc::{string::String, vec::Vec};
use codec::{Compact, Encode};

/// A reference to a type in the registry.
#[derive(
    Clone,
    Default,
    PartialOrd,
    Ord,
    Debug,
    PartialEq,
    Eq,
    Encode,
    serde::Serialize,
    serde::Deserialize,
    autarkie::Grammar,
)]
pub enum FuzzTypeRef {
    #[codec(index = 0)]
    Bool,
    #[codec(index = 1)]
    Char,
    #[codec(index = 2)]
    Str,
    #[codec(index = 3)]
    U8,
    #[codec(index = 4)]
    U16,
    #[codec(index = 5)]
    U32,
    #[codec(index = 6)]
    U64,
    #[codec(index = 7)]
    U128,
    #[codec(index = 8)]
    U256,
    #[codec(index = 9)]
    I8,
    #[codec(index = 10)]
    I16,
    #[codec(index = 11)]
    I32,
    #[codec(index = 12)]
    I64,
    #[codec(index = 13)]
    I128,
    #[codec(index = 14)]
    I256,
    #[codec(index = 15)]
    CompactU8,
    #[codec(index = 16)]
    CompactU16,
    #[codec(index = 17)]
    CompactU32,
    #[codec(index = 18)]
    CompactU64,
    #[codec(index = 19)]
    CompactU128,
    #[codec(index = 20)]
    CompactU256,
    #[codec(index = 21)]
    #[default]
    Void,
    #[codec(index = 22)]
    ById(u32),
}

/// A reference to a type in the registry.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode, Default, Copy)]
pub enum TypeRef {
    #[codec(index = 0)]
    Bool,
    #[codec(index = 1)]
    Char,
    #[codec(index = 2)]
    Str,
    #[codec(index = 3)]
    U8,
    #[codec(index = 4)]
    U16,
    #[codec(index = 5)]
    U32,
    #[codec(index = 6)]
    U64,
    #[codec(index = 7)]
    U128,
    #[codec(index = 8)]
    U256,
    #[codec(index = 9)]
    I8,
    #[codec(index = 10)]
    I16,
    #[codec(index = 11)]
    I32,
    #[codec(index = 12)]
    I64,
    #[codec(index = 13)]
    I128,
    #[codec(index = 14)]
    I256,
    #[codec(index = 15)]
    CompactU8,
    #[codec(index = 16)]
    CompactU16,
    #[codec(index = 17)]
    CompactU32,
    #[codec(index = 18)]
    CompactU64,
    #[codec(index = 19)]
    CompactU128,
    #[codec(index = 20)]
    CompactU256,
    #[codec(index = 21)]
    #[default]
    Void,
    #[codec(index = 22)]
    ById(Compact<u32>),
}

impl TypeRef {
    pub fn id(&self) -> Option<u32> {
        if let Self::ById(id) = self {
            Some(id.0)
        } else {
            None
        }
    }
}

/// The hash type.
pub type Hash = [u8; 32];

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode)]
pub enum TypeDef {
    /// A composite type (e.g. a struct or a tuple)
    #[codec(index = 0)]
    Composite(Vec<Field>),
    /// An enumeration.
    #[codec(index = 1)]
    Enumeration(EnumerationVariant),
    /// A sequence type with runtime known length.
    #[codec(index = 2)]
    Sequence(TypeRef),
    /// An array type with compile-time known length.
    #[codec(index = 3)]
    Array(TypeDefArray),
    /// A tuple type.
    #[codec(index = 4)]
    Tuple(Vec<TypeRef>),
    /// A type representing a sequence of bits.
    #[codec(index = 5)]
    BitSequence(TypeDefBitSequence),
}

#[derive(
    Clone,
    PartialOrd,
    Debug,
    PartialEq,
    Eq,
    Encode,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    autarkie::Grammar,
)]
pub enum FuzzTypeDef {
    /// A composite type (e.g. a struct or a tuple)
    #[codec(index = 0)]
    Composite(Vec<FuzzField>),
    /// An enumeration.
    #[codec(index = 1)]
    Enumeration(FuzzEnumerationVariant),
    /// A sequence type with runtime known length.
    #[codec(index = 2)]
    Sequence(FuzzTypeRef),
    /// An array type with compile-time known length.
    #[codec(index = 3)]
    Array(FuzzTypeDefArray),
    //// A tuple type.
    #[codec(index = 4)]
    Tuple(Vec<FuzzTypeRef>),
    /// A type representing a sequence of bits.
    #[codec(index = 5)]
    BitSequence(TypeDefBitSequence),
}

impl TypeDef {
    /// Returns `self` as [`EnumerationVariant`] or `None` if this isn't an `Enumeration`.
    pub fn as_enumeration(&self) -> Option<&EnumerationVariant> {
        if let Self::Enumeration(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode)]
pub struct Field {
    pub name: Option<String>,
    pub ty: TypeRef,
    pub type_name: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Encode,
    serde::Serialize,
    serde::Deserialize,
    autarkie::Grammar,
)]
pub struct FuzzField {
    pub name: Option<String>,
    pub ty: FuzzTypeRef,
    pub type_name: Option<String>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode)]
pub struct EnumerationVariant {
    pub name: String,
    pub fields: Vec<Field>,
    pub index: Compact<u32>,
}

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Debug,
    Encode,
    serde::Serialize,
    serde::Deserialize,
    autarkie::Grammar,
)]
pub struct FuzzEnumerationVariant {
    pub name: String,
    pub fields: Vec<FuzzField>,
    pub index: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode)]
pub struct TypeDefArray {
    pub len: u32,
    pub type_param: TypeRef,
}

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Debug,
    Encode,
    autarkie::Grammar,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct FuzzTypeDefArray {
    pub len: u32,
    pub type_param: FuzzTypeRef,
}

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Encode,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    autarkie::Grammar,
)]
pub struct TypeDefBitSequence {
    pub num_bytes: u8,
    pub least_significant_bit_first: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode)]
pub struct Type {
    /// The unique path to the type. Can be empty for built-in types
    pub path: Vec<String>,
    /// The actual type definition
    pub type_def: TypeDef,
    /// The unique id of this type.
    pub type_id: Compact<u32>,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Encode, serde::Serialize, serde::Deserialize, autarkie::Grammar,
)]
pub struct FuzzType {
    /// The unique path to the type. Can be empty for built-in types
    pub path: Vec<String>,
    /// The actual type definition
    pub type_def: FuzzTypeDef,
    /// The unique id of this type.
    pub type_id: u32,
}

impl Type {
    /// Returns the hash of this type.
    pub fn hash(&self) -> Hash {
        blake3::hash(&self.encode()).into()
    }
}

impl From<crate::types::FuzzTypeRef> for crate::types::TypeRef {
    fn from(fuzz_type_ref: crate::types::FuzzTypeRef) -> Self {
        use crate::types::{FuzzTypeRef, TypeRef};
        use codec::Compact;

        match fuzz_type_ref {
            FuzzTypeRef::Bool => TypeRef::Bool,
            FuzzTypeRef::Char => TypeRef::Char,
            FuzzTypeRef::Str => TypeRef::Str,
            FuzzTypeRef::U8 => TypeRef::U8,
            FuzzTypeRef::U16 => TypeRef::U16,
            FuzzTypeRef::U32 => TypeRef::U32,
            FuzzTypeRef::U64 => TypeRef::U64,
            FuzzTypeRef::U128 => TypeRef::U128,
            FuzzTypeRef::U256 => TypeRef::U256,
            FuzzTypeRef::I8 => TypeRef::I8,
            FuzzTypeRef::I16 => TypeRef::I16,
            FuzzTypeRef::I32 => TypeRef::I32,
            FuzzTypeRef::I64 => TypeRef::I64,
            FuzzTypeRef::I128 => TypeRef::I128,
            FuzzTypeRef::I256 => TypeRef::I256,
            FuzzTypeRef::CompactU8 => TypeRef::CompactU8,
            FuzzTypeRef::CompactU16 => TypeRef::CompactU16,
            FuzzTypeRef::CompactU32 => TypeRef::CompactU32,
            FuzzTypeRef::CompactU64 => TypeRef::CompactU64,
            FuzzTypeRef::CompactU128 => TypeRef::CompactU128,
            FuzzTypeRef::CompactU256 => TypeRef::CompactU256,
            FuzzTypeRef::Void => TypeRef::Void,
            FuzzTypeRef::ById(id) => TypeRef::ById(Compact(id)),
        }
    }
}

impl From<crate::types::FuzzField> for crate::types::Field {
    fn from(fuzz_field: crate::types::FuzzField) -> Self {
        crate::types::Field {
            name: fuzz_field.name,
            ty: fuzz_field.ty.into(),
            type_name: fuzz_field.type_name,
        }
    }
}

impl From<crate::types::FuzzEnumerationVariant> for crate::types::EnumerationVariant {
    fn from(fuzz_variant: crate::types::FuzzEnumerationVariant) -> Self {
        use codec::Compact;

        crate::types::EnumerationVariant {
            name: fuzz_variant.name,
            fields: fuzz_variant.fields.into_iter().map(|f| f.into()).collect(),
            index: Compact(fuzz_variant.index),
        }
    }
}

impl From<crate::types::FuzzTypeDefArray> for crate::types::TypeDefArray {
    fn from(fuzz_array: crate::types::FuzzTypeDefArray) -> Self {
        crate::types::TypeDefArray {
            len: fuzz_array.len,
            type_param: fuzz_array.type_param.into(),
        }
    }
}

impl From<crate::types::FuzzTypeDef> for crate::types::TypeDef {
    fn from(fuzz_type_def: crate::types::FuzzTypeDef) -> Self {
        use crate::types::{FuzzTypeDef, TypeDef};

        match fuzz_type_def {
            FuzzTypeDef::Composite(fields) => {
                TypeDef::Composite(fields.into_iter().map(|f| f.into()).collect())
            }
            FuzzTypeDef::Enumeration(variant) => TypeDef::Enumeration(variant.into()),
            FuzzTypeDef::Sequence(type_ref) => TypeDef::Sequence(type_ref.into()),
            FuzzTypeDef::Array(array) => TypeDef::Array(array.into()),
            FuzzTypeDef::Tuple(type_refs) => {
                TypeDef::Tuple(type_refs.into_iter().map(|tr| tr.into()).collect())
            }
            FuzzTypeDef::BitSequence(bit_seq) => TypeDef::BitSequence(bit_seq),
        }
    }
}

impl From<crate::types::FuzzType> for crate::types::Type {
    fn from(fuzz_type: crate::types::FuzzType) -> Self {
        use codec::Compact;

        crate::types::Type {
            path: fuzz_type.path,
            type_def: fuzz_type.type_def.into(),
            type_id: Compact(fuzz_type.type_id),
        }
    }
}

impl From<crate::types::FuzzSignedExtensionMetadata> for crate::types::SignedExtensionMetadata {
    fn from(fuzz_ext: crate::types::FuzzSignedExtensionMetadata) -> Self {
        crate::types::SignedExtensionMetadata {
            identifier: fuzz_ext.identifier,
            included_in_extrinsic: fuzz_ext.included_in_extrinsic.into(),
            included_in_signed_data: fuzz_ext.included_in_signed_data.into(),
        }
    }
}

impl From<crate::merkle_tree::FuzzProof> for crate::merkle_tree::Proof {
    fn from(fuzz_proof: crate::merkle_tree::FuzzProof) -> Self {
        crate::merkle_tree::Proof {
            leaves: fuzz_proof
                .leaves
                .into_iter()
                .map(|leaf| leaf.into())
                .collect(),
            leaf_indices: fuzz_proof.leaf_indices,
            nodes: fuzz_proof.nodes,
        }
    }
}

impl From<crate::types::FuzzExtrinsicMetadata> for crate::types::ExtrinsicMetadata {
    fn from(fuzz_extrinsic: crate::types::FuzzExtrinsicMetadata) -> Self {
        crate::types::ExtrinsicMetadata {
            version: fuzz_extrinsic.version,
            address_ty: fuzz_extrinsic.address_ty.into(),
            call_ty: fuzz_extrinsic.call_ty.into(),
            signature_ty: fuzz_extrinsic.signature_ty.into(),
            signed_extensions: fuzz_extrinsic
                .signed_extensions
                .into_iter()
                .map(|ext| ext.into())
                .collect(),
        }
    }
}
#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Debug,
    Encode,
    serde::Serialize,
    serde::Deserialize,
    autarkie::Grammar,
)]
pub struct FuzzExtrinsicMetadata {
    /// Extrinsic version.
    pub version: u8,
    pub address_ty: FuzzTypeRef,
    pub call_ty: FuzzTypeRef,
    pub signature_ty: FuzzTypeRef,
    /// The signed extensions in the order they appear in the extrinsic.
    pub signed_extensions: Vec<FuzzSignedExtensionMetadata>,
}

#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Debug,
    Encode,
    serde::Serialize,
    serde::Deserialize,
    autarkie::Grammar,
)]
pub struct FuzzSignedExtensionMetadata {
    pub identifier: String,
    pub included_in_extrinsic: FuzzTypeRef,
    pub included_in_signed_data: FuzzTypeRef,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode)]
pub struct ExtrinsicMetadata {
    /// Extrinsic version.
    pub version: u8,
    pub address_ty: TypeRef,
    pub call_ty: TypeRef,
    pub signature_ty: TypeRef,
    /// The signed extensions in the order they appear in the extrinsic.
    pub signed_extensions: Vec<SignedExtensionMetadata>,
}

impl ExtrinsicMetadata {
    pub fn hash(&self) -> Hash {
        blake3::hash(&self.encode()).into()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode)]
pub struct SignedExtensionMetadata {
    pub identifier: String,
    pub included_in_extrinsic: TypeRef,
    pub included_in_signed_data: TypeRef,
}

/// The metadata digest.
///
/// The hash of this digest is the "metadata hash".
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode)]
pub enum MetadataDigest {
    Disabled,
    V1 {
        types_tree_root: Hash,
        extrinsic_metadata_hash: Hash,
        spec_version: u32,
        spec_name: String,
        base58_prefix: u16,
        decimals: u8,
        token_symbol: String,
    },
}

impl MetadataDigest {
    /// Returns the hash of this digest.
    pub fn hash(&self) -> Hash {
        blake3::hash(&self.encode()).into()
    }
}
