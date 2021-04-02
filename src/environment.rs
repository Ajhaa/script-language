use std::collections::HashMap;
use crate::expression::ScriptValue;

pub struct Environment {
    pub variables: HashMap<String, Option<ScriptValue>>
}

impl Environment {
    pub fn new() -> Environment {
        Environment { variables: HashMap::new() }
    }

    pub fn put(&mut self, key: &str, val: Option<ScriptValue>) {
        self.variables.insert(key.to_owned(), val);
    }

    pub fn get(&mut self, key: &str) -> Option<&Option<ScriptValue>> {
        self.variables.get(key)
    }
}