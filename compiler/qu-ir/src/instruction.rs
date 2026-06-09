use crate::{function::LocalId, irtype::IrType, value::ValueRef};

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
}
