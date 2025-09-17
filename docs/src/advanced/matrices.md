# Matrix Operations

> 📍 **You are here:** Advanced > Matrix Operations
>
> **Related Topics:** [Noncommutative Algebra](noncommutative-algebra.md) | [System Solving](system-solving.md) | [Symbols & Numbers](../core/symbols-numbers.md)
>
> **Skill Level:** ⭐⭐ Intermediate to ⭐⭐⭐ Advanced

Work with matrices symbolically and numerically in MathHook, with full support for noncommutative algebra where order matters.

## Quick Start (⭐⭐ Start here)

Create and manipulate matrices using MathHook's noncommutative symbol system:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// Create matrix symbols (noncommutative)
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let X = symbol!(X; matrix);

// Matrix multiplication - ORDER MATTERS!
let product_ab = expr!(A * B);  // A*B
let product_ba = expr!(B * A);  // B*A ≠ A*B in general!

// Matrix equation: A*X = B
let equation = expr!(A * X);  // Solve for X
// Solution: X = A^(-1)*B (left division)

// Create numeric matrix
let matrix = Expression::matrix(vec![
    vec![expr!(1), expr!(2)],
    vec![expr!(3), expr!(4)],
]);
```

## Table of Contents

- [Understanding Matrices](#understanding-matrices)
- [Matrix Symbol Types](#matrix-symbol-types)
- [Creating Matrices (⭐⭐ Intermediate)](#creating-matrices--intermediate)
- [Matrix Operations (⭐⭐ Intermediate)](#matrix-operations--intermediate)
- [Matrix Equations (⭐⭐⭐ Advanced)](#matrix-equations--advanced)
- [LaTeX Formatting](#latex-formatting)
- [Real-World Applications](#real-world-applications)
- [Common Patterns](#common-patterns)
- [Common Pitfalls](#common-pitfalls)
- [Performance Considerations](#performance-considerations)
- [See Also](#see-also)

## Understanding Matrices

### What Are Matrices? (Plain English)

A matrix is a rectangular array of numbers, symbols, or expressions arranged in rows and columns. Matrices are fundamental in linear algebra and appear throughout mathematics, science, and engineering.

**Key Property:** Matrix multiplication is **noncommutative** - changing the order changes the result:

$$A \times B \neq B \times A \text{ (in general)}$$

This is different from ordinary numbers where $2 \times 3 = 3 \times 2$.

### Mathematical Background

**Matrix Multiplication:**

For matrices $A_{m \times n}$ and $B_{n \times p}$, the product $C = AB$ has dimensions $m \times p$ with entries:

$$C_{ij} = \sum_{k=1}^{n} A_{ik} B_{kj}$$

**Important:** The number of columns in $A$ must equal the number of rows in $B$.

**Matrix Inverse:**

For square matrix $A$, if inverse $A^{-1}$ exists:

$$A \times A^{-1} = A^{-1} \times A = I$$

where $I$ is the identity matrix.

**Determinant:**

For $2 \times 2$ matrix:

$$\det\begin{pmatrix} a & b \\ c & d \end{pmatrix} = ad - bc$$

### When to Use Matrices

- **Linear systems:** Solve $Ax = b$ for vector $x$
- **Transformations:** Rotations, scaling, projections in graphics
- **Quantum mechanics:** Operators represented as matrices
- **Network analysis:** Adjacency matrices for graphs
- **Economics:** Input-output models
- **Statistics:** Covariance matrices, regression

## Matrix Symbol Types

MathHook distinguishes between scalar and matrix symbols to handle noncommutative algebra correctly.

### Creating Matrix Symbols

Use `symbol!(name; matrix)` to create a matrix symbol:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Matrix symbols (noncommutative)
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let C = symbol!(C; matrix);

// Scalar symbols (commutative) - for comparison
let x = symbol!(x);
let y = symbol!(y);

// Scalars commute: x*y = y*x
// Matrices DON'T: A*B ≠ B*A
```

### Why Type Matters

Matrix symbols have special properties:

1. **Noncommutative multiplication:** $AB \neq BA$
2. **LaTeX formatting:** Rendered as $\mathbf{A}$ (bold)
3. **Equation solving:** Left vs right division distinguished
4. **Educational messages:** Order-aware explanations

See [Noncommutative Algebra](noncommutative-algebra.md) for complete details.

## Creating Matrices (⭐⭐ Intermediate)

### From Expressions

Create matrices from expression arrays:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// 2×2 matrix with symbolic entries
let matrix = Expression::matrix(vec![
    vec![expr!(x), expr!(1)],
    vec![expr!(0), expr!(x)],
]);

// 3×3 identity matrix
let identity = Expression::identity_matrix(3);

// Zero matrix
let zero = Expression::zero_matrix(2, 3);  // 2 rows, 3 columns
```

### From Arrays

Create numeric matrices directly:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// Integer matrix
let int_matrix = Expression::matrix(vec![
    vec![expr!(1), expr!(2)],
    vec![expr!(3), expr!(4)],
]);

// Rational matrix (exact fractions)
let rational_matrix = Expression::matrix(vec![
    vec![Expression::rational(1, 2), Expression::rational(1, 3)],
    vec![Expression::rational(1, 4), Expression::rational(1, 5)],
]);
```

### Special Matrices

Common matrix types:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// Identity matrix
let I = Expression::identity_matrix(3);
// [[1, 0, 0],
//  [0, 1, 0],
//  [0, 0, 1]]

// Diagonal matrix
let diag = Expression::diagonal_matrix(vec![
    expr!(1),
    expr!(2),
    expr!(3),
]);
// [[1, 0, 0],
//  [0, 2, 0],
//  [0, 0, 3]]

// Zero matrix
let zero = Expression::zero_matrix(2, 2);
```

## Matrix Operations (⭐⭐ Intermediate)

### Addition and Subtraction

Matrices add element-wise (must have same dimensions):

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Matrix addition (commutative)
let sum = expr!(A + B);  // A + B = B + A

// Matrix subtraction
let diff = expr!(A - B);  // A - B
```

### Scalar Multiplication

Multiply matrix by scalar:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);

// Scalar multiplication (commutative with scalars)
let scaled = expr!(2 * A);  // 2A = A*2
let scaled2 = expr!(A * 3);  // Same result

let x = symbol!(x);
let symbolic_scale = expr!(x * A);  // xA
```

### Matrix Multiplication (⭐⭐⭐ Critical!)

**Matrix multiplication is NONCOMMUTATIVE - order matters!**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let C = symbol!(C; matrix);

// Order matters!
let ab = expr!(A * B);  // AB
let ba = expr!(B * A);  // BA ≠ AB in general

// Associative (but not commutative)
let abc_left = expr!((A * B) * C);   // (AB)C
let abc_right = expr!(A * (B * C));  // A(BC)
// (AB)C = A(BC) always

// Mixed scalar-matrix
let x = symbol!(x);
let xAB = expr!(x * A * B);  // x(AB) = (xA)B = A(xB)
```

**Dimension Compatibility:**

For $A_{m \times n}$ and $B_{p \times q}$:
- Product $AB$ exists only if $n = p$ (columns of $A$ = rows of $B$)
- Result has dimensions $m \times q$

### Transpose

Transpose flips rows and columns:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);

// Transpose (reverses order in products)
let A_T = expr!(transpose(A));

// Properties:
// (A^T)^T = A
// (AB)^T = B^T A^T  (order reverses!)
```

### Determinant

Determinant of square matrix:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let A = symbol!(A; matrix);

// Symbolic determinant
let det_A = expr!(det(A));

// Numeric determinant
let matrix = Expression::matrix(vec![
    vec![expr!(1), expr!(2)],
    vec![expr!(3), expr!(4)],
]);
let det = expr!(det(matrix));
// Evaluates to: 1*4 - 2*3 = -2
```

### Inverse

Inverse matrix (if it exists):

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);

// Symbolic inverse
let A_inv = expr!(A ^ (-1));  // A^(-1)

// Properties:
// A * A^(-1) = I
// A^(-1) * A = I
// (AB)^(-1) = B^(-1) A^(-1)  (order reverses!)
```

## Matrix Equations (⭐⭐⭐ Advanced)

### Left Division vs Right Division

**Critical Distinction:** For matrix equations, left and right division are different!

**Left Division:** $AX = B$

Solution: $X = A^{-1}B$ (multiply both sides on LEFT by $A^{-1}$)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let solver = MatrixEquationSolver::new();

let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Equation: A*X = B
let equation = expr!((A * X) - B);

let solution = solver.solve(&equation, &X);
// Returns: X = A^(-1)*B (left division)
```

**Right Division:** $XA = B$

Solution: $X = BA^{-1}$ (multiply both sides on RIGHT by $A^{-1}$)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let solver = MatrixEquationSolver::new();

let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Equation: X*A = B
let equation = expr!((X * A) - B);

let solution = solver.solve(&equation, &X);
// Returns: X = B*A^(-1) (right division)
```

**Why It Matters:**

$$A^{-1}B \neq BA^{-1} \text{ (in general)}$$

The solver automatically determines whether to use left or right division based on where the unknown appears.

### Linear System Solving

Solve system $Ax = b$ where $x$ is a vector:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// Coefficient matrix A
let A = Expression::matrix(vec![
    vec![expr!(2), expr!(1)],
    vec![expr!(1), expr!(-1)],
]);

// Right-hand side vector b
let b = Expression::matrix(vec![
    vec![expr!(5)],
    vec![expr!(1)],
]);

// Solve: x = A^(-1)*b
let A_inv = expr!(A ^ (-1));
let x = expr!(A_inv * b);

// Result: x = [2, 1]^T
```

### Matrix-Valued Functions

Functions with matrix arguments:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);

// Matrix exponential
let exp_A = expr!(exp(A));

// Matrix powers
let A_squared = expr!(A ^ 2);  // A*A
let A_cubed = expr!(A ^ 3);    // A*A*A

// Trace (sum of diagonal elements)
let tr_A = expr!(trace(A));
```

## LaTeX Formatting

Matrix symbols render with bold notation in LaTeX:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use latex::{LaTeXFormatter, LaTeXContext};

let A = symbol!(A; matrix);
let B = symbol!(B; matrix);
let X = symbol!(X; matrix);

// Matrix multiplication
let product = expr!(A * B);
let latex = product.to_latex(None).unwrap();
// Output: \mathbf{A}\mathbf{B}

// Matrix equation
let equation = expr!(A * X);
let latex = equation.to_latex(None).unwrap();
// Output: \mathbf{A}\mathbf{X} = \mathbf{B}

// Renders as: 𝐀𝐗 = 𝐁 (bold in LaTeX)
```

Scalar symbols render normally (not bold):

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use latex::LaTeXContext;

let x = symbol!(x);  // Scalar
let latex = expr!(x).to_latex(None).unwrap();
// Output: x (not bold)
```

## Real-World Applications

### 1. Linear Systems (Engineering)

Solve circuit equations using matrix methods:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// Kirchhoff's voltage law for circuit with 3 loops
// Coefficient matrix (resistances)
let R = Expression::matrix(vec![
    vec![expr!(10), expr!(-5), expr!(0)],
    vec![expr!(-5), expr!(15), expr!(-8)],
    vec![expr!(0), expr!(-8), expr!(20)],
]);

// Voltage sources
let V = Expression::matrix(vec![
    vec![expr!(12)],
    vec![expr!(0)],
    vec![expr!(6)],
]);

// Solve for currents: I = R^(-1)*V
let R_inv = expr!(R ^ (-1));
let currents = expr!(R_inv * V);

println!("Circuit currents: {}", currents);
```

### 2. Quantum Mechanics (Physics)

Pauli matrices for spin-1/2 systems:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// Pauli matrices (basis for quantum operators)
let sigma_x = Expression::matrix(vec![
    vec![expr!(0), expr!(1)],
    vec![expr!(1), expr!(0)],
]);

let i = Expression::i();  // Imaginary unit

let sigma_y = Expression::matrix(vec![
    vec![expr!(0), expr!(-i)],
    vec![i, expr!(0)],
]);

let sigma_z = Expression::matrix(vec![
    vec![expr!(1), expr!(0)],
    vec![expr!(0), expr!(-1)],
]);

// Commutation relations: [σ_x, σ_y] = 2iσ_z
let comm_xy = expr!((sigma_x * sigma_y) - (sigma_y * sigma_x));

let expected = expr!(2 * i * sigma_z);

// Verify commutation relation
let difference = expr!(comm_xy - expected);
let simplified = difference.simplify();
// Should equal zero matrix
```

### 3. Graphics and Robotics (Computer Science)

3D rotation matrices:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let theta = symbol!(theta);

// Rotation matrix around z-axis
let R_z = Expression::matrix(vec![
    vec![expr!(cos(theta)), expr!(-sin(theta)), expr!(0)],
    vec![expr!(sin(theta)), expr!(cos(theta)), expr!(0)],
    vec![expr!(0), expr!(0), expr!(1)],
]);

// Point in 3D space
let point = Expression::matrix(vec![
    vec![expr!(1)],
    vec![expr!(0)],
    vec![expr!(0)],
]);

// Rotate point
let rotated = expr!(R_z * point);

println!("Rotated point: {}", rotated);
// Result: [cos(θ), sin(θ), 0]^T
```

### 4. Network Analysis (Graph Theory)

Adjacency matrix for directed graph:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// Graph: 0→1, 0→2, 1→2, 2→0
let adjacency = Expression::matrix(vec![
    vec![expr!(0), expr!(1), expr!(1)],  // Node 0 connects to 1,2
    vec![expr!(0), expr!(0), expr!(1)],  // Node 1 connects to 2
    vec![expr!(1), expr!(0), expr!(0)],  // Node 2 connects to 0
]);

// Number of paths of length 2: A^2
let paths_2 = expr!(adjacency ^ 2);

// Number of paths of length k: A^k
let k = 3;
let paths_k = expr!(adjacency ^ k);
```

## Common Patterns

### Pattern 1: Solving Linear System

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// System: 2x + y = 5, x - y = 1
let A = Expression::matrix(vec![
    vec![expr!(2), expr!(1)],
    vec![expr!(1), expr!(-1)],
]);

let b = Expression::matrix(vec![
    vec![expr!(5)],
    vec![expr!(1)],
]);

// Solution: x = A^(-1)*b
let A_inv = expr!(A ^ (-1));
let solution = expr!(A_inv * b);
let simplified = solution.simplify();

println!("Solution: {}", simplified);
// Result: [2, 1]^T (so x=2, y=1)
```

### Pattern 2: Matrix Power Series

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let A = symbol!(A; matrix);
let t = symbol!(t);

// Matrix exponential (Taylor series)
// exp(At) = I + At + (At)^2/2! + (At)^3/3! + ...

let term1 = Expression::identity_matrix(2);  // I
let term2 = expr!(A * t);                     // At

let at = expr!(A * t);

let term3 = expr!((at ^ 2) * (1/2));         // (At)^2/2!
let term4 = expr!((at ^ 3) * (1/6));         // (At)^3/3!

let exp_approx = expr!(term1 + term2 + term3 + term4);
```

### Pattern 3: Eigenvalue Problem

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let A = symbol!(A; matrix);
let lambda = symbol!(lambda);
let v = symbol!(v; matrix);  // Eigenvector

// Eigenvalue equation: A*v = λ*v
// Rearrange: (A - λI)*v = 0

let I = Expression::identity_matrix(2);
let lambda_I = expr!(lambda * I);

let A_minus_lambda_I = expr!(A - lambda_I);

// Characteristic polynomial: det(A - λI) = 0
let char_poly = expr!(det(A_minus_lambda_I));

println!("Characteristic polynomial: {}", char_poly);
```

## Common Pitfalls

### ❌ Pitfall 1: Assuming Commutativity

**WRONG:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Assume A*B = B*A (WRONG!)
let result = expr!(A * B);
// This is NOT the same as B*A
```

**CORRECT:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Matrices don't commute!
let ab = expr!(A * B);  // AB
let ba = expr!(B * A);  // BA ≠ AB in general

// Only equal in special cases (e.g., A and B are diagonal)
```

### ❌ Pitfall 2: Wrong Division Side

**WRONG:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Solving A*X = B
let X = expr!(B * A ^ (-1));  // WRONG! This is right division
```

**CORRECT:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// Solving A*X = B (left division)
let X = expr!(A ^ (-1) * B);  // Correct: multiply LEFT by A^(-1)

// Solving X*A = B (right division)
let X_right = expr!(B * A ^ (-1));  // Correct: multiply RIGHT by A^(-1)
```

### ❌ Pitfall 3: Dimension Mismatch

**WRONG:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let A = Expression::matrix(vec![
    vec![expr!(1), expr!(2)],     // 2×2 matrix
    vec![expr!(3), expr!(4)],
]);

let B = Expression::matrix(vec![
    vec![expr!(1)],               // 3×1 matrix
    vec![expr!(2)],
    vec![expr!(3)],
]);

// A*B won't work: 2 columns ≠ 3 rows
let product = expr!(A * B);  // Error!
```

**CORRECT:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let A = Expression::matrix(vec![
    vec![expr!(1), expr!(2), expr!(3)],  // 2×3 matrix
    vec![expr!(4), expr!(5), expr!(6)],
]);

let B = Expression::matrix(vec![
    vec![expr!(1)],                      // 3×1 matrix
    vec![expr!(2)],
    vec![expr!(3)],
]);

// A*B works: 3 columns = 3 rows
let product = expr!(A * B);  // OK: 2×3 * 3×1 = 2×1
```

### ❌ Pitfall 4: Singular Matrix Inverse

**WRONG:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// Singular matrix (determinant = 0)
let A = Expression::matrix(vec![
    vec![expr!(1), expr!(2)],
    vec![expr!(2), expr!(4)],  // Second row = 2 * first row
]);

// Inverse doesn't exist!
let A_inv = expr!(A ^ (-1));  // Error or infinity
```

**CORRECT:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Check determinant first
let A = symbol!(A; matrix);
let det_A = expr!(det(A));

// Only invert if det(A) ≠ 0
if det_A.simplify() != expr!(0) {
    let A_inv = expr!(A ^ (-1));
    // Use inverse
} else {
    // Handle singular case (use pseudoinverse, least squares, etc.)
}
```

### ⚠️ Warning: Transpose Order

Transpose reverses multiplication order:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A; matrix);
let B = symbol!(B; matrix);

// (AB)^T = B^T A^T (order reverses!)
let ab_transpose = expr!(transpose(A * B));
let expected = expr!(transpose(B) * transpose(A));

// These are mathematically equal
```

## Performance Considerations

### Memory and Dimensions

- Small matrices (≤ 4×4): Fast, suitable for symbolic work
- Medium matrices (5×5 to 20×20): Acceptable for numeric computation
- Large matrices (> 20×20): Consider numeric libraries (nalgebra, ndarray)

### Symbolic vs Numeric

**Symbolic matrices:**
- Good for: Algebra, calculus, formal proofs
- Slow for: Large dimensions, repeated numerical evaluation

**Numeric matrices:**
- Good for: Large-scale linear algebra, simulations
- Limited: No symbolic manipulation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// Symbolic (flexible but slower)
let A = symbol!(A; matrix);
let result = expr!((A ^ 2) + (2 * A) + 1);  // A^2 + 2A + 1 (algebraic)

// Numeric (fast but no algebra)
let A_numeric = Expression::matrix(vec![
    vec![Expression::float(1.0), Expression::float(2.0)],
    vec![Expression::float(3.0), Expression::float(4.0)],
]);
let result_numeric = A_numeric.evaluate();  // Numerical
```

### Optimization Tips

1. **Simplify early:** Simplify before multiplication
2. **Cache inverses:** Reuse $A^{-1}$ if solving multiple systems
3. **Use sparsity:** For sparse matrices, store only nonzero entries
4. **Parallel operations:** Element-wise operations can parallelize

## API Reference

### Constructors

- `Expression::matrix(rows)` - Create matrix from expression vectors
- `Expression::identity_matrix(n)` - Create n×n identity matrix
- `Expression::zero_matrix(m, n)` - Create m×n zero matrix
- `Expression::diagonal_matrix(diag)` - Create diagonal matrix

### Operations

- `expr!(A + B)` - Matrix addition
- `expr!(A - B)` - Matrix subtraction
- `expr!(A * B)` - Matrix multiplication (noncommutative!)
- `expr!(c * A)` - Scalar multiplication
- `expr!(A ^ n)` - Matrix power
- `expr!(transpose(A))` - Transpose
- `expr!(det(A))` - Determinant
- `expr!(trace(A))` - Trace (sum of diagonal)

### Solvers

- `MatrixEquationSolver::solve(equation, var)` - Solve matrix equations
  - Automatically handles left vs right division
  - Returns `SolverResult::Single(solution)` if solvable

### Properties

- `matrix.dimensions()` - Get (rows, columns)
- `matrix.is_square()` - Check if square
- `matrix.is_diagonal()` - Check if diagonal
- `matrix.is_symmetric()` - Check if symmetric

## See Also

- **[Noncommutative Algebra](noncommutative-algebra.md)** - Symbol types and order-aware operations
- **[System Solving](system-solving.md)** - Linear and nonlinear system solvers
- **[Equation Solving](../operations/solving.md)** - General equation solving techniques
- **[Symbols & Numbers](../core/symbols-numbers.md)** - Symbol creation and properties
- **[LaTeX Formatting](../parser/formatting.md)** - Output formatting options
- **[API Reference: Matrix API](../api/matrix.md)** - Complete API documentation
