use std::collections::HashMap;
use crate::expression::ScriptValue;

type Env = HashMap<String, ScriptValue>;

#[derive(Clone)]
pub struct Environment {
    //pub variables: HashMap<String, ScriptValue>
    pub env: Vec<Env>
}

impl Environment {
    pub fn new() -> Environment {
        Environment { env: vec![HashMap::new()] }
    }

    pub fn enter(&mut self) {
        self.env.push(HashMap::new());
    }

    pub fn exit(&mut self) {
        self.env.pop();
    }

    pub fn put(&mut self, key: &str, val: ScriptValue) {
        self.env.last_mut().unwrap().insert(key.to_owned(), val);
    }

    pub fn get(&mut self, key: &str) -> Option<ScriptValue> {
        for env in self.env.iter().rev() {
            match env.get(key) {
                Some(val) => return Some(val.clone()),
                None => ()
            }
        }
        None
    }
}