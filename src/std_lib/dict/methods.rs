use crate::std_lib::{Interpreter, Value, error::RuntimeError, utils::ensure_dict};
use std::{cell::RefCell, rc::Rc};

pub fn keys(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let dict = ensure_dict(&args[0])?;
    let borrowed_dict = dict.borrow();
    let keys = borrowed_dict.keys();

    let rox_keys = keys
        .map(|k| Value::String(k.clone()))
        .collect::<Vec<Value>>();

    Ok(Value::List(Rc::new(RefCell::new(rox_keys))))
}

pub fn values(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let dict = ensure_dict(&args[0])?;
    let borrowed_dict = dict.borrow();
    let values = borrowed_dict.values();

    let rox_values = values.cloned().collect::<Vec<Value>>();

    Ok(Value::List(Rc::new(RefCell::new(rox_values))))
}

pub fn has(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let dict = ensure_dict(&args[0])?;
    Ok(Value::Boolean(
        dict.borrow().get(&args[1].to_string()).is_some(),
    ))
}

pub fn remove(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let dict = ensure_dict(&args[0])?;
    Ok(dict
        .borrow_mut()
        .remove(&args[1].to_string())
        .unwrap_or(Value::Nil))
}

#[cfg(test)]
mod tests {
    use crate::std_lib::{Value, eval_res};
    use pretty_assertions::assert_eq;

    // helper

    // 由于 HashMap 的迭代顺序是不确定的，将 Value::List 转换为排序后的Vec
    fn sorted_list_values(list: &Value) -> Vec<Value> {
        if let Value::List(rc_list) = list {
            let mut values: Vec<Value> = rc_list.borrow().clone();
            values.sort_by_key(|a| a.to_string());
            values
        } else {
            panic!("Expected list value");
        }
    }

    #[test]
    fn test_dict_keys_method() {
        let ast = r#"
            var dict = {"a": 1, "b": 2, "c": 3};
            var res = dict.keys();
        "#;

        let result = eval_res(ast).unwrap();
        let keys = sorted_list_values(&result);

        assert_eq!(
            keys,
            vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
                Value::String("c".to_string())
            ]
        );
    }

    #[test]
    fn test_dict_values_method() {
        let ast = r#"
            var dict = {"a": 1, "b": 2, "c": "three"};
            var res = dict.values();
        "#;

        let result = eval_res(ast).unwrap();
        let values = sorted_list_values(&result);

        assert_eq!(
            values,
            vec![
                Value::Number(1.0),
                Value::Number(2.0),
                Value::String("three".to_string())
            ]
        );
    }

    #[test]
    fn test_dict_has_method() {
        let ast = r#"
            var dict = {"a": 1, "b": 2, "c": "three"};
            var res = dict["a"];
        "#;

        assert_eq!(eval_res(ast).unwrap(), Value::Number(1.0));
    }

    #[test]
    fn test_dict_remove_method() {
        let ast = r#"
            var dict = {"a": 1, "b": 2, "c": "three"};
            var res = dict.remove("a");
        "#;

        assert_eq!(eval_res(ast).unwrap(), Value::Number(1.0));
    }
}
