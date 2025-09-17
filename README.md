# MathHook

[![Crates.io](https://img.shields.io/crates/v/mathhook.svg)](https://crates.io/crates/mathhook)
[![PyPI](https://img.shields.io/pypi/v/mathhook.svg)](https://pypi.org/project/mathhook/)
[![npm](https://img.shields.io/npm/v/mathhook-node.svg)](https://www.npmjs.com/package/mathhook-node)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

**MathHook** is a high-performance educational computer algebra system (CAS) written in Rust. It provides symbolic mathematics, equation solving, LaTeX parsing, and step-by-step explanations optimized for both performance and educational use.

## Key Features

- **Symbolic Mathematics**: Expressions, algebra, calculus, and matrix operations
- **Multiple Input Formats**: Parse LaTeX, Wolfram Language, and standard mathematical notation
- **Equation Solving**: Linear, quadratic, polynomial, and systems of equations
- **Educational Focus**: Step-by-step explanations for all operations
- **High Performance**: Rust-based core with SIMD optimizations
- **Language Bindings**: Native support for Python and Node.js
- **Memory Efficient**: 32-byte expression representation for optimal cache performance
- **Production Ready**: Zero-copy parsing, arena allocation, thread-safe

## Quick Start

### Rust

```rust
use mathhook_core::prelude::*;

// Create expressions using ergonomic macros
let x = symbol!(x);
let expr = expr!((x ^ 2) + (2 * x) + 1);

// Simplify expressions
let simplified = expr.simplify();
println!("{}", simplified); // x^2 + 2*x + 1

// Solve equations
let mut solver = MathSolver::new();
let equation = Expression::equation(expr!(x ^ 2), expr!(4));
let solutions = solver.solve(&equation, &x);
println!("Solutions: {:?}", solutions); // x = 2, x = -2

// Parse mathematical expressions
let parser = Parser::new(ParserConfig::default());
let parsed = parser.parse(r"\frac{x}{2} + y^2").unwrap();
println!("{}", parsed);
```

### Python

```python
from mathhook import Expression, MathSolver

# Create expressions
x = Expression.symbol('x')
expr = x.pow(2).add(x.multiply(2)).add(1)

# Simplify
simplified = expr.simplify()
print(simplified)

# Parse and evaluate
parsed = Expression.parse(r"\frac{x}{2} + y^2")
print(parsed.to_latex())

# Solve equations
solver = MathSolver()
equation = Expression.equation(x.pow(2), Expression.integer(4))
solutions = solver.solve(equation, 'x')
print(f"Solutions: {solutions}")
```

### Node.js/TypeScript

```typescript
import { JsExpression as Expression, JsMathSolver as MathSolver } from 'mathhook-node';

// Create expressions
const x = Expression.symbol('x');
const expr = x.pow(2).add(x.multiply(2)).add(1);

// Simplify
const simplified = expr.simplify();
console.log(simplified.toString());

// Parse LaTeX
const parsed = Expression.parse(String.raw`\frac{x}{2} + y^2`);
console.log(parsed.toLatex());

// Solve equations
const solver = new MathSolver();
const equation = Expression.equation(x.pow(2), Expression.integer(4));
const solutions = solver.solve(equation, 'x');
console.log(`Solutions: ${solutions}`);
```

## Installation

### Rust

Add to your `Cargo.toml`:

```toml
[dependencies]
mathhook = "0.1"
```

### Python

```bash
pip install mathhook
```

Requires Python 3.8 or higher.

### Node.js

```bash
npm install mathhook-node
```

Requires Node.js 18 or higher.

## Core Capabilities

### Expression Building

Create mathematical expressions programmatically or parse from text:

```rust
// Programmatic construction
let expr = Expression::add(vec![
    Expression::integer(2),
    Expression::pow(symbol!(x), Expression::integer(2))
]);

// Using macros (recommended)
let expr = expr!((2) + (x ^ 2));

// From LaTeX
let parser = Parser::new(ParserConfig::default());
let expr = parser.parse(r"\sin(x) + \cos(y)").unwrap();
let expr = parser.parse(r"\frac{x}{2} + y^2").unwrap();

// From Wolfram Language
let expr = parser.parse("Sin[x] + Cos[y]").unwrap();
let expr = parser.parse("Integrate[x^2, x]").unwrap();
```

### Algebraic Operations

- **Simplification**: Canonical form, identity elimination, term collection
- **Expansion**: Distribute products, expand powers
- **Factoring**: Factor polynomials, extract common terms
- **Substitution**: Replace variables with expressions

### Calculus

- **Derivatives**: Symbolic differentiation with chain, product, and quotient rules
- **Integrals**: Symbolic and numeric integration
- **Limits**: Compute limits at finite and infinite points
- **Series**: Taylor and Laurent series expansions

### Equation Solving

- Linear equations
- Quadratic equations (including complex roots)
- Polynomial equations
- Systems of equations
- Matrix equation solving

### Matrix Operations

- Addition, multiplication, transposition
- Determinant computation
- Matrix inversion
- Eigenvalues and eigenvectors
- LU, QR, and Cholesky decomposition

## Performance

MathHook delivers **nanosecond-to-microsecond performance** for symbolic operations:

| Operation | Time |
|-----------|------|
| Elementary integration (`cos(x)`, `exp(x)`) | **< 300 ns** |
| Simple derivatives | **< 2 us** |
| Polynomial simplification (degree 50) | **< 10 us** |
| Complex calculus (chain + product rules) | **< 500 us** |

*Benchmarked with Criterion.rs. See [`benchmarks/baseline.json`](benchmarks/baseline.json) for raw data.*

### Why So Fast?

- **32-byte expressions**: Two fit per CPU cache line
- **Zero-copy parsing**: Direct AST construction without intermediate allocations
- **SIMD operations**: Vectorized arithmetic for bulk operations
- **No interpreter overhead**: Native Rust, no garbage collector
- **Thread-safe**: Immutable expressions, lock-free operations

## Educational Features

MathHook provides step-by-step explanations for all mathematical operations:

```rust
use mathhook::educational::*;

let x = symbol!(x);
let expr = expr!((x ^ 2) + (2 * x) + 1);

// Get step-by-step simplification
let explanation = expr.explain_simplification();
for step in explanation.steps() {
    println!("{}: {}", step.title, step.description);
}

// Get LaTeX-formatted explanation
println!("{}", explanation.to_latex());
```

## Architecture

MathHook is built as a multi-crate workspace:

- **mathhook-core**: Core mathematical engine (Rust)
- **mathhook-macros**: Useful macros for code generation (Rust)
- **mathhook**: High-level API with ergonomic macros (Rust)
- **mathhook-python**: Python bindings via PyO3
- **mathhook-node**: Node.js bindings via NAPI-RS
- **mathhook-benchmarks**: Performance benchmarking suite

### Design Principles

1. **Mathematical Correctness First**: Every operation must be mathematically correct
2. **Performance**: Cache-friendly data structures, SIMD operations
3. **Ergonomic API**: Macros and operator overloading for natural expression
4. **Educational Value**: Step-by-step explanations for all operations
5. **Multi-Language**: First-class bindings for Python and Node.js

## Try It Online

Launch interactive tutorials directly in Google Colab - no installation required!

[![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AhmedMashour/mathhook-kb/blob/master/colab-notebooks/getting/getting-started-introduction.colab.ipynb)

| Topic | Notebook |
|-------|----------|
| **Getting Started** | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AhmedMashour/mathhook-kb/blob/master/colab-notebooks/getting/getting-started-introduction.colab.ipynb) |
| **Differentiation** | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AhmedMashour/mathhook-kb/blob/master/colab-notebooks/operations/operations-differentiation.colab.ipynb) |
| **Integration** | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AhmedMashour/mathhook-kb/blob/master/colab-notebooks/operations/operations-integration.colab.ipynb) |
| **Expressions** | [![Open In Colab](https://colab.research.google.com/assets/colab-badge.svg)](https://colab.research.google.com/github/AhmedMashour/mathhook-kb/blob/master/colab-notebooks/core/core-expressions.colab.ipynb) |

[Browse all notebooks →](https://github.com/AhmedMashour/mathhook-kb/tree/master/colab-notebooks)

## Documentation

- **[Complete Guide](docs/)**: Comprehensive mdBook documentation with tutorials and advanced topics
- **[Usage Guide](USAGE.md)**: Quick reference and common patterns
- **[Python Documentation](crates/mathhook-python/README.md)**: Python-specific guide
- **[Node.js Documentation](crates/mathhook-node/README.md)**: Node.js-specific guide
- **[API Documentation](https://docs.rs/mathhook)**: Full Rust API reference

To build the documentation locally:

```bash
cd docs && mdbook serve
```

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/AhmedMashour/mathhook.git
cd mathhook

# Build the Rust core
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Building Python Bindings

```bash
cd crates/mathhook-python
pip install maturin
maturin develop
```

### Building Node.js Bindings

```bash
cd crates/mathhook-node
npm install
npm run build
```

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Requirements

- Rust 1.70 or higher
- Python 3.8+ (for Python bindings)
- Node.js 18+ (for Node.js bindings)
- LALRPOP (for parser development)

## License

MathHook is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## Citation

If you use MathHook in academic work, please cite:

```bibtex
@software{mathhook2025,
  title = {MathHook: High-Performance Educational Computer Algebra System},
  author = {Ahmed Mashhour},
  year = {2025},
  url = {https://github.com/AhmedMashour/mathhook}
}
```

## Acknowledgments

- Built with [LALRPOP](https://github.com/lalrpop/lalrpop) for parser generation
- Python bindings powered by [PyO3](https://github.com/PyO3/pyo3)
- Node.js bindings powered by [NAPI-RS](https://github.com/napi-rs/napi-rs)
- Inspired by [SymPy](https://www.sympy.org/)

## Status

MathHook is currently in **beta** (version 0.1.x). The API is stabilizing but may have breaking changes before 1.0.

### Roadmap

- [x] Python bindings (beta - core features implemented)
- [x] Node.js bindings (beta - core features implemented)
- [x] Comprehensive documentation (mdBook)
- [ ] Complete binding feature parity with Rust API
- [ ] WebAssembly support
- [ ] GPU acceleration (CUDA, WebGPU)
- [ ] 1.0 stable release

## Resources

### Website & Knowledge Base

- **[mathhook.org](https://mathhook.org)** - Official website with interactive demos, tutorials, and documentation
- **[mathhook-kb](https://github.com/AhmedMashour/mathhook-kb)** - Knowledge base engine that generates multi-format documentation (Jupyter notebooks, mdBook, LaTeX, Colab tutorials) from a single schema source

## Links

- **Homepage**: https://github.com/AhmedMashour/mathhook
- **Website**: https://mathhook.org
- **Documentation**: https://docs.rs/mathhook
- **Knowledge Base**: https://github.com/AhmedMashour/mathhook-kb
- **PyPI**: https://pypi.org/project/mathhook/
- **npm**: https://www.npmjs.com/package/mathhook-node
- **Issue Tracker**: https://github.com/AhmedMashour/mathhook/issues
