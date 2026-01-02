use super::Literal;
use super::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Literal,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: impl Into<String>,
        line: usize,
        literal: Literal,
    ) -> Self {
        Self {
            token_type,
            lexeme: lexeme.into(),
            line,
            literal,
        }
    }
}
