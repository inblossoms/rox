use crate::evaluate::interpreter::Interpreter;
use std::collections::HashMap;

/// 函数上下文
///
/// 用于在静态分析阶段追踪当前代码是否处于函数定义内部。
/// 状态标记：用于验证 `return` 语句的合法性。
#[derive(Copy, Clone, PartialEq)]
pub enum FunctionType {
    /// 当前不在任何函数内（顶层代码），此时使用 `return` 是非法的。
    None,
    /// 当前处于函数体内，此时可以使用 `return`。
    Function,
    /// 类方法：类方法定义内部，`this` 引用的变量为类的实例变量。
    /// 行为和普通 Function 一样，允许 return value。
    Method,
    /// init: 类的构造函数
    /// 允许写 return; (用于提前结束构造)，但严禁 return value;，(Note: 构造函数永远只会返回 this)
    Initializer,
}

/// 循环上下文
///
/// 用于在静态分析阶段追踪当前代码是否处于循环 (`while`, `for`) 内部。
/// 状态标记：用于验证控制流语句的合法性。
#[derive(Copy, Clone, PartialEq)]
pub enum LoopType {
    /// 当前不在循环内，此时使用 `break` 或 `continue` 是非法的。
    None,
    /// 当前处于循环体内，此时可以使用 `break` 或 `continue`。
    Loop,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ClassType {
    None,
    Class,
    /// 子类状态
    Subclass,
}

/// 语义分析器 (Resolver)
///
/// `Resolver` 是解释器工作流中的关键步骤之一，位于 Parser 和 Interpreter 阶段之间。
/// 用于遍历 AST（Visitor），但**不执行**代码。
///
/// # 核心职责
/// 1. **变量绑定解析 (Variable Resolution)**：
///    计算每个局部变量引用（`Variable` 或 `Assign` 表达式）距离其声明所在的
///    作用域深度（Distance/Hops）。结果存储在 `Interpreter` side table (`locals`) 中。
///    - 使得闭包可以正确捕获环境变量，并解决动态作用域的潜在问题。
/// 2. **静态语义检查 (Static Analysis)**：
///    在运行前检测特定的语法错误，例如：
///    - 在顶层使用 `return`。
///    - 在循环外使用 `break` 或 `continue`。
///    - 在变量初始化完成前读取自身 (`var a = a;`)。
///    - 在同一作用域重复声明变量。
pub struct Resolver<'a> {
    /// 解释器的可变引用
    ///
    /// Resolver 计算出的变量深度信息（Scope Distance）需要写入 Interpreter 中的
    /// `locals` (Side Table)，以便运行时查找。
    pub interpreter: &'a mut Interpreter,

    /// 词法作用域栈
    ///
    /// 用于模拟解释器运行时的环境链，但只关注变量的存在性和初始化状态，不关注具体值。
    /// * **Stack**: `Vec` 的末尾代表当前最内层的作用域。
    /// * **Map Key**: 变量名 (`String`)。
    /// * **Map Value**: 初始化状态 (`bool`)。
    ///   - `false`: **已声明但未定义 (Declared)**。变量名已进入作用域，但初始化表达式尚未解析完毕。
    ///              用于检测 `var a = a;` 这种错误。
    ///   - `true`: **已定义 (Defined)**。变量已初始化完毕，可以安全使用。
    pub scopes: Vec<HashMap<String, bool>>,

    /// 当前函数上下文状态
    ///
    /// 用于检查 `return` 语句是否出现在合法的位置。
    /// 每当进入函数定义时，保存旧状态并设置为 `FunctionType::Function`；退出时恢复。
    pub current_function: FunctionType,

    pub current_class: ClassType,

    /// 当前循环上下文状态
    ///
    /// 用于检查 `break` 和 `continue` 语句是否出现在合法的位置。
    /// 每当进入循环语句时，保存旧状态并设置为 `LoopType::Loop`；退出时恢复。
    pub current_loop: LoopType,
}
