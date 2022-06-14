use crate::function::*;
use crate::interpreter::{Interpreter, ExpressionResult, errors::*};
use crate::object::*;
use crate::token::{Token, TokenType};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub trait Expression: fmt::Debug {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ExpressionResult;

    fn assign(&self, _: &mut Interpreter, _: ScriptValue) -> ExpressionResult {
        panic!("Cannot assign to {:?}", self);
    }
}

pub trait ExpressionVisitor {
    fn visit_variable(&mut self, expr: &VariableExpression) -> ExpressionResult;
    fn visit_value(&mut self, expr: &ScriptValue) -> ExpressionResult;
    fn visit_addition(&mut self, expr: &AdditionExpression) -> ExpressionResult;
    fn visit_multiplication(&mut self, expr: &MultiplicationExpression) -> ExpressionResult;
    fn visit_condition(&mut self, expr: &ConditionExpression) -> ExpressionResult;
    fn visit_function(&mut self, expr: &FunctionExpression) -> ExpressionResult;
    fn visit_access(&mut self, expr: &AccessExpression) -> ExpressionResult;
    fn visit_index(&mut self, expr: &IndexExpression) -> ExpressionResult;
}

#[derive(Debug, Clone)]
pub enum ScriptValue {
    Number(f64),
    String(Rc<RefCell<String>>),
    Boolean(bool),
    Function(Rc<RefCell<Function>>),
    Object(Rc<RefCell<dyn ObjectLike>>),
    List(Rc<RefCell<Vec<ScriptValue>>>),
    None,
    Unit,
}

impl ScriptValue {
    pub fn numeric(&self, other: ScriptValue, operator: Token) -> ExpressionResult {
        match (self, &other) {
            (ScriptValue::Number(left), ScriptValue::Number(right)) => {
                let result = match operator.token_type {
                    TokenType::Plus => left + right,
                    TokenType::Minus => left - right,
                    TokenType::Star => left * right,
                    TokenType::Slash => left / right,
                    _ => return Err(InterpreterError::other(&self, "Impossible addition")),
                };

                Ok(ScriptValue::Number(result))
            }
            // _ => panic!("Cannot {:?} {:?} and {:?}", operator.token_type, self, other),
            _ => Err(InterpreterError::other(&self, "Cannot operate"))
        }
    }

    pub fn boolean(&self, other: ScriptValue, operator: Token) -> ExpressionResult {
        let result = match (self, &other) {
            (ScriptValue::Number(left), ScriptValue::Number(right)) => match operator.token_type {
                TokenType::Equals => left == right,
                TokenType::NotEquals => left != right,
                TokenType::Lesser => left < right,
                TokenType::Greater => left > right,
                TokenType::EqLesser => left <= right,
                TokenType::EqGreater => left >= right,
                _ => panic!("Impossible boolean operation"),
            },
            (ScriptValue::Boolean(left), ScriptValue::Boolean(right)) => match operator.token_type {
                TokenType::Equals => left == right,
                TokenType::NotEquals => left != right,
                _ => panic!("Impossible boolean operation"),
            },
            _ => panic!("Cannot compare {:?} and {:?}", self, other),
        };

        Ok(ScriptValue::Boolean(result))
    }
}

impl Expression for ScriptValue {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ExpressionResult {
        visitor.visit_value(&self)
    }
}

impl fmt::Display for ScriptValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptValue::Number(n) => write!(f, "{}", n),
            ScriptValue::Boolean(b) => write!(f, "{}", b),
            ScriptValue::Function(_) => write!(f, "Func"),
            ScriptValue::Object(o) => write!(f, "{}", o.borrow()),
            ScriptValue::String(s) => write!(f, "{}", s.borrow()),
            ScriptValue::List(l) => write!(f, "{:?}", *l.borrow()),
            ScriptValue::None => write!(f, "null"),
            ScriptValue::Unit => write!(f, "()"),
        }
    }
}

#[derive(Debug)]
pub struct VariableExpression {
    pub identifier: String,
}

impl Expression for VariableExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ExpressionResult {
        visitor.visit_variable(&self)
    }

    fn assign(&self, interpreter: &mut Interpreter, value: ScriptValue) -> ExpressionResult {
        interpreter.env.put(&self.identifier, value.clone());
        Ok(ScriptValue::Unit)
    }
}

#[derive(Debug)]
pub struct ConditionExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token,
}

impl Expression for ConditionExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ExpressionResult {
        visitor.visit_condition(&self)
    }
}

#[derive(Debug)]
pub struct AdditionExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token,
}

impl Expression for AdditionExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ExpressionResult {
        visitor.visit_addition(&self)
    }
}

#[derive(Debug)]
pub struct MultiplicationExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token,
}

impl Expression for MultiplicationExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ExpressionResult {
        visitor.visit_multiplication(&self)
    }
}

#[derive(Debug)]
pub struct FunctionExpression {
    pub expr: Box<dyn Expression>,
    pub params: Vec<Box<dyn Expression>>,
}

impl Expression for FunctionExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ExpressionResult {
        visitor.visit_function(&self)
    }
}

#[derive(Debug)]
pub struct AccessExpression {
    pub expr: Box<dyn Expression>,
    pub field: String,
}

impl Expression for AccessExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ExpressionResult {
        visitor.visit_access(self)
    }

    fn assign(&self, interpreter: &mut Interpreter, value: ScriptValue) -> ExpressionResult {
        let target = self.expr.accept(interpreter)?;
        match target {
            ScriptValue::Object(obj) => {
                Object::set_ref(obj, self.field.clone(), value);
                Ok(ScriptValue::Unit)
            }
            _ => panic!("{:?} is not an object", target),
        }
    }
}

#[derive(Debug)]
pub struct IndexExpression {
    pub expr: Box<dyn Expression>,
    pub index_expr: Box<dyn Expression>,
}

impl Expression for IndexExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ExpressionResult {
        visitor.visit_index(self)
    }

    fn assign(&self, interpreter: &mut Interpreter, value: ScriptValue) -> ExpressionResult{
        let target = self.expr.accept(interpreter)?;
        match target {
            ScriptValue::List(list) => {
                let index = self.index_expr.accept(interpreter)?;
                match index {
                    ScriptValue::Number(n) => {
                        list.borrow_mut()[n as usize] = value;
                        Ok(ScriptValue::Unit)
                    }
                    _ => panic!("Cannot use {} as index", index),
                }
            }
            _ => panic!("{:?} is not an object", target),
        }
    }
}
