use crate::tokenizer::{Literal, Scanner, Token, TokenType};

#[test]
fn logical_and_bitwise_operators() {
    let mut scanner = Scanner::new("& && | ||");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::BitAnd, "&", 1, Literal::None),
            Token::new(TokenType::And, "&&", 1, Literal::None),
            Token::new(TokenType::BitOr, "|", 1, Literal::None),
            Token::new(TokenType::Or, "||", 1, Literal::None),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}