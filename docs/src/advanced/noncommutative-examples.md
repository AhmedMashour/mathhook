# Noncommutative Algebra Examples

Examples for MathHook's noncommutative algebra support.

## Quantum Mechanics

### Position and Momentum Operators

The canonical commutation relation: [x, p] = xp - px = iℏ

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use latex::LaTeXContext;

let x = symbol!(x; operator);  // Position operator
let p = symbol!(p; operator);  // Momentum operator

// Commutator: [x, p] = xp - px
let xp = expr!(x * p);
let px = expr!(p * x);
let commutator = expr!(xp - px);

// Format as LaTeX
let latex = commutator.to_latex(None).unwrap();
// Output: \hat{x}\hat{p} - \hat{p}\hat{x}
```

### Hamiltonian Eigenvalue Equation

Solving H|ψ⟩ = E|ψ⟩

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let H = symbol!(H; operator);     // Hamiltonian
let psi = symbol!(psi; operator); // Wavefunction
let E = symbol!(E; operator);     // Energy

// Equation: H*ψ - E = 0
let equation = expr!((H * psi) - E);

let solver = MatrixEquationSolver::new();
let result = solver.solve(&equation, &psi);
```

### Angular Momentum Operators

Quantum angular momentum: [Lx, Ly] = iℏLz

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let lx = symbol!(Lx; operator);
let ly = symbol!(Ly; operator);

// Lx*Ly product
let lx_ly = expr!(lx * ly);

// Ly*Lx product
let ly_lx = expr!(ly * lx);

// These are NOT equal (noncommutative)
assert_ne!(lx_ly.to_string(), ly_lx.to_string());
```

## Matrix Algebra

### Solving Matrix Equations

#### Left Division: A*X = B

When the unknown matrix appears on the right:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Equation: A*X - B = 0
let equation = expr!((A * X) - B);

let solver = MatrixEquationSolver::new();
let result = solver.solve(&equation, &X);

// Solution: X = A^(-1)*B
// Multiply both sides by A^(-1) on the LEFT
```

#### Right Division: X*A = B

When the unknown matrix appears on the left:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Equation: X*A - B = 0
let equation = expr!((X * A) - B);

let result = solver.solve(&equation, &X);

// Solution: X = B*A^(-1)
// Multiply both sides by A^(-1) on the RIGHT
```

### Matrix Multiplication Order

Matrix multiplication is noncommutative: A*B ≠ B*A in general

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

let AB = expr!(A * B);
let BA = expr!(B * A);

// These are structurally different
assert_ne!(AB.to_string(), BA.to_string());

// LaTeX formatting preserves order
let ab_latex = AB.to_latex(None).unwrap();
let ba_latex = BA.to_latex(None).unwrap();
// ab_latex: \mathbf{A} \cdot \mathbf{B}
// ba_latex: \mathbf{B} \cdot \mathbf{A}
```

### Mixed Scalar-Matrix Operations

Scalars commute with matrices:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let a = symbol!(a);  // Scalar
let M = symbol!(M; matrix);

let aM = expr!(a * M);
let Ma = expr!(M * a);

// a*M = M*a (scalar commutes)
// But still tracked as noncommutative expression
```

## Quaternion Rotations

### Quaternion Basis Elements

Quaternion basis: {1, i, j, k} with multiplication rules

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let i = symbol!(i; quaternion);
let j = symbol!(j; quaternion);
let k = symbol!(k; quaternion);

// i*j = k
let ij = expr!(i * j);

// j*i = -k (different!)
let ji = expr!(j * i);

// Order matters
assert_ne!(ij.to_string(), ji.to_string());
```

### Quaternion Products

All quaternion products are noncommutative:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// j*k = i
let jk = expr!(j * k);

// k*i = j
let ki = expr!(k * i);

// i*j = k
let ij = expr!(i * j);
```

### 3D Rotation Formula

Rotating a vector v by quaternion q: v' = q*v*conj(q)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let q = symbol!(q; quaternion);
let v = symbol!(v; quaternion);  // Vector as pure quaternion

// v' = q*v*q* (where q* is the conjugate)
let rotation = expr!(q * v * q);  // Note: needs conjugate representation
```

## Bulk Symbol Creation

### Multiple Symbols at Once

Using `symbols![]` macro for multiple symbols:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
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
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let matrices = symbols![A, X, B => matrix];
let A = &matrices[0];
let X = &matrices[1];
let B = &matrices[2];

// Equation: A*X - B = 0
let equation = expr!((A * X) - B);

let solver = MatrixEquationSolver::new();
let result = solver.solve(&equation, &X);
```

## LaTeX Parsing with Type Inference

### Parsing Matrix Equations

The parser automatically infers types from LaTeX notation:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let parser = Parser::new(ParserConfig {
    enable_implicit_multiplication: true,
});

// \mathbf{} → Matrix type
let equation = parser.parse(r"\mathbf{A}\mathbf{X} = \mathbf{B}").unwrap();
// Automatically creates matrix symbols A, X, B
```

### Parsing Operator Equations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// \hat{} → Operator type
let equation = parser.parse(r"\hat{H}\hat{\psi} = E\hat{\psi}").unwrap();
// Automatically creates operator symbols H, ψ and scalar E
```

## Educational Features

### Getting Explanatory Messages

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::educational::message_registry::{
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
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// 1. Create symbols
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// 2. Build equation: A*X = B
let equation = expr!((A * X) - B);

// 3. Solve equation
let solver = MatrixEquationSolver::new();
let result = solver.solve(&equation, &X);

// 4. Format solution as LaTeX
if let SolverResult::Single(solution) = result {
    let latex = solution.to_latex(None).unwrap();
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
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Good: Use symbol!() macro
let A = symbol!(A; matrix);

// Avoid: Direct constructor (less ergonomic)
let A_old = symbol!(A; matrix);  // Always use macro
```

### Specify Types Explicitly

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Good: Explicit type prevents confusion
let p = symbol!(p; operator);   // Clearly an operator

// Unclear: Is 'p' a scalar or operator?
let p = symbol!(p);  // Defaults to scalar!
```

### Use Bulk Creation for Related Symbols

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Good: All matrices created together
let matrices = symbols![A, B, C, D, E => matrix];

// Tedious: One by one
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let C = symbol!(C; matrix);
```

### Format Output for Readability

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let expr = /* ... */;
let latex = expr.to_latex(None).unwrap();
println!("LaTeX: {}", latex);
```
