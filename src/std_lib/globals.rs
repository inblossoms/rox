use crate::evaluate::{error::RuntimeError, interpreter::Interpreter, value::Value};
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

// clock(): 获取时间戳（用于性能测试）。clock() -> Number
pub fn clock(_: &mut Interpreter, _: Vec<Value>) -> Result<Value, RuntimeError> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    Ok(Value::Number(since_the_epoch.as_secs_f64()))
}

// input(): 读取用户输入。input(prompt) -> String
pub fn input(_: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError> {
    // 1. 如果有提示符，先打印
    if let Some(prompt) = args.first() {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
    }

    // 2. 读取 stdin
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(|_| RuntimeError::Generic("Failed to read input".into()))?;

    // 3. 去除末尾换行符
    Ok(Value::String(buffer.trim_end().to_string()))
}

// IO 与系统交互 (System Library)
// TODO:
// file_read(path), file_write(path): 文件操作。
// exit(code): 退出程序。
