mod token;
mod scanner;
mod expression;
mod statement;
mod parser;
mod environment;

use scanner::Scanner;
use parser::Parser;
use environment::Environment;

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

    let mut env = Environment::new();

    for stmt in program.iter() {
        stmt.exec(&mut env);
    }

    println!("{:?}", env.variables);
}
