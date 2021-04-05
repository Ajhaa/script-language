mod token;
mod scanner;
mod expression;
mod statement;
mod parser;
mod environment;
mod interpreter;

use scanner::Scanner;
use parser::Parser;
use environment::Environment;
use interpreter::Interpreter;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input = fs::read_to_string(file_name).unwrap();

    let mut scanner = Scanner::new(input);

    let tokens = scanner.scan();

    println!("{:?}\n-----", tokens);

    let mut parser = Parser::new(tokens);

    let program = parser.parse();

    let env = Environment::new();

    let mut interpreter = Interpreter { env };
    interpreter.exec(program);

    println!("{:?}", interpreter.env.dump());
}
