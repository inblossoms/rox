pub mod methods;
use crate::native_fn;

pub fn lookup(name: &str) -> Option<super::Value> {
    match name {
        "keys" => native_fn!("keys", 0, methods::keys),
        "values" => native_fn!("values", 0, methods::values),
        "remove" => native_fn!("remove", 1, methods::remove),
        "has" => native_fn!("has", 1, methods::has),
        _ => None,
    }
}
