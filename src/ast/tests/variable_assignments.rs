use crate::ast::{Expr, Operator, format_expr};

#[test]
fn test_variable_assignments() {
    assert_eq!(format_expr(&Expr::variable("x".to_string())), "x");

    let assign_expr = Expr::assign("x".to_string(), Expr::number("10"));
    assert_eq!(format_expr(&assign_expr), "x = 10;");

    let assign_op_expr = Expr::assign_op(Operator::AddAssign, "y".to_string(), Expr::number("5"));
    assert_eq!(format_expr(&assign_op_expr), "y += 5");
}
