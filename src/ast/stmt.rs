#![allow(dead_code)]

use crate::{
    ast::{Expr, helper::generate_token},
    tokenizer::{Token, TokenType},
};

/// 语句（Statement）：执行动作和控制流的结构（语句用来执行操作，而不是产生值）
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // eg："1+1;"、"func();"
    Expression {
        expr: Expr,
    },

    Empty,

    // Declaration
    // 声明不需要 ExprId，声明节点是变量的定义位置，Resolver 解析器会记录变量定义，
    // 在解释执行阶段，其他引用该变量的地方需要在 locals表 中查找作用域距离，但声明本身不需要。
    VarDecl {
        name: Token,
        initializer: Option<Expr>,
    },

    Function {
        name: Token,
        params: Vec<Token>, // Token： 参数本质是定义在函数作用域中声明的局部变量，每一个参数名需要被当作变量标识符来处理
        body: Vec<Stmt>,    // 函数体是一组语句
    },

    Class {
        name: Token,
        methods: Vec<Stmt>,
    },

    // control flow
    Block {
        body: Vec<Stmt>,
    },

    If {
        condition: Expr,
        then_branch: Box<Stmt>, // Stmt 类型中包含指向自身的指针，允许语句可以包含其他语句（eg：if 结构嵌套）做为子结构
        else_branch: Option<Box<Stmt>>,
    },

    While {
        condition: Expr,
        body: Box<Stmt>,
    },

    For {
        initializer: Option<Box<Stmt>>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Box<Stmt>,
    },

    Print {
        expr: Expr,
    },

    Return {
        keyword: Token,
        value: Option<Expr>,
    },

    Break,
    Continue,
}

impl Stmt {
    pub fn expression(expr: Expr) -> Stmt {
        Stmt::Expression { expr }
    }

    pub fn empty() -> Stmt {
        Stmt::Empty
    }

    pub fn var(name: &str, initializer: Option<Expr>) -> Stmt {
        Stmt::VarDecl {
            name: generate_token(TokenType::Identifier, name),
            initializer,
        }
    }

    pub fn function(name: &str, params: Vec<&str>, body: Vec<Stmt>) -> Stmt {
        let param_tokens = params
            .into_iter()
            .map(|p| generate_token(TokenType::Identifier, p))
            .collect();

        Stmt::Function {
            name: generate_token(TokenType::Identifier, name),
            params: param_tokens,
            body,
        }
    }

    pub fn class(name: &str, methods: Vec<Stmt>) -> Stmt {
        Stmt::Class {
            name: generate_token(TokenType::Identifier, name),
            methods,
        }
    }

    pub fn block(body: Vec<Stmt>) -> Stmt {
        Stmt::Block { body }
    }

    pub fn if_(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Stmt {
        Stmt::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        }
    }

    pub fn while_(condition: Expr, body: Stmt) -> Stmt {
        Stmt::While {
            condition,
            body: Box::new(body),
        }
    }

    pub fn for_(
        initializer: Option<Stmt>,
        condition: Option<Expr>,
        increment: Option<Expr>,
        body: Stmt,
    ) -> Stmt {
        Stmt::For {
            initializer: initializer.map(Box::new),
            condition,
            increment,
            body: Box::new(body),
        }
    }

    pub fn print(expr: Expr) -> Stmt {
        Stmt::Print { expr }
    }

    pub fn return_(expr: Option<Expr>) -> Stmt {
        Stmt::Return {
            keyword: generate_token(TokenType::Return, "return"),
            value: expr,
        }
    }

    pub fn break_() -> Stmt {
        Stmt::Break
    }

    pub fn continue_() -> Stmt {
        Stmt::Continue
    }
}
