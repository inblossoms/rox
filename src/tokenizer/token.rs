use std::fmt::Display;

use super::Literal;
use super::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub literal: Literal,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme)
    }
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

#[derive(Debug)]
pub struct Tokens {
    pub tokens: Vec<Token>,
}
