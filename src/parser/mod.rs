// parser 的职责是确保 AST 合法的
pub mod error;
pub mod expression;
pub mod parse;
pub mod statement;

// 导出 parser 模块中的 Error 类型和 parse 函数
pub use error::Error;
pub use parse::parse;
