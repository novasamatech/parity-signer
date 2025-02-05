extern crate core;
extern crate codec;
extern crate hex;

use codec::{Compact, Encode, Decode};

#[cfg(test)]
mod tests;

/// A reference to a type in the registry.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode, Decode, Default, Copy)]
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode, Decode)]
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode, Decode)]
pub struct Field {
	pub name: Option<String>,
	pub ty: TypeRef,
	pub type_name: Option<String>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode, Decode)]
pub struct EnumerationVariant {
	pub name: String,
	pub fields: Vec<Field>,
	pub index: Compact<u32>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode, Decode)]
pub struct TypeDefArray {
	pub len: u32,
	pub type_param: TypeRef,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Decode, Encode, Debug)]
pub struct TypeDefBitSequence {
	pub num_bytes: u8,
	pub least_significant_bit_first: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode, Decode)]
pub struct Type {
	/// The unique path to the type. Can be empty for built-in types
	pub path: Vec<String>,
	/// The actual type definition
	pub type_def: TypeDef,
	/// The unique id of this type.
	pub type_id: Compact<u32>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode, Decode)]
pub struct ExtrinsicMetadata {
	/// Extrinsic version.
	pub version: u8,
	pub address_ty: TypeRef,
	pub call_ty: TypeRef,
	pub signature_ty: TypeRef,
	/// The signed extensions in the order they appear in the extrinsic.
	pub signed_extensions: Vec<SignedExtensionMetadata>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode, Decode)]
pub struct SignedExtensionMetadata {
	pub identifier: String,
	pub included_in_extrinsic: TypeRef,
	pub included_in_signed_data: TypeRef,
}

/// The metadata digest.
///
/// The hash of this digest is the "metadata hash".
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Encode, Decode)]
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

#[derive(Clone, Debug, PartialEq, Eq, Encode, Decode)]
pub struct Proof {
	/// The leaves of the tree.
	///
	/// They are sorted that the left most leaves are first.
	pub leaves: Vec<Type>,
	/// The indices of the leaves in the tree, in the same order as `leaves`.
	pub leaf_indices: Vec<u32>,
	/// All the node hashes that can not be calculated out of the `leaves`.
	///
	/// These are all the nodes that are required to proof that all the `leaves` are part of the
	/// same merkle tree.
	///
	/// They are sorted from left to right, from the root to the leaf.
	pub nodes: Vec<Hash>,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct ExtraInfo {
	/// The spec version of the runtime.
	pub spec_version: u32,
	/// The spec name of the runtime.
	pub spec_name: String,
	/// The base58 prefix for addresses.
	pub base58_prefix: u16,
	/// The number of decimals of the primary token.
	pub decimals: u8,
	/// The token symbol of the primary token.
	pub token_symbol: String,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MetadataProof {
    proof: Proof,
    extrinsic: ExtrinsicMetadata,
    extra_info: ExtraInfo,
}