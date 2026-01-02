#![allow(dead_code)]
use super::Operator;

#[derive(Debug)]
pub struct AST {
    pub top: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
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
    List {
        elements: Vec<Expr>,
    },
    Dict {
        elements: Vec<(Expr, Expr)>,
    },
    Tuple {
        elements: Vec<Expr>,
    },
    Variable {
        name: String,
    },
    AssignOp {
        op: Operator,
        name: String,
        expr: Box<Expr>,
    },
    Binary {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: Operator,
        expr: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Identifier {
        name: String,
    },
    VarDecl {
        name: String,
        initializer: Box<Expr>,
    },
    Assign {
        name: String,
        expr: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Function {
        name: String,
        args: Vec<String>,
        body: Vec<Expr>,
    },
    Return {
        expr: Box<Expr>,
    },
    Block {
        body: Vec<Expr>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    For {
        initializer: Box<Expr>,
        condition: Box<Expr>,
        increment: Box<Expr>,
        body: Box<Expr>,
    },
    Print {
        expr: Box<Expr>,
    },
    Break,
    Continue,
}

#[allow(dead_code)]
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
    pub fn variable(name: String) -> Expr {
        Expr::Variable { name }
    }
    pub fn assign_op(op: Operator, name: String, expr: Expr) -> Expr {
        Expr::AssignOp {
            op,
            name,
            expr: expr.into(),
        }
    }
    pub fn binary(op: Operator, left: Expr, right: Expr) -> Expr {
        Expr::Binary {
            op,
            left: left.into(),
            right: right.into(),
        }
    }
    pub fn unary(op: Operator, expr: Expr) -> Expr {
        Expr::Unary {
            op,
            expr: expr.into(),
        }
    }
    pub fn grouping(expr: Expr) -> Expr {
        Expr::Grouping { expr: expr.into() }
    }
    pub fn identifier(name: String) -> Expr {
        Expr::Identifier { name }
    }
    pub fn assign(name: String, expr: Expr) -> Expr {
        Expr::Assign {
            name,
            expr: expr.into(),
        }
    }
    pub fn call(name: String, args: Vec<Expr>) -> Expr {
        Expr::Call { name, args }
    }
    pub fn function(name: String, args: Vec<String>, body: Vec<Expr>) -> Expr {
        Expr::Function { name, args, body }
    }
    pub fn return_(expr: Expr) -> Expr {
        Expr::Return { expr: expr.into() }
    }
    pub fn block(body: Vec<Expr>) -> Expr {
        Expr::Block { body }
    }
    pub fn if_(condition: Expr, then_branch: Expr, else_branch: Option<Expr>) -> Expr {
        Expr::If {
            condition: condition.into(),
            then_branch: then_branch.into(),
            else_branch: else_branch.map(|e| Box::new(e)),
        }
    }
    pub fn while_(condition: Expr, body: Expr) -> Expr {
        Expr::While {
            condition: condition.into(),
            body: body.into(),
        }
    }
    pub fn break_() -> Expr {
        Expr::Break
    }
    pub fn continue_() -> Expr {
        Expr::Continue
    }
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

#[cfg(test)]
#[path = "tests/mod.rs"]
mod ast_tests;
