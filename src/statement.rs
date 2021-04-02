use crate::expression::*;
use crate::environment::Environment;

pub trait Statement {
    fn exec(&mut self, env: &mut Environment);
}

pub struct DeclarationStatement {
    pub variables: Vec<String>,
    pub initializer: Option<Box<dyn Expression>>
}

impl Statement for DeclarationStatement {
    fn exec(&mut self, env: &mut Environment) {
        
        let value = if let Some(expr) = &self.initializer {
            Some(expr.eval(env))
        } else {
            None
        };

        for var in &self.variables {
            env.put(&var, value);
        }
    }
}

pub struct ExpressionStatement {
    pub expr: Box<dyn Expression>
}

impl Statement for ExpressionStatement {
    fn exec(&mut self, env: &mut Environment) {
        println!("{}", self.expr.eval(env))
    }
}