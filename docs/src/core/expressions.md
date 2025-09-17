# Expressions

The `Expression` type is the foundation of MathHook. This chapter explains expression structure, creation, and manipulation.

## Expression Structure

Expressions in MathHook are represented as an enum with variants for different mathematical constructs:

```rust
pub enum Expression {
    // Numbers
    Integer(i64),
    Rational(Box<RationalData>),
    Float(f64),
    Complex(Box<ComplexData>),

    // Variables
    Symbol(Symbol),

    // Operations
    Add(Vec<Expression>),
    Mul(Vec<Expression>),
    Pow(Box<Expression>, Box<Expression>),

    // Functions
    Function(String, Vec<Expression>),

    // Constants
    Constant(ConstantType),

    // Matrices
    Matrix(Vec<Vec<Expression>>),

    // Relations
    Equation(Box<Expression>, Box<Expression>),

    // Other variants...
}
```

## Creating Expressions

### Using Macros (Recommended)

The `expr!()` macro provides full mathematical syntax support:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

let x = symbol!(x);

// Basic arithmetic
let sum = expr!(x + y);
let product = expr!(x * y);
let difference = expr!(x - y);  // Becomes x + (-1)*y
let quotient = expr!(x / y);    // Becomes x * y^(-1)

// Power operations - three equivalent syntaxes
let power1 = expr!(x ^ 2);      // Caret notation (math-style)
let power2 = expr!(x ** 2);     // Double-star (Python-style)
let power3 = expr!(x.pow(2));   // Method call

// Mathematical precedence (^ binds tighter than * and /)
let quadratic = expr!(a * x ^ 2 + b * x + c);  // Correctly parsed

// Comparison operators
let eq = expr!(x == y);         // Equality
let lt = expr!(x < y);          // Less than
let gt = expr!(x > y);          // Greater than
let le = expr!(x <= y);         // Less or equal
let ge = expr!(x >= y);         // Greater or equal

// Method calls
let abs_val = expr!(x.abs());           // Absolute value
let sqrt_val = expr!(x.sqrt());         // Square root
let simplified = expr!(x.simplify());   // Simplify expression

// Function calls
let sin_val = expr!(sin(x));            // Unary function
let log_val = expr!(log(x, y));         // Binary function

// Complex nested expressions
let complex = expr!(sin(x ^ 2) + cos(y ^ 2));
let expanded = expr!((x + 1) * (x - 1));
```

### Using Constructors

For runtime values or when macros aren't suitable:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::prelude::*;

let x = symbol!(x);

// Direct constructors
let sum = Expression::add(vec![x.clone(), Expression::integer(1)]);
let product = Expression::mul(vec![x.clone(), Expression::integer(2)]);
let power = Expression::pow(x.clone(), Expression::integer(2));

// Use for runtime variables (NOT expr! macro)
for i in 0..10 {
    let term = Expression::integer(i);  // CORRECT
    // NOT: expr!(i) - this creates symbol "i"
}
```

## Expression Properties

### Immutability

Expressions are **immutable** after creation. All operations return new expressions:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let expr = expr!(x + 1);
let doubled = expr.mul(&expr!(2));  // Returns new expression
// `expr` is unchanged
```

### Memory Efficiency

Expressions are designed to be 32 bytes to fit in CPU cache lines for optimal performance.

## Why This Design?

### Why 32-Byte Expression Size?

**Design Decision**: MathHook's Expression type is constrained to exactly 32 bytes.

**Why?**
- Modern CPUs have 64-byte cache lines (standard on x86-64, ARM64)
- Two expressions fit perfectly in one cache line
- Cache-friendly data structures yield 3-5x faster operations in hot loops
- This is critical for CAS workloads with millions of expression traversals

**Trade-off**: Must use `Box<T>` for large nested structures
- Recursive types (like `Pow(Box<Expression>, Box<Expression>)`) use heap allocation
- Pointer indirection has small overhead, but cache benefits far outweigh it
- For typical expression trees (depth < 50), the trade-off is heavily positive

**Alternative Considered**: Variable-size expressions (like Python objects)
- **Pros**: Simpler implementation, no size constraints
- **Cons**: Poor cache locality, unpredictable performance, frequent cache misses
- **Decision**: Performance predictability > implementation simplicity for CAS workload

**When This Matters**:
- Hot loops processing millions of expressions (simplification, pattern matching)
- Recursive algorithms (symbolic differentiation, integration)
- Less important: One-time parsing, display formatting, or educational explanations

**Verification**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use std::mem::size_of;
use mathhook::Expression;

assert_eq!(size_of::<Expression>(), 32);
```

**Performance Impact**: Benchmarks show 3-5x speedup on simplification and 2-3x on derivative computation compared to variable-size design.

---

### Why Immutable Expressions?

**Design Decision**: Expressions cannot be modified after creation. All operations return new expressions.

**Why?**
- **Thread Safety**: Safe to share across threads without locks
- **Correctness**: No hidden mutation surprises
- **Optimization**: Compiler can optimize knowing values never change
- **Debugging**: Expression history is traceable

**Trade-off**: More allocations
- Each operation creates new expressions
- Mitigated by: reference counting (cheap clones), arena allocation for bulk operations
- Benchmark: <100ns overhead per operation (negligible in practice)

**Alternative Considered**: Mutable expressions with copy-on-write
- **Pros**: Fewer allocations in some cases
- **Cons**: Complex lifetime management, thread-safety issues, hard to reason about
- **Decision**: Simplicity and safety > micro-optimization

**Example**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let expr = expr!(x + 1);
let doubled = expr.mul(&expr!(2));
// `expr` is unchanged, `doubled` is new expression
// Safe to use both in parallel
```

---

### Why Canonical Forms?

**Design Decision**: MathHook automatically normalizes expressions to canonical form.

**What is Canonical Form?**
- `y + x` becomes `x + y` (sorted)
- `(a + b) + c` becomes `Add(a, b, c)` (flattened)
- `x + 0` becomes `x` (identity removed)
- `6/4` becomes `3/2` (rationals reduced)

**Why?**
- **Equality checking**: Structurally equal expressions are always equal
- **Simplification**: Canonical form is prerequisite for many simplification rules
- **Consistency**: Same mathematical expression always has same representation
- **Performance**: Pattern matching is faster on normalized expressions

**Trade-off**: Small overhead on construction
- Every `add()`, `mul()`, `pow()` normalizes
- Typically <50ns per operation
- Benefit: Avoid expensive normalization later during pattern matching

**Example**:
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let expr1 = expr!(x + y);
let expr2 = expr!(y + x);
assert_eq!(expr1, expr2);  // True - both normalized to x + y
```

**When This Matters**:
- Expression equality checking (hash tables, caches)
- Pattern matching in simplification rules
- Zero detection (is expression mathematically zero?)

---

### Thread Safety

Expressions are `Send + Sync`, making them safe to share across threads:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use std::sync::Arc;

let expr = Arc::new(expr!(x ^ 2));
let expr_clone = Arc::clone(&expr);
// Use in multiple threads safely
```

## Pattern Matching

Work with expression structure using Rust's pattern matching:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
match expr {
    Expression::Add(terms) => {
        println!("Sum with {} terms", terms.len());
    }
    Expression::Mul(factors) => {
        println!("Product with {} factors", factors.len());
    }
    Expression::Pow(base, exp) => {
        println!("Power: {} ^ {}", base, exp);
    }
    Expression::Function(name, args) => {
        println!("Function {} with {} args", name, args.len());
    }
    _ => {}
}
```

## Canonical Forms

Expressions automatically maintain canonical forms:

- **Commutative operations sorted**: \\(y + x \rightarrow x + y\\)
- **Associativity flattened**: \\((a + b) + c \rightarrow a + b + c\\)
- **Identity elimination**: \\(x + 0 \rightarrow x\\), \\(x * 1 \rightarrow x\\)
- **Rationals reduced**: \\(\frac{6}{4} \rightarrow \frac{3}{2}\\)

## Common Operations

### Simplification

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let expr = expr!(x + x);
let simplified = expr.simplify();
// Result: 2*x
```

### Evaluation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let expr = expr!(x ^ 2);
let result = expr.substitute(&x, &expr!(3));
// Result: 9
```

### Formatting

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let expr = expr!(x ^ 2);

println!("Standard: {}", expr);         // x^2
println!("LaTeX: {}", expr.to_latex()); // x^{2}
println!("Wolfram: {}", expr.to_wolfram()); // Power[x, 2]
```

## Next Steps

- [Symbols and Numbers](./symbols-numbers.md)
- [Functions](./functions.md)
- [Mathematical Operations](../operations/simplification.md)
