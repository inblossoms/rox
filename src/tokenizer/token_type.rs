#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // single character
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
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
    LogicalAnd, // &&
    LogicalOr,
    Ampersand, // &
    Pipe,
    Xor,
    And, // and
    Or,
    Percent,

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
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Continue,
    Break,
    Try,
    Catch,
    Throw,
    Export,

    Eof,
}
