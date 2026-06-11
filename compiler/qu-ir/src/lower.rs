use qu_ast::{Ast, stmt::{StmtData, StmtRef}};
use qu_module::Module;

use crate::{IrModule, builder::IrBuilder, function::IrCallAbi};

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
        for stmt in &ast {
            self.register_globals(&stmt, &mut builder);
        }

        for stmt in ast {
            self.lower_statement(&stmt, &mut builder, true);
        }
        Ok(())
    }

    fn register_globals(&mut self, stmt: &StmtRef, builder: &mut IrBuilder) -> Option<()> {
        match stmt.data() {
            StmtData::FunctionDefinition(function) => {
                let type_layout = self.module.get_types().get_from_key(&function.name.span)?;
                let ir_type     = self.lower_type(type_layout);
                let function_id = builder.new_function(function.name.value.clone(), ir_type, false, IrCallAbi::C);
                builder.set_current_function(function_id);
                for (index, param) in function.prototype.parameters.iter().enumerate() {
                    let param_name = param.name.value.clone();
                    builder.add_function_parameter(index, param_name);
                }
                builder.pop_current_function();
            }
            _ => (),
        }
        Some(())
    }
}
