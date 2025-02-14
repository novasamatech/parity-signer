// The code is adopted from https://github.com/Zondax/merkleized-metadata/blob/main/src/extrinsic_decoder.rs

use core::sync::atomic::AtomicUsize;
use std::path;

use alloc::{
	collections::{BTreeMap, BTreeSet},
	format,
	string::String,
	vec::Vec,
};

use bitvec::{
	prelude::BitVec,
};

use codec::{Compact, Decode, Input};
use parser::{cards, decoding_commons::OutputCard};
use parser::cards::ParserCard;
use scale_decode::{
	ext::scale_type_resolver::{
		BitsOrderFormat, BitsStoreFormat, Primitive as RPrimitive, ResolvedTypeVisitor, Variant,
	},
	visitor::{DecodeError},
	Field, Visitor,
};

use num_bigint::{BigInt, BigUint};

use crate::{
  state_machine::{DefaultState, PushStateMachine, State, StateInputCompositeValue, StateInputFieldValue},
	types::{Type, TypeDef, TypeRef}
};

pub struct TypeRegistry(BTreeMap<u32, Vec<Type>>);

impl TypeRegistry {
		pub fn new<'a>(types: impl Iterator<Item = &'a Type>) -> Self {
		Self {
				0: types.fold(Default::default(), |mut map, ty| {
					map.entry(ty.type_id.0).or_default().push(ty.clone());
					map
				})
			}
		}

		pub fn get_first_matching(&self, id: &u32) -> Option<&Type> {
			self.0.get(id)?.first()
		}
}

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

fn path_to_string<'a>(path_iterator: impl Iterator<Item = &'a str>) -> String {
	path_iterator.collect::<Vec<_>>().join(" >> ")
}

pub struct CallCardsParser<'registry> {
	pub type_registry: &'registry TypeRegistry,
	pub cards: Vec<OutputCard>,
	pub state: Box<dyn State>,
	pub stack: Vec<Box<dyn State>>
}

impl CallCardsParser<'_> {
	pub fn new<'registry>(type_registry: &'registry TypeRegistry) -> CallCardsParser<'registry> {
		CallCardsParser {
			type_registry,
			cards: vec![],
			state: Box::new(DefaultState { indent: 0 }),
			stack: vec![]
		}
	}
}

impl CallCardsParser<'_> {
	fn cloned_state(&self) -> Box<dyn State> {
		self.state.clone()
	}
}

impl PushStateMachine for CallCardsParser<'_> {
	fn push_state(&mut self) {
		self.stack.push(self.state.clone());
	}

	fn set_state(&mut self, state: Box<dyn State>) {
		self.state = state;
	}

	fn pop_state(&mut self) {
		self.state = self.stack.pop().unwrap();
	}

	fn can_pop_state(&self) -> bool {
		!self.stack.is_empty()
	}
}

impl Visitor for CallCardsParser<'_> {
	type TypeResolver = TypeResolver;
	type Value<'scale, 'resolver> = Self;
	type Error = DecodeError;

	fn visit_bool<'scale, 'resolver>(
		mut self,
		value: bool,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let mut output = self.cloned_state().process_bool(&mut self, value)?;
		self.cards.append(&mut output.cards);

		Ok(self)
	}

	fn visit_char<'scale, 'resolver>(
		mut self,
		value: char,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_char(value)?;
		self.apply_exec(output);

		Ok(self)
	}

	fn visit_u8<'scale, 'resolver>(
		mut self,
		value: u8,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_u8(value)?;
		self.apply_exec(output);

		Ok(self)
	}

	fn visit_u16<'scale, 'resolver>(
		mut self,
		value: u16,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_u16(value)?;
		self.apply_exec(output);

		Ok(self)
	}

	fn visit_u32<'scale, 'resolver>(
		mut self,
		value: u32,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_u32(value)?;
		self.apply_exec(output);

		Ok(self)
	}

	fn visit_u64<'scale, 'resolver>(
		mut self,
		value: u64,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_u64(value)?;
		self.apply_exec(output);

		Ok(self)
	}

	fn visit_u128<'scale, 'resolver>(
		mut self,
		value: u128,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_u128(value)?;
		self.apply_exec(output);

		Ok(self)
	}

	fn visit_u256<'scale, 'resolver>(
		mut self,
		value: &'scale [u8; 32],
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_u256(value)?;
		self.apply_exec(output);

		Ok(self)
	}

	fn visit_i8<'scale, 'resolver>(
		mut self,
		value: i8,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_i8(value)?;
		self.apply_exec(output);

		Ok(self)
	}

	fn visit_i16<'scale, 'resolver>(
		mut self,
		value: i16,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_i16(value)?;
		self.apply_exec(output);

		Ok(self)
	}

	fn visit_i32<'scale, 'resolver>(
		mut self,
		value: i32,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_i32(value)?;
		self.apply_exec(output);

		Ok(self)
	}

	fn visit_i64<'scale, 'resolver>(
		mut self,
		value: i64,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_i64(value)?;
		self.apply_exec(output);

		Ok(self)
	}

	fn visit_i128<'scale, 'resolver>(
		mut self,
		value: i128,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_i128(value)?;
		self.apply_exec(output);

		Ok(self)
	}

	fn visit_i256<'scale, 'resolver>(
		mut self,
		value: &'scale [u8; 32],
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_i256(value)?;
		self.apply_exec(output);

		Ok(self)
	}

	fn visit_sequence<'scale, 'resolver>(
		self,
		value: &mut scale_decode::visitor::types::Sequence<'scale, 'resolver, Self::TypeResolver>,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let mut visitor = self;
		while let Some(field) = value.next() {
			visitor = field?.decode_with_visitor(visitor)?;
		}

		Ok(visitor)
	}

	fn visit_composite<'scale, 'resolver>(
		self,
		value: &mut scale_decode::visitor::types::Composite<'scale, 'resolver, Self::TypeResolver>,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		// We forward `Void` as composite with no fields.
		// TODO: call state also
		if value.remaining() == 0 {
			return Ok(self);
		}

		let mut visitor = self;

		let path = value.path().map(|item| item.to_string()).collect();
		let fields_count = value.fields().len();

		let input = StateInputCompositeValue {
			name: value.name().map(|name| name.to_string()),
			path: Some(path),
			field_count: fields_count
		};

		let mut output = visitor.cloned_state().start_composite(&mut visitor, input)?;
		visitor.cards.append(&mut output.cards);

		while let Some((index, field_result)) = value.enumerate().next() {
			let field = field_result?;
			let field_name = field.name().map(|name| name.to_string());

			let input = StateInputFieldValue {
				index,
				name: field_name
			};

			let mut output = visitor.cloned_state().process_field(&mut visitor, input)?;
			visitor.cards.append(&mut output.cards);

			visitor = field.decode_with_visitor(visitor)?;
		}

		Ok(visitor)
	}

	fn visit_tuple<'scale, 'resolver>(
		self,
		value: &mut scale_decode::visitor::types::Tuple<'scale, 'resolver, Self::TypeResolver>,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {

		let mut visitor = self;
		while let Some((index, field)) = value.enumerate().next() {
			let card = OutputCard {
				card: ParserCard::FieldNumber { 
					number: index + 1, 
					docs_field_number: "".to_string(),
					path_type: "".to_string(),
					docs_type: "".to_string()
				},
				indent: visitor.indent
			};
			
			visitor.cards.push(card);

			visitor = field?.decode_with_visitor(visitor)?;
		}

		Ok(visitor)
	}

	fn visit_str<'scale, 'resolver>(
		mut self,
		value: &mut scale_decode::visitor::types::Str<'scale>,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let target_value = value.as_str()?.to_string();
		self.append_default_card(target_value);

		Ok(self)
	}

	fn visit_variant<'scale, 'resolver>(
		self,
		value: &mut scale_decode::visitor::types::Variant<'scale, 'resolver, Self::TypeResolver>,
		type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let enum_id = type_id.id().ok_or(DecodeError::TypeResolvingError("Expected by id".to_string()))?;
		let enum_type = self.type_registry.get_first_matching(&enum_id).ok_or(
			DecodeError::TypeResolvingError(format!("No type for {:?}", &enum_id).to_string())
		)?;

		let mut visitor = self;

		while let Some(field) = value.fields().next() {
			visitor = field?.decode_with_visitor(visitor)?;
		}

		Ok(visitor)
	}

	fn visit_array<'scale, 'resolver>(
		self,
		value: &mut scale_decode::visitor::types::Array<'scale, 'resolver, Self::TypeResolver>,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {

		let mut visitor = self;
		while let Some(field) = value.next() {
			visitor = field?.decode_with_visitor(visitor)?;
		}

		Ok(visitor)
	}

	fn visit_bitsequence<'scale, 'resolver>(
		mut self,
		value: &mut scale_decode::visitor::types::BitSequence<'scale>,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
    let result: Result<Vec<bool>, _> = value.decode()?.collect();

		let string_repr = result?.into_iter().map(|b| if b  { '1' }  else { '0' }).collect();

		let card = OutputCard {
			card: ParserCard::BitVec(string_repr),
			indent: self.indent
		};

		self.cards.push(card);

		Ok(self)
	}
}