use crate::evaluate::tests::eval_res;

#[test]
fn test_runtime_error_undefined_var() {
    let code = "var res = x;"; // x undefined
    assert!(eval_res(code).is_err());
}

#[test]
fn test_static_error_top_level_return() {
    let code = "return 1;"; // Resolver should catch this
    // 注意：如果是 Resolver 报错，eval_res 会返回 Err("Resolution Error: ...")
    assert!(eval_res(code).is_err());
}

#[test]
fn test_static_error_var_redefinition() {
    let code = "{ var a = 1; var a = 2; }";
    assert!(eval_res(code).is_err());
}
