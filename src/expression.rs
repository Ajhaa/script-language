use crate::token::Token;
use crate::environment::Environment;
use std::fmt;

// pub type ScriptValue = f64;
#[derive(Debug, Clone)]
pub enum ScriptValue {
    Number(f64),
    String(String),
    Boolean(bool),
    None
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
        match (self, &other) {
            (ScriptValue::Number(left), ScriptValue::Number(right)) => {
                let result = match operator {
                    Token::Equals => left == right,
                    Token::NotEquals => left != right,
                    Token::Lesser => left < right,
                    Token::Greater => left > right,
                    Token::EqLesser => left <= right,
                    Token::EqGreater => left >= right,
                    _ => panic!("Impossible compare")
                };

                ScriptValue::Boolean(result)
            },
            _ => panic!("Cannot compare {:?} and {:?}", self, other)
        }
    }
}


impl fmt::Display for ScriptValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptValue::Number(n) => write!(f, "{}", n),
            ScriptValue::Boolean(b) => write!(f, "{}", b),
            ScriptValue::String(s) => write!(f, "{}", s),
            ScriptValue::None => write!(f, "None"),
        }        
    }
}

pub trait Expression {
    fn eval(&self, env: &mut Environment) -> ScriptValue;
}
pub struct NumberExpression {
    pub value: f64
}

impl Expression for NumberExpression {
    fn eval(&self, _env: &mut Environment) -> ScriptValue {
        ScriptValue::Number(self.value)
    }
}

pub struct VariableExpression {
    pub identifier: String
}

impl Expression for VariableExpression {
    fn eval(&self, env: &mut Environment) -> ScriptValue {
        env.get(&self.identifier).unwrap().clone()
    }
}

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


// TODO
pub struct FunctionExpression {
    pub name: String,
    pub params: Vec<Box<dyn Expression>>
}

impl Expression for FunctionExpression {
    fn eval(&self, env: &mut Environment) -> ScriptValue {
        ScriptValue::Number(0.0)
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