pub mod methods;
use crate::native_fn;

pub fn lookup(name: &str) -> Option<super::Value> {
    match name {
        "push" => native_fn!("push", 1, methods::push),
        "pop" => native_fn!("pop", 0, methods::pop),
        "len" => native_fn!("len", 0, methods::len),
        "insert" => native_fn!("insert", 2, methods::insert),
        "join" => native_fn!("join", 1, methods::join),
        "reverse" => native_fn!("reverse", 0, methods::reverse),
        _ => None,
    }
}
