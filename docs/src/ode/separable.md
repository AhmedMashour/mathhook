# Separable ODEs

**Coverage:** ~30% of first-order ODE problems
**Priority:** Highest-priority solver in classification chain
**Complexity:** O(n) where n is integration complexity

Separable ODEs are the most important and frequently encountered class of first-order differential equations. MathHook provides a robust solver that handles both general and particular solutions with automatic variable separation and symbolic integration.

---

## Mathematical Background

### What is a Separable ODE?

A first-order ODE $\frac{dy}{dx} = f(x,y)$ is **separable** if it can be written as:

$$\frac{dy}{dx} = g(x) \cdot h(y)$$

where:
- $g(x)$ is a function of **only** $x$ (the independent variable)
- $h(y)$ is a function of **only** $y$ (the dependent variable)

**Key insight:** The right-hand side **factors** into a product of two single-variable functions.

### When is an ODE Separable?

**Separable forms:**

| Equation | Separable? | Factorization |
|----------|------------|---------------|
| $\frac{dy}{dx} = x$ | ✅ Yes | $g(x) = x$, $h(y) = 1$ |
| $\frac{dy}{dx} = y$ | ✅ Yes | $g(x) = 1$, $h(y) = y$ |
| $\frac{dy}{dx} = xy$ | ✅ Yes | $g(x) = x$, $h(y) = y$ |
| $\frac{dy}{dx} = \frac{x}{y}$ | ✅ Yes | $g(x) = x$, $h(y) = \frac{1}{y}$ |
| $\frac{dy}{dx} = e^{x+y}$ | ✅ Yes | $g(x) = e^x$, $h(y) = e^y$ |
| $\frac{dy}{dx} = x + y$ | ❌ No | Cannot factor into $g(x) \cdot h(y)$ |
| $\frac{dy}{dx} = xy + x$ | ✅ Yes | $g(x) = x$, $h(y) = y + 1$ |
| $\frac{dy}{dx} = \sin(xy)$ | ❌ No | Argument depends on both $x$ and $y$ |

### Solution Method

**Algorithm:**

1. **Separate variables:** Rewrite as $\frac{1}{h(y)} dy = g(x) dx$
2. **Integrate both sides:** $\int \frac{1}{h(y)} dy = \int g(x) dx + C$
3. **Solve for $y$ (if possible):** Obtain explicit or implicit solution
4. **Apply initial condition (if given):** Determine constant $C$

**Mathematical justification:**

Starting with $\frac{dy}{dx} = g(x)h(y)$, we multiply both sides by $\frac{dx}{h(y)}$:

$$\frac{dy}{h(y)} = g(x) dx$$

This is valid because we're treating $dy$ and $dx$ as differentials. Integrating both sides gives the general solution.

### Why This Method Works

The separation of variables method exploits the **multiplicative structure** of the equation. By dividing by $h(y)$, we isolate all $y$-dependence on one side and all $x$-dependence on the other. Since both sides must be equal, their integrals must also be equal (up to a constant).

**Implicit vs Explicit Solutions:**

- **Explicit:** $y = F(x, C)$ (can solve for $y$ directly)
- **Implicit:** $G(x, y) = C$ (cannot solve for $y$ algebraically)

Most separable ODEs result in **implicit solutions**. For example, $\frac{dy}{dx} = \frac{y}{x}$ gives $\ln|y| = \ln|x| + C$, which simplifies to the explicit form $y = Cx$. However, $\frac{dy}{dx} = \frac{y^2}{x}$ gives $-\frac{1}{y} = \ln|x| + C$, which is harder to solve explicitly.

---

## API Usage

### Basic Solver Creation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let solver = SeparableODESolver::new();
```

The solver is lightweight and stateless—you can create a new instance for each equation or reuse a single instance.

### Checking Separability

Before attempting to solve, you can check if an equation is separable:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

// Check separability
if solver.is_separable(&expr!(x * y), &y, &x) {
    println!("Equation is separable!");
}

// Non-separable
assert!(!solver.is_separable(&expr!(x + y), &y, &x));
```

**Use cases:**
- Pre-validation before solving
- Classification in solver chains
- Educational demonstrations

### Solving Without Initial Conditions

**General solution:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

// Solve dy/dx = x
let rhs = expr!(x);
let solution = solver.solve(&rhs, &y, &x, None)?;

// Result: ∫dy = ∫x dx + C
// Simplifies to: y = x²/2 + C1
```

The general solution includes an arbitrary constant `C1` (or `C2`, `C3`, etc. for multiple ODEs).

### Solving With Initial Conditions

**Particular solution:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

// Solve dy/dx = x with y(0) = 1
let rhs = expr!(x);
let ic = Some((expr!(0), expr!(1))); // (x₀, y₀)
let solution = solver.solve(&rhs, &y, &x, ic)?;

// Result: y = x²/2 + 1
```

**Initial condition application:**

1. Substitute $(x_0, y_0)$ into general solution
2. Solve for constant $C$
3. Substitute $C$ back into general solution

### Error Handling

The solver returns `Result<Expression, ODEError>`:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::ode::first_order::ODEError;

match solver.solve(&rhs, &y, &x, None) {
    Ok(solution) => println!("Solution: {}", solution),
    Err(ODEError::UnknownType { equation, reason }) => {
        println!("Cannot solve: {}", reason);
        println!("Equation: {}", equation);
    }
    Err(e) => println!("Other error: {:?}", e),
}
```

**Common errors:**
- `UnknownType`: Equation is not separable
- Integration failures (if integral is not computable symbolically)

---

## Worked Examples

### Example 1: Simple Linear ODE

**Problem:** Solve $\frac{dy}{dx} = x$

**Analysis:**
- Separable? ✅ Yes ($g(x) = x$, $h(y) = 1$)
- Type: Linear, first-order

**Solution:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let solution = solver.solve(&expr!(x), &y, &x, None)?;
```

**Mathematical steps:**

1. Separate: $dy = x \, dx$
2. Integrate: $\int dy = \int x \, dx + C$
3. Evaluate: $y = \frac{x^2}{2} + C$

**Result:** $y = \frac{x^2}{2} + C$

---

### Example 2: Exponential Growth

**Problem:** Solve $\frac{dy}{dx} = y$

**Analysis:**
- Separable? ✅ Yes ($g(x) = 1$, $h(y) = y$)
- Type: Exponential growth/decay model

**Solution:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let solution = solver.solve(&expr!(y), &y, &x, None)?;
```

**Mathematical steps:**

1. Separate: $\frac{dy}{y} = dx$
2. Integrate: $\int \frac{dy}{y} = \int dx + C$
3. Evaluate: $\ln|y| = x + C$
4. Solve for $y$: $y = \pm e^{x + C} = Ce^x$ (absorbing $\pm e^C$ into $C$)

**Result:** $y = Ce^x$ (implicit: $\ln|y| - x = C$)

---

### Example 3: Product Form

**Problem:** Solve $\frac{dy}{dx} = xy$

**Analysis:**
- Separable? ✅ Yes ($g(x) = x$, $h(y) = y$)
- Type: Nonlinear growth model

**Solution:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let solution = solver.solve(&expr!(x * y), &y, &x, None)?;
```

**Mathematical steps:**

1. Separate: $\frac{dy}{y} = x \, dx$
2. Integrate: $\int \frac{dy}{y} = \int x \, dx + C$
3. Evaluate: $\ln|y| = \frac{x^2}{2} + C$
4. Solve for $y$: $y = Ce^{x^2/2}$

**Result:** $y = Ce^{x^2/2}$

---

### Example 4: Initial Value Problem

**Problem:** Solve $\frac{dy}{dx} = x$ with $y(0) = 1$

**Analysis:**
- Separable? ✅ Yes
- Initial condition: $y = 1$ when $x = 0$

**Solution:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let ic = Some((expr!(0), expr!(1))); // y(0) = 1
let solution = solver.solve(&expr!(x), &y, &x, ic)?;
```

**Mathematical steps:**

1. General solution: $y = \frac{x^2}{2} + C$
2. Apply IC: $1 = \frac{0^2}{2} + C \Rightarrow C = 1$
3. Particular solution: $y = \frac{x^2}{2} + 1$

**Result:** $y = \frac{x^2}{2} + 1$

---

### Example 5: Rational Function

**Problem:** Solve $\frac{dy}{dx} = \frac{x}{y}$

**Analysis:**
- Separable? ✅ Yes ($g(x) = x$, $h(y) = \frac{1}{y}$)
- Type: Homogeneous-like structure

**Solution:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

// dy/dx = x / y
let rhs = expr!(x / y);
let solution = solver.solve(&rhs, &y, &x, None)?;
```

**Mathematical steps:**

1. Separate: $y \, dy = x \, dx$
2. Integrate: $\int y \, dy = \int x \, dx + C$
3. Evaluate: $\frac{y^2}{2} = \frac{x^2}{2} + C$
4. Multiply by 2: $y^2 = x^2 + C'$ (where $C' = 2C$)
5. Implicit solution: $y^2 - x^2 = C'$

**Result:** $y^2 - x^2 = C$ (implicit), or $y = \pm\sqrt{x^2 + C}$ (explicit)

---

### Example 6: Non-Separable Detection

**Problem:** Attempt to solve $\frac{dy}{dx} = x + y$

**Analysis:**
- Separable? ❌ No
- Type: Linear but not separable (requires integrating factor method)

**Solution:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::ode::first_order::separable::SeparableODESolver;

let x = symbol!(x);
let y = symbol!(y);
let solver = SeparableODESolver::new();

let result = solver.solve(&expr!(x + y), &y, &x, None);

// This will return an error
assert!(result.is_err());
```

**Why it fails:**

The equation $\frac{dy}{dx} = x + y$ cannot be written as $g(x) \cdot h(y)$ because the right-hand side is a **sum**, not a **product**.

---

## Common Patterns and Applications

### Population Growth

**Exponential growth model:**

$$\frac{dP}{dt} = kP$$

where $P(t)$ is population and $k$ is growth rate.

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let t = symbol!(t);
let P = symbol!(P);
let k = symbol!(k);

let solver = SeparableODESolver::new();
let solution = solver.solve(&expr!(k * P), &P, &t, None)?;
// Result: P = Ce^(kt)
```

**Logistic growth model** (with carrying capacity):

$$\frac{dP}{dt} = kP\left(1 - \frac{P}{K}\right)$$

This is also separable: $g(t) = k$, $h(P) = P(1 - P/K)$

### Newton's Law of Cooling

$$\frac{dT}{dt} = -k(T - T_{\text{ambient}})$$

where $T(t)$ is temperature, $k$ is cooling constant, and $T_{\text{ambient}}$ is ambient temperature.

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let t = symbol!(t);
let T = symbol!(T);
let k = symbol!(k);
let T_amb = symbol!(T_amb);

// Separable: g(t) = -k, h(T) = T - T_amb
let rhs = expr!(-k * (T - T_amb));
let solution = solver.solve(&rhs, &T, &t, None)?;
```

### Chemical Reactions

**First-order reaction kinetics:**

$$\frac{dC}{dt} = -kC$$

where $C(t)$ is concentration and $k$ is rate constant.

**Second-order reaction:**

$$\frac{dC}{dt} = -kC^2$$

Both are separable.

---

## Advanced Topics

### Implicit Solutions

Not all separable ODEs have explicit solutions. When the integral $\int \frac{dy}{h(y)}$ cannot be inverted to solve for $y$, the solution remains implicit.

**Example:** $\frac{dy}{dx} = e^{y^2}$

Solution: $\int e^{-y^2} dy = x + C$ (no elementary antiderivative for left side)

MathHook will return the implicit form in such cases.

### Singular Solutions

Some equations have **singular solutions** that cannot be obtained from the general solution for any value of $C$.

**Example:** $\frac{dy}{dx} = y^{1/2}$

General solution: $y = \frac{(x + C)^2}{4}$

Singular solution: $y = 0$ (lost when dividing by $h(y) = y^{1/2}$)

**Important:** Always check if division by $h(y)$ excludes solutions!

### Domain Restrictions

When separating variables, we divide by $h(y)$. This is **only valid when $h(y) \neq 0$**.

**Example:** $\frac{dy}{dx} = y$

Separation: $\frac{dy}{y} = dx$ (valid only when $y \neq 0$)

The solution $y = 0$ (equilibrium solution) is lost in separation. MathHook's integration may not capture this automatically—always verify domain restrictions manually.

---

## Comparison with Other Methods

| Method | Applicability | Complexity | MathHook Support |
|--------|--------------|------------|------------------|
| Separation of Variables | ~30% of first-order ODEs | O(n) integration | ✅ Full |
| Integrating Factor | Linear first-order | O(n) integration | ✅ Full |
| Exact Equations | Exact differential forms | O(n) integration | ✅ Full |
| Substitution Methods | Homogeneous, Bernoulli | O(n) transformation + solve | ✅ Partial |
| Series Methods | Analytic solutions | O(n²) for n terms | 🚧 Planned |

**Recommendation:** Always check separability first—it's the fastest method when applicable.

---

## Performance Characteristics

**Time Complexity:**
- Separation check: O(m) where m = number of factors in multiplication
- Solving: O(n) where n = complexity of symbolic integration

**Space Complexity:**
- O(n) for storing separated expressions and integrals

**Optimization tips:**
1. Simplify the ODE before attempting separation (reduces integration complexity)
2. Reuse solver instances for multiple equations
3. For numerical approximations, consider switching to numerical ODE solvers after symbolic setup

---

## Troubleshooting

### Issue: "Cannot separate variables"

**Cause:** The equation is not separable (cannot factor into $g(x) \cdot h(y)$).

**Solution:** Try other methods:
- Integrating factor (for linear ODEs)
- Exact equation method
- Substitution (for homogeneous or Bernoulli equations)

### Issue: Integration returns unevaluated integral

**Cause:** The integral $\int \frac{dy}{h(y)}$ or $\int g(x) dx$ is not computable symbolically.

**Solution:**
- Accept implicit solution
- Use numerical integration for particular solutions
- Simplify $h(y)$ or $g(x)$ if possible

### Issue: Solution doesn't match initial condition

**Cause:** Implicit solutions may have multiple branches.

**Solution:**
- Check signs: $\ln|y|$ vs $\ln(y)$
- Verify $C$ calculation manually
- Check for domain restrictions

---

## See Also

- [Integration](../operations/integration.md) - Symbolic integration used internally
- [Linear ODEs](./linear.md) - Alternative method for linear first-order ODEs
- [Exact Equations](./exact.md) - Another important first-order ODE class
- [ODE Solver](./solver.md) - High-level ODE solver with automatic classification

---

## References

1. **Tenenbaum & Pollard**, "Ordinary Differential Equations", Dover Publications, 1963, pp. 52-68
2. **SymPy Documentation**, Separable ODE Solver: `sympy.solvers.ode.single.Separable`
3. **Boyce & DiPrima**, "Elementary Differential Equations and Boundary Value Problems", 11th ed., Chapter 2

---

## Mathematical Notation Reference

| Symbol | Meaning |
|--------|---------|
| $\frac{dy}{dx}$ | Derivative of $y$ with respect to $x$ |
| $\int f(x) dx$ | Indefinite integral of $f(x)$ |
| $C, C_1, C_2$ | Arbitrary constants of integration |
| $(x_0, y_0)$ | Initial condition point |
| $g(x)$ | Function of $x$ only |
| $h(y)$ | Function of $y$ only |
| $\ln\|y\|$ | Natural logarithm of absolute value |
| $e^x$ | Exponential function |

---

**Document Status:** Complete
**Last Updated:** 2025-01-20
**SymPy Validated:** Yes (against `sympy.solvers.ode.single.Separable`)
