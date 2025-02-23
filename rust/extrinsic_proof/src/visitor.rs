// The code is adopted from https://github.com/Zondax/merkleized-metadata/blob/main/src/extrinsic_decoder.rs

use core::sync::atomic::AtomicUsize;

use alloc::{
	collections::BTreeMap,
	format,
	string::String,
	vec::Vec,
};

use parser::decoding_commons::OutputCard;
use scale_decode::{
	ext::scale_type_resolver::{
		BitsOrderFormat, BitsStoreFormat, Primitive as RPrimitive, ResolvedTypeVisitor, Variant
	},
	visitor::DecodeError,
	Field, Visitor
};

use crate::{
  state_machine::{State, StateInputCompound, StateInputCompoundItem, StateOutput},
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

		pub fn get_types_by(&self, id: u32) -> Option<&Vec<Type>> {
			self.0.get(&id)
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

pub struct CallCardsParser<'registry> {
	pub type_registry: &'registry TypeRegistry,
	pub cards: Vec<OutputCard>,
	pub state: Box<dyn State>,
	pub stack: Vec<u32>,
	pub indent: u32
}

impl CallCardsParser<'_> {
	pub fn new<'registry>(type_registry: &'registry TypeRegistry, state: impl State + 'static) -> CallCardsParser<'registry> {
		CallCardsParser {
			type_registry, 
			cards: vec![],
			state: Box::new(state),
			stack: vec![],
			indent: 0
		 }
	}

	fn cloned_state(&self) -> Box<dyn State> {
		self.state.clone()
	}

	fn apply(&mut self, mut output: StateOutput) {
		self.state = output.next_state;
		self.cards.append(&mut output.cards);
		self.indent = output.indent;
	}

	fn push_indent(&mut self) {
		self.stack.push(self.indent);
	}

	fn pop_indent(&mut self) {
		self.indent =	self.stack.pop().unwrap();
	}

	fn get_composite_field_type_name(&self, type_ref: &TypeRef, field_index: usize) -> Option<String> {
		let type_id = type_ref.id()?;

		let result_type = self.type_registry.get_types_by(type_id)?.first()?;
		
		match &result_type.type_def {
				TypeDef::Composite(fields) => fields[field_index].type_name.clone(),
				_ => None
		}
	}

	fn get_enum_field_type_name(&self, type_ref: &TypeRef, variant_index: u32, field_index: usize) -> Option<String> {
		let type_id = type_ref.id()?;

		let result_type = self.type_registry.get_types_by(type_id)?
    													.into_iter()
    													.find(|t| match t.type_def.as_enumeration() {
        												Some(e) => e.index.0 == variant_index,
        												None => false,
    													})?;
		
		match &result_type.type_def {
				TypeDef::Enumeration(variant) => variant.fields[field_index].type_name.clone(),
				_ => None
		}
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
		let output = self.state.process_bool(value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_char<'scale, 'resolver>(
		mut self,
		value: char,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_char(value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_u8<'scale, 'resolver>(
		mut self,
		value: u8,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_u8(value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_u16<'scale, 'resolver>(
		mut self,
		value: u16,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_u16(value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_u32<'scale, 'resolver>(
		mut self,
		value: u32,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_u32( value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_u64<'scale, 'resolver>(
		mut self,
		value: u64,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_u64(value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_u128<'scale, 'resolver>(
		mut self,
		value: u128,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_u128(value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_u256<'scale, 'resolver>(
		mut self,
		value: &'scale [u8; 32],
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_u256(value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_i8<'scale, 'resolver>(
		mut self,
		value: i8,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_i8(value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_i16<'scale, 'resolver>(
		mut self,
		value: i16,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_i16(value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_i32<'scale, 'resolver>(
		mut self,
		value: i32,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_i32(value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_i64<'scale, 'resolver>(
		mut self,
		value: i64,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_i64(value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_i128<'scale, 'resolver>(
		mut self,
		value: i128,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_i128(value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_i256<'scale, 'resolver>(
		mut self,
		value: &'scale [u8; 32],
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let output = self.state.process_i256(value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_str<'scale, 'resolver>(
		mut self,
		value: &mut scale_decode::visitor::types::Str<'scale>,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let target_value = value.as_str()?.to_string();
		let output = self.state.process_str(target_value, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_bitsequence<'scale, 'resolver>(
		mut self,
		value: &mut scale_decode::visitor::types::BitSequence<'scale>,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
    let bits = value.decode()?.collect::<Result<Vec<bool>, _>>()?;

		let output = self.state.process_bitsequence(bits, self.indent)?;
		self.apply(output);

		Ok(self)
	}

	fn visit_sequence<'scale, 'resolver>(
		self,
		value: &mut scale_decode::visitor::types::Sequence<'scale, 'resolver, Self::TypeResolver>,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let mut visitor = self;

		let path = None;

		let seq_items: Vec<_> = value.collect();
		let items_count = seq_items.len();

		let input = StateInputCompound {
			name: None,
			path: &path,
			items_count
		};

		visitor.push_indent();

		let output = visitor.state.process_sequence(&input, visitor.indent)?;
		visitor.apply(output);

		for (index, field_result) in seq_items.into_iter().enumerate() {
			visitor.push_indent();

			let input = StateInputCompoundItem {
				index,
  			name: None,
  			parent_path: &path,
				type_name: None,
  			items_count
			};

			let output = visitor.state.process_sequence_item(&input, visitor.indent)?;
			visitor.apply(output);

			visitor = field_result?.decode_with_visitor(visitor)?;

			visitor.pop_indent();
		}

		visitor.pop_indent();

		Ok(visitor)
	}

	fn visit_composite<'scale, 'resolver>(
		self,
		value: &mut scale_decode::visitor::types::Composite<'scale, 'resolver, Self::TypeResolver>,
		type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let mut visitor = self;

		// We forward `Void` as composite with no fields.
		// TODO: call state also
		if value.remaining() == 0 {
			return Ok(visitor);
		}

		let path = Some(value.path().map(|item| item.to_string()).collect());
		let fields_count = value.fields().len();

		let input = StateInputCompound {
			name: value.name().map(|name| name.to_string()),
			path: &path,
			items_count: fields_count
		};

		visitor.push_indent();

		let output = visitor.state.process_composite(&input, visitor.indent)?;
		visitor.apply(output);

		while let Some((index, field_result)) = value.enumerate().next() {
			visitor.push_indent();

			let field = field_result?;
			let field_name = field.name().map(|name| name.to_string());
			let type_name = visitor.get_composite_field_type_name(&type_id, index);

			let input = StateInputCompoundItem {
				index,
  			name: field_name,
  			parent_path: &path,
				type_name,
  			items_count: fields_count
			};

			let output = visitor.state.process_field(&input, visitor.indent)?;
			visitor.apply(output);

			visitor = field.decode_with_visitor(visitor)?;

			visitor.pop_indent();
		}

		visitor.pop_indent();

		Ok(visitor)
	}

	fn visit_tuple<'scale, 'resolver>(
		self,
		value: &mut scale_decode::visitor::types::Tuple<'scale, 'resolver, Self::TypeResolver>,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {

		let mut visitor = self;

		let path = None;
		let items: Vec<_> = value.collect();
		let items_count = value.count();

		let input = StateInputCompound {
			name: None,
			path: &path,
			items_count
		};

		visitor.push_indent();

		let output = visitor.cloned_state().process_tuple(&input, visitor.indent)?;
		visitor.apply(output);

		for (index, field) in items.into_iter().enumerate() {
			visitor.push_indent();

			let input = StateInputCompoundItem {
				index,
  			name: None,
  			parent_path: &path,
				type_name: None,
  			items_count
			};

			let output = visitor.cloned_state().process_tuple_item(&input, visitor.indent)?;
			visitor.apply(output);

			visitor = field?.decode_with_visitor(visitor)?;

			visitor.pop_indent();
		}

		visitor.pop_indent();

		Ok(visitor)
	}

	fn visit_variant<'scale, 'resolver>(
		self,
		value: &mut scale_decode::visitor::types::Variant<'scale, 'resolver, Self::TypeResolver>,
		type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
		let mut visitor = self;

		let path = None;
		let fields: Vec<_> = value.fields().collect();
		let fields_count = fields.len();

		let input = StateInputCompound {
			name: Some(value.name().to_string()),
			path: &path,
			items_count: fields_count
		};

		visitor.push_indent();

		let output = visitor.cloned_state().process_variant(&input, visitor.indent)?;
		visitor.apply(output);

		for (index, field_result) in fields.into_iter().enumerate() {
			visitor.push_indent();

			let field = field_result?;
			let field_name = field.name().map(|name| name.to_string());
			let type_name = visitor.get_enum_field_type_name(&type_id, value.index() as u32, index);

			let input = StateInputCompoundItem {
				index,
  			name: field_name,
  			parent_path: &path,
				type_name: type_name,
  			items_count: fields_count
			};

			let output = visitor.cloned_state().process_field(&input,visitor.indent)?;
			visitor.apply(output);

			visitor = field.decode_with_visitor(visitor)?;

			visitor.pop_indent();
		}

		visitor.pop_indent();

		Ok(visitor)
	}

	fn visit_array<'scale, 'resolver>(
		self,
		value: &mut scale_decode::visitor::types::Array<'scale, 'resolver, Self::TypeResolver>,
		_type_id: TypeRef,
	) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {

		let mut visitor = self;

		let path = None;
		let items: Vec<_> = value.collect();
		let items_count = items.len();

		let input = StateInputCompound {
			name: None,
			path: &path,
			items_count
		};

		visitor.push_indent();

		let output = visitor.cloned_state().process_array(&input, visitor.indent)?;
		visitor.apply(output);

		for (index, field) in items.into_iter().enumerate() {
			visitor.push_indent();

			let input = StateInputCompoundItem {
				index,
  			name: None,
  			parent_path: &path,
				type_name: None,
  			items_count
			};

			let output = visitor.cloned_state().process_array_item(&input, visitor.indent)?;
			visitor.apply(output);

			visitor = field?.decode_with_visitor(visitor)?;

			visitor.pop_indent();
		}

		visitor.pop_indent();

		Ok(visitor)
	}
}