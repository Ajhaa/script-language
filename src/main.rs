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
use std::rc::Rc;

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

    env.put_new("print", ScriptValue::Function(
        Function::new(
            vec!["target".to_owned()],
            Rc::new(Box::new(WriteStatement { expr: Box::new(VariableExpression { identifier: "target".to_owned() }) })),
            Rc::clone(&env.env)
        )
    ));

    env.put_new("Object", ScriptValue::Function(
        Function::new(
            Vec::new(),
            // Rc::new(Box::new(WriteStatement { expr: Box::new(VariableExpression { identifier: "target".to_owned() }) })),
            Rc::new(Box::new(ExpressionStatement { expr: Box::new(ScriptValue::Object(Object::new()))})),
            Rc::clone(&env.env)
        )
    ));

    let mut interpreter = Interpreter { env };
    interpreter.exec(program);

    //println!("{:?}", interpreter.env.dump());
}
