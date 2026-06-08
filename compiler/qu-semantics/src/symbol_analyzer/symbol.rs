
use qu_diagnostics::span::Span;

use super::scope::ScopeId;
use super::super::type_checker::layout::TypeId;

#[derive(Debug, Clone, Copy)]
pub struct SymbolId(pub usize);

#[derive(Debug, Clone)]
pub struct Symbol {
    pub state: State,
    pub defined_at: Span,
    pub scope_id: ScopeId,
    pub resolved_type: Option<TypeId>,
}

#[derive(Debug, Clone)]
pub enum State {
    Ok,
    Invalid,
    PartiallyInvalid,
}

impl Symbol {
    pub fn new_empty(span: Span, scope_id: ScopeId) -> Symbol {
        Symbol { state: State::Ok, defined_at: span, scope_id, resolved_type: None }
    }
}
