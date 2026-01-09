# Contributing to Rox

<a name="english-guidelines"></a>

## 🇬🇧 English Guidelines

Thank you for your interest in contributing to Rox! 🎉

This document guides you through setting up your development environment, writing tests, and submitting Pull Requests.

### 🛠️ Development Setup

This project uses strict static analysis tools (Clippy, Rustfmt) and Git Hooks. **Before writing any code**, please initialize your environment based on your operating system.

**macOS / Linux:**

```bash
# Add execution permission (only once)
chmod +x setup_dev.sh

# Run the setup script
./setup_dev.sh
```

**Windows (PowerShell):**

```powershell
.\setup_dev.ps1
```

> **Note**: If PowerShell blocks the script execution, run the following command as Administrator and try again:
> `Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser`

These scripts install: `rustfmt`, `clippy`, `cargo-deny`, `cargo-nextest`, `typos`, and `pre-commit` hooks.

### 🧪 Testing Policy (Mandatory)

**Strict Rule: No Tests, No Merge.**

To maintain the stability of the interpreter, every code change must include corresponding functional tests.

1. **New Features**: If you implement a new syntax or feature (e.g., `do-while`), you must add test cases in `src/evaluate/tests.rs` (or relevant parser tests) verifying it works as expected.
2. **Bug Fixes**: If you fix a bug, you must add a **regression test** that reproduces the bug and proves it is fixed.
3. **Refactoring**: Ensure all existing tests pass.

**How to write a functional test:**
We use integration tests to evaluate Rox source code directly. Example in `src/evaluate/tests.rs`:

```rust
#[test]
fn test_my_new_feature() {
    let code = r#"
        // Your Rox code here
        var a = 1;
        var res = a + 1;
    "#;
    // Verify the value of global variable 'res'
    assert_eq!(eval_res(code).unwrap(), Value::Number(2.0));
}
```

### 🌊 Workflow

1. **Fork** the repository.
2. **Clone** your fork locally.
3. Create a branch: `git checkout -b feat/my-cool-feature`.
4. **Write Code & Tests**.
5. **Local Verification**:
   -  Run tests: `cargo nextest run`
   -  The `pre-commit` hook will automatically run formatting and linting when you commit.

### 📝 Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/). This is **required** for generating the [CHANGELOG](CHANGES.md).

**Format**: `<type>(<scope>): <description>`

-  `feat`: New features
-  `fix`: Bug fixes
-  `docs`: Documentation changes
-  `style`: Formatting, missing semi colons, etc.
-  `refactor`: Code refactoring
-  `test`: Adding missing tests
-  `chore`: Build process or aux tool changes

**Examples**:

-  ✅ `feat(parser): add support for 'do-while' loops`
-  ✅ `fix(resolver): fix variable shadowing bug in blocks`
-  ❌ `update code` (Will be rejected by CI or ignored in Changelog)

---

## ⚖️ License

By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).
