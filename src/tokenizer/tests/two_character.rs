use crate::tokenizer::{Literal, Token, TokenType, scanner::Scanner};
use pretty_assertions::assert_eq;

#[test]
fn two_character() {
    let mut scanner = Scanner::new("==<>!===<<=>>");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::EqualEqual, "==", 1, Literal::None),
            Token::new(TokenType::Less, "<", 1, Literal::None),
            Token::new(TokenType::Greater, ">", 1, Literal::None),
            Token::new(TokenType::BangEqual, "!=", 1, Literal::None),
            Token::new(TokenType::EqualEqual, "==", 1, Literal::None),
            Token::new(TokenType::Less, "<", 1, Literal::None),
            Token::new(TokenType::LessEqual, "<=", 1, Literal::None),
            Token::new(TokenType::Greater, ">", 1, Literal::None),
            Token::new(TokenType::Greater, ">", 1, Literal::None),
            Token::new(TokenType::Eof, "", 1, Literal::None)
        ]
    )
}
