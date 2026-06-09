#![allow(unused)]
use qu_common::Storage;
use qu_module::{Module, ModuleMap};
use qu_source::Source;
use std::{collections::HashMap, hash::Hash};

#[derive(Debug, Clone, Copy)]
pub struct ModuleId(pub usize);

#[derive(Debug)]
pub struct Context {
    sources: qu_common::Storage<String, qu_source::Source>,
    modules: ModuleMap,
}

impl Context {
    pub fn new() -> Self {
        Self {
            sources: Storage::new(),
            modules: Storage::new(),
        }
    }

    pub fn source(&mut self, path: &str) -> Result<qu_source::SourceId, std::io::Error> {
        let canon_path = std::fs::canonicalize(path)?.display().to_string();
        match self.sources.get_by_id_key(&canon_path) {
            Some(source_id) => return Ok(qu_source::SourceId(source_id)),
            None => {
                let new_source = Source::new(path.to_string())?;
                Ok(qu_source::SourceId(
                    self.sources.put(path.to_string(), new_source),
                ))
            }
        }
    }

    pub fn get_or_put_new_module(&mut self, name: String) -> ModuleId {
        match self.modules.get_by_id_key(&name) {
            Some(id) => ModuleId(id),
            None => {
                let new_mod = qu_module::Module::new();
                ModuleId(self.modules.put(name, new_mod))
            }
        }
    }

    pub fn get_source<'a>(&'a self, id: qu_source::SourceId) -> Option<&'a String> {
        self.sources.pool.get(id.0).map(|s| &s.content)
    }

    pub fn get_source_file<'a>(&'a self, id: qu_source::SourceId) -> Option<&'a qu_source::Source> {
        self.sources.pool.get(id.0)
    }

    pub fn get_module(&mut self, id: ModuleId) -> Option<&mut Module> {
        self.modules.get_pool_mut().get_mut(id.0)
    }
}
