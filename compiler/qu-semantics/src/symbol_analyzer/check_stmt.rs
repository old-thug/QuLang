use std::collections::{HashMap, HashSet};

use qu_ast::{
    Ast,
    stmt::{self, StmtRef},
};
use qu_common::extract;

use crate::symbol_analyzer;
use qu_entities::{scope::{ScopeFlag, ScopeId}, symbol::{ParameterNames, SymbolData}};
use qu_entities::symbol::Symbol;

use super::SymbolAnalyzer;

impl<'a> SymbolAnalyzer<'a> {
    pub(super) fn check_statements(&mut self, ast: &Ast) -> Option<()> {
        for stmt in ast {
            self.check_statement(stmt)?;
        }
        Some(())
    }

    pub(super) fn check_statement(&mut self, stmt: &StmtRef) -> Option<()> {
        match stmt.data() {
            stmt::StmtData::FunctionDefinition(_) => self.check_function(stmt),
            stmt::StmtData::VariableDecl(_) => self.check_vardecl(stmt),
            stmt::StmtData::Return(_) => self.check_return(stmt),
            stmt::StmtData::Expr(expr) => self.check_expression(expr),
            _ => todo!("{:?}", stmt.data()),
        }
    }

    pub(super) fn check_function(&mut self, stmt: &StmtRef) -> Option<()> {
        extract!(stmt.data(), stmt::StmtData::FunctionDefinition(function));
        let current_scope_id = self.current_scope_id();
        if current_scope_id != ScopeId(0) {
            // Skip rechecking global variables
            if self
                .find_id_in_scope_and(current_scope_id, &function.name.value, |this, sym| {
                    let first_defined = this.get_symbol(sym).map(|i| i.defined_at);
                    super::error::redefinition_of_symbol(
                        this,
                        function.name.clone(),
                        first_defined,
                    );
                    Some(())
                })
                .is_some()
            {
                return None;
            }

        }
        // Create new scope for function parameters
        self.enter_new_scope(ScopeFlag::Function);
        let new_scope_id = self.current_scope_id();
        let mut parameter_names = ParameterNames::new();
        for (index, param) in function.prototype.parameters.iter().enumerate() {
            self.add_new_empty_symbol_to_scope(
                param.name.clone(),
                new_scope_id,
                SymbolData::new_var_data(new_scope_id)
            );
            parameter_names.push((param.name.clone(), index));
        }

        if current_scope_id != ScopeId(0) {
            self.add_new_empty_symbol_to_scope(
                function.name.clone(),
                current_scope_id,
                SymbolData::new_function_data(parameter_names, Vec::new())
            );
        }

        if let Some(ref body) = function.body {
            // Check function body
            self.check_expression(body)?;
        }
        self.leave_scope();
        Some(())
    }

    pub(super) fn check_return(&mut self, stmt: &StmtRef) -> Option<()> {
        extract!(stmt.data(), stmt::StmtData::Return(ret));
        self.check_expression(&ret.expr)
    }

    pub(super) fn check_vardecl(&mut self, stmt: &StmtRef) -> Option<()> {
        extract!(stmt.data(), stmt::StmtData::VariableDecl(vardecl));
        let current_scope_id = self.current_scope_id();
        // Use the shallow version here to allow shadowing from other scopes.
        // if this finds something, the it is within the same scope.
        if self
            .find_id_in_scope_shallow_and(current_scope_id, &vardecl.name.value, |this, id| {
                let first_defined = this.get_symbol(id).map(|i| i.defined_at);
                super::error::redefinition_of_symbol(this, vardecl.name.clone(), first_defined);
                Some(())
            })
            .is_some()
        {
            return None;
        }
        self.check_expression(&vardecl.initializer)?;
        self.add_new_empty_symbol_to_scope(
            vardecl.name.clone(),
            current_scope_id,
            SymbolData::new_var_data(current_scope_id)
        );
        Some(())
    }
}
