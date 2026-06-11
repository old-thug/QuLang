#![allow(unused)]
mod check_expr;
mod check_stmt;
mod collect_stmt;
mod error;

use qu_ast::Ast;
use qu_common::Storage;
use qu_context::ModuleId;
use qu_diagnostics::Diagnostic;
use qu_entities::scope::{Scope, ScopeFlag, ScopeId, ScopeStorage};
use qu_entities::symbol::{Symbol, SymbolData, SymbolId, SymbolStorage};
use qu_module::Module;
use qu_span::{Span, Spanned};

#[derive(Debug)]
pub struct SymbolAnalyzer<'a> {
    current_scope_id: ScopeId,
    diagnostics: Vec<Diagnostic>,
    module: &'a mut Module,
}

impl<'a> SymbolAnalyzer<'a> {
    pub fn new(module: &'a mut Module) -> Self {
        SymbolAnalyzer {
            // Set current scope to global scope
            current_scope_id: ScopeId(0),
            diagnostics: Vec::new(),
            module: module,
        }
    }

    pub fn run(&mut self, ast: &Ast) -> Result<(), Vec<Diagnostic>> {
        self.collect_top_level_symbols(ast);
        self.check_statements(ast);
        if self.diagnostics.len() != 0 {
            return Err(self.diagnostics.clone());
        } else {
            Ok(())
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
        self.module.get_scopes_mut().get_pool_mut().push(new_scope);
        self.current_scope_id = ScopeId(self.module.get_scopes().get_pool().len() - 1);
    }

    pub(super) fn leave_scope(&mut self) {
        // Pop the last scope off the stack
        match self.module.get_scopes_mut().get_pool_mut().pop() {
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
        self.module.get_symbols_mut().get_pool_mut().push(symbol);
        SymbolId(self.module.get_symbols().get_pool().len() - 1)
    }

    pub(super) fn current_scope_id(&self) -> ScopeId {
        self.current_scope_id
    }

    pub(super) fn scope(&self, id: ScopeId) -> Option<&Scope> {
        self.module.get_scopes().get_pool().get(id.0)
    }

    pub(super) fn scope_mut(&mut self, id: ScopeId) -> Option<&mut Scope> {
        self.module.get_scopes_mut().get_pool_mut().get_mut(id.0)
    }

    pub(super) fn get_symbol(&mut self, id: SymbolId) -> Option<&Symbol> {
        self.module.get_symbols().get_pool().get(id.0)
    }

    pub(super) fn get_symbol_mut(&mut self, id: SymbolId) -> Option<&mut Symbol> {
        self.module.get_symbols_mut().get_pool_mut().get_mut(id.0)
    }

    pub(super) fn map_locus_to_symbol(&mut self, span: Span, id: SymbolId) {
        self.module.get_symbols_mut().map(span, id.0);
    }

    pub(super) fn map_locus_to_scope(&mut self, span: Span, id: ScopeId) {
        self.module.get_scopes_mut().map(span, id.0);
    }

    pub(super) fn get_symbol_id_from_name(&mut self, name: String) -> Option<SymbolId> {
        self.find_id_in_scope_and(self.current_scope_id(), &name, |_, id| Some(id))
    }

    pub(super) fn add_new_empty_symbol_to_scope(&mut self, name: qu_ast::Name, scope_id: ScopeId, data: SymbolData) {
        let new_symbol = Symbol::new_empty(name.span, scope_id, data);
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
            .module.get_symbols_mut()
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
        let scope = self.module.get_scopes_mut().get_pool_mut().get(id.0)?;
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
            }
        }
    }

    pub(super) fn find_id_in_scope_shallow_and<F, U>(
        &mut self,
        id: ScopeId,
        name: &str,
        f: F,
    ) -> Option<U>
    where
        F: FnOnce(&mut Self, SymbolId) -> Option<U>,
    {
        let scope = self.module.get_scopes_mut().get_pool_mut().get(id.0)?;
        match scope.find(name) {
            Some(sym) => {
                return f(self, sym);
            }
            None => None,
        }
    }
}
