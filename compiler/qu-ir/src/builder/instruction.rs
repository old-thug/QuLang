use qu_ast::type_hint;

use crate::{instruction::{Instruction, InstructionKind}, irtype::IrType, value::{Value, ValueRef}};

use super::IrBuilder;

impl IrBuilder {
    pub(crate) fn create_alloca(&mut self, name: String, type_hint: IrType) -> Option<ValueRef> {
        let current_function = self.peek_current_function_mut()?;
        let new_local_id     = current_function.local_id;
        current_function.local_id += 1;
        let dst = self.create_or_get_value(Value::Ref(crate::function::LocalId(new_local_id)));
        // arggggghhhh..... rust!
        let current_function = self.peek_current_function_mut()?;
        current_function.local_storage.insert(name, dst);
        self.add_instruction(Instruction(
            Some(dst),
            InstructionKind::Alloca { type_: type_hint }
        ))?;
        Some(dst)
    }

    pub(crate) fn create_store(&mut self, dst: ValueRef, src: ValueRef) -> Option<()> {
        self.add_instruction(Instruction(Some(dst), InstructionKind::Store(src)))
    }
}
