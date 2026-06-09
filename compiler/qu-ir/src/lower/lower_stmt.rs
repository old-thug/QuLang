use qu_ast::{stmt::{StmtData, StmtRef}};
use qu_common::extract;
use qu_entities::layout::TypeKind;

use crate::{builder::IrBuilder, function::IrCallAbi};

use super::IrLowerer;

impl<'a> IrLowerer<'a> {
    pub fn lower_statement(&mut self, stmt: &StmtRef, builder: &mut IrBuilder) -> Option<()> {
        match stmt.data() {
            StmtData::FunctionDefinition(_) => self.lower_function(stmt, builder),
            StmtData::VariableDecl(_) => self.lower_vardecl(stmt, builder),
            StmtData::Return(_) => self.lower_return(stmt, builder),
            _ => todo!("{:?}", stmt.data()),
        }
    }

    fn lower_function(&mut self, stmt: &StmtRef, builder: &mut IrBuilder) -> Option<()> {
        extract!(stmt.data(), StmtData::FunctionDefinition(function));
        let type_layout = self.module.get_types().get_from_key(&function.name.span)?;
        let ir_type     = self.lower_type(type_layout);
        let function_id = builder.new_function(function.name.value.clone(), ir_type, false, IrCallAbi::C);
        builder.set_current_function(function_id);
        for (index, param) in function.prototype.parameters.iter().enumerate() {
            let param_name = param.name.value.clone();
            builder.add_function_parameter(index, param_name);
        }
        if let Some(ref body) = function.body {
            let value = self.lower_expression(body, builder)?;
            builder.create_ret(value)?;
        }
        Some(())
    }

    fn lower_vardecl(&mut self, stmt: &StmtRef, builder: &mut IrBuilder) -> Option<()> {
        extract!(stmt.data(), StmtData::VariableDecl(vardecl));
        if builder.is_in_global_scope() {
            todo!();
        } else {
            let var_type_layout = self.module.get_types().get_from_key(&vardecl.name.span)?;
            let var_ir_type     = self.lower_type(var_type_layout);
            let var_ref         = builder.create_alloca(vardecl.name.value.clone(), var_ir_type)?;
            let initializer     = self.lower_expression(&vardecl.initializer, builder)?;
            builder.create_store(var_ref, initializer);
            Some(())
        }
    }

    fn lower_return(&self, stmt: &StmtRef, builder: &mut IrBuilder) -> Option<()> {
        todo!()
    }
}
