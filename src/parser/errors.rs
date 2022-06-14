use std::fmt;
use crate::token::{Token,TokenType};


#[derive(Debug,Clone)]
pub enum ParserErrorType {
    EOF,
    Unexpected(String),
}
// TODO actual info to parserError
#[derive(Debug, Clone)]
pub struct ParserError {
    pub token: Option<Token>,
    pub err_type: ParserErrorType,
}

impl ParserError {
    pub fn unexpected(token: &Token, should_be: &str) -> ParserError {
        ParserError {
            token: Some(token.clone()),
            err_type: ParserErrorType::Unexpected(should_be.to_string())
        }
    }

    pub fn unexpected_token(token: &Token, should_be: TokenType) -> ParserError {
        ParserError {
            token: Some(token.clone()),
            err_type: ParserErrorType::Unexpected(format!("{:?}", should_be))
        }
    }

    pub fn eof() -> ParserError {
        ParserError {
            token: None,
            err_type: ParserErrorType::EOF
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.err_type {
            ParserErrorType::EOF => {
                write!(f, "Reached end of line while parsing")
            },
            ParserErrorType::Unexpected(expected) => {
                let err_string = if let Some(token) = self.token.clone() {
                    format!("Line {}: expected {}, found {:?}", token.line, expected, token.token_type)
                } else {
                    format!("Expected {}, reached EOF", expected)
                };

                write!(f, "{}", err_string)
            }
        }
    }
}
