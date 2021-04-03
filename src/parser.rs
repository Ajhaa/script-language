use crate::token::Token;
use crate::expression::*;
use crate::statement::*;

use std::iter::Peekable;
use std::vec::IntoIter;

type Program = Vec<Box<dyn Statement>>;

pub struct Parser {
    input: Peekable<IntoIter<Token>>,
    program: Program
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let program = Vec::new();
        Parser { input: tokens.into_iter().peekable(), program}
    }

    pub fn parse(&mut self) -> &mut Program {
        loop {
            let stmt = self.statement();
            self.program.push(stmt);
            if let None = self.peek() {
                break;
            }
        }
        &mut self.program
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

    fn statement(&mut self) -> Box<dyn Statement> {
        let next = self.peek().unwrap();
        match next {
            Token::Var => {
                self.consume();
                // TODO multi var
                let var = self.consume().unwrap();
                if let Token::Identifier(ident) = var {
                    self.expect(&Token::Assign);
                    let expr = self.expression();

                    Box::new(DeclarationStatement { variables: vec![ident], initializer: Some(expr) })
                } else {
                    panic!("stmt paininik");
                }
            },
            Token::If => {
                self.consume();
                let condition = self.expression();
                let if_body = self.statement();

                let else_body = if let Some(Token::Else) = self.peek() {
                    self.consume();
                    Some(self.statement())
                } else {
                    None
                };

                Box::new(IfStatement { condition, if_body, else_body })
            },
            Token::LeftBracket => {
                self.consume();
                let mut body = Vec::new();
                while let Some(token) = self.peek() {
                    if token == &Token::RightBracket {
                        self.consume();
                        break;
                    }
                    let stmt = self.statement();
                    body.push(stmt);
                }

                Box::new(BlockStatement { body })
            }
            _ => Box::new(ExpressionStatement { expr: self.expression() })
        }
    }

    fn expression(&mut self) -> Box<dyn Expression> {
        // let token = self.consume().unwrap();
        // match token {
        //     Token::Identifier(ident) => self.assignment(ident),
        //     Token::Number(x) => Box::new(NumberExpression { value: x.parse().unwrap() }),
        //     _ => panic!("Not implemented")
        // }

        self.condition()
    }

    fn _assignment(&mut self, identifier: String) -> Box<AssignmentExpression> {
        self.expect(&Token::Assign);
        let value = self.expression();
        return Box::new(AssignmentExpression { identifier: identifier.clone(), value: value })
    }

    fn condition(&mut self) -> Box<dyn Expression> {
        let left = self.addition();

        let next = self.peek();

        if let Some(token) = next {
            match token {
                Token::Equals => (),
                Token::NotEquals => (),
                Token::Greater => (),
                Token::Lesser => (),
                Token::EqGreater => (),
                Token::EqLesser => (),
                _ => return left
            }

            let operator = self.consume().unwrap();

            let right = self.condition();
            return Box::new(ConditionExpression{ left, right, operator });
        }
        
        return left;
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
        let left = self.factor();

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

    fn factor(&mut self) -> Box<dyn Expression> {
        let next = self.consume().unwrap();

        match next {
            Token::Number(value) => Box::new(NumberExpression { value }),
            Token::Identifier(identifier) => Box::new(VariableExpression { identifier }),
            Token::LeftParen => {
                let expr = self.expression();
                self.expect(&Token::RightParen);
                expr
            }
            _ => panic!("Not a factor: {:?}", next)
        }
    }

    // fn number(&mut self, x: f64) -> Box<NumberExpression> {
    //     let num = self.consume();
    //     if let Some(token) = num {
    //         if let Token::Number(x) = token {
    //             return Box::new(NumberExpression { value: x });
    //         } else {
    //             panic!("expected number, got {:?}", token)
    //         }
    //     } else {
    //         panic!("EOF while parsing add")
    //     }
    // }
}