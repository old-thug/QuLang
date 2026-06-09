use crate::{GlobalId, function::LocalId};

#[derive(Debug, Clone, Copy)]
pub struct ValueRef(pub usize);

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    ConstantInt(i64),
    ConstantString(String),
    True,
    False,
    Unit,
    Ref(LocalId),
    RefParam(usize),
    RefGlobal(GlobalId),
}
