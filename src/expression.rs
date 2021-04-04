use crate::token::Token;
use crate::environment::Environment;
use crate::statement::*;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Rc<Box<dyn Statement>>,
    pub env: RefCell<Environment>
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Func")
    }
}

impl Function {
    pub fn new(params: Vec<String>, body: Rc<Box<dyn Statement>>, env: RefCell<Environment>) -> Function {
        Function { params, body, env }
    }

    pub fn call(&self, _env: &mut Environment, params: &Vec<Box<dyn Expression>>) -> ScriptValue {
        let mut env = self.env.borrow_mut();
        let mut pass_env = env.clone();
        for i in 0..self.params.len() {
            env.put(&self.params[i], params[i].eval(&mut pass_env));
        }
        let val = (*self.body).eval(&mut env);
        match val {
            StatementValue::Normal(x) => x,
            StatementValue::Return(x) => x 
        }
    }
}

// impl std::clone::Clone for Function {
//     fn clone(&self) -> Function {
//         Function { body: self.body }
//     }
// }

// pub type ScriptValue = f64;
#[derive(Debug, Clone)]
pub enum ScriptValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Function(Function),
    None,
    Unit
}

impl ScriptValue {
    // TODO better name
    fn numeric(&self, other: ScriptValue, operator: Token) -> ScriptValue { 
        match (self, &other) {
            (ScriptValue::Number(left), ScriptValue::Number(right)) => {
                let result = match operator {
                    Token::Plus => left + right,
                    Token::Minus => left - right,
                    Token::Star => left * right,
                    Token::Slash => left / right,
                    _ => panic!("Impossible addition")
                };

                ScriptValue::Number(result)
            },
            _ => panic!("Cannot add {:?} and {:?}", self, other)
        }
    }

    fn boolean(&self, other: ScriptValue, operator: Token) -> ScriptValue {
        let result = match (self, &other) {
            (ScriptValue::Number(left), ScriptValue::Number(right)) => {
                match operator {
                    Token::Equals => left == right,
                    Token::NotEquals => left != right,
                    Token::Lesser => left < right,
                    Token::Greater => left > right,
                    Token::EqLesser => left <= right,
                    Token::EqGreater => left >= right,
                    _ => panic!("Impossible boolean operation")
                }
            },
            (ScriptValue::Boolean(left), ScriptValue::Boolean(right)) => {
                match operator {
                    Token::Equals => left == right,
                    Token::NotEquals => left != right,
                    _ => panic!("Impossible boolean operation")
                }
            }
            _ => panic!("Cannot compare {:?} and {:?}", self, other)
        };

        ScriptValue::Boolean(result)
    }
}

impl Expression for ScriptValue {
    fn eval(&self, _env: &mut Environment) -> ScriptValue {
        self.clone()
    }
}


impl fmt::Display for ScriptValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptValue::Number(n) => write!(f, "{}", n),
            ScriptValue::Boolean(b) => write!(f, "{}", b),
            ScriptValue::String(s) => write!(f, "{}", s),
            ScriptValue::Function(_) => write!(f, "Func"),
            ScriptValue::None => write!(f, "null"),
            ScriptValue::Unit => write!(f, "()")
        }        
    }
}

pub trait Expression: fmt::Debug {
    fn eval(&self, env: &mut Environment) -> ScriptValue;
}
// pub struct ValueExpression {
//     pub value: ScriptValue
// }

// impl Expression for ValueExpression {
//     fn eval(&self, _env: &mut Environment) -> ScriptValue {
//         self.value
//     }
// }

// impl ValueExpression {
//     fn number(num: f64) -> ValueExpression {
//         ValueExpression { value: ScriptValue::Number(num) }
//     }
// }
#[derive(Debug)]
pub struct VariableExpression {
    pub identifier: String
}

impl Expression for VariableExpression {
    fn eval(&self, env: &mut Environment) -> ScriptValue {
        env.get(&self.identifier).unwrap().clone()
    }
}

#[derive(Debug)]
pub struct ConditionExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token
}

impl Expression for ConditionExpression {
    fn eval(&self, env: &mut Environment) -> ScriptValue {
        let left = self.left.eval(env);
        let right = self.right.eval(env);
        left.boolean(right, self.operator.clone())
    }
}

#[derive(Debug)]
pub struct AdditionExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token
}

impl Expression for AdditionExpression {
    fn eval(&self, env: &mut Environment) -> ScriptValue {
        let left = self.left.eval(env);
        let right = self.right.eval(env);
        left.numeric(right, self.operator.clone())
    }
}

#[derive(Debug)]
pub struct MultiplicationExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token
}

impl Expression for MultiplicationExpression {
    fn eval(&self, env: &mut Environment) -> ScriptValue {
        let left = self.left.eval(env);
        let right = self.right.eval(env);
        left.numeric(right, self.operator.clone())
    }
}


#[derive(Debug)]
pub struct FunctionExpression {
    pub name: String,
    pub params: Vec<Box<dyn Expression>>
}

impl Expression for FunctionExpression {
    fn eval(&self, env: &mut Environment) -> ScriptValue {
        let mut env_clone = env.clone();
        let target = env.get(&self.name);
        match target {
            Some(ScriptValue::Function(func)) => {
                func.call(&mut env_clone, &self.params)
            },
            _ => panic!("Cannot call {:?}", target)
        }
    }
}

// pub struct AssignmentExpression {
//     pub left: Box<dyn Expression>,
//     pub right: Box<dyn Expression>
// }

// impl Expression for AssignmentExpression {
//     fn eval(&self, env: &mut Environment) -> ScriptValue {
//         if let VariableExpression = self.left {

//         }
//         // let val = self.expr.eval(env);
//         // env.put(&self.identifier, Some(val));

//         // val
//     }
// }