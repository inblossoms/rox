#![allow(dead_code)]
use super::Operator;
use crate::ast::{Expr, Stmt};

/// 格式化表达式 (返回值)
pub fn format_expr(expr: &Expr) -> String {
    match expr {
        Expr::Number { value } => value.clone(), // number: String, not f64
        Expr::String { value } => format!("\"{}\"", value),
        Expr::Boolean { value } => value.to_string(),
        Expr::Nil => "nil".to_string(), // 做为语言类型系统的一部分，可以被赋值给变量、作为参数传递、被打印。不同于 Stmt::Nil 后者是语法结构，用于满足语法要求，但什么都不做。

        Expr::List { elements } => {
            let elems: Vec<String> = elements.iter().map(format_expr).collect();
            format!("[{}]", elems.join(", "))
        }
        Expr::Dict { elements } => {
            let elems: Vec<String> = elements
                .iter()
                .map(|(k, v)| format!("{}: {}", format_expr(k), format_expr(v)))
                .collect();
            format!("{{{}}}", elems.join(", "))
        }
        Expr::Tuple { elements } => {
            let elems: Vec<String> = elements.iter().map(format_expr).collect();
            format!("({})", elems.join(", "))
        }

        Expr::Variable { name, .. } => name.lexeme.clone(), // 忽略 id

        Expr::Assign { name, expr, .. } => {
            format!("{} = {}", name.lexeme, format_expr(expr))
        }

        Expr::AssignOp { op, name, expr, .. } => {
            format!(
                "{} {} {}",
                name.lexeme,
                format_operator(op),
                format_expr(expr)
            )
        }

        Expr::Binary { op, left, right } => {
            format!(
                "({} {} {})",
                format_expr(left),
                format_operator(op),
                format_expr(right)
            )
        }

        Expr::Logical { op, left, right } => {
            format!(
                "({} {} {})",
                format_expr(left),
                format_operator(op),
                format_expr(right)
            )
        }

        Expr::Unary { op, expr } => {
            format!("({}{})", format_operator(op), format_expr(expr))
        }

        Expr::Grouping { expr } => {
            format!("(group {})", format_expr(expr))
        }

        Expr::Call { callee, args, .. } => {
            let args_str: Vec<String> = args.iter().map(format_expr).collect();
            format!("{}({})", format_expr(callee), args_str.join(", "))
        }

        Expr::Get { object, name, .. } => {
            format!("{}.{}", format_expr(object), name.lexeme)
        }
        Expr::Set {
            object,
            name,
            value,
            ..
        } => {
            format!(
                "{}.{} = {}",
                format_expr(object),
                name.lexeme,
                format_expr(value)
            )
        }

        Expr::This { .. } => "this".to_string(),
    }
}

/// 格式化语句 (执行动作)
pub fn format_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Expression { expr } => {
            format!("{};", format_expr(expr))
        }
        Stmt::VarDecl { name, initializer } => match initializer {
            Some(expr) => format!("var {} = {};", name.lexeme, format_expr(expr)),
            None => format!("var {};", name.lexeme),
        },
        Stmt::Function { name, params, body } => {
            let params_str: Vec<String> = params.iter().map(|t| t.lexeme.clone()).collect();

            let body_str = body
                .iter()
                .map(|s| format_stmt(s))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "fun {}({}) {{ {} }}",
                name.lexeme,
                params_str.join(", "),
                body_str
            )
        }
        Stmt::Class { name, methods } => {
            format!(
                "class {} {{ {} }}",
                name.lexeme,
                methods
                    .iter()
                    .map(|m| format_stmt(m))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
        Stmt::Block { body } => {
            let parts: Vec<String> = body.iter().map(format_stmt).collect();

            format!("{{ {} }}", parts.join(" "))
        }
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let mut result = format!(
                "if ({}) {}",
                format_expr(condition),
                format_stmt(then_branch)
            );
            if let Some(else_b) = else_branch {
                result += &format!(" else {}", format_stmt(else_b));
            }
            result
        }
        Stmt::While { condition, body } => {
            format!("while ({}) {}", format_expr(condition), format_stmt(body))
        }
        Stmt::For {
            initializer,
            condition,
            increment,
            body,
        } => {
            let init_str = match initializer {
                Some(stmt) => format_stmt(stmt), // Stmt 自带分号 (VarDecl 或 ExprStmt)
                None => ";".to_string(),
            };
            let cond_str = condition
                .as_ref()
                .map(|c| format_expr(c))
                .unwrap_or("".to_string());
            let incr_str = increment
                .as_ref()
                .map(|i| format_expr(i))
                .unwrap_or("".to_string());

            format!(
                "for ({} {}; {}) {}",
                init_str,
                cond_str,
                incr_str,
                format_stmt(body)
            )
        }
        Stmt::Print { expr } => {
            format!("print {};", format_expr(expr))
        }
        Stmt::Return { value, .. } => match value {
            Some(expr) => format!("return {};", format_expr(expr)),
            None => "return;".to_string(),
        },
        Stmt::Break => "break;".to_string(),
        Stmt::Continue => "continue;".to_string(),
        Stmt::Empty => ";".to_string(),
    }
}

/// 格式化操作符
pub fn format_operator(op: &Operator) -> &'static str {
    match op {
        Operator::Add => "+",
        Operator::Sub => "-",
        Operator::Mul => "*",
        Operator::Div => "/",
        Operator::Mod => "%",
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
        Operator::AndKeyword => "&&",
        Operator::OrKeyword => "||",
        Operator::BitwiseXor => "^",
        Operator::BitwiseOr => "|",
        Operator::BitwiseAnd => "&",
        Operator::BitwiseNot => "~",
    }
}
