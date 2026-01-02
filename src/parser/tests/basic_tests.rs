use crate::parser::parse;
use crate::tokenizer::{Literal, Token, TokenType, Tokens};

pub fn tok(tt: TokenType, lex: &str) -> Token {
    Token {
        token_type: tt,
        lexeme: lex.to_string(),
        literal: Literal::Nil,
        line: 1,
    }
}

pub fn num(n: f64) -> Token {
    Token {
        token_type: TokenType::Number,
        lexeme: n.to_string(),
        literal: Literal::Number(n),
        line: 1,
    }
}

pub fn string(s: &str) -> Token {
    Token {
        token_type: TokenType::String,
        lexeme: format!("\"{}\"", s),
        literal: Literal::String(s.to_string()),
        line: 1,
    }
}

pub fn ident(name: &str) -> Token {
    Token {
        token_type: TokenType::Identifier,
        lexeme: name.to_string(),
        literal: Literal::Nil,
        line: 1,
    }
}

pub fn eof() -> Token {
    Token {
        token_type: TokenType::Eof,
        lexeme: "".to_string(),
        literal: Literal::Nil,
        line: 1,
    }
}

pub fn make_tokens(mut tokens: Vec<Token>) -> Tokens {
    tokens.push(eof()); // 自动追加 EOF
    Tokens { tokens }
}

#[test]
pub fn test_string_literals_and_concatenation() {
    // 目标：验证字符串字面量解析及二元运算
    // 代码：var s = "Hello" + " " + "World";
    let tokens = make_tokens(vec![
        tok(TokenType::Var, "var"),
        ident("s"),
        tok(TokenType::Equal, "="),
        string("Hello"),
        tok(TokenType::Plus, "+"),
        string(" "),
        tok(TokenType::Plus, "+"),
        string("World"),
        tok(TokenType::Semicolon, ";"),
    ]);

    let result = parse(tokens);
    assert!(result.is_ok(), "字符串拼接解析失败");

    // 验证 AST 结构
    if let crate::ast::Expr::Block { body } = result.unwrap().top.unwrap() {
        if let crate::ast::Expr::Assign { expr, .. } = &body[0] {
            // 顶层应该是 + 操作
            if let crate::ast::Expr::Binary { op, right, .. } = &**expr {
                assert_eq!(*op, crate::ast::Operator::Add);
                // 右侧是 "World"
                match &**right {
                    crate::ast::Expr::String { value } => assert_eq!(value, "World"),
                    _ => panic!("Expected string literal 'World'"),
                }
            } else {
                panic!("Expected Binary expression for string concat");
            }
        }
    }
}

#[test]
fn test_empty_string() {
    // print("");
    let tokens = make_tokens(vec![
        ident("print"),
        tok(TokenType::LeftParen, "("),
        string(""),
        tok(TokenType::RightParen, ")"),
        tok(TokenType::Semicolon, ";"),
    ]);

    let result = parse(tokens).unwrap();

    // ""
    if let crate::ast::Expr::Block { body } = result.top.unwrap() {
        if let crate::ast::Expr::Call { args, .. } = &body[0] {
            assert_eq!(args.len(), 1);
            match &args[0] {
                crate::ast::Expr::String { value } => assert_eq!(value, ""),
                _ => panic!("Expected empty string"),
            }
        }
    }
}

#[test]
fn test_var_declaration_complex() {
    // var x = (1 + 2);
    let tokens = make_tokens(vec![
        tok(TokenType::Var, "var"),
        ident("x"),
        tok(TokenType::Equal, "="),
        tok(TokenType::LeftParen, "("),
        num(1.0),
        tok(TokenType::Plus, "+"),
        num(2.0),
        tok(TokenType::RightParen, ")"),
        tok(TokenType::Semicolon, ";"),
    ]);

    let result = parse(tokens);
    assert!(result.is_ok(), "变量声明解析失败");
}
