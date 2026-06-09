use qu_ast::type_hint::{self, IntegerWidth, TypeRef};
use qu_entities::layout::{self, TypeKind, TypeLayout};
use qu_common::extract;

use super::TypeChecker;

impl<'a> TypeChecker<'a> {
    pub(super) const TYPEID_CHAR: layout::TypeId = layout::TypeId(0);
    pub(super) const TYPEID_STRING: layout::TypeId = layout::TypeId(1);
    pub(super) const TYPEID_BOOL: layout::TypeId = layout::TypeId(2);
    pub(super) const TYPEID_I8: layout::TypeId = layout::TypeId(3);
    pub(super) const TYPEID_I16: layout::TypeId = layout::TypeId(4);
    pub(super) const TYPEID_I32: layout::TypeId = layout::TypeId(5);
    pub(super) const TYPEID_I64: layout::TypeId = layout::TypeId(6);
    pub(super) const TYPEID_U8: layout::TypeId = layout::TypeId(7);
    pub(super) const TYPEID_U16: layout::TypeId = layout::TypeId(8);
    pub(super) const TYPEID_U32: layout::TypeId = layout::TypeId(9);
    pub(super) const TYPEID_U64: layout::TypeId = layout::TypeId(10);
    pub(super) const TYPEID_F32: layout::TypeId = layout::TypeId(11);
    pub(super) const TYPEID_F64: layout::TypeId = layout::TypeId(12);
    /// KEEP This as the last TypeId
    pub(super) const TYPEID_VOID: layout::TypeId = layout::TypeId(13);

    pub(super) fn register_builtin_types(&mut self) {
        let builtins = [
            (Self::TYPEID_CHAR.0, TypeKind::Char),
            (Self::TYPEID_STRING.0, TypeKind::String),
            (Self::TYPEID_BOOL.0, TypeKind::Bool),
            (Self::TYPEID_I8.0, TypeKind::I8),
            (Self::TYPEID_I16.0, TypeKind::I16),
            (Self::TYPEID_I32.0, TypeKind::I32),
            (Self::TYPEID_I64.0, TypeKind::I64),
            (Self::TYPEID_U8.0, TypeKind::U8),
            (Self::TYPEID_U16.0, TypeKind::U16),
            (Self::TYPEID_U32.0, TypeKind::U32),
            (Self::TYPEID_U64.0, TypeKind::U64),
            (Self::TYPEID_F32.0, TypeKind::F32),
            (Self::TYPEID_F64.0, TypeKind::F64),
            (Self::TYPEID_VOID.0, TypeKind::Void),
        ];

        let pool = self.module.get_types_mut().get_pool_mut();
        pool.reserve(Self::TYPEID_VOID.0 + 1);

        for (id, kind) in builtins {
            pool.insert(id, TypeLayout::new_builtin(kind));
        }
    }

    pub(super) fn get_pointer_type_id(&mut self, type_hint: &TypeRef) -> layout::TypeId {
        extract!(type_hint.data, type_hint::TypeData::Pointer(ref inner));
        let inner_type_id = self.resolve_type_to_id(&inner);
        if let Some((id, _)) = self.module.get_types().get_pool().iter().enumerate().find(|(_, t)| {
            if matches!(&t.kind, TypeKind::Pointer(inner_type_id)) {
                return true;
            }
            return false;
        }) {
            return layout::TypeId(id);
        }

        let new_id = self.module.get_types().get_pool().len();
        self.module.get_types_mut()
            .get_pool_mut()
            .push(TypeLayout::new(TypeKind::Pointer(inner_type_id), None));
        layout::TypeId(new_id)
    }

    pub(super) fn resolve_type_to_id(&mut self, type_hint: &TypeRef) -> layout::TypeId {
        match type_hint.data {
            type_hint::TypeData::UnsignedInteger(IntegerWidth::Int8) => Self::TYPEID_U8,
            type_hint::TypeData::UnsignedInteger(IntegerWidth::Int16) => Self::TYPEID_U16,
            type_hint::TypeData::UnsignedInteger(IntegerWidth::Int32) => Self::TYPEID_U32,
            type_hint::TypeData::UnsignedInteger(IntegerWidth::Int64) => Self::TYPEID_U64,
            type_hint::TypeData::SignedInteger(IntegerWidth::Int8) => Self::TYPEID_I8,
            type_hint::TypeData::SignedInteger(IntegerWidth::Int16) => Self::TYPEID_I16,
            type_hint::TypeData::SignedInteger(IntegerWidth::Int32) => Self::TYPEID_I32,
            type_hint::TypeData::SignedInteger(IntegerWidth::Int64) => Self::TYPEID_I64,
            type_hint::TypeData::Bool => Self::TYPEID_BOOL,
            type_hint::TypeData::Void => Self::TYPEID_VOID,
            type_hint::TypeData::Char => Self::TYPEID_CHAR,
            type_hint::TypeData::String => Self::TYPEID_STRING,
            type_hint::TypeData::Pointer(_) => self.get_pointer_type_id(&type_hint),
            _ => todo!(),
        }
    }
}
