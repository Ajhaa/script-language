use std::collections::HashMap;
use crate::expression::ScriptValue;

#[derive(Clone)]
pub struct Environment {
    pub variables: HashMap<String, ScriptValue>
}

impl Environment {
    pub fn new() -> Environment {
        Environment { variables: HashMap::new() }
    }

    pub fn put(&mut self, key: &str, val: ScriptValue) {
        self.variables.insert(key.to_owned(), val);
    }

    pub fn get(&mut self, key: &str) -> Option<ScriptValue> {
        match self.variables.get(key) {
            Some(val) => Some(val.clone()),
            None => None
        }
    }
}