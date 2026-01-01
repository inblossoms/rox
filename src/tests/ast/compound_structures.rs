use crate::ast::{Expr, format_expr};

#[test]
fn test_compound_structures() {
    assert_eq!(format_expr(&Expr::list(vec![])), "()");

    let list_expr = Expr::list(vec![
        Expr::number("1"),
        Expr::string("two"),
        Expr::boolean(true),
    ]);
    assert_eq!(format_expr(&list_expr), "(1twotrue)");

    assert_eq!(format_expr(&Expr::dict(vec![])), "()");

    let dict_expr = Expr::dict(vec![
        (Expr::string("key1"), Expr::number("100")),
        (Expr::string("key2"), Expr::boolean(false)),
    ]);
    assert_eq!(format_expr(&dict_expr), "(key1:100key2:false)");

    let tuple_expr = Expr::tuple(vec![Expr::number("10"), Expr::string("tuple")]);
    assert_eq!(format_expr(&tuple_expr), "(10tuple)");
}
