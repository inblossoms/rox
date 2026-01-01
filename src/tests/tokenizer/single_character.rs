use crate::tokenizer::{Literal, Scanner, Token, TokenType};

#[test]
fn single_character() {
    let mut scanner = Scanner::new("(){};,.-+*");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::LeftParen, "(", 1, Literal::None),
            Token::new(TokenType::RightParen, ")", 1, Literal::None),
            Token::new(TokenType::LeftBrace, "{", 1, Literal::None),
            Token::new(TokenType::RightBrace, "}", 1, Literal::None),
            Token::new(TokenType::Semicolon, ";", 1, Literal::None),
            Token::new(TokenType::Comma, ",", 1, Literal::None),
            Token::new(TokenType::Dot, ".", 1, Literal::None),
            Token::new(TokenType::Minus, "-", 1, Literal::None),
            Token::new(TokenType::Plus, "+", 1, Literal::None),
            Token::new(TokenType::Star, "*", 1, Literal::None),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}
