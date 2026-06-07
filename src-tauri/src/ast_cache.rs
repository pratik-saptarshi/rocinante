use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Default, Clone)]
pub struct AstCache {
    entries: BTreeMap<String, u64>,
    hits: usize,
}

impl AstCache {
    pub fn fingerprint(&mut self, path: &str, contents: &str) -> u64 {
        let hash = hash_value(&(path, contents));
        if self.entries.get(path).copied() == Some(hash) {
            self.hits += 1;
        }
        self.entries.insert(path.to_string(), hash);
        hash
    }

    pub fn hits(&self) -> usize {
        self.hits
    }
}

fn hash_value<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}
