# Contributing to MathHook

Thank you for contributing to MathHook! This guide covers our development rules and standards.

> **Detailed Guides**: See [docs/src/contributing/](docs/src/contributing/) for in-depth documentation on [testing](docs/src/contributing/testing.md), [style](docs/src/contributing/style.md), [correctness](docs/src/contributing/correctness.md), and [documentation](docs/src/contributing/documentation.md).

---

## Priority Hierarchy

MathHook follows a strict priority order. **Never sacrifice a higher priority for a lower one.**

| Priority | Focus | Description |
|----------|-------|-------------|
| 1 | **Mathematical Correctness** | The math must be right. Nothing else matters if it's wrong. |
| 2 | **Error Handling** | Silent failures are correctness failures. |
| 3 | **Performance** | High performance is our selling point. |
| 4 | **Code Quality** | Clean, maintainable code for every commit. |
| 5 | **Architecture** | Long-term maintainability. |

**Remember: MEPCS** — *Math Errors Prevent Clean Systems*

---

## Quick Start

```bash
# Clone and build
git clone https://github.com/AhmedMashour/mathhook.git
cd mathhook
cargo build --release

# Run tests
cargo test

# Format and lint (required before commits)
cargo fmt
cargo clippy -- -D warnings
```

---

## Core Rules

### ✅ Always

1. **Use macros** for symbol and expression creation
2. **Run `cargo fmt`** before committing
3. **Run `cargo clippy -- -D warnings`** with zero warnings
4. **Run `cargo test`** before committing
5. **Verify against SymPy** before implementing math algorithms
6. **Test edge cases**: 0, negative, complex, infinity, undefined
7. **Return `Result<T, E>`** for fallible operations

### ❌ Never

1. Use `Symbol::new()` in application code—use `symbol!()` macro
2. Use `==` to compare floats—use epsilon comparison
3. Use `panic!()` or `unwrap()` in library code
4. Return `NaN` for domain errors—return `MathError` instead
5. Create files over 500 lines
6. Use `mod.rs` pattern—use `modulename.rs` instead
7. Leave TODO comments for critical functionality

---

## Macros

**Always use macros in application code:**

```rust
// ✅ Correct - Use macros
symbol!(x)                     // Scalar symbol
symbol!(A; matrix)             // Matrix symbol
symbol!(p; operator)           // Operator symbol
symbols![x, y, z]              // Multiple symbols
function!(sin, x)              // Function call
expr!(x ^ 2 + 2 * x + 1)       // Expression

// ❌ Wrong - Never use direct constructors
Symbol::new("x")               // Forbidden
Symbol::matrix("A")            // Forbidden
```

### Runtime Variables

```rust
// ❌ Wrong - Creates symbol named "i"
for i in 0..10 {
    expr!(i)  // Bug!
}

// ✅ Correct - Use explicit API
for i in 0..10 {
    Expression::integer(i)
}
```

### Power Operators

```rust
// All equivalent - use what's clearest
expr!(x ^ 2)       // Mathematical notation
expr!(x ** 2)      // Python-style
expr!(x.pow(2))    // Method call
```

---

## Mathematical Correctness

### Why It's Priority #1

MathHook users trust our results:
- Students learning mathematics
- Engineers making calculations
- Researchers validating proofs

A wrong answer **actively harms** users. There is no acceptable error rate.

### Edge Cases to Test

| Case | Example | Why |
|------|---------|-----|
| Zero | `f(0)` | Identity behavior |
| Negative | `f(-1)` | Sign handling |
| Complex | `f(i)` | Branch cuts |
| Infinity | `f(∞)` | Limit behavior |
| Undefined | `tan(π/2)` | Domain restrictions |

### SymPy Validation

```bash
# Validate against SymPy
./scripts/validate.sh           # All modules
./scripts/validate.sh simplify  # Specific module
```

### Numerical Precision

```rust
// ❌ Never compare floats with ==
if result == 0.0 { ... }

// ✅ Always use epsilon comparison
const EPSILON: f64 = 1e-10;
if result.abs() < EPSILON { ... }
```

---

## Testing

### Required Tests

```bash
cargo test                     # All tests
cargo test -p mathhook-core    # Specific crate
cargo test --doc               # Doctests
```

### Test Categories

1. **Unit tests**: Test individual functions
2. **Integration tests**: Test through public API
3. **Edge case tests**: Zero, negative, complex, infinity
4. **Regression tests**: Prevent previously-fixed bugs

### What to Test

```rust
#[test]
fn test_gamma_special_values() {
    // Integer values: Γ(n) = (n-1)!
    assert_eq!(gamma(&expr!(5)).unwrap(), expr!(24));

    // Half-integer: Γ(1/2) = √π
    assert_eq!(gamma(&expr!(1/2)).unwrap(), expr!(sqrt(pi)));

    // Poles (must error)
    assert!(gamma(&expr!(0)).is_err());
}
```

---

## Performance

### Size Constraints

| Type | Maximum |
|------|---------|
| `Expression` | 32 bytes |
| `Number` | 16 bytes |
| Source file | 500 lines |

### Benchmarking

```bash
./scripts/bench.sh save before   # Save baseline
# Make changes
./scripts/bench.sh compare before  # Compare
```

**Never change `Vec<T>` to `&[T]` in constructors** — this caused a 10% regression.

---

## Code Style

### File Organization

```
✅ Correct:
src/
├── parser.rs           # Module file
└── parser/             # Submodules
    ├── lexer.rs
    └── grammar.rs

❌ Wrong:
src/
└── parser/
    └── mod.rs          # Never use mod.rs
```

### Comments Policy

**Default: No comments.** Code should be self-documenting.

```rust
// ✅ Allowed - Mathematical context
// Quadratic formula: x = (-b ± √(b²-4ac)) / 2a

// ✅ Allowed - Non-obvious rationale
// O(n²) but n < 10 in practice

// ❌ Forbidden - Restating code
// Create a new expression
// Loop through items
// Return the result
```

### Error Handling

```rust
// ✅ Always return Result for fallible operations
pub fn log(arg: &Expression) -> Result<Expression, MathError> {
    if arg.is_zero() {
        return Err(MathError::DomainError { ... });
    }
    // ...
}

// ❌ Never panic in library code
panic!("invalid input");  // Forbidden
```

---

## Project Structure

```
mathhook/
├── crates/
│   ├── mathhook-core/       # Core mathematical engine
│   ├── mathhook-macros/     # Procedural macros (expr!, symbol!, etc.)
│   ├── mathhook/            # High-level user API
│   ├── mathhook-python/     # Python bindings
│   ├── mathhook-node/       # Node.js bindings
│   └── mathhook-benchmarks/ # Performance benchmarks
├── docs/                    # mdbook documentation
└── scripts/                 # Build and validation scripts
```

---

## Development Workflow

1. **Create feature branch**: `git checkout -b feature/name`
2. **Implement**: Follow rules above
3. **Test**: `cargo test`
4. **Validate**: `./scripts/validate.sh` (for math changes)
5. **Format**: `cargo fmt`
6. **Lint**: `cargo clippy -- -D warnings`
7. **Commit**: Clear, descriptive message
8. **PR**: Reference related issues

### PR Checklist

- [ ] Code formatted with `cargo fmt`
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] All tests pass (`cargo test`)
- [ ] Doctests pass (`cargo test --doc`)
- [ ] Documentation updated for public API changes
- [ ] Mathematical correctness verified (edge cases tested)
- [ ] No regressions in existing functionality

---

## Building Bindings

### Python

```bash
cd crates/mathhook-python
pip install maturin
maturin develop
```

### Node.js

```bash
cd crates/mathhook-node
npm install
npm run build
```

---

## Documentation

### Rust Docs

Every public API needs documentation with examples:

```rust
/// Compute sine of an expression.
///
/// # Examples
///
/// ```rust
/// use mathhook::prelude::*;
///
/// let result = sin(&expr!(0)).unwrap();
/// assert_eq!(result, expr!(0));
/// ```
pub fn sin(arg: &Expression) -> Result<Expression, MathError> { ... }
```

### mdbook

```bash
cd docs
mdbook serve --open  # Preview at localhost:3000
```

---

## Common Commands

| Task | Command |
|------|---------|
| Build | `cargo build --release` |
| Test | `cargo test` |
| Format | `cargo fmt` |
| Lint | `cargo clippy -- -D warnings` |
| Benchmark | `./scripts/bench.sh run` |
| Validate | `./scripts/validate.sh` |
| Docs | `cd docs && mdbook serve` |

---

## Getting Help

1. **Read the docs**: [docs/src/contributing/](docs/src/contributing/)
2. **Check existing code**: Follow established patterns
3. **Run tests**: They document expected behavior
4. **Open an issue**: For questions or bugs

---

## License

By contributing, you agree that your contributions will be licensed under the same terms as MathHook (MIT OR Apache-2.0).
