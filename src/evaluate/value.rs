use crate::{ast::Stmt, evaluate::environment::Environment};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

// 类 (Class) 运行时结构
#[derive(Debug, Clone, PartialEq)]
pub struct RoxClass {
    pub name: String,
    pub methods: HashMap<String, Value>,
    /// 存储父类，以便查找方法时进行回溯
    pub superclass: Option<Rc<RefCell<RoxClass>>>,
}

impl RoxClass {
    pub fn new(
        name: String,
        methods: HashMap<String, Value>,
        superclass: Option<Rc<RefCell<RoxClass>>>,
    ) -> Self {
        Self {
            name,
            methods,
            superclass,
        }
    }

    /// 查找方法（支持继承）
    // 如果当前类找不到，递归去父类找
    pub fn find_method(&self, name: &str) -> Option<Value> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        }

        if let Some(superclass) = &self.superclass {
            return superclass.borrow().find_method(name);
        }

        None
    }
}

// 类实例 (Instance) 运行时结构
#[derive(Debug, Clone, PartialEq)]
pub struct RoxInstance {
    // 实例必须持有它所属的类
    pub class: Rc<RefCell<RoxClass>>,
    // 字段表
    pub fields: RefCell<HashMap<String, Value>>,
}

impl RoxInstance {
    pub fn new(class: Rc<RefCell<RoxClass>>) -> Self {
        Self {
            class,
            fields: RefCell::new(HashMap::new()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,

    Function {
        name: String,
        args: Vec<String>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },

    Class(Rc<RefCell<RoxClass>>),
    Instance(Rc<RefCell<RoxInstance>>),

    List(Vec<Value>),
    Tuple(Vec<Value>),
    Dict(HashMap<String, Value>),
    Print(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Function { name, .. } => write!(f, "<fn {}>", name),
            Value::Class(class) => write!(f, "<class {}>", class.borrow().name),
            Value::Instance(instance) => {
                write!(f, "<instance {}>", instance.borrow().class.borrow().name)
            }
            Value::List(list) => write!(
                f,
                "[{}]",
                list.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Tuple(tuple) => write!(
                f,
                "({})",
                tuple
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Dict(dict) => write!(
                f,
                "{{{}}}",
                dict.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Print(print) => write!(f, "{}", print),
        }
    }
}

impl Value {
    /// 判断值是否为真值（Truthy）
    ///
    /// # 返回值
    /// * `bool` - 值是否为真值
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            Value::String(s) => !s.is_empty(),
            Value::Number(n) => *n != 0.0,
            _ => true,
        }
    }

    /// 获取值的类型名称
    ///
    /// # 返回值
    /// * `&'static str` - 类型名称字符串
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "Number",
            Value::String(_) => "String",
            Value::Boolean(_) => "Boolean",
            Value::Nil => "Nil",
            Value::Function { .. } => "Function",
            Value::Class(_) => "Class",
            Value::Instance(_) => "Instance",
            Value::List(_) => "List",
            Value::Dict(_) => "Dict",
            Value::Tuple(_) => "Tuple",
            Value::Print(_) => "Print",
        }
    }

    /// 将方法绑定到实例上
    ///
    /// 创建一个新的函数环境，其中 "this" 绑定到给定的 instance。
    pub fn bind(&self, instance: Value) -> Value {
        match self {
            Value::Function {
                name,
                args,
                body,
                closure,
            } => {
                // 创建新环境，父环境是原函数的闭包
                let environment =
                    Rc::new(RefCell::new(Environment::with_enclosing(closure.clone())));

                // 在新环境中定义 "this"
                environment
                    .borrow_mut()
                    .define("this".to_string(), instance);

                // 返回新的 Function，闭包指向包含 "this" 的环境
                Value::Function {
                    name: name.clone(),
                    args: args.clone(),
                    body: body.clone(),
                    closure: environment,
                }
            }

            _ => panic!("Only functions can be bound"),
        }
    }
}
