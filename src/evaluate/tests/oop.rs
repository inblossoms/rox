use crate::evaluate::{Value, tests::eval_res};

/// define、initializer、fields Get/Set
#[test]
fn test_class_fields() {
    let code = r#"
        class Point {}
        
        var p = Point();
        p.x = 10;
        p.y = 20;
        
        // 测试字段读取和简单的算术
        var res = p.x + p.y; 
    "#;

    assert_eq!(eval_res(code).unwrap(), Value::Number(30.0));
}

/// method call and 'this' implicit binding
#[test]
fn test_method_this() {
    let code = r#"
        class Person {
            sayName(target) {
                return this.name + " says hello to " + target;
            }
        }

        var jane = Person();
        jane.name = "Jane";
        
        var res = jane.sayName("Bob");
    "#;

    assert_eq!(
        eval_res(code).unwrap(),
        Value::String("Jane says hello to Bob".to_string())
    );
}

/// Persistent Binding
#[test]
fn test_method_closure_binding() {
    let code = r#"
        class Box {
            setValue(v) {
                this.val = v;
            }
            getValue() {
                return this.val;
            }
        }

        var box = Box();
        box.setValue("Gold");
        
        // 把方法取出来赋值给变量
        var getMethod = box.getValue;
        
        // 在外部调用，this 应该依然指向 box，而不是 nil 或 global
        var res = getMethod(); 
    "#;

    assert_eq!(eval_res(code).unwrap(), Value::String("Gold".to_string()));
}

/// Override
#[test]
fn test_inheritance_override() {
    let code = r#"
        class Animal {
            speak() { return "silent"; }
        }

        class Dog < Animal {
            speak() { return "bark"; }
        }

        class Cat < Animal {} // 继承但不重写

        var dog = Dog();
        var cat = Cat();
        
        var res = dog.speak() + " and " + cat.speak();
    "#;

    assert_eq!(
        eval_res(code).unwrap(),
        Value::String("bark and silent".to_string())
    );
}

/// Super
#[test]
fn test_super_calls() {
    let code = r#"
        class Doughnut {
            cook() {
                return "Fry until golden brown.";
            }
        }

        class BostonCream < Doughnut {
            cook() {
                // 调用父类方法，并拼接字符串
                return super.cook() + " | Pipe full of custard.";
            }
        }

        var res = BostonCream().cook();
    "#;

    assert_eq!(
        eval_res(code).unwrap(),
        Value::String("Fry until golden brown. | Pipe full of custard.".to_string())
    );
}

/// init 与早起返回（提前返回）
#[test]
fn test_initializer() {
    let code = r#"
        class User {
            init(name) {
                this.name = name;
                // init 隐式返回 this，不需要写 return this;
                // 但如果写 return; 也是合法的（提前结束）
            }
        }

        var user = User("Admin");
        var res = user.name;
    "#;

    assert_eq!(eval_res(code).unwrap(), Value::String("Admin".to_string()));
}

#[test]
fn test_oop_grand_finale() {
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
                // 这里的 super.method() 应该调用 A.method()
                return super.method();
            }
        }

        class C < B {
            // C 没有重写 method，所以 C().method() 应该调用 B.method()
        }

        var b = B();
        var c = C();
        
        // 1. 调用 B 的 test -> super.method -> A.method -> "A"
        var res1 = b.test(); 
        
        // 2. 调用 C 的 method -> 继承自 B -> "B"
        var res2 = c.method();
        
        var res = res1 + res2;
    "#;

    assert_eq!(eval_res(code).unwrap(), Value::String("AB".to_string()));
}

/// 负面测试：静态检查 (Resolver 应该报错)
#[test]
fn test_error_super_outside_class() {
    let code = "print super.cook();";
    // Resolver 应该抛出 "Can't use 'super' outside of a class."
    assert!(eval_res(code).is_err());
}

#[test]
fn test_error_this_outside_class() {
    let code = "print this;";
    // Resolver 应该抛出 "Can't use 'this' outside of a class."
    assert!(eval_res(code).is_err());
}

#[test]
fn test_error_return_value_from_init() {
    let code = r#"
        class Foo {
            init() {
                return "something"; // Error
            }
        }
    "#;
    // Resolver 应该抛出 "Can't return a value from an initializer."
    assert!(eval_res(code).is_err());
}
