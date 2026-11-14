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
        self.entries.insert(label.into(), value);

        Ok(())
    }
    pub fn fetch(&self, label: &str) -> Result<i32, String> {
        if let Some(value) = self.entries.get(label.into()) {
            return Ok(*value)
        }

        Err("No such store exists.".into())
    }
}

#[cfg(test)]
mod tests {
   use super::Store;

    #[test]
    fn upsert_adds_new_store_successfully() {
        let mut store = Store::new();

        let result = store.upsert("LABEL", 10);
        assert!(result.is_ok());

        let stored = store.fetch("LABEL");
        assert!(stored.is_ok());
        assert_eq!(10, stored.unwrap())
    }
    #[test]
    fn upsert_updates_store_successfully() {
        let mut store = Store::new();

        _ = store.upsert("LABEL", 10);

        let result = store.upsert("LABEL", 20);

        let stored = store.fetch("LABEL");
        assert!(stored.is_ok());
        assert_eq!(20, stored.unwrap())
    }
    #[test]
    fn fetch_fails_if_no_store_with_label() {
        let mut store = Store::new();

        let empty = store.fetch("LABEL");
        assert!(empty.is_err());

        _ = store.upsert("LABEL", 10);

        let unknown = store.fetch("UNKNOWN");
        assert!(empty.is_err());
    }
}