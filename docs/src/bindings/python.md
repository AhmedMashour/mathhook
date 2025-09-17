# Python API Guide

Complete guide to using MathHook from Python via PyO3 bindings.

## Installation

```bash
pip install mathhook
```

**Requirements**:
- Python 3.8 or higher
- pip 20.0 or higher (for binary wheel support)

**Platform Support**:
- Linux (x86_64, aarch64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

## Quick Start

```python
from mathhook import Symbol, parse, simplify

# Create symbols
x = Symbol('x')
y = Symbol('y')

# Build expressions
expr = x**2 + 2*x + 1

# Simplify
simplified = simplify(expr)
print(simplified)  # (x + 1)^2
```

## Why MathHook for Python?

### Performance Comparison

**100x Faster Than SymPy** for large expressions:

```python
import time
from mathhook import parse, simplify
# import sympy  # For comparison

# Large polynomial expression
expr_str = " + ".join([f"{i}*x**{i}" for i in range(100)])

# MathHook
start = time.time()
expr = parse(expr_str)
result = simplify(expr)
mathhook_time = time.time() - start

# SymPy (for comparison)
# start = time.time()
# expr_sympy = sympy.sympify(expr_str)
# result_sympy = sympy.simplify(expr_sympy)
# sympy_time = time.time() - start

print(f"MathHook: {mathhook_time:.4f}s")
# Typical: MathHook 0.001s vs SymPy 0.1s (100x faster)
```

### When to Use MathHook vs SymPy

**Use MathHook when**:
- Performance is critical (real-time applications, large expressions)
- You need symbolic preprocessing for numerical simulations
- Working with expressions with >50 terms
- Building interactive applications (web, Jupyter with fast response)

**Use SymPy when**:
- Need advanced features: logic, sets, abstract algebra
- Educational prototyping (rich ecosystem)
- Assumption system is critical
- Working with small expressions where speed doesn't matter

**Use Both**:
- Prototype with SymPy, optimize with MathHook for production
- Use MathHook for hot loops, SymPy for one-time complex operations

---

## API Reference

### Symbols

Create mathematical variables:

```python
from mathhook import Symbol

x = Symbol('x')
y = Symbol('y')
theta = Symbol('theta')
```

**Equality**:
```python
x1 = Symbol('x')
x2 = Symbol('x')
assert x1 == x2  # Same symbol name = equal
```

### Expressions

#### Creating Expressions

**Method 1: Operator Overloading** (Pythonic, recommended)

```python
from mathhook import Symbol

x = Symbol('x')

# Arithmetic operators
expr = x**2 + 2*x + 1
expr = (x + 1) * (x - 1)
expr = x / (x + 1)
expr = -x

# Supported operators: +, -, *, /, **, -unary
```

**Method 2: Builder Methods**

```python
from mathhook import Symbol, Expression

x = Symbol('x')

# Explicit construction
expr = Expression.add(x.pow(2), Expression.mul(2, x), 1)
expr = Expression.sub(x, 1)
expr = Expression.div(x, x.add(1))
```

**Method 3: Parsing** (from LaTeX or standard notation)

```python
from mathhook import parse

expr = parse("x^2 + 2*x + 1")
expr = parse(r"\frac{x^2 + 1}{x - 1}")  # LaTeX
expr = parse("sin(x) + cos(x)")
```

#### Expression Operations

**Simplification**:
```python
from mathhook import simplify

expr = parse("x + x")
result = simplify(expr)  # 2*x

expr = parse("(x + 1) * (x - 1)")
result = simplify(expr)  # x^2 - 1
```

**Expansion**:
```python
from mathhook import expand

expr = parse("(x + 1)^2")
result = expand(expr)  # x^2 + 2*x + 1
```

**Factorization**:
```python
from mathhook import factor

expr = parse("x^2 - 1")
result = factor(expr)  # (x - 1) * (x + 1)
```

**Substitution**:
```python
x = Symbol('x')
expr = x**2 + 2*x + 1

# Substitute x = 3
result = expr.substitute(x, 3)
print(result)  # 16

# Substitute with another expression
y = Symbol('y')
result = expr.substitute(x, y + 1)
print(result)  # (y + 1)^2 + 2*(y + 1) + 1
```

**Evaluation**:
```python
from mathhook import parse, Symbol

# Evaluate numeric expressions
expr = parse("2 + 3 * 4")
result = expr.evaluate()
print(result)  # 14

# Evaluate symbolic expressions (performs simplification)
expr = parse("x + x")
result = expr.evaluate()
print(result)  # 2*x

# Evaluation with domain checking
try:
    expr = parse("sqrt(-1)")
    result = expr.evaluate()
except Exception as e:
    print(f"Domain error: {e}")

# Combining substitution with evaluation
x = Symbol('x')
expr = x**2 + 2*x + 1
substituted = expr.substitute(x, 3)
result = substituted.evaluate()
print(result)  # 16
```

**Evaluation with Context** (Advanced):

For more control over evaluation behavior, use `EvalContext`:

```python
from mathhook import PyExpression as Expression, EvalContext

x = Expression.symbol("x")
y = Expression.symbol("y")

# Formula: x² + 2xy + y²
expr = x.pow(Expression.integer(2)).add(
    Expression.integer(2).multiply(x).multiply(y)
).add(y.pow(Expression.integer(2)))

# Create numerical context with variable substitutions
ctx = EvalContext.numeric({
    "x": Expression.integer(3),
    "y": Expression.integer(4)
})

# Evaluate: (3)² + 2(3)(4) + (4)² = 9 + 24 + 16 = 49
result = expr.evaluate_with_context(ctx)
print(result)  # 49

# Symbolic evaluation (no numerical conversion)
ctx_symbolic = EvalContext.symbolic()
result_symbolic = expr.evaluate_with_context(ctx_symbolic)
print(result_symbolic)  # x^2 + 2*x*y + y^2 (still symbolic)

# Custom configuration
ctx_custom = EvalContext(
    variables={"x": Expression.integer(5)},
    numeric=True,
    precision=128,
    simplify_first=True
)
result_custom = expr.evaluate_with_context(ctx_custom)
print(result_custom)  # Evaluates with x=5, y symbolic
```

**EvalContext Configuration**:

- `variables`: Dictionary mapping variable names to Expression values
- `numeric`: Perform numerical evaluation (default: True)
- `precision`: Bits of precision for floating-point (default: 53 for f64)
- `simplify_first`: Simplify before evaluation (default: True)

**Factory Methods**:

```python
# Symbolic-only context (no numerical evaluation)
ctx = EvalContext.symbolic()

# Numerical context with variables
ctx = EvalContext.numeric({"x": Expression.integer(3)})

# Custom context
ctx = EvalContext(variables={...}, numeric=True, precision=128)
```

### Calculus Operations

#### Derivatives

```python
from mathhook import Symbol, derivative

x = Symbol('x')
expr = x**3

# First derivative
df = derivative(expr, x)
print(df)  # 3*x^2

# Second derivative
d2f = derivative(expr, x, order=2)
print(d2f)  # 6*x

# Partial derivatives
x, y = Symbol('x'), Symbol('y')
expr = x**2 * y
df_dx = derivative(expr, x)  # 2*x*y
df_dy = derivative(expr, y)  # x^2
```

#### Integration

```python
from mathhook import Symbol, integrate

x = Symbol('x')

# Indefinite integral
expr = x**2
integral = integrate(expr, x)
print(integral)  # x^3 / 3 + C

# Definite integral
integral = integrate(expr, x, lower=0, upper=2)
print(integral)  # 8/3
```

#### Limits

```python
from mathhook import Symbol, limit

x = Symbol('x')

# Limit as x -> 0
expr = parse("sin(x) / x")
result = limit(expr, x, 0)
print(result)  # 1

# One-sided limits
result = limit(expr, x, 0, direction='+')  # Right limit
result = limit(expr, x, 0, direction='-')  # Left limit
```

### Equation Solving

#### Algebraic Equations

```python
from mathhook import Symbol, solve

x = Symbol('x')

# Linear equation: 2*x + 3 = 7
solutions = solve(2*x + 3, 7, x)
print(solutions)  # [x = 2]

# Quadratic equation: x^2 - 5*x + 6 = 0
solutions = solve(x**2 - 5*x + 6, 0, x)
print(solutions)  # [x = 2, x = 3]

# Multiple variables
x, y = Symbol('x'), Symbol('y')
solutions = solve([x + y - 5, x - y - 1], [x, y])
print(solutions)  # {x: 3, y: 2}
```

### Matrix Operations

```python
from mathhook import Matrix, Symbol

x = Symbol('x')

# Create matrix
A = Matrix([
    [1, 2],
    [3, 4]
])

# Matrix operations
det = A.determinant()  # -2
inv = A.inverse()
transpose = A.transpose()

# Symbolic matrices
B = Matrix([
    [x, 1],
    [0, x]
])
det_B = B.determinant()  # x^2
```

### Functions

#### Elementary Functions

```python
from mathhook import (
    sin, cos, tan, asin, acos, atan,     # Trigonometric
    sinh, cosh, tanh,                     # Hyperbolic
    exp, ln, log, sqrt,                   # Exponential/Logarithmic
    abs, sign, floor, ceil, round         # Rounding/Sign
)

x = Symbol('x')

# Trigonometric
expr = sin(x)**2 + cos(x)**2
result = simplify(expr)  # 1

# Inverse trigonometric
expr = asin(sin(x))
result = simplify(expr)  # x (with domain assumptions)

# Hyperbolic
expr = sinh(x)**2 - cosh(x)**2
result = simplify(expr)  # -1

# Exponential and logarithmic
expr = exp(ln(x))
result = simplify(expr)  # x

# Square root
expr = sqrt(x**2)
result = simplify(expr)  # |x| (with assumption handling)

# Rounding and sign functions
expr = floor(2.7)  # 2
expr = ceil(2.3)   # 3
expr = round(2.5)  # 3 (rounds to nearest even)
expr = sign(-5)    # -1
expr = abs(-5)     # 5
```

#### Special Functions

```python
from mathhook import (
    gamma, factorial, digamma,           # Gamma functions
    zeta, erf, erfc,                     # Special functions
    polygamma, bessel_j, bessel_y, beta  # Advanced special functions
)

x, n = Symbol('x'), Symbol('n')

# Gamma function and factorial
expr = gamma(x + 1) / gamma(x)
result = simplify(expr)  # x

expr = factorial(5)
result = simplify(expr)  # 120

# Digamma (logarithmic derivative of gamma)
expr = digamma(1)  # -γ (Euler-Mascheroni constant)

# Riemann zeta function
expr = zeta(2)  # π²/6

# Error functions
expr = erf(0)   # 0
expr = erfc(0)  # 1

# Polygamma function (nth derivative of digamma)
expr = polygamma(1, x)  # Trigamma function

# Bessel functions
expr = bessel_j(0, x)  # Bessel function of first kind
expr = bessel_y(0, x)  # Bessel function of second kind

# Beta function
expr = beta(2, 3)  # Β(2,3) = 1/12
```

#### Number Theory Functions

```python
from mathhook import gcd, lcm, modulo, isprime

# Greatest common divisor
result = gcd(12, 18)  # 6

# Least common multiple
result = lcm(4, 6)  # 12

# Modulo operation
result = modulo(17, 5)  # 2

# Primality testing
result = isprime(17)  # True
result = isprime(18)  # False

# Symbolic number theory
x, y = Symbol('x'), Symbol('y')
expr = gcd(x, y)  # Symbolic GCD
```

#### Polynomial Functions

```python
from mathhook import degree, roots, parse

x = Symbol('x')

# Polynomial degree
poly = parse('x^3 + 2*x^2 + x + 1')
deg = degree(poly, 'x')  # 3

# Polynomial roots
poly = parse('x^2 - 5*x + 6')
solutions = roots(poly, 'x')  # {2, 3}

# Works with symbolic coefficients
a, b, c = Symbol('a'), Symbol('b'), Symbol('c')
poly = a*x**2 + b*x + c
deg = degree(poly, 'x')  # 2
```

### Constants

```python
from mathhook import pi, e, I, oo

# Mathematical constants
expr = parse("sin(pi)")
result = simplify(expr)  # 0 (exact)

expr = exp(I * pi)
result = simplify(expr)  # -1 (Euler's identity)

# Infinity
expr = limit(1/x, x, 0, direction='+')  # oo (infinity)
```

---

## Advanced Features

### LaTeX Input and Output

```python
from mathhook import parse, to_latex

# Parse LaTeX
expr = parse(r"\frac{x^2 + 1}{x - 1}")

# Convert to LaTeX
latex_str = to_latex(expr)
print(latex_str)  # \frac{x^{2} + 1}{x - 1}
```

### Step-by-Step Explanations

```python
from mathhook import Symbol, explain_steps

x = Symbol('x')
expr = (x + 1)**2

# Get step-by-step expansion
steps = explain_steps(expr, operation='expand')

for step in steps:
    print(f"{step['title']}: {step['expression']}")
    print(f"  Explanation: {step['description']}")

# Output:
# Step 1: Original expression: (x + 1)^2
# Step 2: Apply power rule: (x + 1) * (x + 1)
# Step 3: Multiply: x^2 + x + x + 1
# Step 4: Combine like terms: x^2 + 2*x + 1
```

### Assumptions System

```python
from mathhook import Symbol

# Symbol with assumptions
x = Symbol('x', positive=True)
y = Symbol('y', real=True, nonzero=True)

expr = sqrt(x**2)
result = simplify(expr)  # x (not |x|, because x > 0)

# Query assumptions
print(x.is_positive)  # True
print(x.is_real)  # True (implied by positive)
```

### Performance Configuration

```python
from mathhook import set_config

# Configure for Python context
set_config({
    'parallel': True,          # Enable parallel processing
    'simd': True,              # Enable SIMD operations
    'cache_size': 10000,       # Expression cache size
    'simplify_auto': False,    # Don't auto-simplify
})
```

---

## Integration with NumPy

```python
import numpy as np
from mathhook import Symbol, lambdify

x = Symbol('x')
expr = x**2 + 2*x + 1

# Convert to NumPy-compatible function
f = lambdify(expr, [x], 'numpy')

# Evaluate on NumPy array
x_values = np.linspace(-5, 5, 100)
y_values = f(x_values)

# Use with NumPy operations
mean = np.mean(y_values)
std = np.std(y_values)
```

---

## Integration with Matplotlib

```python
import matplotlib.pyplot as plt
import numpy as np
from mathhook import Symbol, lambdify, derivative

x = Symbol('x')
expr = x**3 - 3*x**2 + 2

# Convert expression and derivative to NumPy functions
f = lambdify(expr, [x], 'numpy')
df = lambdify(derivative(expr, x), [x], 'numpy')

# Plot
x_values = np.linspace(-2, 4, 200)
plt.plot(x_values, f(x_values), label='f(x)')
plt.plot(x_values, df(x_values), label="f'(x)")
plt.legend()
plt.grid()
plt.show()
```

---

## Performance Best Practices

### 1. Reuse Symbols

```python
# GOOD: Create symbol once
x = Symbol('x')
for i in range(1000):
    expr = x**2 + i*x

# BAD: Create symbol repeatedly
for i in range(1000):
    x = Symbol('x')  # Unnecessary interning overhead
    expr = x**2 + i*x
```

### 2. Use Operator Overloading

```python
# GOOD: Pythonic operators (optimized path)
expr = x**2 + 2*x + 1

# LESS GOOD: Explicit construction (more overhead)
expr = Expression.add(
    x.pow(2),
    Expression.mul(2, x),
    1
)
```

### 3. Simplify Strategically

```python
# GOOD: Simplify only when needed
expr = build_complex_expression()
# ... do many operations ...
final = simplify(expr)  # Simplify once at end

# BAD: Over-simplification
expr = x + 1
expr = simplify(expr)  # Unnecessary
expr = expr * 2
expr = simplify(expr)  # Unnecessary
```

### 4. Use Lambdify for Numerical Evaluation

```python
# GOOD: Compile to NumPy function for repeated evaluation
f = lambdify(expr, [x], 'numpy')
results = [f(i) for i in range(1000000)]  # Fast

# BAD: Substitute repeatedly
results = [expr.substitute(x, i) for i in range(1000000)]  # Slow
```

---

## Common Pitfalls

### Pitfall 1: Integer Division

```python
# WRONG: Python 3 division
x = Symbol('x')
expr = x / 2  # Creates symbolic x / 2 (correct)

# WRONG: Integer division in coefficient
expr = x * (1 / 3)  # 0.333... (float approximation)

# CORRECT: Use rational
from mathhook import Rational
expr = x * Rational(1, 3)  # Exact 1/3
```

### Pitfall 2: Mutability Expectation

```python
# WRONG: Expressions are immutable
expr = x**2
expr.simplify()  # Returns new expression, doesn't modify expr
print(expr)  # Still x**2

# CORRECT
expr = x**2
expr = simplify(expr)  # Assign result
print(expr)  # Simplified
```

---

## API Compatibility

### SymPy Migration

<!-- See [SymPy Migration Guide](../appendix/sympy-migration.md) for detailed comparison. -->

**Quick Reference**:

| SymPy | MathHook |
|-------|----------|
| `Symbol('x')` | `Symbol('x')` |
| `sympify('x**2')` | `parse('x^2')` |
| `simplify(expr)` | `simplify(expr)` |
| `expand(expr)` | `expand(expr)` |
| `factor(expr)` | `factor(expr)` |
| `diff(expr, x)` | `derivative(expr, x)` |
| `integrate(expr, x)` | `integrate(expr, x)` |
| `solve(eq, x)` | `solve(eq, 0, x)` |
| `latex(expr)` | `to_latex(expr)` |

---

## Next Steps

- [Node.js API Guide](./nodejs.md) - JavaScript/TypeScript bindings
<!-- - [SymPy Migration Guide](../appendix/sympy-migration.md) - Port existing code -->
- [Performance Benchmarks](../performance/benchmarking.md) - Detailed comparisons
