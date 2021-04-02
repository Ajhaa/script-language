use crate::token::Token;

pub type ScriptValue = f64;

pub trait Expression {
    fn eval(&self) -> ScriptValue;
}
pub struct NumberExpression {
    pub value: f64
}

impl Expression for NumberExpression {
    fn eval(&self) -> ScriptValue {
        self.value
    }
}

pub struct AdditionExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token
}

impl Expression for AdditionExpression {
    fn eval(&self) -> ScriptValue {
        match self.operator {
            Token::Plus => self.left.eval() + self.right.eval(),
            Token::Minus => self.left.eval() - self.right.eval(),
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
    fn eval(&self) -> ScriptValue {
        match self.operator {
            Token::Star => self.left.eval() * self.right.eval(),
            Token::Slash => self.left.eval() / self.right.eval(),
            _ => panic!("")
        }
    }
}

pub struct AssignmentExpression {
    pub identifier: String,
    pub value: Box<dyn Expression>
}

impl Expression for AssignmentExpression {
    fn eval(&self) -> ScriptValue {
        self.value.eval()
    }
}