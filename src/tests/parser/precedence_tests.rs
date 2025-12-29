use super::*;
use crate::ast::Expr;
use crate::ast::Operator;
use crate::{parser::parse, tokenizer::TokenType};

#[test]
fn test_precedence_unary_vs_binary() {
    // 目标：验证 -a * b 解析为 (-a) * b，而不是 -(a * b)
    // 代码： -1 * 2;
    let tokens = make_tokens(vec![
        tok(TokenType::Minus, "-"),
        num(1.0),
        tok(TokenType::Star, "*"),
        num(2.0),
        tok(TokenType::Semicolon, ";"),
    ]);

    let result = parse(tokens).unwrap();
    let stmt = &result.top.unwrap();

    // 解构 AST 验证结构
    if let Expr::Block { body } = stmt {
        let expr_stmt = &body[0]; // 这是一个 ExpressionStatement
        if let Expr::Binary { left, op, right } = expr_stmt {
            assert_eq!(*op, Operator::Mul, "顶层操作符应该是乘法 (*)");

            // 验证左侧是 Unary (-1)
            if let Expr::Unary {
                op: u_op,
                expr: u_expr,
            } = &**left
            {
                assert_eq!(*u_op, Operator::Sub, "左侧应该是负号 (-)");
                match &**u_expr {
                    Expr::Number { value } => assert_eq!(value, "1"),
                    _ => panic!("Expected number 1 inside unary"),
                }
            } else {
                panic!("Left side of * should be Unary expression");
            }

            // 验证右侧是 Number (2)
            match &**right {
                Expr::Number { value } => assert_eq!(value, "2"),
                _ => panic!("Right side should be number 2"),
            }
        } else {
            panic!("Expected Binary expression");
        }
    }
}

#[test]
fn test_precedence_multiplication_vs_addition() {
    // 目标：验证 1 + 2 * 3 解析为 1 + (2 * 3)
    let tokens = make_tokens(vec![
        num(1.0),
        tok(TokenType::Plus, "+"),
        num(2.0),
        tok(TokenType::Star, "*"),
        num(3.0),
        tok(TokenType::Semicolon, ";"),
    ]);

    let result = parse(tokens).unwrap();
    // 简化验证：只检查顶层是 Plus
    if let Expr::Block { body } = result.top.unwrap() {
        if let Expr::Binary { op, .. } = &body[0] {
            assert_eq!(*op, Operator::Add, "优先级错误：顶层应该是加法 (+)");
        } else {
            panic!("Expected Binary expression");
        }
    }
}

#[test]
fn test_associativity_assignment() {
    // 目标：验证赋值是右结合的 a = b = 1 解析为 a = (b = 1)
    let tokens = make_tokens(vec![
        ident("a"),
        tok(TokenType::Equal, "="),
        ident("b"),
        tok(TokenType::Equal, "="),
        num(1.0),
        tok(TokenType::Semicolon, ";"),
    ]);

    let result = parse(tokens).unwrap();
    if let Expr::Block { body } = result.top.unwrap() {
        if let Expr::Assign { name, expr } = &body[0] {
            assert_eq!(name, "a");
            // a 的值应该是另一个赋值表达式 (b = 1)
            if let Expr::Assign {
                name: name_inner, ..
            } = &**expr
            {
                assert_eq!(name_inner, "b");
            } else {
                panic!("赋值应该是右结合的");
            }
        }
    }
}
