# Crate Structure

**Last Updated:** 2025-12-14T1730

MathHook is organized as a Rust workspace with specialized crates for different concerns.

## Workspace Overview

```
mathhook/
├── mathhook-core/       # Core mathematical engine
├── mathhook-macros/     # Procedural macros
├── mathhook/            # High-level user API
├── mathhook-python/     # Python bindings (PyO3)
├── mathhook-node/       # Node.js bindings (NAPI)
└── mathhook-benchmarks/ # Performance benchmarks
```

## mathhook-core

The core mathematical engine providing fundamental types and operations.

**Responsibilities:**
- Expression type and AST representation
- Mathematical operations (simplification, evaluation, differentiation)
- Parser implementation (LALRPOP-based)
- Special function implementations
- Polynomial operations
- Solver algorithms

**Key Types:**
- `Expression` - 32-byte AST node (cache-optimized)
- `Number` - 16-byte tagged union (integer/rational/float/complex)
- `Symbol` - Interned strings for O(1) equality

**Public API:**
```rust
use mathhook_core::{Expression, symbol, expr};

let x = symbol!(x);
let polynomial = expr!(x^2 + 2*x + 1);
```

## mathhook-macros

Procedural macros for ergonomic expression creation. All macros are re-exported by `mathhook-core` for convenience.

**Responsibilities:**
- Compile-time code generation for symbols and expressions
- Natural mathematical syntax parsing
- Type inference for symbol kinds (scalar, matrix, operator, quaternion)

### Available Macros

| Macro | Purpose | Example |
|-------|---------|---------|
| `symbol!(name)` | Create scalar symbol | `symbol!(x)` |
| `symbol!(name; type)` | Create typed symbol | `symbol!(A; matrix)` |
| `symbols![...]` | Create multiple symbols | `symbols![x, y, z]` |
| `function!(name, args...)` | Create function call | `function!(sin, x)` |
| `expr!(...)` | Create expression with math syntax | `expr!(x^2 + 2*x + 1)` |

### Symbol Types

Symbols can be declared with different algebraic properties:

```rust
use mathhook_macros::*;

// Scalar (commutative) - default
let x = symbol!(x);

// Matrix (noncommutative): A*B ≠ B*A
let a = symbol!(A; matrix);
let b = symbol!(B; matrix);

// Operator (noncommutative): [x,p] = xp - px ≠ 0
let p = symbol!(p; operator);

// Quaternion (noncommutative): i*j = k, j*i = -k
let i = symbol!(i; quaternion);
```

### Expression Syntax

The `expr!` macro supports full mathematical syntax with natural operator precedence:

```rust
use mathhook_macros::expr;

// Arithmetic operations
expr!(42)           // Integer literal
expr!(3.14)         // Float literal
expr!(x + y)        // Addition
expr!(x - y)        // Subtraction (becomes x + (-1)*y)
expr!(x * y)        // Multiplication
expr!(x / y)        // Division (becomes x * y^(-1))

// Power operations - THREE equivalent syntaxes
expr!(x ^ 2)        // Caret (natural math notation)
expr!(x ** 2)       // Double-star (Python-style)
expr!(x.pow(2))     // Method call

// Mathematical precedence (^ binds tighter than * and /)
expr!(2 * x ^ 2)    // Parsed as 2 * (x^2), NOT (2*x)^2
expr!(a * x^2 + b*x + c)  // Quadratic - no parentheses needed

// Right-associativity for power
expr!(2 ^ 3 ^ 4)    // Parsed as 2^(3^4) = 2^81

// Comparison operators
expr!(x == y)       // Equality
expr!(x < y)        // Less than
expr!(x > y)        // Greater than
expr!(x <= y)       // Less than or equal
expr!(x >= y)       // Greater than or equal

// Method calls
expr!(x.abs())      // Absolute value
expr!(x.sqrt())     // Square root
expr!(x.simplify()) // Simplify expression

// Function calls (any arity)
expr!(sin(x))       // Unary function
expr!(log(x, base)) // Binary function
expr!(f(a, b, c))   // N-ary function

// Complex nested expressions
expr!(sin(x^2) + cos(y^2))
expr!((x + 1) * (x - 1))
expr!(a*x^3 + b*x^2 + c*x + d)
```

### Important: Runtime Variables

Macros capture identifiers as **symbols**, not values. Use explicit API for runtime values:

```rust
use mathhook_core::Expression;

// ❌ WRONG - creates Symbol("i"), not integers
for i in 0..10 {
    expr!(i)  // Creates symbol "i" every iteration!
}

// ✅ CORRECT - use explicit API for runtime values
for i in 0..10 {
    Expression::integer(i)
}

// ❌ WRONG - creates Symbol("val")
let val = 42;
expr!(val)

// ✅ CORRECT
let val = 42;
Expression::integer(val)
```

**Rule:** If the value comes from a variable, loop, or conditional → use explicit API.

### Macro Implementation

All macros are **procedural macros** implemented using `syn` and `quote`:

```rust
// mathhook-macros/src/lib.rs
use proc_macro::TokenStream;

#[proc_macro]
pub fn symbol(input: TokenStream) -> TokenStream {
    // Parse and generate Symbol construction code
}

#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    // Parse mathematical syntax and generate Expression AST
}
```

### Re-exports

For convenience, `mathhook-core` re-exports all macros:

```rust
// mathhook-core/src/lib.rs
pub use mathhook_macros::{symbol, symbols, function, expr};
```

Users can import from either crate:

```rust
// Direct from macros crate
use mathhook_macros::{symbol, expr};

// Or from core (recommended)
use mathhook_core::{symbol, expr};
```

## mathhook

High-level user-facing API that wraps `mathhook-core` with convenience functions.

**Responsibilities:**
- Simplified public API
- String-based parsing functions
- Convenience methods and builders
- Integration utilities

**Public API:**
```rust
use mathhook::{parse, simplify, differentiate};

let result = simplify("x + x")?;  // Returns "2*x"
let derivative = differentiate("x^2", "x")?;  // Returns "2*x"
```

## mathhook-python

Python bindings using PyO3 for native performance in Python environments.

**Responsibilities:**
- Python class wrappers for Rust types
- Type conversion between Python and Rust
- Python-idiomatic API design
- NumPy integration

**Public API:**
```python
from mathhook import Symbol, parse, simplify

x = Symbol("x")
expr = parse("x^2 + 2*x + 1")
simplified = simplify(expr)
```

## mathhook-node

Node.js/TypeScript bindings using NAPI-RS for native performance in JavaScript environments.

**Responsibilities:**
- JavaScript class wrappers for Rust types
- Type conversion between JS and Rust
- TypeScript type definitions
- Promise-based async API where appropriate

**Public API:**
```typescript
import { Symbol, parse, simplify } from 'mathhook';

const x = new Symbol("x");
const expr = parse("x^2 + 2*x + 1");
const simplified = simplify(expr);
```

## mathhook-benchmarks

Performance benchmarking suite using Criterion.

**Responsibilities:**
- Comparative benchmarks vs SymPy, Symbolica
- Regression detection for performance changes
- Memory usage profiling
- Cache efficiency metrics

**Usage:**
```bash
./scripts/bench.sh run              # Full benchmark suite
./scripts/bench.sh quick            # Quick run
./scripts/bench.sh save baseline    # Save baseline
./scripts/bench.sh compare baseline # Compare to baseline
```

## Dependency Graph

```
mathhook-benchmarks
    ↓
mathhook  ←─── mathhook-python
    ↓              ↓
mathhook-core ←────┘ ←─── mathhook-node
    ↓
mathhook-macros
```

**Key Properties:**
- No circular dependencies
- `mathhook-macros` has no dependencies on other mathhook crates
- `mathhook-core` depends only on `mathhook-macros`
- Language bindings depend on `mathhook-core` (not high-level `mathhook`)

## Design Principles

### Separation of Concerns

Each crate has a single, well-defined responsibility:
- **macros**: Compile-time code generation only
- **core**: Mathematical operations and types
- **mathhook**: High-level convenience API
- **bindings**: Language-specific wrappers

### Performance Isolation

Performance-critical code lives in `mathhook-core`:
- 32-byte `Expression` constraint
- Zero-copy operations
- SIMD-enabled bulk operations
- Minimal allocations

### Testing Strategy

Each crate has its own test suite:

```bash
cargo test -p mathhook-macros      # Macro expansion tests
cargo test -p mathhook-core        # Core math correctness
cargo test -p mathhook             # High-level API tests
cargo test -p mathhook-python      # Python binding tests
cargo test -p mathhook-node        # Node binding tests
```

## Next Steps

- [Design Principles](./principles.md)
- [Type System](./type-system.md)
- [Memory Layout](./memory-layout.md)
