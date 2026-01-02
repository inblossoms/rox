use crate::ast::{Expr, format_expr};

#[test]
fn test_basic_expressions() {
    assert_eq!(format_expr(&Expr::number("42")), "42");

    assert_eq!(format_expr(&Expr::string("hello")), "hello");

    assert_eq!(format_expr(&Expr::boolean(true)), "true");
    assert_eq!(format_expr(&Expr::boolean(false)), "false");

    assert_eq!(format_expr(&Expr::nil()), "nil");
}
