#[derive(PartialEq, Clone, Debug)]
pub enum Token {
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

pub trait Should<T> {
    fn should_be(&self, other: T);
}

impl Should<&Token> for Option<&Token> {
    fn should_be(&self, other: &Token) {
        if let Some(t) = self {
            if *t != other {
                panic!("Expected {:?}, got {:?}", t, other);
            }
        } else {
            panic!("Expected {:?}, was None", other);
        }
    }
}
