pub mod environment;
pub mod error;
pub mod interpreter;
pub mod value;

pub use interpreter::Interpreter;
pub use value::Value;

#[cfg(test)]
pub mod tests;

#[cfg(test)]
pub use tests::eval_res;
