use crate::parser::tests::assert_parse;

#[test]
fn test_var_declaration() {
    assert_parse("var a = 1;", "var a = 1;");
    assert_parse("var b;", "var b;");
}

#[test]
fn test_assignment() {
    assert_parse("a = 1;", "a = 1;");
    // 赋值是右结合的: a = b = 2 -> a = (b = 2)
    assert_parse("a = b = 2;", "a = b = 2;");
}

#[test]
fn test_block() {
    assert_parse("{ var a = 1; print a; }", "{ var a = 1; print a; }");
}

#[test]
fn test_if_else() {
    assert_parse(
        "if (true) print 1; else print 2;",
        "if (true) print 1; else print 2;",
    );
}

#[test]
fn test_dangling_else() {
    // 悬挂 Else 问题：else 应该绑定到最近的 if
    // if (first) if (second) whenTrue; else whenFalse;
    // 应该解析为:
    // if (first) {
    //    if (second) whenTrue; else whenFalse;
    // }
    assert_parse("if (a) if (b) c; else d;", "if (a) if (b) c; else d;");
}

#[test]
fn test_while_loop() {
    assert_parse("while (a < 10) a = a + 1;", "while ((a < 10)) a = (a + 1);");
}

#[test]
fn test_for_loop_native() {
    // 测试我们是否正确生成了 Expr::For (Stmt::For)
    // 完整的 for
    assert_parse(
        "for (var i=0; i<10; i=i+1) print i;",
        "for (var i = 0; (i < 10); i = (i + 1)) print i;",
    );

    // 空子句 for
    assert_parse("for (;;) { break; }", "for (; ; ) { break; }");
}
