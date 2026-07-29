use core::hash::Hash;
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct Scopes<K, V> {
    base: FxHashMap<K, V>,
    scopes: Vec<FxHashMap<K, V>>,
}

impl<K: Eq + Hash, V> Scopes<K, V> {
    pub fn push_scope(&mut self) {
        self.scopes.push(FxHashMap::default());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn insert(&mut self, k: K, v: V) {
        self.scopes
            .last_mut()
            .unwrap_or(&mut self.base)
            .insert(k, v);
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(k))
            .or_else(|| self.base.get(k))
    }
}

impl<K, V> Default for Scopes<K, V> {
    fn default() -> Self {
        Self {
            base: FxHashMap::default(),
            scopes: Vec::default(),
        }
    }
}
