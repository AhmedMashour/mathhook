# Pattern Matching

Pattern matching is a powerful technique in MathHook for identifying, transforming, and simplifying mathematical expressions. MathHook combines Rust's native pattern matching with specialized mathematical pattern recognition to enable sophisticated symbolic manipulation.

## Why Pattern Matching?

**Mathematical patterns are everywhere:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// These are all instances of the pattern: x^n
let patterns = vec![
    expr!(x^2),      // n=2
    expr!(x^3),      // n=3
    expr!(x^(-1)),   // n=-1 (reciprocal)
    expr!(x^(1/2)),  // n=1/2 (square root)
];

// Power rule derivative: d/dx(x^n) = n*x^(n-1)
// Single pattern, infinite instances
```

**Benefits of Pattern Matching:**
- **Simplification:** Recognize and apply algebraic identities
- **Transformation:** Convert between equivalent forms
- **Analysis:** Detect expression structure and properties
- **Optimization:** Apply context-specific rewrites for performance

## Rust Pattern Matching Basics

### Expression Variant Matching

MathHook expressions are Rust enums, enabling native pattern matching:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

fn analyze_expression(expr: &Expression) -> String {
    match expr {
        // Match specific variants
        Expression::Integer(n) => format!("Integer: {}", n),
        Expression::Symbol(s) => format!("Variable: {}", s.name()),

        // Match operations
        Expression::Add(terms) => {
            format!("Sum of {} terms", terms.len())
        }

        Expression::Mul(factors) => {
            format!("Product of {} factors", factors.len())
        }

        Expression::Pow(base, exp) => {
            format!("Power: ({})^({})", analyze_expression(base), analyze_expression(exp))
        }

        // Match functions
        Expression::Function { name, args } => {
            format!("Function {}: {} args", name, args.len())
        }

        // Catch-all for other variants
        _ => "Other expression type".to_string(),
    }
}
```

### Nested Pattern Matching

Match complex nested structures:

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

fn detect_quadratic(expr: &Expression) -> bool {
    match expr {
        // Pattern: a*x^2 + b*x + c
        Expression::Add(terms) if terms.len() == 3 => {
            // Check if terms match quadratic pattern
            let has_x_squared = terms.iter().any(|t| matches!(
                t,
                Expression::Mul(factors) if factors.iter().any(|f| matches!(
                    f,
                    Expression::Pow(base, exp) if matches!(**exp, Expression::Integer(2))
                ))
            ));

            let has_x = terms.iter().any(|t| matches!(
                t,
                Expression::Mul(factors) if factors.iter().any(|f| matches!(f, Expression::Symbol(_)))
            ));

            let has_constant = terms.iter().any(|t| matches!(
                t,
                Expression::Integer(_) | Expression::Rational(_)
            ));

            has_x_squared && has_x && has_constant
        }
        _ => false,
    }
}
```

## Mathematical Pattern Recognition

### Algebraic Patterns

#### Pattern: Sum of Powers

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);
let a = symbol!(a);
let b = symbol!(b);
let n = symbol!(n);

// Pattern: (a + b)^n expansion
// Recognizes binomial theorem applications
let binomial = expr!((a + b)^n);

// Pattern: a^2 + 2*a*b + b^2 = (a + b)^2
// Recognizes perfect square trinomial
let perfect_square = expr!(a^2 + 2*a*b + b^2);
let factored = perfect_square.factor();
assert_eq!(factored, expr!((a + b)^2));
```

#### Pattern: Difference of Squares

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let a = symbol!(a);
let b = symbol!(b);

// Pattern: a^2 - b^2 = (a + b)(a - b)
let diff_squares = expr!(a^2 - b^2);
let factored = diff_squares.factor();
assert_eq!(factored, expr!((a + b) * (a - b)));

// Recognizes in complex forms
let x = symbol!(x);
let example = expr!(x^4 - 16);
// Recognizes as (x^2)^2 - 4^2
let factored_example = example.factor();
assert_eq!(factored_example, expr!((x^2 + 4) * (x^2 - 4)));
```

#### Pattern: Sum and Difference Formulas

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Trigonometric identities as patterns
// sin(a + b) = sin(a)*cos(b) + cos(a)*sin(b)
let sin_sum = expr!(sin(a + b));
let expanded = sin_sum.expand();
assert_eq!(expanded, expr!(sin(a)*cos(b) + cos(a)*sin(b)));

// cos(a - b) = cos(a)*cos(b) + sin(a)*sin(b)
let cos_diff = expr!(cos(a - b));
let expanded_cos = cos_diff.expand();
assert_eq!(expanded_cos, expr!(cos(a)*cos(b) + sin(a)*sin(b)));
```

### Calculus Patterns

#### Pattern: Power Rule Derivatives

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Pattern matcher recognizes: d/dx(x^n) = n*x^(n-1)
fn matches_power_rule(expr: &Expression) -> Option<Expression> {
    match expr {
        Expression::Pow(base, exp) if matches!(**base, Expression::Symbol(_)) => {
            // Derivative: n*x^(n-1)
            Some(expr!(exp * (base ^ (exp + (-1)))))
        }
        _ => None,
    }
}

// Automatic application
let f = expr!(x^5);
let df = f.derivative(&x, 1);
assert_eq!(df, expr!(5 * x^4));
```

#### Pattern: Chain Rule

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Pattern: d/dx f(g(x)) = f'(g(x)) * g'(x)
// MathHook automatically recognizes composition

let f = expr!(sin(x^2));
// Pattern: sin(u) where u = x^2
// Derivative: cos(u) * du/dx = cos(x^2) * 2x
let df = f.derivative(&x, 1);
assert_eq!(df, expr!(2*x*cos(x^2)));

// Nested composition
let nested = expr!(sin(cos(x)));
let d_nested = nested.derivative(&x, 1);
// cos(cos(x)) * (-sin(x))
assert_eq!(d_nested, expr!(-sin(x)*cos(cos(x))));
```

#### Pattern: Product Rule

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Pattern: d/dx(f*g) = f'*g + f*g'
let f = expr!(x^2);
let g = expr!(sin(x));
let product = expr!(f * g);

let derivative = product.derivative(&x, 1);
// 2*x*sin(x) + x^2*cos(x)
assert_eq!(derivative, expr!(2*x*sin(x) + (x^2)*cos(x)));
```

### Trigonometric Patterns

#### Pythagorean Identities

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Pattern: sin^2(x) + cos^2(x) = 1
let identity = expr!(sin(x)^2 + cos(x)^2);
assert_eq!(identity.simplify(), expr!(1));

// Pattern: 1 + tan^2(x) = sec^2(x)
let tan_identity = expr!(1 + tan(x)^2);
assert_eq!(tan_identity.simplify(), expr!(sec(x)^2));

// Pattern: 1 + cot^2(x) = csc^2(x)
let cot_identity = expr!(1 + cot(x)^2);
assert_eq!(cot_identity.simplify(), expr!(csc(x)^2));
```

#### Angle Addition/Subtraction

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Pattern: sin(a ± b)
let sin_sum = expr!(sin(a + b));
let expanded = sin_sum.expand();
// sin(a)*cos(b) + cos(a)*sin(b)

let sin_diff = expr!(sin(a - b));
let expanded_diff = sin_diff.expand();
// sin(a)*cos(b) - cos(a)*sin(b)

// Pattern: cos(a ± b)
let cos_sum = expr!(cos(a + b));
let expanded_cos = cos_sum.expand();
// cos(a)*cos(b) - sin(a)*sin(b)
```

#### Double Angle Formulas

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Pattern: sin(2*x) = 2*sin(x)*cos(x)
let double_sin = expr!(sin(2*x));
let expanded = double_sin.expand();
assert_eq!(expanded, expr!(2*sin(x)*cos(x)));

// Pattern: cos(2*x) = cos^2(x) - sin^2(x)
let double_cos = expr!(cos(2*x));
// Multiple equivalent forms:
// cos^2(x) - sin^2(x)
// 2*cos^2(x) - 1
// 1 - 2*sin^2(x)
```

### Exponential and Logarithm Patterns

#### Logarithm Laws

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Pattern: ln(a*b) = ln(a) + ln(b)
let log_product = expr!(ln(a*b));
assert_eq!(log_product.expand(), expr!(ln(a) + ln(b)));

// Pattern: ln(a/b) = ln(a) - ln(b)
let log_quotient = expr!(ln(a/b));
assert_eq!(log_quotient.expand(), expr!(ln(a) - ln(b)));

// Pattern: ln(a^n) = n*ln(a)
let log_power = expr!(ln(a^n));
assert_eq!(log_power.expand(), expr!(n*ln(a)));
```

#### Exponential Identities

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Pattern: e^(a+b) = e^a * e^b
let exp_sum = expr!(e^(a + b));
assert_eq!(exp_sum.expand(), expr!(e^a * e^b));

// Pattern: (e^a)^b = e^(a*b)
let exp_power = expr!((e^a)^b);
assert_eq!(exp_power.simplify(), expr!(e^(a*b)));

// Pattern: e^(ln(x)) = x
let exp_ln = expr!(e^(ln(x)));
assert_eq!(exp_ln.simplify(), x);

// Pattern: ln(e^x) = x
let ln_exp = expr!(ln(e^x));
assert_eq!(ln_exp.simplify(), x);
```

## Custom Pattern Matching

### Implementing Pattern Matchers

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::{Expression, Symbol};

/// Pattern matcher for polynomial expressions
fn is_polynomial(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        // Constant term
        Expression::Integer(_) | Expression::Rational(_) => true,

        // Variable itself
        Expression::Symbol(s) if s == var => true,

        // Power of variable
        Expression::Pow(base, exp) => {
            matches!(**base, Expression::Symbol(ref s) if s == var) &&
            matches!(**exp, Expression::Integer(n) if n >= 0)
        }

        // Sum or product of polynomials
        Expression::Add(terms) | Expression::Mul(factors) => {
            terms.iter().all(|t| is_polynomial(t, var)) ||
            factors.iter().all(|f| is_polynomial(f, var))
        }

        _ => false,
    }
}

// Usage
let x = symbol!(x);
assert!(is_polynomial(&expr!(x^2 + 3*x + 1), &x));
assert!(!is_polynomial(&expr!(sin(x)), &x));
```

### Pattern-Based Simplification

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use mathhook::Expression;

/// Custom simplification rule: x*0 → 0
fn simplify_multiply_by_zero(expr: Expression) -> Expression {
    match expr {
        Expression::Mul(factors) => {
            if factors.iter().any(|f| matches!(f, Expression::Integer(0))) {
                Expression::integer(0)
            } else {
                Expression::Mul(factors)
            }
        }
        _ => expr,
    }
}

// Apply recursively to entire expression tree
fn apply_rule_recursive(expr: Expression) -> Expression {
    match expr {
        Expression::Add(terms) => {
            expr!(add: terms.into_iter().map(apply_rule_recursive).collect())
        }
        Expression::Mul(factors) => {
            simplify_multiply_by_zero(
                expr!(mul: factors.into_iter().map(apply_rule_recursive).collect())
            )
        }
        Expression::Pow(base, exp) => {
            Expression::pow(
                apply_rule_recursive(*base),
                apply_rule_recursive(*exp)
            )
        }
        _ => expr,
    }
}
```

## Real-World Pattern Matching

### Physics: Dimensional Analysis

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Pattern: Recognize dimensional consistency
fn check_dimensions(expr: &Expression) -> bool {
    // Pattern match to extract dimensional components
    match expr {
        // Force = mass * acceleration
        Expression::Mul(factors) if factors.len() == 2 => {
            // Check if factors have correct dimensions
            true  // Simplified check
        }
        _ => false,
    }
}

// Example: F = m*a (Newton's second law)
let m = symbol!(m);  // mass [kg]
let a = symbol!(a);  // acceleration [m/s²]
let F = expr!(m * a); // force [kg⋅m/s²] = [N]
```

### Engineering: Transfer Function Simplification

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Pattern: Recognize standard transfer function forms
let s = symbol!(s);
let omega_n = symbol!(omega_n);  // Natural frequency
let zeta = symbol!(zeta);         // Damping ratio

// Second-order system pattern
let second_order = expr!(omega_n^2 / (s^2 + 2*zeta*omega_n*s + omega_n^2));

// Pattern matcher can identify:
// - Poles and zeros
// - System type (first-order, second-order, etc.)
// - Stability characteristics
```

### Mathematics: Integral Patterns

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
let x = symbol!(x);

// Pattern: ∫ u' * f(u) dx → ∫ f(u) du (substitution)
// Recognizes when to apply u-substitution

let integrand = expr!(2*x * e^(x^2));
// Pattern: u = x^2, u' = 2*x
// ∫ 2x * e^(x^2) dx = ∫ e^u du = e^u = e^(x^2)

let integral = integrand.integrate(&x);
assert_eq!(integral, expr!(e^(x^2)));
```

## Pattern Matching Performance

### Efficient Matching Strategies

**1. Early Termination:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Stop matching as soon as pattern fails
fn quick_polynomial_check(expr: &Expression) -> bool {
    match expr {
        // Quick reject: definitely not polynomial
        Expression::Function { .. } => false,

        // Quick accept: definitely polynomial
        Expression::Integer(_) => true,
        Expression::Symbol(_) => true,

        // Recursive check only when necessary
        _ => true,  // Full check required
    }
}
```

**2. Pattern Caching:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
use std::collections::HashMap;

// Cache pattern recognition results
struct PatternCache {
    cache: HashMap<Expression, bool>,
}

impl PatternCache {
    fn is_polynomial(&mut self, expr: &Expression) -> bool {
        if let Some(&result) = self.cache.get(expr) {
            return result;
        }

        let result = /* ... pattern matching ... */;
        self.cache.insert(expr.clone(), result);
        result
    }
}
```

**3. Structural Hashing:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// Hash expression structure for fast lookup
use std::hash::{Hash, Hasher};

fn structural_hash(expr: &Expression) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    match expr {
        Expression::Add(terms) => {
            "add".hash(&mut hasher);
            terms.len().hash(&mut hasher);
        }
        Expression::Mul(factors) => {
            "mul".hash(&mut hasher);
            factors.len().hash(&mut hasher);
        }
        _ => expr.hash(&mut hasher),
    }
    hasher.finish()
}
```

## Common Pitfalls

### ❌ Pitfall 1: Incomplete Pattern Matching

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Forgets negative exponents
fn is_polynomial_bad(expr: &Expression) -> bool {
    match expr {
        Expression::Pow(base, exp) => {
            matches!(**exp, Expression::Integer(n) if n >= 0)  // Missing check!
        }
        _ => true,
    }
}

// ✅ CORRECT: Complete matching
fn is_polynomial_good(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Pow(base, exp) => {
            matches!(**base, Expression::Symbol(s) if s == var) &&
            matches!(**exp, Expression::Integer(n) if n >= 0)
        }
        _ => true,
    }
}
```

### ❌ Pitfall 2: Order Sensitivity

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Assumes specific term order
fn detect_quadratic_bad(expr: &Expression) -> bool {
    match expr {
        Expression::Add(terms) if terms.len() == 3 => {
            // Assumes order: [x^2, x, constant]
            // But terms might be reordered!
            true
        }
        _ => false,
    }
}

// ✅ CORRECT: Check all terms regardless of order
fn detect_quadratic_good(expr: &Expression) -> bool {
    match expr {
        Expression::Add(terms) if terms.len() == 3 => {
            let has_quadratic = terms.iter().any(|t| /* ... */);
            let has_linear = terms.iter().any(|t| /* ... */);
            let has_constant = terms.iter().any(|t| /* ... */);
            has_quadratic && has_linear && has_constant
        }
        _ => false,
    }
}
```

### ❌ Pitfall 3: Performance Blind Spots

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// ❌ WRONG: Expensive recursive matching without memoization
fn expensive_pattern_match(expr: &Expression) -> bool {
    match expr {
        Expression::Add(terms) => {
            terms.iter().all(|t| expensive_pattern_match(t))
            // Recomputes same subexpressions multiple times!
        }
        _ => true,
    }
}

// ✅ CORRECT: Use memoization or structural sharing
use std::collections::HashMap;

fn fast_pattern_match(expr: &Expression, cache: &mut HashMap<Expression, bool>) -> bool {
    if let Some(&result) = cache.get(expr) {
        return result;
    }

    let result = match expr {
        Expression::Add(terms) => {
            terms.iter().all(|t| fast_pattern_match(t, cache))
        }
        _ => true,
    };

    cache.insert(expr.clone(), result);
    result
}
```

## Limitations

### Pattern Complexity

**Not all mathematical patterns are efficiently recognizable:**

- ✅ **Polynomial patterns:** Fast and reliable
- ✅ **Trigonometric identities:** Pre-computed lookup
- ⚠️ **Arbitrary algebraic equivalence:** NP-hard in general
- ❌ **Diophantine equations:** Undecidable in general

### Performance Trade-offs

**Pattern matching can be expensive:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// O(n) pattern matching on expression tree of size n
// For large expressions, caching is essential
```

### False Positives/Negatives

**Pattern matching is heuristic-based:**

```rust
# extern crate mathhook_book;
# use mathhook_book::mathhook;
# use mathhook::prelude::*;
// May not recognize all equivalent forms
let expr1 = expr!((x + 1)^2);
let expr2 = expr!(x^2 + 2*x + 1);
// Pattern matcher may not immediately see these as equivalent
// Requires simplification first
```

## See Also

- **[Expressions](./expressions.md)** - Expression structure and variants
- **[Simplification](../operations/simplification.md)** - Automatic pattern application
- **[Differentiation](../operations/differentiation.md)** - Calculus pattern matching
- **[Integration](../operations/integration.md)** - Integral pattern recognition
- **[Expansion and Factoring](../operations/expansion-factoring.md)** - Algebraic patterns
- **API Reference:** `Expression` enum, pattern matching methods

## Mathematical References

- **Pattern Recognition:** Knuth, *The Art of Computer Programming*, Vol. 4A
- **Term Rewriting:** Baader & Nipkow, *Term Rewriting and All That*
- **Computer Algebra:** von zur Gathen & Gerhard, *Modern Computer Algebra*
- **Symbolic Computation:** Geddes et al., *Algorithms for Computer Algebra*
