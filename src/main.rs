use std::collections::HashMap;

fn main() {}

struct Database {
    store: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: &str, value: String) {
        self.store.insert(key.into(), value);
    }

    fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).cloned()
    }
    fn delete(&mut self, key: &str) {
        self.store.remove(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_key() {
        let mut db = Database::new();
        let key = String::from("name");
        let val = String::from("nql");

        db.set(&key, val.clone());

        let result: Option<String> = db.get(&key);
        assert_eq!(result, Some(val.into()));
    }

    #[test]
    fn should_return_none_when_key_is_missing() {
        let db = Database::new();
        let key = String::from("name");

        let result: Option<String> = db.get(&key);
        assert_eq!(result, None);
    }

    #[test]
    fn should_remove_val_when_delete() {
        let key = String::from("name");
        let val = String::from("nql");

        let mut db = Database::new();
        db.set(&key, val.clone());
        db.delete(&key);

        let result = db.get(&key);
        assert_eq!(result, None);
    }
}
