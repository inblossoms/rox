use crate::ast::{Expr, Operator};
use crate::evaluate::{interpreter::evaluate_tests::create_test_interpreter, value::Value};

#[test]
fn test_function_definition_and_call() {
    let mut interpreter = create_test_interpreter();

    // Define a function: fun add(x, y) { x + y }
    let func_def = Expr::Function {
        name: "add".to_string(),
        args: vec!["x".to_string(), "y".to_string()],
        body: vec![Expr::Binary {
            left: Box::new(Expr::Variable {
                name: "x".to_string(),
            }),
            op: Operator::Add,
            right: Box::new(Expr::Variable {
                name: "y".to_string(),
            }),
        }],
    };
    interpreter.evaluate(&func_def).unwrap();

    // Call the function: add(3, 5)
    let call = Expr::Call {
        name: "add".to_string(),
        args: vec![
            Expr::Number {
                value: "3".to_string(),
            },
            Expr::Number {
                value: "5".to_string(),
            },
        ],
    };
    let result = interpreter.evaluate(&call).unwrap();
    assert_eq!(result, Value::Number(8.0));
}

#[test]
fn test_function_with_closure() {
    let mut interpreter = create_test_interpreter();

    // Define outer variable
    let outer_def = Expr::VarDecl {
        name: "x".to_string(),
        initializer: Box::new(Expr::Number {
            value: "10".to_string(),
        }),
    };
    interpreter.evaluate(&outer_def).unwrap();

    // Define function that captures x
    let func_def = Expr::Function {
        name: "get_x".to_string(),
        args: vec![],
        body: vec![Expr::Variable {
            name: "x".to_string(),
        }],
    };
    interpreter.evaluate(&func_def).unwrap();

    // Call the function
    let call = Expr::Call {
        name: "get_x".to_string(),
        args: vec![],
    };
    let result = interpreter.evaluate(&call).unwrap();
    assert_eq!(result, Value::Number(10.0));
}

#[test]
fn test_return_statement() {
    let mut interpreter = create_test_interpreter();

    // Function with return: fn test() { return 42; 100 }
    let func_def = Expr::Function {
        name: "test".to_string(),
        args: vec![],
        body: vec![
            Expr::Return {
                expr: Box::new(Expr::Number {
                    value: "42".to_string(),
                }),
            },
            Expr::Number {
                value: "100".to_string(),
            }, // This should not be executed
        ],
    };
    interpreter.evaluate(&func_def).unwrap();

    // Call the function
    let call = Expr::Call {
        name: "test".to_string(),
        args: vec![],
    };
    let result = interpreter.evaluate(&call).unwrap();
    assert_eq!(result, Value::Number(42.0));
}
