use crate::std_lib::{Interpreter, Value, error::RuntimeError};

pub fn push(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    // args[0] 是 list 实例，args[1] 是要 push 的值
    if let Value::List(list) = &args[0] {
        list.borrow_mut().push(args[1].clone());
        Ok(Value::Nil)
    } else {
        Err(RuntimeError::TypeError("Expected list.".into()))
    }
}

pub fn pop(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    // args[0] 是 list 实例
    if let Value::List(list) = &args[0] {
        let value = list.borrow_mut().pop();
        Ok(value.unwrap_or(Value::Nil))
    } else {
        Err(RuntimeError::TypeError("Expected list.".into()))
    }
}
