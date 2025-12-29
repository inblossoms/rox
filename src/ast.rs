#[derive(Debug)]
pub struct AST {
    pub top: Option<Expr>,
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
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
    Or,
    And,
}

#[derive(Debug)]
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

        Expr::Break => "break".to_string(),
        Expr::Continue => "continue".to_string(),
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
        Operator::And => "&&",
        Operator::Or => "||",
    }
}

pub fn main() {
    let expression = Expr::binary(
        Operator::Mul,
        Expr::unary(Operator::Sub, Expr::number("123")),
        Expr::grouping(Expr::number("234")),
    );

    println!("{}", format_expr(&expression));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_expressions() {
        assert_eq!(format_expr(&Expr::number("42")), "42");

        assert_eq!(format_expr(&Expr::string("hello")), "hello");

        assert_eq!(format_expr(&Expr::boolean(true)), "true");
        assert_eq!(format_expr(&Expr::boolean(false)), "false");

        assert_eq!(format_expr(&Expr::nil()), "nil");
    }

    #[test]
    fn test_compound_structures() {
        assert_eq!(format_expr(&Expr::list(vec![])), "()");

        let list_expr = Expr::list(vec![
            Expr::number("1"),
            Expr::string("two"),
            Expr::boolean(true),
        ]);
        assert_eq!(format_expr(&list_expr), "(1twotrue)");

        assert_eq!(format_expr(&Expr::dict(vec![])), "()");

        let dict_expr = Expr::dict(vec![
            (Expr::string("key1"), Expr::number("100")),
            (Expr::string("key2"), Expr::boolean(false)),
        ]);
        assert_eq!(format_expr(&dict_expr), "(key1:100key2:false)");

        let tuple_expr = Expr::tuple(vec![Expr::number("10"), Expr::string("tuple")]);
        assert_eq!(format_expr(&tuple_expr), "(10tuple)");
    }

    #[test]
    fn test_operator_expressions() {
        let unary_expr = Expr::unary(Operator::Not, Expr::boolean(true));
        assert_eq!(format_expr(&unary_expr), "(! true)");

        let binary_expr = Expr::binary(Operator::Add, Expr::number("5"), Expr::number("7"));
        assert_eq!(format_expr(&binary_expr), "(5 + 7)");

        // 分组表达式
        let grouping_expr = Expr::grouping(Expr::number("42"));
        assert_eq!(format_expr(&grouping_expr), "(group 42)");

        // 嵌套操作
        let nested_expr = Expr::binary(
            Operator::Mul,
            Expr::unary(Operator::Sub, Expr::number("123")),
            Expr::grouping(Expr::number("234")),
        );
        assert_eq!(format_expr(&nested_expr), "((- 123) * (group 234))");
    }

    #[test]
    fn test_variable_assignments() {
        assert_eq!(format_expr(&Expr::variable("x".to_string())), "x");

        let assign_expr = Expr::assign("x".to_string(), Expr::number("10"));
        assert_eq!(format_expr(&assign_expr), "x = 10;");

        let assign_op_expr =
            Expr::assign_op(Operator::AddAssign, "y".to_string(), Expr::number("5"));
        assert_eq!(format_expr(&assign_op_expr), "y += 5");
    }

    #[test]
    fn test_control_flow() {
        let if_expr = Expr::if_(
            Expr::binary(Operator::Greater, Expr::number("10"), Expr::number("5")),
            Expr::block(vec![Expr::number("1")]),
            Some(Expr::block(vec![Expr::number("0")])),
        );
        assert_eq!(format_expr(&if_expr), "if (10 > 5) {1} else {0}");

        let if_no_else_expr = Expr::if_(
            Expr::boolean(true),
            Expr::block(vec![Expr::string("yes")]),
            None,
        );
        assert_eq!(format_expr(&if_no_else_expr), "if true {yes}");

        let while_expr = Expr::while_(
            Expr::binary(
                Operator::Less,
                Expr::variable("i".to_string()),
                Expr::number("10"),
            ),
            Expr::block(vec![Expr::assign_op(
                Operator::AddAssign,
                "i".to_string(),
                Expr::number("1"),
            )]),
        );
        assert_eq!(format_expr(&while_expr), "while (i < 10) {i += 1}");

        // 代码块
        let block_expr = Expr::block(vec![
            Expr::assign("x".to_string(), Expr::number("1")),
            Expr::assign("y".to_string(), Expr::number("2")),
            Expr::binary(
                Operator::Add,
                Expr::variable("x".to_string()),
                Expr::variable("y".to_string()),
            ),
        ]);
        assert_eq!(format_expr(&block_expr), "{x = 1;y = 2;(x + y)}");
    }

    #[test]
    fn test_function_expressions() {
        let func_expr = Expr::function(
            "add".to_string(),
            vec!["a".to_string(), ",".to_string(), "b".to_string()],
            vec![Expr::return_(Expr::binary(
                Operator::Add,
                Expr::variable("a".to_string()),
                Expr::variable("b".to_string()),
            ))],
        );
        assert_eq!(format_expr(&func_expr), "fn add(a,b) {return (a + b)}");

        let call_expr = Expr::call(
            "add".to_string(),
            vec![Expr::number("3"), Expr::number("4")],
        );
        assert_eq!(format_expr(&call_expr), "add(3,4)");

        let return_expr = Expr::return_(Expr::number("42"));
        assert_eq!(format_expr(&return_expr), "return 42");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(format_expr(&Expr::block(vec![])), "{}");

        // 深度嵌套表达式
        let nested_expr = Expr::binary(
            Operator::Mul,
            Expr::binary(Operator::Add, Expr::number("1"), Expr::number("2")),
            Expr::binary(Operator::Sub, Expr::number("3"), Expr::number("4")),
        );
        assert_eq!(format_expr(&nested_expr), "((1 + 2) * (3 - 4))");

        // 包含特殊字符的字符串
        assert_eq!(format_expr(&Expr::string("hello\"world")), "hello\"world");

        let long_list = Expr::list((0..10).map(|i| Expr::number(&i.to_string())).collect());
        assert_eq!(format_expr(&long_list), "(0123456789)");
    }
}
