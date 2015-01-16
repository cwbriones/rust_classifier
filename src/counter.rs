use std::collections::hash_map::{HashMap, Entry, Iter};

pub struct Counter {
    inner: HashMap<String, u32>
}

impl Counter {
    pub fn new() -> Counter {
        Counter { inner: HashMap::new() }
    }

    pub fn get(&self, label: &str) -> u32 {
        match self.inner.get(label) {
            Some(val) => *val,
            None => 0
        }
    }

    pub fn increment(&mut self, label: &str) -> u32 {
        match self.inner.entry(label.to_string()) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
                *e.get()
            }
            Entry::Vacant(mut e) => {
                e.insert(1);
                1
            }
        }
    }

    pub fn iter(&self) -> Iter<String, u32> {
        self.inner.iter()
    }
}
