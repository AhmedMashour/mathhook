# MathHook Python Usage Guide

Comprehensive guide for using MathHook in Python.

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Expression Creation](#expression-creation)
4. [Algebraic Operations](#algebraic-operations)
5. [Calculus](#calculus)
6. [Solving Equations](#solving-equations)
7. [Matrix Operations](#matrix-operations)
8. [Parsing and Formatting](#parsing-and-formatting)
9. [Educational Features](#educational-features)
10. [Performance Tips](#performance-tips)
11. [Integration with NumPy/SciPy](#integration-with-numpy-scipy)
12. [Jupyter Notebook Support](#jupyter-notebook-support)

## Installation

```bash
pip install mathhook
```

For development:

```bash
# Clone repository
git clone https://github.com/AhmedMashour/mathhook.git
cd mathhook/crates/mathhook-python

# Install in editable mode
pip install maturin
maturin develop
```

## Quick Start

```python
from mathhook import symbol, sin, cos, solve

# Create symbols
x = symbol('x')

# Build and simplify expressions
expr = (x + 1)**2
expanded = expr.expand()  # x^2 + 2*x + 1

# Calculus
derivative = expr.diff(x)  # 2*x + 2
integral = expr.integrate(x)  # x^3/3 + x^2 + x

# Solve equations
solutions = solve(x**2 - 4, x)  # [2, -2]
```

## Expression Creation

### Basic Types

```python
from mathhook import Expression

# Numbers
integer = Expression.integer(42)
rational = Expression.rational(3, 4)  # 3/4
floating = Expression.float(3.14159)

# Complex numbers
from mathhook import I
complex_num = 3 + 4*I
```

### Symbols

```python
from mathhook import symbol, symbols

# Single symbol
x = symbol('x')

# Multiple symbols
x, y, z = symbols('x y z')

# With assumptions (future feature)
# n = symbol('n', integer=True, positive=True)
```

### Using Operators

```python
from mathhook import symbol

x = symbol('x')
y = symbol('y')

# Arithmetic
addition = x + y
subtraction = x - y
multiplication = x * y
division = x / y
power = x ** 2

# Compound expressions
poly = x**2 + 2*x + 1
rational = (x + 1) / (x - 1)
nested = ((x + y) ** 2) / (x - y)
```

### Mathematical Functions

```python
from mathhook import (
    # Trigonometric
    sin, cos, tan, cot, sec, csc,
    asin, acos, atan, atan2,

    # Hyperbolic
    sinh, cosh, tanh,
    asinh, acosh, atanh,

    # Exponential and logarithmic
    exp, log, ln, log10,

    # Roots and powers
    sqrt, cbrt, root,

    # Special functions
    factorial, gamma, beta,
    erf, erfc,
)

x = symbol('x')

# Trigonometric
trig = sin(x)**2 + cos(x)**2  # = 1

# Exponential
exponential = exp(x)
logarithm = log(x, 10)  # log base 10
natural_log = ln(x)      # natural log

# Special
fact = factorial(5)  # 120
```

### Constants

```python
from mathhook import pi, e, I, oo, zoo, nan

# Pi
circle_area = pi * r**2

# Euler's number
growth = e**t

# Imaginary unit
complex_num = 3 + 4*I

# Infinity
limit_result = 1 / oo  # 0

# Complex infinity
complex_inf = zoo

# Not a number
undefined = nan
```

## Algebraic Operations

### Simplification

```python
from mathhook import symbol, sin, cos, simplify

x = symbol('x')

# Automatic simplification
expr = x + x + x
print(expr)  # 3*x

# Explicit simplification
expr = (x**2 - 1) / (x - 1)
simplified = simplify(expr)  # x + 1

# Trigonometric simplification
expr = sin(x)**2 + cos(x)**2
simplified = simplify(expr)  # 1

# Rational simplification
from mathhook import cancel
expr = (x**2 + 2*x + 1) / (x + 1)
simplified = cancel(expr)  # x + 1
```

### Expansion

```python
from mathhook import symbol, expand

x, y = symbols('x y')

# Expand products
expr = (x + 1) * (x + 2)
expanded = expand(expr)  # x^2 + 3*x + 2

# Expand powers
expr = (x + y)**3
expanded = expand(expr)  # x^3 + 3*x^2*y + 3*x*y^2 + y^3

# Expand complex expressions
expr = (x + 1)**2 * (x - 1)
expanded = expand(expr)  # x^3 + x^2 - x - 1
```

### Factorization

```python
from mathhook import symbol, factor

x = symbol('x')

# Factor polynomials
expr = x**2 - 1
factored = factor(expr)  # (x - 1)(x + 1)

expr = x**2 + 5*x + 6
factored = factor(expr)  # (x + 2)(x + 3)

# Factor over different domains
expr = x**2 + 1
factored = factor(expr)  # x^2 + 1 (irreducible over reals)
factored_complex = factor(expr, domain='complex')  # (x - I)(x + I)
```

### Collecting Terms

```python
from mathhook import symbol, collect

x, y = symbols('x y')

# Collect coefficients
expr = x*y + x*y**2 + y**2 + y
collected = collect(expr, x)  # x*(y + y^2) + y^2 + y
collected = collect(expr, y)  # y*(x + 1) + y^2*(x + 1)
```

### Substitution

```python
from mathhook import symbol

x, y, z = symbols('x y z')

expr = x**2 + 2*x + 1

# Substitute single value
result = expr.subs(x, 3)  # 16

# Substitute expression
result = expr.subs(x, y + 1)  # (y + 1)^2 + 2*(y + 1) + 1

# Multiple substitutions
expr = x + y + z
result = expr.subs({x: 1, y: 2, z: 3})  # 6

# Simultaneous substitution
expr = x + y
result = expr.subs({x: y, y: x})  # Swaps x and y
```

## Calculus

### Derivatives

```python
from mathhook import symbol, sin, cos, exp, diff

x = symbol('x')

# First derivative
expr = x**3
derivative = diff(expr, x)  # 3*x^2

# Higher-order derivatives
second_deriv = diff(expr, x, 2)  # 6*x
third_deriv = diff(expr, x, 3)  # 6

# Using method
derivative = expr.diff(x)
second_deriv = expr.diff(x, 2)

# Partial derivatives
from mathhook import symbols
x, y, z = symbols('x y z')
expr = x**2 * y + y**2 * z

dx = diff(expr, x)  # 2*x*y
dy = diff(expr, y)  # x^2 + 2*y*z
dz = diff(expr, z)  # y^2

# Mixed partials
d2xy = diff(expr, x, y)  # 2*x
```

### Integrals

```python
from mathhook import symbol, sin, cos, integrate

x = symbol('x')

# Indefinite integrals
expr = x**2
integral = integrate(expr, x)  # x^3/3

# Definite integrals
result = integrate(expr, (x, 0, 1))  # 1/3

# Multiple integrals
from mathhook import symbols
x, y = symbols('x y')
expr = x * y
double_integral = integrate(integrate(expr, x), y)  # x^2*y^2/4

# Limits of integration
result = integrate(expr, (x, 0, 1), (y, 0, 2))

# Trigonometric integrals
integral = integrate(sin(x), x)  # -cos(x)
```

### Limits

```python
from mathhook import symbol, sin, limit, oo

x = symbol('x')

# Finite limits
expr = sin(x) / x
lim = limit(expr, x, 0)  # 1

# Limits at infinity
expr = 1 / x
lim = limit(expr, x, oo)  # 0

# One-sided limits
expr = 1 / x
lim_right = limit(expr, x, 0, '+')  # oo
lim_left = limit(expr, x, 0, '-')   # -oo

# Complex limits
from mathhook import sqrt
expr = sqrt(x) / x
lim = limit(expr, x, oo)  # 0
```

### Series Expansions

```python
from mathhook import symbol, sin, cos, exp, series

x = symbol('x')

# Taylor series around 0
series_sin = series(sin(x), x, 0, 6)
# x - x^3/6 + x^5/120 + O(x^6)

# Series around different point
series_exp = series(exp(x), x, 1, 4)
# e + e*(x-1) + e*(x-1)^2/2 + e*(x-1)^3/6 + O((x-1)^4)

# Laurent series (for functions with poles)
expr = 1 / x
laurent = series(expr, x, 0, 3)
# 1/x + O(x^2)

# Remove order term
series_pure = series_sin.removeO()
```

## Solving Equations

### Algebraic Equations

```python
from mathhook import symbol, solve

x = symbol('x')

# Linear equations
solutions = solve(2*x + 3 - 7, x)  # [2]

# Quadratic equations
solutions = solve(x**2 - 5*x + 6, x)  # [2, 3]

# Complex solutions
solutions = solve(x**2 + 1, x)  # [I, -I]

# Cubic and higher
solutions = solve(x**3 - 8, x)  # [2, -1 - sqrt(3)*I, -1 + sqrt(3)*I]

# Multiple solutions
from mathhook import Eq
eq = Eq(x**2, 4)
solutions = solve(eq, x)  # [2, -2]
```

### Systems of Equations

```python
from mathhook import symbols, solve

x, y = symbols('x y')

# Linear system
solutions = solve([
    x + y - 5,
    x - y - 1
], [x, y])
# {x: 3, y: 2}

# Nonlinear system
solutions = solve([
    x**2 + y**2 - 25,
    x - y - 1
], [x, y])

# Underdetermined system
x, y, z = symbols('x y z')
solutions = solve([
    x + y + z - 6,
    2*x - y + z - 3
], [x, y, z])
# Parametric solution in terms of z
```

### Differential Equations

```python
from mathhook import symbols, Function, dsolve, Eq

x = symbols('x')
f = Function('f')

# First-order ODE: dy/dx = y
eq = Eq(f(x).diff(x), f(x))
solution = dsolve(eq, f(x))
# f(x) = C1*exp(x)

# Second-order ODE: d²y/dx² + y = 0
eq = Eq(f(x).diff(x, 2) + f(x), 0)
solution = dsolve(eq, f(x))
# f(x) = C1*sin(x) + C2*cos(x)

# With initial conditions
eq = Eq(f(x).diff(x), f(x))
solution = dsolve(eq, f(x), ics={f(0): 1})
# f(x) = exp(x)
```

### Inequality Solving

```python
from mathhook import symbol, solve

x = symbol('x')

# Solve inequalities
solutions = solve(x**2 - 4 < 0, x)
# (-2, 2)

# Multiple inequalities
solutions = solve([x > 0, x < 5], x)
# (0, 5)
```

## Matrix Operations

### Creating Matrices

```python
from mathhook import Matrix, symbols

# From nested lists
A = Matrix([[1, 2], [3, 4]])

# From symbolic expressions
x = symbol('x')
B = Matrix([[x, x**2], [1, x]])

# Special matrices
I = Matrix.eye(3)      # 3x3 identity
Z = Matrix.zeros(2, 3)  # 2x3 zero matrix
O = Matrix.ones(2, 2)   # 2x2 ones matrix
D = Matrix.diag([1, 2, 3])  # Diagonal matrix

# Random matrices
R = Matrix.random(3, 3)  # Random integer matrix
```

### Basic Operations

```python
from mathhook import Matrix

A = Matrix([[1, 2], [3, 4]])
B = Matrix([[5, 6], [7, 8]])

# Addition and subtraction
C = A + B
D = A - B

# Scalar multiplication
E = 2 * A

# Matrix multiplication
F = A * B

# Element-wise operations
G = A.multiply_elementwise(B)

# Transpose
AT = A.T
# or
AT = A.transpose()

# Conjugate transpose
AH = A.H
```

### Matrix Properties

```python
from mathhook import Matrix

A = Matrix([[1, 2], [3, 4]])

# Determinant
det = A.det()  # -2

# Trace
tr = A.trace()  # 5

# Rank
rank = A.rank()  # 2

# Inverse
A_inv = A.inv()

# Adjugate
adj = A.adjugate()

# Condition number
cond = A.condition_number()
```

### Matrix Decomposition

```python
from mathhook import Matrix

A = Matrix([[4, 2], [2, 3]])

# Eigenvalues and eigenvectors
eigenvals = A.eigenvals()
# {5.0: 1, 2.0: 1}

eigenvects = A.eigenvects()
# [(5.0, 1, [Matrix([[...]])]), (2.0, 1, [Matrix([[...]])])]

# LU decomposition
L, U, P = A.LUdecomposition()

# QR decomposition
Q, R = A.QRdecomposition()

# Cholesky decomposition (for positive definite)
L = A.cholesky()

# Singular Value Decomposition
U, S, V = A.singular_value_decomposition()

# Diagonalization
P, D = A.diagonalize()
```

### Solving Linear Systems

```python
from mathhook import Matrix

A = Matrix([[2, 1], [1, 3]])
b = Matrix([5, 6])

# Solve Ax = b
x = A.solve(b)
# Matrix([[1], [3]])

# Least squares solution
x = A.solve_least_squares(b)

# Using LU decomposition
x = A.LUsolve(b)
```

## Parsing and Formatting

### Parsing Expressions

```python
from mathhook import parse

# Standard notation
expr = parse("2*x + sin(y)")

# LaTeX
expr = parse(r"\frac{x^2}{2} + \sqrt{y}")
expr = parse(r"\int_{0}^{1} x^2 \, dx")

# Wolfram Language
expr = parse("Sin[x] + Cos[y]")
expr = parse("D[x^2, x]")

# With explicit format
from mathhook import parse_latex, parse_wolfram
expr = parse_latex(r"\frac{dy}{dx}")
expr = parse_wolfram("D[y, x]")
```

### Output Formatting

```python
from mathhook import symbol

x = symbol('x')
expr = x**2 / 2

# String representation
print(str(expr))  # x^2/2
print(repr(expr))  # Expression(...)

# LaTeX
latex = expr.to_latex()
print(latex)  # \frac{x^{2}}{2}

# Pretty printing
from mathhook import pprint
pprint(expr)
#  2
# x
# ──
# 2

# Unicode
unicode_str = expr.to_unicode()

# Wolfram
wolfram = expr.to_wolfram()
print(wolfram)  # Divide[Power[x, 2], 2]

# Code generation
from mathhook import ccode, pycode
c_code = ccode(expr)  # C code
py_code = pycode(expr)  # Python code
```

### LaTeX Rendering

```python
# In Jupyter
from mathhook import init_printing, symbol
init_printing()

x = symbol('x')
expr = x**2 + 2*x + 1
# Displays as LaTeX in Jupyter
display(expr)
```

## Educational Features

### Step-by-Step Solutions

```python
from mathhook import symbol, solve

x = symbol('x')
equation = x**2 - 5*x + 6

# Get step-by-step solution
steps = solve(equation, x, steps=True)

for i, step in enumerate(steps):
    print(f"Step {i+1}: {step.description}")
    print(f"  Expression: {step.expression}")
    print()

# Output:
# Step 1: Original equation
#   Expression: x^2 - 5*x + 6 = 0
#
# Step 2: Factor the quadratic
#   Expression: (x - 2)(x - 3) = 0
#
# Step 3: Set each factor to zero
#   Expression: x - 2 = 0 or x - 3 = 0
#
# Step 4: Solve for x
#   Expression: x = 2 or x = 3
```

### Derivative Steps

```python
from mathhook import symbol, sin

x = symbol('x')
expr = sin(x**2)

# Get derivative explanation
explanation = expr.diff(x, show_steps=True)

print(explanation)
# Step 1: Apply chain rule to sin(x^2)
# Step 2: Derivative of sin(u) is cos(u)
# Step 3: Derivative of x^2 is 2*x
# Step 4: Result: 2*x*cos(x^2)
```

## Performance Tips

### Configuration

```python
from mathhook import configure

# Use Python-optimized settings
configure(binding='python')

# Custom configuration
configure(
    simd_enabled=True,
    simd_threshold=100,
    cache_size=50000,
    parallel_enabled=False
)
```

### Caching

```python
from mathhook import symbol, cache

x = symbol('x')

# Cache expensive computations
@cache
def expensive_simplification(expr):
    return expr.simplify()

result = expensive_simplification(x**100 + x**99 + 1)
```

### Bulk Operations

```python
from mathhook import simplify_many

# Process many expressions at once
expressions = [x**2 + 2*x + 1 for x in symbols('x1:100')]
simplified = simplify_many(expressions)
```

### Numerical Evaluation

```python
from mathhook import symbol, N

x = symbol('x')
expr = x**2 + 1

# Symbolic evaluation
result = expr.subs(x, 2)  # 5 (exact)

# Numerical evaluation
result = N(expr.subs(x, 2.5))  # 7.25 (float)

# Control precision
from decimal import Decimal
result = N(expr.subs(x, Decimal('2.5')), prec=50)
```

## Integration with NumPy/SciPy

### Converting to NumPy

```python
from mathhook import symbol, lambdify
import numpy as np

x = symbol('x')
expr = x**2 + 2*x + 1

# Create numpy function
f = lambdify(x, expr, 'numpy')

# Evaluate on arrays
x_vals = np.linspace(0, 10, 100)
y_vals = f(x_vals)

# Plot
import matplotlib.pyplot as plt
plt.plot(x_vals, y_vals)
plt.show()
```

### Solving with SciPy

```python
from mathhook import symbol
import scipy.optimize

x = symbol('x')
expr = x**2 - 2

# Convert to numerical function
f = lambdify(x, expr)

# Solve numerically
root = scipy.optimize.fsolve(f, 1.0)
print(root)  # ~1.414
```

## Jupyter Notebook Support

### Setup

```python
from mathhook import init_printing
init_printing(use_latex=True)
```

### Display

```python
from mathhook import symbol
from IPython.display import display, Latex

x = symbol('x')
expr = x**2 + 2*x + 1

# Automatic LaTeX rendering
display(expr)

# Manual LaTeX
display(Latex(expr.to_latex()))
```

### Interactive Widgets

```python
from mathhook import symbol
from ipywidgets import interact

x = symbol('x')

@interact(n=(0, 10))
def plot_polynomial(n):
    expr = sum(x**i for i in range(n+1))
    display(expr)
```

## Best Practices

1. **Use symbols consistently**: Define symbols once at the top
2. **Cache expensive operations**: Use `@cache` for repeated computations
3. **Simplify incrementally**: Simplify at appropriate stages, not everything at once
4. **Use lambdify for numerical work**: Convert to NumPy for performance
5. **Check assumptions**: Be explicit about variable domains when needed
6. **Profile before optimizing**: Use `%timeit` in Jupyter

## Common Patterns

### Polynomial Manipulation

```python
from mathhook import symbol, Poly

x = symbol('x')

# Create polynomial
p = Poly(x**3 + 2*x**2 + x + 1, x)

# Get coefficients
coeffs = p.all_coeffs()  # [1, 2, 1, 1]

# Evaluate
value = p.eval(2)  # 15

# Roots
roots = p.real_roots()
```

### Trigonometric Simplification

```python
from mathhook import symbol, sin, cos, trigsimp

x = symbol('x')

# Simplify trig expressions
expr = sin(x)**2 + cos(x)**2
simplified = trigsimp(expr)  # 1

# Expand trig
from mathhook import expand_trig
expr = sin(x + y)
expanded = expand_trig(expr)  # sin(x)*cos(y) + cos(x)*sin(y)
```

## Troubleshooting

### Common Errors

```python
# TypeError: unsupported operand
# Solution: Ensure both operands are Expression objects
x = symbol('x')
# Wrong: expr = 2 + x  (might work due to __radd__)
# Right: expr = Expression.integer(2) + x

# NameError: symbol not defined
# Solution: Import and define symbols
from mathhook import symbol
x = symbol('x')

# AttributeError: method not found
# Solution: Check API documentation for correct method name
```

## Further Reading

- [API Reference](https://mathhook.readthedocs.io)
- [Examples Repository](https://github.com/AhmedMashour/mathhook/tree/main/examples)
- [Rust Core Documentation](../../README.md)
- [Performance Guide](../../docs/performance.md)
