# Piecewise Functions

> 📍 **You are here:** Advanced > Piecewise Functions
>
> **Related Topics:** [Functions](../core/functions.md) | [Solving](../operations/solving.md) | [Differentiation](../operations/differentiation.md)
>
> **Skill Level:** ⭐⭐ Intermediate to ⭐⭐⭐ Advanced

Define functions with different formulas in different regions, essential for modeling discontinuous behavior and conditional logic.

## Quick Start (⭐⭐ Start here)

Create piecewise functions using condition-expression pairs:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Absolute value: |x| = { x if x ≥ 0, -x if x < 0 }
let abs_x = Expression::piecewise(
    vec![
        (expr!(x), expr!(x >= 0)),
        (expr!(-x), expr!(x < 0)),
    ],
    None,  // No default case
);

// Step function: H(x) = { 0 if x < 0, 1 if x ≥ 0 }
let heaviside = Expression::piecewise(
    vec![
        (expr!(0), expr!(x < 0)),
        (expr!(1), expr!(x >= 0)),
    ],
    None,
);

// With default case
let clamped = Expression::piecewise(
    vec![
        (expr!(0), expr!(x < 0)),
        (expr!(x), expr!(x >= 0 && x <= 1)),
    ],
    Some(expr!(1)),  // Default: 1 for x > 1
);
```

## Table of Contents

- [Understanding Piecewise Functions](#understanding-piecewise-functions)
- [Creating Piecewise Functions (⭐⭐ Intermediate)](#creating-piecewise-functions--intermediate)
- [Automatic Simplification](#automatic-simplification)
- [Operations on Piecewise Functions (⭐⭐⭐ Advanced)](#operations-on-piecewise-functions--advanced)
- [Evaluation](#evaluation)
- [Real-World Applications](#real-world-applications)
- [Common Patterns](#common-patterns)
- [Common Pitfalls](#common-pitfalls)
- [Performance Considerations](#performance-considerations)
- [See Also](#see-also)

## Understanding Piecewise Functions

### What Are Piecewise Functions? (Plain English)

A piecewise function is a function defined by multiple sub-functions, each applying to a specific interval of the domain. Different rules apply in different regions.

**Example:** Absolute value function:

$$|x| = \begin{cases}
x & \text{if } x \geq 0 \\
-x & \text{if } x < 0
\end{cases}$$

### Mathematical Background

**General Form:**

$$f(x) = \begin{cases}
f_1(x) & \text{if } C_1(x) \\
f_2(x) & \text{if } C_2(x) \\
\vdots & \\
f_n(x) & \text{if } C_n(x) \\
f_{\text{default}} & \text{otherwise}
\end{cases}$$

where:
- $f_i(x)$ are expression formulas
- $C_i(x)$ are boolean conditions
- $f_{\text{default}}$ is optional default case

**Key Properties:**

1. **Evaluation order:** Conditions checked sequentially (first match wins)
2. **Domain coverage:** Union of all condition domains should cover intended domain
3. **Continuity:** Function may be discontinuous at boundary points
4. **Differentiability:** May not be differentiable at boundaries

### When to Use Piecewise Functions

- **Physics:** Step functions, potential barriers, collision dynamics
- **Economics:** Tax brackets, tiered pricing, supply-demand with thresholds
- **Engineering:** Saturating amplifiers, clipping functions, control systems
- **Signal Processing:** Filters, thresholding, windowing
- **Mathematics:** Absolute value, signum function, indicator functions

## Creating Piecewise Functions (⭐⭐ Intermediate)

### Basic Syntax

Create piecewise functions with condition-expression pairs:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Simple piecewise: { 1 if x > 0, 0 otherwise }
let pw = Expression::piecewise(
    vec![
        (expr!(1), expr!(x > 0)),
    ],
    Some(expr!(0)),  // Default case
);
```

**Constructor Signature:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
Expression::piecewise(
    pieces: Vec<(Expression, Expression)>,  // (value, condition) pairs
    default: Option<Expression>,             // Optional default
) -> Expression
```

### Multiple Conditions

Define multiple regions:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Three-region function
let three_region = Expression::piecewise(
    vec![
        (expr!(-1), expr!(x < -1)),     // f(x) = -1 for x < -1
        (expr!(x), expr!(x >= -1 && x <= 1)),  // f(x) = x for -1 ≤ x ≤ 1
        (expr!(1), expr!(x > 1)),       // f(x) = 1 for x > 1
    ],
    None,  // No default needed (all cases covered)
);
```

### With Default Case

Provide fallback when no condition matches:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// With default
let pw = Expression::piecewise(
    vec![
        (expr!(x^2), expr!(x >= 0)),
    ],
    Some(expr!(0)),  // Default for x < 0
);
```

### Without Default Case

When all conditions should cover domain:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Covers all real numbers
let signum = Expression::piecewise(
    vec![
        (expr!(-1), expr!(x < 0)),
        (expr!(0), expr!(x == 0)),
        (expr!(1), expr!(x > 0)),
    ],
    None,  // No default needed
);
```

## Automatic Simplification

MathHook's piecewise constructor automatically simplifies in several cases:

### Removing False Branches

Conditions that are always false are removed:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Constructor removes impossible conditions
let pw = Expression::piecewise(
    vec![
        (expr!(x), expr!(x > 0)),
        (expr!(0), expr!(1 == 0)),  // Always false - removed!
    ],
    None,
);
// Result: Only (x, x > 0) remains
```

### Collapsing to Single Expression

If condition is always true, returns expression directly:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Condition always true
let pw = Expression::piecewise(
    vec![
        (expr!(x^2), expr!(1 == 1)),  // Always true
    ],
    None,
);
// Result: Simplifies to just x^2 (not piecewise!)
```

### No Conditions

If no conditions remain, returns default:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

// No valid conditions
let pw = Expression::piecewise(
    vec![],
    Some(expr!(0)),
);
// Result: Simplifies to 0
```

## Operations on Piecewise Functions (⭐⭐⭐ Advanced)

### Differentiation

Differentiate piecewise functions piece-by-piece:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// f(x) = { x^2 if x ≥ 0, -x^2 if x < 0 }
let f = Expression::piecewise(
    vec![
        (expr!(x^2), expr!(x >= 0)),
        (expr!(-x^2), expr!(x < 0)),
    ],
    None,
);

// Derivative
let df = f.derivative(&x, 1);
// Result: { 2x if x ≥ 0, -2x if x < 0 }
```

**Important:** Derivative may not exist at boundary points:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Absolute value |x|
let abs_x = Expression::piecewise(
    vec![
        (expr!(x), expr!(x >= 0)),
        (expr!(-x), expr!(x < 0)),
    ],
    None,
);

let d_abs = abs_x.derivative(&x, 1);
// Result: { 1 if x > 0, -1 if x < 0 }
// Note: Undefined at x=0 (sharp corner)
```

### Integration

Integrate piecewise functions:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Step function
let step = Expression::piecewise(
    vec![
        (expr!(0), expr!(x < 0)),
        (expr!(1), expr!(x >= 0)),
    ],
    None,
);

// Integral (ramp function)
let integral = step.integrate(&x);
// Result: { 0 if x < 0, x if x ≥ 0 }
```

### Arithmetic Operations

Combine piecewise functions:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

let f = Expression::piecewise(
    vec![
        (expr!(x), expr!(x >= 0)),
        (expr!(-x), expr!(x < 0)),
    ],
    None,
);

let g = Expression::piecewise(
    vec![
        (expr!(1), expr!(x > 0)),
        (expr!(0), expr!(x <= 0)),
    ],
    None,
);

// Addition
let sum = expr!(f + g);

// Multiplication
let product = expr!(f * g);

// Result: Piecewise with combined conditions
```

### Composition

Compose piecewise functions:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

let f = Expression::piecewise(
    vec![
        (expr!(x^2), expr!(x >= 0)),
        (expr!(0), expr!(x < 0)),
    ],
    None,
);

let g = expr!(2*x);

// f(g(x)) = f(2x)
let composition = f.substitute(&x, &g);
// Result: { (2x)^2 if 2x ≥ 0, 0 if 2x < 0 }
//       = { 4x^2 if x ≥ 0, 0 if x < 0 }
```

## Evaluation

### Symbolic Evaluation

Evaluate with symbolic arguments:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);
let y = symbol!(y);

let pw = Expression::piecewise(
    vec![
        (expr!(x^2), expr!(x >= 0)),
        (expr!(0), expr!(x < 0)),
    ],
    None,
);

// Substitute symbolic value
let result = pw.substitute(&x, &expr!(y + 1));
// Result: { (y+1)^2 if y+1 ≥ 0, 0 if y+1 < 0 }
//       = { (y+1)^2 if y ≥ -1, 0 if y < -1 }
```

### Numerical Evaluation

Evaluate with numeric values:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

let abs_x = Expression::piecewise(
    vec![
        (expr!(x), expr!(x >= 0)),
        (expr!(-x), expr!(x < 0)),
    ],
    None,
);

// Evaluate at x = 5
let result1 = abs_x.substitute(&x, &expr!(5));
// Result: 5

// Evaluate at x = -3
let result2 = abs_x.substitute(&x, &expr!(-3));
// Result: 3
```

### Condition Evaluation

Conditions are evaluated symbolically when possible:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

let pw = Expression::piecewise(
    vec![
        (expr!(1), expr!(x > 0)),
        (expr!(0), expr!(x <= 0)),
    ],
    None,
);

// With concrete value
let result = pw.substitute(&x, &expr!(5));
// MathHook evaluates: 5 > 0? → True → returns 1

let result2 = pw.substitute(&x, &expr!(-2));
// MathHook evaluates: -2 > 0? → False → checks next → -2 ≤ 0? → True → returns 0
```

## Real-World Applications

### 1. Physics: Potential Barriers

Quantum mechanics potential well:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);
let V0 = symbol!(V0);  // Potential height

// Finite square well potential
let potential = Expression::piecewise(
    vec![
        (expr!(0), expr!(x >= -1 && x <= 1)),  // Inside well
    ],
    Some(V0.into()),  // Outside well: V = V0
);

// Schrödinger equation: -ħ²/2m d²ψ/dx² + V(x)ψ = Eψ
```

### 2. Economics: Tax Brackets

Progressive tax system:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let income = symbol!(income);

// US-style progressive tax (simplified)
// 10% on first $10k, 12% on next $30k, 22% on remainder
let tax = Expression::piecewise(
    vec![
        (expr!(0.10 * income), expr!(income <= 10000)),
        (expr!(1000 + 0.12 * (income - 10000)), expr!(income <= 40000)),
    ],
    Some(expr!(4600 + 0.22 * (income - 40000))),  // Over $40k
);

// Calculate tax for $50,000 income
let tax_owed = tax.substitute(&income, &expr!(50000));
// Result: 4600 + 0.22 * 10000 = $6,800
```

### 3. Engineering: Saturating Amplifier

Amplifier with saturation limits:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let v_in = symbol!(v_in);
let gain = expr!(10);
let v_sat = expr!(5);  // Saturation voltage

// Output voltage with saturation
let v_out = Expression::piecewise(
    vec![
        (expr!(-v_sat), expr!(gain * v_in < -v_sat)),  // Negative saturation
        (expr!(gain * v_in), expr!(gain * v_in >= -v_sat && gain * v_in <= v_sat)),  // Linear region
    ],
    Some(v_sat),  // Positive saturation
);

// Input-output characteristic
// v_out = { -5 if 10*v_in < -5 (i.e., v_in < -0.5)
//         { 10*v_in if -0.5 ≤ v_in ≤ 0.5
//         { 5 if v_in > 0.5
```

### 4. Signal Processing: Clipping Function

Audio clipping or thresholding:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let signal = symbol!(signal);
let threshold = expr!(0.8);

// Hard clipping
let clipped = Expression::piecewise(
    vec![
        (expr!(-threshold), expr!(signal < -threshold)),
        (expr!(signal), expr!(signal >= -threshold && signal <= threshold)),
    ],
    Some(threshold),  // Clip positive peaks
);

// Soft clipping (tanh-like)
let soft_clipped = Expression::piecewise(
    vec![
        (expr!(signal), expr!(abs(signal) <= threshold)),
    ],
    Some(expr!(threshold * tanh(signal / threshold))),  // Smooth saturation
);
```

## Common Patterns

### Pattern 1: Absolute Value

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

let abs_x = Expression::piecewise(
    vec![
        (expr!(x), expr!(x >= 0)),
        (expr!(-x), expr!(x < 0)),
    ],
    None,
);

// Derivative (signum function)
let signum = abs_x.derivative(&x, 1);
// Result: { 1 if x > 0, -1 if x < 0 }
```

### Pattern 2: Heaviside Step Function

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Unit step function H(x)
let heaviside = Expression::piecewise(
    vec![
        (expr!(0), expr!(x < 0)),
        (expr!(1), expr!(x >= 0)),
    ],
    None,
);

// Shifted step H(x - a)
let a = symbol!(a);
let shifted_step = Expression::piecewise(
    vec![
        (expr!(0), expr!(x < a)),
        (expr!(1), expr!(x >= a)),
    ],
    None,
);
```

### Pattern 3: Ramp Function

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Ramp: { 0 if x < 0, x if x ≥ 0 }
let ramp = Expression::piecewise(
    vec![
        (expr!(0), expr!(x < 0)),
        (expr!(x), expr!(x >= 0)),
    ],
    None,
);

// Relation to Heaviside: ramp(x) = x * H(x)
let heaviside = Expression::piecewise(
    vec![
        (expr!(0), expr!(x < 0)),
        (expr!(1), expr!(x >= 0)),
    ],
    None,
);

let ramp_alt = expr!(x * heaviside);
```

### Pattern 4: Rectangular Pulse

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Pulse from a to b
let a = expr!(1);
let b = expr!(3);

let pulse = Expression::piecewise(
    vec![
        (expr!(0), expr!(x < a)),
        (expr!(1), expr!(x >= a && x <= b)),
    ],
    Some(expr!(0)),  // After b
);

// Alternative: pulse(x) = H(x-a) - H(x-b)
```

## Common Pitfalls

### ❌ Pitfall 1: Overlapping Conditions

**WRONG:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Ambiguous: x=0 matches both conditions
let pw = Expression::piecewise(
    vec![
        (expr!(1), expr!(x >= 0)),
        (expr!(-1), expr!(x <= 0)),  // Overlaps at x=0
    ],
    None,
);
// Which value at x=0? (First match wins: 1)
```

**CORRECT:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Clear separation
let pw = Expression::piecewise(
    vec![
        (expr!(1), expr!(x > 0)),
        (expr!(0), expr!(x == 0)),
        (expr!(-1), expr!(x < 0)),
    ],
    None,
);
// Unambiguous for all x
```

### ❌ Pitfall 2: Missing Default Case

**WRONG:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// What if x < 0?
let pw = Expression::piecewise(
    vec![
        (expr!(x^2), expr!(x >= 0)),
    ],
    None,  // No default! Undefined for x < 0
);
```

**CORRECT:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Cover all cases
let pw = Expression::piecewise(
    vec![
        (expr!(x^2), expr!(x >= 0)),
    ],
    Some(expr!(0)),  // Default for x < 0
);

// Or explicit coverage
let pw2 = Expression::piecewise(
    vec![
        (expr!(x^2), expr!(x >= 0)),
        (expr!(0), expr!(x < 0)),
    ],
    None,  // All cases covered
);
```

### ❌ Pitfall 3: Condition Evaluation Order

**WRONG:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Second condition never checked!
let pw = Expression::piecewise(
    vec![
        (expr!(1), expr!(x >= 0)),   // Matches for x ≥ 0
        (expr!(2), expr!(x > 5)),    // Never reached! (already matched above)
    ],
    None,
);
```

**CORRECT:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// More specific conditions first
let pw = Expression::piecewise(
    vec![
        (expr!(2), expr!(x > 5)),    // Check specific case first
        (expr!(1), expr!(x >= 0)),   // Then general case
    ],
    Some(expr!(0)),  // Default for x < 0
);
```

### ❌ Pitfall 4: Symbolic vs Numerical Conditions

**WRONG:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);
let a = symbol!(a);

// Cannot evaluate x > a symbolically without assumptions
let pw = Expression::piecewise(
    vec![
        (expr!(1), expr!(x > a)),
    ],
    Some(expr!(0)),
);

// This stays symbolic (correct behavior, but may not simplify as expected)
```

**CORRECT:**
```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Use concrete values when possible
let pw = Expression::piecewise(
    vec![
        (expr!(1), expr!(x > 0)),  // Concrete threshold
    ],
    Some(expr!(0)),
);

// Or use assumptions (when implemented)
// let a = symbol!(a).assume_positive();
```

### ⚠️ Warning: Discontinuities

Piecewise functions may be discontinuous:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

let x = symbol!(x);

// Discontinuous at x=0
let discontinuous = Expression::piecewise(
    vec![
        (expr!(1), expr!(x >= 0)),
        (expr!(-1), expr!(x < 0)),
    ],
    None,
);

// lim (x→0⁺) = 1
// lim (x→0⁻) = -1
// Limits don't match → discontinuous
```

## Performance Considerations

### Evaluation Efficiency

- **Condition checking:** Sequential (stops at first match)
- **Many conditions:** Linear search - keep number of pieces small
- **Nested piecewise:** Can be slow - consider flattening

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Avoid deeply nested piecewise
// ❌ Slow:
let nested = Expression::piecewise(
    vec![
        (Expression::piecewise(vec![(expr!(1), expr!(y > 0))], None), expr!(x > 0)),
    ],
    None,
);

// ✅ Better: Flatten
let flat = Expression::piecewise(
    vec![
        (expr!(1), expr!(x > 0 && y > 0)),
    ],
    None,
);
```

### Simplification

- **Automatic simplification:** Constructor simplifies trivial cases
- **Manual simplification:** Call `.simplify()` for complex piecewise
- **Condition simplification:** Simplify conditions first for better performance

### Memory Usage

- Each piece stores: expression + condition (2 expressions)
- Many pieces → higher memory usage
- Consider combining similar conditions

## API Reference

### Constructor

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
Expression::piecewise(
    pieces: Vec<(Expression, Expression)>,  // (value, condition) pairs
    default: Option<Expression>,             // Optional default
) -> Expression
```

### Methods

- `.evaluate()` - Evaluate piecewise with current values
- `.substitute(var, value)` - Substitute and evaluate
- `.derivative(var, order)` - Differentiate piece-by-piece
- `.integrate(var)` - Integrate piece-by-piece
- `.simplify()` - Simplify conditions and expressions

### Automatic Simplifications

- Removes branches with `False` conditions
- Returns single expression if condition always `True`
- Returns default if no valid conditions

## See Also

- **[Functions](../core/functions.md)** - Function creation and evaluation
- **[Solving](../operations/solving.md)** - Equation solving with piecewise
- **[Differentiation](../operations/differentiation.md)** - Derivatives of piecewise
- **[Integration](../operations/integration.md)** - Integrals of piecewise
- **[Assumptions](assumptions.md)** - Symbolic condition evaluation
- **[Pattern Matching](../core/pattern-matching.md)** - Pattern-based transformations
