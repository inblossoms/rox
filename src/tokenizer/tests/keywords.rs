use crate::tokenizer::{Literal, Token, TokenType, scanner::Scanner};

#[test]
fn keywords() {
    let mut scanner = Scanner::new(
        "ray and class else false for fun if nil or print return super this true var while",
    );
    let tokens = scanner.scan_tokens();
    assert_eq!(
        tokens.unwrap().tokens,
        vec![
            Token::new(TokenType::Identifier, "ray", 1, Literal::None),
            Token::new(TokenType::And, "and", 1, Literal::None),
            Token::new(TokenType::Class, "class", 1, Literal::None),
            Token::new(TokenType::Else, "else", 1, Literal::None),
            Token::new(TokenType::False, "false", 1, Literal::None),
            Token::new(TokenType::For, "for", 1, Literal::None),
            Token::new(TokenType::Fun, "fun", 1, Literal::None),
            Token::new(TokenType::If, "if", 1, Literal::None),
            Token::new(TokenType::Nil, "nil", 1, Literal::None),
            Token::new(TokenType::Or, "or", 1, Literal::None),
            Token::new(TokenType::Print, "print", 1, Literal::None),
            Token::new(TokenType::Return, "return", 1, Literal::None),
            Token::new(TokenType::Super, "super", 1, Literal::None),
            Token::new(TokenType::This, "this", 1, Literal::None),
            Token::new(TokenType::True, "true", 1, Literal::None),
            Token::new(TokenType::Var, "var", 1, Literal::None),
            Token::new(TokenType::While, "while", 1, Literal::None),
            Token::new(TokenType::Eof, "", 1, Literal::None),
        ]
    )
}
