use crate::std_lib::{Interpreter, Value, error::RuntimeError};

pub fn len(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    // args[0] 是 'this'，即调用该方法的字符串本身
    if let Value::String(s) = &args[0] {
        Ok(Value::Number(s.len() as f64))
    } else {
        Err(RuntimeError::TypeError("Expected string.".into()))
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluate::{Value, eval_res};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_string_len_method() {
        let ast = r#"
				var s = "hello world";
				var res = s.len();
		  "#;
        assert_eq!(eval_res(ast).unwrap(), Value::Number(11.0));
    }
}
