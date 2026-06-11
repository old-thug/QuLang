use std::collections::{HashMap, HashSet};

use qu_ast::Name;
use qu_common::Storage;
use qu_span::Span;

use super::layout::TypeId;
use super::scope::ScopeId;

#[derive(Debug, Clone, Copy)]
pub struct SymbolId(pub usize);

pub type SymbolStorage = Storage<Span, Symbol>;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub defined_at: Span,
    pub resolved_type: Option<TypeId>,
    pub data: SymbolData,
}

#[derive(Debug, Clone)]
pub enum State {
    Ok,
    Invalid,
    PartiallyInvalid,
}

pub type ParameterNames = Vec<(Name, usize)>;

#[derive(Debug, Clone)]
pub enum SymbolData {
    Function {
        parameter_names: ParameterNames,
        parameter_types: Vec<TypeId>,
    },
    Variable {
        state: State,
        scope_id: ScopeId,
    },
}

impl SymbolData {
    pub fn new_function_data(parameter_names: ParameterNames, parameter_types: Vec<TypeId>) -> Self {
        Self::Function {
            parameter_names,
            parameter_types
        }
    }

    pub fn new_var_data(scope_id: ScopeId) -> Self {
        Self::Variable { state: State::Ok, scope_id }
    }
}

impl Symbol {
    pub fn new_empty(span: Span, scope_id: ScopeId, data: SymbolData) -> Symbol {
        Symbol {
            data,
            defined_at: span,
            resolved_type: None,
        }
    }
}
