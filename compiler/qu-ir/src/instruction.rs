use qu_ast::expr::BinaryOperator;

use crate::{function::LocalId, irtype::IrType, value::{Value, ValueRef}};

#[derive(Debug, Clone)]
pub struct Instruction(
    /* dest */ pub Option<ValueRef>,
    /* inst */ pub InstructionKind,
);

#[derive(Debug, Clone)]
pub enum InstructionKind {
    Alloca {
        type_: IrType,
        /* align: usize */
    },
    Store(/* src */ ValueRef),
    LoadLocal(/* id */ LocalId),
    Return(/* value */ ValueRef),
    Call {
        callee: ValueRef,
        args: Vec<ValueRef>,
    },
    Binop {
        op: BinaryOperator,
        lhs: ValueRef,
        rhs: ValueRef,
    }
}
