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

#[test]
fn test_class_properties() {
    let code = r#"
        class Toast {}
        var toast = Toast();
        
        // 1. Set property
        toast.jam = "grape";
        
        // 2. Get property and check result
        var res = toast.jam; 
    "#;

    assert_eq!(eval_res(code).unwrap(), Value::String("grape".to_string()));
}

#[test]
fn test_class_property_undefined() {
    let code = r#"
        class T {}
        var t = T();
        print t.missing; // Should runtime error
    "#;

    assert!(eval_res(code).is_err());
}
