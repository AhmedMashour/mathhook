# Introduction

Welcome to the MathHook documentation! MathHook is a high-performance educational computer algebra system (CAS) written in Rust, designed to combine mathematical correctness with exceptional performance.

## What is MathHook?

MathHook is a symbolic mathematics engine that can:

- **Parse** mathematical expressions from multiple formats (LaTeX, Wolfram Language, standard notation)
- **Simplify** algebraic expressions using canonical forms and mathematical identities
- **Differentiate** and **integrate** expressions symbolically
- **Solve** equations and systems of equations
- **Manipulate** matrices with full linear algebra support
- **Explain** mathematical operations step-by-step for educational purposes

## Why MathHook?

### Performance-First Design

MathHook is built from the ground up for speed:

- **32-byte expression representation** fits perfectly in CPU cache lines
- **SIMD operations** for vectorized arithmetic (2-4x speedup)
- **Zero-copy parsing** directly constructs AST without intermediate allocations
- **Thread-safe immutable expressions** enable parallel processing
- **10-100x faster** than SymPy for common operations

### Mathematical Correctness

Every operation in MathHook is designed to be mathematically correct:

- Exact rational arithmetic (never loses precision)
- Proper domain handling (sqrt, log, division by zero)
- Canonical forms for reliable equality checking
- Validated against SymPy

### Educational Focus

MathHook provides step-by-step explanations for all mathematical operations, making it ideal for:

- Educational software
- Mathematics learning platforms
- Interactive mathematics tools
- Automated tutoring systems

### Multi-Language Support

MathHook provides first-class bindings for:

- **Rust** (native API with ergonomic macros)
- **Python** (via PyO3)
- **Node.js/TypeScript** (via NAPI-RS)
- **WebAssembly** (coming soon)

## Key Features

### Expression Building

Create mathematical expressions naturally:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
use mathhook::prelude::*;

let x = symbol!(x);
let expr = expr!(add: (x ^ 2), (2 * x), 1);
```

### Symbolic Computation

Perform algebraic manipulations symbolically:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# let x = symbol!(x);
# let expr = expr!(add: (x ^ 2), (2 * x), 1);
let simplified = expr.simplify();
let expanded = expr.expand();
let factored = expr.factor();
```

### Calculus Operations

Compute derivatives and integrals:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# let x = symbol!(x);
# let expr = expr!(add: (x ^ 2), (2 * x), 1);
let derivative = expr.derivative(x.clone());
let integral = expr.integrate(x, 0);
```

### Equation Solving

Solve equations and systems:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
# let x = symbol!(x);
# let equation = expr!((x ^ 2) - 4);
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x);
```

### Matrix Operations

Full linear algebra support:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let matrix = Expression::matrix(vec![
    vec![expr!(1), expr!(2)],
    vec![expr!(3), expr!(4)],
]);
```

## Architecture

MathHook is organized as a multi-crate workspace:

- **mathhook-core**: Core mathematical engine (pure Rust)
- **mathhook**: High-level API with ergonomic macros
- **mathhook-python**: Python bindings
- **mathhook-node**: Node.js/TypeScript bindings
- **mathhook-benchmarks**: Performance benchmarking suite

## Design Principles

MathHook follows five core principles (in priority order):

1. **Mathematical Correctness First**: Every operation must be mathematically correct
2. **Performance**: Cache-friendly data structures, SIMD operations, parallel processing
3. **Ergonomic API**: Macros and operator overloading for natural expression
4. **Educational Value**: Step-by-step explanations for all operations
5. **Multi-Language**: First-class bindings for Python, Node.js, and WebAssembly

## Getting Started

Ready to start using MathHook? Continue to:

- [Installation](./getting-started/installation.md) - Set up MathHook in your project
- [Quick Start](./getting-started/quick-start.md) - Your first 5 minutes with MathHook
- [Basic Usage](./getting-started/basic-usage.md) - Learn the fundamentals

## Community and Support

- **GitHub**: [https://github.com/AhmedMashour/mathhook](https://github.com/AhmedMashour/mathhook)
- **Documentation**: [https://docs.rs/mathhook](https://docs.rs/mathhook)
- **Issue Tracker**: [https://github.com/AhmedMashour/mathhook/issues](https://github.com/AhmedMashour/mathhook/issues)

## License

MathHook is dual-licensed under MIT or Apache 2.0. You may choose either license for your use.

## Acknowledgments

MathHook builds on the shoulders of giants:

- **LALRPOP** for parser generation
- **PyO3** for Python bindings
- **NAPI-RS** for Node.js bindings
- **SymPy** for inspiration and validation

---

Let's dive in and explore what MathHook can do!
