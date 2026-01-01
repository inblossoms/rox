#![allow(dead_code)]
#[derive(Debug)]
pub struct AST {
    pub top: Option<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    Not,
    Assign,
    AddAssign,
    LogicalOr,
    LogicalAnd,
    BitwiseAnd,
    BitwiseOr,
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
    fn number(value: impl Into<String>) -> Expr {
        Expr::Number {
            value: value.into(),
        }
    }
    fn string(value: impl Into<String>) -> Expr {
        Expr::String {
            value: value.into(),
        }
    }
    fn boolean(value: bool) -> Expr {
        Expr::Boolean { value }
    }
    fn nil() -> Expr {
        Expr::Nil
    }
    fn list(elements: Vec<Expr>) -> Expr {
        Expr::List { elements }
    }
    fn dict(elements: Vec<(Expr, Expr)>) -> Expr {
        Expr::Dict { elements }
    }
    fn tuple(elements: Vec<Expr>) -> Expr {
        Expr::Tuple { elements }
    }
    fn variable(name: String) -> Expr {
        Expr::Variable { name }
    }
    fn assign_op(op: Operator, name: String, expr: Expr) -> Expr {
        Expr::AssignOp {
            op,
            name,
            expr: expr.into(),
        }
    }
    fn binary(op: Operator, left: Expr, right: Expr) -> Expr {
        Expr::Binary {
            op,
            left: left.into(),
            right: right.into(),
        }
    }
    fn unary(op: Operator, expr: Expr) -> Expr {
        Expr::Unary {
            op,
            expr: expr.into(),
        }
    }
    fn grouping(expr: Expr) -> Expr {
        Expr::Grouping { expr: expr.into() }
    }
    fn identifier(name: String) -> Expr {
        Expr::Identifier { name }
    }
    fn assign(name: String, expr: Expr) -> Expr {
        Expr::Assign {
            name,
            expr: expr.into(),
        }
    }
    fn call(name: String, args: Vec<Expr>) -> Expr {
        Expr::Call { name, args }
    }
    fn function(name: String, args: Vec<String>, body: Vec<Expr>) -> Expr {
        Expr::Function { name, args, body }
    }
    fn return_(expr: Expr) -> Expr {
        Expr::Return { expr: expr.into() }
    }
    fn block(body: Vec<Expr>) -> Expr {
        Expr::Block { body }
    }
    fn if_(condition: Expr, then_branch: Expr, else_branch: Option<Expr>) -> Expr {
        Expr::If {
            condition: condition.into(),
            then_branch: then_branch.into(),
            else_branch: else_branch.map(|e| Box::new(e)),
        }
    }
    fn while_(condition: Expr, body: Expr) -> Expr {
        Expr::While {
            condition: condition.into(),
            body: body.into(),
        }
    }
    fn break_() -> Expr {
        Expr::Break
    }
    fn continue_() -> Expr {
        Expr::Continue
    }
    fn is_number(&self) -> bool {
        matches!(self, Expr::Number { .. })
    }
    fn is_string(&self) -> bool {
        matches!(self, Expr::String { .. })
    }
    fn is_boolean(&self) -> bool {
        matches!(self, Expr::Boolean { .. })
    }
    fn is_nil(&self) -> bool {
        matches!(self, Expr::Nil)
    }
}

pub fn format_expr(expr: &Expr) -> String {
    match expr {
        Expr::Number { value } => format!("{}", value),
        Expr::String { value } => format!("{}", value),
        Expr::Boolean { value } => format!("{}", value),
        Expr::Nil => "nil".to_string(),
        Expr::List { elements } => {
            let mut result = "(".to_string();
            for (_i, element) in elements.iter().enumerate() {
                result += &format_expr(element);
            }

            result += ")";

            result
        }
        Expr::Dict { elements } => {
            let mut result = "(".to_string();
            for (_i, (key, value)) in elements.iter().enumerate() {
                result += &format_expr(key);
                result += ":";
                result += &format_expr(value);
            }
            result += ")";

            result
        }
        Expr::Tuple { elements } => {
            let mut result = "(".to_string();
            for (_i, element) in elements.iter().enumerate() {
                result += &format_expr(element);
            }

            result += ")";

            result
        }
        Expr::Variable { name } => name.clone(),
        Expr::AssignOp { op, name, expr } => {
            format!("{} {} {}", name, format_operator(&op), format_expr(expr))
        }
        Expr::Binary { op, left, right } => {
            format!(
                "({} {} {})",
                format_expr(left),
                format_operator(&op),
                format_expr(right)
            )
        }
        Expr::Unary { op, expr } => {
            format!("({} {})", format_operator(&op), format_expr(expr))
        }
        Expr::Grouping { expr } => {
            format!("(group {})", format_expr(expr))
        }
        Expr::Identifier { name } => name.clone(),
        Expr::Assign { name, expr } => {
            format!("{} = {};", name, format_expr(expr))
        }
        Expr::Call { name, args } => {
            let mut result = name.clone();
            result += "(";
            for (_i, arg) in args.iter().enumerate() {
                result += &format_expr(arg);
                result += ",";
            }
            result.pop();
            result += ")";
            result
        }
        Expr::Function { name, args, body } => {
            let mut result = "fn ".to_string() + &name.clone();
            result += "(";
            for (_i, arg) in args.iter().enumerate() {
                result += arg;
            }
            result += ") {";
            for (_i, stmt) in body.iter().enumerate() {
                result += &format_stmt(stmt);
            }
            result += "}";

            result
        }
        Expr::Return { expr } => {
            format!("return {}", format_expr(expr))
        }
        Expr::Block { body } => {
            let mut result = "{".to_string();
            for (_i, stmt) in body.iter().enumerate() {
                result += &format_stmt(stmt);
            }
            result += "}";

            result
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let mut result = "if ".to_string();
            result += &format_expr(condition);
            result += &format_stmt(then_branch);

            if let Some(else_branch) = else_branch {
                result += " else";
                result += &format_stmt(else_branch);
            }

            result
        }
        Expr::While { condition, body } => {
            let mut result = "while ".to_string();

            result += &format_expr(condition);
            result += &format_stmt(body);

            result
        }
        Expr::VarDecl { name, initializer } => {
            let mut result = "var ".to_string();

            result += name;
            result += " = ";
            result += &format_expr(initializer);

            result
        }
        Expr::Break => "break".to_string(),
        Expr::Continue => "continue".to_string(),
        Expr::Print { expr } => "print ".to_string() + &format_expr(expr),
    }
}
fn format_stmt(stmt: &Expr) -> String {
    match stmt {
        Expr::Block { body } => {
            let mut result = " {".to_string();
            for (_i, stmt) in body.iter().enumerate() {
                result += &format_stmt(stmt);
            }
            result += "}";

            result
        }
        _ => format_expr(stmt),
    }
}

fn format_operator(op: &Operator) -> &'static str {
    match op {
        Operator::Add => "+",
        Operator::Sub => "-",
        Operator::Mul => "*",
        Operator::Div => "/",
        Operator::Assign => "=",
        Operator::AddAssign => "+=",
        Operator::Not => "!",
        Operator::NotEqual => "!=",
        Operator::Equal => "==",
        Operator::Greater => ">",
        Operator::GreaterEqual => ">=",
        Operator::Less => "<",
        Operator::LessEqual => "<=",
        Operator::LogicalAnd => "&&",
        Operator::LogicalOr => "||",
        Operator::BitwiseOr => "|",
        Operator::BitwiseAnd => "&",
    }
}

#[cfg(test)]
#[path = "tests/ast/mod.rs"]
mod ast_tests;
