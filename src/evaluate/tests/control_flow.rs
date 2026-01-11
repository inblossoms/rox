use crate::evaluate::{Value, tests::eval_res};
use pretty_assertions::assert_eq;

#[test]
fn test_if_statement() {
    let code = r#"
        var res = 0;
        if (true) {
            res = 10;
        } else {
            res = 20;
        }
    "#;
    assert_eq!(eval_res(code).unwrap(), Value::Number(10.0));
}

#[test]
fn test_while_loop() {
    let code = r#"
        var i = 0;
        var res = 0;
        while (i < 5) {
            res = res + i;
            i = i + 1;
        }
    "#;
    // 0+1+2+3+4 = 10
    assert_eq!(eval_res(code).unwrap(), Value::Number(10.0));
}

#[test]
fn test_for_loop_with_continue() {
    let code = r#"
        var res = 0;
        for (var i = 0; i < 5; i = i + 1) {
            if (i == 2) continue;
            res = res + 1;
        }
    "#;
    // i=0,1,3,4 (skip 2) -> total 4 increments
    assert_eq!(eval_res(code).unwrap(), Value::Number(4.0));
}

#[test]
fn test_loop_break() {
    let code = r#"
        var res = 0;
        while (true) {
            res = res + 1;
            if (res == 3) break;
        }
    "#;
    assert_eq!(eval_res(code).unwrap(), Value::Number(3.0));
}
