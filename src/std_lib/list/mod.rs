pub mod methods;

pub fn lookup(name: &str) -> Option<super::Value> {
    let func: super::value::NativeFn = match name {
        "push" => methods::push,
        "pop" => methods::pop,
        _ => return None,
    };

    Some(super::Value::NativeFunction {
        name: name.to_string(),
        arity: 1, // 列表方法只接受一个参数
        func,
    })
}
