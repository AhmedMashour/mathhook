# Design Principles

MathHook is built on five core principles, listed in strict priority order.

## 1. Mathematical Correctness First

**This is the absolute highest priority. All other principles are secondary.**

Every mathematical operation in MathHook must be correct in ALL cases:

- Equation solving handles all valid cases correctly
- Simplification preserves mathematical equivalence
- Domain restrictions are respected (sqrt, log, division by zero)
- Symbolic operations maintain algebraic properties

### Zero Tolerance for Regressions

Any modification that breaks existing correct functionality is an unacceptable failure.

### Validation Against References

- **SymPy** (`~/Documents/work/math/sympy/`): Primary validation reference

## 2. Performance

After correctness, performance is the next priority.

### 32-Byte Expression Constraint

Expressions are exactly 32 bytes to fit two expressions per 64-byte cache line (standard on modern CPUs). This constraint:

- Enables efficient memory access patterns
- Improves CPU cache utilization
- Provides 10-100x speedup over Python-based systems

### Zero-Copy Parsing

Parse strings directly into AST without intermediate allocations.

### SIMD Operations

Vectorized operations for bulk arithmetic provide 2-4x speedup:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// SIMD automatically used for arrays > 100 elements
let values = vec![1.0, 2.0, 3.0, /* ... many values */];
let sum = simd_bulk_add_numeric(&values);
```

### Thread Safety

Immutable expressions enable parallel processing without locks:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let expressions = vec![/* many expressions */];
let simplified = parallel_bulk_simplify(&expressions);
```

## 3. Ergonomic API

MathHook provides natural, intuitive APIs.

### Macros for Clarity

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Clean and readable
let expr = expr!((x ^ 2) + (2 * x) + 1);

// vs verbose API
let expr = expr!((x ^ 2) + (2 * x) + 1);
```

### Operator Overloading

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Natural arithmetic
let sum = x + y;
let product = x * y;
```

## 4. Educational Value

MathHook provides step-by-step explanations for all operations.

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let explanation = expr.explain_simplification();

for step in explanation.steps() {
    println!("{}: {}", step.title, step.description);
}
```

This makes MathHook ideal for:

- Educational software
- Learning platforms
- Interactive mathematics tools
- Automated tutoring systems

## 5. Multi-Language Support

MathHook provides first-class bindings for multiple languages:

- **Rust** (native)
- **Python** (via PyO3)
- **Node.js/TypeScript** (via NAPI-RS)
- **WebAssembly** (coming soon)

All bindings maintain the same correctness and performance guarantees.

## Architectural Constraints

### Type System Constraints

These are non-negotiable:

- **Expression**: 32 bytes (cache-line optimization)
- **Number**: 16 bytes (fits in Expression)
- **Symbol**: String interning for O(1) equality

### Immutability

All expressions are immutable after creation. This enables:

- Thread-safe sharing
- Reliable caching
- Predictable behavior

### Canonical Forms

Expressions maintain canonical forms automatically:

- Commutative operations sorted
- Associativity flattened
- Identity elements eliminated
- Rationals reduced

## Next Steps

- [Type System](./type-system.md)
- [Function Intelligence System](./function-intelligence.md)
- [Memory Layout](./memory-layout.md)
- [Thread Safety](./thread-safety.md)
