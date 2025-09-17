# Substitution

> 📍 **You are here:** Operations > Substitution
>
> **Related Topics:** [Solving](solving.md) | [Integration](integration.md) | [Differentiation](differentiation.md)
>
> **Skill Level:** ⭐ Beginner (numerical) | ⭐⭐ Intermediate (expression) | ⭐⭐⭐ Advanced (u-substitution, change of variables)

Replace variables with values or expressions to evaluate, simplify, or transform expressions.

## Quick Start (⭐ Start here if you're new)

Substitute a value in 3 lines:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Substitute x = 2 into x² + 3x
let expression = expr!(x ^ 2 + 3 * x);
let result = expression.substitute(&x, &expr!(2));
// Result: 4 + 6 = 10

println!("{}", result);
```

## Table of Contents

- [Understanding Substitution](#understanding-substitution)
- [Numerical Substitution (⭐ Beginner)](#numerical-substitution--beginner)
- [Expression Substitution (⭐⭐ Intermediate)](#expression-substitution--intermediate)
- [U-Substitution (⭐⭐⭐ Advanced)](#u-substitution--advanced)
- [Change of Variables](#change-of-variables)
- [Real-World Applications](#real-world-applications)
- [Common Patterns](#common-patterns)
- [Performance Considerations](#performance-considerations)

## Understanding Substitution

### What is Substitution? (Plain English)

**Substitution** replaces variables with specific values or other expressions.

**Examples:**
- `x² + 3x` with `x = 2` → `4 + 6 = 10` (numerical substitution)
- `x² + 3x` with `x = y + 1` → `(y+1)² + 3(y+1)` (expression substitution)
- `∫ 2x·e^(x²) dx` with `u = x²` → `∫ e^u du` (u-substitution)

### Mathematical Background

**Function Evaluation:**

$$f(a) = f(x)|_{x=a}$$

Substitute $x = a$ into function $f(x)$.

**Composition:**

$$f(g(x)) = f(u)|_{u=g(x)}$$

Substitute $u = g(x)$ into function $f(u)$.

**U-Substitution (Integration):**

$$\int f(g(x)) \cdot g'(x) \, dx = \int f(u) \, du$$

where $u = g(x)$ and $du = g'(x) \, dx$.

**Change of Variables (Multivariable):**

$$\frac{\partial f}{\partial x} = \frac{\partial f}{\partial u} \cdot \frac{\partial u}{\partial x}$$

Chain rule for variable transformation.

**Reference:** Stewart, *Calculus* 8th ed., Chapter 5 (Integrals) §5.5 (Substitution Rule)

### When to Use Substitution

**Use substitution for:**
1. **Evaluation:** Compute numerical value at specific point
2. **Verification:** Check if value satisfies equation
3. **Simplification:** Replace complex subexpression with simple variable
4. **Integration:** U-substitution to transform difficult integrals
5. **Change of coordinates:** Transform between coordinate systems

**Don't substitute when:**
- Need general symbolic form (substitution loses generality)
- Value would complicate expression further
- Alternative method is simpler (e.g., direct integration)

## Numerical Substitution (⭐ Beginner)

### Single Variable Substitution

Replace variable with number:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Substitute x = 2 into x² + 3x
let expr1 = expr!(x ^ 2 + 3 * x);
let result1 = expr1.substitute(&x, &expr!(2));
// Result: 4 + 6 = 10

// Substitute x = -1 into x³ - 2x + 1
let expr2 = expr!(x ^ 3 - 2 * x + 1);
let result2 = expr2.substitute(&x, &expr!(-1));
// Result: -1 + 2 + 1 = 2

// Substitute x = 0 into sin(x)/x (indeterminate form)
let expr3 = expr!(sin(x) / x);
let result3 = expr3.substitute(&x, &expr!(0));
// Result: undefined (0/0)
// Use limit instead: lim(x→0) sin(x)/x = 1
```

### Multiple Variables

Substitute multiple variables:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Substitute x = 2, y = 3 into x² + y²
let expression = expr!(x ^ 2 + y ^ 2);
let result = expression.substitute(&x, &expr!(2))
                       .substitute(&y, &expr!(3));
// Result: 4 + 9 = 13

// Substitute x = 1, y = -1 into xy + x - y
let expr2 = expr!(x * y + x - y);
let result2 = expr2.substitute(&x, &expr!(1))
                   .substitute(&y, &expr!(-1));
// Result: -1 + 1 - (-1) = 1
```

### Function Evaluation

Evaluate functions at points:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Evaluate sin(x) at x = π/2
let sin_expr = expr!(sin(x));
let sin_val = sin_expr.substitute(&x, &Expression::pi_over_2());
// Result: 1

// Evaluate e^x at x = 1
let exp_expr = expr!(exp(x));
let exp_val = exp_expr.substitute(&x, &expr!(1));
// Result: e (≈ 2.718)

// Evaluate log(x) at x = e
let log_expr = expr!(log(x));
let log_val = log_expr.substitute(&x, &Expression::e());
// Result: 1
```

## Expression Substitution (⭐⭐ Intermediate)

### Replace with Another Expression

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Substitute x = y + 1 into x² + 3x
let expression = expr!(x ^ 2 + 3 * x);
let substituted = expression.substitute(&x, &expr!(y + 1));
// Result: (y+1)² + 3(y+1)

// Expand for cleaner form
let expanded = substituted.expand();
// Result: y² + 2y + 1 + 3y + 3 = y² + 5y + 4

// Substitute x = 2y into x² - y²
let expr2 = expr!(x ^ 2 - y ^ 2);
let sub2 = expr2.substitute(&x, &expr!(2 * y));
// Result: (2y)² - y² = 4y² - y² = 3y²
```

### Parametric Substitution

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let t = symbol!(t);
let x = symbol!(x);
let y = symbol!(y);

// Parametric curve: x = cos(t), y = sin(t)
// Substitute into x² + y² = 1
let circle = expr!(x ^ 2 + y ^ 2);
let parametric = circle.substitute(&x, &expr!(cos(t)))
                       .substitute(&y, &expr!(sin(t)));
// Result: cos²(t) + sin²(t)

// Simplify using trigonometric identity
let simplified = parametric.simplify();
// Result: 1 (verifies circle equation)
```

### Transformation of Coordinates

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Cartesian to polar: x = r·cos(θ), y = r·sin(θ)
let x = symbol!(x);
let y = symbol!(y);
let r = symbol!(r);
let theta = symbol!(theta);

// Transform x² + y² into polar
let cartesian = expr!(x ^ 2 + y ^ 2);
let polar = cartesian.substitute(&x, &expr!(r * cos(theta)))
                     .substitute(&y, &expr!(r * sin(theta)));
// Result: r²·cos²(θ) + r²·sin²(θ)

let simplified = polar.simplify();
// Result: r² (using cos²+sin²=1)
```

## U-Substitution (⭐⭐⭐ Advanced)

### Basic U-Substitution

Transform difficult integrals:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);
let u = symbol!(u);

// Integrate: ∫ 2x·e^(x²) dx
// Let u = x², then du = 2x dx
let integrand = expr!(2 * x * exp(x ^ 2));

// Manual substitution
let u_expr = expr!(x ^ 2);  // u = x²
let integrand_u = integrand.substitute(&expr!(x ^ 2), &u);
// Result: ∫ e^u du = e^u + C

// Back-substitute: e^(x²) + C
let result = expr!(exp(u)).substitute(&u, &u_expr);
// Result: e^(x²) + C
```

### Chain Rule Recognition

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);
let u = symbol!(u);

// Integrate: ∫ sin(x)·cos(x) dx
// Let u = sin(x), then du = cos(x) dx
let integrand = expr!(sin(x) * cos(x));

// Substitute u = sin(x)
let integrand_u = integrand.substitute(&expr!(sin(x)), &u);
// Result: ∫ u du = u²/2 + C

// Back-substitute
let result = expr!(u ^ 2 / 2).substitute(&u, &expr!(sin(x)));
// Result: sin²(x)/2 + C
```

### Inverse Substitution

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use integrals::Integration;

let x = symbol!(x);
let u = symbol!(u);

// Integrate: ∫ 1/(1+x²) dx
// Let x = tan(u), then dx = sec²(u) du
// 1 + x² = 1 + tan²(u) = sec²(u)
let integrand = expr!(1 / (1 + x ^ 2));

// After substitution: ∫ sec²(u)/sec²(u) du = ∫ 1 du = u + C
// Back-substitute: u = arctan(x)
let result = expr!(atan(x));
// Result: arctan(x) + C
```

## Change of Variables

### Polar Coordinates

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);
let r = symbol!(r);
let theta = symbol!(theta);

// Transform ∂f/∂x to polar coordinates
// x = r·cos(θ), y = r·sin(θ)
let f = expr!(x ^ 2 + y ^ 2);  // Example function

// Partial derivative ∂f/∂x in Cartesian
let df_dx = f.differentiate(&x);
// Result: 2x

// Convert to polar
let df_dx_polar = df_dx.substitute(&x, &expr!(r * cos(theta)))
                       .substitute(&y, &expr!(r * sin(theta)));
// Result: 2r·cos(θ)
```

### Jacobian Transformation

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Change of variables: u = x + y, v = x - y
let x = symbol!(x);
let y = symbol!(y);
let u = symbol!(u);
let v = symbol!(v);

// Solve for x, y in terms of u, v:
// x = (u + v)/2, y = (u - v)/2

// Transform integral: ∫∫ f(x,y) dx dy → ∫∫ f(u,v)·|J| du dv
// Jacobian: J = ∂(x,y)/∂(u,v) = 1/2

let f = expr!(x ^ 2 - y ^ 2);
let f_transformed = f.substitute(&x, &expr!((u + v) / 2))
                     .substitute(&y, &expr!((u - v) / 2));
// Result: ((u+v)/2)² - ((u-v)/2)²

let simplified = f_transformed.simplify();
// Result: uv (simplified form)
```

## Real-World Applications

### 1. Physics (Projectile Trajectory)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let t = symbol!(t);
let v0 = symbol!(v0);  // Initial velocity
let theta = symbol!(theta);  // Launch angle

// Position: x(t) = v₀·cos(θ)·t, y(t) = v₀·sin(θ)·t - (g/2)t²
let x_t = expr!(v0 * cos(theta) * t);
let y_t = expr!(v0 * sin(theta) * t - (9.8 / 2) * (t ^ 2));

// Evaluate at t = 2 seconds, v₀ = 20 m/s, θ = 45°
let x_val = x_t.substitute(&t, &expr!(2))
               .substitute(&v0, &expr!(20))
               .substitute(&theta, &Expression::pi_over_4());
// Result: x ≈ 28.3 m

let y_val = y_t.substitute(&t, &expr!(2))
               .substitute(&v0, &expr!(20))
               .substitute(&theta, &Expression::pi_over_4());
// Result: y ≈ 8.6 m
```

### 2. Economics (Demand Function)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let p = symbol!(p);  // Price
let q = symbol!(q);  // Quantity

// Demand: q = 100 - 2p
// Revenue: R = p·q = p(100 - 2p)
let demand = expr!(100 - 2 * p);
let revenue = expr!(p * q);

// Substitute demand into revenue
let revenue_function = revenue.substitute(&q, &demand);
// Result: R(p) = p(100 - 2p) = 100p - 2p²

// Evaluate at price p = 20
let revenue_at_20 = revenue_function.substitute(&p, &expr!(20));
// Result: 100(20) - 2(20)² = 2000 - 800 = 1200
```

### 3. Engineering (Signal Processing)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let t = symbol!(t);
let omega = symbol!(omega);  // Angular frequency

// Signal: s(t) = sin(ωt)
// Evaluate at t = 1, ω = 2π
let signal = expr!(sin(omega * t));
let evaluated = signal.substitute(&t, &expr!(1))
                      .substitute(&omega, &Expression::pi() * expr!(2));
// Result: sin(2π) = 0
```

### 4. Finance (Present Value)

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let FV = symbol!(FV);  // Future value
let r = symbol!(r);    // Interest rate
let t = symbol!(t);    // Time

// Present value: PV = FV / (1 + r)^t
let pv_formula = expr!(FV / ((1 + r) ^ t));

// Calculate PV of $1000 in 5 years at 5% interest
let pv = pv_formula.substitute(&FV, &expr!(1000))
                   .substitute(&r, &expr!(0.05))
                   .substitute(&t, &expr!(5));
// Result: 1000 / (1.05)⁵ ≈ 783.53
```

## Common Patterns (Cookbook)

### Pattern 1: Verify Solutions

Substitute to check if value solves equation:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Equation: x² - 5x + 6 = 0
// Claimed solution: x = 2
let equation = expr!(x ^ 2 - 5 * x + 6);
let check = equation.substitute(&x, &expr!(2));
// Result: 4 - 10 + 6 = 0 ✓ (verified!)

// Check another solution: x = 3
let check2 = equation.substitute(&x, &expr!(3));
// Result: 9 - 15 + 6 = 0 ✓ (verified!)
```

### Pattern 2: Function Composition

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let u = symbol!(u);

// Compose f(g(x)) where f(u) = u² and g(x) = sin(x)
let f = expr!(u ^ 2);
let g = expr!(sin(x));

// Compose: f(g(x)) = (sin(x))²
let composed = f.substitute(&u, &g);
// Result: sin²(x)
```

### Pattern 3: Simplify Before Substituting

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Expression: (x + 1)(x - 1)
let expression = expr!((x + 1) * (x - 1));

// Expand first, then substitute
let expanded = expression.expand();  // x² - 1
let result = expanded.substitute(&x, &expr!(10));
// Result: 100 - 1 = 99

// vs. substituting directly (also correct, but messier)
let direct = expression.substitute(&x, &expr!(10));
// Result: (10 + 1)(10 - 1) = 11 * 9 = 99
```

### Pattern 4: Partial Substitution

Substitute only some variables:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);
let z = symbol!(z);

// Expression: x² + y² + z²
let expression = expr!(x ^ 2 + y ^ 2 + z ^ 2);

// Substitute only x and y, leave z symbolic
let partial = expression.substitute(&x, &expr!(1))
                        .substitute(&y, &expr!(2));
// Result: 1 + 4 + z² = 5 + z²
```

## Common Pitfalls

### Pitfall 1: Substituting Without Simplifying

❌ **WRONG - Cluttered result:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

let expression = expr!((x + 1) * (x + 2));
let substituted = expression.substitute(&x, &expr!(3));
// Result: (3 + 1)(3 + 2) (not simplified)
```

✅ **CORRECT - Simplify after substitution:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let substituted = expression.substitute(&x, &expr!(3));
let simplified = substituted.simplify();
// Result: 20 (much cleaner)
```

### Pitfall 2: Order of Substitution Matters

❌ **WRONG - Incorrect order:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let y = symbol!(y);

// Expression: x + y
// Substitute x = y, then y = 2
let expr = expr!(x + y);
let step1 = expr.substitute(&x, &expr!(y));
// Result: y + y = 2y

let step2 = step1.substitute(&y, &expr!(2));
// Result: 2*2 = 4 (WRONG! Should be 2 + 2 = 4, not 2y where y=2)
```

✅ **CORRECT - Substitute all at once or in correct order:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Substitute x = y, y = 2 simultaneously:
// x + y where x = y and y = 2 → 2 + 2 = 4
let result = expr.substitute(&x, &expr!(2))
                 .substitute(&y, &expr!(2));
// Result: 4 ✓
```

### Pitfall 3: Forgetting Chain Rule in U-Substitution

❌ **WRONG - Missing du/dx term:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Integrate: ∫ e^(x²) dx (missing 2x factor)
// WRONG: Let u = x², ∫ e^u du = e^u
// Missing: du = 2x dx (need 2x in integrand)
```

✅ **CORRECT - Include derivative term:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Correct integral: ∫ 2x·e^(x²) dx
// Let u = x², du = 2x dx
// ∫ e^u du = e^u + C = e^(x²) + C
```

### Pitfall 4: Symbolic vs Numerical Context

❌ **WRONG - Expecting numerical result from symbolic substitution:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let a = symbol!(a);

// Substitute x = a (both symbolic)
let expression = expr!(x ^ 2);
let result = expression.substitute(&x, &expr!(a));
// Result: a² (symbolic, not numeric)
```

✅ **CORRECT - Understand symbolic vs numerical:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// For numerical result, substitute numerical value
let result = expression.substitute(&x, &expr!(5));
// Result: 25 (numerical)
```

## Performance Considerations

### When Substitution is Expensive

**Substitution cost depends on:**
1. **Expression size:** Large expressions require more traversal
2. **Number of occurrences:** More occurrences of variable → more replacements
3. **Complexity of replacement:** Substituting complex expressions is slower

**Optimization Strategies:**

1. **Simplify before substituting:**
   ```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
   let simplified = expression.simplify();
   let result = simplified.substitute(&x, &value);
   ```

2. **Batch substitutions:**
   ```rust
   // Instead of multiple substitute() calls, use batch method
   let substitutions = vec![(x.clone(), expr!(1)), (y.clone(), expr!(2))];
   let result = expression.substitute_many(&substitutions);
   ```

3. **Cache repeated substitutions:**
   ```rust
   // If substituting same value many times, cache the result
   let cached = expression.substitute(&x, &value);
   // Reuse cached for other operations
   ```

## API Reference

### Methods

```rust
impl Expression {
    /// Substitute variable with expression
    pub fn substitute(&self, var: &Symbol, value: &Expression) -> Expression;

    /// Substitute multiple variables at once
    pub fn substitute_many(&self, substitutions: &[(Symbol, Expression)]) -> Expression;

    /// Substitute and simplify in one step
    pub fn substitute_and_simplify(&self, var: &Symbol, value: &Expression) -> Expression;
}
```

### Substitution Trait

```rust
pub trait Substitute {
    /// Perform substitution
    fn substitute(&self, var: &Symbol, value: &Expression) -> Self;
}

impl Substitute for Expression { /* ... */ }
```

## See Also

- **[Solving](solving.md)** - Find solutions, then verify with substitution
- **[Integration](integration.md)** - U-substitution for difficult integrals
- **[Differentiation](differentiation.md)** - Chain rule with variable substitution
- **[Simplification](simplification.md)** - Simplify after substitution
- **External:** [U-Substitution](https://en.wikipedia.org/wiki/Integration_by_substitution) (Wikipedia)
- **External:** [Change of Variables](https://en.wikipedia.org/wiki/Change_of_variables) (Wikipedia)
