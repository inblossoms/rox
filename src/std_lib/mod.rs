// pub mod dict;
// pub mod globals;
pub mod list;
pub mod string;
pub mod utils;

pub use crate::evaluate::*;

/// 查找原生方法
///
/// # 参数
/// * `target` - 调用方法的对象 (用于判断类型: String? List? ..)
/// * `name` - 方法名 (如 "len", "push")
pub fn lookup_method(target: &Value, name: &str) -> Option<Value> {
    match target {
        Value::String(_) => string::lookup(name),
        Value::List(_) => list::lookup(name),
        //   Value::Dict(_) => dict::lookup(name),
        _ => None,
    }
}

#[macro_export]
macro_rules! native_fn {
    ($name:expr, $arity:expr, $func:path) => {
        Some($crate::evaluate::value::Value::NativeFunction {
            name: $name.to_string(),
            arity: $arity,
            func: $func,
        })
    };
}
