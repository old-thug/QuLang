use qu_ast::expr::{self, ExprRef};
use qu_diagnostics::span::Span;

use crate::{extract, symbol_analyzer::check_stmt};

use super::{SymbolAnalyzer, scope::ScopeFlag};


impl SymbolAnalyzer {
    pub(super) fn check_expression(&mut self, expr: &ExprRef) -> Option<()> {
        match expr.data() {
            expr::ExprData::Block(_) => self.check_block_expr(expr),
            expr::ExprData::Identifier(_) => self.check_identifier(expr),
            expr::ExprData::Integer(_) => Some(()),
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
            },
            None => todo!(),
        }
        Some(())
    }
}
