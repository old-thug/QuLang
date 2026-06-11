use qu_ast::{expr::{CallArgument, ExprData, ExprRef}, stmt::StmtData};
use qu_common::extract;
use qu_entities::layout::TypeKind;

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
            ExprData::Call(_) => self.lower_call(expr, builder),
            ExprData::BinaryOperation(_) => self.lower_binop(expr, builder),
            _ => todo!("{:?}", expr.data()),
        }
    }

    pub(crate) fn lower_block(&mut self, expr: &ExprRef, builder: &mut IrBuilder) -> Option<ValueRef> {
        extract!(expr.data(), ExprData::Block(block));
        for stmt in &block.stmts {
            match stmt.data() {
                StmtData::Return(ret) => {
                    let ret_val = self.lower_expression(&ret.expr, builder)?;
                    builder.create_ret(ret_val)?;
                    return None;
                },
                _ => {
                    self.lower_statement(stmt, builder, false)?;
                }
            }
        }
        Some(builder.create_void())
    }

    pub(crate) fn lower_identifier(&mut self, expr: &ExprRef, builder: &mut IrBuilder) -> Option<ValueRef> {
        extract!(expr.data(), ExprData::Identifier(name));
        match builder.peek_current_function() {
            Some(function) => {
                if let Some(ref foo) = function.local_storage.get(name) {
                    return Some(**foo);
                }
            },
            None => (),
        }

        let Some((idx, _)) = self.ir_module.globals.iter().enumerate().find(|(idx, gb)| {
            match gb {
                GlobalValue::Constant(name_) if name_ == name => return true,
                GlobalValue::Function(function) => {
                    if function.get_name() == name { return true }
                }
                _ => (),
            }
            false
        }) else {
            todo!("cannot find name : {name}");
        };
        Some(builder.create_or_get_value(Value::RefGlobal(GlobalId(idx))))
    }

    pub(crate) fn lower_call(&mut self, expr: &ExprRef, builder: &mut IrBuilder) -> Option<ValueRef> {
        extract!(expr.data(), ExprData::Call(call));
        let type_id     = self.module.get_types().get_by_id_key(&expr.span)?;
        let type_layout = &self.module.get_types().get_pool()[type_id];
        let ir_type     = self.lower_type(&type_layout);
        let callee      = self.lower_expression(&call.callee, builder)?;
        let callee_type_id = self.module.get_types().get_by_id_key(&call.callee.span)?;
        let callee_type_layout = &self.module.get_types().get_pool()[callee_type_id];
        extract!(&callee_type_layout.kind, TypeKind::Function { return_type, parameter_names, parameter_types });
        let mut arguments   = Vec::with_capacity(parameter_types.len());
        for argument in &call.arguments {
            match argument {
                CallArgument::Assign(name, expr) => {
                    let value_ref = self.lower_expression(&expr, builder)?;
                    if let Some((name, idx)) = parameter_names.iter().find(|(name, _)| name.value == name.value) {
                        arguments.insert(*idx, value_ref);
                    } else {
                        unreachable!();
                    }
                },
                CallArgument::Expr(value) => {
                    let value_ref = self.lower_expression(&value, builder)?;
                    arguments.push(value_ref);
                }
            }
        }
        builder.create_call(ir_type, callee, arguments)
    }

    pub(crate) fn lower_binop(&mut self, expr: &ExprRef, builder: &mut IrBuilder) -> Option<ValueRef> {
        extract!(expr.data(), ExprData::BinaryOperation(binop));
        let resulting_type = self.module.get_types().get_from_key(&expr.span)?;
        let ir_type        = self.lower_type(resulting_type);
        let lhs = self.lower_expression(&binop.left, builder)?;
        let rhs = self.lower_expression(&binop.right, builder)?;
        builder.create_binop(ir_type, lhs, rhs, binop.operator.clone())
    }
}
