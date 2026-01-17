use crate::std_lib::{Interpreter, Value, error::RuntimeError, utils::ensure_list};

pub fn push(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    // args[0] 是 list 实例
    let list = ensure_list(&args[0])?;
    list.borrow_mut().push(args[1].clone());
    Ok(Value::String(args[1].to_string()))
}

pub fn pop(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let list = ensure_list(&args[0])?;
    let value = list.borrow_mut().pop();
    Ok(value.unwrap_or(Value::Nil))
}

pub fn len(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let list = ensure_list(&args[0])?;
    let value = list.borrow_mut().len();
    Ok(Value::Number(value as f64))
}

pub fn insert(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let list = ensure_list(&args[0])?;

    // 获取插入位置
    let index = match &args[1] {
        Value::Number(n) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return Err(RuntimeError::IndexError(
                    "Index must be a non-negative integer".to_string(),
                ));
            }
            *n as usize
        }
        _ => {
            return Err(RuntimeError::TypeError(
                "Index must be a number".to_string(),
            ));
        }
    };

    // 检查索引越界
    let list_len = list.borrow().len();
    if index > list_len {
        return Err(RuntimeError::IndexError(format!(
            "Index {} out of bounds (length {})",
            index, list_len
        )));
    }

    // 插入值 (args[2])
    let value = args[2].clone();

    list.borrow_mut().insert(index, value);

    Ok(Value::None)
}

pub fn join(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let list = ensure_list(&args[0])?;

    let separator = match &args[1] {
        Value::String(s) => s.clone(),
        _ => args[1].to_string(),
    };

    let list_ref = list.borrow();

    let joined = list_ref
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(&separator);

    Ok(Value::String(joined))
}

pub fn reverse(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let list = ensure_list(&args[0])?;
    let mut list_ref = list.borrow_mut();
    list_ref.reverse();
    Ok(Value::None)
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::std_lib::{Value, eval_res};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_push() {
        let ast = r#"
		   var list = [0];
		   var res = list.push(1);
		"#;

        assert_eq!(eval_res(ast).unwrap(), Value::String("1".to_string()));
    }

    #[test]
    fn test_pop() {
        let ast = r#"
		   var list = [1, 2, 3];
		   var res = list.pop();
		"#;

        assert_eq!(eval_res(ast).unwrap(), Value::Number(3.0));
    }

    #[test]
    fn test_join() {
        let ast = r#"
		   var list = [1, 2, 3];
		   var res = list.join(" ^ ");
		"#;

        assert_eq!(
            eval_res(ast).unwrap(),
            Value::String("1 ^ 2 ^ 3".to_string())
        );
    }

    #[test]
    fn test_reverse() {
        let ast = r#"
		   var list = [1, 2, 3];
		   list.reverse();
		   var res = list;
		"#;

        let expected = Value::List(Rc::new(RefCell::new(vec![
            Value::Number(3.0),
            Value::Number(2.0),
            Value::Number(1.0),
        ])));
        assert_eq!(eval_res(ast).unwrap(), expected);
    }

    #[test]
    fn test_len() {
        let ast = r#"
		   var list = [1, 2, 3];
		   var res = list.len();
		"#;

        assert_eq!(eval_res(ast).unwrap(), Value::Number(3.0));
    }

    #[test]
    fn test_insert() {
        let ast = r#"
		   var list = [1, 2, 3];
		   list.insert(0, 0);
		   var res = list;
		"#;

        let expected = Value::List(Rc::new(RefCell::new(vec![
            Value::Number(0.0),
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ])));
        assert_eq!(eval_res(ast).unwrap(), expected);
    }
}
