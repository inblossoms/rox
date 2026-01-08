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

#[test]
fn test_class_this_binding() {
    let code = r#"
        class Egotist {
            speak() {
                return this;
            }
        }

        var res = Egotist().speak();
    "#;

    let ret = eval_res(code).unwrap();
    // 检查是否可以像闭包一样被传递，并且 "this" 是持久绑定的
    assert_eq!(ret.to_string(), "<instance Egotist>");
}

#[test]
fn test_class_method_access_fields() {
    let code = r#"
        class Cake {
            taste() {
                var adjective = "delicious";
                return "The " + this.flavor + " cake is " + adjective + "!";
            }
        }

        var cake = Cake();
        cake.flavor = "German chocolate";
        var res = cake.taste(); 
    "#;

    assert_eq!(
        eval_res(code).unwrap(),
        Value::String("The German chocolate cake is delicious!".to_string())
    );
}

#[test]
fn test_this_outside_class() {
    let code = "print this;";
    assert!(eval_res(code).is_err()); // Resolver will panic
}

#[test]
fn test_inheritance() {
    let code = r#"
        class Doughnut {
            cook() {
                return "Fry until golden brown.";
            }
        }

        class BostonCream < Doughnut {}

        var res = BostonCream().cook();
    "#;
    assert_eq!(
        eval_res(code).unwrap(),
        Value::String("Fry until golden brown.".to_string())
    );
}

#[test]
fn test_super_call() {
    let code = r#"
        class A {
            method() {
                return "A";
            }
        }

        class B < A {
            method() {
                return "B";
            }
            test() {
                return super.method();
            }
        }

        class C < B {}

        var res = C().test();
    "#;
    // C 继承 B，B 继承 A。
    // C().test() -> B.test() -> super.method() -> A.method() -> "A"
    assert_eq!(eval_res(code).unwrap(), Value::String("A".to_string()));
}
