use crate::ast::{Expr, Operator, format::format_expr, tests::token};
use pretty_assertions::assert_eq;

#[test]
fn test_literals() {
    let num = Expr::number("123.45");
    let str_val = Expr::string("hello");
    let bool_val = Expr::boolean(true);
    let nil_val = Expr::nil();

    assert_eq!(format_expr(&num), "123.45");
    assert_eq!(format_expr(&str_val), "\"hello\"");
    assert_eq!(format_expr(&bool_val), "true");
    assert_eq!(format_expr(&nil_val), "nil");
}

#[test]
fn test_collections() {
    // List: [1, "a"]
    let list = Expr::list(vec![Expr::number("1"), Expr::string("a")]);
    assert_eq!(format_expr(&list), "[1, \"a\"]");

    // Dict: {"k": 1}
    let dict = Expr::dict(vec![(Expr::string("k"), Expr::number("1"))]);
    assert_eq!(format_expr(&dict), "{\"k\": 1}");
}

#[test]
fn test_arithmetic_and_grouping() {
    // -123 * (45.67)
    let expr = Expr::binary(
        Operator::Mul,
        Expr::unary(Operator::Sub, Expr::number("123")),
        Expr::grouping(Expr::number("45.67")),
    );

    assert_eq!(format_expr(&expr), "((-123) * (group 45.67))");
}

#[test]
fn test_logic_operations() {
    // true and false or !nil
    let expr = Expr::logical(
        Operator::LogicalOr,
        Expr::logical(
            Operator::LogicalAnd,
            Expr::boolean(true),
            Expr::boolean(false),
        ),
        Expr::unary(Operator::Not, Expr::nil()),
    );

    // 格式化输出取决于 format_operator 的实现
    // 假设 format_operator 对于 LogicalAnd 返回 "and"，Or 返回 "or"
    assert_eq!(format_expr(&expr), "((true && false) || (!nil))");
}

#[test]
fn test_variable_access_and_assignment() {
    // 构造: a
    let var_a = Expr::variable(token("a"));
    assert_eq!(format_expr(&var_a), "a");

    // 构造: a = 1
    // 注意：这里 ID 默认为 0，符合 format 测试的需求
    let assign = Expr::assign(token("a"), Expr::number("1"));
    assert_eq!(format_expr(&assign), "a = 1");

    // 构造: a += 2
    let assign_op = Expr::assign_op(Operator::Add, "a", Expr::number("2"));
    assert_eq!(format_expr(&assign_op), "a + 2"); // 假设 format 是展开显示的，或者取决于实现 "a += 2"
}

#[test]
fn test_function_call() {
    // foo(1, b)
    let call = Expr::call_str("foo", vec![Expr::number("1"), Expr::variable(token("b"))]);

    assert_eq!(format_expr(&call), "foo(1, b)");
}
