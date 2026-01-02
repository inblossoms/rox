use crate::tokenizer::{Literal, Token, TokenType, scanner::Scanner};

#[test]
fn assignment_operators() {
    let mut scanner = Scanner::new("+= -= *= /= =");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::PlusEqual, "+=", 1, Literal::None),
            Token::new(TokenType::MinusEqual, "-=", 1, Literal::None),
            Token::new(TokenType::StarEqual, "*=", 1, Literal::None),
            Token::new(TokenType::SlashEqual, "/=", 1, Literal::None),
            Token::new(TokenType::Equal, "=", 1, Literal::None),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}
