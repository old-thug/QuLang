use qu_ast::Ast;
use qu_module::Module;

use crate::IrModule;

mod lower_expr;
mod lower_stmt;
mod lower_type;

#[derive(Debug)]
pub struct IrLowerer<'a> {
    ir_module: &'a mut IrModule,
    module: &'a Module,
}

impl<'a> IrLowerer<'a> {
    pub fn new(ir_module: &'a mut IrModule, module: &'a Module) -> Self {
        Self { ir_module, module }
    }

    pub fn lower_ast(&mut self, ast: Ast) -> Result<(), ()> {
        let mut builder = self.ir_module.get_builder();
        for stmt in ast {
            self.lower_statement(&stmt, &mut builder);
        }
        Ok(())
    }
}
