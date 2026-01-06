use crate::evaluate::{Value, tests::eval_res};

#[test]
fn test_arithmetic() {
    let code = "var res = (1 + 2) * 3 - 4 / 2;";
    assert_eq!(eval_res(code).unwrap(), Value::Number(7.0));
}

#[test]
fn test_string_concatenation() {
    let code = "var res = \"hello\" + \" \" + \"world\";";
    assert_eq!(
        eval_res(code).unwrap(),
        Value::String("hello world".to_string())
    );
}

#[test]
fn test_comparison() {
    assert_eq!(eval_res("var res = 1 < 2;").unwrap(), Value::Boolean(true));
    assert_eq!(eval_res("var res = 1 >= 1;").unwrap(), Value::Boolean(true));
    assert_eq!(
        eval_res("var res = 1 == 2;").unwrap(),
        Value::Boolean(false)
    );
    assert_eq!(
        eval_res("var res = \"a\" != \"b\";").unwrap(),
        Value::Boolean(true)
    );
}

#[test]
fn test_logic() {
    // And
    assert_eq!(
        eval_res("var res = true and false;").unwrap(),
        Value::Boolean(false)
    );
    assert_eq!(
        eval_res("var res = true and true;").unwrap(),
        Value::Boolean(true)
    );
    // Or
    assert_eq!(
        eval_res("var res = false or true;").unwrap(),
        Value::Boolean(true)
    );
    assert_eq!(
        eval_res("var res = false or false;").unwrap(),
        Value::Boolean(false)
    );
    assert_eq!(eval_res("var res = 0 or 2;").unwrap(), Value::Number(2.0));
}
