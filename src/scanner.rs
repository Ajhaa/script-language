use crate::token::{Token,TokenType};

use std::array;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Scanner<'a> {
    line: usize,
    input: Peekable<IntoIter<char>>,
    tokens: Vec<Token>,
    keywords: HashMap<&'a str, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(string: String) -> Scanner<'a> {
        let chars: Vec<char> = string.chars().collect();
        Scanner {
            line: 1,
            tokens: Vec::new(),
            input: chars.into_iter().peekable(),
            keywords: HashMap::<_, _>::from_iter(array::IntoIter::new([
                ("var", TokenType::Var),
                ("if", TokenType::If),
                ("else", TokenType::Else),
                ("fn", TokenType::Func),
                ("while", TokenType::While),
                ("true", TokenType::Boolean(true)),
                ("false", TokenType::Boolean(false)),
                ("null", TokenType::None),
                ("return", TokenType::Return),
            ])),
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn consume(&mut self) -> Option<char> {
        self.input.next()
    }

    fn match_or(&mut self, should: char, result: TokenType, default: TokenType) -> TokenType {
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
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Star,
            '/' => TokenType::Slash,
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBracket,
            '}' => TokenType::RightBracket,
            '[' => TokenType::LeftBrace,
            ']' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '=' => self.match_or('=', TokenType::Equals, TokenType::Assign),
            '<' => self.match_or('=', TokenType::EqLesser, TokenType::Lesser),
            '>' => self.match_or('=', TokenType::EqGreater, TokenType::Greater),
            '!' => self.match_or('=', TokenType::NotEquals, TokenType::Not),
            '&' => self.match_or('&', TokenType::And, TokenType::BitAnd),
            '|' => self.match_or('|', TokenType::Or, TokenType::BitOr),
            //'\n' => Token::LineBreak,
            'A'..='Z' | 'a'..='z' | '_' => self.word(next),
            '0'..='9' => self.number(next),
            '"' => self.string(),
            '\n' => {
                self.line += 1;
                TokenType::Nothing
            },
            ' ' | '\r' => TokenType::Nothing,
            _ => panic!("Unexpected {} at line {}", next, self.line),
        };

        if let TokenType::Nothing = token {
            None
        } else {
            Some(Token { tokenType: token, line: self.line, col: 0 })
        }
    }

    fn string(&mut self) -> TokenType {
        let mut s = String::new();

        while let Some(c) = self.input.next() {
            if c == '"' {
                break;
            }

            s.push(c);
        }

        TokenType::String(s)
    }

    fn word(&mut self, first: char) -> TokenType {
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
            TokenType::Identifier(s)
        }
    }

    fn number(&mut self, first: char) -> TokenType {
        let mut s = String::from(first);

        while let Some(c) = self.input.peek() {
            match c {
                '0'..='9' => s.push(self.input.next().unwrap()),
                _ => break,
            }
        }

        TokenType::Number(s.parse().unwrap())
    }
}
