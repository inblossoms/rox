use crate::parser::tests::assert_error;

#[test]
fn test_error_missing_semicolon() {
    assert_error("var a = 1", "Expect ';' after variable declaration");
}

#[test]
fn test_error_missing_paren() {
    assert_error("if true) print 1;", "Expect '(' after 'if'");
}

#[test]
fn test_error_invalid_assignment() {
    // 1 是右值，不能赋值给它
    assert_error("1 = a;", "Invalid assignment target");
    assert_error("a + b = c;", "Invalid assignment target");
}

#[test]
fn test_error_unexpected_token() {
    // 无法识别的语法结构
    assert_error("var 1 = a;", "Expect variable name");
}
