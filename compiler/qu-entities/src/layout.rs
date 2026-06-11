use std::collections::HashSet;

use qu_ast::Name;
use qu_span::Span;

use crate::symbol::ParameterNames;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TypeId(pub usize);

#[derive(Debug, Clone)]
pub struct TypeLayout {
    pub kind: TypeKind,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeKind {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Usize,
    Isize,
    String,
    Char,
    Bool,
    Void,
    Tuple(Vec<TypeId>),
    Array {
        count: usize,
        type_hint: TypeId,
    },
    Pointer(TypeId),
    Placeholder(Option<TypeId>),
    Named(String),
    Function {
        return_type: TypeId,
        parameter_names: ParameterNames,
        parameter_types: Vec<TypeId>,
    },
}

impl TypeLayout {
    pub fn new_builtin(kind: TypeKind) -> Self {
        Self { kind, span: None }
    }

    pub fn new(kind: TypeKind, span: Option<Span>) -> Self {
        Self { kind, span }
    }

    pub fn is_numeric_type(&self) -> bool {
        match self.kind {
            TypeKind::I8
            | TypeKind::I16
            | TypeKind::I32
            | TypeKind::I64
            | TypeKind::U8
            | TypeKind::U32
            | TypeKind::U16
            | TypeKind::U64 => true,
            _ => false,
        }
    }

    pub(crate) fn to_numeric_width(&self) -> (bool, usize) {
        match self.kind {
            TypeKind::I8 => (true, 8),
            TypeKind::I16 => (true, 16),
            TypeKind::I32 => (true, 32),
            TypeKind::I64 => (true, 64),
            TypeKind::U8 => (false, 8),
            TypeKind::U16 => (false, 16),
            TypeKind::U32 => (false, 32),
            TypeKind::U64 => (false, 64),
            _ => unreachable!(),
        }
    }

    pub fn compare_numeric_widths(&self, source: &TypeLayout) -> i32 {
        let (target_signed, target_width) = self.to_numeric_width();
        let (source_signed, source_width) = source.to_numeric_width();

        // Positive = widening (e.g., 32 - 8 = 24)
        // Negative = narrowing (e.g., 16 - 64 = -48)
        let width_diff = (target_width as i32) - (source_width as i32);

        match (target_signed, source_signed) {
            // Same signedness
            (true, true) | (false, false) => width_diff,

            // Unsigned source to Signed target (e.g., u16 -> i32)
            // You need 1 extra bit in the target to safely hold the unsigned value's MSB without flipping the sign bit.
            (true, false) => width_diff - 1,

            // Signed source to Unsigned target (e.g., i32 -> u32)
            // This is inherently a lossy conversion for negative values,
            // so we penalize it heavily to indicate it's a risky cast.
            (false, true) => {
                if width_diff > 0 {
                    width_diff // It's wider, but still technically a lossy reinterpretation
                } else {
                    width_diff - 1 // Narrower AND lost the sign bit
                }
            }
        }
    }
}
