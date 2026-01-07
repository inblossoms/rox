use crate::evaluate::{Value, tests::eval_res};

#[test]
fn test_class_instantiation() {
    let code = r#"
        class Bagel {}
        var res = Bagel(); 
    "#;

    let ret = "<instance Bagel>";
    assert_eq!(eval_res(code).unwrap().to_string(), ret.to_string()); // <instance Bagel>
}

#[test]
fn test_class_instantiation_internal() {
    let code = r#"
        class Bagel {}
        var res = Bagel();
    "#;

    let result = eval_res(code).unwrap();

    // 解构 Value::Instance
    if let Value::Instance(instance_rc) = result {
        let instance = instance_rc.borrow();

        let class = instance.class.borrow();

        assert_eq!(class.name, "Bagel");

        assert!(instance.fields.borrow().is_empty());
    } else {
        panic!("Expected Value::Instance, got {:?}", result);
    }
}
