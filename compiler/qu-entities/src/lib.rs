#![allow(unused)]
pub mod layout;
pub mod scope;
pub mod symbol;
pub use layout as type_layout;
use layout::TypeLayout;
use qu_common::Storage;
use qu_span::Span;

pub type TypeStorage = Storage<Span, TypeLayout>;
