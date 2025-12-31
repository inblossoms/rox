use crate::ast::{Expr, Operator};
use crate::evaluate::{interpreter::evaluate_tests::create_test_interpreter, value::Value};

#[test]
fn test_compound_assignment() {
    let mut interpreter = create_test_interpreter();

    // Set x = 10
    let init = Expr::VarDecl {
        name: "x".to_string(),
        initializer: Box::new(Expr::Number {
            value: "10".to_string(),
        }),
    };
    interpreter.evaluate(&init).unwrap();

    // x += 5 (should result in 15)
    let compound_assign = Expr::AssignOp {
        op: Operator::Add,
        name: "x".to_string(),
        expr: Box::new(Expr::Number {
            value: "5".to_string(),
        }),
    };
    let result = interpreter.evaluate(&compound_assign).unwrap();
    assert_eq!(result, Value::Number(15.0));

    // Verify the variable was updated
    let var_check = Expr::Variable {
        name: "x".to_string(),
    };
    let final_value = interpreter.evaluate(&var_check).unwrap();
    assert_eq!(final_value, Value::Number(15.0));
}
