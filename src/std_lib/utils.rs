use crate::std_lib::{Value, error::RuntimeError};
use std::{cell::RefCell, collections::HashMap};

pub fn ensure_string(val: &Value) -> Result<&String, RuntimeError> {
    if let Value::String(s) = val {
        Ok(s)
    } else {
        Err(RuntimeError::TypeError("Expected string.".into()))
    }
}

pub fn ensure_list(args: &Value) -> Result<&RefCell<Vec<Value>>, RuntimeError> {
    if let Value::List(list) = args {
        Ok(list)
    } else {
        Err(RuntimeError::TypeError("Expected list.".into()))
    }
}

pub fn ensure_dict(val: &Value) -> Result<&RefCell<HashMap<String, Value>>, RuntimeError> {
    if let Value::Dict(dict) = val {
        Ok(dict)
    } else {
        Err(RuntimeError::TypeError("Expected dict.".into()))
    }
}
