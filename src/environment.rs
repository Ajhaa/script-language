use std::collections::HashMap;
use crate::expression::ScriptValue;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Env {
    pub variables: HashMap<String, ScriptValue>,
    pub parent: Rc<RefCell<Option<Env>>>
}

impl Env {
    pub fn put(&mut self, key: String, val: ScriptValue) {
        self.variables.insert(key, val);
    }

    pub fn get(&self, key: &str) -> Option<ScriptValue> {
        match self.variables.get(key) {
            Some(val) => return Some(val.clone()),
            None => {
                let parent = self.parent.borrow();
                match &*parent {
                    Some(env) => env.get(key),
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
}