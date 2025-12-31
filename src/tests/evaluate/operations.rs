use crate::ast::{Expr, Operator};
use crate::evaluate::{
    interpreter::{RuntimeError, evaluate_tests::create_test_interpreter},
    value::Value,
};

#[test]
fn test_binary_arithmetic_operations() {
    let mut interpreter = create_test_interpreter();

    // Test addition
    let expr = Expr::Binary {
        left: Box::new(Expr::Number {
            value: "10".to_string(),
        }),
        op: Operator::Add,
        right: Box::new(Expr::Number {
            value: "5".to_string(),
        }),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Number(15.0));

    // Test subtraction
    let expr = Expr::Binary {
        left: Box::new(Expr::Number {
            value: "10".to_string(),
        }),
        op: Operator::Sub,
        right: Box::new(Expr::Number {
            value: "5".to_string(),
        }),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Number(5.0));

    // Test multiplication
    let expr = Expr::Binary {
        left: Box::new(Expr::Number {
            value: "10".to_string(),
        }),
        op: Operator::Mul,
        right: Box::new(Expr::Number {
            value: "5".to_string(),
        }),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Number(50.0));

    // Test division
    let expr = Expr::Binary {
        left: Box::new(Expr::Number {
            value: "10".to_string(),
        }),
        op: Operator::Div,
        right: Box::new(Expr::Number {
            value: "2".to_string(),
        }),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Number(5.0));

    // Test division by zero
    let expr = Expr::Binary {
        left: Box::new(Expr::Number {
            value: "10".to_string(),
        }),
        op: Operator::Div,
        right: Box::new(Expr::Number {
            value: "0".to_string(),
        }),
    };
    let result = interpreter.evaluate(&expr);
    assert!(matches!(result, Err(RuntimeError::DivisionByZero)));
}

#[test]
fn test_binary_comparison_operations() {
    let mut interpreter = create_test_interpreter();

    // Test greater than
    let expr = Expr::Binary {
        left: Box::new(Expr::Number {
            value: "10".to_string(),
        }),
        op: Operator::Greater,
        right: Box::new(Expr::Number {
            value: "5".to_string(),
        }),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));

    // Test less than or equal
    let expr = Expr::Binary {
        left: Box::new(Expr::Number {
            value: "5".to_string(),
        }),
        op: Operator::LessEqual,
        right: Box::new(Expr::Number {
            value: "5".to_string(),
        }),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_binary_equality_operations() {
    let mut interpreter = create_test_interpreter();

    // Test equality
    let expr = Expr::Binary {
        left: Box::new(Expr::Number {
            value: "5".to_string(),
        }),
        op: Operator::Equal,
        right: Box::new(Expr::Number {
            value: "5".to_string(),
        }),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));

    // Test inequality
    let expr = Expr::Binary {
        left: Box::new(Expr::Number {
            value: "5".to_string(),
        }),
        op: Operator::NotEqual,
        right: Box::new(Expr::Number {
            value: "10".to_string(),
        }),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}

#[test]
fn test_logical_operations() {
    let mut interpreter = create_test_interpreter();

    // Test AND short-circuit
    let expr = Expr::Binary {
        left: Box::new(Expr::Boolean { value: false }),
        op: Operator::And,
        right: Box::new(Expr::Number {
            value: "100".to_string(),
        }),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Boolean(false));

    // Test OR short-circuit
    let expr = Expr::Binary {
        left: Box::new(Expr::Boolean { value: true }),
        op: Operator::Or,
        right: Box::new(Expr::Number {
            value: "100".to_string(),
        }),
    };
    let result = interpreter.evaluate(&expr).unwrap();
    assert_eq!(result, Value::Boolean(true));
}
