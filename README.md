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

## ⚡ Language Highlights

Rox extends the Lox language with modern features, making it a capable scripting language.

### 1. Robust Type System & Collections

Supports **Lists**, **Dicts**, and **Tuples** with native method chaining. Rox is strongly typed (no implicit type coercion failures).

```javascript
var data = [1, 2, 3];
var map = {"a": 10, "b": 20};

// Native methods with Lambdas
var squared = data.map(fun(x) { return x * x; });
print "Result: " + squared; // [1, 4, 9]
```

### 2. Full Object-Oriented Programming

Complete support for **Classes**, **Inheritance**, **Mixins** (via closures), and **Static Analysis** for `this`/`super`.

```javascript
class Shape {
    init(name) { this.name = name; }
    area() { return 0; }
}

class Circle < Shape {
    init(r) {
        super.init("Circle");
        this.r = r;
    }
    area() { return math.PI * this.r * this.r; }
}

print Circle(5).area();
```

### 3. Modular System

Build complex applications with **File-based Modules**. Features isolated environments, caching, and cycle detection.

```javascript
// math_lib.rox
export var PI = 3.14159;
export fun add(a, b) { return a + b; }

// main.rox
var m = import("./math_lib.rox");
print m.add(10, 5);
```

### 4. Safety & Control Flow

Includes **Try-Catch** for error handling and standard loop controls (`break`/`continue`).

```javascript
try {
    var file = fs.readFile("missing.txt");
} catch (e) {
    print "Error handled: " + e;
}

for (var i = 0; i < 10; i += 1) {
    if (i % 2 == 0) continue;
    print i;
}
```

### 5. Built-in Standard Library

Everything you need to get started.

-  **Math**: `sin`, `cos`, `sqrt`, `pow`, `abs`, etc.
-  **IO/FS**: `input()`, `clock()`, `fs.readFile`, `fs.writeFile`.
-  **Core**: String manipulation (`len`, `upper`), List operations (`push`, `pop`, `join`), Dict access.

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

### 1. Functional Programming with Collections

Rox supports anonymous functions (lambdas) and native list operations.

```javascript
var numbers = [1, 2, 3, 4, 5];

// Use map with a lambda function
var squared = numbers.map(fun(n) {
    return n * n;
});

// Use filter
var evens = squared.filter(fun(n) {
    return n % 2 == 0;
});

print evens; // Output: [4, 16]
```

### 2. Object-Oriented Programming

Support for classes, inheritance, `this`, and `super`.

```javascript
class Shape {
    init(name) {
        this.name = name;
    }

    area() {
        return 0;
    }

    describe() {
        print "I am a " + this.name + " with area " + this.area();
    }
}

class Circle < Shape {
    init(radius) {
        super.init("Circle");
        this.radius = radius;
    }

    area() {
        return math.PI * this.radius * this.radius;
    }
}

var c = Circle(4);
c.describe();
// Output: I am a Circle with area 50.265482...
```

### 3. Error Handling & Control Flow

Robust control flow with `try-catch` and loop controls.

```javascript
fun riskyOperation(x) {
    if (x < 0) {
        throw "Negative number error!";
    }
    return 100 / x;
}

var inputs = [10, 0, -5, 20];

for (var i = 0; i < inputs.len(); i += 1) {
    var n = inputs[i];

    if (n == 0) {
        print "Skipping zero to avoid division error...";
        continue;
    }

    try {
        var result = riskyOperation(n);
        print "Result: " + result;
    } catch (err) {
        print "Caught exception: " + err;
    }
}
```

### 4. Module System

Rox features a module system with explicit exports and path resolution.

**`math_utils.rox`**:

```javascript
var internal_rate = 1.5; // Private variable

export fun scale(n) {
    return n * internal_rate;
}

export var version = "1.0.0";
```

**`main.rox`**:

```javascript
var utils = import("./math_utils.rox");

print "Using utils version: " + utils.version;
print utils.scale(10); // Output: 15
// print utils.internal_rate; // Error: Module has no export 'internal_rate'
```

### 5. File System & Native Modules

Interaction with the OS using the built-in `fs` module.

```javascript
var path = "log.txt";

if (fs.exists(path)) {
    var content = fs.readFile(path);
    print "Current log: " + content;
} else {
    fs.writeFile(path, "Initialization log...");
    print "Log file created.";
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
├── std_lib/        // Native modules
├── reader/         // Source file reading
├── ast/            // AST definitions (Expr/Stmt)
├── tokenizer/      // Lexical analysis
├── parser/         // Parsing logic & ParseHelper
├── resolver/       // Semantic analysis & Variable resolution
├── evaluate/       // Runtime execution
├── diagnostic.rs   // Diagnostic messages
├── error.rs        // Unified error handling
└── main.rs         // Entry point (CLI)
```

## 🤝 Contributing

Contributions are welcome! If you are interested in fixing a bug or adding a new feature, please read our [CONTRIBUTING](CONTRIBUTING.md) | [贡献](CONTRIBUTING_zh.md) guide first to set up your development environment.

## ⚖️ License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

_Built with ❤️ in Rust._
