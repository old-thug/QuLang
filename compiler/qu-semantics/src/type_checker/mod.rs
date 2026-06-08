#![allow(unused)]
mod check_expr;
mod check_stmt;
pub mod layout;
mod resolver;
mod type_coercer;

use std::any::Any;

use layout::{TypeId, TypeLayout};
use qu_ast::{Ast, stmt};
use qu_context::Storage;
use qu_diagnostics::{
    Diagnostic,
    span::{self, Span},
};
use type_coercer::{CoerceResult, TypeCoercer};

use crate::symbol_analyzer::{ScopeStorage, SymbolStorage};

pub type TypeStorage = Storage<Span, TypeLayout>;

#[derive(Debug)]
pub struct TypeChecker<'a> {
    scopes: &'a ScopeStorage,
    symbols: &'a mut SymbolStorage,
    types: TypeStorage,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(scopes: &'a ScopeStorage, symbols: &'a mut SymbolStorage) -> Self {
        Self {
            scopes,
            symbols,
            types: TypeStorage::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn run(&mut self, ast: &Ast) -> Result<TypeStorage, Vec<Diagnostic>> {
        self.register_builtin_types();
        self.collect_types(ast);
        for stmt in ast {
            self.check_statement(stmt);
        }

        if self.diagnostics.len() != 0 {
            Err(std::mem::replace(&mut self.diagnostics, Vec::new()))
        } else {
            Ok(std::mem::replace(&mut self.types, TypeStorage::new()))
        }
    }

    fn collect_types(&mut self, ast: &Ast) -> Option<()> {
        for stmt in ast {
            match stmt.data() {
                stmt::StmtData::TypeDefinition(_) => todo!(),
                _ => {}
            }
        }
        Some(())
    }

    pub(super) fn emit_diag(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    pub(super) fn map_locus_to_type(&mut self, span: Span, type_id: &TypeId) {
        self.types.map(span, type_id.0);
    }

    pub(super) fn get_type_name(&mut self, type_id: &TypeId) -> String {
        let layout = self.get_type_layout_from_id(type_id).unwrap();
        match layout.kind {
            layout::TypeKind::I8 => format!("i8"),
            layout::TypeKind::I16 => format!("i16"),
            layout::TypeKind::I32 => format!("i32"),
            layout::TypeKind::I64 => format!("i64"),
            layout::TypeKind::U8 => format!("u8"),
            layout::TypeKind::U16 => format!("u16"),
            layout::TypeKind::U32 => format!("u32"),
            layout::TypeKind::U64 => format!("u64"),
            layout::TypeKind::String => format!("string"),
            layout::TypeKind::Char => format!("char"),
            layout::TypeKind::Void => format!("void"),
            layout::TypeKind::Bool => format!("bool"),
            _ => todo!(),
        }
    }

    fn get_type_id_of_kind(&mut self, type_kind: layout::TypeKind) -> layout::TypeId {
        for (id, type_layout) in self.types.get_pool().iter().enumerate() {
            if matches!(&type_layout.kind, type_kind) {
                return layout::TypeId(id);
            }
        }
        let new_id = self.types.get_pool().len();
        self.types
            .get_pool_mut()
            .push(TypeLayout::new(type_kind, None));
        layout::TypeId(new_id)
    }

    pub(super) fn get_type_layout_from_id(&self, type_id: &TypeId) -> Option<&TypeLayout> {
        self.types.get_pool().get(type_id.0)
    }

    pub(super) fn try_coerce(
        &mut self,
        target: (Span, TypeId),
        source: (Span, TypeId),
    ) -> Option<()> {
        match TypeCoercer::coerce(self, target.1, source.1) {
            CoerceResult::Identity => Some(()),
            CoerceResult::IntegerWidening => {
                let diag = Diagnostic::new(
                    qu_diagnostics::Severity::Warning,
                    format!("implicit casting"),
                    source.0,
                    format!(
                        "type `{}` is implicitly casted to match `{}`",
                        self.get_type_name(&source.1),
                        self.get_type_name(&target.1)
                    ),
                );
                self.emit_diag(diag);
                Some(())
            }
            CoerceResult::IntegerShrinking => {
                let diag = Diagnostic::new(
                    qu_diagnostics::Severity::Error,
                    format!("type mismatch"),
                    source.0,
                    format!(
                        "cannot implicitly shrink `{}` to fit `{}`",
                        self.get_type_name(&source.1),
                        self.get_type_name(&target.1)
                    ),
                );
                self.emit_diag(diag);
                None
            }
            _ => {
                let diag = Diagnostic::new(
                    qu_diagnostics::Severity::Error,
                    format!("type mismatch"),
                    source.0,
                    format!(
                        "expected type `{}` got value of type `{}`",
                        self.get_type_name(&target.1),
                        self.get_type_name(&source.1)
                    ),
                )
                .with_label(format!("type specified here"), target.0);
                self.emit_diag(diag);
                None
            }
        }
    }
}
