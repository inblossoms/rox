# Rox - A Robust Interpreter in Rust

[![CI](https://github.com/inblossoms/rox/actions/workflows/ci.yml/badge.svg)](https://github.com/inblossoms/rox/actions/workflows/ci.yml)
![Language](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

**Rox** is a strongly-typed, tree-walk interpreter, implemented in Rust.

Based on the excellent book [_Crafting Interpreters_](https://craftinginterpreters.com/) by Bob Nystrom, this implementation diverges to Rust patterns, focusing on memory safety, strict type checking, and a robust error handling architecture.

## ✨ Key Features

-  **Tree-Walk Architecture:** Implements a complete recursive descent parser and a direct AST evaluator.
-  **Object-Oriented:** Full support for Classes, Inheritance, Methods, Initializers, and super calls.
-  **Lexical Scoping & Closures:** Robust environment management allowing for first-class functions and closures.
-  **Strict Type System:** Rox adopts a Rust-like philosophy, rejecting implicit type coercions (e.g., "1" + 1 raises a runtime error).
-  **Control Flow:** Supports if-else, while, and for loops, along with semantic checks for break, continue, and return.
-  **Extended Operations:** support for bitwise arithmetic and compound assignment operators.

## 🛠 Architecture & Design

Rox implements a standard compiler pipeline with a focus on separation of concerns:

1. **Tokenizer (Lexer)**: Converts raw source code into a stream of tokens. Handles comments and line tracking.
2. **Parser**: A recursive descent parser that produces an Abstract Syntax Tree (AST).
   -  _Design Choice_: Separation of `Expr` (Expressions producing values) and `Stmt` (Statements performing actions) for type safety.
   -  _Fail-Fast_: Static checks for loop boundaries (preventing `break` outside loops) are handled here.
3. **Resolver (Semantic Analysis)**: A static analysis pass that resolves variable scope distances.
   -  Solves the "funarg problem" for closures.
   -  Performs static checks for `return` locations, `this` usage, and variable shadowing rules.
4. **Interpreter**: The runtime engine.
   -  **Environment**: Uses `Rc<RefCell<Environment>>` to manage the scope chain, allowing efficient memory sharing for closures without a garbage collector.
   -  **Side Tables**: Utilizes resolution data to perform direct variable lookups (hopping scopes) rather than dynamic searches.

## 🚀 Getting Started

### Prerequisites

-  [Rust Toolchain](https://www.rust-lang.org/tools/install) (cargo 1.x+)

### Installation

Clone the repository and build the project:

```bash
git clone https://github.com/inblossoms/rox.git
cd rox
cargo build --release
```

### Usage

**1. REPL Mode (Interactive)**
Run without arguments to start the interactive shell:

```bash
cargo run
```

```javascript
> var a = "Hello";
> print a + " World";
Hello World
```

**2. Script Mode**
Run a `.rox` file:

```bash
cargo run -- scripts/fibonacci.rox
```

## 📝 Syntax Examples

**Classes and Inheritance:**

```javascript
class Doughnut {
  cook() {
    print "Fry until golden brown.";
  }
}

class BostonCream < Doughnut {
  cook() {
    super.cook();
    print "Pipe full of custard.";
  }
}

BostonCream().cook();
```

**Closures:**

```javascript
fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    print i;
  }
  return count;
}

var counter = makeCounter();
counter(); // "1"
counter(); // "2"
```

**Bitwise Operations & Control Flow:**

```javascript
var flags = 5; // 0101
var mask = 1;  // 0001

if ((flags & mask) == 1) {
    print "Bit is set!";
}

for (var i = 0; i < 10; i += 1) {
    if (i % 2 == 0) continue;
    print i;
}
```

## 🧪 Testing

The project includes a comprehensive test suite covering AST structure, parser logic, and runtime evaluation (including scope resolution).

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test parser
cargo test evaluate
```

## 📂 Project Structure

```text
src/
├── reader/         // Source file reading
├── ast/            // AST definitions (Expr/Stmt)
├── tokenizer/      // Lexical analysis
├── parser/         // Parsing logic & ParseHelper
├── resolver/       // Semantic analysis & Variable resolution
├── evaluate/       // Runtime execution
├── error.rs        // Unified error handling
└── main.rs         // Entry point (CLI)
```

## 🤝 Contributing

Contributions are welcome! If you are interested in fixing a bug or adding a new feature, please read our [CONTRIBUTING](CONTRIBUTING.md) | [贡献](CONTRIBUTING_zh.md) guide first to set up your development environment.

## ⚖️ License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

_Built with ❤️ in Rust._
