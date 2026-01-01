use crate::evaluate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, Value>,
    /// 上层作用域环境
    // 使用 Rc<RefCell> 允许共享和修改
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    /// 创建一个带有父级环境的新环境 (用于 Block 或 函数调用)
    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    /// 在当前环境中定义一个变量
    ///
    /// # 参数
    /// * `name` - 变量名
    /// * `value` - 变量值
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    /// 从环境中获取变量的值
    ///
    /// 按照当前作用域到父级作用域的顺序查找变量
    ///
    /// # 参数
    /// * `name` - 要获取的变量名
    ///
    /// # 返回值
    /// * `Some(Value)` - 找到的变量值
    /// * `None` - 变量不存在
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

    /// 处理变量赋值行为
    ///
    /// 在当前作用域或父级作用域中查找变量并赋值
    ///
    /// # 参数
    /// * `name` - 变量名
    /// * `value` - 要赋的值
    ///
    /// # 返回值
    /// * `true` - 赋值成功
    /// * `false` - 变量未定义
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
