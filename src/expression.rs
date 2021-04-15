use crate::token::Token;
use crate::environment::*;
use crate::statement::*;
use crate::object::*;
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
    fn visit_access(&mut self, expr: &AccessExpression) -> ScriptValue;
    fn visit_index(&mut self, expr: &IndexExpression) -> ScriptValue;
}

pub struct Function {
    pub params: Vec<String>,
    pub body: Rc<Box<dyn Statement>>,
    // pub env: Rc<RefCell<Option<Env>>>
    pub env: Environment
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Func({})", self.params.join(","))
    }
}

impl Function {
    pub fn new(params: Vec<String>, body: Rc<Box<dyn Statement>>, env: Rc<RefCell<Option<Env>>>) -> Rc<RefCell<Function>> {
        let env = Environment { env };
        Rc::new(RefCell::new(Function { params, body, env }))
    }

    pub fn call(&self, base: &mut Interpreter, params: &Vec<Box<dyn Expression>>) -> ScriptValue {
        let mut interpreter = Interpreter { env: self.env.clone() };
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
    String(Rc<RefCell<String>>),
    Boolean(bool),
    Function(Rc<RefCell<Function>>),
    Object(Rc<RefCell<dyn ObjectLike>>),
    List(Rc<RefCell<Vec<ScriptValue>>>),
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
            ScriptValue::Function(_) => write!(f, "Func"),
            ScriptValue::Object(o) => write!(f, "{}", o.borrow()),
            ScriptValue::String(s) => write!(f, "{}", s.borrow()),
            ScriptValue::List(l) => write!(f, "{:?}", *l.borrow()),
            ScriptValue::None => write!(f, "null"),
            ScriptValue::Unit => write!(f, "()")
        }        
    }
}

pub trait Expression: fmt::Debug {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ScriptValue;

    fn assign(&self, _: &mut Interpreter, _: ScriptValue) {
        panic!("Cannot assign to {:?}", self);
    }
}


#[derive(Debug)]
pub struct VariableExpression {
    pub identifier: String
}

impl Expression for VariableExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ScriptValue {
        visitor.visit_variable(&self)
    }

    fn assign(&self, interpreter: &mut Interpreter, value: ScriptValue) {
        interpreter.env.put(&self.identifier, value.clone());
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

#[derive(Debug)]
pub struct AccessExpression {
    pub expr: Box<dyn Expression>,
    pub field: String
}

impl Expression for AccessExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ScriptValue {
        visitor.visit_access(self)
    }

    fn assign(&self, interpreter: &mut Interpreter, value: ScriptValue) {
        let target = self.expr.accept(interpreter);
        match target {
            ScriptValue::Object(obj) => {
                // obj.borrow_mut().set(self.field.clone(), value.clone());
                // Object::set_ref(obj, self.field.clone(), value);
                Object::set_ref(obj, self.field.clone(), value);
            },
            _ => panic!("{:?} is not an object", target)
        };
    }
}

#[derive(Debug)]
pub struct IndexExpression {
    pub expr: Box<dyn Expression>,
    pub index_expr: Box<dyn Expression>
}

impl Expression for IndexExpression {
    fn accept(&self, visitor: &mut dyn ExpressionVisitor) -> ScriptValue {
        visitor.visit_index(self)
    }

    fn assign(&self, interpreter: &mut Interpreter, value: ScriptValue) {
        let target = self.expr.accept(interpreter);
        match target {
            ScriptValue::List(list) => {
                let index = self.index_expr.accept(interpreter);
                match index {
                    ScriptValue::Number(n) => {
                        list.borrow_mut()[n as usize] = value;
                    },
                    _ => panic!("Cannot use {} as index", index)
                }
            },
            _ => panic!("{:?} is not an object", target)
        };
    }
}