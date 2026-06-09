use qu_ast::expr::{self, ExprRef};
use qu_entities::type_layout as layout;

use qu_common::extract;

use super::TypeChecker;

impl<'a> TypeChecker<'a> {
    pub(super) fn check_expression(&mut self, expr: &ExprRef) -> Option<layout::TypeId> {
        match expr.data() {
            expr::ExprData::Block(_) => self.check_block_expr(expr),
            expr::ExprData::Identifier(name) => self.check_identifier(expr),
            expr::ExprData::Integer(_) => Some(Self::TYPEID_I32),
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
        let symbol = self.module.get_symbols_mut().get_from_key(&expr.span)?;
        Some(
            symbol
                .resolved_type
                .expect(&format!("identifier `{ident}` is awaiting inference")),
        )
    }
}
