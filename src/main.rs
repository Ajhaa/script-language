mod token;
mod scanner;
mod expression;
mod statement;
mod parser;
mod environment;
mod interpreter;
mod object;
mod function;
mod builtin;

use scanner::Scanner;
use parser::Parser;
use environment::Environment;
use interpreter::Interpreter;
use builtin::create_builtins;

use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];

    let input = fs::read_to_string(file_name).unwrap();
    let scanner = Scanner::new(input);
    let tokens = scanner.scan();

    let mut parser = Parser::new(tokens);
    let program = parser.parse();

    let mut env = Environment::new();
    create_builtins(&mut env);

    let mut interpreter = Interpreter { env };
    interpreter.exec(program);
}
