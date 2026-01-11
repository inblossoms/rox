use crate::tokenizer::{Literal, Token, TokenType, scanner::Scanner};
use pretty_assertions::assert_eq;

#[test]
fn identifiers_with_underscores() {
    let mut scanner = Scanner::new("_private __internal var1 var_name");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::Identifier, "_private", 1, Literal::None),
            Token::new(TokenType::Identifier, "__internal", 1, Literal::None),
            Token::new(TokenType::Identifier, "var1", 1, Literal::None),
            Token::new(TokenType::Identifier, "var_name", 1, Literal::None),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}

#[test]
fn identifiers_with_numbers() {
    let mut scanner = Scanner::new("var1 var2abc abc3def var_4");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::Identifier, "var1", 1, Literal::None),
            Token::new(TokenType::Identifier, "var2abc", 1, Literal::None),
            Token::new(TokenType::Identifier, "abc3def", 1, Literal::None),
            Token::new(TokenType::Identifier, "var_4", 1, Literal::None),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}

#[test]
fn identifier_vs_keyword() {
    let mut scanner = Scanner::new("class classroom class1 _class");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::Class, "class", 1, Literal::None),
            Token::new(TokenType::Identifier, "classroom", 1, Literal::None),
            Token::new(TokenType::Identifier, "class1", 1, Literal::None),
            Token::new(TokenType::Identifier, "_class", 1, Literal::None),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}
