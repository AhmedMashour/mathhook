# Noncommutative Algebra in MathHook

This guide provides comprehensive documentation for MathHook's noncommutative algebra support, implemented across Waves 8-12.

## Table of Contents

1. [Overview](#overview)
2. [Symbol Types](#symbol-types)
3. [Creating Symbols](#creating-symbols)
4. [Equation Solving](#equation-solving)
5. [LaTeX Formatting](#latex-formatting)
6. [Educational Features](#educational-features)
7. [Real-World Examples](#real-world-examples)
8. [API Reference](#api-reference)
9. [Implementation Details](#implementation-details)

## Overview

Noncommutative algebra is essential for many areas of mathematics and physics where the order of operations matters. MathHook provides first-class support for noncommutative algebra through:

- **Four symbol types**: Scalar (commutative), Matrix, Operator, Quaternion (all noncommutative)
- **Type-aware parsing**: Automatic type inference from LaTeX notation
- **Smart equation solving**: Distinguishes left division (A*X=B) from right division (X*A=B)
- **Educational features**: Explains why order matters and shows step-by-step solutions
- **Proper LaTeX formatting**: Type-specific notation (bold for matrices, hat for operators)

### Key Insight: Why Order Matters

In commutative algebra (standard arithmetic), order doesn't matter:
- `2 * 3 = 3 * 2 = 6`
- `x * y = y * x`

In noncommutative algebra, order is critical:
- Matrix multiplication: `A * B ≠ B * A` in general
- Quantum operators: `[x, p] = xp - px = iℏ ≠ 0`
- Quaternions: `i * j = k`, but `j * i = -k`

MathHook's type system ensures correct handling of both cases.

## Symbol Types

MathHook supports four symbol types, each with different commutativity properties:

### 1. Scalar (Default - Commutative)

**Use for**: Real numbers, complex numbers, standard variables

**Properties**:
- Commutative: `x * y = y * x`
- Associative: `(x * y) * z = x * (y * z)`
- Default type when no specification given

**Examples**:
```rust
use mathhook_core::symbol;

let x = symbol!(x);           // Scalar symbol
let theta = symbol!(theta);   // Greek letter, still scalar
let alpha = symbol!(alpha);   // Another scalar
```

**LaTeX Output**: Standard notation (no special formatting)
- `x` → `x`
- `theta` → `\theta`

### 2. Matrix (Noncommutative)

**Use for**: Linear algebra, matrix equations, transformations

**Properties**:
- Noncommutative: `A * B ≠ B * A` in general
- Associative: `(A * B) * C = A * (B * C)`
- Inverses require careful ordering

**Examples**:
```rust
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let X = symbol!(X; matrix);
```

**LaTeX Output**: Bold notation
- `A` → `\mathbf{A}`
- `X` → `\mathbf{X}`

### 3. Operator (Noncommutative)

**Use for**: Quantum mechanics, differential operators, functional analysis

**Properties**:
- Noncommutative: `[x, p] = xp - px ≠ 0`
- Represents physical observables or mathematical operations
- Order reflects measurement or operation sequence

**Examples**:
```rust
let x_op = symbol!(x; operator);  // Position operator
let p = symbol!(p; operator);      // Momentum operator
let H = symbol!(H; operator);      // Hamiltonian
```

**LaTeX Output**: Hat notation
- `x` → `\hat{x}`
- `p` → `\hat{p}`
- `H` → `\hat{H}`

### 4. Quaternion (Noncommutative)

**Use for**: 3D rotations, spatial orientation, graphics programming

**Properties**:
- Noncommutative: `i * j = k`, but `j * i = -k`
- Basis: `{1, i, j, k}` with `i² = j² = k² = ijk = -1`
- Compact rotation representation

**Examples**:
```rust
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);
let k = symbol!(k; quaternion);
```

**LaTeX Output**: Standard notation (quaternion context clear from usage)
- `i` → `i`
- `j` → `j`
- `k` → `k`

## Creating Symbols

### Single Symbol Creation

Use the `symbol!()` macro for individual symbols:

```rust
use mathhook_core::symbol;

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

Use the `symbols![]` macro for multiple symbols of the same type:

```rust
use mathhook_core::symbols;

// Multiple scalars
let scalars = symbols![x, y, z];
assert_eq!(scalars.len(), 3);

// Multiple matrices
let matrices = symbols![A, B, C => matrix];

// Multiple operators
let operators = symbols![p, x, H => operator];

// Multiple quaternions
let quats = symbols![i, j, k => quaternion];
```

Note: `symbols![]` returns a `Vec<Symbol>`, so access via indexing:

```rust
let syms = symbols![x, y, z];
let x = &syms[0];
let y = &syms[1];
let z = &syms[2];
```

## Equation Solving

MathHook's equation solver understands noncommutative algebra and distinguishes left from right division.

### Left Division: A*X = B

When the unknown appears on the RIGHT of multiplication:

```rust
use mathhook_core::{symbol, Expression};
use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
use mathhook_core::algebra::solvers::EquationSolver;

let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Equation: A*X = B
let equation = Expression::add(vec![
    Expression::mul(vec![
        Expression::symbol(A),
        Expression::symbol(X.clone())
    ]),
    Expression::mul(vec![Expression::integer(-1), Expression::symbol(B)]),
]);

let solver = MatrixEquationSolver::new();
let result = solver.solve(&equation, &X);
// Solution: X = A^(-1) * B (inverse on the LEFT)
```

**Educational Note**: We multiply both sides by A⁻¹ on the LEFT:
```
A*X = B
A^(-1)*(A*X) = A^(-1)*B
(A^(-1)*A)*X = A^(-1)*B
I*X = A^(-1)*B
X = A^(-1)*B
```

### Right Division: X*A = B

When the unknown appears on the LEFT of multiplication:

```rust
// Equation: X*A = B
let equation = Expression::add(vec![
    Expression::mul(vec![
        Expression::symbol(X.clone()),
        Expression::symbol(A)
    ]),
    Expression::mul(vec![Expression::integer(-1), Expression::symbol(B)]),
]);

let result = solver.solve(&equation, &X);
// Solution: X = B * A^(-1) (inverse on the RIGHT)
```

**Educational Note**: We multiply both sides by A⁻¹ on the RIGHT:
```
X*A = B
(X*A)*A^(-1) = B*A^(-1)
X*(A*A^(-1)) = B*A^(-1)
X*I = B*A^(-1)
X = B*A^(-1)
```

### Why This Matters

For matrices A and B, generally:
- `A^(-1) * B ≠ B * A^(-1)`
- Wrong order gives wrong solution!

The solver automatically determines which division to use based on where the unknown appears.

## LaTeX Formatting

MathHook's LaTeX formatter produces type-appropriate notation:

### Parsing LaTeX with Type Inference

The parser infers symbol types from LaTeX notation:

```rust
use mathhook_core::parser::latex::parse_latex;

// Matrix equation (bold notation → matrix type)
let eq1 = parse_latex(r"\mathbf{A}\mathbf{X} = \mathbf{B}").unwrap();

// Operator equation (hat notation → operator type)
let eq2 = parse_latex(r"\hat{H}\hat{\psi} = E\hat{\psi}").unwrap();

// Scalar equation (no special notation → scalar type)
let eq3 = parse_latex(r"ax = b").unwrap();
```

### Formatting Expressions to LaTeX

The formatter uses type-aware notation:

```rust
use mathhook_core::formatter::latex::LatexFormatter;
use mathhook_core::formatter::Formatter;

let A = symbol!(A; matrix);
let p = symbol!(p; operator);
let x = symbol!(x);

let formatter = LatexFormatter::new();

// Matrix: bold notation
let matrix_expr = Expression::symbol(A);
let latex = formatter.format(&matrix_expr);
// Output: \mathbf{A}

// Operator: hat notation
let op_expr = Expression::symbol(p);
let latex = formatter.format(&op_expr);
// Output: \hat{p}

// Scalar: standard notation
let scalar_expr = Expression::symbol(x);
let latex = formatter.format(&scalar_expr);
// Output: x
```

## Educational Features

MathHook provides educational explanations for noncommutative operations:

### Message Registry

Access educational messages via the registry:

```rust
use mathhook_core::educational::messages::MessageKey;
use mathhook_core::educational::registry::EDUCATIONAL_REGISTRY;

// Get explanation for left division
if let Some(msg) = EDUCATIONAL_REGISTRY.get_message(MessageKey::LeftDivisionExplanation) {
    println!("Left division: {}", msg);
}

// Get warning about noncommutativity
if let Some(msg) = EDUCATIONAL_REGISTRY.get_message(MessageKey::NoncommutativeWarning) {
    println!("Warning: {}", msg);
}

// Get operator commutator explanation
if let Some(msg) = EDUCATIONAL_REGISTRY.get_message(MessageKey::OperatorCommutator) {
    println!("Commutator: {}", msg);
}
```

### Available Message Keys

- `LeftDivisionExplanation`: Explains A*X=B → X=A⁻¹*B
- `RightDivisionExplanation`: Explains X*A=B → X=B*A⁻¹
- `NoncommutativeWarning`: General warning about order mattering
- `OperatorCommutator`: Explains commutator relations
- `MatrixMultiplicationOrder`: Explains why AB ≠ BA

## Real-World Examples

See `examples/noncommutative_algebra_examples.rs` for comprehensive examples:

### Example 1: Quantum Mechanics

```rust
// Position and momentum operators don't commute
let x = symbol!(x; operator);
let p = symbol!(p; operator);

// Commutator: [x, p] = xp - px = iℏ
let commutator = /* ... */;

// Solve Hamiltonian eigenvalue equation: H*ψ = E*ψ
```

### Example 2: Matrix Algebra

```rust
// Solve matrix equations with proper division
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Left division: A*X = B → X = A^(-1)*B
// Right division: X*A = B → X = B*A^(-1)
```

### Example 3: Quaternion Rotations

```rust
// Quaternion multiplication order matters
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);

// i*j = k, but j*i = -k
// Rotation formula: v' = q*v*conj(q)
```

Run the examples:
```bash
cargo run --example noncommutative_algebra_examples
```

## API Reference

### Symbol Creation

```rust
// Single symbol with type
symbol!(x)                    // Scalar (default)
symbol!(A; matrix)            // Matrix
symbol!(p; operator)          // Operator
symbol!(i; quaternion)        // Quaternion

// Bulk symbols
symbols![x, y, z]             // Vec of scalars
symbols![A, B, C => matrix]   // Vec of matrices
```

### Symbol Type Checking

```rust
use mathhook_core::core::symbol::SymbolType;

let x = symbol!(x);
assert_eq!(x.symbol_type(), SymbolType::Scalar);

let A = symbol!(A; matrix);
assert_eq!(A.symbol_type(), SymbolType::Matrix);
```

### Commutativity Checking

```rust
use mathhook_core::core::commutativity::Commutativity;

let x = symbol!(x);
assert_eq!(x.commutativity(), Commutativity::Commutative);

let A = symbol!(A; matrix);
assert_eq!(A.commutativity(), Commutativity::Noncommutative);
```

### Equation Solving

```rust
use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
use mathhook_core::algebra::solvers::{EquationSolver, SolverResult};

let solver = MatrixEquationSolver::new();
let result = solver.solve(&equation, &variable);

match result {
    SolverResult::Single(solution) => println!("Solution: {}", solution),
    SolverResult::Multiple(solutions) => println!("Multiple solutions"),
    SolverResult::NoSolution => println!("No solution"),
}
```

## Implementation Details

### Wave-by-Wave Implementation

The noncommutative algebra support was implemented across five waves:

**Wave 8: Parser Type Inference** (27 tests)
- LaTeX notation parsing: `\mathbf{A}`, `\hat{p}`
- Automatic type inference from notation
- Symbol type propagation through expressions

**Wave 9: Symbol Creation Macros** (25 tests)
- `symbol!()` macro with type specification
- String-based symbol creation
- Type system integration

**Wave 9.1: Enhanced Syntax** (37 tests)
- `symbols![]` bulk creation macro
- Comma-separated identifier syntax
- Type inference with arrow syntax

**Wave 10: Equation Solvers** (41 tests, 10/10 perfect score)
- Left division: A*X=B → X=A⁻¹*B
- Right division: X*A=B → X=B*A⁻¹
- Mixed equations with scalars and matrices
- Integration with MathSolver API

**Wave 11: Educational & Formatting** (30 tests)
- Type-aware LaTeX formatting
- Educational message registry
- Step-by-step explanations
- Operator hat notation, matrix bold notation

**Wave 12: Examples & Integration** (23+ tests)
- Real-world examples (quantum, matrices, quaternions)
- Cross-wave integration testing
- Regression prevention
- Comprehensive documentation

**Total**: 183+ tests across all waves

### File Structure

Core implementation files:
- `src/core/symbol.rs` - Symbol type definition and methods
- `src/core/commutativity.rs` - Commutativity enum and logic
- `src/macros/symbols.rs` - Symbol creation macros
- `src/algebra/solvers/matrix_equations.rs` - Matrix equation solver
- `src/formatter/latex.rs` - Type-aware LaTeX formatter
- `src/educational/messages.rs` - Educational message registry
- `src/parser/latex/mod.rs` - LaTeX parser with type inference

Test files:
- `tests/parser_type_inference_tests.rs` - Parser tests
- `tests/macro_enhancement_tests.rs` - Macro tests
- `tests/matrix_equation_solver_tests.rs` - Solver tests
- `tests/educational_noncommutative_messages_tests.rs` - Educational tests
- `tests/noncommutative_integration_tests.rs` - Integration tests

Examples:
- `examples/noncommutative_algebra_examples.rs` - Comprehensive examples

### Design Principles

1. **Type Safety**: Symbol types enforced at compile time via Rust's type system
2. **Zero Cost**: Type information doesn't add runtime overhead
3. **Backward Compatible**: Scalar symbols work exactly as before
4. **Educational**: Clear explanations for why order matters
5. **Ergonomic**: Macros make common cases simple and clear

### Performance

Noncommutative symbols have identical performance to commutative symbols:
- Symbol creation: O(1) with string interning
- Type checking: O(1) via enum match
- Commutativity check: O(1) inline function
- No runtime overhead for type system

## Further Reading

- Examples: `cargo run --example noncommutative_algebra_examples`
- Tests: `cargo test noncommutative`
- Source: `crates/mathhook-core/src/core/symbol.rs`
- CLAUDE.md: Implementation guidelines and architecture

## Support

For questions or issues:
1. Check the examples in `examples/noncommutative_algebra_examples.rs`
2. Review test cases for usage patterns
3. Consult CLAUDE.md for implementation details
