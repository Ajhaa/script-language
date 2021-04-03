use crate::token::Token;
use crate::expression::*;
use crate::statement::*;

use itertools::structs::MultiPeek;
use itertools::multipeek;
use std::vec::IntoIter;

type Program = Vec<Box<dyn Statement>>;

pub struct Parser<'a> {
    input: MultiPeek<IntoIter<&'a Token>>,
    program: Program,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        let program = Vec::new();
        let mut refs = Vec::new();
        for token in tokens {
            refs.push(token);
        }
        Parser { input: multipeek(refs.into_iter()), program }
    }

    pub fn parse(&mut self) -> &mut Program {
        loop {
            let stmt = self.statement();
            self.program.push(stmt);
            if let None = self.current() {
                break;
            }
        }
        &mut self.program
    }

    fn current(&mut self) -> Option<&&Token> {
        self.reset_peek();
        return self.input.peek();
    }

    fn lookahead(&mut self) -> Option<&&Token> {
        return self.input.peek();
    }

    fn reset_peek(&mut self) {
        self.input.reset_peek();
    }

    fn advance(&mut self) -> Option<&Token> {
        self.input.next()
    }
    
    fn skip(&mut self, amount: usize) -> Option<&Token> {
        self.input.nth(amount - 1)
    }

    fn expect(&mut self, value: Token) {
        let val = self.advance().unwrap();
        if val != &value {
            panic!("Expected {:?}, got {:?}", value, val);
        }
    }

    fn statement(&mut self) -> Box<dyn Statement> {
        let next = self.current();
        // println!("Starting statement with {:?}", next);
        match next.unwrap() {
            Token::Var => {
                // TODO multi var
                self.advance();
                let var = self.advance().unwrap();
                if let Token::Identifier(ident) = var {
                    let identifier = ident.to_owned();
                    self.expect(Token::Assign);
                    let expr = self.expression();

                    Box::new(DeclarationStatement { variables: vec![identifier], initializer: Some(expr) })
                } else {
                    panic!("stmt paininik");
                }
            },
            Token::Identifier(ident) => {
                let identifier = ident.to_owned();
                if let Some(Token::Assign) = self.lookahead() {
                    self.skip(2);
                    //self.advance();
                    let expr = self.expression();

                    Box::new(AssignmentStatement { identifier: identifier, expr })
                } else {
                    // self.reset_peek();
                    Box::new(ExpressionStatement { expr: self.expression() })
                }
                
            },
            Token::If => {
                self.advance();
                let condition = self.expression();
                let if_body = self.statement();

                let else_body = if let Some(Token::Else) = self.current() {
                    self.advance();
                    Some(self.statement())
                } else {
                    None
                };

                Box::new(IfStatement { condition, if_body, else_body })
            },
            Token::While => {
                self.advance();
                let condition = self.expression();
                let body = self.statement();

                Box::new(WhileStatement { condition, body })
            },
            // Token::Func => {
            //     self.expect(&Token::LeftParen);

            // },
            Token::LeftBracket => {
                self.advance();
                let mut body = Vec::new();
                while let Some(token) = self.current() {
                    if token == &&Token::RightBracket {
                        self.advance();
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

        let next = self.current();

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

            // self.advance();
            let operator = self.advance().unwrap().clone();

            let right = self.condition();
            return Box::new(ConditionExpression{ left, right, operator });
        }
        
        return left;
    }

    fn addition(&mut self) -> Box<dyn Expression> {
        let left = self.multiplication();
        
        let next = self.current();

        if let Some(token) = next {
            match token {
                Token::Plus => (),
                Token::Minus => (),
                _ => return left
            }

            let operator = self.advance().unwrap().clone();

            let right = self.addition();
            return Box::new(AdditionExpression{ left, right, operator });
        }
        
        return left;
    }

    fn multiplication(&mut self) -> Box<dyn Expression> {
        let left = self.factor();

        let next = self.current();

        if let Some(token) = next {
            match token {
                Token::Star => (),
                Token::Slash => (),
                _ => return left
            }

            let operator = self.advance().unwrap().clone();

            let right = self.multiplication();
            return Box::new(MultiplicationExpression{ left, right, operator });
        }
        
        return left;
    }

    fn factor(&mut self) -> Box<dyn Expression> {
        let next = self.advance().unwrap();

        match next {
            Token::Number(value) => Box::new(NumberExpression { value: *value }),
            Token::Identifier(identifier) => Box::new(VariableExpression { identifier: identifier.to_owned() }),
            Token::LeftParen => {
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