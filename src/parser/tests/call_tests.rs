use crate::{parser::parse, tokenizer::TokenType};

use super::*;

#[test]
fn test_call_args_expressions() {
    // 函数参数可以为复杂表达式：log(1 + 2, "res: " + str);
    let tokens = make_tokens(vec![
        ident("log"),
        tok(TokenType::LeftParen, "("),
        // Arg 1: 1 + 2
        num(1.0),
        tok(TokenType::Plus, "+"),
        num(2.0),
        tok(TokenType::Comma, ","),
        // Arg 2: "res: " + str
        string("res: "),
        tok(TokenType::Plus, "+"),
        ident("str"),
        tok(TokenType::RightParen, ")"),
        tok(TokenType::Semicolon, ";"),
    ]);

    let result = parse(tokens).unwrap();

    if let crate::ast::Expr::Block { body } = result.top.unwrap() {
        if let crate::ast::Expr::Call { args, .. } = &body[0] {
            assert_eq!(args.len(), 2, "应该有两个参数");

            // 检查第一个参数是否为二元表达式
            if !matches!(args[0], crate::ast::Expr::Binary { .. }) {
                panic!("First arg should be a binary expression");
            }
        }
    }
}

#[test]
fn test_call_precedence() {
    // 函数调用优先级应高于加法: f(1) + 2;  => (call) + 2
    let tokens = make_tokens(vec![
        ident("f"),
        tok(TokenType::LeftParen, "("),
        num(1.0),
        tok(TokenType::RightParen, ")"),
        tok(TokenType::Plus, "+"),
        num(2.0),
        tok(TokenType::Semicolon, ";"),
    ]);

    let result = parse(tokens).unwrap();

    if let crate::ast::Expr::Block { body } = result.top.unwrap() {
        if let crate::ast::Expr::Binary { left, op, right } = &body[0] {
            assert_eq!(*op, crate::ast::Operator::Add);

            // 左侧必须是 Call
            if !matches!(**left, crate::ast::Expr::Call { .. }) {
                panic!("Left side of + should be a function call");
            }
            // 右侧是 Number
            match &**right {
                crate::ast::Expr::Number { value } => assert_eq!(value, "2"),
                _ => panic!("Right side should be number"),
            }
        }
    }
}

#[test]
fn test_function_call_nested() {
    // print(add(1, 2));
    let tokens = make_tokens(vec![
        ident("print"),
        tok(TokenType::LeftParen, "("),
        ident("add"),
        tok(TokenType::LeftParen, "("),
        num(1.0),
        tok(TokenType::Comma, ","),
        num(2.0),
        tok(TokenType::RightParen, ")"),
        tok(TokenType::RightParen, ")"),
        tok(TokenType::Semicolon, ";"),
    ]);

    let result = parse(tokens);
    assert!(result.is_ok(), "嵌套函数调用解析失败");
}

#[test]
fn test_error_call_missing_right_paren() {
    // print("hello" ; (缺少右括号)
    let tokens = make_tokens(vec![
        ident("print"),
        tok(TokenType::LeftParen, "("),
        string("hello"),
        tok(TokenType::Semicolon, ";"),
    ]);

    let result = parse(tokens);
    assert!(result.is_err());
    assert!(result.err().unwrap().message.contains("Expected ')'"));
}
