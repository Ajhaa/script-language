use crate::expression::*;
use crate::statement::*;
use crate::environment::*;

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
        let val = stmt.expr.accept(self);
        self.env.put(&stmt.identifier, val);
        
        StatementValue::Normal(ScriptValue::Unit)
    }

    fn visit_declaration(&mut self, stmt: &DeclarationStatement) -> StatementValue {  
        let value = if let Some(expr) = &stmt.initializer {
            expr.accept(self)
        } else {
            ScriptValue::None
        };

        for var in &stmt.variables {
            self.env.put(&var, value.clone());
        }

        StatementValue::Normal(ScriptValue::Unit)
    }

    fn visit_block(&mut self, stmt: &BlockStatement) -> StatementValue {
        for stmt in &stmt.body {
            let ret = stmt.accept(self);
            if let StatementValue::Normal(_) = ret {
                continue;
            } else {
                return ret
            }
        }

        StatementValue::Normal(ScriptValue::Unit)
    }

    // TODO function env
    fn visit_function(&mut self, stmt: &FunctionStatement) -> StatementValue {
        let func = Function::new(stmt.params.clone(), stmt.body.clone(), self.env.clone());
        self.env.put(&stmt.name, ScriptValue::Function(func));

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
        self.env.get(&expr.identifier).unwrap().clone()
    }

    fn visit_condition(&mut self, expr: &ConditionExpression) -> ScriptValue {
        let left = expr.left.accept(self);
        let right = expr.right.accept(self);
        left.boolean(right, expr.operator.clone())
    }

    fn visit_function(&mut self, expr: &FunctionExpression) -> ScriptValue {
        let target = self.env.get(&expr.name);
        match target {
            Some(ScriptValue::Function(func)) => {
                func.borrow().call(self, &expr.params)
            },
            _ => panic!("Cannot call {:?}", target)
        }
    }
}