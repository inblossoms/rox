use crate::tokenizer::{Literal, Token, TokenType, scanner::Scanner};
use pretty_assertions::assert_eq;

#[test]
fn logical_and_bitwise_operators() {
    let mut scanner = Scanner::new("& && | ||");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::Ampersand, "&", 1, Literal::None),
            Token::new(TokenType::LogicalAnd, "&&", 1, Literal::None),
            Token::new(TokenType::Pipe, "|", 1, Literal::None),
            Token::new(TokenType::LogicalOr, "||", 1, Literal::None),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}
