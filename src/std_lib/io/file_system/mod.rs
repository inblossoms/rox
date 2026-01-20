mod methods;

use crate::evaluate::value::{RoxModule, Value};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

// 创建 fs 模块对象
pub fn create_module() -> Value {
    let mut exports = HashMap::new();

    // helper
    let define_native = |name: &str, arity: usize, func| Value::NativeFunction {
        name: name.to_string(),
        arity,
        func,
    };

    // 方法注册
    exports.insert(
        "readFile".to_string(),
        define_native("readFile", 1, methods::read_file),
    );
    exports.insert(
        "writeFile".to_string(),
        define_native("writeFile", 2, methods::write_file),
    );
    exports.insert(
        "exists".to_string(),
        define_native("exists", 1, methods::exists),
    );

    let module = RoxModule {
        name: "fs".to_string(),
        exports,
        is_initialized: true, // 原生模块天然是初始化好的
    };

    Value::Module(Rc::new(RefCell::new(module)))
}
