use alloc::{collections::BTreeMap, format, string::String, vec::Vec};

use parity_scale_codec::Decode;
use scale_decode::{visitor::DecodeAsTypeResult, Visitor};

use merkleized_metadata::{
    types::{Type, TypeDef, TypeRef},
    ExtraInfo, TypeResolver,
};

use crate::{
    decoding_commons::OutputCard,
    state::{State, StateError, StateInputCompound, StateInputCompoundItem, StateOutput},
};

/// Maximum allowed recursion depth to prevent stack overflow from deeply nested types.
/// This protects against malicious payloads like deeply nested utility.batch calls.
///
/// Calculation for mobile safety (iOS secondary thread = 512KB, Android ~1MB):
/// - Each batch nesting level uses 4 stack entries (variant, field, sequence, item)
/// - Each level also adds approx. 2 decode_with_visitor calls to native stack
/// - Empirically tested: 128 works safely, 256 overflows in debug builds
/// - 128 / 4 = 32 levels of nested batch calls supported
///
/// This is far beyond any legitimate use case (complex governance rarely exceeds 6 levels).
const MAX_RECURSION_DEPTH: u32 = 128;

/// Implements the state machine responsible for decoding SCALE-encoded data
/// into a human-readable format, represented as a list of output cards.
///
/// On one hand, this state machine follows the visitor protocol, handling
/// decoding logic for each supported type. On the other hand, it delegates
/// to the current state the decision-making regarding the generated output cards and
/// transitions to the next state.
///
/// For more information on how state transitions and output generation are handled,
/// refer to the [`State`] trait documentation.
pub struct TypeRegistry(BTreeMap<u32, Vec<Type>>);

impl TypeRegistry {
    pub fn new<'a>(types: impl Iterator<Item = &'a Type>) -> Self {
        let tree: BTreeMap<u32, Vec<Type>> = types.fold(Default::default(), |mut map, ty| {
            map.entry(ty.type_id.0).or_default().push(ty.clone());
            map
        });

        Self(tree)
    }

    pub fn get_types_by(&self, id: u32) -> Option<&Vec<Type>> {
        self.0.get(&id)
    }

    pub fn get_first_type(&self, type_ref: &TypeRef) -> Option<Type> {
        let type_id = type_ref.id()?;

        self.get_types_by(type_id)?.first().cloned()
    }

    pub fn get_composite_field_type_name(
        &self,
        type_ref: &TypeRef,
        field_index: usize,
    ) -> Option<String> {
        let result_type = self.get_first_type(type_ref)?;

        match &result_type.type_def {
            TypeDef::Composite(fields) => fields[field_index].type_name.clone(),
            _ => None,
        }
    }

    pub fn get_enum_variant_type(&self, type_ref: &TypeRef, variant_index: u32) -> Option<Type> {
        let type_id = type_ref.id()?;

        self.get_types_by(type_id)?
            .iter()
            .find(|t| match t.type_def.as_enumeration() {
                Some(e) => e.index.0 == variant_index,
                None => false,
            })
            .cloned()
    }

    pub fn get_enum_field_type_name(
        &self,
        type_ref: &TypeRef,
        variant_index: u32,
        field_index: usize,
    ) -> Option<String> {
        let result_type = self.get_enum_variant_type(type_ref, variant_index)?;

        match &result_type.type_def {
            TypeDef::Enumeration(variant) => variant.fields[field_index].type_name.clone(),
            _ => None,
        }
    }
}

// Implementation of the State Machine to decode scale encoded data into human readable format represented by the list of output cards.
//
// From one hand the State Machine implements visitor protocol to handle decoding for each possible type. From the other hands
// for given type and state the State Machine delegates decision about output cards and next state to the current state.
// See State trait for further details.
pub struct StateMachineParser<'registry> {
    pub type_registry: &'registry TypeRegistry,
    pub extra_info: ExtraInfo,
    pub cards: Vec<OutputCard>,
    pub state: Box<dyn State>,
    pub stack: Vec<u32>,
    pub indent: u32,
}

impl StateMachineParser<'_> {
    pub fn new(
        type_registry: &TypeRegistry,
        extra_info: ExtraInfo,
        state: impl State + 'static,
    ) -> StateMachineParser<'_> {
        StateMachineParser {
            type_registry,
            extra_info,
            cards: vec![],
            state: Box::new(state),
            stack: vec![],
            indent: 0,
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
        self.indent = self.stack.pop().unwrap();
    }
}

impl Visitor for StateMachineParser<'_> {
    type TypeResolver = TypeResolver;
    type Value<'scale, 'resolver> = Self;
    type Error = StateError;

    /// Early bailout for types that could cause DoS.
    ///
    /// This method is called before the type is resolved and decoded. By returning
    /// `DecodeAsTypeResult::Decoded(Err(...))` here, we bypass the normal decoding path
    /// entirely, including the `skip_decoding` call that would otherwise iterate through
    /// all elements (potentially billions of times for malicious input).
    ///
    /// We check for:
    /// 1. Recursion depth - prevents stack overflow from deeply nested types
    /// 2. Array/sequence length - prevents DoS from huge claimed lengths
    ///
    /// Note: Length validation assumes non-zero-sized element types (at least 1 byte per item).
    /// Arrays or sequences with zero-sized element types (e.g., `[(); N]` or `Vec<()>`) are
    /// not supported and will be rejected if their length exceeds available bytes.
    fn unchecked_decode_as_type<'scale, 'resolver>(
        self,
        input: &mut &'scale [u8],
        type_id: TypeRef,
        _types: &'resolver Self::TypeResolver,
    ) -> DecodeAsTypeResult<Self, Result<Self::Value<'scale, 'resolver>, Self::Error>> {
        // Check recursion depth first - this prevents stack overflow from deeply nested types.
        // We must check here (before normal decoding) because scale_decode's skip_decoding
        // also recurses and would overflow the stack if we only checked in our visitor methods.
        if self.stack.len() >= MAX_RECURSION_DEPTH as usize {
            return DecodeAsTypeResult::Decoded(Err(StateError::BadInput(format!(
                "Recursion depth {} exceeds maximum allowed {}",
                self.stack.len(),
                MAX_RECURSION_DEPTH
            ))));
        }

        if let Some(ty) = self.type_registry.get_first_type(&type_id) {
            match &ty.type_def {
                // Each element of array or sequence requires at least 1 byte for non-zero-sized types.
                TypeDef::Array(arr) => {
                    if arr.len as usize > input.len() {
                        return DecodeAsTypeResult::Decoded(Err(StateError::BadInput(format!(
                            "Array claims {} items but only {} bytes remain",
                            arr.len,
                            input.len()
                        ))));
                    }
                }
                TypeDef::Sequence(_) => {
                    // Peek at the compact-encoded length without consuming bytes
                    let mut peek_input = *input;
                    if let Ok(parity_scale_codec::Compact(claimed_len)) =
                        <parity_scale_codec::Compact<u32>>::decode(&mut peek_input)
                    {
                        let remaining_bytes = peek_input.len();
                        if claimed_len as usize > remaining_bytes {
                            return DecodeAsTypeResult::Decoded(Err(StateError::BadInput(
                                format!(
                                    "Sequence claims {} items but only {} bytes remain",
                                    claimed_len, remaining_bytes
                                ),
                            )));
                        }
                    }
                }
                _ => {}
            }
        }
        DecodeAsTypeResult::Skipped(self)
    }

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
        let output = self.state.process_u32(value, self.indent)?;
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

    fn visit_u256<'resolver>(
        mut self,
        value: &[u8; 32],
        _type_id: TypeRef,
    ) -> Result<Self::Value<'_, 'resolver>, Self::Error> {
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

    fn visit_i256<'resolver>(
        mut self,
        value: &[u8; 32],
        _type_id: TypeRef,
    ) -> Result<Self::Value<'_, 'resolver>, Self::Error> {
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
        let bits = value
            .decode()?
            .collect::<Result<Vec<bool>, _>>()
            .map_err(|_| StateError::BadInput("bitsequence".to_string()))?;

        let output = self.state.process_bitsequence(bits, self.indent)?;
        self.apply(output);

        Ok(self)
    }

    fn visit_sequence<'scale, 'resolver>(
        self,
        value: &mut scale_decode::visitor::types::Sequence<'scale, 'resolver, Self::TypeResolver>,
        type_id: TypeRef,
    ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
        let mut visitor = self;

        let seq_type = visitor.type_registry.get_first_type(&type_id);
        let path = seq_type.as_ref().map(|v| v.path.clone());

        // Extract item type from sequence's TypeDef to avoid using iterator which pre-decodes with IgnoreVisitor
        let item_type_id = seq_type.and_then(|ty| match &ty.type_def {
            TypeDef::Sequence(inner_type) => Some(*inner_type),
            _ => None,
        });

        let items_count = value.remaining();

        let input = StateInputCompound {
            name: None,
            path: &path,
            type_name: path.as_ref().and_then(|p| p.last()).cloned(),
            extra_info: visitor.extra_info.clone(),
            items_count,
        };

        visitor.push_indent();

        let output = visitor.state.process_sequence(&input, visitor.indent)?;
        visitor.apply(output);

        // Get item path once (all items have the same type in a sequence)
        let item_path = item_type_id
            .as_ref()
            .and_then(|id| visitor.type_registry.get_first_type(id))
            .map(|v| v.path);

        for index in 0..items_count {
            visitor.push_indent();

            let input = StateInputCompoundItem {
                index,
                name: None,
                path: &item_path,
                extra_info: visitor.extra_info.clone(),
                type_name: item_path.as_ref().and_then(|p| p.last()).cloned(),
                items_count,
            };

            let output = visitor
                .state
                .process_sequence_item(&input, visitor.indent)?;
            visitor.apply(output);

            // Use decode_item directly instead of iterator to avoid IgnoreVisitor pre-decoding
            visitor = value
                .decode_item(visitor)
                .ok_or_else(|| StateError::BadInput("Unexpected end of sequence".into()))??;

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
        if value.remaining() == 0 {
            let output = visitor.state.process_void(visitor.indent)?;
            visitor.apply(output);

            return Ok(visitor);
        }

        let path = visitor
            .type_registry
            .get_first_type(&type_id)
            .map(|v| v.path);

        let fields_count = value.fields().len();

        let input = StateInputCompound {
            name: value.name().map(|name| name.to_string()),
            path: &path,
            type_name: path.as_ref().and_then(|p| p.last()).cloned(),
            extra_info: visitor.extra_info.clone(),
            items_count: fields_count,
        };

        visitor.push_indent();

        let output = visitor.state.process_composite(&input, visitor.indent)?;
        visitor.apply(output);

        // Extract field metadata upfront to avoid using the iterator which pre-decodes with IgnoreVisitor.
        let field_infos: Vec<_> = value
            .fields()
            .iter()
            .map(|f| (f.name.map(|s| s.to_string()), f.id))
            .collect();

        for (index, (field_name, field_type_id)) in field_infos.into_iter().enumerate() {
            visitor.push_indent();

            let type_name = visitor
                .type_registry
                .get_composite_field_type_name(&type_id, index);
            let field_path = visitor
                .type_registry
                .get_first_type(&field_type_id)
                .map(|v| v.path);

            let input = StateInputCompoundItem {
                index,
                name: field_name,
                path: &field_path,
                extra_info: visitor.extra_info.clone(),
                type_name,
                items_count: fields_count,
            };

            let output = visitor.state.process_field(&input, visitor.indent)?;
            visitor.apply(output);

            // Use decode_item directly instead of iterator to avoid IgnoreVisitor pre-decoding
            visitor = value
                .decode_item(visitor)
                .ok_or_else(|| StateError::BadInput("Unexpected end of items".into()))??;

            visitor.pop_indent();
        }

        visitor.pop_indent();

        Ok(visitor)
    }

    fn visit_tuple<'scale, 'resolver>(
        self,
        value: &mut scale_decode::visitor::types::Tuple<'scale, 'resolver, Self::TypeResolver>,
        type_id: TypeRef,
    ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
        let mut visitor = self;

        let tuple_type = visitor.type_registry.get_first_type(&type_id);
        let path = tuple_type.as_ref().map(|v| v.path.clone());

        // Extract item type refs from tuple's TypeDef to avoid using iterator which pre-decodes with IgnoreVisitor
        let item_type_refs: Vec<TypeRef> = tuple_type
            .and_then(|ty| match ty.type_def {
                TypeDef::Tuple(refs) => Some(refs),
                _ => None,
            })
            .unwrap_or_default();

        let items_count = value.remaining();

        let input = StateInputCompound {
            name: None,
            path: &path,
            type_name: path.as_ref().and_then(|p| p.last()).cloned(),
            extra_info: visitor.extra_info.clone(),
            items_count,
        };

        visitor.push_indent();

        let output = visitor
            .cloned_state()
            .process_tuple(&input, visitor.indent)?;
        visitor.apply(output);

        // Use decode_item directly instead of iterator to avoid IgnoreVisitor pre-decoding
        for index in 0..items_count {
            visitor.push_indent();

            let item_path = item_type_refs
                .get(index)
                .and_then(|type_ref| visitor.type_registry.get_first_type(type_ref))
                .map(|ty| ty.path);

            let input = StateInputCompoundItem {
                index,
                name: None,
                path: &item_path,
                extra_info: visitor.extra_info.clone(),
                type_name: item_path.as_ref().and_then(|p| p.last()).cloned(),
                items_count,
            };

            let output = visitor
                .cloned_state()
                .process_tuple_item(&input, visitor.indent)?;
            visitor.apply(output);

            visitor = value
                .decode_item(visitor)
                .ok_or_else(|| StateError::BadInput("Unexpected end of tuple items".into()))??;

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

        let path = visitor
            .type_registry
            .get_enum_variant_type(&type_id, value.index() as u32)
            .map(|v| v.path);

        let fields_count = value.fields().remaining();
        let variant_index = value.index();

        let input = StateInputCompound {
            name: Some(value.name().to_string()),
            path: &path,
            type_name: path.as_ref().and_then(|p| p.last()).cloned(),
            extra_info: visitor.extra_info.clone(),
            items_count: fields_count,
        };

        visitor.push_indent();

        let output = visitor
            .cloned_state()
            .process_variant(&input, visitor.indent)?;
        visitor.apply(output);

        // Extract field metadata upfront to avoid using the iterator which pre-decodes with IgnoreVisitor.
        // This prevents stack overflow when decoding deeply nested types.
        let field_infos: Vec<_> = value
            .fields()
            .fields()
            .iter()
            .map(|f| (f.name.map(|s| s.to_string()), f.id))
            .collect();

        for (index, (field_name, field_type_id)) in field_infos.into_iter().enumerate() {
            visitor.push_indent();

            let type_name = visitor.type_registry.get_enum_field_type_name(
                &type_id,
                variant_index as u32,
                index,
            );

            let field_path = visitor
                .type_registry
                .get_first_type(&field_type_id)
                .map(|v| v.path);

            let input = StateInputCompoundItem {
                index,
                name: field_name,
                path: &field_path,
                extra_info: visitor.extra_info.clone(),
                type_name,
                items_count: fields_count,
            };

            let output = visitor
                .cloned_state()
                .process_field(&input, visitor.indent)?;
            visitor.apply(output);

            // Use decode_item directly instead of iterator to avoid IgnoreVisitor pre-decoding
            visitor = value
                .fields()
                .decode_item(visitor)
                .ok_or_else(|| StateError::BadInput("Unexpected end of fields".into()))??;

            visitor.pop_indent();
        }

        visitor.pop_indent();

        Ok(visitor)
    }

    fn visit_array<'scale, 'resolver>(
        self,
        value: &mut scale_decode::visitor::types::Array<'scale, 'resolver, Self::TypeResolver>,
        type_id: TypeRef,
    ) -> Result<Self::Value<'scale, 'resolver>, Self::Error> {
        let mut visitor = self;

        let arr_type = visitor.type_registry.get_first_type(&type_id);
        let path = arr_type.as_ref().map(|v| v.path.clone());

        // Extract item type from array's TypeDef to avoid using iterator which pre-decodes with IgnoreVisitor
        let item_type_id = arr_type.and_then(|ty| match &ty.type_def {
            TypeDef::Array(arr) => Some(arr.type_param),
            _ => None,
        });

        let items_count = value.remaining();

        let input = StateInputCompound {
            name: None,
            path: &path,
            type_name: path.as_ref().and_then(|p| p.last()).cloned(),
            extra_info: visitor.extra_info.clone(),
            items_count,
        };

        visitor.push_indent();

        let output = visitor
            .cloned_state()
            .process_array(&input, visitor.indent)?;
        visitor.apply(output);

        // Get item path once (all items have the same type in an array)
        let item_path = item_type_id
            .as_ref()
            .and_then(|id| visitor.type_registry.get_first_type(id))
            .map(|v| v.path);

        // Use decode_item directly instead of iterator to avoid IgnoreVisitor pre-decoding
        for index in 0..items_count {
            visitor.push_indent();

            let input = StateInputCompoundItem {
                index,
                name: None,
                path: &item_path,
                extra_info: visitor.extra_info.clone(),
                type_name: item_path.as_ref().and_then(|p| p.last()).cloned(),
                items_count,
            };

            let output = visitor
                .cloned_state()
                .process_array_item(&input, visitor.indent)?;
            visitor.apply(output);

            visitor = value
                .decode_item(visitor)
                .ok_or_else(|| StateError::BadInput("Unexpected end of array items".into()))??;

            visitor.pop_indent();
        }

        visitor.pop_indent();

        Ok(visitor)
    }
}
