use crate::ast::{Expr, Operator, format_expr};

#[test]
fn test_function_expressions() {
    let func_expr = Expr::function(
        "add".to_string(),
        vec!["a".to_string(), ",".to_string(), "b".to_string()],
        vec![Expr::return_(Expr::binary(
            Operator::Add,
            Expr::variable("a".to_string()),
            Expr::variable("b".to_string()),
        ))],
    );
    assert_eq!(format_expr(&func_expr), "fn add(a,b) {return (a + b)}");

    let call_expr = Expr::call(
        "add".to_string(),
        vec![Expr::number("3"), Expr::number("4")],
    );
    assert_eq!(format_expr(&call_expr), "add(3,4)");

    let return_expr = Expr::return_(Expr::number("42"));
    assert_eq!(format_expr(&return_expr), "return 42");
}
