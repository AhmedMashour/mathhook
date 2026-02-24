# MathHook

[![Crates.io](https://img.shields.io/crates/v/mathhook.svg)](https://crates.io/crates/mathhook)
[![Documentation](https://docs.rs/mathhook/badge.svg)](https://docs.rs/mathhook)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../../LICENSE)

High-performance educational computer algebra system for Rust.

## Features

- **Memory-optimized**: 32-byte Expression enum for cache performance
- **Hybrid API**: Expression methods + separate solver objects
- **Multi-format parsing**: LaTeX, Wolfram Language, standard notation
- **Educational focus**: Step-by-step explanations
- **Comprehensive macros**: `symbol!`, `symbols!`, `expr!`, `function!`

## Installation

```toml
[dependencies]
mathhook = "0.2.0"
```

## Quick Start

```rust,no_run
use mathhook::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create symbols using macros
    let x = symbol!(x);

    // Build expressions using expr! macro
    let quadratic = expr!(x^2 + 2*x + 1);

    // Simplify
    let simplified = quadratic.simplify();

    // Calculus
    let derivative = quadratic.derivative(x.clone());

    // Parse from string
    let parser = Parser::new(&ParserConfig::default());
    let parsed = parser.parse("sin(x)^2 + cos(x)^2")?;

    Ok(())
}
```

## Actual API Reference

For the complete API documentation, see **[docs.rs/mathhook](https://docs.rs/mathhook)**.

### Quick Reference

The prelude (`use mathhook::prelude::*`) provides:

| Export | Purpose |
|--------|---------|
| `Expression` | Core 32-byte symbolic expression type |
| `Symbol` | Symbolic variables |
| `Number` | Integer, Rational, Float, Complex |
| `MathSolver` | Equation solving |
| `Parser` | Multi-format expression parsing |
| `symbol!`, `expr!` | Expression construction macros |

### Memory Guarantee

```rust
use mathhook::prelude::*;
assert!(std::mem::size_of::<Expression>() <= 32);
```

## Documentation

- **[docs.rs/mathhook](https://docs.rs/mathhook)** - Full API reference
- **[mdbook Documentation](../../docs/)** - Guides and tutorials
- **[Architecture](../../docs/src/architecture/)** - System design

## License

MathHook is dual-licensed under MIT OR Apache-2.0. See [LICENSE](../../LICENSE).
