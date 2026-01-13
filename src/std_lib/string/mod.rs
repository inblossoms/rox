pub mod methods;
use super::value::{NativeFn, Value};

pub fn lookup(name: &str) -> Option<Value> {
    let func: NativeFn = match name {
        "len" => methods::len,
        //   "upper" => methods::upper,
        // ...
        _ => return None,
    };

    Some(Value::NativeFunction {
        name: name.to_string(),
        arity: 0, // Note：arity 指除 this 以外的参数个数
        func,
    })
}
