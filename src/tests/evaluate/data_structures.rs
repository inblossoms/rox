use crate::ast::Expr;
use crate::evaluate::{interpreter::evaluate_tests::create_test_interpreter, value::Value};
use std::collections::HashMap;

#[test]
fn test_list_operations() {
    let mut interpreter = create_test_interpreter();

    // Create a list: [1, 2, 3]
    let list_expr = Expr::List {
        elements: vec![
            Expr::Number {
                value: "1".to_string(),
            },
            Expr::Number {
                value: "2".to_string(),
            },
            Expr::Number {
                value: "3".to_string(),
            },
        ],
    };
    let result = interpreter.evaluate(&list_expr).unwrap();
    assert_eq!(
        result,
        Value::List(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ])
    );
}

#[test]
fn test_tuple_operations() {
    let mut interpreter = create_test_interpreter();

    let tuple_expr = Expr::Tuple {
        elements: vec![
            Expr::Number {
                value: "1".to_string(),
            },
            Expr::String {
                value: "hello".to_string(),
            },
        ],
    };
    let result = interpreter.evaluate(&tuple_expr).unwrap();
    assert_eq!(
        result,
        Value::Tuple(vec![Value::Number(1.0), Value::String("hello".to_string()),])
    );
}

#[test]
fn test_dict_operations() {
    let mut interpreter = create_test_interpreter();

    let dict_expr = Expr::Dict {
        elements: vec![
            (
                Expr::String {
                    value: "key1".to_string(),
                },
                Expr::Dict {
                    elements: vec![
                        (
                            Expr::String {
                                value: "key1".to_string(),
                            },
                            Expr::Number {
                                value: "1".to_string(),
                            },
                        ),
                        (
                            Expr::String {
                                value: "key2".to_string(),
                            },
                            Expr::Boolean { value: true },
                        ),
                    ],
                },
            ),
            (
                Expr::String {
                    value: "key2".to_string(),
                },
                Expr::String {
                    value: "value".to_string(),
                },
            ),
        ],
    };
    let result = interpreter.evaluate(&dict_expr).unwrap();
    let mut expected_dict = HashMap::new();
    expected_dict.insert(
        "key1".to_string(),
        Value::Dict(HashMap::from([
            ("key1".to_string(), Value::Number(1.0)),
            ("key2".to_string(), Value::Boolean(true)),
        ])),
    );
    expected_dict.insert("key2".to_string(), Value::String("value".to_string()));
    assert_eq!(result, Value::Dict(expected_dict));
}
