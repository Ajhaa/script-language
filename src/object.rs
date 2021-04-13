use std::collections::HashMap;
use crate::expression::*;
use crate::environment::*;

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

    pub fn set_ref(obj: Rc<RefCell<Object>>, key: String, value: ScriptValue) {
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

    pub fn get(&self, key: &str) -> Option<ScriptValue> {
        match self.fields.get(key) {
            Some(val) => Some(val.clone()),
            None => None
        }
    }

    pub fn get_ref(obj: Rc<RefCell<Object>>, key: &str) -> Option<ScriptValue> {
        match obj.borrow().fields.get(key) {
            // insert self
            // Some(ScriptValue::Function(func)) => {
            //     let mut borrowed = func.borrow_mut();
            //     // let mut env = borrowed.env.borrow_mut();
            //     borrowed.env.enter();
            //     borrowed.env.put_new("self", ScriptValue::Object(obj.clone()));
            //     borrowed.env.exit();

            //     Some(ScriptValue::Function(func.clone()))
            // },
            Some(val) => Some(val.clone()),
            None => None
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        println!("Object dropped");
    }
}