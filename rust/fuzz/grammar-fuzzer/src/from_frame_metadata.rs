use alloc::{
    collections::{BTreeMap, BTreeSet},
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use frame_metadata::{
    v15::{ExtrinsicMetadata, SignedExtensionMetadata},
    RuntimeMetadata,
};
use scale_info::{
    form::PortableForm, interner::UntrackedSymbol, Field, PortableRegistry, Type, TypeDef,
    TypeDefArray, TypeDefBitSequence, TypeDefPrimitive, Variant,
};

use crate::{merkle_tree::TypeId, types};

/// The type information generated from the FRAME metadata.
pub struct TypeInformation {
    /// The extrinsic metadata in the final form.
    pub extrinsic_metadata: types::ExtrinsicMetadata,
    /// All the types.
    pub types: BTreeMap<TypeId, types::Type>,
}

pub struct FrameMetadataPrepared {
    accessible_types: BTreeSet<u32>,
    frame_type_registry: PortableRegistry,
    extrinsic_metadata: ExtrinsicMetadata<PortableForm>,
}

impl FrameMetadataPrepared {
    pub fn prepare(metadata: &RuntimeMetadata) -> Result<Self, String> {
        let metadata = match metadata {
            RuntimeMetadata::V15(m) => m,
            _ => return Err("Only supports metadata V15".into()),
        };

        let frame_type_registry = metadata.types.clone();
        let extrinsic_metadata = metadata.extrinsic.clone();

        let mut accessible_types = Default::default();

        collect_accessible_types(
            extrinsic_metadata.call_ty.id,
            &mut accessible_types,
            &frame_type_registry,
        )?;
        collect_accessible_types(
            extrinsic_metadata.address_ty.id,
            &mut accessible_types,
            &frame_type_registry,
        )?;
        collect_accessible_types(
            extrinsic_metadata.signature_ty.id,
            &mut accessible_types,
            &frame_type_registry,
        )?;

        extrinsic_metadata
            .signed_extensions
            .iter()
            .try_for_each(|se| {
                collect_accessible_types(se.ty.id, &mut accessible_types, &frame_type_registry)?;
                collect_accessible_types(
                    se.additional_signed.id,
                    &mut accessible_types,
                    &frame_type_registry,
                )
            })?;

        Ok(Self {
            frame_type_registry,
            accessible_types,
            extrinsic_metadata,
        })
    }

    fn get_type(&self, id: u32) -> &Type<PortableForm> {
        &self.frame_type_registry.types[id as usize].ty
    }

    pub fn as_type_information(&self) -> Result<TypeInformation, String> {
        let mut next_id = 0;
        let frame_id_to_id = self
            .accessible_types
            .iter()
            .filter(|&id| self.get_type(*id).is_basic_type())
            .map(|id| {
                let new_id = next_id;
                next_id += 1;
                (*id, new_id)
            })
            .collect::<BTreeMap<u32, u32>>();

        let type_context = TypeContext {
            frame_id_to_id: &frame_id_to_id,
            frame_type_registry: &self.frame_type_registry,
        };

        let extrinsic_metadata = self.extrinsic_metadata.as_basic_type(type_context)?;
        let types =
            frame_id_to_id
                .iter()
                .try_fold(BTreeMap::default(), |mut res, (frame_id, id)| {
                    res.extend(
                        self.get_type(*frame_id)
                            .as_basic_type(type_context)?
                            .into_iter()
                            .map(|mut ty| {
                                ty.type_id = id.into();

                                let id = if let Some(variant) = ty.type_def.as_enumeration() {
                                    TypeId::Enumeration {
                                        type_id: *id,
                                        variant: variant.index.0,
                                    }
                                } else {
                                    TypeId::Other(*id)
                                };

                                (id, ty)
                            }),
                    );

                    Ok::<_, String>(res)
                })?;

        Ok(TypeInformation {
            extrinsic_metadata,
            types,
        })
    }
}

fn collect_accessible_types(
    ty_id: u32,
    accessible_types: &mut BTreeSet<u32>,
    registry: &PortableRegistry,
) -> Result<(), String> {
    if !accessible_types.insert(ty_id) {
        return Ok(());
    }

    let ty = &registry
        .types
        .get(ty_id as usize)
        .ok_or_else(|| format!("Could not find type with id `{ty_id}` in the registry"))?
        .ty;

    match &ty.type_def {
        TypeDef::Composite(c) => c
            .fields
            .iter()
            .try_for_each(|f| collect_accessible_types(f.ty.id, accessible_types, registry))?,
        TypeDef::Variant(v) => v.variants.iter().try_for_each(|v| {
            v.fields
                .iter()
                .try_for_each(|f| collect_accessible_types(f.ty.id, accessible_types, registry))
        })?,
        TypeDef::Sequence(s) => {
            collect_accessible_types(s.type_param.id, accessible_types, registry)?
        }
        TypeDef::Array(a) => collect_accessible_types(a.type_param.id, accessible_types, registry)?,
        TypeDef::Tuple(t) => t
            .fields
            .iter()
            .try_for_each(|t| collect_accessible_types(t.id, accessible_types, registry))?,
        // Primitive types are not individual types in the final type information.
        TypeDef::Primitive(_) => {}
        // `Compact` is converted to a primitive like type and thus, the inner type is not required.
        TypeDef::Compact(_) => {}
        // The order and store types are also not required.
        TypeDef::BitSequence(_) => {}
    };

    Ok(())
}

#[derive(Clone, Copy)]
struct TypeContext<'a> {
    frame_type_registry: &'a PortableRegistry,
    frame_id_to_id: &'a BTreeMap<u32, u32>,
}

impl<'a> TypeContext<'a> {
    fn get_type(&self, id: u32) -> Result<&Type<PortableForm>, String> {
        self.frame_type_registry
            .types
            .get(id as usize)
            .map(|t| &t.ty)
            .ok_or_else(|| format!("Could not find type for id `{id}`."))
    }
}

trait AsBasicTypeRef {
    fn as_basic_type_ref(&self, context: TypeContext<'_>) -> Result<types::TypeRef, String>;
}

trait AsBasicType {
    type BasicType;

    fn as_basic_type(&self, context: TypeContext<'_>) -> Result<Self::BasicType, String>;
}

trait IsBasicType {
    fn is_basic_type(&self) -> bool;
}

impl<T> AsBasicTypeRef for UntrackedSymbol<T> {
    fn as_basic_type_ref(&self, context: TypeContext<'_>) -> Result<types::TypeRef, String> {
        let frame_type = context.get_type(self.id)?;

        let mut collector = CollectPrimitives::default();
        collector.visit_type(context, frame_type)?;

        Ok(match &frame_type.type_def {
            TypeDef::Primitive(p) => match p {
                scale_info::TypeDefPrimitive::Bool => types::TypeRef::Bool,
                scale_info::TypeDefPrimitive::Char => types::TypeRef::Char,
                scale_info::TypeDefPrimitive::Str => types::TypeRef::Str,
                scale_info::TypeDefPrimitive::U8 => types::TypeRef::U8,
                scale_info::TypeDefPrimitive::U16 => types::TypeRef::U16,
                scale_info::TypeDefPrimitive::U32 => types::TypeRef::U32,
                scale_info::TypeDefPrimitive::U64 => types::TypeRef::U64,
                scale_info::TypeDefPrimitive::U128 => types::TypeRef::U128,
                scale_info::TypeDefPrimitive::U256 => types::TypeRef::U256,
                scale_info::TypeDefPrimitive::I8 => types::TypeRef::I8,
                scale_info::TypeDefPrimitive::I16 => types::TypeRef::I16,
                scale_info::TypeDefPrimitive::I32 => types::TypeRef::I32,
                scale_info::TypeDefPrimitive::I64 => types::TypeRef::I64,
                scale_info::TypeDefPrimitive::I128 => types::TypeRef::I128,
                scale_info::TypeDefPrimitive::I256 => types::TypeRef::I256,
            },
            TypeDef::Compact(_) => {
                if collector.found.len() > 1 {
                    return Err(format!("Unexpected: {:?}", collector.found));
                } else if let Some(found) = collector.found.first() {
                    match found {
                        scale_info::TypeDefPrimitive::U8 => types::TypeRef::CompactU8,
                        scale_info::TypeDefPrimitive::U16 => types::TypeRef::CompactU16,
                        scale_info::TypeDefPrimitive::U32 => types::TypeRef::CompactU32,
                        scale_info::TypeDefPrimitive::U64 => types::TypeRef::CompactU64,
                        scale_info::TypeDefPrimitive::U128 => types::TypeRef::CompactU128,
                        scale_info::TypeDefPrimitive::U256 => types::TypeRef::CompactU256,
                        p => {
                            return Err(format!("Unsupported primitive type for `Compact`: {p:?}"))
                        }
                    }
                } else {
                    types::TypeRef::Void
                }
            }
            TypeDef::Variant(v) if v.variants.is_empty() => types::TypeRef::Void,
            TypeDef::Composite(c) if c.fields.is_empty() => types::TypeRef::Void,
            TypeDef::Tuple(t) if t.fields.is_empty() => types::TypeRef::Void,
            _ => types::TypeRef::ById(context.frame_id_to_id.get(&self.id).unwrap().into()),
        })
    }
}

impl AsBasicType for Type<PortableForm> {
    type BasicType = Vec<types::Type>;

    fn as_basic_type(&self, context: TypeContext) -> Result<Self::BasicType, String> {
        let path = self
            .path
            .segments
            .iter()
            .map(|s| AsRef::<str>::as_ref(s).to_string())
            .collect::<Vec<_>>();

        let type_def = match &self.type_def {
            TypeDef::Compact(_) | TypeDef::Primitive(_) => return Ok(Vec::new()),
            TypeDef::Composite(c) if c.fields.is_empty() => return Ok(Vec::new()),
            TypeDef::Variant(v) if v.variants.is_empty() => return Ok(Vec::new()),
            TypeDef::Tuple(t) if t.fields.is_empty() => return Ok(Vec::new()),
            TypeDef::Variant(v) => {
                let mut variants = v.variants.clone();
                variants.sort_by_key(|v| v.index);

                return variants
                    .iter()
                    .map(|v| {
                        Ok(types::Type {
                            path: path.clone(),
                            type_def: types::TypeDef::Enumeration(v.as_basic_type(context)?),
                            type_id: 0u32.into(),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>();
            }
            TypeDef::Array(a) => types::TypeDef::Array(a.as_basic_type(context)?),
            TypeDef::Composite(c) => types::TypeDef::Composite(
                c.fields
                    .iter()
                    .map(|f| f.as_basic_type(context))
                    .collect::<Result<_, _>>()?,
            ),
            TypeDef::Sequence(s) => {
                types::TypeDef::Sequence(s.type_param.as_basic_type_ref(context)?)
            }
            TypeDef::Tuple(t) => types::TypeDef::Tuple(
                t.fields
                    .iter()
                    .map(|t| t.as_basic_type_ref(context))
                    .collect::<Result<_, _>>()?,
            ),
            TypeDef::BitSequence(b) => types::TypeDef::BitSequence(b.as_basic_type(context)?),
        };

        Ok(vec![types::Type {
            path,
            type_def,
            type_id: 0u32.into(),
        }])
    }
}

impl IsBasicType for Type<PortableForm> {
    fn is_basic_type(&self) -> bool {
        match &self.type_def {
            TypeDef::Compact(_) | TypeDef::Primitive(_) => false,
            TypeDef::Variant(v) if v.variants.is_empty() => false,
            TypeDef::Composite(c) if c.fields.is_empty() => false,
            TypeDef::Tuple(t) if t.fields.is_empty() => false,
            _ => true,
        }
    }
}

impl AsBasicType for Field<PortableForm> {
    type BasicType = types::Field;

    fn as_basic_type(&self, context: TypeContext) -> Result<Self::BasicType, String> {
        Ok(types::Field {
            name: self
                .name
                .as_ref()
                .map(|n| AsRef::<str>::as_ref(n).to_string()),
            ty: self.ty.as_basic_type_ref(context)?,
            type_name: self
                .type_name
                .as_ref()
                .map(|n| AsRef::<str>::as_ref(n).to_string()),
        })
    }
}

impl AsBasicType for Variant<PortableForm> {
    type BasicType = types::EnumerationVariant;

    fn as_basic_type(&self, context: TypeContext) -> Result<types::EnumerationVariant, String> {
        Ok(types::EnumerationVariant {
            name: AsRef::<str>::as_ref(&self.name).to_string(),
            fields: self
                .fields
                .iter()
                .map(|f| f.as_basic_type(context))
                .collect::<Result<_, _>>()?,
            index: (self.index as u32).into(),
        })
    }
}

impl AsBasicType for TypeDefArray<PortableForm> {
    type BasicType = types::TypeDefArray;

    fn as_basic_type(&self, context: TypeContext) -> Result<types::TypeDefArray, String> {
        Ok(types::TypeDefArray {
            len: self.len,
            type_param: self.type_param.as_basic_type_ref(context)?,
        })
    }
}

impl AsBasicType for TypeDefBitSequence<PortableForm> {
    type BasicType = types::TypeDefBitSequence;

    fn as_basic_type(&self, context: TypeContext) -> Result<types::TypeDefBitSequence, String> {
        let mut collector = CollectPrimitives::default();
        collector.visit_type(context, context.get_type(self.bit_store_type.id)?)?;

        let num_bytes = if collector.found.len() == 1 {
            match &collector.found[0] {
                TypeDefPrimitive::U8 => 1,
                TypeDefPrimitive::U16 => 2,
                TypeDefPrimitive::U32 => 4,
                TypeDefPrimitive::U64 => 8,
                p => {
                    return Err(format!(
                        "Invalid primitive type {p:?} as store type for `BitSequence`: {self:?}"
                    ))
                }
            }
        } else {
            return Err(format!("Only expected to find `1` primitive type as store type for `BitSequence`: {self:?}"));
        };

        Ok(types::TypeDefBitSequence {
            num_bytes,
            least_significant_bit_first: *context
                .get_type(self.bit_order_type.id)?
                .path
                .segments
                .iter()
                .find(|p| **p == "Lsb0" || **p == "Msb0")
                .ok_or("`bit_order_type` should be either `Lsb0` or `Msb0`")?
                == "Lsb0",
        })
    }
}

#[derive(Default)]
struct CollectPrimitives {
    found: Vec<TypeDefPrimitive>,
    already_visited: BTreeSet<u32>,
}

impl Visitor for CollectPrimitives {
    fn visit_primitive(&mut self, primitive: &scale_info::TypeDefPrimitive) {
        self.found.push(primitive.clone());
    }

    fn already_visited(&mut self, id: u32) -> bool {
        !self.already_visited.insert(id)
    }
}

trait Visitor {
    fn visit_type_def(
        &mut self,
        context: TypeContext,
        type_def: &TypeDef<PortableForm>,
    ) -> Result<(), String> {
        visit_type_def(self, context, type_def)
    }

    fn visit_type(&mut self, context: TypeContext, ty: &Type<PortableForm>) -> Result<(), String> {
        visit_type(self, context, ty)
    }

    fn visit_primitive(&mut self, _primitive: &scale_info::TypeDefPrimitive) {}

    fn already_visited(&mut self, id: u32) -> bool;
}

fn visit_type<V: Visitor + ?Sized>(
    visitor: &mut V,
    context: TypeContext,
    ty: &Type<PortableForm>,
) -> Result<(), String> {
    visitor.visit_type_def(context, &ty.type_def)
}

fn visit_type_def<V: Visitor + ?Sized>(
    visitor: &mut V,
    context: TypeContext,
    type_def: &TypeDef<PortableForm>,
) -> Result<(), String> {
    match type_def {
        TypeDef::Variant(v) => {
            v.variants.iter().try_for_each(|v| {
                for f in &v.fields {
                    if !visitor.already_visited(f.ty.id) {
                        visitor.visit_type(context, context.get_type(f.ty.id)?)?
                    }
                }

                Ok::<_, String>(())
            })?;
        }
        TypeDef::Array(a) => {
            if !visitor.already_visited(a.type_param.id) {
                visitor.visit_type(context, context.get_type(a.type_param.id)?)?
            }
        }
        TypeDef::Composite(c) => {
            c.fields.iter().try_for_each(|f| {
                if !visitor.already_visited(f.ty.id) {
                    visitor.visit_type(context, context.get_type(f.ty.id)?)?
                }

                Ok::<_, String>(())
            })?;
        }
        TypeDef::Sequence(s) => {
            if !visitor.already_visited(s.type_param.id) {
                visitor.visit_type(context, context.get_type(s.type_param.id)?)?
            }
        }
        TypeDef::Tuple(t) => t.fields.iter().try_for_each(|t| {
            if !visitor.already_visited(t.id) {
                visitor.visit_type(context, context.get_type(t.id)?)?
            }

            Ok::<_, String>(())
        })?,
        TypeDef::Compact(c) => {
            if !visitor.already_visited(c.type_param.id) {
                visitor.visit_type(context, context.get_type(c.type_param.id)?)?
            }
        }
        TypeDef::Primitive(p) => visitor.visit_primitive(p),
        TypeDef::BitSequence(b) => {
            if !visitor.already_visited(b.bit_order_type.id) {
                visitor.visit_type(context, context.get_type(b.bit_order_type.id)?)?
            }

            if !visitor.already_visited(b.bit_store_type.id) {
                visitor.visit_type(context, context.get_type(b.bit_store_type.id)?)?
            }
        }
    }

    Ok(())
}

impl AsBasicType for frame_metadata::v15::ExtrinsicMetadata<PortableForm> {
    type BasicType = types::ExtrinsicMetadata;

    fn as_basic_type(&self, context: TypeContext) -> Result<types::ExtrinsicMetadata, String> {
        Ok(types::ExtrinsicMetadata {
            version: self.version,
            address_ty: self.address_ty.as_basic_type_ref(context)?,
            call_ty: self.call_ty.as_basic_type_ref(context)?,
            signature_ty: self.signature_ty.as_basic_type_ref(context)?,
            signed_extensions: self
                .signed_extensions
                .iter()
                .map(|se| se.as_basic_type(context))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl AsBasicType for SignedExtensionMetadata<PortableForm> {
    type BasicType = types::SignedExtensionMetadata;

    fn as_basic_type(
        &self,
        context: TypeContext,
    ) -> Result<types::SignedExtensionMetadata, String> {
        Ok(types::SignedExtensionMetadata {
            identifier: AsRef::<str>::as_ref(&self.identifier).to_string(),
            included_in_extrinsic: self.ty.as_basic_type_ref(context)?,
            included_in_signed_data: self.additional_signed.as_basic_type_ref(context)?,
        })
    }
}
