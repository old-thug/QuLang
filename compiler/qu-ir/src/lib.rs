#![allow(unused)]
use std::alloc::GlobalAlloc;

use builder::IrBuilder;
use global::GlobalValue;
use value::{Value, ValueRef};
pub mod lower;
mod builder;
pub mod function;
pub mod global;
pub mod instruction;
pub mod irtype;
pub mod value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlobalId(pub usize);

#[derive(Debug, Clone)]
pub struct IrModule {
    name: String,
    globals: Vec<GlobalValue>,
    value_pool: Vec<Value>,
    constant_void_value: ValueRef,
}

impl IrModule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            globals: Vec::new(),
            value_pool: vec![Value::Unit],
            constant_void_value: ValueRef(0),
        }
    }

    pub fn get_builder<'a>(&'a mut self) -> IrBuilder {
        IrBuilder::new(self)
    }

    pub fn get_globals(&self) -> &Vec<GlobalValue> {
        &self.globals
    }

    pub fn value_pool(&self) -> &[Value] {
        &self.value_pool
    }
}
