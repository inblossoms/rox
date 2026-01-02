use crate::tokenizer::{Literal, Token, TokenType, scanner::Scanner};

#[test]
fn literals() {
    let mut scanner = Scanner::new("\"abc\"123.456");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(
                TokenType::String,
                "\"abc\"",
                1,
                Literal::String("abc".to_string())
            ),
            Token::new(TokenType::Number, "123.456", 1, Literal::Number(123.456)),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}
