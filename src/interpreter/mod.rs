use crate::environment::*;
use crate::expression::*;
use crate::function::*;
use crate::statement::*;

use errors::{InterpreterError, InterpreterErrorType};

use std::rc::Rc;

pub mod errors;

pub type StatementResult = Result<StatementValue, InterpreterError>;
pub type ExpressionResult = Result<ScriptValue, InterpreterError>;
pub struct Interpreter {
    pub env: Environment,
}

impl Interpreter {
    pub fn exec(&mut self, program: &Vec<Box<dyn Statement>>) -> Result<(), InterpreterError> {
        for stmt in program {
            stmt.accept(self)?;
        }

        Ok(())
    }
}

impl StatementVisitor for Interpreter {
    fn visit_assignment(&mut self, stmt: &AssignmentStatement) -> StatementResult {
        let value = stmt.expr.accept(self)?;

        stmt.assignee.assign(self, value);

        Ok(StatementValue::Normal(ScriptValue::Unit))
    }

    fn visit_declaration(&mut self, stmt: &DeclarationStatement) -> StatementResult {
        let value = if let Some(expr) = &stmt.initializer {
            expr.accept(self)?
        } else {
            ScriptValue::None
        };

        for var in &stmt.variables {
            self.env.put_new(&var, value.clone());
        }

        Ok(StatementValue::Normal(ScriptValue::Unit))
    }

    fn visit_block(&mut self, stmt: &BlockStatement) -> StatementResult {
        self.env.enter();
        for stmt in &stmt.body {
            let ret = stmt.accept(self)?;
            if let StatementValue::Normal(_) = ret {
                continue;
            } else {
                self.env.exit();
                return Ok(ret);
            }
        }

        self.env.exit();

        Ok(StatementValue::Normal(ScriptValue::Unit))
    }

    fn visit_function(&mut self, stmt: &FunctionStatement) -> StatementResult {
        self.env.enter();
        let func = Function::new(
            stmt.params.clone(),
            stmt.body.clone(),
            Rc::clone(&self.env.env),
        );
        self.env.exit();
        self.env.put_new(&stmt.name, ScriptValue::Function(func));

        Ok(StatementValue::Normal(ScriptValue::Unit))
    }

    fn visit_if(&mut self, stmt: &IfStatement) -> StatementResult {
        if let ScriptValue::Boolean(true) = stmt.condition.accept(self)? {
            stmt.if_body.accept(self)
        } else if let Some(else_body) = &stmt.else_body {
            else_body.accept(self)
        } else {
            Ok(StatementValue::Normal(ScriptValue::Unit))
        }
    }

    fn visit_while(&mut self, stmt: &WhileStatement) -> StatementResult {
        while let ScriptValue::Boolean(true) = stmt.condition.accept(self)? {
            let res = stmt.body.accept(self)?;
            if let StatementValue::Return(_) = res {
                return Ok(res);
            }
        }

        Ok(StatementValue::Normal(ScriptValue::Unit))
    }

    fn visit_return(&mut self, stmt: &ReturnStatement) -> StatementResult {
        Ok(
            StatementValue::Return(
                stmt.expr.accept(self)?
            )
        )
    }

    fn visit_expression(&mut self, stmt: &ExpressionStatement) -> StatementResult {
        Ok(StatementValue::Normal(stmt.expr.accept(self)?))
    }

    fn visit_internal(&mut self, stmt: &InternalStatement) -> StatementResult {
        (stmt.func)(self)
    }
}

impl ExpressionVisitor for Interpreter {
    fn visit_addition(&mut self, expr: &AdditionExpression) -> ExpressionResult {
        let left = expr.left.accept(self)?;
        let right = expr.right.accept(self)?;
        left.numeric(right, expr.operator.clone())
    }

    fn visit_multiplication(&mut self, expr: &MultiplicationExpression) -> ExpressionResult {
        let left = expr.left.accept(self)?;
        let right = expr.right.accept(self)?;
        left.numeric(right, expr.operator.clone())
    }

    fn visit_value(&mut self, expr: &ScriptValue) -> ExpressionResult {
        Ok(expr.clone())
    }

    fn visit_variable(&mut self, expr: &VariableExpression) -> ExpressionResult {
        match self.env.get(&expr.identifier) {
            Some(var) => Ok(var.clone()),
            //None => panic!("variable not found {}", &expr.identifier),
            None => Err(InterpreterError::new(&ScriptValue::None, InterpreterErrorType::UndefinedVariable(expr.identifier.clone())))
        }
    }

    fn visit_condition(&mut self, expr: &ConditionExpression) -> ExpressionResult {
        let left = expr.left.accept(self)?;
        let right = expr.right.accept(self)?;
        left.boolean(right, expr.operator.clone())
    }

    fn visit_function(&mut self, expr: &FunctionExpression) -> ExpressionResult {
        let target = expr.expr.accept(self)?;
        self.env.enter();
        let val = match target {
            ScriptValue::Function(func) => {
                let f = func.borrow();
                //let mut wrapper = Environment { env: Rc::clone(&f.env) };
                //wrapper.enter();
                let ret = f.call(self, &expr.params)?;
                // wrapper.exit();
                ret
            }
            _ => return Err(InterpreterError::new(&target, InterpreterErrorType::NotCallable))
        };
        self.env.exit();
        Ok(val)
    }

    fn visit_access(&mut self, expr: &AccessExpression) -> ExpressionResult {
        let target = expr.expr.accept(self)?;
        match &target {
            ScriptValue::Object(obj) => match obj.borrow().get(&expr.field) {
                Some(val) => Ok(val),
                //None => panic!("Object has no property {}", &expr.field),
                None => Err(InterpreterError::new(&target, InterpreterErrorType::PropertyNotFound))
            },
            _ => Err(InterpreterError::new(&target, InterpreterErrorType::NotObject))
        }
    }

    fn visit_index(&mut self, expr: &IndexExpression) -> ExpressionResult {
        let target = expr.expr.accept(self)?;
        match &target {
            ScriptValue::List(list) => {
                let index = expr.index_expr.accept(self)?;

                match index {
                    ScriptValue::Number(n) => Ok(list.borrow()[n as usize].clone()),
                    // _ => panic!("Index has to be a number, not {:?}", index),
                    _ => Err(InterpreterError::new(&target, InterpreterErrorType::InvalidIndex))
                }
            }
            _ => Err(InterpreterError::new(&target, InterpreterErrorType::NotIndexable))
        }
    }
}
