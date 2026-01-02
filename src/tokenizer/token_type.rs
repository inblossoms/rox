#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // single character
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    PlusEqual,
    MinusEqual,
    SlashEqual,
    StarEqual,
    Semicolon,
    Slash,
    Star,

    // one or two character
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    String,
    Number,

    // keywords
    BitAnd,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    BitOr,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Continue,
    Break,

    Eof,
}
