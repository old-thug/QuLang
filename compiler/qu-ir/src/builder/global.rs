use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{
    GlobalId, function::{self, FunctionId, IrCallAbi, IrFunction}, global::GlobalValue, instruction::Instruction, irtype::IrType, value::Value
};

use super::IrBuilder;

impl IrBuilder {
    pub(crate) fn get_global(&self, id: GlobalId) -> Option<&GlobalValue> {
        unsafe { &*self.module }.globals.get(id.0)
    }

    pub(crate) fn get_global_mut(&mut self, id: GlobalId) -> Option<&mut GlobalValue> {
        unsafe { &mut *self.module }.globals.get_mut(id.0)
    }

    pub(crate) fn find_global(&self, name: &str) -> Option<GlobalId> {
        if let Some((id, _)) = unsafe { &*self.module }
        .get_globals()
            .iter()
            .enumerate()
            .find(|(id, p)| {
                match p {
                    GlobalValue::Constant(name_) if name_ == name => return true,
                    GlobalValue::Function(function) if function.get_name() == name => return true,
                    _ => (),
                }
                false
            }) {
                return Some(GlobalId(id));
            }
        None
    }

    pub(crate) fn peek_current_function_mut(&mut self) -> Option<&mut IrFunction> {
        let id = self.function_stack.last()?;
        let global = self.get_global_mut(*id)?;
        if let GlobalValue::Function(function) = global {
            return Some(function);
        }
        return None;
    }

    pub(crate) fn peek_current_function(&self) -> Option<&IrFunction> {
        let id = self.function_stack.last()?;
        let global = self.get_global(*id)?;
        if let GlobalValue::Function(function) = global {
            return Some(function);
        }
        return None;
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> Option<()> {
        let current_function = self.peek_current_function_mut()?;
        current_function.add_instruction(instruction);
        Some(())
    }

    pub fn add_function_parameter(&mut self, index: usize, name: String) -> Option<()> {
        let value = self.create_or_get_value(Value::RefParam(index));
        let current_function = self.peek_current_function_mut()?;
        current_function.local_storage.insert(name, value);
        Some(())
    }

    pub fn new_function(
        &mut self,
        name: String,
        type_: IrType,
        is_external: bool,
        abi: IrCallAbi,
    ) -> GlobalId {
        let module = unsafe { &mut *self.module };
        for (id, global) in module.globals.iter().enumerate() {
            match global {
                GlobalValue::Function(prev) => {
                    if prev.get_name() == name {
                        return GlobalId(id);
                    }
                }
                _ => (),
            }
        }
        let new_function = IrFunction::new(name, type_, is_external, abi);
        module.globals.push(GlobalValue::Function(new_function));
        GlobalId(module.globals.len() - 1)
    }
}
