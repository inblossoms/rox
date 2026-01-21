use crate::{
    evaluate::error::RuntimeError,
    std_lib::{
        Value,
        math::{rand_int, rand_range, random},
        value::RoxModule,
    },
};
use std::{cell::RefCell, collections::HashMap, f64::consts::PI, rc::Rc};

/// 从参数列表中提取 f64，类型不对则报错
fn get_num(args: &[Value], index: usize) -> Result<f64, RuntimeError> {
    match args.get(index) {
        Some(Value::Number(n)) => Ok(*n),
        _ => Err(RuntimeError::TypeError(format!(
            "Argument {} must be a number.",
            index + 1
        ))),
    }
}

macro_rules! math_parameterless {
    ($func_name:ident, $rust_fn:expr) => {
        |_, args| {
            if args.len() != 0 {
                return Err(RuntimeError::Generic(format!(
                    "'{}' expects 0 argument.",
                    stringify!($func_name)
                )));
            }
            Ok(Value::Number($rust_fn))
        }
    };
}

/// 一元数学函数 ( f64 -> f64 )
/// eg. abs, ceil, floor, sqrt, sin, cos
macro_rules! math_unary {
    ($func_name:ident, $rust_fn:expr) => {
        |_, args| {
            if args.len() != 1 {
                return Err(RuntimeError::Generic(format!(
                    "'{}' expects 1 argument.",
                    stringify!($func_name)
                )));
            }
            let val = get_num(&args, 0)?;
            // 直接调用传入的 Rust 函数
            let res = $rust_fn(val);
            Ok(Value::Number(res))
        }
    };
}

/// 二元数学函数 ( (f64, f64) -> f64 )
/// eg. pow, min, max
macro_rules! math_binary {
    ($func_name:ident, $rust_fn:expr) => {
        |_, args| {
            if args.len() != 2 {
                return Err(RuntimeError::Generic(format!(
                    "'{}' expects 2 arguments.",
                    stringify!($func_name)
                )));
            }
            let v1 = get_num(&args, 0)?;
            let v2 = get_num(&args, 1)?;
            let res = $rust_fn(v1, v2);
            Ok(Value::Number(res))
        }
    };
}

/// 定义返回整数的一元函数 ( f64 -> i32/i64 -> f64 )
macro_rules! math_unary_int {
    ($func_name:ident, $rust_fn:expr) => {
        |_, args| {
            if args.len() != 1 {
                return Err(RuntimeError::Generic(format!(
                    "'{}' expects 1 argument.",
                    stringify!($func_name)
                )));
            }
            let val = get_num(&args, 0)?;
            // 整个解释器的实现中一切 number 类型均为 f64
            // 处理类型转换：Rust 算出来是 int 需转回 f64
            let res = $rust_fn(val) as f64;
            Ok(Value::Number(res))
        }
    };
}

macro_rules! math_binary_int {
    ($func_name:ident, $rust_fn:expr) => {
        |_, args| {
            if args.len() != 2 {
                return Err(RuntimeError::Generic(format!(
                    "'{}' expects 2 argument.",
                    stringify!($func_name)
                )));
            }
            let l_val = get_num(&args, 0)?;
            let r_val = get_num(&args, 1)?;
            let res = $rust_fn(l_val, r_val) as f64;
            Ok(Value::Number(res))
        }
    };
}

// 模块构建

pub fn create_module() -> Value {
    let mut exports = HashMap::new();

    // helper
    let register = |exports_map: &mut HashMap<String, Value>, name: &str, arity: usize, func| {
        exports_map.insert(
            name.to_string(),
            Value::NativeFunction {
                name: name.to_string(),
                arity,
                func,
            },
        );
    };

    // 方法注册
    // 常量作为 Number 存入
    exports.insert("PI".to_string(), Value::Number(PI));
    exports.insert("E".to_string(), Value::Number(std::f64::consts::E));

    register(
        &mut exports,
        "random",
        0,
        math_parameterless!(random, random()),
    );
    register(&mut exports, "abs", 1, math_unary!(abs, f64::abs));
    register(&mut exports, "ceil", 1, math_unary!(ceil, f64::ceil));
    register(&mut exports, "floor", 1, math_unary!(floor, f64::floor));
    register(&mut exports, "round", 1, math_unary!(round, f64::round));
    register(&mut exports, "sqrt", 1, math_unary!(sqrt, f64::sqrt));

    register(&mut exports, "sin", 1, math_unary!(sin, f64::sin));
    register(&mut exports, "cos", 1, math_unary!(cos, f64::cos));
    register(&mut exports, "tan", 1, math_unary!(tan, f64::tan));

    register(&mut exports, "log", 1, math_unary!(log, f64::ln)); // 自然对数
    register(&mut exports, "log10", 1, math_unary!(log10, f64::log10));
    register(&mut exports, "exp", 1, math_unary!(exp, f64::exp));

    register(&mut exports, "pow", 2, math_binary!(pow, f64::powf));
    register(&mut exports, "min", 2, math_binary!(min, f64::min));
    register(&mut exports, "max", 2, math_binary!(max, f64::max));

    // 判断是否是整数
    register(&mut exports, "is_int", 1, |_, args| {
        let v = get_num(&args, 0)?;
        // 处理非 f64 返回，值返回 Boolean
        Ok(Value::Boolean(v.fract() == 0.0))
    });

    // 强制转整型
    register(
        &mut exports,
        "to_int",
        1,
        math_unary_int!(to_int, |v: f64| v as i64),
    );

    register(
        &mut exports,
        "rand_int",
        2,
        math_binary_int!(rand_int, |min: f64, max: f64| rand_int(
            min as i32, max as i32
        )),
    );

    #[allow(clippy::redundant_closure)]
    register(
        &mut exports,
        "rand_range",
        2,
        math_binary!(rand_range, |min, max| rand_range(min, max)),
    );

    let module = RoxModule {
        name: "math".to_string(),
        exports,
        is_initialized: true,
    };

    Value::Module(Rc::new(RefCell::new(module)))
}
