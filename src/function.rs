use crate::{environment::*, expression::*, interpreter::*, statement::*};

use std::{cell::RefCell, fmt, rc::Rc};

pub struct Function {
    pub params: Vec<String>,
    pub body: Rc<dyn Statement>,
    pub env: Environment,
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Func({})", self.params.join(","))
    }
}

impl Function {
    pub fn new(
        params: Vec<String>,
        body: Rc<dyn Statement>,
        env: Rc<RefCell<Env>>,
    ) -> Rc<RefCell<Function>> {
        let env = Environment { env };
        Rc::new(RefCell::new(Function { params, body, env }))
    }

    pub fn call(&self, base: &mut Interpreter, params: &Vec<Box<dyn Expression>>) -> ExpressionResult {
        let mut interpreter = Interpreter {
            env: self.env.clone(),
        };
        interpreter.env.enter();
        for i in 0..self.params.len() {
            let val = params[i].accept(base)?;
            let key = self.params[i].clone();
            interpreter.env.put_new(&key, val);
        }
        let val = (*self.body).accept(&mut interpreter)?;
        interpreter.env.exit();

        Ok(
            match val {
                StatementValue::Normal(x) => x,
                StatementValue::Return(x) => x,
            }
        )
    }
}
