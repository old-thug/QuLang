use qu_ast::{expr::{ExprData, ExprRef}, stmt::StmtData};
use qu_common::extract;

use crate::{GlobalId, builder::IrBuilder, global::GlobalValue, value::{Value, ValueRef}};

use super::IrLowerer;


impl<'a> IrLowerer<'a> {
    pub(crate) fn lower_expression(&mut self, expr: &ExprRef, builder: &mut IrBuilder) -> Option<ValueRef> {
        match expr.data() {
            ExprData::Block(_) => self.lower_block(expr, builder),
            ExprData::Identifier(_) => self.lower_identifier(expr, builder),
            ExprData::Integer(value) => {
                Some(builder.create_constant_int(*value))
            },
            _ => todo!("{:?}", expr.data()),
        }
    }

    pub(crate) fn lower_block(&mut self, expr: &ExprRef, builder: &mut IrBuilder) -> Option<ValueRef> {
        extract!(expr.data(), ExprData::Block(block));
        for stmt in &block.stmts {
            match stmt.data() {
                StmtData::Return(ret) => {
                    return self.lower_expression(&ret.expr, builder);
                },
                _ => {
                    self.lower_statement(stmt, builder)?;
                }
            }
        }
        Some(builder.create_void())
    }

    pub(crate) fn lower_identifier(&mut self, expr: &ExprRef, builder: &mut IrBuilder) -> Option<ValueRef> {
        extract!(expr.data(), ExprData::Identifier(name));
        match builder.peek_current_function() {
            Some(function) => {
                function.local_storage.get(name).copied()
            },
            None => {
                let Some((idx, _)) = self.ir_module.globals.iter().enumerate().find(|(idx, gb)| {
                    match gb {
                        GlobalValue::Constant(name_) if name_ == name => true,
                        GlobalValue::Function(function) if function.get_name() == name => true,
                        _ => false,
                    }
                }) else {
                    todo!();
                };
                Some(builder.create_or_get_value(Value::RefGlobal(GlobalId(idx))))
            },
        }
    }
}
