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

use std::env;
use std::fs;

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

    env.create_internal_function("print", vec!["target".to_owned()],
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

    let mut interpreter = Interpreter { env };
    interpreter.exec(program);
}
