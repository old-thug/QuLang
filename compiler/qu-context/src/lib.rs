use std::{collections::HashMap, hash::Hash};

pub mod module;
pub mod source;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SourceId(pub usize);

#[derive(Debug, Clone, Copy)]
pub struct ModuleId(pub usize);

#[derive(Debug)]
pub struct Storage<K: Hash + Eq, V> {
    map: HashMap<K, usize>,
    pool: Vec<V>,
}

#[derive(Debug)]
pub struct Context {
    sources: Storage<String, source::Source>,
    modules: Storage<String, module::Module>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            sources: Storage::new(),
            modules: Storage::new(),
        }
    }

    pub fn source(&mut self, path: &str) -> Result<SourceId, std::io::Error> {
        let canon_path = std::fs::canonicalize(path)?.display().to_string();
        match self.sources.get_by_id_key(canon_path) {
            Some(source_id) => { return Ok(SourceId(source_id)) },
            None => {
                let new_source = source::Source::new(path.to_string())?;
                Ok(SourceId(self.sources.put(path.to_string(), new_source)))
            },
        }
    }

    pub fn get_or_put_new_module(&mut self, name: String) -> ModuleId {
        match self.modules.get_by_id_key(name.clone()) {
            Some(id) => ModuleId(id),
            None => {
                let new_mod = module::Module::new();
                ModuleId(self.modules.put(name, new_mod))
            }
        }
    }

    pub fn get_source<'a>(&'a self, id: SourceId) -> Option<&'a String> {
        self.sources.pool.get(id.0).map(|s| &s.content)
    }

    pub fn get_source_file<'a>(&'a self, id: SourceId) -> Option<&'a source::Source> {
        self.sources.pool.get(id.0)
    }
}

impl<K: Hash + Eq, V> Storage<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            pool: Vec::new(),
        }
    }

    pub fn get_by_id_key(&self, key: K) -> Option<usize> {
        self.map.get(&key).copied()
    }

    pub fn put(&mut self, key: K, value: V) -> usize {
        match self.get_by_id_key(key) {
            Some(id) => {
                self.pool.insert(id, value);
                return id;
            },
            None => {
                let new_id = self.pool.len();
                self.pool.push(value);
                return new_id;
            }
        }
    }
}
