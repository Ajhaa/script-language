use crate::expression::*;
use crate::statement::*;
use crate::token::*;
use errors::*;

use std::cell::RefCell;
use std::rc::Rc;

pub mod errors;

type Program = Vec<Box<dyn Statement>>;


type ExpressionResult = Result<Box<dyn Expression>, ParserError>;


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
       let current = self.current().ok_or(ParserError::eof())?.clone();

        let stmt: Box<dyn Statement> = match current.token_type {
            TokenType::Var => {
                // TODO multi var
                self.advance();
                let var = self.consume().ok_or(ParserError::eof())?;
                if let TokenType::Identifier(ident) = &var.token_type {
                    let identifier = ident.to_owned();
                    self.consume().should_be(TokenType::Assign)?;
                    let expr = self.expression()?;

                    Box::new(DeclarationStatement {
                        variables: vec![identifier],
                        initializer: Some(expr),
                    })
                } else {
                    return Err(ParserError::unexpected(&current, "identifier"));
                }
            }
            TokenType::Identifier(_) => {
                let expr = self.expression()?;
                if let Some(TokenType::Assign) = self.current().unwrap_type() {
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
            TokenType::If => {
                self.advance();
                let condition = self.expression()?;
                let if_body = self.statement()?;

                let else_body = if let Some(TokenType::Else) = self.current().unwrap_type() {
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
            TokenType::While => {
                self.advance();
                let condition = self.expression()?;
                let body = self.statement()?;

                Box::new(WhileStatement { condition, body })
            }
            TokenType::Func => {
                let next = self.advance();
                if let Some(TokenType::Identifier(ident)) = next.unwrap_type() {
                    let name = ident.to_owned();

                    self.advance().should_be(TokenType::LeftParen)?;
                    self.advance();
                    let mut params = Vec::new();
                    while let Some(TokenType::Identifier(ident)) = self.current().unwrap_type() {
                        params.push(ident.clone());
                        if let Some(TokenType::Comma) = self.advance().unwrap_type() {
                            self.consume();
                        } else {
                            break;
                        }
                    }
                    self.consume().should_be(TokenType::RightParen)?;
                    let body = self.statement()?;

                    Box::new(FunctionStatement {
                        name,
                        params,
                        body: Rc::from(body),
                    })
                } else {
                    return Err(ParserError::unexpected(&current, "identifier"));
                }
            }
            TokenType::Return => {
                self.consume();
                let expr = self.expression()?;
                Box::new(ReturnStatement { expr })
            }
            TokenType::LeftBracket => {
                self.advance();
                let mut body = Vec::new();
                while let Some(token) = self.current() {
                    if token.token_type == TokenType::RightBracket {
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
            match token.token_type {
                TokenType::Equals => (),
                TokenType::NotEquals => (),
                TokenType::Greater => (),
                TokenType::Lesser => (),
                TokenType::EqGreater => (),
                TokenType::EqLesser => (),
                _ => return Ok(left),
            }

            // TODO consume etc error handling
            let operator = self.consume().ok_or(ParserError::eof())?.clone();

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
            match token.token_type {
                TokenType::Plus => (),
                TokenType::Minus => (),
                _ => return Ok(left),
            }

            let operator = self.consume().ok_or(ParserError::eof())?.clone();

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
            match token.token_type {
                TokenType::Star => (),
                TokenType::Slash => (),
                _ => return Ok(left),
            }

            let operator = self.consume().ok_or(ParserError::eof())?.clone();

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
        let next = self.consume().ok_or(ParserError::eof())?;

        let factor: Box<dyn Expression> = match &next.token_type {
            TokenType::Number(value) => Box::new(ScriptValue::Number(*value)),
            TokenType::String(string) => Box::new(ScriptValue::String(Rc::new(RefCell::new(
                string.to_owned(),
            )))),
            TokenType::Boolean(b) => Box::new(ScriptValue::Boolean(*b)),
            TokenType::None => Box::new(ScriptValue::None),
            TokenType::Identifier(identifier) => {
                let ident = identifier.to_owned();

                Box::new(VariableExpression { identifier: ident })
            }
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume().should_be(TokenType::RightParen)?;
                expr
            }
            //_ => panic!("Not a factor: {:?}", next),
            _ => return Err(ParserError::unexpected(next, "factor"))
        };

        self.call_and_access(factor)
    }

    fn call_and_access(&mut self, base: Box<dyn Expression>) -> ExpressionResult {
        let call = if let Some(TokenType::LeftParen) = self.current().unwrap_type() {
            self.advance();
            let mut params = Vec::new();
            while let Some(token) = self.current() {
                if token.token_type == TokenType::RightParen {
                    break;
                }

                let expr = self.expression()?;
                params.push(expr);

                if let Some(TokenType::Comma) = self.current().unwrap_type() {
                    self.consume();
                } else {
                    break;
                }
            }
            self.consume().should_be(TokenType::RightParen)?;
            let new_base = Box::new(FunctionExpression { expr: base, params });
            self.call_and_access(new_base)?
        } else {
            base
        };

        let index = if let Some(TokenType::LeftBrace) = self.current().unwrap_type() {
            self.advance();
            let index_expr = self.expression()?;
            self.consume().should_be(TokenType::RightBrace)?;

            let new_base = Box::new(IndexExpression {
                expr: call,
                index_expr,
            });
            self.call_and_access(new_base)?
        } else {
            call
        };

        if let Some(TokenType::Dot) = self.current().unwrap_type() {
            self.advance();
            let current = self.consume();
            match current.unwrap_type() {
                Some(TokenType::Identifier(ident)) => {
                    let new_base = Box::new(AccessExpression {
                        expr: index,
                        field: ident.to_owned(),
                    });
                    self.call_and_access(new_base)
                }
                // Some(other) => panic!("Cannot access {:?}", other),
                // None => panic!("Unexpected EOF when parsing"),
                Some(_) => Err(ParserError::unexpected(current.unwrap(), "object")),
                None => Err(ParserError::eof())
            }
        } else {
            Ok(index)
        }
    }
}
