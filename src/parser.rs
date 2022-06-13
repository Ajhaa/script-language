use crate::expression::*;
use crate::statement::*;
use crate::token::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

type Program = Vec<Box<dyn Statement>>;

// TODO actual info to parserError
#[derive(Debug, Clone)]
pub struct ParserError;

type ExpressionResult = Result<Box<dyn Expression>, ParserError>;

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse error")
    }
}

pub struct Parser {
    //input: MultiPeek<IntoIter<&'a Token>>,
    input: Tokens,
    program: Program,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            input: Tokens::new(tokens),
            program: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<&mut Program, ParserError> {
        loop {
            let stmt = self.statement()?;
            self.program.push(stmt);
            if let None = self.current() {
                break;
            }
        }
        Ok(&mut self.program)
    }

    fn current(&mut self) -> Option<&Token> {
        return self.input.current();
    }

    fn _lookahead(&mut self) -> Option<&Token> {
        return self.input._peek();
    }

    fn advance(&mut self) -> Option<&Token> {
        self.input.advance()
    }

    fn consume(&mut self) -> Option<&Token> {
        self.input.consume()
    }

    fn _skip(&mut self, amount: usize) -> Option<&Token> {
        self.input._skip(amount)
    }

    fn statement(&mut self) -> Result<Box<dyn Statement>, ParserError> {
        let current = self.current();
        let stmt: Box<dyn Statement> = match current.unwrap() {
            Token::Var => {
                // TODO multi var
                self.advance();
                let var = self.consume().unwrap();
                if let Token::Identifier(ident) = var {
                    let identifier = ident.to_owned();
                    self.consume().should_be(&Token::Assign);
                    let expr = self.expression()?;

                    Box::new(DeclarationStatement {
                        variables: vec![identifier],
                        initializer: Some(expr),
                    })
                } else {
                    return Err(ParserError);
                }
            }
            Token::Identifier(_) => {
                let expr = self.expression()?;
                if let Some(&Token::Assign) = self.current() {
                    self.consume();
                    let value = self.expression()?;
                    Box::new(AssignmentStatement {
                        assignee: expr,
                        expr: value,
                    })

                    // self.consume();
                } else {
                    Box::new(ExpressionStatement { expr })
                }
            }
            Token::If => {
                self.advance();
                let condition = self.expression()?;
                let if_body = self.statement()?;

                let else_body = if let Some(Token::Else) = self.current() {
                    self.advance();
                    Some(self.statement()?)
                } else {
                    None
                };

                Box::new(IfStatement {
                    condition,
                    if_body,
                    else_body,
                })
            }
            Token::While => {
                self.advance();
                let condition = self.expression()?;
                let body = self.statement()?;

                Box::new(WhileStatement { condition, body })
            }
            Token::Func => {
                let next = self.advance();
                if let Some(Token::Identifier(ident)) = next {
                    let name = ident.to_owned();

                    self.advance().should_be(&Token::LeftParen);
                    self.advance();
                    let mut params = Vec::new();
                    while let Some(Token::Identifier(ident)) = self.current() {
                        params.push(ident.clone());
                        if let Some(Token::Comma) = self.advance() {
                            self.consume();
                        } else {
                            break;
                        }
                    }
                    self.consume().should_be(&Token::RightParen);
                    let body = self.statement()?;

                    Box::new(FunctionStatement {
                        name,
                        params,
                        body: Rc::from(body),
                    })
                } else {
                    return Err(ParserError);
                }
            }
            Token::Return => {
                self.consume();
                let expr = self.expression()?;
                Box::new(ReturnStatement { expr })
            }
            Token::LeftBracket => {
                self.advance();
                let mut body = Vec::new();
                while let Some(token) = self.current() {
                    if token == &Token::RightBracket {
                        self.advance();
                        break;
                    }
                    let stmt = self.statement()?;
                    body.push(stmt);
                }

                Box::new(BlockStatement { body })
            }
            _ => Box::new(ExpressionStatement {
                expr: self.expression()?,
            }),
        };

        Ok(stmt)
    }

    fn expression(&mut self) -> ExpressionResult {
        self.condition()
    }

    fn condition(&mut self) -> ExpressionResult {
        let left = self.addition()?;

        let next = self.current();

        if let Some(token) = next {
            match token {
                Token::Equals => (),
                Token::NotEquals => (),
                Token::Greater => (),
                Token::Lesser => (),
                Token::EqGreater => (),
                Token::EqLesser => (),
                _ => return Ok(left),
            }

            // TODO consume etc error handling
            let operator = self.consume().unwrap().clone();

            let right = self.condition()?;
            return Ok(Box::new(ConditionExpression {
                left,
                right,
                operator,
            }));
        }

        return Ok(left);
    }

    fn addition(&mut self) -> ExpressionResult {
        let left = self.multiplication()?;

        let next = self.current();

        if let Some(token) = next {
            match token {
                Token::Plus => (),
                Token::Minus => (),
                _ => return Ok(left),
            }

            let operator = self.consume().unwrap().clone();

            let right = self.addition()?;
            return Ok(Box::new(AdditionExpression {
                left,
                right,
                operator,
            }));
        }

        return Ok(left);
    }

    fn multiplication(&mut self) -> ExpressionResult {
        let left = self.factor()?;

        let next = self.current();

        if let Some(token) = next {
            match token {
                Token::Star => (),
                Token::Slash => (),
                _ => return Ok(left),
            }

            let operator = self.consume().unwrap().clone();

            let right = self.multiplication()?;
            return Ok(Box::new(MultiplicationExpression {
                left,
                right,
                operator,
            }));
        }

        return Ok(left);
    }

    fn factor(&mut self) -> ExpressionResult {
        let next = self.consume().unwrap();

        let factor: Box<dyn Expression> = match next {
            Token::Number(value) => Box::new(ScriptValue::Number(*value)),
            Token::String(string) => Box::new(ScriptValue::String(Rc::new(RefCell::new(
                string.to_owned(),
            )))),
            Token::Boolean(b) => Box::new(ScriptValue::Boolean(*b)),
            Token::None => Box::new(ScriptValue::None),
            Token::Identifier(identifier) => {
                let ident = identifier.to_owned();

                Box::new(VariableExpression { identifier: ident })
            }
            Token::LeftParen => {
                let expr = self.expression()?;
                self.consume().should_be(&Token::RightParen);
                expr
            }
            _ => panic!("Not a factor: {:?}", next),
        };

        self.call_and_access(factor)
    }

    fn call_and_access(&mut self, base: Box<dyn Expression>) -> ExpressionResult {
        let call = if let Some(Token::LeftParen) = self.current() {
            self.advance();
            let mut params = Vec::new();
            while let Some(token) = self.current() {
                if token == &Token::RightParen {
                    break;
                }

                let expr = self.expression()?;
                params.push(expr);

                if let Some(Token::Comma) = self.current() {
                    self.consume();
                } else {
                    break;
                }
            }
            self.consume().should_be(&Token::RightParen);
            let new_base = Box::new(FunctionExpression { expr: base, params });
            self.call_and_access(new_base)?
        } else {
            base
        };

        let index = if let Some(Token::LeftBrace) = self.current() {
            self.advance();
            let index_expr = self.expression()?;
            self.consume().should_be(&Token::RightBrace);

            let new_base = Box::new(IndexExpression {
                expr: call,
                index_expr,
            });
            self.call_and_access(new_base)?
        } else {
            call
        };

        if let Some(Token::Dot) = self.current() {
            self.advance();
            match self.consume() {
                Some(Token::Identifier(ident)) => {
                    let new_base = Box::new(AccessExpression {
                        expr: index,
                        field: ident.to_owned(),
                    });
                    self.call_and_access(new_base)
                }
                // Some(other) => panic!("Cannot access {:?}", other),
                // None => panic!("Unexpected EOF when parsing"),
                Some(other) => Err(ParserError),
                None => Err(ParserError)
            }
        } else {
            Ok(index)
        }
    }
}
