use crate::{parser::parse, tokenizer::TokenType};

use super::*;

#[test]
fn test_control_flow_if_else() {
    // if (true) { a = 1; } else { a = 2; }
    let tokens = make_tokens(vec![
        tok(TokenType::If, "if"),
        tok(TokenType::LeftParen, "("),
        tok(TokenType::True, "true"),
        tok(TokenType::RightParen, ")"),
        tok(TokenType::LeftBrace, "{"),
        ident("a"),
        tok(TokenType::Equal, "="),
        num(1.0),
        tok(TokenType::Semicolon, ";"),
        tok(TokenType::RightBrace, "}"),
        tok(TokenType::Else, "else"),
        tok(TokenType::LeftBrace, "{"),
        ident("a"),
        tok(TokenType::Equal, "="),
        num(2.0),
        tok(TokenType::Semicolon, ";"),
        tok(TokenType::RightBrace, "}"),
    ]);

    let result = parse(tokens);
    assert!(result.is_ok());
}
