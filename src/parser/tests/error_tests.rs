use super::*;
use crate::{parser::parse, tokenizer::TokenType};

#[test]
fn test_error_invalid_assignment_target() {
    // 错误：1 = 2; (不能给左值赋值)
    let tokens = make_tokens(vec![
        num(1.0),
        tok(TokenType::Equal, "="),
        num(2.0),
        tok(TokenType::Semicolon, ";"),
    ]);

    let result = parse(tokens);
    assert!(result.is_err(), "检测到无效的赋值目标");
    let err = result.err().unwrap();
    assert_eq!(err.message, "[line 1]: Invalid assignment target.");
}

#[test]
fn test_error_missing_semicolon() {
    // 错误：var a = 1 (缺少分号)
    let tokens = make_tokens(vec![
        tok(TokenType::Var, "var"),
        ident("a"),
        tok(TokenType::Equal, "="),
        num(1.0),
        // Missing Semicolon
    ]);

    let result = parse(tokens);
    assert!(result.is_err());
    assert!(result.err().unwrap().message.contains("Expect ';'"));
}

#[test]
fn test_error_unbalanced_paren() {
    // 错误：(1 + 2
    let tokens = make_tokens(vec![
        tok(TokenType::LeftParen, "("),
        num(1.0),
        tok(TokenType::Plus, "+"),
        num(2.0),
        tok(TokenType::Semicolon, ";"),
    ]);

    let result = parse(tokens);
    assert!(result.is_err());
    assert!(result.err().unwrap().message.contains("Expected ')'"));
}

#[test]
fn test_error_missing_expression_binary() {
    // 错误：1 + ;
    let tokens = make_tokens(vec![
        num(1.0),
        tok(TokenType::Plus, "+"),
        tok(TokenType::Semicolon, ";"),
    ]);

    let result = parse(tokens);
    assert!(result.is_err());
    assert!(result.err().unwrap().message.contains("Expected a primary expression (boolean, number, string, identifier, or grouping expression)."));
}
