
#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Identifier(String),
    Number(f64),
    Plus,
    Minus,
    Star,
    Assign,
    // LineBreak,
    Slash,
    Var,
    LeftParen,
    RightParen
}

