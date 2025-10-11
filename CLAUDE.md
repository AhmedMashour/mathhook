# CLAUDE.md

This file provides guidance to Claude Code when working with the MathHook CAS codebase.

---

## CRITICAL: Core Mathematical Correctness Directive

**This is the absolute highest priority. All other guidelines are secondary.**

**IMPORTANT**: This CLAUDE.md file is the single source of truth. If anything in `.mathhook_sessions/` or elsewhere contradicts this document, CLAUDE.md takes precedence. Ignore contradicting information and flag the conflict.

### Non-Negotiable Requirements

1. **Mathematical Correctness First**: Every mathematical operation must be correct in ALL cases. No exceptions.
   - Equation solving must handle all valid cases correctly
   - Simplification must preserve mathematical equivalence
   - Evaluation must respect domain restrictions (e.g., sqrt of negatives, division by zero)
   - Symbolic operations must maintain algebraic properties

2. **Zero Tolerance for Regressions**: Any modification that breaks existing correct functionality is an unacceptable failure, regardless of what new functionality it adds.

3. **Verification Before Completion**: Before marking any task complete:
   - Run full test suite: `cargo test`
   - Verify affected functionality manually with edge cases
   - Check that doctests still pass: `cargo test --doc`
   - Confirm no mathematical accuracy was lost

4. **Authoritative References for Validation**:
   - **SymPy** (`~/Documents/work/math/sympy/`): Primary reference for algorithms and correctness validation
   - **Symbolica** (`~/Documents/work/math/symbolica`): Secondary reference for symbolic mathematics
   - When implementing any mathematical algorithm, verify against these references first

---

## Project Overview

MathHook is a high-performance educational computer algebra system (CAS) written in Rust. It provides symbolic mathematics, equation solving, LaTeX parsing, and step-by-step explanations optimized for both performance and educational use.

### Key Technical Constraints

These are non-negotiable architectural constraints. Violating them requires explicit discussion.

#### Expression Type (32-byte hard constraint)
- **Size**: Must remain 32 bytes for cache-line optimization
- **Structure**: Enum with variants for all expression types (Add, Multiply, Power, Function, Symbol, Number, etc.)
- **Memory**: Uses `Box<T>` for recursive structures to maintain size constraint
- **Performance Impact**: Fits in cache line; adding bytes hurts performance significantly

#### Number Type (16-byte hard constraint)
- **Size**: Exactly 16 bytes (do not modify)
- **Representation**: Tagged union supporting:
  - Integers (arbitrary precision via `BigInt` pointer)
  - Rationals (numerator/denominator as `BigInt` pointers)
  - Floats (f64)
- **Operations**: All arithmetic must preserve exactness when possible (prefer rationals over floats)

#### Symbol Type (Critical for variable handling)
- **Purpose**: Represents mathematical variables (x, y, theta, etc.)
- **Implementation**: String interning for O(1) equality comparison
- **Comparison**: Symbols with same name are identical (pointer equality after interning)
- **Cloning**: Cheap (just increments reference count)
- **Usage**: Never use raw strings for variables; always use `Symbol::new(name)`
- **Properties**: Symbols can have assumptions (positive, real, integer, etc.) - respect these in simplification

#### Function Type (Universal Function Intelligence System)
- **Architecture**: Modular intelligence per function family
  - Elementary: `sin`, `cos`, `exp`, `log`, `sqrt`, etc.
  - Special: `gamma`, `zeta`, `bessel`, `erf`, etc.
  - Number theory: `factorial`, `gcd`, `lcm`, etc.
  - Polynomials: Polynomial-specific functions
  
- **Registry**: `UniversalFunctionRegistry` provides O(1) lookup
  - Maps function name → function intelligence
  - Each function has: evaluation strategy, properties, simplification rules, educational explanations

- **Properties**: Functions know their own mathematical properties
  - Domain/range restrictions
  - Symmetry (even, odd, periodic)
  - Special values (e.g., sin(0) = 0, sin(π/2) = 1)
  - Derivatives and integrals

- **Evaluation**: Multi-strategy evaluation
  - Exact symbolic evaluation when possible
  - SIMD-optimized numerical evaluation
  - Handles special cases (sin(π) → 0, not 1.2246467991473532e-16)

#### LALRPOP Grammar (Parser Architecture)

**Critical Understanding**: The parser has two stages:
1. **Lexer** (`parser/lexer/`): Tokenizes input and inserts implicit multiplication tokens
2. **Parser** (`parser/grammar.lalrpop`): LALRPOP-generated LR(1) parser

**Implicit Multiplication Rules** (handled in lexer):
```
2x        → 2 * x
(a)(b)    → a * b
2(x+1)    → 2 * (x + 1)
sin x     → sin(x) * (no implicit multiplication before function args)
x y       → x * y (when both are identifiers)
2.5x      → 2.5 * x
```

**Operator Precedence** (highest to lowest):
1. Function application: `sin(x)`, `log(y)`
2. Exponentiation: `^` (right-associative!)
3. Implicit/explicit multiplication: `*`, `/` (left-associative)
4. Addition/subtraction: `+`, `-` (left-associative)

**LALRPOP Specifics**:
- **Grammar Type**: LR(1) - one token lookahead
- **Conflict Resolution**: Defaults to shift on shift/reduce conflicts
- **Right Associativity**: Exponentiation is special-cased as right-associative (e.g., `2^3^4` → `2^(3^4)`)
- **Location**: `crates/mathhook-core/src/parser/grammar.lalrpop`
- **Generated Output**: `crates/mathhook-core/src/parser/grammar.rs` (do not edit manually)

**Common Parser Pitfalls**:
- **Shift/Reduce Conflicts**: LALRPOP reports these; they usually indicate ambiguous grammar
- **Precedence Errors**: Test thoroughly with complex expressions: `2+3*4^5`
- **Implicit Multiplication Bugs**: The lexer handles this; don't duplicate in grammar
- **Function Call Ambiguity**: `sin(x)(y)` should parse as `sin(x) * y`, not `(sin(x))(y)`

**Testing Parser Changes**: Always test these cases after grammar modifications:
```rust
// Basic operations
"2+3*4"           // Should: 2 + (3 * 4) = 14
"2^3^4"           // Should: 2^(3^4) = 2^81 (right-associative)

// Implicit multiplication
"2x"              // Should: 2 * x
"2(x+1)"          // Should: 2 * (x + 1)
"(a+b)(c+d)"      // Should: (a + b) * (c + d)
"2.5x"            // Should: 2.5 * x

// Functions
"sin(x)cos(y)"    // Should: sin(x) * cos(y)
"2sin(x)"         // Should: 2 * sin(x)
"sin x"           // Should: sin(x) (NOT sin * x)
"log(x)/log(y)"   // Should: log(x) / log(y)

// Edge cases
"--x"             // Should: -(-x) = x (double negation)
"2+-3"            // Should: 2 + (-3) = -1
"x^-2"            // Should: x^(-2) = 1/x^2
```

#### Zero-Copy Parsing
- Parse strings directly into AST without intermediate allocations where possible
- Use string slices (`&str`) over `String` during parsing
- Convert to owned types only when storing in `Expression`

#### SIMD Operations
- Located in `core/performance/`
- Vectorized operations for arrays of numbers
- Must provide scalar fallback for non-SIMD targets
- Test with both AVX2 and SSE2 feature flags

---

## Documentation Standards (Strictly Enforced)

### Module and Function Documentation

1. **Use `//!` ONLY for module documentation** (at the top of files)
2. **Use `///` ONLY for item documentation** (functions, structs, enums, traits)
3. **Inline `//` comments are FORBIDDEN** except for:
   - Annotating specific mathematical formulas (e.g., `// Quadratic formula: x = (-b ± √(b²-4ac)) / 2a`)
   - Explaining critical business logic that isn't self-evident from code

### Documentation Requirements

Every public function MUST include:
- Clear description of what it does
- `# Arguments` section documenting each parameter
- `# Examples` section with a runnable doctest in a ````rust` block
- `# Panics` section if the function can panic
- `# Safety` section if the function is unsafe

### Prohibited Content

- **No emojis anywhere** (code, comments, documentation, commit messages)
- **No ALL CAPS** except for constants (e.g., `const MAX_DEPTH: usize = 100;`)
- **No TODO comments** - implement completely or don't implement at all
- **No placeholder implementations** - if a function exists, it must be fully correct

### Legacy Cleanup

If you encounter violations of these rules, delete them immediately.

### Example of Correct Documentation

```rust
/// Computes the derivative of an expression with respect to a variable
///
/// Uses the chain rule, product rule, and quotient rule as needed. For polynomial
/// expressions, applies power rule. For transcendental functions, uses standard
/// calculus derivative rules.
///
/// # Arguments
///
/// * `expr` - The expression to differentiate
/// * `var` - The variable to differentiate with respect to
/// * `order` - The order of differentiation (1 for first derivative, 2 for second, etc.)
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{Expression, Symbol};
///
/// let x = Symbol::new("x");
/// let expr = Expression::pow(
///     Expression::symbol(x.clone()),
///     Expression::integer(2)
/// );
/// let derivative = expr.derivative(&x, 1);
/// assert_eq!(derivative.to_string(), "2*x");
/// ```
///
/// # Panics
///
/// Panics if `order` is 0, as the 0th derivative is the expression itself.
pub fn derivative(&self, var: &Symbol, order: u32) -> Expression {
    // Implementation
}
```

---

## Code Quality Principles (Priority Order)

After mathematical correctness, prioritize in this exact order:

1. **Performance & Memory Efficiency**: Write highly performant, cache-friendly code
   - Profile hot paths with `cargo bench`
   - Minimize allocations in tight loops
   - Use arena allocation for bulk operations
   - Prefer stack allocation over heap when possible

2. **Readability & Idiomatic Rust**: Follow Rust best practices
   - Reference: [The Rust Book](https://doc.rust-lang.org/book/)
   - Use iterators over manual loops
   - Leverage the type system for correctness
   - Prefer `impl Trait` over boxed trait objects when possible

3. **Memory Safety & Performance Optimization**:
   - Reference: [Rust Performance Book](https://nnethercote.github.io/perf-book/)
   - Minimize bounds checking in hot paths
   - Use `#[inline]` judiciously on small, frequently-called functions
   - Consider SIMD for vectorizable operations

4. **Modularity**: Design components to be loosely coupled with clear interfaces

5. **Meaningful Testing**: Write tests that validate correctness and edge cases, not just coverage

6. **Architectural Patterns Over Hardcoding**: Avoid hardcoded matches for mathematical elements
   - **NEVER hardcode function names** (sin, cos, etc.) in implementation logic
   - Use the `UniversalFunctionRegistry` for function-specific behavior
   - Leverage type system and traits over string matching
   - Example:
     ```rust
     // ❌ DON'T: Hardcoded function matching
     match func_name.as_str() {
         "sin" => /* special handling */,
         "cos" => /* special handling */,
         _ => /* generic */
     }

     // ✅ DO: Use registry-based dispatch
     if let Some(intelligence) = registry.get_function(func_name) {
         intelligence.evaluate(args)
     }
     ```
   - This principle applies to: functions, constants, operators, and mathematical patterns
   - Benefits: extensibility, maintainability, testability, performance (O(1) lookup)

---

## Common AI Pitfalls (CRITICAL - Read Carefully)

### Mathematical Errors to Avoid

1. **Off-by-one in polynomial degrees**: When working with polynomials, be extremely careful with degree calculations
2. **Sign errors in derivatives**: Chain rule application is error-prone for complex expressions
3. **Domain restrictions**: Always check for division by zero, sqrt of negatives, log of non-positives
4. **Floating point comparison**: Never use `==` for floats; use epsilon comparison or symbolic equality
5. **Rational arithmetic overflow**: Integer operations in `Number` type can overflow; handle carefully

### Rust-Specific Mistakes

1. **Unnecessary cloning**: Many operations can take references instead of owned values
2. **Iterator anti-patterns**: Don't collect() unnecessarily; chain iterators instead
3. **Premature optimization**: Profile before optimizing; don't assume what's slow
4. **Lifetime over-complication**: Keep lifetimes simple; prefer owned types when unclear

### Parser Gotchas (LALRPOP)

1. **Shift/reduce conflicts**: LALRPOP defaults to shift; this may not be what you want
2. **Implicit multiplication precedence**: Be extremely careful with operator precedence when modifying grammar
3. **Lexer-parser mismatch**: The lexer pre-processes for implicit multiplication; don't duplicate this in grammar
4. **Token lookahead**: LR(1) only has 1 token lookahead; complex disambiguation needs lexer help

When modifying `grammar.lalrpop`:
- Run `lalrpop crates/mathhook-core/src/parser/grammar.lalrpop` to see errors before cargo build
- Test extensively: `cargo test -p mathhook-core parser`
- Check that implicit multiplication still works: `2x`, `(a)(b)`, `2(x+1)`

---

## Macro Usage Guidelines

### Available Macros

MathHook provides declarative macros for ergonomic expression creation:

- **`symbol!(x)`** - Create symbols
- **`function!(name, args...)`** - Create function expressions
- **`expr!(...)`** - Create expressions with operator parsing

### When to Use Macros vs Explicit API

**ALWAYS prefer macros for (Mandatory - Replace During Modification):**

1. **Symbol Creation**: `symbol!(x)` instead of `Symbol::new("x")`
   ```rust
   // ❌ Don't use:
   Symbol::new("x")
   
   // ✅ Always use:
   symbol!(x)
   ```

2. **Simple Expressions**: Single operations are clean and readable
   ```rust
   // ✅ Good - clean and readable:
   expr!(x + y)
   expr!(2 * x)
   expr!(x ^ 2)
   expr!(sin(x))
   expr!((x + 1) * (x - 1))  // Use parens for grouping
   
   // ❌ Don't use verbose API for simple cases:
   Expression::add(vec![
       Expression::symbol(Symbol::new("x")),
       Expression::symbol(Symbol::new("y"))
   ])
   ```

3. **Test Code and Documentation**: Makes examples immediately readable
   ```rust
   #[test]
   fn test_derivative() {
       let x = symbol!(x);
       let f = expr!(x ^ 2);
       let df = f.derivative(&x, 1);
       assert_eq!(df, expr!(2 * x));  // ✅ Clear and readable
   }
   ```

**Use Explicit API for:**

1. **Complex Mixed Operations**: When precedence is unclear
   ```rust
   // ⚠️  Unclear: expr!(2*x + y*z - 3)
   // ✅ Clear with explicit API or explicit grouping:
   expr!((2*x) + (y*z) - 3)  // Good with parens
   // OR use explicit API for very complex cases
   ```

2. **Programmatic Construction**: Loops, conditionals, dynamic building
   ```rust
   // Building terms in a loop - explicit API is clearer:
   let mut terms = Vec::new();
   for i in 0..n {
       terms.push(Expression::mul(vec![
           Expression::integer(coeffs[i]),
           Expression::pow(symbol!(x), Expression::integer(i))
       ]));
   }
   Expression::add(terms)
   ```

3. **Multi-term Operations**: Use helper variants
   ```rust
   // For many terms, use explicit helpers:
   expr!(add: x, y, z, w)      // ✅ Clear
   expr!(mul: 2, x, y, z)      // ✅ Clear
   ```

### Macro Capabilities and Limitations

**What Works Well (Use These Patterns):**
```rust
// ✅ Literals and symbols
expr!(42)
expr!(x)
expr!(pi)    // Constants become zero-arg functions

// ✅ Single binary operations
expr!(x + y)
expr!(x - y)
expr!(x * y)
expr!(x / y)
expr!(x ^ 2)

// ✅ Unary operations
expr!(-x)

// ✅ Function calls (0-3 args)
expr!(sin(x))
expr!(log(x, y))
expr!(f(x, y, z))

// ✅ Grouped operations with explicit parentheses
expr!((x + 1) * (x - 1))
expr!((2*x) + 3)

// ✅ Multi-term explicit operations
expr!(add: x, y, z)
expr!(mul: 2, x, y)
```

**Limitations (Declarative Macro Constraints):**

1. **Mixed operators without parentheses may have precedence issues**
   ```rust
   // ⚠️  Precedence unclear:
   expr!(2*x + 3)
   
   // ✅ Use explicit grouping:
   expr!((2*x) + 3)
   ```

2. **Functions with 4+ arguments need explicit helper**
   ```rust
   // ❌ Won't match macro patterns:
   expr!(f(a, b, c, d, e))
   
   // ✅ Use function! macro:
   function!(f, expr!(a), expr!(b), expr!(c), expr!(d), expr!(e))
   ```

3. **Complex nested operations - use explicit API when clearer**
   ```rust
   // If readability suffers with macros, use explicit API
   ```

**Future Enhancement (Procedural Macro):**

A future procedural macro version will support full mathematical syntax:
```rust
expr!(2*x + 3*y^2 - sin(x)/cos(x))  // Future goal
```

Until then, use parentheses for grouping and explicit helpers for complex cases.

### Gradual Migration Strategy (Enforce During Code Modifications)

When touching existing code, actively migrate to macro usage:

**Priority 1: Always migrate (No exceptions)**
```rust
// Find and replace these ALWAYS:
Symbol::new("x")          → symbol!(x)
Symbol::new("theta")      → symbol!(theta)
```

**Priority 2: Migrate when touching the code**
```rust
// Old verbose patterns:
Expression::add(vec![
    Expression::symbol(Symbol::new("x")),
    Expression::symbol(Symbol::new("y"))
])

// ✅ New clean pattern:
expr!(x + y)
```

```rust
// Old:
Expression::pow(
    Expression::symbol(Symbol::new("x")),
    Expression::integer(2)
)

// ✅ New:
expr!(x ^ 2)
```

```rust
// Old:
Expression::function("sin", vec![Expression::symbol(Symbol::new("x"))])

// ✅ New:
expr!(sin(x))
```

**Migration Guidelines:**

1. **Don't migrate if macro makes it less clear** - readability always wins
2. **Do migrate simple patterns** - they're clearer with macros
3. **Use explicit grouping** - When in doubt, add parentheses: `expr!((2*x) + (3*y))`
4. **Test after migration** - Run relevant tests to ensure behavior unchanged

**Example Migration:**

```rust
// Before:
let derivative = Expression::mul(vec![
    Expression::integer(2),
    Expression::symbol(Symbol::new("x"))
]);

// After:
let derivative = expr!(2 * x);
```

### Examples of Good Macro Usage

```rust
// ✅ Excellent - symbols are always cleaner with macros
let x = symbol!(x);
let theta = symbol!(theta);

// ✅ Excellent - simple expressions are very readable
let quadratic = expr!((a*x^2) + (b*x) + c);
let trig_identity = expr!(sin(x)^2 + cos(x)^2);
let derivative = expr!(2 * x);

// ✅ Good - test cases are immediately understandable
#[test]
fn test_derivative_power_rule() {
    let x = symbol!(x);
    let f = expr!(x ^ 3);
    let df = f.derivative(&x, 1);
    assert_eq!(df, expr!(3 * (x ^ 2)));
}

// ✅ Good - documentation examples are clean
/// # Examples
/// ```rust
/// let x = symbol!(x);
/// let expr = expr!(sin(x) / cos(x));  // tan(x)
/// let simplified = expr.simplify();
/// assert_eq!(simplified, expr!(tan(x)));
/// ```

// ✅ Good - explicit grouping for complex expressions
let complex = expr!((2*x + 1) * (3*y - 2));

// ⚠️ Acceptable - multi-term with explicit helper
let polynomial = expr!(add: 
    expr!(x ^ 3),
    expr!(-2 * (x ^ 2)),
    expr!(3 * x),
    expr!(-5)
);

// ❌ Avoid - macro doesn't improve clarity here
let programmatic = {
    let mut terms = Vec::new();
    for i in 0..n {
        terms.push(expr!(mul: expr!(coef), expr!(x ^ i)));  // Awkward
    }
    Expression::add(terms)  // Just use explicit API
};

// ✅ Better - use explicit API for programmatic construction
let programmatic = {
    let mut terms = Vec::new();
    for i in 0..n {
        terms.push(Expression::mul(vec![
            Expression::integer(coefficients[i]),
            Expression::pow(symbol!(x), Expression::integer(i))
        ]));
    }
    Expression::add(terms)
};
```

---

## Architecture

### Workspace Structure

```
mathhook/
├── mathhook-core/       # Core mathematical engine
├── mathhook/            # High-level user API with ergonomic macros
├── mathhook-python/     # Python bindings (PyO3)
├── mathhook-node/       # Node.js bindings (NAPI)
└── mathhook-benchmarks/ # Performance benchmarks (Criterion)
```

### Core Module Organization (mathhook-core)

**Module Size Limit**: Maximum 500 lines per file. Split larger modules into focused sub-modules.

```
src/
├── core/                 # Foundational types (Expression, Symbol, Number)
│   ├── expression/       # Central Expression enum (32-byte target)
│   ├── number.rs         # 16-byte number representation
│   ├── symbol.rs         # Variable symbols
│   ├── constants.rs      # Mathematical constants (π, e, etc.)
│   ├── arena.rs          # Memory arena for bulk allocation
│   └── performance/      # SIMD operations
├── parser/               # Multi-format parsing (LaTeX, Wolfram, standard)
│   ├── grammar.lalrpop   # LALRPOP grammar (generates grammar.rs)
│   ├── lexer/            # Tokenization and implicit multiplication
│   └── cache.rs          # Parse caching
├── functions/            # Universal Function Intelligence System
│   ├── elementary/       # sin, cos, exp, log, etc.
│   ├── special/          # gamma, zeta, bessel, etc.
│   ├── number_theory/    # factorial, gcd, etc.
│   └── polynomials/      # Polynomial functions
├── algebra/              # Algebraic operations (simplify, expand, factor)
├── calculus/             # Derivatives and integrals
│   ├── derivatives/      # Symbolic differentiation
│   └── integrals/        # Symbolic and numeric integration
├── solvers/              # Equation solving (linear, quadratic, polynomial, systems)
├── matrix/               # Matrix operations and decomposition
├── educational/          # Step-by-step explanations
├── formatter/            # Output formatting (LaTeX, human-readable)
├── simplify/             # Simplification strategies and zero detection
└── serialize/            # Serialization support
```

### Hybrid API Design Philosophy

MathHook provides two complementary API styles:

1. **Expression-centric API** (functional, method chaining):
```rust
let result = Expression::add(vec![
    Expression::integer(2),
    Expression::integer(3)
]).simplify();
```

2. **Solver object API** (stateful, configuration-driven):
```rust
let mut solver = MathSolver::new()
    .with_precision(1e-10)
    .with_max_iterations(1000);
let solutions = solver.solve(&equation, &Symbol::new("x"));
```

Choose the appropriate style for the use case. Don't force one pattern where the other is more natural.

---

## Performance Optimization Guidelines

### Hot Path Identification

Profile before optimizing:
```bash
cargo bench
cargo flamegraph --bench benchmark_name
```

### Memory Layout Constraints

1. **Expression**: Target 32 bytes (hard constraint for cache-line optimization)
   - Current size: Check with `std::mem::size_of::<Expression>()`
   - If modifications exceed 32 bytes, discuss architectural changes first

2. **Number**: Exactly 16 bytes (current, do not modify)
   - Supports: integers, rationals, floats
   - Tagged union with discriminant

### Optimization Techniques

1. **Arena Allocation**: Use for bulk expression allocation
   - Located in `core/arena.rs`
   - Use when creating many short-lived expressions

2. **SIMD Operations**: Available in `core/performance/`
   - Use for vectorizable arithmetic on arrays
   - Test with both AVX2 and SSE fallbacks

3. **Inlining**: Use `#[inline]` for:
   - Small functions (<10 lines)
   - Functions called in hot loops
   - Getters and simple operations

4. **Avoid Allocations**:
   - Prefer `&[T]` over `Vec<T>` in function signatures when not modifying
   - Use `SmallVec` for collections with known small size (<8 elements)
   - Reuse buffers when possible

### Benchmarking Requirements

Add benchmarks for:
- New core operations (derivatives, integrals, solving)
- Modifications to hot paths
- Parser changes

Location: `crates/mathhook-benchmarks/benches/`

---

## Testing Strategy

### Test Categories

1. **Unit Tests**: In each module (target 100% for core mathematical functionality)
2. **Integration Tests**: In `tests/` directory
3. **Doctests**: ALL public API functions must have working examples
4. **Property Tests**: Use `proptest` for algebraic properties (commutativity, associativity, etc.)
5. **Benchmarks**: For performance-critical paths

### Testing Philosophy

**DO**:
- Test edge cases: zero, infinity, undefined, complex numbers
- Test mathematical properties: `a + b == b + a`, `(a * b) * c == a * (b * c)`
- Test domain boundaries: sqrt(-1), log(0), division by zero
- Use meaningful test names: `test_quadratic_solver_handles_complex_roots`
- Test both success and failure cases

**DON'T**:
- Test implementation details (test behavior, not internals)
- Write brittle tests that break with refactoring
- Test trivial getters/setters
- Aim for coverage metrics over meaningful validation

### Test Naming Convention

```rust
#[test]
fn test_<operation>_<scenario>_<expected_outcome>() {
    // Example: test_derivative_of_polynomial_returns_correct_result
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific crate
cargo test -p mathhook-core

# Specific test
cargo test test_quadratic_solver

# Don't stop on first failure
cargo test --no-fail-fast

# With output
cargo test -- --nocapture

# Doctests only
cargo test --doc

# Release mode (for performance-sensitive tests)
cargo test --release
```

---

## Building and Development Commands

### Build Commands

```bash
# Full workspace build
cargo build

# Release build (optimized)
cargo build --release

# Specific crate
cargo build -p mathhook-core

# Check without building (fast)
cargo check

# Check all targets
cargo check --all-targets
```

### Parser Regeneration

The parser uses LALRPOP (LR(1) parser generator). When modifying the grammar:

```bash
# Option 1: Let cargo build handle it
cargo build -p mathhook-core

# Option 2: Manual regeneration (see errors immediately)
lalrpop crates/mathhook-core/src/parser/grammar.lalrpop

# The generated parser will be at:
# crates/mathhook-core/src/parser/grammar.rs
```

**CRITICAL**: After parser changes, always test:
```bash
cargo test -p mathhook-core parser
```

### Linting and Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy
cargo clippy

# Clippy in CI mode (fail on warnings)
cargo clippy -- -D warnings
```

---

## Development Workflow

### When Adding New Features

1. **Design First**: Consider architectural impact before coding
2. **Write Documentation**: Document the API with examples (TDD for docs)
3. **Implement**: Write the feature
4. **Verify Doctests**: `cargo test --doc`
5. **Add Unit Tests**: Cover edge cases
6. **Add Benchmarks**: If performance-critical
7. **Verify No Regressions**: Run full test suite

### When Modifying the Parser

1. **Edit**: `crates/mathhook-core/src/parser/grammar.lalrpop`
2. **Regenerate**: `lalrpop crates/mathhook-core/src/parser/grammar.lalrpop` OR `cargo build -p mathhook-core`
3. **Test**: `cargo test -p mathhook-core parser`
4. **Manual Verification**: Test implicit multiplication edge cases
   - `2x` → `2 * x`
   - `(a)(b)` → `a * b`  
   - `2(x+1)` → `2 * (x + 1)`
   - `sin(x)cos(x)` → `sin(x) * cos(x)`

### When Adding Mathematical Functions

1. **Implement Intelligence**: Add to appropriate module in `functions/`
   - Elementary: `functions/elementary/`
   - Special: `functions/special/`
   - Number theory: `functions/number_theory/`
   - Polynomials: `functions/polynomials/`

2. **Register**: Add to `UniversalFunctionRegistry`

3. **Implement Methods**:
   - `evaluate()`: Numerical evaluation with SIMD optimization
   - `properties()`: O(1) lookup for mathematical properties
   - `explain()`: Educational explanation generation
   - `simplify()`: Simplification rules

4. **Add Tests**: Cover all mathematical properties

5. **Document**: Include mathematical definition and examples

6. **Verify Against SymPy**: Ensure correctness matches SymPy behavior

### When Solving Mathematical Bugs

1. **Reproduce**: Create a minimal test case that fails
2. **Validate Expected Behavior**: Check SymPy/Symbolica for correct answer
3. **Identify Root Cause**: Don't guess; use debugging and tests
4. **Fix**: Implement correction
5. **Prevent Regression**: Add test case to suite
6. **Verify Broadly**: Run full test suite to ensure no side effects

---

## Session Management

**Development notes are stored in** `.mathhook_sessions/`

### Document Precedence (CRITICAL)

**CLAUDE.md is the authoritative source.** If you encounter contradictions:

1. **CLAUDE.md always wins** - Follow the rules and patterns in this document
2. **Flag the conflict** - Tell me: "I found a conflict between CLAUDE.md and [session note X]. CLAUDE.md says [Y], but the session note says [Z]. I'm following CLAUDE.md."
3. **Don't waste time** - Don't try to reconcile or merge contradicting information; just follow CLAUDE.md
4. **Suggest cleanup** - After flagging, suggest: "Should I update [session note] to match CLAUDE.md?"

### When to Update Session Notes

When you complete significant work or discover important patterns/decisions:
1. Add findings to appropriate session note
2. Keep notes factual and concise (what was done, what was learned)
3. **Do not duplicate CLAUDE.md content** - Session notes are for specific implementation details, not general rules

### When to Update CLAUDE.md

Update this file when:
- A new architectural pattern emerges that should be standard
- A new constraint is discovered (performance, mathematical correctness)
- You repeatedly encounter the same mistake or confusion
- A critical dependency or build process changes
- You discover a better way to organize or explain something
- A rule in a session note should become a universal rule

**Tell me immediately when you update this file and explain why.**

### Session Notes Purpose

Session notes capture:
- Specific implementation decisions for features
- Debugging discoveries and solutions
- Performance profiling results
- Temporary workarounds (that should eventually be removed)
- Context for "why we did X instead of Y"

Session notes do NOT replace CLAUDE.md rules.

---

## Current Development Focus

Active areas of development:
- Enhanced function intelligence system
- Pattern matching and smart evaluation  
- System equation solving
- Advanced simplification strategies
- Macro system improvements (operator parsing)

Check `.mathhook_sessions/` for detailed notes on specific development sessions.

---

## When to Update This Document

Update CLAUDE.md when:
- A new architectural pattern emerges
- A new constraint is discovered (performance, mathematical correctness)
- You repeatedly encounter the same mistake or confusion
- A new critical dependency is added
- Build/test processes change
- You discover a better way to organize or explain something

**Tell me immediately when you update this file and explain why.**