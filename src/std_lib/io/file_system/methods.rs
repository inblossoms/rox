use crate::evaluate::{error::RuntimeError, interpreter::Interpreter, value::Value};
use std::fs;
use std::path::Path;

// fs.readFile(path) -> String
pub fn read_file(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let path_str = match args.first() {
        Some(Value::String(s)) => s,
        _ => return Err(RuntimeError::TypeError("Path must be a string.".into())),
    };

    match fs::read_to_string(path_str) {
        Ok(content) => Ok(Value::String(content)),
        Err(e) => Err(RuntimeError::Generic(format!("Failed to read file: {}", e))),
    }
}

// fs.writeFile(path, content) -> Nil
pub fn write_file(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::Generic("Expected 2 arguments.".into()));
    }

    let path_str = match &args[0] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("Path must be a string.".into())),
    };

    let content = match &args[1] {
        Value::String(s) => s,
        _ => return Err(RuntimeError::TypeError("Content must be a string.".into())),
    };

    match fs::write(path_str, content) {
        Ok(_) => Ok(Value::Nil),
        Err(e) => Err(RuntimeError::Generic(format!(
            "Failed to write file: {}",
            e
        ))),
    }
}

// fs.exists(path) -> Bool
pub fn exists(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    let path_str = match args.first() {
        Some(Value::String(s)) => s,
        _ => return Err(RuntimeError::TypeError("Path must be a string.".into())),
    };

    Ok(Value::Boolean(Path::new(path_str).exists()))
}
