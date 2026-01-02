use crate::tokenizer::{Literal, Token, TokenType, scanner::Scanner};

#[test]
fn unterminated_string() {
    let mut scanner = Scanner::new(r#""This string doesn't end"#);
    let result = scanner.scan_tokens();
    assert!(result.is_err());
}

#[test]
fn empty_string() {
    let mut scanner = Scanner::new(r#""""#);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(
                TokenType::String,
                r#""""#,
                1,
                Literal::String("".to_string())
            ),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}

#[test]
fn string_with_spaces() {
    let mut scanner = Scanner::new(r#""hello world""#);
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(
                TokenType::String,
                r#""hello world""#,
                1,
                Literal::String("hello world".to_string())
            ),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}

#[test]
fn string_with_newlines() {
    let mut scanner = Scanner::new("\"line1\nline2\"");
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(
                TokenType::String,
                "\"line1\nline2\"",
                2,
                Literal::String("line1\nline2".to_string())
            ),
            Token::new(TokenType::Eof, "", 2, Literal::None),
        ]
    )
}
