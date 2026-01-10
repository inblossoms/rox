use crate::evaluate::value::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// 运行时环境 (Environment)
///
/// 用于存储变量名到变量值的映射。支持词法作用域（Lexical Scoping）。
/// 上下文环境以树状结构组织，每个环境可能有一个“封闭（Enclosing）”的父环境。
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    /// 当前作用域内定义的变量
    pub values: HashMap<String, Value>,

    /// 外层（父级）作用域的引用
    ///
    /// 使用 `Rc<RefCell<...>>` 是为了支持多重所有权和内部可变性：
    /// 1. `Rc`: 允许闭包和解释器栈同时持有同一个环境的引用。
    /// 2. `RefCell`: 允许在持有不可变引用时修改环境（eg. 定义新变量）。
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    /// 创建一个新的全局环境 (Global Environment)
    ///
    /// 全局环境没有父级作用域 (`enclosing` 为 `None`)。
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    /// 创建一个新的局部环境 (Local Environment)
    ///
    /// 新环境将被嵌套在指定的 `enclosing` 环境内部。
    /// 通常用于进入代码块 (`{ ... }`) 或函数调用时。
    ///
    /// # 参数
    /// * `enclosing` - 父级环境的强引用
    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    /// 在**当前**作用域中定义一个变量
    ///
    /// 无论父级作用域是否存在同名变量，都会在当前层级创建一个新的绑定。
    /// HashMap 特性：允许变量遮蔽 (Shadowing：同名键值会覆盖)。
    ///
    /// # 参数
    /// * `name` - 变量名
    /// * `value` - 变量的初始值
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    /// 动态查找变量值 (Dynamic Lookup)
    ///
    /// 从当前作用域开始，沿着 `enclosing` 链向上查找，直到找到变量或到达全局环境。
    ///
    /// **Note**：此方法主要用于查找全局变量，或者在没有 Resolver 优化时的查找方式。
    /// 如果 Resolver 已经计算了距离，应优先使用 `get_at`。
    ///
    /// # 参数
    /// * `name` - 要查找的变量名
    ///
    /// # 返回值
    /// * `Some(Value)` - 找到的变量值
    /// * `None` - 变量未定义（在整个链条中都找不到）
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

    /// 动态变量赋值 (Dynamic Assignment)
    ///
    /// 尝试更新一个**已存在**的变量。它会沿着作用域链向上查找。
    /// 如果当前作用域没有该变量，会尝试在父级作用域中赋值。
    ///
    /// **注意**：此方法不允许创建新变量（那是 `define` 的工作）。
    ///
    /// # 参数
    /// * `name` - 变量名
    /// * `value` - 要赋的新值
    ///
    /// # 返回值
    /// * `true` - 赋值成功（找到了变量并更新）
    /// * `false` - 赋值失败（变量未定义）
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

    // --- Resolver 静态解析支持 ---

    /// 在指定的距离（深度）获取变量值 (Static Lookup)
    ///
    /// 利用 Resolver 计算出的 `distance` (跳跃步数)，直接定位到定义该变量的环境，
    /// 从而绕过动态查找，解决“闭包捕获环境”和“变量遮蔽”的歧义问题。
    ///
    /// # 参数
    /// * `distance` - 变量定义距离当前环境的层数（0 表示当前环境）
    /// * `name` - 变量名
    ///
    /// # Panics
    /// 如果 `distance` 大于实际环境链的深度，或者在目标环境找不到变量，
    /// 代码会 Panic。理论上不会发生，因为 Resolver 阶段已经保证了变量的存在性。
    pub fn get_at(&self, distance: usize, name: &str) -> Option<Value> {
        if distance == 0 {
            // 就在当前环境
            return self.values.get(name).cloned();
        }
        // 递归去父环境找 (distance - 1)
        // unwrap 是安全的，因为 Resolver 保证了 distance 是有效的，且父环境一定存在
        self.enclosing
            .as_ref()
            .unwrap()
            .borrow()
            .get_at(distance - 1, name)
    }

    /// 在指定的距离（深度）赋值 (Static Assignment)
    ///
    /// 利用 Resolver 计算出的 `distance`，直接在特定层级的环境中更新变量。
    ///
    /// # 参数
    /// * `distance` - 目标环境距离当前环境的层数
    /// * `name` - 变量名
    /// * `value` - 新值
    pub fn assign_at(&mut self, distance: usize, name: &str, value: Value) {
        if distance == 0 {
            self.values.insert(name.to_string(), value);
            return;
        }
        // 递归向上传递赋值操作
        self.enclosing
            .as_ref()
            .unwrap()
            .borrow_mut()
            .assign_at(distance - 1, name, value);
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
