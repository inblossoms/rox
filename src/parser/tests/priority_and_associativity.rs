use crate::parser::tests::assert_parse;

#[test]
fn test_precedence_add_mul() {
    // 乘法优先级高于加法: 1 + 2 * 3 -> 1 + (2 * 3)
    assert_parse("1 + 2 * 3;", "(1 + (2 * 3));");

    // 从左向右结合: 1 * 2 + 3 -> (1 * 2) + 3
    assert_parse("1 * 2 + 3;", "((1 * 2) + 3);");
}

#[test]
fn test_precedence_comparison_equality() {
    // 比较运算优先级高于相等运算: 1 < 2 == 3 > 4
    // 解析为: ((1 < 2) == (3 > 4))
    assert_parse("1 < 2 == 3 > 4;", "((1 < 2) == (3 > 4));");
}

#[test]
fn test_precedence_logic() {
    // AND 优先级高于 OR
    // true or false and nil -> true or (false and nil)
    assert_parse("true || false && nil;", "(true || (false && nil));");
}

#[test]
fn test_grouping() {
    // 括号改变优先级
    assert_parse("(1 + 2) * 3;", "((group (1 + 2)) * 3);");
}

#[test]
fn test_unary() {
    // 一元运算结合性
    assert_parse("- -1;", "(-(-1));");
    assert_parse("!true;", "(!true);");
}
