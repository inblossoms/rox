use super::*;
use crate::{parser::parse, tokenizer::TokenType};

#[test]
fn test_function_decl_no_params_empty_body() {
    // fun noop() {}
    let tokens = make_tokens(vec![
        tok(TokenType::Fun, "fun"),
        ident("noop"),
        tok(TokenType::LeftParen, "("),
        tok(TokenType::RightParen, ")"),
        tok(TokenType::LeftBrace, "{"),
        tok(TokenType::RightBrace, "}"),
    ]);

    let result = parse(tokens).unwrap();

    if let crate::ast::Expr::Block { body } = result.top.unwrap() {
        if let crate::ast::Expr::Function {
            name,
            args,
            body: func_body,
        } = &body[0]
        {
            assert_eq!(name, "noop");
            assert!(args.is_empty(), "参数列表应为空");
            assert!(func_body.is_empty(), "函数体应为空");
        } else {
            panic!("Expected Function declaration");
        }
    }
}

#[test]
fn test_function_decl_multiple_params() {
    // fun add(a, b, c) { var sum = a + b + c; }
    let tokens = make_tokens(vec![
        tok(TokenType::Fun, "fun"),
        ident("add"),
        tok(TokenType::LeftParen, "("),
        ident("a"),
        tok(TokenType::Comma, ","),
        ident("b"),
        tok(TokenType::Comma, ","),
        ident("c"),
        tok(TokenType::RightParen, ")"),
        tok(TokenType::LeftBrace, "{"),
        // Body content
        tok(TokenType::Var, "var"),
        ident("sum"),
        tok(TokenType::Equal, "="),
        ident("a"),
        tok(TokenType::Plus, "+"),
        ident("b"),
        tok(TokenType::Plus, "+"),
        ident("c"),
        tok(TokenType::Semicolon, ";"),
        tok(TokenType::RightBrace, "}"),
    ]);

    let result = parse(tokens).unwrap();

    if let crate::ast::Expr::Block { body } = result.top.unwrap() {
        if let crate::ast::Expr::Function { args, .. } = &body[0] {
            assert_eq!(args.len(), 3);
            assert_eq!(args[0], "a");
            assert_eq!(args[1], "b");
            assert_eq!(args[2], "c");
        }
    }
}

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
fn test_error_function_missing_comma() {
    // err ：fun foo(a b) {} (缺少逗号)
    let tokens = make_tokens(vec![
        tok(TokenType::Fun, "fun"),
        ident("foo"),
        tok(TokenType::LeftParen, "("),
        ident("a"),
        // Missing comma
        ident("b"),
        tok(TokenType::RightParen, ")"),
        tok(TokenType::LeftBrace, "{"),
        tok(TokenType::RightBrace, "}"),
    ]);

    let result = parse(tokens);
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap().message,
        "[line 1]: Expect ')' after parameters."
    );
    // 解释：因为没有逗号，解析器认为 'a' 是唯一的参数，期望看到 ')', 但看到了 'b'
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
