use crate::ast::{Expr, Operator};
use crate::evaluate::{interpreter::evaluate_tests::create_test_interpreter, value::Value};

#[test]
fn test_if_statement() {
    let mut interpreter = create_test_interpreter();

    // Test true branch
    let if_expr = Expr::If {
        condition: Box::new(Expr::Boolean { value: true }),
        then_branch: Box::new(Expr::Number {
            value: "42".to_string(),
        }),
        else_branch: Some(Box::new(Expr::Number {
            value: "0".to_string(),
        })),
    };
    let result = interpreter.evaluate(&if_expr).unwrap();
    assert_eq!(result, Value::Number(42.0));

    // Test false branch
    let if_expr = Expr::If {
        condition: Box::new(Expr::Boolean { value: false }),
        then_branch: Box::new(Expr::Number {
            value: "42".to_string(),
        }),
        else_branch: Some(Box::new(Expr::Number {
            value: "0".to_string(),
        })),
    };
    let result = interpreter.evaluate(&if_expr).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_while_loop() {
    let mut interpreter = create_test_interpreter();

    // Simple counter test: while x < 5, x = x + 1
    // First set x = 0
    let init = Expr::VarDecl {
        name: "x".to_string(),
        initializer: Box::new(Expr::Number {
            value: "0".to_string(),
        }),
    };
    interpreter.evaluate(&init).unwrap();

    // Create the loop: while x < 5 { x = x + 1 }
    let loop_expr = Expr::While {
        condition: Box::new(Expr::Binary {
            left: Box::new(Expr::Variable {
                name: "x".to_string(),
            }),
            op: Operator::Less,
            right: Box::new(Expr::Number {
                value: "5".to_string(),
            }),
        }),
        body: Box::new(Expr::Assign {
            name: "x".to_string(),
            expr: Box::new(Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: "x".to_string(),
                }),
                op: Operator::Add,
                right: Box::new(Expr::Number {
                    value: "1".to_string(),
                }),
            }),
        }),
    };

    interpreter.evaluate(&loop_expr).unwrap();

    // Check that x is now 5
    let var_expr = Expr::Variable {
        name: "x".to_string(),
    };
    let result = interpreter.evaluate(&var_expr).unwrap();
    assert_eq!(result, Value::Number(5.0));
}
