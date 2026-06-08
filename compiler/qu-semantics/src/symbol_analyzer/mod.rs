#![allow(unused)]
mod check_stmt;
mod check_expr;
mod collect_stmt;
mod error;
mod scope;
mod symbol;

use qu_ast::Ast;
use qu_context::Storage;
use qu_diagnostics::{
    Diagnostic,
    span::{self, Span},
};
use scope::{Scope, ScopeFlag, ScopeId};
use symbol::{Symbol, SymbolId};

pub type SymbolStorage = Storage<Span, Symbol>;
pub type ScopeStorage = Storage<Span, Scope>;

#[derive(Debug, Clone)]
pub struct SymbolAnalyzer {
    symbols: SymbolStorage,
    scopes: ScopeStorage,
    current_scope_id: ScopeId,
    diagnostics: Vec<Diagnostic>,
}

impl SymbolAnalyzer {
    pub fn new() -> SymbolAnalyzer {
        let mut scopes = ScopeStorage::new();
        // Append global scope
        scopes
            .get_pool_mut()
            .push(Scope::new(None, scope::ScopeFlag::Global));
        SymbolAnalyzer {
            symbols: SymbolStorage::new(),
            scopes,
            // Set current scope to global scope
            current_scope_id: ScopeId(0),
            diagnostics: Vec::new(),
        }
    }

    pub fn run(&mut self, ast: &Ast) -> Result<(SymbolStorage, ScopeStorage), Vec<Diagnostic>> {
        self.collect_top_level_symbols(ast);
        self.check_statements(ast);
        if self.diagnostics.len() != 0 {
            return Err(self.diagnostics.clone());
        } else {
            let symbols = std::mem::replace(&mut self.symbols, SymbolStorage::new());
            let scopes = std::mem::replace(&mut self.scopes, ScopeStorage::new());
            Ok((symbols, scopes))
        }
    }

    pub(super) fn emit_diag(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }

    fn collect_top_level_symbols(&mut self, ast: &Ast) {
        for ref stmt in ast {
            self.collect_stmt(stmt);
        }
    }

    pub(super) fn enter_new_scope(&mut self, flag: ScopeFlag) {
        let parent_id = self.current_scope_id;
        let new_scope = Scope::new(Some(parent_id), flag);
        self.scopes.get_pool_mut().push(new_scope);
        self.current_scope_id = ScopeId(self.scopes.get_pool().len() - 1);
    }

    pub(super) fn leave_scope(&mut self) {
        // Pop the last scope off the stack
        match self.scopes.get_pool_mut().pop() {
            Some(scope) => {
                // Set current id to id of last scope's parent or the global scope.
                self.current_scope_id = scope.parent.unwrap_or(ScopeId(0));
            }
            None => {
                // nothing on the stack.
                // strange.
                unreachable!();
            }
        }
    }

    pub(super) fn get_new_symbol_id(&mut self, symbol: Symbol) -> SymbolId {
        self.symbols.get_pool_mut().push(symbol);
        SymbolId(self.symbols.get_pool().len() - 1)
    }

    pub(super) fn current_scope_id(&self) -> ScopeId {
        self.current_scope_id
    }

    pub(super) fn scope(&self, id: ScopeId) -> Option<&Scope> {
        self.scopes.get_pool().get(id.0)
    }

    pub(super) fn scope_mut(&mut self, id: ScopeId) -> Option<&mut Scope> {
        self.scopes.get_pool_mut().get_mut(id.0)
    }

    pub(super) fn get_symbol(&mut self, id: SymbolId) -> Option<&Symbol> {
        self.symbols.get_pool().get(id.0)
    }

    pub(super) fn get_symbol_mut(&mut self, id: SymbolId) -> Option<&mut Symbol> {
        self.symbols.get_pool_mut().get_mut(id.0)
    }

    pub(super) fn map_locus_to_symbol(&mut self, span: Span, id: SymbolId) {
        self.symbols.map(span, id.0);
    }

    pub(super) fn map_locus_to_scope(&mut self, span: Span, id: ScopeId) {
        self.scopes.map(span, id.0);
    }

    pub(super) fn get_symbol_id_from_name(&mut self, name: String) -> Option<SymbolId> {
        self.find_id_in_scope_and(self.current_scope_id(), &name, |_, id| {
            Some(id)
        })
    }

    pub(super) fn add_new_empty_symbol_to_scope(&mut self, name: qu_ast::Name, scope_id: ScopeId) {
        let new_symbol = Symbol::new_empty(name.span, scope_id);
        let new_symbol_id = self.get_new_symbol_id(new_symbol);
        self.map_locus_to_symbol(name.span, new_symbol_id);
        self.map_locus_to_scope(name.span, scope_id);
        match self.scope_mut(scope_id) {
            Some(scope) => {
                scope.add(name.value.clone(), new_symbol_id);
            }
            None => unreachable!("invalid scope {:?}", scope_id),
        }
    }

    /// # Safety / FIXME
    /// This uses an unsafe raw pointer cast to bypass the Rust borrow checker's
    /// aliasing rules, allowing us to pass `&mut self` and a `&mut Symbol` (which
    /// lives inside `self`) to the same closure simultaneously.
    ///
    /// ### CRITICAL RULES FOR CLOSURES USING THIS METHOD:
    /// 1. **No Reallocations:** The closure `f` MUST NOT push new symbols to the pool,
    ///    clear the pool, or do anything that triggers a vector reallocation. If the
    ///    pool reallocates, the `&mut Symbol` reference becomes a dangling pointer (Use-After-Free).
    /// 2. **No Memory Aliasing:** The closure must not attempt to feetch or mutate this
    ///    exact same symbol via `self` while the `symbol` parameter is alive.
    pub(super) fn get_symbol_and<F, U>(&mut self, id: SymbolId, f: F) -> Option<U>
    where
        F: FnOnce(&mut Self, &mut Symbol) -> U,
    {
        let ptr = self as *mut Self;
        let symbol = unsafe { &mut (*ptr) }
            .symbols
            .get_pool_mut()
            .get_mut(id.0)?;
        //let symbol = self.symbols.get_pool_mut().get_mut(id.0)?;
        Some(f(self, symbol))
    }

    // Performs an earger search
    pub(super) fn find_id_in_scope_and<F, U>(&mut self, id: ScopeId, name: &str, f: F) -> Option<U>
    where
        F: FnOnce(&mut Self, SymbolId) -> Option<U>,
    {
        let scope = self.scopes.get_pool_mut().get(id.0)?;
        match scope.find(name) {
            Some(sym) => {
                return f(self, sym);
            }
            None => {
                if let Some(parent_id) = scope.parent {
                    self.find_id_in_scope_and(parent_id, name, f)
                } else {
                    None
                }
            },
        }
    }

    pub(super) fn find_id_in_scope_shallow_and<F, U>(&mut self, id: ScopeId, name: &str, f: F) -> Option<U>
    where
        F: FnOnce(&mut Self, SymbolId) -> Option<U>,
    {
        let scope = self.scopes.get_pool_mut().get(id.0)?;
        match scope.find(name) {
            Some(sym) => {
                return f(self, sym);
            }
            None => {
                None
            },
        }
    }
}
