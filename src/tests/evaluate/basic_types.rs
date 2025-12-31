use crate::ast::{Expr, Operator};
use crate::evaluate::{interpreter::Interpreter, value::Value};

pub fn create_test_interpreter() -> Interpreter {
    Interpreter::new()
}

#[test]
fn test_number_evaluation() {
    let mut interpreter = create_test_interpreter();
    let expr = Expr::Number {
        value: "42".to_string(),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_string_evaluation() {
    let mut interpreter = create_test_interpreter();
    let expr = Expr::String {
        value: "hello".to_string(),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::String("hello".to_string()));
}

#[test]
fn test_boolean_evaluation() {
    let mut interpreter = create_test_interpreter();
    let expr = Expr::Boolean { value: true };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_nil_evaluation() {
    let mut interpreter = create_test_interpreter();
    let expr = Expr::Nil;
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Nil);
}

#[test]
fn test_unary_operations() {
    let mut interpreter = create_test_interpreter();

    // Test unary minus
    let expr = Expr::Unary {
        op: Operator::Sub,
        expr: Box::new(Expr::Number {
            value: "42".to_string(),
        }),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Number(-42.0));

    // Test logical NOT
    let expr = Expr::Unary {
        op: Operator::Not,
        expr: Box::new(Expr::Boolean { value: true }),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Boolean(false));
}
