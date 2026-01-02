use crate::ast::Expr;
use crate::evaluate::{
    interpreter::{RuntimeError, evaluate_tests::create_test_interpreter},
    value::Value,
};

#[test]
fn test_variable_definition_and_access() {
    let mut interpreter = create_test_interpreter();

    // Define a variable
    let assign_expr = Expr::VarDecl {
        name: "x".to_string(),
        initializer: Box::new(Expr::Number {
            value: "42".to_string(),
        }),
    };
    interpreter.evaluate(&assign_expr).unwrap();

    // Access the variable
    let var_expr = Expr::Variable {
        name: "x".to_string(),
    };
    let result = interpreter.evaluate(&var_expr).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_variable_assignment() {
    let mut interpreter = create_test_interpreter();

    // Define and assign
    let assign_expr = Expr::VarDecl {
        name: "x".to_string(),
        initializer: Box::new(Expr::Number {
            value: "42".to_string(),
        }),
    };
    interpreter.evaluate(&assign_expr).unwrap();

    // Reassign
    let reassign_expr = Expr::VarDecl {
        name: "x".to_string(),
        initializer: Box::new(Expr::Number {
            value: "100".to_string(),
        }),
    };
    interpreter.evaluate(&reassign_expr).unwrap();

    // Check new value
    let var_expr = Expr::Variable {
        name: "x".to_string(),
    };
    let result = interpreter.evaluate(&var_expr).unwrap();
    assert_eq!(result, Value::Number(100.0));
}

#[test]
fn test_undefined_variable() {
    let mut interpreter = create_test_interpreter();

    let var_expr = Expr::Variable {
        name: "undefined_var".to_string(),
    };
    let result = interpreter.evaluate(&var_expr);
    assert!(matches!(result, Err(RuntimeError::UndefinedVariable(_))));
}

#[test]
fn test_block_scoping() {
    let mut interpreter = create_test_interpreter();

    // Define variable in outer scope
    let outer_assign = Expr::VarDecl {
        name: "x".to_string(),
        initializer: Box::new(Expr::Number {
            value: "10".to_string(),
        }),
    };
    interpreter.evaluate(&outer_assign).unwrap();

    // Create a block with inner scope
    let inner_assign = Expr::VarDecl {
        name: "x".to_string(), // Same name as outer scope
        initializer: Box::new(Expr::Number {
            value: "20".to_string(),
        }),
    };
    let block = Expr::Block {
        body: vec![inner_assign],
    };

    interpreter.evaluate(&block).unwrap();

    // Check that outer scope variable is unchanged
    let var_expr = Expr::Variable {
        name: "x".to_string(),
    };
    let result = interpreter.evaluate(&var_expr).unwrap();
    assert_eq!(result, Value::Number(10.0)); // Should still be 10, not 20
}
