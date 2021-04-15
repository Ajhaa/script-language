
#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    None,
    Plus,
    Minus,
    Star,
    Not,
    Assign,
    Equals,
    NotEquals,
    Greater,
    Lesser,
    EqGreater,
    EqLesser,
    // LineBreak,
    Slash,
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
    Return,
    Dot,
    // TODO better solution,
    Nothing
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

