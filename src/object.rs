use std::collections::HashMap;
use crate::expression::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt::{Display, Debug, Formatter, Result};

pub trait ObjectLike: Debug + Display {
    fn get(&self, key: &str) -> Option<ScriptValue>;
    fn set(&mut self, key: String, val: ScriptValue);  
}

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

    pub fn set_ref(obj: Rc<RefCell<dyn ObjectLike>>, key: String, value: ScriptValue) {
        let mut obj_ref = obj.borrow_mut();
        match value {
            ScriptValue::Function(func) => {
                // let new_func = func.clone();
                let mut borrowed = func.borrow_mut();
                borrowed.env.enter();
                borrowed.env.put_new("self", ScriptValue::Object(obj.clone()));
                // borrowed.env.exit();
                let method = Function::new(borrowed.params.clone(), borrowed.body.clone(), Rc::clone(&borrowed.env.env));
                borrowed.env.exit();
                obj_ref.set(key, ScriptValue::Function(method));
            }
            _ => obj_ref.set(key, value)
        }
    }
}

impl ObjectLike for Object {
    fn set(&mut self, key: String, value: ScriptValue) {
        self.fields.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<ScriptValue> {
        match self.fields.get(key) {
            Some(val) => Some(val.clone()),
            None => None
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{ ")?;

        write!(f, "{}", self.fields.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<_>>().join(", "))?;

        write!(f, " }}")
    }
}

impl ObjectLike for String {
    fn set(&mut self, _: String, _: ScriptValue) {
        panic!("Cannot set field for immutable string");
    }

    fn get(&self, key: &str) -> Option<ScriptValue> {
        match key {
            "length" => {
                // Todo easy way to return functions?
                Some(ScriptValue::Number(self.len() as f64))
            },
            _ => panic!("String has no property {}", key)
        }
    }
}


impl Drop for Object {
    fn drop(&mut self) {
        println!("Object dropped");
    }
}