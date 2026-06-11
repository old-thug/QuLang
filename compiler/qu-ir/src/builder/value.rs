use crate::{GlobalId, function::LocalId, global::GlobalValue, instruction::{Instruction, InstructionKind}, irtype::IrType, value::{Value, ValueRef}};

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

    // TODO: move to super::instruction
    pub fn create_ret(&mut self, value: ValueRef) -> Option<()> {
        self.add_instruction(Instruction(
            None,
            InstructionKind::Return(value)
        ))
    }

    pub(crate) fn create_temporary(&mut self, irtype: IrType) -> Option<ValueRef> {
        let current_function = self.peek_current_function_mut()?;
        let new_id   = current_function.local_id;
        current_function.local_id += 1;
        let dst    = self.create_or_get_value(Value::Ref(LocalId(new_id)));
        let current_function = self.peek_current_function_mut()?;
        current_function.add_instruction(Instruction(Some(dst), InstructionKind::Alloca { type_: irtype }));
        Some(dst)
    }

    pub(crate) fn create_global(&mut self, name: String, value: GlobalValue) -> Option<ValueRef> {
        for (idx, glob) in unsafe { &*self.module }.get_globals().iter().enumerate() {
            match glob {
                value => return Some(self.create_or_get_value(Value::RefGlobal(crate::GlobalId(idx)))),
                _ => (),
            }
        }
        let id = unsafe { &mut *self.module }.get_globals().len();
        unsafe { &mut *self.module }.globals.push(value);
        return Some(self.create_or_get_value(Value::RefGlobal(GlobalId(id))));
    }
}
