# Noncommutative Algebra in MathHook

Noncommutative algebra support for matrices, quantum operators, and quaternions.

## Table of Contents

1. [Overview](#overview)
2. [Quick Start](#quick-start)
3. [Symbol Types](#symbol-types)
4. [Creating Symbols](#creating-symbols)
5. [Usage Examples](#usage-examples)
6. [Further Reading](#further-reading)

## Overview

Noncommutative algebra is essential for mathematics and physics where order matters. MathHook provides first-class support through:

- **Four symbol types**: Scalar (commutative), Matrix, Operator, Quaternion (all noncommutative)
- **Type-aware parsing**: Automatic type inference from LaTeX notation
- **Smart equation solving**: Distinguishes left division (A*X=B) from right division (X*A=B)
- **Educational features**: Step-by-step explanations showing why order matters
- **Proper LaTeX formatting**: Type-specific notation (bold for matrices, hat for operators)

### Why Order Matters

In commutative algebra, order doesn't matter:
- `2 * 3 = 3 * 2 = 6`
- `x * y = y * x`

In noncommutative algebra, order is critical:
- Matrix multiplication: `A * B ≠ B * A` in general
- Quantum operators: `[x, p] = xp - px = iℏ ≠ 0`
- Quaternions: `i * j = k`, but `j * i = -k`

MathHook's type system ensures correct handling of both cases.

## Quick Start

```rust
use mathhook_core::{symbol, symbols, Expression};
use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
use mathhook_core::algebra::solvers::EquationSolver;
use mathhook_core::formatter::latex::LaTeXContext;

// Create matrix symbols
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Build equation: A*X = B
let equation = Expression::add(vec![
    Expression::mul(vec![
        Expression::symbol(A),
        Expression::symbol(X.clone())
    ]),
    Expression::mul(vec![Expression::integer(-1), Expression::symbol(B)]),
]);

// Solve equation
let solver = MatrixEquationSolver::new();
let result = solver.solve(&equation, &X);

// Format solution as LaTeX
if let SolverResult::Single(solution) = result {
    let latex = solution.to_latex(LaTeXContext::default()).unwrap();
    println!("Solution: {}", latex);  // \mathbf{A}^{-1} \cdot \mathbf{B}
}
```

## Symbol Types

MathHook supports four symbol types with different commutativity properties:

### 1. Scalar (Default - Commutative)

Use for real numbers, complex numbers, standard variables.

**Properties**:
- Commutative: `x * y = y * x`
- Associative: `(x * y) * z = x * (y * z)`
- Default type when no specification given

**Example**:
```rust
let x = symbol!(x);           // Scalar symbol
let theta = symbol!(theta);   // Greek letter, still scalar
```

**LaTeX Output**: Standard notation (`x`, `\theta`)

### 2. Matrix (Noncommutative)

Use for linear algebra, matrix equations, transformations.

**Properties**:
- Noncommutative: `A * B ≠ B * A` in general
- Associative: `(A * B) * C = A * (B * C)`
- Inverses require careful ordering

**Example**:
```rust
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
```

**LaTeX Output**: Bold notation (`\mathbf{A}`, `\mathbf{B}`)

### 3. Operator (Noncommutative)

Use for quantum mechanics, differential operators, functional analysis.

**Properties**:
- Noncommutative: `[x, p] = xp - px ≠ 0`
- Represents physical observables or mathematical operations
- Order reflects measurement or operation sequence

**Example**:
```rust
let x_op = symbol!(x; operator);  // Position operator
let p = symbol!(p; operator);      // Momentum operator
```

**LaTeX Output**: Hat notation (`\hat{x}`, `\hat{p}`)

### 4. Quaternion (Noncommutative)

Use for 3D rotations, spatial orientation, graphics programming.

**Properties**:
- Noncommutative: `i * j = k`, but `j * i = -k`
- Basis: `{1, i, j, k}` with `i² = j² = k² = ijk = -1`
- Compact rotation representation

**Example**:
```rust
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);
let k = symbol!(k; quaternion);
```

**LaTeX Output**: Standard notation (`i`, `j`, `k`)

## Creating Symbols

### Single Symbol Creation

Use `symbol!()` macro with optional type:

```rust
// Scalars (default)
let x = symbol!(x);
let y = symbol!(y);

// Matrices
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Operators
let p = symbol!(p; operator);
let H = symbol!(H; operator);

// Quaternions
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);
```

### Bulk Symbol Creation

Use `symbols![]` macro for multiple symbols:

```rust
// Multiple scalars
let scalars = symbols![x, y, z];

// Multiple matrices
let matrices = symbols![A, B, C => matrix];

// Multiple operators
let operators = symbols![p, x, H => operator];

// Multiple quaternions
let quats = symbols![i, j, k => quaternion];

// Access via indexing
let x = &scalars[0];
let y = &scalars[1];
```

## Usage Examples

### Matrix Equations

#### Left Division: A*X = B

```rust
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Equation: A*X = B → A*X - B = 0
let equation = Expression::add(vec![
    Expression::mul(vec![
        Expression::symbol(A),
        Expression::symbol(X.clone())
    ]),
    Expression::mul(vec![Expression::integer(-1), Expression::symbol(B)]),
]);

let solver = MatrixEquationSolver::new();
let result = solver.solve(&equation, &X);
// Solution: X = A^(-1)*B (inverse on the LEFT)
```

#### Right Division: X*A = B

```rust
// Equation: X*A = B → X*A - B = 0
let equation = Expression::add(vec![
    Expression::mul(vec![
        Expression::symbol(X.clone()),
        Expression::symbol(A)
    ]),
    Expression::mul(vec![Expression::integer(-1), Expression::symbol(B)]),
]);

let result = solver.solve(&equation, &X);
// Solution: X = B*A^(-1) (inverse on the RIGHT)
```

### Quantum Operators

Position and momentum commutator:

```rust
let x = symbol!(x; operator);
let p = symbol!(p; operator);

// Commutator: [x, p] = xp - px
let xp = Expression::mul(vec![
    Expression::symbol(x.clone()),
    Expression::symbol(p.clone())
]);

let px = Expression::mul(vec![
    Expression::symbol(p.clone()),
    Expression::symbol(x.clone())
]);

let commutator = Expression::add(vec![
    xp,
    Expression::mul(vec![Expression::integer(-1), px])
]);

// Format as LaTeX
let latex = commutator.to_latex(LaTeXContext::default()).unwrap();
// Output: \hat{x}\hat{p} - \hat{p}\hat{x}
```

### Quaternion Multiplication

```rust
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);

// i*j = k
let ij = Expression::mul(vec![
    Expression::symbol(i.clone()),
    Expression::symbol(j.clone())
]);

// j*i = -k (different!)
let ji = Expression::mul(vec![
    Expression::symbol(j.clone()),
    Expression::symbol(i.clone())
]);

// Order matters
assert_ne!(ij.to_string(), ji.to_string());
```

### LaTeX Parsing with Type Inference

The parser automatically infers types from LaTeX notation:

```rust
use mathhook_core::parser::Parser;
use mathhook_core::parser::config::ParserConfig;

let parser = Parser::new(ParserConfig {
    enable_implicit_multiplication: true,
});

// Matrix equation (bold notation → matrix type)
let eq1 = parser.parse(r"\mathbf{A}\mathbf{X} = \mathbf{B}").unwrap();

// Operator equation (hat notation → operator type)
let eq2 = parser.parse(r"\hat{H}\hat{\psi} = E\hat{\psi}").unwrap();

// Scalar equation (no special notation → scalar type)
let eq3 = parser.parse(r"ax = b").unwrap();
```

## Further Reading

### Detailed Documentation

- **[API Reference](docs/noncommutative_api_reference.md)**: Complete API documentation for all symbol types and functions
- **[Examples](docs/noncommutative_examples.md)**: Extended examples covering quantum mechanics, matrices, and quaternions

### Implementation Details

**Wave-by-Wave Development** (Waves 8-12):

- **Wave 8**: Parser type inference from LaTeX (27 tests)
- **Wave 9**: Symbol creation macros (25 tests)
- **Wave 9.1**: Enhanced bulk creation syntax (37 tests)
- **Wave 10**: Equation solvers with left/right division (41 tests, 10/10)
- **Wave 11**: Educational features and formatting (30 tests)
- **Wave 12**: Integration and examples (25 tests)

**Total**: 185+ tests across all waves, all passing.

### File Structure

**Core implementation**:
- `src/core/symbol.rs` - Symbol type definition
- `src/core/commutativity.rs` - Commutativity logic
- `src/macros/symbols.rs` - Symbol creation macros
- `src/algebra/solvers/matrix_equations.rs` - Equation solver
- `src/formatter/latex.rs` - Type-aware LaTeX formatter
- `src/parser/latex/mod.rs` - Parser with type inference

**Tests**:
- `tests/parser_type_inference_tests.rs` - Parser tests
- `tests/macro_enhancement_tests.rs` - Macro tests
- `tests/matrix_equation_solver_tests.rs` - Solver tests
- `tests/educational_noncommutative_*.rs` - Educational tests
- `tests/noncommutative_integration_*.rs` - Integration tests

**Examples**:
- Run: `cargo run --example noncommutative_algebra_examples`

### Design Principles

1. **Type Safety**: Symbol types enforced at compile time
2. **Zero Cost**: No runtime overhead for type information
3. **Backward Compatible**: Scalar symbols work exactly as before
4. **Educational**: Clear explanations for students
5. **Ergonomic**: Macros make common cases simple

### Support

For questions or issues:
1. Check examples: `cargo run --example noncommutative_algebra_examples`
2. Review detailed documentation in `docs/` directory
3. Run tests: `cargo test noncommutative`
4. Consult CLAUDE.md for implementation details
