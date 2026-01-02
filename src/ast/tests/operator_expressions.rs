use crate::ast::{Expr, Operator, format_expr};

#[test]
fn test_operator_expressions() {
    let unary_expr = Expr::unary(Operator::Not, Expr::boolean(true));
    assert_eq!(format_expr(&unary_expr), "(! true)");

    let binary_expr = Expr::binary(Operator::Add, Expr::number("5"), Expr::number("7"));
    assert_eq!(format_expr(&binary_expr), "(5 + 7)");

    // 分组表达式
    let grouping_expr = Expr::grouping(Expr::number("42"));
    assert_eq!(format_expr(&grouping_expr), "(group 42)");

    // 嵌套操作
    let nested_expr = Expr::binary(
        Operator::Mul,
        Expr::unary(Operator::Sub, Expr::number("123")),
        Expr::grouping(Expr::number("234")),
    );
    assert_eq!(format_expr(&nested_expr), "((- 123) * (group 234))");
}