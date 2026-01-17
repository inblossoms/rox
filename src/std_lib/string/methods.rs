use std::{cell::RefCell, rc::Rc};

use crate::std_lib::{Interpreter, Value, error::RuntimeError, utils::ensure_string};

pub fn len(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let s = ensure_string(&args[0])?;
    Ok(Value::Number(s.len() as f64))
}

// str.split(delimiter) -> List
pub fn split(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let s = ensure_string(&args[0])?;
    let delimiter = ensure_string(&args[1])?;

    Ok(Value::List(Rc::new(RefCell::new(
        s.split(delimiter)
            .map(|s| Value::String(s.to_string()))
            .collect::<Vec<Value>>(),
    ))))
}

// str.substring(start, end)
pub fn substring(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let s = ensure_string(&args[0])?;
    let start = args[1].clone().to_string().parse::<usize>().unwrap();
    let end = args[2].clone().to_string().parse::<usize>().unwrap();

    Ok(Value::String(s[start..end].to_string()))
}

// str.replace(old, new)
pub fn replace(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let s = ensure_string(&args[0])?;
    let old = ensure_string(&args[1])?;
    let new = ensure_string(&args[2])?;

    Ok(Value::String(s.replace(old, new)))
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::evaluate::{Value, eval_res};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_string_len_method() {
        let ast = r#"
				var s = "hello ray";
				var res = s.len();
		  "#;
        assert_eq!(eval_res(ast).unwrap(), Value::Number(9.0));
    }

    #[test]
    fn test_string_split_method() {
        let ast = r#"
				var s = "hello,ray";
				var res = s.split(",");
		  "#;

        let expected = Value::List(Rc::new(RefCell::new(vec![
            Value::String("hello".to_string()),
            Value::String("ray".to_string()),
        ])));
        assert_eq!(eval_res(ast).unwrap(), expected);
    }

    #[test]
    fn test_string_substring_method() {
        let ast = r#"
				var s = "hello ray";
				var res = s.substring(6, 9);
		  "#;

        assert_eq!(eval_res(ast).unwrap(), Value::String("ray".to_string()));
    }

    #[test]
    fn test_string_replace_method() {
        let ast = r#"
				var s = "hello world";
				var res = s.replace("world", "ray");
		  "#;
        let expected = Value::String("hello ray".to_string());
        assert_eq!(eval_res(ast).unwrap(), expected);
    }
}
