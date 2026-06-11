use std::collections::HashSet;

use qu_ast::{
    stmt::{self, StmtRef},
    type_hint,
};
use qu_common::extract;
use qu_entities::{
    layout::TypeKind,
    symbol::{Symbol, SymbolData},
};

use crate::type_checker::type_coercer::TypeCoercer;

use super::TypeChecker;

impl<'a> TypeChecker<'a> {
    pub(super) fn check_statement(&mut self, stmt: &StmtRef) -> Option<()> {
        match stmt.data() {
            stmt::StmtData::FunctionDefinition(_) => self.check_function(stmt),
            stmt::StmtData::Return(_) => self.check_return(stmt),
            stmt::StmtData::VariableDecl(_) => self.check_vardecl(stmt),
            stmt::StmtData::Expr(expr) => {
                self.check_expression(expr)?;
                Some(())
            }
            _ => todo!("{:?}", stmt.data()),
        }
    }

    pub(super) fn check_function(&mut self, stmt: &StmtRef) -> Option<()> {
        extract!(stmt.data(), stmt::StmtData::FunctionDefinition(function));
        let should_check =
            if let Some(sym) = self.module.get_symbols().get_from_key(&function.name.span) {
                sym.resolved_type.is_none()
            } else {
                true
            };
        if should_check {
            self.check_function_(function)?;
        }
        if let Some(ref body) = function.body {
            self.check_expression(body)?;
        }

        Some(())
    }

    pub(super) fn check_return(&mut self, stmt: &StmtRef) -> Option<()> {
        extract!(stmt.data(), stmt::StmtData::Return(ret));
        self.check_expression(&ret.expr)?;
        Some(())
    }

    pub(super) fn check_vardecl(&mut self, stmt: &StmtRef) -> Option<()> {
        extract!(stmt.data(), stmt::StmtData::VariableDecl(vardecl));
        // println!("checking {}", vardecl.name.value);
        let source_type_id = self.check_expression(&vardecl.initializer)?;
        let type_id = match vardecl.type_hint {
            Some(ref type_hint) => {
                let target_type_id = self.resolve_type_to_id(type_hint);
                self.try_coerce(
                    (type_hint.span, target_type_id),
                    (vardecl.initializer.span, source_type_id),
                )?;
                target_type_id
            }
            None => source_type_id,
        };
        let symbol = self
            .module
            .get_symbols_mut()
            .get_from_key_mut(&vardecl.name.span)?;
        symbol.resolved_type = Some(type_id);
        self.map_locus_to_type(vardecl.name.span, &type_id);
        Some(())
    }
}
