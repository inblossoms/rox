#![allow(dead_code)]
use super::Operator;
use crate::{
    ast::{helper::generate_token, stmt::Stmt},
    tokenizer::{Token, TokenType},
};

#[derive(Debug)]
pub struct AST {
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExprId(pub usize);

/// Expr：表达式节点，程序中所有可能的表达式类型。
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // literals
    Number {
        value: String,
    },
    String {
        value: String,
    },
    Boolean {
        value: bool,
    },
    Nil,

    // collections
    List {
        elements: Vec<Expr>,
    },
    Dict {
        elements: Vec<(Expr, Expr)>,
    },
    Tuple {
        elements: Vec<Expr>,
    },

    // variable reading
    Variable {
        id: ExprId,
        name: Token,
    },

    // variable writing
    Assign {
        id: ExprId,
        name: Token,
        expr: Box<Expr>,
    },

    // compound assignment
    AssignOp {
        id: ExprId,
        op: Operator,
        name: Token,
        expr: Box<Expr>,
    },

    // operations
    Binary {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: Operator,
        expr: Box<Expr>,
    },

    // 逻辑运算具有短路行为，不同于 Binary
    Logical {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },

    // 调用
    Call {
        id: ExprId, // 函数名本质上也是变量引用
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
}

impl Expr {
    pub fn number(value: impl Into<String>) -> Expr {
        Expr::Number {
            value: value.into(),
        }
    }

    pub fn string(value: impl Into<String>) -> Expr {
        Expr::String {
            value: value.into(),
        }
    }

    pub fn boolean(value: bool) -> Expr {
        Expr::Boolean { value }
    }

    /// 空值对象
    pub fn nil() -> Expr {
        Expr::Nil
    }

    pub fn list(elements: Vec<Expr>) -> Expr {
        Expr::List { elements }
    }

    pub fn dict(elements: Vec<(Expr, Expr)>) -> Expr {
        Expr::Dict { elements }
    }

    pub fn tuple(elements: Vec<Expr>) -> Expr {
        Expr::Tuple { elements }
    }

    /// 创建变量对象
    pub fn variable(name: Token) -> Expr {
        Expr::Variable {
            id: ExprId(0),
            name,
        }
    }

    /// 通过给定的字符串切片创建一个变量
    pub fn variable_str(name: &str) -> Expr {
        Expr::Variable {
            id: ExprId(0),
            name: generate_token(TokenType::Identifier, name),
        }
    }

    pub fn assign(name: Token, expr: Expr) -> Expr {
        Expr::Assign {
            id: ExprId(0),
            name,
            expr: Box::new(expr),
        }
    }

    pub fn assign_op(op: Operator, name: &str, expr: Expr) -> Expr {
        Expr::AssignOp {
            id: ExprId(0),
            name: generate_token(TokenType::Identifier, name),
            op,
            expr: Box::new(expr),
        }
    }

    pub fn binary(op: Operator, left: Expr, right: Expr) -> Expr {
        Expr::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn unary(op: Operator, expr: Expr) -> Expr {
        Expr::Unary {
            op,
            expr: Box::new(expr),
        }
    }

    pub fn logical(op: Operator, left: Expr, right: Expr) -> Expr {
        Expr::Logical {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn grouping(expr: Expr) -> Expr {
        Expr::Grouping {
            expr: Box::new(expr),
        }
    }

    pub fn call(callee: Expr, args: Vec<Expr>) -> Expr {
        Expr::Call {
            id: ExprId(0),
            // callee: Token
            // 当 callee 类型限制为 Token 时，这将意味着只支持通过变量名来调用
            callee: Box::new(callee),
            args,
        }
    }

    /// 对指定的函数进行调用
    pub fn call_str(name: &str, args: Vec<Expr>) -> Expr {
        Expr::Call {
            id: ExprId(0),
            callee: Box::new(Expr::variable_str(name)),
            args,
        }
    }

    // helper

    pub fn is_number(&self) -> bool {
        matches!(self, Expr::Number { .. })
    }
    pub fn is_string(&self) -> bool {
        matches!(self, Expr::String { .. })
    }
    pub fn is_boolean(&self) -> bool {
        matches!(self, Expr::Boolean { .. })
    }
    pub fn is_nil(&self) -> bool {
        matches!(self, Expr::Nil)
    }
}
