#![allow(dead_code)]
use super::expr::Expr;

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
        Expr::For {
            initializer,
            condition,
            increment,
            body,
        } => {
            todo!()
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

fn format_operator(op: &super::Operator) -> &'static str {
    match op {
        super::Operator::Add => "+",
        super::Operator::Sub => "-",
        super::Operator::Mul => "*",
        super::Operator::Div => "/",
        super::Operator::Assign => "=",
        super::Operator::AddAssign => "+=",
        super::Operator::Not => "!",
        super::Operator::NotEqual => "!=",
        super::Operator::Equal => "==",
        super::Operator::Greater => ">",
        super::Operator::GreaterEqual => ">=",
        super::Operator::Less => "<",
        super::Operator::LessEqual => "<=",
        super::Operator::LogicalAnd => "&&",
        super::Operator::LogicalOr => "||",
        super::Operator::BitwiseOr => "|",
        super::Operator::BitwiseAnd => "&",
    }
}
