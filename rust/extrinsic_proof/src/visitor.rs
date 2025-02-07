// The code is adopted from https://github.com/Zondax/merkleized-metadata/blob/main/src/extrinsic_decoder.rs

use core::sync::atomic::AtomicUsize;

use alloc::{
	collections::{BTreeMap, BTreeSet},
	format,
	string::String,
	vec::Vec,
};
use codec::{Compact, Decode, Input};
use scale_decode::{
	ext::scale_type_resolver::{
		BitsOrderFormat, BitsStoreFormat, Primitive as RPrimitive, ResolvedTypeVisitor, Variant,
	},
	visitor::{DecodeError},
	Field, Visitor,
};

use crate::{
  types::{Type, TypeDef, TypeRef, TypeId}
};

use parser::decoding_commons::OutputCard;


/// Decoding happens recursively and we need some upper bound to stop somewhere
/// to prevent a stack overflow.
const MAX_STACK_DEPTH: usize = 1000;

pub struct TypeResolver {
	raw_type_id_to_types: BTreeMap<u32, Vec<Type>>,
	stack_depth: AtomicUsize,
}

impl TypeResolver {
	pub fn new<'a>(types: impl Iterator<Item = &'a Type>) -> Self {
		Self {
			raw_type_id_to_types: types.fold(Default::default(), |mut map, ty| {
				map.entry(ty.type_id.0).or_default().push(ty.clone());
				map
			}),
			stack_depth: Default::default(),
		}
	}
}

impl scale_decode::TypeResolver for TypeResolver {
	type TypeId = TypeRef;

	type Error = String;

	fn resolve_type<'this, V: ResolvedTypeVisitor<'this, TypeId = TypeRef>>(
		&'this self,
		type_id: TypeRef,
		visitor: V,
	) -> Result<V::Value, Self::Error> {
		let type_id = match type_id {
			TypeRef::ById(id) => id.0,
			TypeRef::Bool => return Ok(visitor.visit_primitive(RPrimitive::Bool)),
			TypeRef::Char => return Ok(visitor.visit_primitive(RPrimitive::Char)),
			TypeRef::Str => return Ok(visitor.visit_primitive(RPrimitive::Str)),
			TypeRef::U8 => return Ok(visitor.visit_primitive(RPrimitive::U8)),
			TypeRef::U16 => return Ok(visitor.visit_primitive(RPrimitive::U16)),
			TypeRef::U32 => return Ok(visitor.visit_primitive(RPrimitive::U32)),
			TypeRef::U64 => return Ok(visitor.visit_primitive(RPrimitive::U64)),
			TypeRef::U128 => return Ok(visitor.visit_primitive(RPrimitive::U128)),
			TypeRef::U256 => return Ok(visitor.visit_primitive(RPrimitive::U256)),
			TypeRef::I8 => return Ok(visitor.visit_primitive(RPrimitive::I8)),
			TypeRef::I16 => return Ok(visitor.visit_primitive(RPrimitive::I16)),
			TypeRef::I32 => return Ok(visitor.visit_primitive(RPrimitive::I32)),
			TypeRef::I64 => return Ok(visitor.visit_primitive(RPrimitive::I64)),
			TypeRef::I128 => return Ok(visitor.visit_primitive(RPrimitive::I128)),
			TypeRef::I256 => return Ok(visitor.visit_primitive(RPrimitive::I256)),
			TypeRef::CompactU8 => return Ok(visitor.visit_compact(TypeRef::U8)),
			TypeRef::CompactU16 => return Ok(visitor.visit_compact(TypeRef::U16)),
			TypeRef::CompactU32 => return Ok(visitor.visit_compact(TypeRef::U32)),
			TypeRef::CompactU64 => return Ok(visitor.visit_compact(TypeRef::U64)),
			TypeRef::CompactU128 => return Ok(visitor.visit_compact(TypeRef::U128)),
			TypeRef::CompactU256 => return Err("CompactU256 not supported right now".into()),
			TypeRef::Void =>
				return Ok(visitor.visit_composite(core::iter::empty(), core::iter::empty())),
		};

		if self.stack_depth.fetch_add(1, core::sync::atomic::Ordering::Relaxed) >= MAX_STACK_DEPTH {
			return Err("Reached stack limit".into());
		}

		let types = self
			.raw_type_id_to_types
			.get(&type_id)
			.ok_or_else(|| format!("Unknown type id {type_id}"))?;

		if types.is_empty() {
			return Err(format!("{type_id} type is empty"));
		}

		let type_def = &types[0].type_def;
		let value = match type_def {
			TypeDef::Array(a) => visitor.visit_array(a.type_param, a.len as usize),
			TypeDef::Composite(c) => visitor.visit_composite(
				core::iter::empty(),
				c.iter().map(|f| Field { name: f.name.as_deref(), id: f.ty }),
			),
			TypeDef::Enumeration(_) => {
				if types.iter().any(|t| t.type_def.as_enumeration().is_none()) {
					return Err(format!("All type defs of {type_id} must be an `Enumeration`"));
				}

				visitor.visit_variant(
					core::iter::empty(),
					types.iter().map(|t| {
						let TypeDef::Enumeration(v) = &t.type_def else {
							panic!("Checked above to be an `Enumeration`; qed")
						};

						Variant {
							index: v.index.0 as _,
							name: &v.name,
							fields: v
								.fields
								.iter()
								.map(|f| Field { name: f.name.as_deref(), id: f.ty }),
						}
					}),
				)
			},
			TypeDef::Sequence(s) => visitor.visit_sequence(core::iter::empty(), *s),
			TypeDef::Tuple(t) => visitor.visit_tuple(t.iter().copied()),
			TypeDef::BitSequence(b) => {
				let store_format = match b.num_bytes {
					1 => BitsStoreFormat::U8,
					2 => BitsStoreFormat::U16,
					4 => BitsStoreFormat::U32,
					8 => BitsStoreFormat::U64,
					b => return Err(format!("Unsupported number of bytes {b} for type {type_id}")),
				};

				let bit_order = if b.least_significant_bit_first {
					BitsOrderFormat::Lsb0
				} else {
					BitsOrderFormat::Msb0
				};

				visitor.visit_bit_sequence(store_format, bit_order)
			},
		};

		self.stack_depth.fetch_sub(1, core::sync::atomic::Ordering::Relaxed);

		Ok(value)
	}
}

#[derive(Clone, Default)]
pub struct CollectOutputCards {
	pub cards: Vec<OutputCard>,
}

impl Visitor for CollectOutputCards {
	type TypeResolver = TypeResolver;
	type Value<'scale, 'resolver> = Self;
	type Error = DecodeError;

	fn visit_bool<'scale, 'resolver>(
		self,
		_value: bool,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_char<'scale, 'resolver>(
		self,
		_value: char,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_u8<'scale, 'resolver>(
		self,
		_value: u8,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_u16<'scale, 'resolver>(
		self,
		_value: u16,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_u32<'scale, 'resolver>(
		self,
		_value: u32,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_u64<'scale, 'resolver>(
		self,
		_value: u64,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_u128<'scale, 'resolver>(
		self,
		_value: u128,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_u256<'scale, 'resolver>(
		self,
		_value: &'scale [u8; 32],
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_i8<'scale, 'resolver>(
		self,
		_value: i8,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_i16<'scale, 'resolver>(
		self,
		_value: i16,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_i32<'scale, 'resolver>(
		self,
		_value: i32,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_i64<'scale, 'resolver>(
		self,
		_value: i64,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_i128<'scale, 'resolver>(
		self,
		_value: i128,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_i256<'scale, 'resolver>(
		self,
		_value: &'scale [u8; 32],
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_sequence<'scale, 'resolver>(
		mut self,
		value: &mut scale_decode::visitor::types::Sequence<'scale, 'resolver, Self::TypeResolver>,
		type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		self.accessed_types
			.insert(TypeId::Other(type_id.id().expect("Sequence is always referenced by id; qed")));

		let mut visitor = self;
		while let Some(field) = value.next() {
			visitor = field?.decode_with_visitor(visitor)?;
		}

		Ok(visitor)
	}

	fn visit_composite<'scale, 'resolver>(
		mut self,
		value: &mut scale_decode::visitor::types::Composite<'scale, 'resolver, Self::TypeResolver>,
		type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		// We forward `Void` as composite with no fields.
		if value.remaining() == 0 {
			return Ok(self);
		}

		self.accessed_types.insert(TypeId::Other(
			type_id.id().expect("Composite is always referenced by id; qed"),
		));

		let mut visitor = self;
		while let Some(field) = value.next() {
			visitor = field?.decode_with_visitor(visitor)?;
		}

		Ok(visitor)
	}

	fn visit_tuple<'scale, 'resolver>(
		mut self,
		value: &mut scale_decode::visitor::types::Tuple<'scale, 'resolver, Self::TypeResolver>,
		type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		self.accessed_types
			.insert(TypeId::Other(type_id.id().expect("Tuple is always referenced by id; qed")));

		let mut visitor = self;
		while let Some(field) = value.next() {
			visitor = field?.decode_with_visitor(visitor)?;
		}

		Ok(visitor)
	}

	fn visit_str<'scale, 'resolver>(
		self,
		_value: &mut scale_decode::visitor::types::Str<'scale>,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		Ok(self)
	}

	fn visit_variant<'scale, 'resolver>(
		mut self,
		value: &mut scale_decode::visitor::types::Variant<'scale, 'resolver, Self::TypeResolver>,
		type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		self.accessed_types.insert(TypeId::Enumeration {
			type_id: type_id.id().expect("Enumeration is always referenced by id; qed"),
			variant: value.index() as u32,
		});

		let mut visitor = self;
		while let Some(field) = value.fields().next() {
			visitor = field?.decode_with_visitor(visitor)?;
		}

		Ok(visitor)
	}

	fn visit_array<'scale, 'resolver>(
		mut self,
		value: &mut scale_decode::visitor::types::Array<'scale, 'resolver, Self::TypeResolver>,
		type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		self.accessed_types.insert(TypeId::Other(
			type_id.id().expect("BitSequence is always referenced by id; qed"),
		));

		let mut visitor = self;
		while let Some(field) = value.next() {
			visitor = field?.decode_with_visitor(visitor)?;
		}

		Ok(visitor)
	}

	fn visit_bitsequence<'scale, 'resolver>(
		mut self,
		_value: &mut scale_decode::visitor::types::BitSequence<'scale>,
		type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		self.accessed_types.insert(TypeId::Other(
			type_id.id().expect("BitSequence is always referenced by id; qed"),
		));
		Ok(self)
	}
}