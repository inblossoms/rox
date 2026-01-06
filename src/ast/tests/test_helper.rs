use crate::tokenizer::{Literal, Token, TokenType};

/// 快速生成 Identifier Token
pub fn token(name: &str) -> Token {
    Token {
        token_type: TokenType::Identifier,
        lexeme: name.to_string(),
        literal: Literal::Nil,
        line: 1,
    }
}

/// 快速生成 Return Token
pub fn token_return() -> Token {
    Token {
        token_type: TokenType::Return,
        lexeme: "return".to_string(),
        literal: Literal::Nil,
        line: 1,
    }
}
