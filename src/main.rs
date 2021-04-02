mod token;
mod expression;
mod scanner;
mod parser;

use scanner::Scanner;
use parser::Parser;

fn main() {
    let input = String::from("2 * 2 / 3 - 1");
    let mut scanner = Scanner::new(input);

    let tokens = scanner.scan();

    println!("{:?}\n-----", tokens);

    let mut parser = Parser::new(tokens);

    let program = parser.parse();

    for expr in program {
        println!("{}", expr.eval());
    }
}
