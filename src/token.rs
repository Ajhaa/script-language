use crate::parser::ParserError;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Plus,
    Minus,
    Star,
    Slash,
    Assign,
    And,
    Or,
    Not,
    BitAnd,
    BitOr,
    Equals,
    NotEquals,
    Greater,
    Lesser,
    EqGreater,
    EqLesser,
    // LineBreak,
    Var,
    Func,
    If,
    Else,
    While,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Return,
    None,
    // TODO better solution,
    Nothing,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub tokenType: TokenType,
    pub line: usize,
    pub col: usize
}

pub struct Tokens {
    input: Vec<Token>,
    index: usize,
}

impl Tokens {
    pub fn new(vec: Vec<Token>) -> Tokens {
        Tokens {
            input: vec,
            index: 0,
        }
    }

    pub fn current(&self) -> Option<&Token> {
        self.input.get(self.index)
    }

    pub fn consume(&mut self) -> Option<&Token> {
        let curr = self.index;
        self.index += 1;
        self.input.get(curr)
    }

    pub fn advance(&mut self) -> Option<&Token> {
        self.index += 1;
        self.input.get(self.index)
    }

    pub fn _peek(&self) -> Option<&Token> {
        self.input.get(self.index + 1)
    }

    pub fn _skip(&mut self, amount: usize) -> Option<&Token> {
        self.index += amount;
        self.current()
    }
}

pub trait TokenTrait {
    fn should_be(&self, other: TokenType) -> Result<(), ParserError>;
    fn might_be(&self, other: TokenType) -> Option<()>;
    fn unwrap_type(&self) -> Option<TokenType>;

}

impl TokenTrait for Option<&Token> {
    fn should_be(&self, other: TokenType) -> Result<(), ParserError> {
        if let Some(t) = self {
            if t.tokenType != other {
                // panic!("Expected {:?}, got {:?}", t, other);
                return Err(ParserError)
            }
        } else {
            // panic!("Expected {:?}, was None", other);
            return Err(ParserError)
        }

        Ok(())
    }

    fn might_be(&self, other: TokenType) -> Option<()> {
        if let Some(t) = self {
            if t.tokenType != other {
                // panic!("Expected {:?}, got {:?}", t, other);
                return None;
            }
        } else {
            // panic!("Expected {:?}, was None", other);
            return None;
        }

        Some(())
    }

    fn unwrap_type(&self) -> Option<TokenType> {
        if let Some(t) = self {
            Some(t.tokenType.clone())
        } else {
            None
        }
    }
}
