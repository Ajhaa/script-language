use crate::token::Token;

use std::array;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Scanner<'a> {
    input: Peekable<IntoIter<char>>,
    tokens: Vec<Token>,
    keywords: HashMap<&'a str, Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(string: String) -> Scanner<'a> {
        let chars: Vec<char> = string.chars().collect();
        Scanner {
            tokens: Vec::new(),
            input: chars.into_iter().peekable(),
            keywords: HashMap::<_, _>::from_iter(array::IntoIter::new([
                ("var", Token::Var),
                ("if", Token::If),
                ("else", Token::Else),
                ("fn", Token::Func),
                ("while", Token::While),
                ("true", Token::Boolean(true)),
                ("false", Token::Boolean(false)),
                ("null", Token::None),
                ("return", Token::Return),
            ])),
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn consume(&mut self) -> Option<char> {
        self.input.next()
    }

    fn match_or(&mut self, should: char, result: Token, default: Token) -> Token {
        if let Some(token) = self.peek() {
            if token == &should {
                self.consume();
                return result;
            }
        }

        default
    }

    pub fn scan(mut self) -> Vec<Token> {
        loop {
            if let Some(token) = self.next_token() {
                self.tokens.push(token);
            }
            if let None = self.peek() {
                break;
            }
        }
        self.tokens
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
            '[' => Token::LeftBrace,
            ']' => Token::RightBrace,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '=' => self.match_or('=', Token::Equals, Token::Assign),
            '<' => self.match_or('=', Token::EqLesser, Token::Lesser),
            '>' => self.match_or('=', Token::EqGreater, Token::Greater),
            '!' => self.match_or('=', Token::NotEquals, Token::Not),
            '&' => self.match_or('&', Token::And, Token::BitAnd),
            '|' => self.match_or('|', Token::Or, Token::BitOr),
            //'\n' => Token::LineBreak,
            'A'..='Z' | 'a'..='z' | '_' => self.word(next),
            '0'..='9' => self.number(next),
            '"' => self.string(),
            ' ' | '\r' | '\n' => Token::Nothing,
            _ => panic!("Unexpected {}", next),
        };

        if let Token::Nothing = token {
            None
        } else {
            Some(token)
        }
    }

    fn string(&mut self) -> Token {
        let mut s = String::new();

        while let Some(c) = self.input.next() {
            if c == '"' {
                break;
            }

            s.push(c);
        }

        Token::String(s)
    }

    fn word(&mut self, first: char) -> Token {
        let mut s = String::from(first);

        while let Some(c) = self.input.peek() {
            match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => s.push(self.input.next().unwrap()),
                _ => break,
            }
        }

        if let Some(keyword) = self.keywords.get(&*s) {
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
                _ => break,
            }
        }

        Token::Number(s.parse().unwrap())
    }
}
