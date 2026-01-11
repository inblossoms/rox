use crate::evaluate::{Value, tests::eval_res};
use pretty_assertions::assert_eq;

#[test]
fn test_function_call() {
    let code = r#"
        fun add(a, b) {
            return a + b;
        }
        var res = add(10, 20);
    "#;
    assert_eq!(eval_res(code).unwrap(), Value::Number(30.0));
}

#[test]
fn test_recursive_function() {
    let code = r#"
        fun fib(n) {
            if (n <= 1) return n;
            return fib(n - 2) + fib(n - 1);
        }
        var res = fib(10);
    "#;
    assert_eq!(eval_res(code).unwrap(), Value::Number(55.0));
}

#[test]
fn test_closure() {
    // 著名的闭包计数器测试
    let code = r#"
        fun makeCounter() {
            var i = 0;
            fun count() {
                i = i + 1;
                return i;
            }
            return count;
        }
        var counter = makeCounter();
        counter(); // 1
        var res = counter(); // 2
    "#;
    assert_eq!(eval_res(code).unwrap(), Value::Number(2.0));
}

#[test]
fn test_resolver_shadowing() {
    // 测试 Resolver 是否正确计算了距离
    let code = r#"
        var a = "global";
        var res = "";
        {
            fun showA() {
                return a;
            }
            var a = "block";
            res = showA(); // 应该返回 "global"，如果返回 "block" 说明 Resolver 没工作
        }
    "#;
    assert_eq!(eval_res(code).unwrap(), Value::String("global".to_string()));
}
