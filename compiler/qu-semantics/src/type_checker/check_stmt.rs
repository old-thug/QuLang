use qu_ast::{stmt::{self, StmtRef}, type_hint};

use crate::{extract, type_checker::{layout::TypeKind, type_coercer::TypeCoercer}};

use super::TypeChecker;

impl<'a> TypeChecker<'a> {
    pub(super) fn check_statement(&mut self, stmt: &StmtRef) -> Option<()> {
        match stmt.data() {
            stmt::StmtData::FunctionDefinition(_) => self.check_function(stmt),
            stmt::StmtData::Return(_) => self.check_return(stmt),
            stmt::StmtData::VariableDecl(_) => self.check_vardecl(stmt),
            _ => todo!("{:?}", stmt.data()),
        }
    }

    pub(super) fn check_function(&mut self, stmt: &StmtRef) -> Option<()> {
        extract!(stmt.data(), stmt::StmtData::FunctionDefinition(function));
        let return_type_id = match function.prototype.return_type {
            Some(ref type_hint) => self.resolve_type_to_id(type_hint),
            None => Self::TYPEID_VOID,
        };
        let mut parameter_types = Vec::new();
        for param in &function.prototype.parameters {
            let param_type_id = match (&param.type_hint, &param.default_value) {
                (Some(type_hint), None) => self.resolve_type_to_id(&type_hint),
                (None, Some(expression)) => self.check_expression(&expression)?,
                (Some(type_hint), Some(expression)) => {
                    let target_type_id = self.resolve_type_to_id(type_hint);
                    let source_type_id = self.check_expression(expression)?;
                    self.try_coerce((type_hint.span, target_type_id), (expression.span, source_type_id))?;
                    target_type_id
                },
                _ => unreachable!(),
            };
            let symbol = self.symbols.get_from_key_mut(&param.name.span)?;
            symbol.resolved_type = Some(param_type_id);
            parameter_types.push(param_type_id);
        }
        let function_type_id = self.get_type_id_of_kind(TypeKind::Function {
            parameter_types,
            return_type: return_type_id,
        });
        let sym = self.symbols.get_from_key_mut(&function.name.span)?;
        sym.resolved_type = Some(function_type_id);

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
                self.try_coerce((type_hint.span, target_type_id), (vardecl.initializer.span, source_type_id))?;
                target_type_id
            }
            None => {
                source_type_id
            },
        };
        let symbol = self.symbols.get_from_key_mut(&vardecl.name.span)?;
        symbol.resolved_type = Some(type_id);
        self.map_locus_to_type(vardecl.name.span, &type_id);
        Some(())
    }
}
