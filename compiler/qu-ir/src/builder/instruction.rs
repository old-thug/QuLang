use qu_ast::{expr::BinaryOperator, type_hint};

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

    pub(crate) fn create_call(&mut self, irtype: IrType, callee: ValueRef, args: Vec<ValueRef>) -> Option<ValueRef> {
        let dst = self.create_temporary(irtype)?;
        self.peek_current_function_mut()?.add_instruction(Instruction(Some(dst), InstructionKind::Call { callee, args }));
        Some(dst)
    }

    pub(crate) fn create_binop(&mut self, irtype: IrType, lhs: ValueRef, rhs: ValueRef, op: BinaryOperator) -> Option<ValueRef> {
        let dst = self.create_temporary(irtype)?;
        self.peek_current_function_mut()?.add_instruction(Instruction(Some(dst), InstructionKind::Binop { op, lhs, rhs }));
        Some(dst)
    }
}
