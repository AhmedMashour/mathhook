# Noncommutative Algebra Examples

Examples for MathHook's noncommutative algebra support.

## Quantum Mechanics

### Position and Momentum Operators

The canonical commutation relation: [x, p] = xp - px = iℏ

```rust
use mathhook_core::{symbol, Expression};
use mathhook_core::formatter::latex::LaTeXContext;

let x = symbol!(x; operator);  // Position operator
let p = symbol!(p; operator);  // Momentum operator

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

### Hamiltonian Eigenvalue Equation

Solving H|ψ⟩ = E|ψ⟩

```rust
use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
use mathhook_core::algebra::solvers::EquationSolver;

let H = symbol!(H; operator);     // Hamiltonian
let psi = symbol!(psi; operator); // Wavefunction
let E = symbol!(E; operator);     // Energy

// Equation: H*ψ - E = 0
let equation = Expression::add(vec![
    Expression::mul(vec![
        Expression::symbol(H),
        Expression::symbol(psi.clone())
    ]),
    Expression::mul(vec![Expression::integer(-1), Expression::symbol(E)]),
]);

let solver = MatrixEquationSolver::new();
let result = solver.solve(&equation, &psi);
```

### Angular Momentum Operators

Quantum angular momentum: [Lx, Ly] = iℏLz

```rust
let lx = symbol!(Lx; operator);
let ly = symbol!(Ly; operator);

// Lx*Ly product
let lx_ly = Expression::mul(vec![
    Expression::symbol(lx.clone()),
    Expression::symbol(ly.clone())
]);

// Ly*Lx product
let ly_lx = Expression::mul(vec![
    Expression::symbol(ly.clone()),
    Expression::symbol(lx.clone())
]);

// These are NOT equal (noncommutative)
assert_ne!(lx_ly.to_string(), ly_lx.to_string());
```

## Matrix Algebra

### Solving Matrix Equations

#### Left Division: A*X = B

When the unknown matrix appears on the right:

```rust
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Equation: A*X - B = 0
let equation = Expression::add(vec![
    Expression::mul(vec![
        Expression::symbol(A.clone()),
        Expression::symbol(X.clone())
    ]),
    Expression::mul(vec![Expression::integer(-1), Expression::symbol(B.clone())]),
]);

let solver = MatrixEquationSolver::new();
let result = solver.solve(&equation, &X);

// Solution: X = A^(-1)*B
// Multiply both sides by A^(-1) on the LEFT
```

#### Right Division: X*A = B

When the unknown matrix appears on the left:

```rust
// Equation: X*A - B = 0
let equation = Expression::add(vec![
    Expression::mul(vec![
        Expression::symbol(X.clone()),
        Expression::symbol(A.clone())
    ]),
    Expression::mul(vec![Expression::integer(-1), Expression::symbol(B.clone())]),
]);

let result = solver.solve(&equation, &X);

// Solution: X = B*A^(-1)
// Multiply both sides by A^(-1) on the RIGHT
```

### Matrix Multiplication Order

Matrix multiplication is noncommutative: A*B ≠ B*A in general

```rust
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

let AB = Expression::mul(vec![
    Expression::symbol(A.clone()),
    Expression::symbol(B.clone())
]);

let BA = Expression::mul(vec![
    Expression::symbol(B.clone()),
    Expression::symbol(A.clone())
]);

// These are structurally different
assert_ne!(AB.to_string(), BA.to_string());

// LaTeX formatting preserves order
let ab_latex = AB.to_latex(LaTeXContext::default()).unwrap();
let ba_latex = BA.to_latex(LaTeXContext::default()).unwrap();
// ab_latex: \mathbf{A} \cdot \mathbf{B}
// ba_latex: \mathbf{B} \cdot \mathbf{A}
```

### Mixed Scalar-Matrix Operations

Scalars commute with matrices:

```rust
let a = symbol!(a);  // Scalar
let M = symbol!(M; matrix);

let aM = Expression::mul(vec![
    Expression::symbol(a.clone()),
    Expression::symbol(M.clone())
]);

let Ma = Expression::mul(vec![
    Expression::symbol(M.clone()),
    Expression::symbol(a.clone())
]);

// a*M = M*a (scalar commutes)
// But still tracked as noncommutative expression
```

## Quaternion Rotations

### Quaternion Basis Elements

Quaternion basis: {1, i, j, k} with multiplication rules

```rust
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);
let k = symbol!(k; quaternion);

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

### Quaternion Products

All quaternion products are noncommutative:

```rust
// j*k = i
let jk = Expression::mul(vec![
    Expression::symbol(j.clone()),
    Expression::symbol(k.clone())
]);

// k*i = j
let ki = Expression::mul(vec![
    Expression::symbol(k.clone()),
    Expression::symbol(i.clone())
]);

// i*j = k
let ij = Expression::mul(vec![
    Expression::symbol(i.clone()),
    Expression::symbol(j.clone())
]);
```

### 3D Rotation Formula

Rotating a vector v by quaternion q: v' = q*v*conj(q)

```rust
let q = symbol!(q; quaternion);
let v = symbol!(v; quaternion);  // Vector as pure quaternion

// v' = q*v*q*
let rotation = Expression::mul(vec![
    Expression::symbol(q.clone()),
    Expression::symbol(v.clone()),
    Expression::symbol(q.clone()),  // q* (conjugate)
]);
```

## Bulk Symbol Creation

### Multiple Symbols at Once

Using `symbols![]` macro for multiple symbols:

```rust
use mathhook_core::symbols;

// Multiple scalars
let scalars = symbols![x, y, z];
let x = &scalars[0];
let y = &scalars[1];
let z = &scalars[2];

// Multiple matrices
let matrices = symbols![A, B, C => matrix];
let A = &matrices[0];
let B = &matrices[1];
let C = &matrices[2];

// Multiple operators
let operators = symbols![p, x, h => operator];
let p = &operators[0];
let x_op = &operators[1];
let h = &operators[2];

// Multiple quaternions
let quaternions = symbols![i, j, k => quaternion];
let i = &quaternions[0];
let j = &quaternions[1];
let k = &quaternions[2];
```

### Integration with Equation Solving

```rust
let matrices = symbols![A, X, B => matrix];
let A = &matrices[0];
let X = &matrices[1];
let B = &matrices[2];

let equation = Expression::add(vec![
    Expression::mul(vec![
        Expression::symbol(A.clone()),
        Expression::symbol(X.clone())
    ]),
    Expression::mul(vec![Expression::integer(-1), Expression::symbol(B.clone())]),
]);

let solver = MatrixEquationSolver::new();
let result = solver.solve(&equation, &X);
```

## LaTeX Parsing with Type Inference

### Parsing Matrix Equations

The parser automatically infers types from LaTeX notation:

```rust
use mathhook_core::parser::Parser;
use mathhook_core::parser::config::ParserConfig;

let parser = Parser::new(ParserConfig {
    enable_implicit_multiplication: true,
});

// \mathbf{} → Matrix type
let equation = parser.parse(r"\mathbf{A}\mathbf{X} = \mathbf{B}").unwrap();
// Automatically creates matrix symbols A, X, B
```

### Parsing Operator Equations

```rust
// \hat{} → Operator type
let equation = parser.parse(r"\hat{H}\hat{\psi} = E\hat{\psi}").unwrap();
// Automatically creates operator symbols H, ψ and scalar E
```

## Educational Features

### Getting Explanatory Messages

```rust
use mathhook_core::educational::message_registry::{
    MessageBuilder, MessageCategory, MessageType
};

// Get left division explanation
let msg = MessageBuilder::new(
    MessageCategory::NoncommutativeAlgebra,
    MessageType::LeftMultiplyInverse,
    0
).build();

if let Some(message) = msg {
    println!("Step {}: {}", message.step, message.description);
}
```

### Complete Workflow Example

```rust
// 1. Create symbols
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// 2. Build equation: A*X = B
let equation = Expression::add(vec![
    Expression::mul(vec![
        Expression::symbol(A.clone()),
        Expression::symbol(X.clone())
    ]),
    Expression::mul(vec![Expression::integer(-1), Expression::symbol(B.clone())]),
]);

// 3. Solve equation
let solver = MatrixEquationSolver::new();
let result = solver.solve(&equation, &X);

// 4. Format solution as LaTeX
if let SolverResult::Single(solution) = result {
    let latex = solution.to_latex(LaTeXContext::default()).unwrap();
    println!("Solution: {}", latex);
    // Output: \mathbf{A}^{-1} \cdot \mathbf{B}
}

// 5. Get educational explanation
let msg = MessageBuilder::new(
    MessageCategory::NoncommutativeAlgebra,
    MessageType::LeftMultiplyInverse,
    0
).build();
```

## Best Practices

### Always Use Macros for Symbol Creation

```rust
// Good: Use symbol!() macro
let A = symbol!(A; matrix);

// Avoid: Direct constructor (less ergonomic)
let A = Symbol::matrix("A");
```

### Specify Types Explicitly

```rust
// Good: Explicit type prevents confusion
let p = symbol!(p; operator);   // Clearly an operator

// Unclear: Is 'p' a scalar or operator?
let p = symbol!(p);  // Defaults to scalar!
```

### Use Bulk Creation for Related Symbols

```rust
// Good: All matrices created together
let matrices = symbols![A, B, C, D, E => matrix];

// Tedious: One by one
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let C = symbol!(C; matrix);
```

### Format Output for Readability

```rust
let expr = /* ... */;
let latex = expr.to_latex(LaTeXContext::default()).unwrap();
println!("LaTeX: {}", latex);
```
