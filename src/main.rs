mod token;
mod scanner;
mod expression;
mod statement;
mod parser;
mod environment;
mod interpreter;
mod object;

use scanner::Scanner;
use parser::Parser;
use expression::*;
use statement::*;
use environment::Environment;
use interpreter::Interpreter;
use object::Object;

use std::io::prelude::*;
use std::env;
use std::fs;
use std::cell::RefCell;
use std::rc::Rc;
use std::net::TcpListener;
use std::net::TcpStream;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input = fs::read_to_string(file_name).unwrap();

    let mut scanner = Scanner::new(input);

    let tokens = scanner.scan();

    //println!("{:?}\n-----", tokens);

    let mut parser = Parser::new(tokens);

    let program = parser.parse();

    let mut env = Environment::new();

    env.create_internal_function("print", vec!["target"],
        |inpr| {
            let val = inpr.env.get("target").unwrap();
            println!("{}", val);
            StatementValue::Normal(ScriptValue::Unit)
        }
    );

    env.create_internal_function("Object", Vec::new(), 
        |_| {
            StatementValue::Normal(
                ScriptValue::Object(Object::new())
            )
        }
    );

    env.create_internal_function("List", vec!["size"], 
        |inpr| {
            let size = match inpr.env.get("size") {
                Some(ScriptValue::Number(n)) => n as usize,
                Some(other) => panic!("Not a size {:?}", other),
                None => 0
            };

            StatementValue::Normal(
                ScriptValue::List(Rc::new(RefCell::new(vec!(ScriptValue::None; size))))
            )
        }
    );

    env.create_internal_function("map", vec!["func", "list"],
        |inpr| {
            let func = match inpr.env.get("func") {
                Some(ScriptValue::Function(f)) => f,
                Some(other) => panic!("Not a function {:?}", other),
                None => panic!("Expected args")
            };

            let list = match inpr.env.get("list") {
                Some(ScriptValue::List(l)) => l,
                Some(other) => panic!("Not a list {:?}", other),
                None => panic!("Expected 2 args")
            };

            let mapped: Vec<ScriptValue> = list.borrow().iter().map(|e| func.borrow_mut().call(inpr, &vec![Box::new(e.clone()) as Box<dyn Expression>])).collect();

            StatementValue::Normal(
                ScriptValue::List(Rc::new(RefCell::new(mapped)))
            )
        }
    );

    let mut interpreter = Interpreter { env };
    interpreter.exec(program);
}
