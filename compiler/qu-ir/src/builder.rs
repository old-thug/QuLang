pub mod global;
pub mod value;
pub mod instruction;

use qu_common::Storage;

use crate::{
    GlobalId, IrModule,
    value::{Value, ValueRef},
};

#[derive(Debug)]
pub struct IrBuilder {
    module: *mut IrModule,
    function_stack: Vec<GlobalId>,
}

impl IrBuilder {
    pub fn new(module: &mut IrModule) -> Self {
        Self {
            module,
            function_stack: Vec::new(),
        }
    }

    pub(crate) fn set_current_function(&mut self, function_id: GlobalId) {
        self.function_stack.push(function_id);
    }

    pub(crate) fn pop_current_function(&mut self) -> Option<GlobalId> {
        self.function_stack.pop()
    }

    pub(crate) fn is_in_global_scope(&self) -> bool {
        self.function_stack.len() == 0
    }

}
