use crate::{
    ast::Stmt,
    evaluate::{Interpreter, environment::Environment, error::RuntimeError},
};
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

// 模块的内部结构
#[derive(Debug, Clone, PartialEq)]
pub struct RoxModule {
    pub name: String,
    pub exports: HashMap<String, Value>, // 导出表
    /// 状态标记：区分由模块未初始化完成导致的变量未初始化或者变量不存在
    pub is_initialized: bool,
}

impl RoxModule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            exports: HashMap::new(),
            is_initialized: false, // 默认为 false
        }
    }
}

/// 生函数类型别名
/// 接收解释器引用(为了访问环境或报错)和参数列表
pub type NativeFn = fn(&mut Interpreter, Vec<Value>) -> Result<Value, RuntimeError>;

#[allow(unpredictable_function_pointer_comparisons, dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    None,

    Function {
        name: String,
        args: Vec<String>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    },

    Class(Rc<RefCell<RoxClass>>),
    Instance(Rc<RefCell<RoxInstance>>),

    List(Rc<RefCell<Vec<Value>>>),
    Tuple(Vec<Value>),
    Dict(Rc<RefCell<HashMap<String, Value>>>),
    Print(String),

    // 原生方法
    NativeFunction {
        name: String, // 函数的名 用于错误显示
        arity: usize, // 函数的参数个数
        func: NativeFn,
    },

    // 用于原生方法绑定 (类似于 obj.method)
    // 当在 List/String 上调用 Get 时，生成这个值。
    BoundNativeMethod {
        receiver: Box<Value>, // 'this' 对象 (eg. 那个 List 实例) this 指向的数据
        method: Box<Value>,   // NativeFunction 本身，要执行的函数
    },

    // 模块化
    Module(Rc<RefCell<RoxModule>>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::None => write!(f, "none"),
            Value::Nil => write!(f, "nil"),
            Value::Function { name, .. } => write!(f, "<fn {}>", name),
            Value::Class(class) => write!(f, "<class {}>", class.borrow().name),
            Value::Instance(instance) => {
                write!(f, "<instance {}>", instance.borrow().class.borrow().name)
            }
            Value::NativeFunction { name, .. } => write!(f, "<native fn {}>", name),
            Value::BoundNativeMethod { method, .. } => write!(f, "{}", method), // 委托给内部的 NativeFunction 打印
            Value::List(list) => write!(
                f,
                "[{}]",
                list.borrow()
                    .iter()
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
                dict.borrow()
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Print(print) => write!(f, "{}", print),
            Value::Module(m) => write!(f, "<module '{}'>", m.borrow().name),
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
            Value::None => "None",
            Value::Nil => "Nil",
            Value::Function { .. } => "Function",
            Value::Class(_) => "Class",
            Value::Instance(_) => "Instance",
            Value::List(_) => "List",
            Value::Dict(_) => "Dict",
            Value::Tuple(_) => "Tuple",
            Value::Print(_) => "Print",
            Value::Module { .. } => "Module",
            Value::NativeFunction { .. } => "NativeFunction",
            Value::BoundNativeMethod { .. } => "BoundNativeMethod",
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
