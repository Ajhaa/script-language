use std::collections::HashMap;
use crate::expression::*;
use crate::statement::*;
use crate::function::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Env {
    pub variables: HashMap<String, ScriptValue>,
    pub parent: Option<Rc<RefCell<Env>>>
}

impl Env {
    pub fn put(&mut self, key: String, value: ScriptValue) {
        match self.variables.get(&key) {
            Some(_) => {
                self.variables.insert(key, value);
            },
            None => {
                match &self.parent {
                    Some(env) => {
                        let mut parent = env.borrow_mut();
                        parent.put(key, value);
                    },
                    None => panic!("Variable {} not defined", key)
                }
            }
        };
    }

    pub fn put_new(&mut self, key: String, value: ScriptValue) {
        self.variables.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<ScriptValue> {
        match self.variables.get(key) {
            Some(val) => return Some(val.clone()),
            None => {
                match &self.parent {
                    Some(env) => {
                        let parent = env.borrow();
                        parent.get(key)
                    },
                    None => None
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Environment {
    pub env: Rc<RefCell<Env>>
} 

impl Environment {
    pub fn new() -> Environment {
        let parent = None;
        let env = Rc::new(RefCell::new(Env {variables: HashMap::new(), parent }));
        Environment { env }
    }

    pub fn enter(&mut self) {
        let parent = Some(Rc::clone(&self.env));
        let next = Rc::new(RefCell::new(Env {variables: HashMap::new(), parent }));
        self.env = next;
    }

    pub fn exit(&mut self) {
        let env = Rc::clone(&self.env);
        let current = env.borrow();

        match &current.parent {
            Some(env) => {
                self.env = Rc::clone(&env);
            }
            None => panic!("ASDAS")
        }
    }

    pub fn put(&mut self, key: &str, val: ScriptValue) {
        let mut env = self.env.borrow_mut();
        env.put(key.to_owned(), val);
    }

    pub fn put_new(&mut self, key: &str, val: ScriptValue) {
        let mut env = self.env.borrow_mut();
        env.put_new(key.to_owned(), val);
    }

    pub fn get(&self, key: &str) -> Option<ScriptValue> {
        let env = self.env.borrow();
        env.get(key)
    }

    pub fn _dump(&self) -> HashMap<String, ScriptValue> {
        let env = self.env.borrow();
        env.variables.clone()
    }

    pub fn create_internal_function(&mut self, name: &str, params: Vec<&str>, func: InternalFunction) {
        self.put_new(name, ScriptValue::Function(
            Function::new(
                params.iter().map(|e| e.to_string()).collect(),
                Rc::new(Box::new(
                    InternalStatement {
                        func
                    }
                )),
                Rc::clone(&self.env)
            ),
        ));
    }
}