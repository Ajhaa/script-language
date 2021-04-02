use crate::token::Token;
use crate::expression::*;

use std::iter::Peekable;
use std::vec::IntoIter;

type Program = Vec<Box<dyn Expression>>;

pub struct Parser {
    input: Peekable<IntoIter<Token>>,
    program: Program
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let program = Vec::new() ;
        Parser { input: tokens.into_iter().peekable(), program}
    }

    pub fn parse(&mut self) -> &Program {
        loop {
            let expr = self.expression();
            self.program.push(expr);
            if let None = self.peek() {
                break;
            }
        }
        &self.program
    }

    fn peek(&mut self) -> Option<&Token> {
        return self.input.peek();
    }

    fn consume(&mut self) -> Option<Token> {
        return self.input.next();
    }

    fn expect(&mut self, value: &Token) {
        let val = &self.consume().unwrap();
        if val != value {
            panic!("Token apua");
        }
    }

    fn expression(&mut self) -> Box<dyn Expression> {
        // let token = self.consume().unwrap();
        // match token {
        //     Token::Identifier(ident) => self.assignment(ident),
        //     Token::Number(x) => Box::new(NumberExpression { value: x.parse().unwrap() }),
        //     _ => panic!("Not implemented")
        // }

        self.addition()
    }

    fn assignment(&mut self, identifier: String) -> Box<AssignmentExpression> {
        self.expect(&Token::Assign);
        let value = self.expression();
        return Box::new(AssignmentExpression { identifier: identifier.clone(), value: value })
    }

    fn addition(&mut self) -> Box<dyn Expression> {
        let left = self.multiplication();
        
        let next = self.peek();

        if let Some(token) = next {
            match token {
                Token::Plus => (),
                Token::Minus => (),
                _ => return left
            }

            let operator = self.consume().unwrap();

            let right = self.addition();
            return Box::new(AdditionExpression{ left, right, operator });
        }
        
        return left;
    }

    fn multiplication(&mut self) -> Box<dyn Expression> {
        let left = self.number();

        let next = self.peek();

        if let Some(token) = next {
            match token {
                Token::Star => (),
                Token::Slash => (),
                _ => return left
            }
            let operator = self.consume().unwrap();

            let right = self.multiplication();
            return Box::new(MultiplicationExpression{ left, right, operator });
        }
        
        return left;
    }

    fn number(&mut self) -> Box<NumberExpression> {
        let num = self.consume();
        if let Some(token) = num {
            if let Token::Number(x) = token {
                return Box::new(NumberExpression { value: x });
            } else {
                panic!("expected number, got {:?}", token)
            }
        } else {
            panic!("EOF while parsing add")
        }
    }
}