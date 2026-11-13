use std::collections::HashMap;

pub struct Store {
    entries: HashMap<String, i32>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
}

impl Store {
    pub fn upsert(&mut self, label: &str, value: i32) -> Result<(), String> {
        todo!()
    }
    pub fn fetch(&self, label: &str) -> Result<i32, String> {
        todo!()
    }
}