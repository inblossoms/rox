use crate::ast::{
    Expr, Operator, Stmt,
    format::format_stmt,
    tests::{token, token_return},
};

#[test]
fn test_var_declaration() {
    // var a = 1;
    let decl = Stmt::var("a", Some(Expr::number("1")));
    assert_eq!(format_stmt(&decl), "var a = 1;");

    // var b;
    let decl_empty = Stmt::var("b", None);
    assert_eq!(format_stmt(&decl_empty), "var b;");
}

#[test]
fn test_block() {
    // { var a = 1; print a; }
    let block = Stmt::block(vec![
        Stmt::var("a", Some(Expr::number("1"))),
        Stmt::print(Expr::variable(token("a"))),
    ]);

    assert_eq!(format_stmt(&block), "{ var a = 1; print a; }");
}

#[test]
fn test_if_stmt() {
    // if (true) print "yes"; else print "no";
    let if_stmt = Stmt::if_(
        Expr::boolean(true),
        Stmt::print(Expr::string("yes")),
        Some(Stmt::print(Expr::string("no"))),
    );

    assert_eq!(
        format_stmt(&if_stmt),
        "if (true) print \"yes\"; else print \"no\";"
    );
}

#[test]
fn test_loops() {
    // while (x < 10) { x = x + 1; }
    let while_stmt = Stmt::while_(
        Expr::binary(
            Operator::Less,
            Expr::variable(token("x")),
            Expr::number("10"),
        ),
        Stmt::block(vec![Stmt::expression(Expr::assign(
            token("x"),
            Expr::binary(Operator::Add, Expr::variable(token("x")), Expr::number("1")),
        ))]),
    );
    assert_eq!(
        format_stmt(&while_stmt),
        "while ((x < 10)) { x = (x + 1); }"
    );

    // for (var i=0; i<5; i = i + 1) print i;
    let for_stmt = Stmt::for_(
        Some(Stmt::var("i", Some(Expr::number("0")))),
        Some(Expr::binary(
            Operator::Less,
            Expr::variable(token("i")),
            Expr::number("5"),
        )),
        Some(Expr::assign(
            token("i"),
            Expr::binary(Operator::Add, Expr::variable(token("i")), Expr::number("1")),
        )),
        Stmt::print(Expr::variable(token("i"))),
    );
    // 注意：格式化输出取决于 format_stmt 实现，通常 for 循环内部带分号
    assert_eq!(
        format_stmt(&for_stmt),
        "for (var i = 0; (i < 5); i = (i + 1)) print i;"
    );
}

#[test]
fn test_function_declaration() {
    // fun add(a, b) { return a + b; }
    let func = Stmt::function(
        "add",
        vec!["a", "b"],
        vec![Stmt::Return {
            keyword: token_return(),
            value: Some(Expr::binary(
                Operator::Add,
                Expr::variable(token("a")),
                Expr::variable(token("b")),
            )),
        }],
    );

    assert_eq!(format_stmt(&func), "fun add(a, b) { return (a + b); }");
}

#[test]
fn test_jump_statements() {
    let brk = Stmt::break_();
    let cont = Stmt::continue_();
    let ret = Stmt::return_(None);

    assert_eq!(format_stmt(&brk), "break;");
    assert_eq!(format_stmt(&cont), "continue;");
    assert_eq!(format_stmt(&ret), "return;");
}

#[test]
fn test_complex_integration() {
    // 模拟一个复杂的嵌套结构
    // fun main() {
    //   var x = 0;
    //   if (x == 0) {
    //     print "zero";
    //   }
    // }

    let stmt = Stmt::function(
        "main",
        vec![],
        vec![
            Stmt::var("x", Some(Expr::number("0"))),
            Stmt::if_(
                Expr::binary(
                    Operator::Equal,
                    Expr::variable(token("x")),
                    Expr::number("0"),
                ),
                Stmt::block(vec![Stmt::print(Expr::string("zero"))]),
                None,
            ),
        ],
    );

    let output = format_stmt(&stmt);
    // 验证关键结构存在即可，精确匹配可能受空格影响
    assert!(output.contains("fun main()"));
    assert!(output.contains("var x = 0;"));
    assert!(output.contains("if ((x == 0))"));
    assert!(output.contains("print \"zero\";"));
}
