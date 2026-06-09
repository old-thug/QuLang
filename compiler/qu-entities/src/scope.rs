use std::collections::HashMap;

use qu_common::Storage;
use qu_span::Span;

use super::symbol::SymbolId;

#[derive(Debug, Clone)]
pub struct SymbolMap(pub HashMap<String, SymbolId>);

pub type ScopeStorage = Storage<Span, Scope>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ScopeId(pub usize);

#[derive(Debug, Clone)]
pub struct Scope {
    pub parent: Option<ScopeId>,
    pub symbols: SymbolMap,
    pub flag: ScopeFlag,
}

#[derive(Debug, Clone, Copy)]
pub enum ScopeFlag {
    Global,
    Function,
    Loop,
    Plain,
}

impl Scope {
    pub fn new(parent: Option<ScopeId>, flag: ScopeFlag) -> Self {
        Self {
            symbols: SymbolMap(HashMap::new()),
            parent,
            flag,
        }
    }

    pub fn add(&mut self, name: String, id: SymbolId) -> Option<SymbolId> {
        self.symbols.0.insert(name, id)
    }

    pub fn find(&self, name: &str) -> Option<SymbolId> {
        self.symbols.0.get(name).copied()
    }
}
