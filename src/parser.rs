use crate::token::Token;
use crate::expression::*;
use crate::statement::*;

use std::iter::Peekable;
use std::vec::IntoIter;

type Program = Vec<Box<dyn Statement>>;

pub struct Parser<'a> {
    input: Peekable<IntoIter<&'a Token>>,
    program: Program,
    current: Option<&'a Token>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        let program = Vec::new();
        let mut refs = Vec::new();
        for token in tokens {
            refs.push(token);
        }
        Parser { input: refs.into_iter().peekable(), program, current: None }
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

    fn peek(&mut self) -> Option<&&Token> {
        return self.input.peek();
    }

    fn consume(&mut self) {
        self.current = self.input.next();
    }

    fn expect(&mut self, value: Token) {
        self.consume();
        if self.current.unwrap() != &value {
            panic!("Expected {:?}, got {:?}", value, self.current.unwrap());
        }
    }

    fn statement(&mut self) -> Box<dyn Statement> {
        self.consume();
        match self.current.unwrap() {
            Token::Var => {
                // TODO multi var
                self.consume();
                let var = self.current.unwrap();
                if let Token::Identifier(ident) = var {
                    self.expect(Token::Assign);
                    self.consume();
                    let expr = self.expression();

                    Box::new(DeclarationStatement { variables: vec![ident.to_owned()], initializer: Some(expr) })
                } else {
                    panic!("stmt paininik");
                }
            },
            Token::Identifier(identifier) => {
                //let identifier = ident.to_owned();
                if let Some(Token::Assign) = self.peek() {
                    self.consume();
                    self.consume();
                    let expr = self.expression();

                    Box::new(AssignmentStatement { identifier: identifier.to_owned(), expr })
                } else {
                    Box::new(ExpressionStatement { expr: self.expression() })
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
            Token::While => {
                self.consume();
                let condition = self.expression();
                let body = self.statement();

                Box::new(WhileStatement { condition, body })
            }
            // Token::Func => {
            //     self.consume();
            //     self.expect(&Token::LeftParen);

            // },
            Token::LeftBracket => {
                let mut body = Vec::new();
                while let Some(token) = self.peek() {
                    if token == &&Token::RightBracket {
                        self.consume();
                        break;
                    }
                    let stmt = self.statement();
                    body.push(stmt);
                }

                Box::new(BlockStatement { body })
            }
            _ => {
                Box::new(ExpressionStatement { expr: self.expression() })
            }
        }
    }

    fn expression(&mut self) -> Box<dyn Expression> {
        self.condition()
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

            self.consume();
            let operator = self.current.unwrap().clone();

            self.consume();
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

            self.consume();
            let operator = self.current.unwrap().clone();

            self.consume();
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

            self.consume();
            let operator = self.current.unwrap().clone();

            self.consume();
            let right = self.multiplication();
            return Box::new(MultiplicationExpression{ left, right, operator });
        }
        
        return left;
    }

    fn factor(&mut self) -> Box<dyn Expression> {
        let next = self.current.unwrap();

        match next {
            Token::Number(value) => Box::new(NumberExpression { value: *value }),
            Token::Identifier(identifier) => Box::new(VariableExpression { identifier: identifier.to_owned() }),
            Token::LeftParen => {
                self.consume();
                let expr = self.expression();
                self.expect(Token::RightParen);
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