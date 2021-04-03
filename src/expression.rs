use crate::token::Token;
use crate::environment::Environment;

pub type ScriptValue = f64;

pub trait Expression {
    fn eval(&self, env: &mut Environment) -> ScriptValue;
}
pub struct NumberExpression {
    pub value: f64
}

impl Expression for NumberExpression {
    fn eval(&self, _env: &mut Environment) -> ScriptValue {
        self.value
    }
}

pub struct VariableExpression {
    pub identifier: String
}

impl Expression for VariableExpression {
    fn eval(&self, env: &mut Environment) -> ScriptValue {
        env.get(&self.identifier).unwrap().unwrap()
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
        let value = match self.operator {
            Token::Equals => left == right,
            Token::NotEquals => left != right,
            Token::Lesser => left < right,
            Token::Greater => left > right,
            Token::EqLesser => left <= right,
            Token::EqGreater => left >= right,
            _ => panic!("Not a condition operator: {:?}", self.operator)
        };

        if value {1.0} else {0.0}
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
        match self.operator {
            Token::Plus => left + right,
            Token::Minus => left - right,
            _ => panic!("")
        }
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
        match self.operator {
            Token::Star => left * right,
            Token::Slash => left / right,
            _ => panic!("")
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