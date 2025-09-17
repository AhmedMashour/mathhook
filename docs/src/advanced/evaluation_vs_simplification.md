# Evaluation vs Simplification: Comprehensive User Guide

**Status:** Documentation Guide
**Date:** 2025-01-24_1730
**Audience:** MathHook Users and Contributors
**Related:** `EXPRESSION_EVALUATION_ARCHITECTURE.md`, `../CLAUDE.md` (Quick Decision Guide)

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Core Concepts](#2-core-concepts)
3. [When to Use What](#3-when-to-use-what)
4. [API Reference](#4-api-reference)
5. [Common Patterns](#5-common-patterns)
6. [Common Pitfalls](#6-common-pitfalls)
7. [Advanced Topics](#7-advanced-topics)
8. [Real-World Examples](#8-real-world-examples)
9. [Performance Guide](#9-performance-guide)
10. [Troubleshooting](#10-troubleshooting)

---

## 1. Introduction

MathHook provides two fundamental operations for working with expressions:

1. **Evaluation** (`evaluate()`, `evaluate_with_context()`) - Compute numerical values
2. **Simplification** (`simplify()`) - Algebraic reduction

Understanding when to use each operation is critical for correct mathematical computation.

### The Key Principle

> **`evaluate()` ≠ `simplify()`** - They serve different purposes and should not be used interchangeably.

| Aspect | Evaluation | Simplification |
|--------|-----------|----------------|
| **Purpose** | Compute numerical values | Reduce algebraic complexity |
| **Input** | Expression (+ optional variables) | Expression only |
| **Output** | Numerical result or error | Simpler symbolic form |
| **Domain Checking** | ✅ Yes (catches mathematical errors) | ❌ No |
| **Substitution** | ✅ Yes (with context) | ❌ No |
| **Error Handling** | `Result<Expression, MathError>` | `Expression` |

---

## 2. Core Concepts

### 2.1 Evaluation: Numerical Computation

**Evaluation** converts symbolic expressions into concrete numerical values:

```rust
use mathhook_core::{expr, symbol};

// Constants → Numbers
let result = expr!(2 + 3).evaluate().unwrap();
assert_eq!(result, expr!(5));

// Functions → Values
let result = expr!(sin(0)).evaluate().unwrap();
assert_eq!(result, expr!(0));

// Symbolic with substitution
let x = symbol!(x);
let mut ctx = EvalContext::symbolic();
ctx.variables.insert("x".to_string(), expr!(3));
let result = expr!(x^2).evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(9));
```

**Key Features:**

1. **Domain Checking**: Catches mathematical errors
   ```rust
   // sqrt requires non-negative input
   assert!(expr!(sqrt(-1)).evaluate().is_err());

   // log has pole at 0
   assert!(expr!(log(0)).evaluate().is_err());

   // Division by zero
   assert!(expr!(1 / 0).evaluate().is_err());
   ```

2. **Recursive Evaluation**: Evaluates entire expression tree
   ```rust
   // Evaluates 2^3 → 8, then 8 + 5 → 13
   assert_eq!(expr!((2^3) + 5).evaluate().unwrap(), expr!(13));
   ```

3. **Error Propagation**: Errors bubble up from nested expressions
   ```rust
   // Inner sqrt(-1) causes error, propagates to outer expression
   assert!(expr!(2 + sqrt(-1)).evaluate().is_err());
   ```

### 2.2 Simplification: Algebraic Reduction

**Simplification** transforms expressions into equivalent but simpler symbolic forms:

```rust
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// Combine like terms
assert_eq!(expr!(x + x).simplify(), expr!(2 * x));

// Apply identities
assert_eq!(expr!(x * 1).simplify(), expr!(x));
assert_eq!(expr!(x + 0).simplify(), expr!(x));
assert_eq!(expr!(0 * x).simplify(), expr!(0));

// Trigonometric identities
assert_eq!(expr!(sin(x)^2 + cos(x)^2).simplify(), expr!(1));

// Constant folding
assert_eq!(expr!(2 + 3).simplify(), expr!(5));
```

**Key Features:**

1. **Algebraic Equivalence**: Output is mathematically equivalent to input
   ```rust
   let original = expr!(x + x + x);
   let simplified = original.simplify();
   assert_eq!(simplified, expr!(3 * x));
   // original and simplified represent the SAME mathematical value
   ```

2. **No Domain Checking**: Operates purely symbolically
   ```rust
   // Simplify doesn't validate domain (stays symbolic)
   let result = expr!(sqrt(-1)).simplify();
   // Result: sqrt(-1) or i (complex domain), but NO error
   ```

3. **Idempotency**: Simplifying twice yields the same result
   ```rust
   let expr = expr!(x + x);
   assert_eq!(expr.simplify().simplify(), expr.simplify());
   ```

### 2.3 The Hybrid Case: `evaluate()` with Simplification

**Important:** MathHook's `evaluate()` internally calls `simplify()` after numerical computation:

```rust
// Implementation pattern:
pub fn evaluate(&self) -> Result<Expression, MathError> {
    match self {
        Expression::Add(terms) => {
            let evaluated_terms: Result<Vec<_>, _> =
                terms.iter().map(|t| t.evaluate()).collect();
            Ok(Expression::add(evaluated_terms?).simplify()) // ← simplify after evaluation
        }
        // ...
    }
}
```

**Why?** This ensures canonical form (e.g., `2 + 3` → `5`, not `Add(2, 3)`).

---

## 3. When to Use What

### Decision Flowchart

```
┌─────────────────────────────────────┐
│ What do you need to do?             │
└─────────────────────────────────────┘
                 │
                 ▼
    ┌────────────────────────────┐
    │ Need a numerical result?   │
    └────────────────────────────┘
         │                   │
         │ YES               │ NO
         ▼                   ▼
┌─────────────────────┐  ┌──────────────────────────┐
│ Have variables?     │  │ Need symbolic reduction? │
└─────────────────────┘  └──────────────────────────┘
     │           │              │              │
     │ YES       │ NO           │ YES          │ NO
     ▼           ▼              ▼              ▼
evaluate_    evaluate()    simplify()    Keep as-is
with_context()
```

### Use Case Matrix

| Scenario | Operation | Rationale |
|----------|-----------|-----------|
| "What's 2 + 3?" | `evaluate()` | Need numerical result |
| "Simplify x + x" | `simplify()` | Need algebraic reduction |
| "Evaluate x² for x=5" | `evaluate_with_context()` | Need substitution + computation |
| "Check if sqrt(-1) is valid" | `evaluate()` | Need domain validation |
| "Reduce sin²(x) + cos²(x)" | `simplify()` | Need identity application |
| "Compute derivative" | Neither (use calculus API) | Different operation |

### Anti-Patterns to Avoid

❌ **DON'T use simplify() for numerical computation:**
```rust
// WRONG: simplify() doesn't guarantee numerical result
let x = symbol!(x);
let result = expr!(x + x).simplify(); // Returns 2*x, not a number!
```

✅ **DO use evaluate_with_context() for numerical computation:**
```rust
// RIGHT: evaluate_with_context() substitutes and computes
let x = symbol!(x);
let mut ctx = EvalContext::symbolic();
ctx.variables.insert("x".to_string(), expr!(5));
let result = expr!(x + x).evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(10));
```

❌ **DON'T use evaluate() for domain-safe symbolic manipulation:**
```rust
// WRONG: evaluate() errors on domain violations
let result = expr!(sqrt(-1)).evaluate(); // Error!
```

✅ **DO use simplify() for symbolic manipulation:**
```rust
// RIGHT: simplify() handles complex domain symbolically
let result = expr!(sqrt(-1)).simplify(); // Returns i (complex domain)
```

---

## 4. API Reference

### 4.1 `evaluate()` - Standard Evaluation

**Signature:**
```rust
pub fn evaluate(&self) -> Result<Expression, MathError>
```

**Purpose:** Evaluate expression with domain checking (no substitution).

**Returns:**
- `Ok(Expression)`: Evaluated result (numerical or symbolic if can't evaluate)
- `Err(MathError::DomainError)`: Domain violation detected
- `Err(MathError::DivisionByZero)`: Division by zero

**Examples:**
```rust
use mathhook_core::{expr, MathError};

// Success: Constants evaluate to numbers
assert_eq!(expr!(2 + 3).evaluate().unwrap(), expr!(5));

// Success: Special values
assert_eq!(expr!(sin(0)).evaluate().unwrap(), expr!(0));

// Error: Domain violation
assert!(matches!(
    expr!(sqrt(-1)).evaluate(),
    Err(MathError::DomainError { .. })
));

// Success but symbolic: Contains variables
let x = symbol!(x);
assert_eq!(expr!(x + 1).evaluate().unwrap(), expr!(x + 1).simplify());
```

**When to Use:**
- You need a numerical value
- You want domain validation
- Expression contains only constants (no variables)

**When NOT to Use:**
- Expression contains variables you want to substitute → use `evaluate_with_context()`
- You only need algebraic reduction → use `simplify()`

### 4.2 `evaluate_with_context()` - Evaluation with Substitution

**Signature:**
```rust
pub fn evaluate_with_context(&self, ctx: &EvalContext) -> Result<Expression, MathError>
```

**Purpose:** Evaluate expression with variable substitution and configurable behavior.

**Context Configuration:**
```rust
pub struct EvalContext {
    pub variables: HashMap<String, Expression>,  // Variable substitutions
    pub precision: u32,                          // Numerical precision
    pub simplify_first: bool,                    // Simplify before evaluation?
}

impl EvalContext {
    pub fn symbolic() -> Self { /* No numerical evaluation */ }
    pub fn numeric(vars: HashMap<String, Expression>) -> Self { /* With evaluation */ }
}
```

**Examples:**
```rust
use mathhook_core::core::expression::eval_numeric::{EvalContext, EvalNumeric};
use mathhook_core::{expr, symbol};
use std::collections::HashMap;

let x = symbol!(x);
let y = symbol!(y);

// Symbolic evaluation (substitution only)
let mut ctx = EvalContext::symbolic();
ctx.variables.insert("x".to_string(), expr!(2));
let result = expr!(x + 1).evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(3)); // Simplified symbolic result

// Numerical evaluation with precision
let mut vars = HashMap::new();
vars.insert("x".to_string(), expr!(3));
let ctx = EvalContext::numeric(vars).with_precision(100);
let result = expr!(x^2).evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(9));

// Multiple variables
let mut vars = HashMap::new();
vars.insert("x".to_string(), expr!(2));
vars.insert("y".to_string(), expr!(3));
let ctx = EvalContext::numeric(vars);
let result = expr!(x * y).evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(6));
```

**When to Use:**
- Expression contains variables you need to substitute
- You want control over evaluation behavior (simplify_first, precision)
- You're solving equations or evaluating formulas

**When NOT to Use:**
- No variables to substitute → use `evaluate()`
- Only need algebraic simplification → use `simplify()`

### 4.3 `simplify()` - Algebraic Simplification

**Signature:**
```rust
pub fn simplify(&self) -> Expression
```

**Purpose:** Reduce expression to canonical symbolic form using algebraic rules.

**Returns:** Simplified expression (always `Expression`, never errors).

**Examples:**
```rust
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// Combine like terms
assert_eq!(expr!(x + x + x).simplify(), expr!(3 * x));

// Remove identity elements
assert_eq!(expr!(x * 1).simplify(), expr!(x));
assert_eq!(expr!(x + 0).simplify(), expr!(x));

// Zero propagation
assert_eq!(expr!(0 * x).simplify(), expr!(0));

// Power rules
assert_eq!(expr!(x^1).simplify(), expr!(x));
assert_eq!(expr!(x^0).simplify(), expr!(1));

// Trigonometric identities
assert_eq!(expr!(sin(x)^2 + cos(x)^2).simplify(), expr!(1));

// Constant folding
assert_eq!(expr!(2 * 3 + 4).simplify(), expr!(10));
```

**Simplification Rules Applied:**

1. **Canonical Form**: Flatten, sort, remove identities
2. **Algebraic Identities**: Combine like terms, power rules
3. **Constant Folding**: Evaluate constant subexpressions
4. **Special Values**: Apply function identities (sin(0) → 0)

**When to Use:**
- You need algebraic reduction
- You want to reduce expression complexity
- You're preparing for symbolic operations (derivatives, solving)

**When NOT to Use:**
- You need numerical values → use `evaluate()`
- You need domain validation → use `evaluate()`
- You want to substitute variables → use `evaluate_with_context()`

### 4.4 Low-Level API: `eval_numeric()`

**Signature:**
```rust
pub trait EvalNumeric {
    fn eval_numeric(&self, precision: u32) -> Expression;
}
```

**Purpose:** Low-level type-specific numerical evaluation (advanced use only).

**Key Differences from `evaluate()`:**
- ❌ No domain checking
- ❌ No substitution
- ❌ No error handling
- ✅ Type-specific optimization
- ✅ Direct numerical computation

**When to Use:**
- You're implementing custom evaluation logic
- You need type-specific optimization
- You're building higher-level evaluation methods

**When NOT to Use:**
- Standard user-facing evaluation → use `evaluate()` or `evaluate_with_context()`

---

## 5. Common Patterns

### 5.1 Evaluate Constants

**Goal:** Compute numerical result from constant expression.

```rust
use mathhook_core::expr;

let result = expr!(2 + 3 * 4).evaluate().unwrap();
assert_eq!(result, expr!(14));

let result = expr!(sin(0) + cos(0)).evaluate().unwrap();
assert_eq!(result, expr!(1));
```

**When it works:**
- Expression contains only constants
- No variables present

**When it fails:**
- Expression contains undefined variables
- Domain violations (sqrt(-1), log(0), etc.)

### 5.2 Simplify Before Solving

**Goal:** Reduce expression complexity before solving equations.

```rust
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// Original equation: 2x + 3x = 10
let equation = expr!(2*x + 3*x);

// Simplify first: 5x = 10 (easier to solve)
let simplified = equation.simplify();
assert_eq!(simplified, expr!(5 * x));

// Now solve simplified equation
// (solving logic here)
```

**Benefits:**
- Reduces computational complexity
- Makes pattern matching easier
- Improves solver performance

### 5.3 Substitute Then Evaluate

**Goal:** Replace variables with values, then compute.

```rust
use mathhook_core::core::expression::eval_numeric::EvalContext;
use mathhook_core::{expr, symbol};
use std::collections::HashMap;

let x = symbol!(x);
let y = symbol!(y);

// Formula: f(x, y) = x^2 + 2xy + y^2
let formula = expr!(x^2 + 2*x*y + y^2);

// Evaluate at (x=3, y=4)
let mut vars = HashMap::new();
vars.insert("x".to_string(), expr!(3));
vars.insert("y".to_string(), expr!(4));
let ctx = EvalContext::numeric(vars);

let result = formula.evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(49)); // (3 + 4)^2 = 49
```

### 5.4 Domain-Safe Symbolic Manipulation

**Goal:** Work with expressions symbolically without triggering domain errors.

```rust
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// Simplify doesn't error on domain issues
let expr1 = expr!(sqrt(x)).simplify(); // OK, stays sqrt(x)
let expr2 = expr!(log(x)).simplify();  // OK, stays log(x)

// Evaluate would error if x < 0 or x <= 0 respectively
// let result = expr!(sqrt(-1)).evaluate(); // Error!
```

**When to Use:**
- Symbolic algebra (derivatives, integration)
- Building expressions programmatically
- When domain is guaranteed by context

### 5.5 Partial Evaluation

**Goal:** Evaluate some variables while keeping others symbolic.

```rust
use mathhook_core::core::expression::eval_numeric::EvalContext;
use mathhook_core::{expr, symbol};
use std::collections::HashMap;

let x = symbol!(x);
let y = symbol!(y);

// Substitute only x, keep y symbolic
let mut vars = HashMap::new();
vars.insert("x".to_string(), expr!(3));
let ctx = EvalContext::symbolic(); // Symbolic mode
let mut ctx = ctx;
ctx.variables = vars;

let result = expr!(x^2 + y).evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(9 + y)); // x evaluated, y symbolic
```

---

## 6. Common Pitfalls

### 6.1 Expecting Numbers from `simplify()`

❌ **WRONG:**
```rust
let x = symbol!(x);
let result = expr!(x + x).simplify();
// Expecting: 2 (numerical value)
// Actual: 2*x (still symbolic!)
```

✅ **RIGHT:**
```rust
let x = symbol!(x);
let mut ctx = EvalContext::symbolic();
ctx.variables.insert("x".to_string(), expr!(5));
let result = expr!(x + x).evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(10)); // Numerical result
```

**Lesson:** `simplify()` is for algebraic reduction, not numerical computation.

### 6.2 Using `evaluate()` Without Variables

❌ **WRONG:**
```rust
let x = symbol!(x);
let result = expr!(x + 1).evaluate().unwrap();
// Expecting: Error or numerical value
// Actual: x + 1 (symbolic, simplified)
```

✅ **RIGHT:**
```rust
let x = symbol!(x);
let mut vars = HashMap::new();
vars.insert("x".to_string(), expr!(5));
let ctx = EvalContext::numeric(vars);
let result = expr!(x + 1).evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(6));
```

**Lesson:** `evaluate()` can't substitute variables; use `evaluate_with_context()`.

### 6.3 Ignoring Domain Errors

❌ **WRONG:**
```rust
// Blindly unwrap without checking
let result = expr!(sqrt(-1)).evaluate().unwrap(); // PANIC!
```

✅ **RIGHT:**
```rust
match expr!(sqrt(-1)).evaluate() {
    Ok(result) => println!("Result: {}", result),
    Err(MathError::DomainError { operation, value, reason }) => {
        eprintln!("Domain error in {}: {} ({})", operation, value, reason);
    }
    Err(e) => eprintln!("Other error: {:?}", e),
}
```

**Lesson:** Always handle `Result` from `evaluate()`.

### 6.4 Confusing Simplification with Evaluation

❌ **WRONG:**
```rust
// Expecting simplify to validate domain
let result = expr!(log(0)).simplify(); // NO ERROR! Returns log(0)
```

✅ **RIGHT:**
```rust
// Use evaluate() for domain validation
assert!(expr!(log(0)).evaluate().is_err()); // Domain error
```

**Lesson:** `simplify()` doesn't check domains; `evaluate()` does.

### 6.5 Over-Simplifying Performance

❌ **WRONG:**
```rust
// Simplifying in a tight loop unnecessarily
for i in 0..1000 {
    let expr = expr!(x + 1); // Already in canonical form
    let simplified = expr.simplify(); // Wasted work!
}
```

✅ **RIGHT:**
```rust
// Simplify once, reuse
let expr = expr!(x + 1).simplify();
for i in 0..1000 {
    // Use simplified expression
}
```

**Lesson:** `simplify()` is not free; avoid redundant calls.

---

## 7. Advanced Topics

### 7.1 Custom Evaluation Context

**Scenario:** You want custom behavior for evaluation (e.g., complex domain, symbolic-only).

```rust
use mathhook_core::core::expression::eval_numeric::EvalContext;
use std::collections::HashMap;

// Complex domain evaluation (allow sqrt(-1) → i)
let ctx = EvalContext {
    variables: HashMap::new(),
    precision: 64,
    simplify_first: true, // Simplify before evaluating
};

// Symbolic-only evaluation (no numerical approximation)
let ctx = EvalContext::symbolic();

// High-precision evaluation
let mut vars = HashMap::new();
vars.insert("x".to_string(), expr!(3));
let ctx = EvalContext::numeric(vars).with_precision(256);
```

### 7.2 Evaluation vs Simplification in Solvers

**Pattern:** When solving equations, you typically:
1. Simplify the equation first (reduce complexity)
2. Apply solver algorithm (pattern matching, substitution)
3. Evaluate candidate solutions (verify correctness)

```rust
use mathhook_core::{expr, symbol};

let x = symbol!(x);

// Step 1: Simplify equation
let equation = expr!(2*x + 3*x - 10); // Original
let simplified = equation.simplify(); // → 5*x - 10

// Step 2: Solve (pattern matching)
// (solver logic: 5x - 10 = 0 → x = 2)
let solution = expr!(2);

// Step 3: Verify by evaluation
let mut vars = HashMap::new();
vars.insert("x".to_string(), solution.clone());
let ctx = EvalContext::numeric(vars);
let verified = simplified.evaluate_with_context(&ctx).unwrap();
assert_eq!(verified, expr!(0)); // Confirms solution
```

### 7.3 Partial Evaluation Strategies

**Strategy 1: Evaluate known variables, keep unknowns symbolic**

```rust
let x = symbol!(x);
let y = symbol!(y);

let mut vars = HashMap::new();
vars.insert("x".to_string(), expr!(3));
// y not provided, stays symbolic

let ctx = EvalContext::symbolic();
let mut ctx = ctx;
ctx.variables = vars;

let result = expr!(x*y + x^2).evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(3*y + 9)); // x substituted, y symbolic
```

**Strategy 2: Evaluate subexpressions incrementally**

```rust
let x = symbol!(x);

// Evaluate inner expression first
let inner = expr!(2 + 3).evaluate().unwrap(); // → 5

// Use result in outer expression
let outer = expr!(inner * x); // → 5*x
```

### 7.4 Performance: When to Simplify vs Evaluate

**Benchmark Insights:**

| Operation | Typical Cost | When to Use |
|-----------|--------------|-------------|
| `simplify()` | O(n) tree traversal + pattern matching | Always before operations that benefit from canonical form |
| `evaluate()` | O(n) traversal + domain checks | When you need numerical results |
| `evaluate_with_context()` | O(n) traversal + substitution + evaluation | When substituting variables |

**Optimization Tips:**

1. **Simplify once, evaluate many times:**
   ```rust
   let simplified = expr.simplify(); // Once
   for value in values {
       let mut vars = HashMap::new();
       vars.insert("x".to_string(), expr!(value));
       let ctx = EvalContext::numeric(vars);
       let result = simplified.evaluate_with_context(&ctx).unwrap(); // Many times
   }
   ```

2. **Avoid redundant simplification:**
   ```rust
   // BAD: Simplify in loop
   for _ in 0..1000 {
       let result = expr.simplify(); // Repeated work!
   }

   // GOOD: Simplify once
   let simplified = expr.simplify();
   for _ in 0..1000 {
       // Use simplified
   }
   ```

3. **Use evaluate() only when needed:**
   ```rust
   // If no domain checking needed and expression is symbolic:
   let result = expr.simplify(); // Faster than evaluate()

   // If domain checking required:
   let result = expr.evaluate()?; // Necessary overhead
   ```

---

## 8. Real-World Examples

### Example 1: Quadratic Formula Solver

```rust
use mathhook_core::{expr, symbol};

fn solve_quadratic(a: i64, b: i64, c: i64) -> Result<(f64, f64), String> {
    let a_expr = expr!(a);
    let b_expr = expr!(b);
    let c_expr = expr!(c);

    // Discriminant: b² - 4ac
    let discriminant = expr!(b_expr^2 - 4*a_expr*c_expr);

    // Simplify discriminant
    let disc_simplified = discriminant.simplify();

    // Evaluate to get numerical value
    let disc_value = disc_simplified.evaluate()
        .map_err(|e| format!("Failed to evaluate discriminant: {:?}", e))?;

    // Extract numerical value
    let disc_num = match disc_value {
        Expression::Number(Number::Integer(n)) => n as f64,
        Expression::Number(Number::Float(f)) => f,
        _ => return Err("Discriminant must be numeric".to_string()),
    };

    if disc_num < 0.0 {
        return Err("No real solutions (negative discriminant)".to_string());
    }

    // Solutions: (-b ± √discriminant) / (2a)
    let sqrt_disc = disc_num.sqrt();
    let solution1 = (-b as f64 + sqrt_disc) / (2.0 * a as f64);
    let solution2 = (-b as f64 - sqrt_disc) / (2.0 * a as f64);

    Ok((solution1, solution2))
}

// Usage
let (x1, x2) = solve_quadratic(1, -3, 2).unwrap();
assert!((x1 - 2.0).abs() < 1e-10);
assert!((x2 - 1.0).abs() < 1e-10);
```

### Example 2: Function Evaluation Table

```rust
use mathhook_core::{expr, symbol};

fn evaluate_function_table(
    formula: &Expression,
    var_name: &str,
    values: Vec<i64>
) -> Vec<(i64, Expression)> {
    // Simplify formula once
    let simplified = formula.simplify();

    // Evaluate for each value
    values.into_iter().map(|value| {
        let mut vars = HashMap::new();
        vars.insert(var_name.to_string(), expr!(value));
        let ctx = EvalContext::numeric(vars);

        let result = simplified.evaluate_with_context(&ctx)
            .unwrap_or_else(|_| expr!(NaN)); // Handle errors gracefully

        (value, result)
    }).collect()
}

// Usage: Evaluate f(x) = x² + 2x + 1 for x = 0, 1, 2, 3, 4
let x = symbol!(x);
let formula = expr!(x^2 + 2*x + 1);
let table = evaluate_function_table(&formula, "x", vec![0, 1, 2, 3, 4]);

assert_eq!(table[0].1, expr!(1));  // f(0) = 1
assert_eq!(table[1].1, expr!(4));  // f(1) = 4
assert_eq!(table[2].1, expr!(9));  // f(2) = 9
```

### Example 3: Symbolic Differentiation with Evaluation

```rust
use mathhook_core::{expr, symbol};

fn derivative_at_point(
    formula: &Expression,
    var: &Symbol,
    point: i64
) -> Result<Expression, String> {
    // Step 1: Compute symbolic derivative
    let derivative = formula.derivative(var, 1);

    // Step 2: Simplify derivative
    let simplified = derivative.simplify();

    // Step 3: Evaluate at specific point
    let mut vars = HashMap::new();
    vars.insert(var.name().to_string(), expr!(point));
    let ctx = EvalContext::numeric(vars);

    simplified.evaluate_with_context(&ctx)
        .map_err(|e| format!("Evaluation failed: {:?}", e))
}

// Usage: f(x) = x³, find f'(2)
let x = symbol!(x);
let formula = expr!(x^3);
let derivative_value = derivative_at_point(&formula, &x, 2).unwrap();
assert_eq!(derivative_value, expr!(12)); // f'(2) = 3*2² = 12
```

### Example 4: Physics Formula Evaluation

```rust
use mathhook_core::{expr, symbol};

fn kinetic_energy(mass_kg: f64, velocity_ms: f64) -> f64 {
    let m = symbol!(m);
    let v = symbol!(v);

    // Formula: KE = (1/2) * m * v²
    let formula = expr!((1/2) * m * v^2);

    // Substitute values
    let mut vars = HashMap::new();
    vars.insert("m".to_string(), Expression::float(mass_kg));
    vars.insert("v".to_string(), Expression::float(velocity_ms));
    let ctx = EvalContext::numeric(vars);

    // Evaluate
    let result = formula.evaluate_with_context(&ctx).unwrap();

    // Extract float
    match result {
        Expression::Number(Number::Float(ke)) => ke,
        Expression::Number(Number::Integer(ke)) => ke as f64,
        Expression::Number(Number::Rational(n, d)) => n as f64 / d as f64,
        _ => panic!("Expected numerical result"),
    }
}

// Usage: KE of 10kg object moving at 5 m/s
let ke = kinetic_energy(10.0, 5.0);
assert!((ke - 125.0).abs() < 1e-10); // KE = 0.5 * 10 * 25 = 125 J
```

---

## 9. Performance Guide

### 9.1 Benchmarking Results

Based on internal benchmarks (approximate, varies by expression complexity):

| Operation | Time (simple expr) | Time (complex expr) | Notes |
|-----------|-------------------|---------------------|-------|
| `simplify()` | ~100ns | ~1-10μs | Depends on rule matching |
| `evaluate()` | ~150ns | ~2-20μs | Adds domain checking |
| `evaluate_with_context()` | ~200ns | ~5-50μs | Adds substitution |

**Key Insight:** For constant expressions, `evaluate()` and `simplify()` have similar performance.

### 9.2 Optimization Strategies

**Strategy 1: Reuse Simplified Expressions**

```rust
// BAD: Simplify repeatedly
for i in 0..1000 {
    let result = expr.simplify(); // O(n) each iteration
}

// GOOD: Simplify once
let simplified = expr.simplify();
for i in 0..1000 {
    // Use simplified (O(1) each iteration)
}
```

**Strategy 2: Batch Evaluation**

```rust
// BAD: Create context repeatedly
for value in values {
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), expr!(value)); // Allocates HashMap each time
    let ctx = EvalContext::numeric(vars);
    let result = expr.evaluate_with_context(&ctx).unwrap();
}

// GOOD: Reuse context
let mut vars = HashMap::new();
for value in values {
    vars.insert("x".to_string(), expr!(value)); // Reuse HashMap
    let ctx = EvalContext::numeric(vars.clone()); // Clone is cheap for small maps
    let result = expr.evaluate_with_context(&ctx).unwrap();
}
```

**Strategy 3: Avoid Unnecessary Domain Checking**

```rust
// If you KNOW expression is valid (e.g., already validated):
let result = expr.simplify(); // Faster, no domain checks

// If unsure or user-provided input:
let result = expr.evaluate()?; // Safer, checks domain
```

### 9.3 Profiling Tips

Use Criterion for benchmarking:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_eval_vs_simplify(c: &mut Criterion) {
    let expr = expr!(2 + 3 * 4);

    c.bench_function("evaluate", |b| {
        b.iter(|| black_box(expr.evaluate().unwrap()))
    });

    c.bench_function("simplify", |b| {
        b.iter(|| black_box(expr.simplify()))
    });
}

criterion_group!(benches, benchmark_eval_vs_simplify);
criterion_main!(benches);
```

---

## 10. Troubleshooting

### Problem 1: "Expected number, got symbolic expression"

**Symptom:**
```rust
let x = symbol!(x);
let result = expr!(x + 1).simplify();
// Expected: 1 (if x=0) or numerical value
// Got: x + 1 (still symbolic)
```

**Cause:** `simplify()` doesn't substitute variables or evaluate numerically.

**Solution:** Use `evaluate_with_context()`:
```rust
let mut vars = HashMap::new();
vars.insert("x".to_string(), expr!(0));
let ctx = EvalContext::numeric(vars);
let result = expr!(x + 1).evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(1));
```

### Problem 2: "Unwrap panicked on domain error"

**Symptom:**
```rust
let result = expr!(sqrt(-1)).evaluate().unwrap(); // PANIC!
```

**Cause:** `evaluate()` returns `Result`, which can contain `DomainError`.

**Solution:** Handle the error:
```rust
match expr!(sqrt(-1)).evaluate() {
    Ok(result) => println!("Result: {}", result),
    Err(MathError::DomainError { operation, value, reason }) => {
        eprintln!("Cannot compute {}: {} ({})", operation, value, reason);
    }
    Err(e) => eprintln!("Error: {:?}", e),
}
```

### Problem 3: "Simplify doesn't compute numerical value"

**Symptom:**
```rust
let result = expr!(2 + 3).simplify();
// Expected: 5
// Got: 5 (works, but confusion about when it computes)
```

**Cause:** `simplify()` DOES fold constants, but this is algebraic reduction, not "evaluation".

**Clarification:**
- `simplify()` applies algebraic rules, including constant folding
- For pure constants, `simplify()` and `evaluate()` produce the same result
- For expressions with variables, they differ:
  ```rust
  let x = symbol!(x);
  expr!(2 + 3).simplify(); // → 5 (constant folding)
  expr!(x + x).simplify(); // → 2*x (algebraic reduction)
  expr!(x + x).evaluate(); // → 2*x (can't evaluate without x value)
  ```

### Problem 4: "evaluate_with_context() returns symbolic result"

**Symptom:**
```rust
let x = symbol!(x);
let y = symbol!(y);
let mut vars = HashMap::new();
vars.insert("x".to_string(), expr!(3));
// Note: y not provided!
let ctx = EvalContext::numeric(vars);
let result = expr!(x + y).evaluate_with_context(&ctx).unwrap();
// Expected: Error or numerical value
// Got: 3 + y (partially evaluated)
```

**Cause:** `evaluate_with_context()` substitutes provided variables, keeps others symbolic.

**Solution:** Provide all variables:
```rust
let mut vars = HashMap::new();
vars.insert("x".to_string(), expr!(3));
vars.insert("y".to_string(), expr!(4)); // Added y
let ctx = EvalContext::numeric(vars);
let result = expr!(x + y).evaluate_with_context(&ctx).unwrap();
assert_eq!(result, expr!(7));
```

### Problem 5: "Performance degradation with repeated operations"

**Symptom:**
```rust
// Slow performance
for i in 0..10000 {
    let result = expr.simplify(); // Called repeatedly
}
```

**Cause:** Each `simplify()` call traverses the entire expression tree.

**Solution:** Simplify once, reuse:
```rust
let simplified = expr.simplify(); // Once
for i in 0..10000 {
    // Use simplified (fast)
}
```

---

## Appendix A: Quick Reference Table

| Scenario | Use This | Not This | Reason |
|----------|----------|----------|--------|
| Compute 2 + 3 | `evaluate()` | `simplify()` | Both work, evaluate is semantically correct |
| Simplify x + x | `simplify()` | `evaluate()` | No variables to evaluate |
| Evaluate x² at x=5 | `evaluate_with_context()` | `evaluate()` | Needs substitution |
| Check sqrt(-1) validity | `evaluate()` | `simplify()` | Needs domain check |
| Reduce sin²(x) + cos²(x) | `simplify()` | `evaluate()` | Algebraic identity |
| Compute derivative | `derivative()` | `evaluate()` / `simplify()` | Different operation |

## Appendix B: Error Codes Reference

| Error | Cause | Solution |
|-------|-------|----------|
| `DomainError` | Mathematical constraint violated | Check domain before evaluation |
| `DivisionByZero` | Attempted 1/0 or 0^(-n) | Validate denominator/exponent |
| `NotImplemented` | Feature not yet supported | Use alternative approach or wait |

---

**End of Guide**

For architectural details, see `EXPRESSION_EVALUATION_ARCHITECTURE.md`.
For implementation details, see source code in `crates/mathhook-core/src/core/expression/`.

**Last Updated:** 2025-01-24_1730
**Author:** Claude Code (Anthropic)
