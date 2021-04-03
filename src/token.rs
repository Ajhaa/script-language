
#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Identifier(String),
    Number(f64),
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
    If,
    Else,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    // TODO better solution,
    Nothing
}

