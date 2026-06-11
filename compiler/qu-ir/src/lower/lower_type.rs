use qu_entities::layout::{TypeKind, TypeLayout};

use crate::irtype::IrType;

use super::IrLowerer;


impl<'a> IrLowerer<'a> {
    pub fn lower_type(&mut self, type_layout: &TypeLayout) -> IrType {
        match type_layout.kind {
            TypeKind::I8 => IrType::I8,
            TypeKind::I16 => IrType::I16,
            TypeKind::I32 => IrType::I32,
            TypeKind::I64 => IrType::I64,
            TypeKind::U8 => IrType::U8,
            TypeKind::U16 => IrType::U16,
            TypeKind::U32 => IrType::U32,
            TypeKind::U64 => IrType::U64,
            TypeKind::String => IrType::TypedPointer(Box::new(IrType::Char)),
            TypeKind::Function { return_type, ref parameter_types, .. } => {
                let mut parameters = Vec::new();
                let return_type_layout = &self.module.get_types().get_pool()[return_type.0];
                let return_type = self.lower_type(return_type_layout);
                for param in parameter_types {
                    let param_type_layout = &self.module.get_types().get_pool()[param.0];
                    parameters.push(self.lower_type(param_type_layout));
                }
                IrType::Function { return_type: Box::new(return_type), parameter_type: parameters }
            },
            TypeKind::Void => IrType::Unit,
            TypeKind::Char => IrType::Char,
            TypeKind::Pointer(inner) => {
                let inner_layout = &self.module.get_types().get_pool()[inner.0];
                let inner = self.lower_type(inner_layout);
                return IrType::TypedPointer(Box::new(inner));
            }
            _ => todo!("{:?}", type_layout),
        }
    }
}
