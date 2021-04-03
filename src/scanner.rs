use crate::token::Token;

use std::iter::Peekable;
use std::vec::IntoIter;
use std::array;
use std::iter::FromIterator;
use std::collections::HashMap;


pub struct Scanner {
    input: Peekable<IntoIter<char>>,
    tokens: Vec<Token>,
    keywords: HashMap<String, Token>
}

impl Scanner {
    pub fn new(string: String) -> Scanner {
        let chars: Vec<char> = string.chars().collect(); 
        Scanner { 
            tokens: Vec::new(), 
            input: chars.into_iter().peekable(),
            keywords: HashMap::<_,_>::from_iter(array::IntoIter::new([
                ("var".to_owned(), Token::Var), 
                ("if".to_owned(), Token::If),
                ("else".to_owned(), Token::Else)
            ]))
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn consume(&mut self) -> Option<char> {
        self.input.next()
    }

    pub fn scan(&mut self) -> Vec<Token> {
        loop {
            if let Some(token) = self.next_token() {
                self.tokens.push(token);
            }
            if let None = self.peek() {
                break;
            }
        }
        self.tokens.clone()
    }

    fn next_token(&mut self) -> Option<Token> {
        let next = self.input.next().unwrap();

        let token = match next {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBracket,
            '}' => Token::RightBracket,
            '=' => {
                if let Some('=') = self.peek() {
                    self.consume();
                    Token::Equals
                } else {
                    Token::Assign
                }
            },
            '<' => {
                if let Some('=') = self.peek() {
                    self.consume();
                    Token::EqLesser
                } else {
                    Token::Lesser
                }
            },
            '>' => {
                if let Some('=') = self.peek() {
                    self.consume();
                    Token::EqGreater
                } else {
                    Token::Greater
                }
            },
            '!' => {
                if let Some('=') = self.peek() {
                    self.consume();
                    Token::NotEquals
                } else {
                    Token::Not
                }
            },
            //'\n' => Token::LineBreak,
            'A'..='Z' | 'a'..='z' | '_' => self.word(next),
            '0'..='9' => self.number(next),
            ' ' | '\r' | '\n' => Token::Nothing,
            _ => panic!("Unexpected {}", next)  
        };

        if let Token::Nothing = token {
            None
        } else {
            Some(token)
        }
    }

    fn word(&mut self, first: char) -> Token {
        let mut s = String::from(first);

        while let Some(c) = self.input.peek() {
            match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => s.push(self.input.next().unwrap()),
                _ => break
            }
        }

        if let Some(keyword) = self.keywords.get(&s) {
            keyword.clone()
        } else {
            Token::Identifier(s)
        }
    }

    fn number(&mut self, first: char) -> Token {
        let mut s = String::from(first);

        while let Some(c) = self.input.peek() {
            match c {
                '0'..='9' => s.push(self.input.next().unwrap()),
                _ => break
            }
        }

        Token::Number(s.parse().unwrap())
    }
}