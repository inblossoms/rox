pub mod methods;
use super::value::Value;
use crate::native_fn;
pub fn lookup(name: &str) -> Option<Value> {
    match name {
        "len" => native_fn!("len", 0, methods::len),
        "split" => native_fn!("split", 1, methods::split),
        "substring" => native_fn!("substring", 2, methods::substring),
        "replace" => native_fn!("replace", 2, methods::replace),
        _ => None,
    }
}
