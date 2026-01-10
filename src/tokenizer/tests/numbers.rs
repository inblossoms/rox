use std::f64::consts;

use crate::tokenizer::{Literal, Token, TokenType, scanner::Scanner};
use pretty_assertions::assert_eq;

#[test]
fn integer() {
    let mut scanner = Scanner::new("42");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::Number, "42", 1, Literal::Number(42.0)),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}

#[test]
fn float() {
    let pi = consts::PI.to_string();
    let mut scanner = Scanner::new(&pi);

    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::Number, &pi, 1, Literal::Number(consts::PI)),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}

#[test]
fn leading_decimal_point() {
    let mut scanner = Scanner::new(".5");
    // This should fail since .5 doesn't start with a digit
    // The tokenizer currently only handles digits at the start
    // This test documents this behavior
    let result = scanner.scan_tokens();
    // This will likely produce an error or unexpected tokens
    // Since . is not handled as a number start
    assert!(result.is_ok());
    let tokens = result.unwrap().tokens;
    assert_eq!(tokens[0].token_type, TokenType::Dot);
}

#[test]
fn trailing_decimal_point() {
    let mut scanner = Scanner::new("5.");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::Number, "5.", 1, Literal::Number(5.0)),
            // Token::new(TokenType::Dot, ".", 1, Literal::None),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}

#[test]
fn zero_prefixed_number() {
    let mut scanner = Scanner::new("0755");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::Number, "0755", 1, Literal::Number(755.0)),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}
