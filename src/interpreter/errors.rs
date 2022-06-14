use std::fmt;
use super::ScriptValue;
#[derive(Debug,Clone)]
pub enum InterpreterErrorType {
    UndefinedVariable(String),
    NotCallable,
    NotObject,
    PropertyNotFound,
    InvalidIndex,
    NotIndexable,
    Other(String)
}

#[derive(Debug,Clone)]
pub struct InterpreterError {
    pub target: ScriptValue,
    pub err_type: InterpreterErrorType
}

impl InterpreterError {
    pub fn new(target: &ScriptValue, err_type: InterpreterErrorType) -> InterpreterError {
        InterpreterError {
            target: target.clone(),
            err_type
        }
    }

    pub fn other(target: &ScriptValue, msg: &str) -> InterpreterError {
        InterpreterError::new(target, InterpreterErrorType::Other(msg.to_string()))
    }
}


impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.err_type {
            InterpreterErrorType::UndefinedVariable(ident) => write!(f, "Variable not found: {}", ident),
            InterpreterErrorType::NotCallable => write!(f, "Not callable: {}", self.target),
            InterpreterErrorType::NotObject => write!(f, "Not an object: {}", self.target),
            InterpreterErrorType::PropertyNotFound => write!(f, "Property not found: {}", self.target),
            InterpreterErrorType::InvalidIndex => write!(f, "Cannot index with: {}", self.target),
            InterpreterErrorType::NotIndexable => write!(f, "Not indexable: {}", self.target),
            InterpreterErrorType::Other(msg) => write!(f, "{}: {}", msg, self.target),

        }
    }
}
