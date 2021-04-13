use crate::expression::*;
use crate::statement::*;
use crate::environment::*;
use crate::object::*;

use std::rc::Rc;

pub struct Interpreter {
    pub env: Environment
}

impl Interpreter {
    pub fn exec(&mut self, program: &Vec<Box<dyn Statement>>) {
        for stmt in program {
            stmt.accept(self);
        }
    }
}

impl StatementVisitor for Interpreter {
    fn visit_assignment(&mut self, stmt: &AssignmentStatement) -> StatementValue {
        let value = stmt.expr.accept(self);

        stmt.assignee.assign(self, value);
        
        StatementValue::Normal(ScriptValue::Unit)
    }

    fn visit_declaration(&mut self, stmt: &DeclarationStatement) -> StatementValue {  
        let value = if let Some(expr) = &stmt.initializer {
            expr.accept(self)
        } else {
            ScriptValue::None
        };

        for var in &stmt.variables {
            self.env.put_new(&var, value.clone());
        }

        StatementValue::Normal(ScriptValue::Unit)
    }

    fn visit_block(&mut self, stmt: &BlockStatement) -> StatementValue {
        self.env.enter();
        for stmt in &stmt.body {
            let ret = stmt.accept(self);
            if let StatementValue::Normal(_) = ret {
                continue;
            } else {
                self.env.exit();
                return ret
            }
        }

        self.env.exit();

        StatementValue::Normal(ScriptValue::Unit)
    }

    // TODO function env
    fn visit_function(&mut self, stmt: &FunctionStatement) -> StatementValue {
        self.env.enter();
        let func = Function::new(stmt.params.clone(), stmt.body.clone(), Rc::clone(&self.env.env));
        self.env.exit();
        self.env.put_new(&stmt.name, ScriptValue::Function(func));

        StatementValue::Normal(ScriptValue::Unit)
    }

    fn visit_if(&mut self, stmt: &IfStatement) -> StatementValue {
        if let ScriptValue::Boolean(true) = stmt.condition.accept(self) {
            stmt.if_body.accept(self)
        } else if let Some(else_body) = &stmt.else_body {
            else_body.accept(self)
        } else {
            StatementValue::Normal(ScriptValue::Unit)
        }
    }

    fn visit_while(&mut self, stmt: &WhileStatement) -> StatementValue {
        while let ScriptValue::Boolean(true) = stmt.condition.accept(self) {
            let res = stmt.body.accept(self);
            if let StatementValue::Return(_) = res {
                return res
            }
        }

        StatementValue::Normal(ScriptValue::Unit)
    }

    fn visit_return(&mut self, stmt: &ReturnStatement) -> StatementValue {
        StatementValue::Return(stmt.expr.accept(self))
    }

    fn visit_expression(&mut self, stmt: &ExpressionStatement) -> StatementValue {
        StatementValue::Normal(stmt.expr.accept(self))
    }

    fn visit_write(&mut self, stmt: &WriteStatement) -> StatementValue {
        let val = stmt.expr.accept(self);
        // TODO possible to print without ln?
        println!("{}", val);

        StatementValue::Normal(ScriptValue::Unit)
    }

    fn visit_internal(&mut self, stmt: &InternalStatement) -> StatementValue {
        (stmt.func)(self)
    }

}

impl ExpressionVisitor for Interpreter {
    fn visit_addition(&mut self, expr: &AdditionExpression) -> ScriptValue {
        let left = expr.left.accept(self);
        let right = expr.right.accept(self);
        left.numeric(right, expr.operator.clone())
    }

    fn visit_multiplication(&mut self, expr: &MultiplicationExpression) -> ScriptValue {
        let left = expr.left.accept(self);
        let right = expr.right.accept(self);
        left.numeric(right, expr.operator.clone())
    }

    fn visit_value(&mut self, expr: &ScriptValue) -> ScriptValue {
        expr.clone()
    }

    fn visit_variable(&mut self, expr: &VariableExpression) -> ScriptValue {
        match self.env.get(&expr.identifier) {
            Some(var) => var.clone(),
            None => panic!("variable not found {}", &expr.identifier)
        }
    }

    fn visit_condition(&mut self, expr: &ConditionExpression) -> ScriptValue {
        let left = expr.left.accept(self);
        let right = expr.right.accept(self);
        left.boolean(right, expr.operator.clone())
    }

    fn visit_function(&mut self, expr: &FunctionExpression) -> ScriptValue {
        let target = expr.expr.accept(self);
        self.env.enter();
        let val = match target {
            ScriptValue::Function(func) => {
                let f = func.borrow();
                //let mut wrapper = Environment { env: Rc::clone(&f.env) };
                //wrapper.enter();
                let ret = f.call(self, &expr.params);
                // wrapper.exit();
                ret
            },
            _ => panic!("Cannot call {:?}", target)
        };
        self.env.exit();
        val
    }

    fn visit_access(&mut self, expr: &AccessExpression) -> ScriptValue {
        let target = expr.expr.accept(self);
        match target {
            ScriptValue::Object(obj) => {
                match obj.borrow().get(&expr.field) {
                    Some(val) => val,
                    None => panic!("Object has no property {}", &expr.field)
                }
            },
            _ => panic!("{:?} is not an object", target)
        }
    }
}