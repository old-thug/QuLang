use std::collections::HashSet;

use qu_ast::stmt::{self, StmtRef};
use qu_diagnostics::{Diagnostic, Label, Severity};
use qu_entities::symbol::{ParameterNames, Symbol, SymbolData};
use qu_common::extract;

use crate::symbol_analyzer::SymbolAnalyzer;

impl<'a> SymbolAnalyzer<'a> {
    pub(super) fn collect_stmt(&mut self, stmt: &StmtRef) -> Option<()> {
        match stmt.data() {
            stmt::StmtData::FunctionDefinition(_) => self.collect_function(stmt),
            stmt::StmtData::VariableDecl(_) => todo!("global variable declaration"),
            _ => todo!(),
        }
    }

    pub(super) fn collect_function(&mut self, stmt: &StmtRef) -> Option<()> {
        extract!(stmt.data(), stmt::StmtData::FunctionDefinition(function));
        let current_scope_id = self.current_scope_id();
        {
            let function_name = &function.name;
            if self
                .find_id_in_scope_and(current_scope_id, &function_name.value, |this, id| {
                    let first_defined = this.get_symbol(id).map(|i| i.defined_at);
                    super::error::redefinition_of_symbol(
                        this,
                        function_name.clone(),
                        first_defined,
                    );
                    Some(())
                })
                .is_some()
            {
                return None;
            }

            let mut parameter_names = ParameterNames::new();
            for (index, param) in function.prototype.parameters.iter().enumerate() {
                parameter_names.push((param.name.clone(), index));
            }

            self.add_new_empty_symbol_to_scope(
                function_name.clone(),
                current_scope_id,
                SymbolData::new_function_data(parameter_names, Vec::new()),
            );
        }

        // Check the function body.
        Some(())
    }
}
