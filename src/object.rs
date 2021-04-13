use std::collections::HashMap;
use crate::expression::*;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Object {
    pub fields: HashMap<String, ScriptValue>
}

impl Object {
    pub fn new() -> Rc<RefCell<Object>> {
        Rc::new(
            RefCell::new(
                Object { fields: HashMap::new() }
            )
        )
    }

    pub fn set(&mut self, key: String, value: ScriptValue) {
        self.fields.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<ScriptValue> {
        match self.fields.get(key) {
            Some(val) => Some(val.clone()),
            None => None
        }
    }
}