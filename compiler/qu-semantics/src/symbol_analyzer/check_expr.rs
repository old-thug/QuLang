use std::ops::Mul;

use qu_ast::expr::{self, BinaryOperator, CallArgument, ExprRef, UnaryOperator};
use qu_diagnostics::{Diagnostic, Severity};
use qu_entities::scope::ScopeFlag;
use qu_span::Span;
use qu_common::extract;

use crate::symbol_analyzer::check_stmt;

use super::SymbolAnalyzer;

impl<'a> SymbolAnalyzer<'a> {
    pub(super) fn check_expression(&mut self, expr: &ExprRef) -> Option<()> {
        match expr.data() {
            expr::ExprData::Block(_) => self.check_block_expr(expr),
            expr::ExprData::Identifier(_) => self.check_identifier(expr),
            expr::ExprData::Integer(_) => Some(()),
            expr::ExprData::Call(_) => self.check_call(expr),
            expr::ExprData::BinaryOperation(_) => self.check_binop(expr),
            _ => todo!("{:?}", expr.data()),
        }
    }

    pub(super) fn check_block_expr(&mut self, expr: &ExprRef) -> Option<()> {
        extract!(expr.data(), expr::ExprData::Block(block));
        self.check_block_expr_(expr.span, block)
    }

    pub(super) fn check_block_expr_(&mut self, span: Span, block: &expr::Block) -> Option<()> {
        // Enter new lexical scope
        self.enter_new_scope(ScopeFlag::Plain);
        let current_scope_id = self.current_scope_id();
        self.map_locus_to_scope(span, current_scope_id);
        for stmt in &block.stmts {
            self.check_statement(stmt)?;
        }
        self.leave_scope();
        Some(())
    }

    pub(super) fn check_identifier(&mut self, expr: &ExprRef) -> Option<()> {
        extract!(expr.data(), expr::ExprData::Identifier(name));
        match self.get_symbol_id_from_name(name.to_string()) {
            Some(sym_id) => {
                self.map_locus_to_symbol(expr.span, sym_id);
            }
            None => {
                self.emit_diag(Diagnostic::new(
                    Severity::Error,
                    format!("undefined symbol"),
                    expr.span(),
                    format!("cannot find symbol named `{}` in current scope", name),
                ));
                return None;
            },
        }
        Some(())
    }

    pub(super) fn check_call(&mut self, expr: &ExprRef) -> Option<()> {
        extract!(expr.data(), expr::ExprData::Call(call));
        self.check_expression(&call.callee)?;
        for arg in &call.arguments {
            match arg {
                CallArgument::Assign(_name, value) => {
                    // unfortunately we can't verify `_name` now
                    // since we have no way of linking the callee back to it's orginal signature
                    self.check_expression(value)?;
                },
                CallArgument::Expr(value) => {
                    self.check_expression(value)?;
                }
            }
        }
        Some(())
    }

    pub(super) fn check_binop(&mut self, expr: &ExprRef) -> Option<()> {
        extract!(expr.data(), expr::ExprData::BinaryOperation(binop));
        self.check_expression(&binop.left)?;
        self.check_expression(&binop.right)?;
        Some(())
    }
}
