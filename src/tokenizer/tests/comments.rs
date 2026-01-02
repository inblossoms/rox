use crate::tokenizer::{Literal, Token, TokenType, scanner::Scanner};

#[test]
fn single_line_comments() {
    let mut scanner = Scanner::new("// This is a comment\nvar x = 1;");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::Var, "var", 2, Literal::None),
            Token::new(TokenType::Identifier, "x", 2, Literal::None),
            Token::new(TokenType::Equal, "=", 2, Literal::None),
            Token::new(TokenType::Number, "1", 2, Literal::Number(1.0)),
            Token::new(TokenType::Semicolon, ";", 2, Literal::None),
            Token::new(TokenType::Eof, "", 2, Literal::None),
        ]
    )
}

#[test]
fn multi_line_comments() {
    let mut scanner = Scanner::new("/* This is a\nmulti-line comment */ 42");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::Number, "42", 2, Literal::Number(42.0)),
            Token::new(TokenType::Eof, "", 2, Literal::None),
        ]
    )
}

#[test]
fn mixed_comments_and_code() {
    let mut scanner = Scanner::new("var x = 10; // comment\n/* another */ var y = 20;");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::Var, "var", 1, Literal::None),
            Token::new(TokenType::Identifier, "x", 1, Literal::None),
            Token::new(TokenType::Equal, "=", 1, Literal::None),
            Token::new(TokenType::Number, "10", 1, Literal::Number(10.0)),
            Token::new(TokenType::Semicolon, ";", 1, Literal::None),
            Token::new(TokenType::Var, "var", 2, Literal::None),
            Token::new(TokenType::Identifier, "y", 2, Literal::None),
            Token::new(TokenType::Equal, "=", 2, Literal::None),
            Token::new(TokenType::Number, "20", 2, Literal::Number(20.0)),
            Token::new(TokenType::Semicolon, ";", 2, Literal::None),
            Token::new(TokenType::Eof, "", 2, Literal::None),
        ]
    )
}
