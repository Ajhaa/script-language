mod builtin;
mod environment;
mod expression;
mod function;
mod interpreter;
mod object;
mod parser;
mod scanner;
mod statement;
mod token;

use builtin::create_builtins;
use environment::Environment;
use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input = fs::read_to_string(file_name).unwrap();
    let scanner = Scanner::new(input);
    let tokens = scanner.scan();

    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(result) => result,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    let mut env = Environment::new();
    create_builtins(&mut env);

    let mut interpreter = Interpreter { env };
    let result = interpreter.exec(program);

    if let Err(error) = result {
        println!("{}", error);
        process::exit(1);
    }
}
