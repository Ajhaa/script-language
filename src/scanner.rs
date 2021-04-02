use crate::token::Token;

use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Scanner {
    input: Peekable<IntoIter<char>>,
    tokens: Vec<Token>
}

impl Scanner {
    pub fn new(string: String) -> Scanner {
        let chars: Vec<char> = string.chars().collect(); 
        Scanner { tokens: Vec::new(), input: chars.into_iter().peekable() }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    pub fn scan(&mut self) -> Vec<Token> {
        loop {
            let token = self.next_token();
            self.tokens.push(token);
            if let None = self.peek() {
                break;
            }
        }
        self.tokens.clone()
    }

    fn next_token(&mut self) -> Token {
        let next = self.input.next().unwrap();

        match next {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '=' => Token::Assign,
            '/' => Token::Slash,
            '\n' => Token::LineBreak,
            'A'..='Z' | 'a'..='z' | '_' => self.word(next),
            '0'..='9' => self.number(next),
            ' ' | '\r' => self.next_token(),
            _ => panic!("Unexpected {}", next)  
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

        // TODO check keywords
        Token::Identifier(s)
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