use crate::{environment::Environment, expression::*, object::*, statement::*, interpreter::errors::InterpreterError};


use std::{cell::RefCell, rc::Rc};

pub fn create_builtins(env: &mut Environment) {
    env.create_internal_function("print", vec!["target"], |inpr| {
        let val = inpr.env.get("target").unwrap();
        println!("{}", val);
        Ok(StatementValue::Normal(ScriptValue::Unit))
    });

    env.create_internal_function("Object", Vec::new(), |_| {
        Ok(StatementValue::Normal(ScriptValue::Object(Object::new())))
    });

    env.create_internal_function("List", vec!["size"], |inpr| {
        let size = match inpr.env.get("size") {
            Some(ScriptValue::Number(n)) => n as usize,
            Some(other) => return Err(InterpreterError::other(&other, "Not a size")),
            None => 0,
        };

        Ok(StatementValue::Normal(ScriptValue::List(Rc::new(RefCell::new(
            vec![ScriptValue::None; size],
        )))))
    });

    env.create_internal_function("map", vec!["func", "list"], |inpr| {
        let func = match inpr.env.get("func") {
            Some(ScriptValue::Function(f)) => f,
            Some(other) => panic!("Not a function {:?}", other),
            None => panic!("Expected args"),
        };

        let list = match inpr.env.get("list") {
            Some(ScriptValue::List(l)) => l,
            Some(other) => panic!("Not a list {:?}", other),
            None => panic!("Expected 2 args"),
        };

        let mapped: Result<Vec<ScriptValue>, InterpreterError> = list
            .borrow()
            .iter()
            .map(|e| {
                func.borrow_mut()
                    .call(inpr, &vec![Box::new(e.clone()) as Box<dyn Expression>])
            })
            .collect();

        match mapped {
            Ok(result) => Ok(StatementValue::Normal(ScriptValue::List(Rc::new(RefCell::new(result))))),
            Err(error) => Err(error)
        }
    });
}
