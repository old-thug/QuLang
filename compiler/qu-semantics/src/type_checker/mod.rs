#![allow(unused)]
mod check_expr;
mod check_stmt;
mod resolver;
mod type_coercer;

use std::any::Any;
use std::collections::HashSet;

use qu_ast::stmt::FunctionDefinition;
use qu_ast::{Ast, stmt};
use qu_common::{Storage, extract};
use qu_diagnostics::Diagnostic;
use qu_entities::layout::{TypeId, TypeKind, TypeLayout};
use qu_entities::scope::ScopeStorage;
use qu_entities::symbol::{SymbolData, SymbolStorage};
use qu_entities::{TypeStorage, layout};
use qu_module::Module;
use qu_span::Span;
use type_coercer::{CoerceResult, TypeCoercer};

#[derive(Debug)]
pub struct TypeChecker<'a> {
    module: &'a mut Module,
    diagnostics: Vec<Diagnostic>,
}

impl<'a> TypeChecker<'a> {
    pub fn new(module: &'a mut Module) -> Self {
        Self {
            module,
            diagnostics: Vec::new(),
        }
    }

    pub fn run(&mut self, ast: &Ast) -> Result<(), Vec<Diagnostic>> {
        self.register_builtin_types();
        self.collect_types(ast);
        for stmt in ast {
            self.check_statement(stmt);
        }

        if self.diagnostics.len() != 0 {
            Err(std::mem::replace(&mut self.diagnostics, Vec::new()))
        } else {
            Ok(())
        }
    }

    pub(super) fn check_function_(&mut self, function: &FunctionDefinition) -> Option<()> {
        let return_type_id = match function.prototype.return_type {
            Some(ref type_hint) => self.resolve_type_to_id(type_hint),
            None => Self::TYPEID_VOID,
        };
        let mut parameter_types = Vec::new();
        for param in &function.prototype.parameters {
            let param_type_id = match (&param.type_hint, &param.default_value) {
                (Some(type_hint), None) => self.resolve_type_to_id(&type_hint),
                (None, Some(expression)) => self.check_expression(&expression)?,
                (Some(type_hint), Some(expression)) => {
                    let target_type_id = self.resolve_type_to_id(type_hint);
                    let source_type_id = self.check_expression(expression)?;
                    self.try_coerce(
                        (type_hint.span, target_type_id),
                        (expression.span, source_type_id),
                    )?;
                    target_type_id
                }
                _ => unreachable!(),
            };

            if let Some(symbol) = self
                .module
                .get_symbols_mut()
                .get_from_key_mut(&param.name.span)
            {
                symbol.resolved_type = Some(param_type_id);
            }

            parameter_types.push(param_type_id);
            self.module
                .get_types_mut()
                .map(param.name.span, param_type_id.0);
        }

        let mut parameter_names = Vec::new();
        if let Some(function_symbol) = self
            .module
            .get_symbols_mut()
            .get_from_key_mut(&function.name.span)
        {
            match &mut function_symbol.data {
                SymbolData::Function {
                    parameter_names: pn,
                    parameter_types: pt,
                    ..
                } => {
                    parameter_names = pn.clone();
                    for param_type_id in &parameter_types {
                        pt.push(*param_type_id);
                    }

                    assert_eq!(
                        parameter_names.len(),
                        parameter_types.len(),
                        "names of function parameter must match the types of the parameters",
                    );
                }
                _ => unreachable!(),
            }
        } else {
            return None;
        }

        // generate layout and register back to type table
        let function_type_layout = TypeKind::Function {
            return_type: return_type_id,
            parameter_names,
            parameter_types: parameter_types.clone(),
        };
        let function_type_id = self.get_type_id_of_kind(function_type_layout);
        self.module
            .get_types_mut()
            .map(function.name.span, function_type_id.0);

        let name = self.get_type_name(&function_type_id);
        // re-fetch final symbol for atomic completion
        if let Some(function_symbol) = self
            .module
            .get_symbols_mut()
            .get_from_key_mut(&function.name.span)
        {
            function_symbol.resolved_type = Some(function_type_id);
        }
        Some(())
    }

    fn collect_types(&mut self, ast: &Ast) -> Option<()> {
        for stmt in ast {
            match stmt.data() {
                stmt::StmtData::TypeDefinition(_) => todo!(),
                // TODO: move this to it's own function
                stmt::StmtData::FunctionDefinition(function) => {
                    self.check_function_(function)?;
                }
                _ => {}
            }
        }
        Some(())
    }

    pub(super) fn emit_diag(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    pub(super) fn map_locus_to_type(&mut self, span: Span, type_id: &TypeId) {
        self.module.get_types_mut().map(span, type_id.0);
    }

    pub(super) fn get_type_name(&mut self, type_id: &TypeId) -> String {
        let this = unsafe { self as *mut Self };
        let layout = self.get_type_layout_from_id(type_id).unwrap();
        match layout.kind {
            layout::TypeKind::I8 => format!("i8"),
            layout::TypeKind::I16 => format!("i16"),
            layout::TypeKind::I32 => format!("i32"),
            layout::TypeKind::I64 => format!("i64"),
            layout::TypeKind::U8 => format!("u8"),
            layout::TypeKind::U16 => format!("u16"),
            layout::TypeKind::U32 => format!("u32"),
            layout::TypeKind::U64 => format!("u64"),
            layout::TypeKind::String => format!("string"),
            layout::TypeKind::Char => format!("char"),
            layout::TypeKind::Void => format!("void"),
            layout::TypeKind::Bool => format!("bool"),
            layout::TypeKind::Function {
                return_type,
                ref parameter_names,
                ref parameter_types,
            } => {
                let mut out = format!("fn(");
                parameter_types.iter().enumerate().for_each(|(idx, ty)| {
                    if idx != 0 {
                        out += ", ";
                    }
                    out += &format!("{}", unsafe { &mut *this }.get_type_name(ty));
                });
                out += &format!(") -> {}", unsafe { &mut *this }.get_type_name(&return_type));
                out
            }
            layout::TypeKind::Pointer(inner) => format!("*{}", self.get_type_name(&inner)),
            _ => todo!("{:?}", layout.kind),
        }
    }

    fn get_type_id_of_kind(&mut self, type_kind: layout::TypeKind) -> layout::TypeId {
        for (id, type_layout) in self.module.get_types_mut().get_pool().iter().enumerate() {
            if type_layout.kind == type_kind {
                return layout::TypeId(id);
            }
        }
        let new_id = self.module.get_types_mut().get_pool().len();
        self.module
            .get_types_mut()
            .get_pool_mut()
            .push(TypeLayout::new(type_kind, None));
        layout::TypeId(new_id)
    }

    pub(super) fn get_type_layout_from_id(&self, type_id: &TypeId) -> Option<&TypeLayout> {
        self.module.get_types().get_pool().get(type_id.0)
    }

    pub(super) fn try_coerce(
        &mut self,
        target: (Span, TypeId),
        source: (Span, TypeId),
    ) -> Option<()> {
        match TypeCoercer::coerce(self, target.1, source.1) {
            CoerceResult::Identity => Some(()),
            CoerceResult::IntegerWidening => {
                let diag = Diagnostic::new(
                    qu_diagnostics::Severity::Warning,
                    format!("implicit casting"),
                    source.0,
                    format!(
                        "type `{}` is implicitly casted to match `{}`",
                        self.get_type_name(&source.1),
                        self.get_type_name(&target.1)
                    ),
                );
                self.emit_diag(diag);
                Some(())
            }
            CoerceResult::IntegerShrinking => {
                let diag = Diagnostic::new(
                    qu_diagnostics::Severity::Error,
                    format!("type mismatch"),
                    source.0,
                    format!(
                        "cannot implicitly shrink `{}` to fit `{}`",
                        self.get_type_name(&source.1),
                        self.get_type_name(&target.1)
                    ),
                );
                self.emit_diag(diag);
                None
            }
            _ => {
                let diag = Diagnostic::new(
                    qu_diagnostics::Severity::Error,
                    format!("type mismatch"),
                    source.0,
                    format!(
                        "expected type `{}` got value of type `{}`",
                        self.get_type_name(&target.1),
                        self.get_type_name(&source.1)
                    ),
                )
                .with_label(format!("type specified here"), target.0);
                self.emit_diag(diag);
                None
            }
        }
    }

    pub(super) fn can_coerce(&mut self, target: TypeId, source: TypeId) -> bool {
        match TypeCoercer::coerce(self, target, source) {
            CoerceResult::Identity => true,
            CoerceResult::IntegerWidening => true,
            _ => false,
        }
    }
}
