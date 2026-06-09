#![allow(unused)]
use qu_entities::{
    TypeStorage, scope::{Scope, ScopeStorage}, symbol::{SymbolId, SymbolStorage}
};

pub type ModuleMap = qu_common::Storage<String, Module>;

#[derive(Debug)]
pub struct Module {
    exports: Vec<SymbolId>,
    symbols: SymbolStorage,
    scopes: ScopeStorage,
    types: TypeStorage,
}

impl Module {
    pub fn new() -> Self {
        let mut scopes = ScopeStorage::new();
        scopes.get_pool_mut().push(Scope::new(None, qu_entities::scope::ScopeFlag::Global));
        Self {
            symbols: SymbolStorage::new(),
            exports: Vec::new(),
            scopes,
            types: TypeStorage::new(),
        }
    }

    pub fn get_scopes_mut(&mut self) -> &mut ScopeStorage {
        &mut self.scopes
    }

    pub fn get_symbols_mut(&mut self) -> &mut SymbolStorage {
        &mut self.symbols
    }

    pub fn get_scopes(&self) -> &ScopeStorage {
        &self.scopes
    }

    pub fn get_types(&self) -> &TypeStorage {
        &self.types
    }

    pub fn get_types_mut(&mut self) -> &mut TypeStorage {
        &mut self.types
    }

    pub fn get_symbols(&self) -> &SymbolStorage {
        &self.symbols
    }
}
