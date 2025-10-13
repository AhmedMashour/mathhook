# Frequently Asked Questions

## General

### What is MathHook?

MathHook is a high-performance educational computer algebra system (CAS) written in Rust.

### How does it compare to SymPy?

MathHook is 10-100x faster than SymPy for common operations while maintaining mathematical correctness.

### What languages are supported?

Rust (native), Python, Node.js/TypeScript, and WebAssembly (coming soon).

## Usage

### How do I create expressions?

Use the `expr!` and `symbol!` macros:

```rust
let x = symbol!(x);
let expr = expr!((x ^ 2) + 1);
```

### Why use rationals instead of floats?

Rationals provide exact arithmetic without precision loss. Use floats only when approximation is acceptable.

### How do I parse LaTeX?

```rust
use mathhook_core::parser::{Parser, ParserConfig};

let parser = Parser::new(ParserConfig::default());
let expr = parser.parse(r"\frac{x^2}{2}").unwrap();
```

## Performance

### How fast is MathHook?

10-100x faster than SymPy, competitive with other native CAS systems.

### Does it support parallel processing?

Yes, expressions are immutable and thread-safe. Use `parallel_bulk_simplify` for bulk operations.

### What is SIMD?

SIMD (Single Instruction Multiple Data) vectorizes arithmetic for 2-4x speedup on large arrays.

## Development

### How do I contribute?

See [Contributing Guide](../contributing/development.md) and [CLAUDE.md](../../CLAUDE.md).

### What are the design principles?

1. Mathematical correctness first
2. Performance
3. Ergonomic API
4. Educational value
5. Multi-language support

See [Design Principles](../architecture/principles.md) for details.

## Troubleshooting

### Parse errors?

Check syntax, use explicit `*` for multiplication, ensure balanced parentheses.

### Domain errors?

Check for sqrt of negatives, log of non-positives, or division by zero.

### Import errors?

Reinstall package: `pip install --force-reinstall mathhook`
