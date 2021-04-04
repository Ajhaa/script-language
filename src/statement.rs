use crate::expression::*;
use crate::environment::Environment;

use std::cell::RefCell;
use std::rc::Rc;

pub enum StatementValue {
    Normal(ScriptValue),
    Return(ScriptValue)
}

pub trait Statement {
    fn exec(&self, env: &mut Environment) {
        self.eval(env);
    }

    fn eval(&self, env: &mut Environment) -> StatementValue {
        self.exec(env);
        StatementValue::Normal(ScriptValue::Unit)
    }
}

pub struct DeclarationStatement {
    pub variables: Vec<String>,
    pub initializer: Option<Box<dyn Expression>>
}

impl Statement for DeclarationStatement {
    fn exec(&self, env: &mut Environment) {
        
        let value = if let Some(expr) = &self.initializer {
            expr.eval(env)
        } else {
            ScriptValue::None
        };

        for var in &self.variables {
            env.put(&var, value.clone());
        }
    }
}

pub struct AssignmentStatement {
    pub identifier: String,
    pub expr: Box<dyn Expression>
}

impl Statement for AssignmentStatement {
    fn exec(&self, env: &mut Environment) {
        let value = self.expr.eval(env);
        env.put(&self.identifier, value);
    }
} 

pub struct IfStatement {
    pub condition: Box<dyn Expression>,
    pub if_body: Box<dyn Statement>,
    pub else_body: Option<Box<dyn Statement>>
}

impl Statement for IfStatement {
    fn eval(&self, env: &mut Environment) -> StatementValue {
        if let ScriptValue::Boolean(true) = self.condition.eval(env) {
            self.if_body.eval(env)
        } else if let Some(stmt) = &self.else_body {
            stmt.eval(env)
        } else {
            StatementValue::Normal(ScriptValue::Unit)
        }
    }
}

pub struct WhileStatement {
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Statement>,
}

impl Statement for WhileStatement {
    fn eval(&self, env: &mut Environment) -> StatementValue{
        while let ScriptValue::Boolean(true) = self.condition.eval(env) {
            let res = self.body.eval(env);
            if let StatementValue::Return(_) = res {
                return res
            }
        }

        StatementValue::Normal(ScriptValue::Unit)
    }
}

pub struct FunctionStatement {
    pub name: String,
    pub params: Vec<String>,
    pub body: Rc<Box<dyn Statement>>
}

impl Statement for FunctionStatement {
    fn exec(&self, env: &mut Environment) {
        let func = Function::new(self.params.clone(), self.body.clone(), RefCell::new(env.clone()));
        env.put(&self.name, ScriptValue::Function(func));
    }
}

pub struct ExpressionStatement {
    pub expr: Box<dyn Expression>
}

impl Statement for ExpressionStatement {
    fn eval(&self, env: &mut Environment) -> StatementValue {
        StatementValue::Normal(self.expr.eval(env))
    }
}

pub struct BlockStatement {
    pub body: Vec<Box<dyn Statement>>
}

impl Statement for BlockStatement {
    fn eval(&self, env: &mut Environment) -> StatementValue {
        for stmt in &self.body {
            let ret = stmt.eval(env);
            if let StatementValue::Normal(_) = ret {
                continue;
            } else {
                return ret
            }
        }

        StatementValue::Normal(ScriptValue::Unit)
    }
}

pub struct ReturnStatement {
    pub expr: Box<dyn Expression>
}

impl Statement for ReturnStatement {
    fn eval(&self, env: &mut Environment) -> StatementValue {
        StatementValue::Return(self.expr.eval(env))
    }
}