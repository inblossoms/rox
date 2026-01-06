use crate::tokenizer::{Literal, Token, TokenType};

/// 用于在构造时快速生成虚拟 Token
pub fn generate_token(token_type: TokenType, lexeme: &str) -> Token {
    Token {
        token_type,
        lexeme: lexeme.to_string(),
        literal: Literal::Nil,
        line: 0,
    }
}
