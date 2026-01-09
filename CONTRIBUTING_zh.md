# Contributing to Rox

<a href="./CONTRIBUTING.md">[🇬🇧 English Guidelines](#CONTRIBUTING.md)</a>

---

<a name="中文指南"></a>

## 🇨🇳 中文指南

感谢你有兴趣为 Rox 做出贡献！🎉

本文档旨在指导你配置开发环境、编写测试以及发起 Pull Request。

### 🛠️ 开发环境配置

本项目使用严格的静态代码分析工具。在提交代码前，请务必根据您的操作系统初始化开发环境。

**macOS / Linux:**

```bash
# 添加执行权限（仅需一次）
chmod +x setup_dev.sh

# 运行脚本
./setup_dev.sh
```

**Windows (PowerShell):**

```powershell
.\setup_dev.ps1
```

> **提示**: 如果 PowerShell 提示“禁止运行脚本”，请先以管理员身份运行以下命令开启权限，然后重试：
> `Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser`

此脚本将安装：`rustfmt`, `clippy`, `cargo-deny`, `cargo-nextest`, `typos`, 以及 `pre-commit` hooks。

### 🧪 测试准则 (必须遵守)

**原则：无测试，不合并。**

为了保持解释器的稳定性，每一次代码修改都必须包含必要的功能测试。

1. **新功能**: 如果你添加了新特性（例如 `do-while` 循环），必须在 `src/evaluate/tests.rs`（或 Parser 测试）中添加对应的 Rox 代码测试用例。
2. **Bug 修复**: 如果你修复了一个 Bug，必须添加一个 **回归测试**。该测试在修复前应失败，在修复后应通过。
3. **重构**: 必须确保现有的所有测试全部通过。

**如何编写功能测试：**
我们通常通过执行 Rox 源码并检查全局变量 `res` 的值来验证。示例：

```rust
#[test]
fn test_my_new_feature() {
    let code = r#"
        // 在这里写 Rox 代码
        var a = 1;
        var res = a + 1;
    "#;
    // 验证结果
    assert_eq!(eval_res(code).unwrap(), Value::Number(2.0));
}
```

### 🌊 工作流

1. **Fork** 本仓库到你的账号。
2. **Clone** 你的 Fork 到本地。
3. 创建分支：`git checkout -b feat/my-cool-feature`。
4. **编写代码并添加测试**。
5. **本地验证**：
   -  手动运行测试：`cargo nextest run`
   -  当你执行 `git commit` 时，`pre-commit` 会自动运行格式化和检查。

### 📝 Commit 规范

本项目使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范，这对于自动化生成 [CHANGELOG](CHANGES.md) 至关重要。

**格式**： `<type>(<scope>): <description>`

**常用类型**：

-  `feat`: 新功能 (Features)
-  `fix`: 修补 Bug (Bug Fixes)
-  `docs`: 文档修改 (Documentation)
-  `style`: 代码格式修改，不影响逻辑 (Style)
-  `refactor`: 代码重构 (Refactoring)
-  `test`: 测试代码修改 (Tests)
-  `chore`: 构建过程或辅助工具的变动

**示例**：

-  ✅ `feat(parser): add support for 'do-while' loops`
-  ✅ `fix(resolver): fix variable shadowing bug in blocks`
-  ❌ `update code` (描述不清，会被 CI 拦截或归类为 Other)

---

## ⚖️ License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
