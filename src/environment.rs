use std::collections::HashMap;
use crate::expression::*;
use crate::statement::*;
use crate::object::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Env {
    pub variables: HashMap<String, ScriptValue>,
    pub parent: Rc<RefCell<Option<Env>>>
}

impl Env {
    pub fn put(&mut self, key: String, value: ScriptValue) {
        match self.variables.get(&key) {
            Some(_) => {
                self.variables.insert(key, value);
            },
            None => {
                let mut parent = self.parent.borrow_mut();
                match &mut *parent {
                    Some(env) => {
                        env.put(key, value);
                    },
                    None => panic!("Variable {} not defined", key)
                };
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
                let parent = self.parent.borrow();
                match &*parent {
                    Some(env) => {
                        env.get(key)
                    },
                    None => None
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct Environment {
    //pub variables: HashMap<String, ScriptValue>
    pub env: Rc<RefCell<Option<Env>>>
}

impl Environment {
    pub fn new() -> Environment {
        let parent =  Rc::new(RefCell::new(None));
        let env = Rc::new(RefCell::new(Some(Env {variables: HashMap::new(), parent })));
        Environment { env }
    }

    pub fn enter(&mut self) {
        let parent = Rc::clone(&self.env);
        let next = Rc::new(RefCell::new(Some(Env {variables: HashMap::new(), parent })));
        self.env = next;
    }

    pub fn exit(&mut self) {
        let env = Rc::clone(&self.env);
        let previous = env.borrow();
        match &*previous {
            Some(env) => self.env = Rc::clone(&env.parent),
            None => panic!("ASDAS")
        }
    }

    pub fn put(&mut self, key: &str, val: ScriptValue) {
        let env = Rc::clone(&self.env);
        let mut previous = env.borrow_mut();

        match &mut *previous {
            Some(e) => e.put(key.to_owned(), val),
            None => panic!("ASDAS")
        }
           
    }

    pub fn put_new(&mut self, key: &str, val: ScriptValue) {
        let env = Rc::clone(&self.env);
        let mut previous = env.borrow_mut();

        match &mut *previous {
            Some(e) => e.put_new(key.to_owned(), val),
            None => panic!("ASDAS")
        }   
    }

    pub fn get(&self, key: &str) -> Option<ScriptValue> {
        let env = Rc::clone(&self.env);
        let previous = env.borrow();

        match &*previous {
            Some(e) => e.get(key),
            None => panic!("ASDAS")
        }
    }

    pub fn dump(&self) -> HashMap<String, ScriptValue> {
        let env = Rc::clone(&self.env);

        let previous = env.borrow();
        match &*previous {
            Some(e) => e.variables.clone(),
            None => panic!("ASDAS")
        }
    }

    pub fn create_internal_function(&mut self, name: &str, params: Vec<String>, func: InternalFunction) {
        self.put_new(name, ScriptValue::Function(
            Function::new(
                params,
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