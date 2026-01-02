use crate::ast::{Expr, Operator, format_expr};

#[test]
fn test_edge_cases() {
    assert_eq!(format_expr(&Expr::block(vec![])), "{}");

    // 深度嵌套表达式
    let nested_expr = Expr::binary(
        Operator::Mul,
        Expr::binary(Operator::Add, Expr::number("1"), Expr::number("2")),
        Expr::binary(Operator::Sub, Expr::number("3"), Expr::number("4")),
    );
    assert_eq!(format_expr(&nested_expr), "((1 + 2) * (3 - 4))");

    // 包含特殊字符的字符串
    assert_eq!(format_expr(&Expr::string("hello\"world")), "hello\"world");

    let long_list = Expr::list((0..10).map(|i| Expr::number(&i.to_string())).collect());
    assert_eq!(format_expr(&long_list), "(0123456789)");
}
