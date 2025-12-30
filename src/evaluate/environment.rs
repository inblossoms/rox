use crate::evaluate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, Value>,
    // 父级环境，使用 Rc<RefCell> 允许共享和修改
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    // 创建一个带有父级环境的新环境 (用于 Block 或 函数调用)
    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    // 定义变量 (var x = 1;)
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    // 获取变量 (x)
    pub fn get(&self, name: &str) -> Option<Value> {
        // 1. 先查当前作用域
        if let Some(value) = self.values.get(name) {
            return Some(value.clone());
        }

        // 2. 递归查父级作用域
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }

        None
    }

    // 赋值变量 (x = 2;)
    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        // 1. 如果变量在当前作用域存在，更新它
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            return true;
        }

        // 2. 否则去父级找
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }

        false // 变量未定义
    }
}
