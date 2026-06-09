use crate::function::{FunctionId, IrFunction};

#[derive(Debug, Clone, Copy)]
pub struct GlobalId(pub usize);

#[derive(Debug, Clone)]
pub enum GlobalValue {
    Function(IrFunction),
    Constant(String),
}
