# Solving Equations

> 📍 **You are here:** Operations > Solving
>
> **Related Topics:** [System Solving](system-solving.md) | [Matrices](matrices.md) | [Noncommutative Algebra](../advanced/noncommutative-algebra.md)
>
> **Skill Level:** ⭐ Beginner (linear) | ⭐⭐ Intermediate (quadratic, polynomial) | ⭐⭐⭐ Advanced (transcendental, numerical)

Find solutions to equations symbolically and numerically.

## Quick Start (⭐ Start here if you're new)

Solve a simple equation in 3 lines:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Solve: 2x + 3 = 0
let equation = expr!(2 * x + 3);
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x);
// Result: x = -3/2

println!("Solutions: {:?}", solutions);
```

## Table of Contents

- [Understanding Equation Solving](#understanding-equation-solving)
- [Linear Equations (⭐ Beginner)](#linear-equations--beginner)
- [Quadratic Equations (⭐⭐ Intermediate)](#quadratic-equations--intermediate)
- [Polynomial Equations](#polynomial-equations)
- [Transcendental Equations (⭐⭐⭐ Advanced)](#transcendental-equations--advanced)
- [Matrix Equations](#matrix-equations)
- [Real-World Applications](#real-world-applications)
- [Common Patterns](#common-patterns)
- [Performance Considerations](#performance-considerations)

## Understanding Equation Solving

### What is Equation Solving? (Plain English)

**Equation solving** finds values of variables that make an equation true.

**Examples:**
- `2x + 3 = 0` → `x = -3/2` (one solution)
- `x² - 5x + 6 = 0` → `x = 2` or `x = 3` (two solutions)
- `sin(x) = 0` → `x = 0, π, 2π, ...` (infinitely many solutions)

### Mathematical Background

**Equation Types:**

1. **Linear:** $ax + b = 0$ (solution: $x = -b/a$)
2. **Quadratic:** $ax^2 + bx + c = 0$ (solution: quadratic formula)
3. **Polynomial:** $a_n x^n + \cdots + a_1 x + a_0 = 0$
4. **Transcendental:** Involves trigonometric, exponential, or logarithmic functions
5. **Systems:** Multiple equations with multiple variables

**Quadratic Formula:**

$$x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$$

**Discriminant ($\Delta$):**

$$\Delta = b^2 - 4ac$$

- $\Delta > 0$: Two distinct real roots
- $\Delta = 0$: One repeated real root
- $\Delta < 0$: Two complex conjugate roots

**Reference:** Stewart, *Calculus* 8th ed., Appendix A (Algebra Review)

### When to Use Solving

**Use equation solving for:**
1. **Finding zeros:** Determine where functions equal zero
2. **Intersection points:** Find where two functions are equal
3. **Parameter estimation:** Determine unknown values from constraints
4. **Optimization:** Critical points where derivative equals zero
5. **Physics problems:** Solve for time, velocity, position

**Don't use symbolic solving when:**
- No closed-form solution exists (use numerical methods)
- Expression is too complex (simplify first)
- Only approximate solution needed (numerical is faster)

## Linear Equations (⭐ Beginner)

### Single Variable Linear

Solve `ax + b = 0`:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Solve: 2x + 3 = 0
let eq1 = expr!(2 * x + 3);
let mut solver = MathSolver::new();
let sol1 = solver.solve(&eq1, &x);
// Result: x = -3/2

// Solve: 5x - 10 = 0
let eq2 = expr!(5 * x - 10);
let sol2 = solver.solve(&eq2, &x);
// Result: x = 2

// Solve: x/3 + 1 = 0
let eq3 = expr!(x / 3 + 1);
let sol3 = solver.solve(&eq3, &x);
// Result: x = -3
```

### Multiple Variables (Choose Which to Solve)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Equation: 2x + 3y - 6 = 0
let equation = expr!(2 * x + 3 * y - 6);
let mut solver = MathSolver::new();

// Solve for x: x = (6 - 3y)/2
let solve_x = solver.solve(&equation, &x);
// Result: x = 3 - (3/2)y

// Solve for y: y = (6 - 2x)/3
let solve_y = solver.solve(&equation, &y);
// Result: y = 2 - (2/3)x
```

## Quadratic Equations (⭐⭐ Intermediate)

### Standard Form Quadratic

Solve `ax² + bx + c = 0`:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Solve: x² - 5x + 6 = 0
let eq1 = expr!(x ^ 2 - 5 * x + 6);
let mut solver = MathSolver::new();
let solutions = solver.solve(&eq1, &x);
// Result: [x = 2, x = 3]

// Solve: x² - 4 = 0 (difference of squares)
let eq2 = expr!(x ^ 2 - 4);
let sol2 = solver.solve(&eq2, &x);
// Result: [x = -2, x = 2]

// Solve: 2x² + 3x - 2 = 0
let eq3 = expr!(2 * (x ^ 2) + 3 * x - 2);
let sol3 = solver.solve(&eq3, &x);
// Result: [x = 1/2, x = -2]
```

### Complex Roots

When discriminant is negative:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Solve: x² + 1 = 0 (complex roots)
let equation = expr!(x ^ 2 + 1);
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x);
// Result: [x = i, x = -i]

// Solve: x² - 2x + 5 = 0
// Discriminant: 4 - 20 = -16 < 0 (complex roots)
let eq2 = expr!(x ^ 2 - 2 * x + 5);
let sol2 = solver.solve(&eq2, &x);
// Result: [x = 1 + 2i, x = 1 - 2i]
```

### Repeated Roots

When discriminant is zero:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Solve: x² - 6x + 9 = 0 (discriminant = 0)
// (x - 3)² = 0
let equation = expr!(x ^ 2 - 6 * x + 9);
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x);
// Result: [x = 3] (multiplicity 2)
```

## Polynomial Equations

### Cubic and Higher-Degree

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Solve: x³ - 6x² + 11x - 6 = 0
// Factors: (x - 1)(x - 2)(x - 3)
let cubic = expr!(x ^ 3 - 6 * (x ^ 2) + 11 * x - 6);
let mut solver = MathSolver::new();
let solutions = solver.solve(&cubic, &x);
// Result: [x = 1, x = 2, x = 3]

// Solve: x⁴ - 1 = 0 (difference of fourth powers)
let quartic = expr!(x ^ 4 - 1);
let sol2 = solver.solve(&quartic, &x);
// Result: [x = 1, x = -1, x = i, x = -i]
```

### Factorization Strategy

Use factoring for polynomials:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use expand::Expand;

let x = symbol!(x);

// Solve: x³ - x = 0
// Factor: x(x² - 1) = x(x - 1)(x + 1)
let equation = expr!(x ^ 3 - x);
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x);
// Result: [x = 0, x = 1, x = -1]

// Verify by expansion
let factored = expr!(x * (x - 1) * (x + 1));
let expanded = factored.expand();
assert_eq!(expanded, equation);
```

## Transcendental Equations (⭐⭐⭐ Advanced)

### Trigonometric Equations

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Solve: sin(x) = 0
let eq1 = expr!(sin(x));
let mut solver = MathSolver::new();
let solutions = solver.solve(&eq1, &x);
// Result: [x = 0, x = π, x = 2π, ...] (infinitely many)
// Typically returns: x = n·π where n ∈ ℤ

// Solve: cos(x) = 1/2
let eq2 = expr!(cos(x) - 1 / 2);
let sol2 = solver.solve(&eq2, &x);
// Result: [x = π/3 + 2πn, x = -π/3 + 2πn]

// Solve: tan(x) = 1
let eq3 = expr!(tan(x) - 1);
let sol3 = solver.solve(&eq3, &x);
// Result: [x = π/4 + πn]
```

### Exponential and Logarithmic

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Solve: e^x = 5
let eq1 = expr!(exp(x) - 5);
let mut solver = MathSolver::new();
let sol1 = solver.solve(&eq1, &x);
// Result: x = ln(5)

// Solve: log(x) = 2
let eq2 = expr!(log(x) - 2);
let sol2 = solver.solve(&eq2, &x);
// Result: x = e² (if natural log)

// Solve: 2^x = 8
let eq3 = expr!(2 ^ x - 8);
let sol3 = solver.solve(&eq3, &x);
// Result: x = 3 (since 2³ = 8)
```

### Numerical Methods

When no closed form exists:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Solve: x = cos(x) (transcendental, no closed form)
let equation = expr!(x - cos(x));
let mut solver = MathSolver::new()
    .with_numerical_fallback(true)
    .with_tolerance(1e-10);

let solution = solver.solve(&equation, &x);
// Result: x ≈ 0.739085133... (numerical approximation)

// Newton's method: x_{n+1} = x_n - f(x_n)/f'(x_n)
```

## Matrix Equations

### Left and Right Division

For matrix equations (noncommutative):

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Matrix symbols
let A = symbol!(A; matrix);
let X = symbol!(X; matrix);
let B = symbol!(B; matrix);

// Left division: A*X = B → X = A⁻¹*B
let left_eq = expr!(A * X - B);
let mut solver = MathSolver::new();
let solution_left = solver.solve(&left_eq, &X);
// Result: X = A⁻¹*B

// Right division: X*A = B → X = B*A⁻¹
let right_eq = expr!(X * A - B);
let solution_right = solver.solve(&right_eq, &X);
// Result: X = B*A⁻¹

// Note: A⁻¹*B ≠ B*A⁻¹ for matrices!
```

### Linear Systems (Matrix Form)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// System: 2x + 3y = 8
//         x - y = -1
// Matrix form: [2 3; 1 -1] * [x; y] = [8; -1]

let A = symbol!(A; matrix);  // Coefficient matrix
let x = symbol!(x; matrix);  // Solution vector
let b = symbol!(b; matrix);  // Constant vector

let equation = expr!(A * x - b);
let mut solver = MathSolver::new();
let solution = solver.solve(&equation, &x);
// Result: x = A⁻¹*b
```

## Real-World Applications

### 1. Physics (Projectile Motion)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let t = symbol!(t);
let v0 = symbol!(v0);  // Initial velocity
let h = symbol!(h);    // Initial height

// Position: y(t) = -16t² + v₀t + h
// Find time when projectile hits ground (y = 0)
let position = expr!(-16 * (t ^ 2) + v0 * t + h);

// Substitute values: v₀ = 64 ft/s, h = 80 ft
let position_vals = position.substitute(&v0, &expr!(64))
                             .substitute(&h, &expr!(80));

let mut solver = MathSolver::new();
let times = solver.solve(&position_vals, &t);
// Result: t ≈ 5 seconds (ignoring negative solution)
```

### 2. Economics (Break-Even Analysis)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let q = symbol!(q);  // Quantity

// Revenue: R(q) = 50q
// Cost: C(q) = 1000 + 30q
// Break-even: R(q) = C(q)
let revenue = expr!(50 * q);
let cost = expr!(1000 + 30 * q);
let break_even = expr!(revenue - cost);

let mut solver = MathSolver::new();
let quantity = solver.solve(&break_even, &q);
// Result: q = 50 units
```

### 3. Engineering (Circuit Analysis)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let i = symbol!(i);  // Current
let R = symbol!(R);  // Resistance
let V = symbol!(V);  // Voltage

// Ohm's law: V = I·R
// Given V = 12V, R = 4Ω, find I
let ohms_law = expr!(V - i * R);
let equation = ohms_law.substitute(&V, &expr!(12))
                        .substitute(&R, &expr!(4));

let mut solver = MathSolver::new();
let current = solver.solve(&equation, &i);
// Result: i = 3 A
```

### 4. Finance (Compound Interest)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let t = symbol!(t);
let P = symbol!(P);  // Principal
let A = symbol!(A);  // Amount
let r = symbol!(r);  // Interest rate

// Compound interest: A = P(1 + r)^t
// Find time to double: 2P = P(1 + r)^t
let compound = expr!(A - P * ((1 + r) ^ t));
let double = compound.substitute(&A, &expr!(2 * P));

let mut solver = MathSolver::new();
let time = solver.solve(&double, &t);
// Result: t = log(2)/log(1 + r) (Rule of 72 approximation)
```

## Common Patterns (Cookbook)

### Pattern 1: Check for Multiple Solutions

Always check solution count:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

let equation = expr!(x ^ 2 - 4);
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x);

match solutions.len() {
    0 => println!("No solutions"),
    1 => println!("One solution: {:?}", solutions[0]),
    2 => println!("Two solutions: {:?}, {:?}", solutions[0], solutions[1]),
    _ => println!("{} solutions found", solutions.len()),
}
```

### Pattern 2: Simplify Before Solving

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Complex equation
let equation = expr!((x + 1) * (x - 1) - (x ^ 2 - 1));

// Simplify first
let simplified = equation.simplify();  // → 0
// No need to solve! Already simplified to 0 (true for all x)

// Better: Expand before solving if needed
let equation2 = expr!((x + 1) * (x + 2));
let expanded = equation2.expand();  // x² + 3x + 2
let mut solver = MathSolver::new();
let solutions = solver.solve(&expanded, &x);
```

### Pattern 3: Domain Restrictions

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Solve: √x = 2 (x must be ≥ 0)
let equation = expr!(sqrt(x) - 2);
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x);
// Result: x = 4

// Verify domain: 4 ≥ 0 ✓

// Solve: log(x) = 1 (x must be > 0)
let eq2 = expr!(log(x) - 1);
let sol2 = solver.solve(&eq2, &x);
// Result: x = e (≈ 2.718)
// Verify domain: e > 0 ✓
```

### Pattern 4: Parametric Solutions

Solve in terms of parameters:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let a = symbol!(a);
let b = symbol!(b);

// Solve: ax + b = 0 (parametric in a, b)
let equation = expr!(a * x + b);
let mut solver = MathSolver::new();
let solution = solver.solve(&equation, &x);
// Result: x = -b/a (symbolic solution)

// Now substitute specific values
let specific = solution.substitute(&a, &expr!(2))
                       .substitute(&b, &expr!(6));
// Result: x = -3
```

## Common Pitfalls

### Pitfall 1: Forgetting to Check All Solutions

❌ **WRONG - Using only first solution:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

let equation = expr!(x ^ 2 - 4);
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x);

// WRONG: Only using solutions[0]
let answer = &solutions[0];  // x = -2
// Missed: x = 2 (second solution)
```

✅ **CORRECT - Check all solutions:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let solutions = solver.solve(&equation, &x);

// Check count first
if solutions.is_empty() {
    println!("No solutions");
} else {
    println!("All solutions: {:?}", solutions);  // [-2, 2]
}
```

### Pitfall 2: Domain Violations

❌ **WRONG - Ignoring domain restrictions:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Solve: √(x - 1) = -2
let equation = expr!(sqrt(x - 1) - (-2));
let mut solver = MathSolver::new();
let solution = solver.solve(&equation, &x);
// Result: x = 5

// WRONG: √(5 - 1) = 2, not -2!
// Square root cannot be negative (in real domain)
```

✅ **CORRECT - Verify solutions in original equation:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Always verify by substitution
let check = equation.substitute(&x, &solution);
// If check ≠ 0, solution is invalid
```

### Pitfall 3: Missing Complex Solutions

❌ **WRONG - Expecting real solutions only:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Solve: x² + 1 = 0
let equation = expr!(x ^ 2 + 1);
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x);

// WRONG: Expecting no solutions
// CORRECT: [x = i, x = -i] (complex solutions exist)
```

✅ **CORRECT - Consider complex domain:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// MathHook returns complex solutions by default
// Use .is_complex() to check solution type
```

### Pitfall 4: Numerical Instability

❌ **WRONG - Using symbolic solver for ill-conditioned problems:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Solve: x¹⁰ - 1 = 0 (high-degree, numerical issues)
let equation = expr!(x ^ 10 - 1);
let mut solver = MathSolver::new();
let solutions = solver.solve(&equation, &x);
// May have numerical errors for some roots
```

✅ **CORRECT - Use numerical solver with appropriate tolerance:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let mut solver = MathSolver::new()
    .with_numerical_fallback(true)
    .with_tolerance(1e-12);  // Higher precision

let solutions = solver.solve(&equation, &x);
```

## Performance Considerations

### When Solving is Expensive

**Solving cost depends on:**
1. **Equation degree:** Quadratic < cubic < quartic < higher-degree
2. **Transcendental functions:** Require numerical methods (iterative)
3. **Symbolic complexity:** Large expressions slow solving

**Optimization Strategies:**

1. **Simplify first:**
   ```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
   let simplified = equation.simplify();
   let solutions = solver.solve(&simplified, &x);
   ```

2. **Factor when possible:**
   ```rust
   // x³ - x = x(x² - 1) = x(x-1)(x+1)
   // Solving factored form is faster
   ```

3. **Use numerical for transcendental:**
   ```rust
   // For x = cos(x), numerical is much faster than symbolic
   let solver = MathSolver::new().with_numerical_fallback(true);
   ```

## API Reference

### Methods

```rust
impl MathSolver {
    /// Create new solver
    pub fn new() -> Self;

    /// Solve equation for variable
    pub fn solve(&mut self, equation: &Expression, var: &Symbol) -> Vec<Expression>;

    /// Configure numerical fallback
    pub fn with_numerical_fallback(self, enable: bool) -> Self;

    /// Set tolerance for numerical methods
    pub fn with_tolerance(self, tolerance: f64) -> Self;

    /// Set maximum iterations
    pub fn with_max_iterations(self, max_iter: usize) -> Self;
}
```

### Solving Trait

```rust
pub trait Solve {
    /// Solve equation for variable
    fn solve(&self, var: &Symbol) -> Vec<Expression>;
}

impl Solve for Expression { /* ... */ }
```

## See Also

- **[System Solving](system-solving.md)** - Multiple equations, multiple variables
- **[Matrices](matrices.md)** - Matrix equation solving
- **[Noncommutative Algebra](../advanced/noncommutative-algebra.md)** - Left vs right division
- **[Substitution](substitution.md)** - Verify solutions by substitution
- **[Numerical Methods](../advanced/numerical-methods.md)** - Newton's method, bisection
- **External:** [Quadratic Formula](https://en.wikipedia.org/wiki/Quadratic_formula) (Wikipedia)
- **External:** [Polynomial Root Finding](https://en.wikipedia.org/wiki/Root-finding_algorithms) (Wikipedia)
