# System Solving

> 📍 **You are here:** Advanced > System Solving
>
> **Related Topics:** [Matrix Operations](matrices.md) | [Equation Solving](../operations/solving.md) | [Noncommutative Algebra](noncommutative-algebra.md)
>
> **Skill Level:** ⭐⭐ Intermediate to ⭐⭐⭐ Advanced

Solve systems of equations (linear and nonlinear) with multiple unknowns using substitution, elimination, and matrix methods.

## Quick Start (⭐⭐ Start here)

Solve linear systems using different methods:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;
// System: { 2x + y = 5, x - y = 1 }
let x = symbol!(x);
let y = symbol!(y);

let solver = SystemSolver::new();

// Method 1: Equations as list
let equations = vec![
    expr!(2*x + y - 5),  // 2x + y = 5
    expr!(x - y - 1),    // x - y = 1
];
let vars = vec![x.clone(), y.clone()];

let solution = solver.solve_system(&equations, &vars);
// Result: { x = 2, y = 1 }

// Method 2: Matrix form Ax = b
let A = Expression::matrix(vec![
    vec![expr!(2), expr!(1)],
    vec![expr!(1), expr!(-1)],
]);
let b = Expression::matrix(vec![
    vec![expr!(5)],
    vec![expr!(1)],
]);

// Solution: x = A^(-1)*b
let solution_matrix = expr!(A ^ (-1) * b);
```

## Table of Contents

- [Understanding System Solving](#understanding-system-solving)
- [Linear Systems (⭐⭐ Intermediate)](#linear-systems--intermediate)
- [Matrix Methods (⭐⭐ Intermediate)](#matrix-methods--intermediate)
- [Nonlinear Systems (⭐⭐⭐ Advanced)](#nonlinear-systems--advanced)
- [Underdetermined and Overdetermined Systems](#underdetermined-and-overdetermined-systems)
- [Real-World Applications](#real-world-applications)
- [Common Patterns](#common-patterns)
- [Common Pitfalls](#common-pitfalls)
- [Performance Considerations](#performance-considerations)
- [See Also](#see-also)

## Understanding System Solving

### What Are Systems of Equations? (Plain English)

A system of equations is multiple equations that must all be satisfied simultaneously. The solution is the set of values that makes ALL equations true at once.

**Example:** Linear system
$$\begin{cases}
2x + y = 5 \\
x - y = 1
\end{cases}$$

Solution: $x = 2, y = 1$ (only values that satisfy both equations)

### Mathematical Background

**Linear System:**

General form:
$$\begin{cases}
a_{11}x_1 + a_{12}x_2 + \cdots + a_{1n}x_n = b_1 \\
a_{21}x_1 + a_{22}x_2 + \cdots + a_{2n}x_n = b_2 \\
\vdots \\
a_{m1}x_1 + a_{m2}x_2 + \cdots + a_{mn}x_n = b_m
\end{cases}$$

**Matrix Form:** $Ax = b$ where:
- $A$ = coefficient matrix ($m \times n$)
- $x$ = unknown vector ($n \times 1$)
- $b$ = constant vector ($m \times 1$)

**Number of Solutions:**

1. **Unique solution:** $m = n$ and $\det(A) \neq 0$
2. **No solution:** System inconsistent (contradictory equations)
3. **Infinite solutions:** System underdetermined (more variables than equations or dependent equations)

### When to Use System Solving

- **Physics:** Coupled oscillators, circuit analysis, force balance
- **Economics:** Equilibrium models, supply-demand systems, optimization
- **Engineering:** Structural analysis, control systems, network flows
- **Chemistry:** Reaction networks, stoichiometry, equilibrium
- **Computer Science:** Constraint satisfaction, optimization, graph algorithms

## Linear Systems (⭐⭐ Intermediate)

### Substitution Method

Solve one equation for one variable, substitute into others:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// System: { 2x + y = 5, x - y = 1 }

// Step 1: Solve second equation for x
// x = y + 1

// Step 2: Substitute into first equation
let substituted = expr!(2*(y + 1) + y - 5);
// 2y + 2 + y - 5 = 0
// 3y - 3 = 0

let y_val = expr!(1);  // y = 1

// Step 3: Back-substitute
let x_val = expr!(y_val + 1);  // x = 2

println!("Solution: x = {}, y = {}", x_val, y_val);
```

### Elimination Method

Add/subtract equations to eliminate variables:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// System: { 2x + y = 5, x - y = 1 }

// Equation 1: 2x + y = 5
let eq1 = expr!(2*x + y - 5);

// Equation 2: x - y = 1
let eq2 = expr!(x - y - 1);

// Add equations to eliminate y
// (2x + y) + (x - y) = 5 + 1
// 3x = 6
let combined = expr!(eq1 + eq2);
// x = 2

let x_val = expr!(2);

// Substitute back into eq2
// x - y = 1
// 2 - y = 1
// y = 1
let y_val = expr!(1);

println!("Solution: x = {}, y = {}", x_val, y_val);
```

### Three Variables

Linear system with three unknowns:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);
let z = symbol!(z);

// System: { x + y + z = 6, 2x - y + z = 3, x + 2y - z = 2 }

// Use elimination or matrix method
// This system has solution: x = 1, y = 2, z = 3
```

## Matrix Methods (⭐⭐ Intermediate)

### Direct Matrix Inversion

For square systems ($m = n$):

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);
let y = symbol!(y);

// System in matrix form: Ax = b
let A = Expression::matrix(vec![
    vec![expr!(2), expr!(1)],
    vec![expr!(1), expr!(-1)],
]);

let b = Expression::matrix(vec![
    vec![expr!(5)],
    vec![expr!(1)],
]);

// Solution: x = A^(-1) * b
let A_inv = expr!(A ^ (-1));
let solution = expr!(A_inv * b);

// Simplify to get numeric values
let solution_simplified = solution.simplify();
println!("Solution: {}", solution_simplified);
// Result: [[2], [1]] (x=2, y=1)
```

### Gaussian Elimination

Systematic elimination for larger systems:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// Augmented matrix [A|b]
let augmented = Expression::matrix(vec![
    vec![expr!(2), expr!(1), expr!(5)],   // 2x + y = 5
    vec![expr!(1), expr!(-1), expr!(1)],  // x - y = 1
]);

// Row operations to reach reduced row echelon form
// [1 0 | 2]
// [0 1 | 1]

// Solution: x = 2, y = 1
```

### Matrix Equation Solving

Solve equations with matrix unknowns:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let solver = MatrixEquationSolver::new();

let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Matrix equation: A*X = B
let equation = expr!((A * X) - B);

let solution = solver.solve(&equation, &X);
// Returns: X = A^(-1)*B (left division)

// For X*A = B (right division):
// Returns: X = B*A^(-1)
```

See [Matrix Operations](matrices.md) for complete details on matrix equation solving.

## Nonlinear Systems (⭐⭐⭐ Advanced)

### Substitution for Nonlinear Systems

Solve nonlinear systems by substitution:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// System: { x^2 + y^2 = 25, x + y = 5 }

// Step 1: Solve linear equation for y
// y = 5 - x

// Step 2: Substitute into nonlinear equation
let substituted = expr!(x^2 + (5 - x)^2 - 25);
// x^2 + 25 - 10x + x^2 - 25 = 0
// 2x^2 - 10x = 0
// 2x(x - 5) = 0

// Solutions: x = 0 or x = 5

// Step 3: Find corresponding y values
// If x = 0: y = 5
// If x = 5: y = 0

// Two solutions: (0, 5) and (5, 0)
```

### Newton's Method for Systems

Iterative method for nonlinear systems:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);
let y = symbol!(y);

// System: { f(x,y) = 0, g(x,y) = 0 }
let f = expr!(x^2 + y^2 - 4);
let g = expr!(x - y);

// Jacobian matrix:
// J = [ ∂f/∂x  ∂f/∂y ]
//     [ ∂g/∂x  ∂g/∂y ]

let df_dx = f.derivative(&x, 1);  // 2x
let df_dy = f.derivative(&y, 1);  // 2y
let dg_dx = g.derivative(&x, 1);  // 1
let dg_dy = g.derivative(&y, 1);  // -1

let jacobian = Expression::matrix(vec![
    vec![df_dx, df_dy],
    vec![dg_dx, dg_dy],
]);

// Newton iteration: x_new = x_old - J^(-1) * F(x_old)
// Requires initial guess and iteration
```

### Polynomial Systems

Systems of polynomial equations:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// System: { x^2 - y = 1, x + y^2 = 4 }

// Can have multiple solutions (up to product of degrees)
// This system: degree 2 * degree 2 = up to 4 solutions

// Use substitution or numerical methods
```

## Underdetermined and Overdetermined Systems

### Underdetermined Systems (More Variables than Equations)

System has infinitely many solutions:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);
let z = symbol!(z);

// System: { x + y + z = 5 }  (1 equation, 3 variables)

// General solution (2 free parameters):
// z = t1 (free parameter)
// y = t2 (free parameter)
// x = 5 - t1 - t2

// Infinite solutions: any (x, y, z) where x + y + z = 5
```

**Parametric Solution:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);
let z = symbol!(z);
let t1 = symbol!(t1);  // Free parameter
let t2 = symbol!(t2);  // Free parameter

// General solution:
let x_solution = expr!(5 - t1 - t2);
let y_solution = t1.into();
let z_solution = t2.into();

// Verify: x + y + z = (5 - t1 - t2) + t1 + t2 = 5 ✓
```

### Overdetermined Systems (More Equations than Variables)

System may have no exact solution:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// System: { x + y = 1, 2x + 2y = 3, x - y = 0 }
// (3 equations, 2 variables)

// Equations 1 and 2 are inconsistent:
// If x + y = 1, then 2(x + y) = 2, not 3
// → No exact solution

// Use least squares to find best approximate solution
```

**Least Squares Solution:**

For inconsistent system $Ax \approx b$, minimize $||Ax - b||^2$:

$$x_{\text{LS}} = (A^T A)^{-1} A^T b$$

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let A = Expression::matrix(vec![
    vec![expr!(1), expr!(1)],
    vec![expr!(2), expr!(2)],
    vec![expr!(1), expr!(-1)],
]);

let b = Expression::matrix(vec![
    vec![expr!(1)],
    vec![expr!(3)],
    vec![expr!(0)],
]);

// A^T * A
let AT = expr!(transpose(A));
let ATA = expr!(AT * A);

// (A^T * A)^(-1)
let ATA_inv = expr!(ATA ^ (-1));

// A^T * b
let ATb = expr!(AT * b);

// Least squares solution: (A^T A)^(-1) A^T b
let x_ls = expr!(ATA_inv * ATb);

println!("Least squares solution: {}", x_ls.simplify());
```

## Real-World Applications

### 1. Physics: Coupled Oscillators

Two masses connected by springs:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x1 = symbol!(x1);  // Position of mass 1
let x2 = symbol!(x2);  // Position of mass 2

// Equilibrium equations:
// k1*x1 - k2*(x2 - x1) = 0
// k2*(x2 - x1) - k3*x2 = 0

let k1 = expr!(10);  // Spring constants
let k2 = expr!(20);
let k3 = expr!(15);

let eq1 = expr!(k1*x1 - k2*(x2 - x1));
let eq2 = expr!(k2*(x2 - x1) - k3*x2);

// Solve for equilibrium positions
// System in matrix form:
let A = Expression::matrix(vec![
    vec![expr!(k1 + k2), expr!(-k2)],
    vec![expr!(-k2), expr!(k2 + k3)],
]);

// Right-hand side (external forces)
let F = Expression::matrix(vec![
    vec![expr!(0)],
    vec![expr!(0)],
]);

// Solution: x = A^(-1) * F
let positions = expr!(A ^ (-1) * F);
```

### 2. Economics: Market Equilibrium

Supply and demand in multiple markets:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let p1 = symbol!(p1);  // Price of good 1
let p2 = symbol!(p2);  // Price of good 2

// Supply: S1 = 2p1 - p2, S2 = p2 + p1
// Demand: D1 = 10 - p1, D2 = 12 - 2p2

// Equilibrium: S1 = D1, S2 = D2
// 2p1 - p2 = 10 - p1  →  3p1 - p2 = 10
// p2 + p1 = 12 - 2p2  →  p1 + 3p2 = 12

// System: { 3p1 - p2 = 10, p1 + 3p2 = 12 }

// Matrix form:
let A = Expression::matrix(vec![
    vec![expr!(3), expr!(-1)],
    vec![expr!(1), expr!(3)],
]);

let b = Expression::matrix(vec![
    vec![expr!(10)],
    vec![expr!(12)],
]);

// Equilibrium prices: p = A^(-1) * b
let prices = expr!(A ^ (-1) * b);
// Solution: p1 = 4, p2 = 2.67 (approximately)
```

### 3. Engineering: Structural Analysis

Truss structure with force balance:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// Node forces in equilibrium
// Each node: ΣFx = 0, ΣFy = 0

let F1 = symbol!(F1);  // Force in member 1
let F2 = symbol!(F2);  // Force in member 2
let F3 = symbol!(F3);  // Force in member 3

// Node 1: Horizontal equilibrium
// F1*cos(30°) + F2 = 0

// Node 1: Vertical equilibrium
// F1*sin(30°) - 1000 = 0  (1000 N load)

// Node 2: Horizontal equilibrium
// -F2 + F3*cos(45°) = 0

// Node 2: Vertical equilibrium
// F3*sin(45°) = 0

// System of 4 equations, 3 unknowns (overdetermined)
// Use least squares if inconsistent
```

### 4. Chemistry: Reaction Networks

Chemical equilibrium concentrations:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let A = symbol!(A);  // Concentration of species A
let B = symbol!(B);  // Concentration of species B
let C = symbol!(C);  // Concentration of species C

// Reaction network:
// A ⇌ B (equilibrium constant K1 = 2)
// B ⇌ C (equilibrium constant K2 = 3)

// Conservation of mass:
// A + B + C = 10 (total concentration)

// Equilibrium conditions:
// B/A = K1 = 2  →  B = 2A
// C/B = K2 = 3  →  C = 3B = 6A

// System: { A + B + C = 10, B = 2A, C = 6A }
// Substitute: A + 2A + 6A = 10 → 9A = 10 → A = 10/9

let A_val = expr!(10 / 9);
let B_val = expr!(2 * A_val);
let C_val = expr!(6 * A_val);

println!("Equilibrium: A = {}, B = {}, C = {}", A_val, B_val, C_val);
```

## Common Patterns

### Pattern 1: 2×2 Linear System

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);
let y = symbol!(y);

// General 2×2: { ax + by = e, cx + dy = f }
let a = expr!(2);
let b = expr!(1);
let c = expr!(1);
let d = expr!(-1);
let e = expr!(5);
let f = expr!(1);

// Matrix form
let A = Expression::matrix(vec![
    vec![a.clone(), b.clone()],
    vec![c.clone(), d.clone()],
]);

let rhs = Expression::matrix(vec![
    vec![e.clone()],
    vec![f.clone()],
]);

// Solution
let solution = expr!((A ^ (-1)) * rhs);

// Or use Cramer's rule:
// x = (e*d - b*f) / (a*d - b*c)
// y = (a*f - e*c) / (a*d - b*c)
let det = expr!((a * d) - (b * c));

let x_val = expr!(((e * d) - (b * f)) / det);
let y_val = expr!(((a * f) - (e * c)) / det);
```

### Pattern 2: System with Parameters

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);
let a = symbol!(a);  // Parameter

// System: { x + y = a, x - y = 1 }

// Solution (symbolic):
// 2x = a + 1 → x = (a + 1) / 2
// 2y = a - 1 → y = (a - 1) / 2

let x_solution = expr!((a + 1) / 2);
let y_solution = expr!((a - 1) / 2);

// Verify: x + y = (a+1)/2 + (a-1)/2 = a ✓
//         x - y = (a+1)/2 - (a-1)/2 = 1 ✓
```

### Pattern 3: Homogeneous System

System with zero right-hand side:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);
let y = symbol!(y);
let z = symbol!(z);

// Homogeneous system: { 2x + y - z = 0, x - y + 2z = 0 }

let A = Expression::matrix(vec![
    vec![expr!(2), expr!(1), expr!(-1)],
    vec![expr!(1), expr!(-1), expr!(2)],
]);

// Always has trivial solution: (0, 0, 0)
// Has nontrivial solutions if det(A) = 0 (singular matrix)

let det_A = expr!(det(A));
if det_A.simplify() == expr!(0) {
    println!("Nontrivial solutions exist (nullspace is nontrivial)");
} else {
    println!("Only trivial solution: (0, 0, 0)");
}
```

## Common Pitfalls

### ❌ Pitfall 1: No Solution (Inconsistent System)

**WRONG:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Inconsistent: { x + y = 1, x + y = 2 }
// Cannot satisfy both!

// Trying to solve will fail or give nonsense
```

**CORRECT:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Check for consistency before solving
// x + y = 1 and x + y = 2 are contradictory

// Detect inconsistency:
// Subtract equations: 0 = -1 (contradiction!)

println!("System has no solution (inconsistent)");

// Use least squares for best approximate solution
```

### ❌ Pitfall 2: Infinitely Many Solutions

**WRONG:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Underdetermined: { x + y = 5 }
// Expecting unique solution...
```

**CORRECT:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);
let t = symbol!(t);  // Free parameter

// Underdetermined: { x + y = 5 }
// General solution (1 free parameter):
let x_solution = expr!(5 - t);
let y_solution = t.into();

println!("Infinite solutions: x = 5 - t, y = t (for any t)");
```

### ❌ Pitfall 3: Singular Matrix

**WRONG:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// Singular matrix (det = 0)
let A = Expression::matrix(vec![
    vec![expr!(1), expr!(2)],
    vec![expr!(2), expr!(4)],  // Dependent row
]);

// Cannot invert!
let A_inv = expr!(A ^ (-1));  // Error or infinity
```

**CORRECT:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let A = Expression::matrix(vec![
    vec![expr!(1), expr!(2)],
    vec![expr!(2), expr!(4)],
]);

// Check determinant first
let det_A = expr!(det(A));

if det_A.simplify() == expr!(0) {
    println!("Matrix is singular - system may have no solution or infinite solutions");
    // Use pseudoinverse or least squares
} else {
    let A_inv = expr!(A ^ (-1));
    // Proceed with solution
}
```

### ❌ Pitfall 4: Numerical Instability

**WRONG:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// System with very different scales
// { 0.0001*x + y = 1, x + 10000*y = 2 }
// Direct solution may lose precision
```

**CORRECT:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Scale equations first:
// Multiply first equation by 10000:
// { x + 10000*y = 10000, x + 10000*y = 2 }
// Now equations are comparable in magnitude

// Or use condition number to detect ill-conditioning
```

### ⚠️ Warning: Dependency vs Contradiction

**Dependent equations** (same constraint repeated):
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// { x + y = 5, 2x + 2y = 10 }
// Second is 2 × first → infinite solutions
```

**Contradictory equations** (impossible to satisfy):
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// { x + y = 5, x + y = 6 }
// Cannot have both → no solution
```

## Performance Considerations

### Matrix Size Impact

- **Small systems (2×2, 3×3):** Direct methods fast
- **Medium systems (≤20×20):** Gaussian elimination efficient
- **Large systems (>20×20):** Consider iterative methods (not yet implemented)

### Symbolic vs Numeric

**Symbolic solving:**
- Good for: Small systems, parametric solutions, exact answers
- Slow for: Large systems, complex expressions

**Numeric solving:**
- Good for: Large systems, approximations, speed
- Limited: No symbolic manipulation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Symbolic (exact but slower)
let solution_symbolic = expr!(A ^ (-1) * b);

// Numeric (fast but approximate)
let solution_numeric = solution_symbolic.evaluate();
```

### Sparsity

For sparse matrices (mostly zeros):
- Store only nonzero elements
- Use specialized sparse solvers
- Avoid forming dense matrices

## API Reference

### System Solvers

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let solver = SystemSolver::new();

// Solve system of equations
solver.solve_system(
    equations: &[Expression],  // List of equations (= 0 form)
    variables: &[Symbol],       // Variables to solve for
) -> Result<HashMap<Symbol, Expression>, SolverError>
```

### Matrix Equation Solver

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let solver = MatrixEquationSolver::new();

// Solve A*X = B or X*A = B
solver.solve(
    equation: &Expression,
    variable: &Symbol,
) -> SolverResult
```

### Direct Matrix Methods

- `expr!(A ^ (-1) * b)` - Matrix inversion solution
- `expr!(transpose(A))` - Matrix transpose
- `expr!(det(A))` - Determinant
- Least squares: `(A^T A)^(-1) A^T b`

## See Also

- **[Matrix Operations](matrices.md)** - Matrix algebra and noncommutative multiplication
- **[Equation Solving](../operations/solving.md)** - Single equation solving
- **[Noncommutative Algebra](noncommutative-algebra.md)** - Matrix symbol types
- **[Substitution](../operations/substitution.md)** - Variable substitution
- **[Symbols & Numbers](../core/symbols-numbers.md)** - Creating variables
- **[API Reference: Solver API](../api/solver.md)** - Complete API documentation
