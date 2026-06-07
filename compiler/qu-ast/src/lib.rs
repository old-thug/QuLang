#![allow(unused)]
use qu_diagnostics::span::Span;

use crate::stmt::StmtRef;

pub mod expr;
pub mod type_hint;
pub mod stmt;
pub mod generics;

#[derive(Debug, Clone)]
pub struct Name {
    pub span: Span,
    pub value: String,
}

pub type Ast = Vec<StmtRef>;

#[derive(Debug, Clone)]
pub enum Visibility {
    // Public every where
    Public,
    // Public within related modules but not globally
    Shared,
    // Completely private
    Private,
}
