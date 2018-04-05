use std::collections::hash_map::Keys;
use std::collections::HashMap;
use std::option::Option;

#[allow(dead_code)]
pub struct MemoryCache<T> {
    data: HashMap<String, T>,
}

impl<T> MemoryCache<T> {
    pub fn new() -> MemoryCache<T> {
        MemoryCache { data: HashMap::new() }
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn keys(&self) -> Keys<String, T> {
        return self.data.keys();
    }

    pub fn add(&mut self, key: String, value: T) {
        if !self.data.contains_key(&key) {
            &mut self.data.insert(key, value);
        }
    }

    pub fn remove(&mut self, key: String) {
        if self.data.contains_key(&key) {
            &mut self.data.remove(&key);
        }
    }

    pub fn exists(&self, key: String) -> bool {
        return self.data.contains_key(&key);
    }

    pub fn get(&self, key: String) -> Option<&T> {
        if !self.data.contains_key(&key) {
            return None::<&T>;
        } else {
            return self.data.get(&key);
        }

//            match self.data.get(&key) {
//                Some(&value) => value,
//                None => None::<T>,
//            }
    }
}