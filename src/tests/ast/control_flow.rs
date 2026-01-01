use crate::ast::{Expr, Operator, format_expr};

#[test]
fn test_control_flow() {
    let if_expr = Expr::if_(
        Expr::binary(Operator::Greater, Expr::number("10"), Expr::number("5")),
        Expr::block(vec![Expr::number("1")]),
        Some(Expr::block(vec![Expr::number("0")])),
    );
    assert_eq!(format_expr(&if_expr), "if (10 > 5) {1} else {0}");

    let if_no_else_expr = Expr::if_(
        Expr::boolean(true),
        Expr::block(vec![Expr::string("yes")]),
        None,
    );
    assert_eq!(format_expr(&if_no_else_expr), "if true {yes}");

    let while_expr = Expr::while_(
        Expr::binary(
            Operator::Less,
            Expr::variable("i".to_string()),
            Expr::number("10"),
        ),
        Expr::block(vec![Expr::assign_op(
            Operator::AddAssign,
            "i".to_string(),
            Expr::number("1"),
        )]),
    );
    assert_eq!(format_expr(&while_expr), "while (i < 10) {i += 1}");

    // 代码块
    let block_expr = Expr::block(vec![
        Expr::assign("x".to_string(), Expr::number("1")),
        Expr::assign("y".to_string(), Expr::number("2")),
        Expr::binary(
            Operator::Add,
            Expr::variable("x".to_string()),
            Expr::variable("y".to_string()),
        ),
    ]);
    assert_eq!(format_expr(&block_expr), "{x = 1;y = 2;(x + y)}");
}
