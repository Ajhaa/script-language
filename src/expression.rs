use crate::token::Token;
use crate::environment::*;
use crate::statement::*;
use crate::interpreter::Interpreter;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

pub trait ExpressionVisitor {
    fn visit_variable(&mut self, expr: &VariableExpression) -> ScriptValue;
    fn visit_value(&mut self, expr: &ScriptValue) -> ScriptValue;
    fn visit_addition(&mut self, expr: &AdditionExpression) -> ScriptValue;
    fn visit_multiplication(&mut self, expr: &MultiplicationExpression) -> ScriptValue;
    fn visit_condition(&mut self, expr: &ConditionExpression) -> ScriptValue;
    fn visit_function(&mut self, expr: &FunctionExpression) -> ScriptValue;
}

pub struct Function {
    pub params: Vec<String>,
    pub body: Rc<Box<dyn Statement>>,
    pub env: Rc<RefCell<Option<Env>>>
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Func({})", self.params.join(","))
    }
}

impl Function {
    pub fn new(params: Vec<String>, body: Rc<Box<dyn Statement>>, env: Rc<RefCell<Option<Env>>>) -> Rc<RefCell<Function>> {
        Rc::new(RefCell::new(Function { params, body, env }))
    }

    pub fn call(&self, base: &mut Interpreter, params: &Vec<Box<dyn Expression>>) -> ScriptValue {
        let mut interpreter = Interpreter { env: Environment { env: Rc::clone(&self.env) }};
        interpreter.env.enter();
        for i in 0 .. self.params.len() {
            let val = params[i].accept(base);
            let key = self.params[i].clone();
            interpreter.env.put_new(&key, val);
        }
        let val = (*self.body).accept(&mut interpreter);
        interpreter.env.exit();
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
    String(Rc<String>),
    Boolean(bool),
    Function(Rc<RefCell<Function>>),
    None,
    Unit
}

impl ScriptValue {
    // TODO better name
    pub fn numeric(&self, other: ScriptValue, operator: Token) -> ScriptValue { 
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
            _ => panic!("Cannot {:?} {:?} and {:?}", operator, self, other)
        }
    }

    pub fn boolean(&self, other: ScriptValue, operator: Token) -> ScriptValue {
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
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ScriptValue {
        visitor.visit_value(&self)
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
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ScriptValue;
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
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ScriptValue {
        visitor.visit_variable(&self)
    }
}

#[derive(Debug)]
pub struct ConditionExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token
}

impl Expression for ConditionExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ScriptValue {
        visitor.visit_condition(&self)
    }
}

#[derive(Debug)]
pub struct AdditionExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token
}

impl Expression for AdditionExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ScriptValue {
        visitor.visit_addition(&self)
    }
}

#[derive(Debug)]
pub struct MultiplicationExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: Token
}

impl Expression for MultiplicationExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ScriptValue {
        visitor.visit_multiplication(&self)
    }
}


#[derive(Debug)]
pub struct FunctionExpression {
    pub expr: Box<dyn Expression>,
    pub params: Vec<Box<dyn Expression>>
}

impl Expression for FunctionExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ScriptValue {
        visitor.visit_function(&self)
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