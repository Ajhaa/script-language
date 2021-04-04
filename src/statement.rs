use crate::expression::*;
use crate::environment::Environment;

use std::rc::Rc;

pub trait Statement {
    fn exec(&self, env: &mut Environment);
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
    fn exec(&self, env: &mut Environment) {
        if let ScriptValue::Boolean(true) = self.condition.eval(env) {
            self.if_body.exec(env);
        } else if let Some(stmt) = &self.else_body {
            stmt.exec(env);
        }
    }
}

pub struct WhileStatement {
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Statement>,
}

impl Statement for WhileStatement {
    fn exec(&self, env: &mut Environment) {
        while let ScriptValue::Boolean(true) = self.condition.eval(env) {
            self.body.exec(env);
        }
    }
}

pub struct FunctionStatement {
    pub name: String,
    pub params: Vec<String>,
    pub body: Rc<Box<dyn Statement>>
}

impl Statement for FunctionStatement {
    fn exec(&self, env: &mut Environment) {
        let func = Function::new(self.params.clone(), self.body.clone());
        env.put(&self.name, ScriptValue::Function(func));
    }
}

pub struct ExpressionStatement {
    pub expr: Box<dyn Expression>
}

impl Statement for ExpressionStatement {
    fn exec(&self, env: &mut Environment) {
        println!("{}", self.expr.eval(env))
    }
}

pub struct BlockStatement {
    pub body: Vec<Box<dyn Statement>>
}

impl Statement for BlockStatement {
    fn exec(&self, env: &mut Environment) {
        for stmt in &self.body {
            stmt.exec(env);
        }
    }
}