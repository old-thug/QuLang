use qu_diagnostics::span::Span;

use crate::{Name, generics::Generics};

#[derive(Debug, Clone)]
pub struct Type {
    span: Span,
    data: TypeData,
    mutability: Mutability,
}

#[derive(Debug, Clone, Default)]
pub enum Mutability {
    Mutable,
    Immutable,
    #[default]
    ImplicitlyImmutable,
}

pub type TypeRef = Box<Type>;

#[derive(Debug, Clone)]
pub enum TypeData {
    SignedInteger(IntegerWidth),
    UnsignedInteger(IntegerWidth),
    String,
    Char,
    Bool,
    Void,
    Pointer(TypeRef),
    Reference(TypeRef),
    Array(Array),
    Enum(Enum),
    Record(Record),
    Named(Name),
}

#[derive(Debug, Clone)]
pub enum IntegerWidth {
    Int8,
    Int16,
    Int32,
    Int64,
}

#[derive(Debug, Clone)]
pub struct Array {
    count: usize,
    elem_type: TypeRef,
}

#[derive(Debug, Clone)]
pub struct EnumPayload {
    is_record: bool,
    data: TypeRef,
}

#[derive(Debug, Clone)]
pub struct EnumField {
    name: Name,
    payload: Option<EnumPayload>,
}

// type Boolean =
//     True
//     | False
//     ;
//
// type Expr =
//      Int(i64)
//      | String(string)
//      | Char(char)
//      | Bool(bool)
//      | Call {
//          callee: *const Expr,
//          args: []*Expr,
//      }
//      ;
#[derive(Debug, Clone)]
pub struct Enum {
    generics: Generics,
    fields: Vec<EnumField>,
}

#[derive(Debug, Clone)]
pub struct Record {
    generics: Generics,
    fields: Vec<(Name, TypeRef)>,
}

impl Type {
    pub fn new(span: Span, data: TypeData, mutability: Mutability) -> Box<Self> {
        Box::new(Self { span, data, mutability })
    }
}