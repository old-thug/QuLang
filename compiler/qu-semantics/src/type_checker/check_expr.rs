use qu_ast::expr::{self, CallArgument, ExprRef};
use qu_diagnostics::{Diagnostic, Severity};
use qu_entities::{layout::TypeKind, type_layout as layout};

use qu_common::extract;

use super::TypeChecker;

impl<'a> TypeChecker<'a> {
    pub(super) fn check_expression(&mut self, expr: &ExprRef) -> Option<layout::TypeId> {
        match expr.data() {
            expr::ExprData::Block(_) => self.check_block_expr(expr),
            expr::ExprData::Identifier(name) => self.check_identifier(expr),
            expr::ExprData::Integer(_) => Some(Self::TYPEID_I32),
            expr::ExprData::Call(_) => self.check_call(expr),
            expr::ExprData::BinaryOperation(_) => self.check_binop(expr),
            _ => todo!("{:?}", expr.data()),
        }
    }

    pub(super) fn check_block_expr(&mut self, expr: &ExprRef) -> Option<layout::TypeId> {
        extract!(expr.data(), expr::ExprData::Block(block));
        for stmt in &block.stmts {
            self.check_statement(stmt)?;
        }
        // Blocks don't actually always return void but yeilding expressions have not been
        // added to the parser yet
        Some(Self::TYPEID_VOID)
    }

    pub(super) fn check_identifier(&mut self, expr: &ExprRef) -> Option<layout::TypeId> {
        extract!(expr.data(), expr::ExprData::Identifier(ident));
        let symbol = self.module.get_symbols().get_from_key(&expr.span)?;
        Some(
            {
                let type_id = symbol
                    .resolved_type
                    .expect(&format!("identifier `{ident}` is awaiting inference"));
                self.module.get_types_mut().map(expr.span(), type_id.0);
                type_id
            }
        )
    }

    pub(super) fn check_call(&mut self, expr: &ExprRef) -> Option<layout::TypeId> {
        extract!(expr.data(), expr::ExprData::Call(call));
        let type_id = self.check_expression(&call.callee)?;
        let type_layout = self.get_type_layout_from_id(&type_id)?.clone(); // TODO: don't clone
        match &type_layout.kind {
            TypeKind::Function { return_type, parameter_names, parameter_types } => {
                let mandatory_parameters = {};
                let mut has_seen_name = false;
                for (index, argument) in call.arguments.iter().enumerate() {
                    match argument {
                        CallArgument::Expr(value) => {
                            if has_seen_name {
                                unreachable!();
                            }
                            let source_id = self.check_expression(value)?;
                            if !self.can_coerce(parameter_types[index], source_id) {
                                let target_name = self.get_type_name(&parameter_types[index]);
                                let source_name = self.get_type_name(&source_id);
                                self.emit_diag(Diagnostic::new(
                                    Severity::Error,
                                    format!("unexpected type"),
                                    value.span,
                                    format!("expected `{}` got `{}`", target_name, source_name),
                                ));
                                return None;
                            }
                        },
                        CallArgument::Assign(name, value) => {
                            if let Some((name, name_to_type_idx)) = parameter_names.iter().find(|(p, i)| p.value == name.value) {
                                let source_type_id = self.check_expression(value)?;
                                self.try_coerce((parameter_names[index].0.span, parameter_types[*name_to_type_idx]), (value.span, source_type_id))?;
                                has_seen_name = true;
                            } else {
                                self.emit_diag(Diagnostic::new(
                                    Severity::Error,
                                    format!("invalid symbol"),
                                    name.span,
                                    format!("function has no parameter named `{}`", name.value),
                                ));
                            }
                        },
                    }
                }
                self.module.get_types_mut().map(expr.span(), return_type.0);
                Some(*return_type)
            },
            _ => {
                let type_name = self.get_type_name(&type_id);
                self.emit_diag(Diagnostic::new(
                    Severity::Error,
                    format!("invalid type"),
                    call.callee.span(),
                    format!("type `{}` is not callable", type_name),
                ));
                return None;
            },
        }
    }

    pub(super) fn check_binop(&mut self, expr: &ExprRef) -> Option<layout::TypeId> {
        extract!(expr.data(), expr::ExprData::BinaryOperation(binop));

        let lhs_type_id = self.check_expression(&binop.left)?;
        let rhs_type_id = self.check_expression(&binop.right)?;

        // TODO: perform deeper analysis on the operation
        self.try_coerce((binop.left.span, lhs_type_id), (binop.right.span, rhs_type_id))?;
        self.module.get_types_mut().map(expr.span, lhs_type_id.0);
        Some(lhs_type_id)
    }
}
