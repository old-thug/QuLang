use std::collections::HashMap;

use qu_common::Storage;

use crate::{
    GlobalId,
    instruction::{self, Instruction},
    irtype::IrType,
    value::{Value, ValueRef},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct LocalId(pub usize);

pub type FunctionId = GlobalId;

pub type LocalStorage = HashMap<String, ValueRef>;

#[derive(Debug, Clone, Copy)]
pub enum IrCallAbi {
    C,
    System,
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub type_: IrType,
    pub local_id: usize,
    pub local_storage: LocalStorage,
    pub instructions: Vec<Instruction>,
    pub is_external: bool,
    pub abi: IrCallAbi,
}

impl IrFunction {
    pub fn new(name: String, type_: IrType, is_external: bool, abi: IrCallAbi) -> Self {
        Self {
            name,
            type_,
            local_id: 0,
            local_storage: LocalStorage::new(),
            instructions: Vec::new(),
            is_external,
            abi,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        if !self.is_external {
            self.instructions.push(instruction);
        }
    }
}
