use crate::{instruction::{Instruction, InstructionKind}, value::{Value, ValueRef}};

use super::IrBuilder;

impl IrBuilder {
    pub fn create_void(&self) -> ValueRef {
        unsafe { &*self.module }.constant_void_value
    }

    pub fn create_or_get_value(&mut self, value: Value) -> ValueRef {
        let module = unsafe { &mut *self.module };
        for (i, value_) in module.value_pool.iter().enumerate() {
            if *value_ == value {
                return ValueRef(i);
            }
        }
        let new_id = module.value_pool.len();
        module.value_pool.push(value);
        ValueRef(new_id)
    }

    pub fn create_constant_int(&mut self, value: i64) -> ValueRef {
        self.create_or_get_value(Value::ConstantInt(value))
    }

    pub fn create_constant_string(&mut self, value: String) -> ValueRef {
        self.create_or_get_value(Value::ConstantString(value))
    }

    pub fn create_ret(&mut self, value: ValueRef) -> Option<()> {
        self.add_instruction(Instruction(
            None,
            InstructionKind::Return(value)
        ))
    }
}
