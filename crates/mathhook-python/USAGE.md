# MathHook Python Usage Guide

Updated: 2025-12-27T23:30

Comprehensive guide for using MathHook Python bindings.

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Symbol Creation](#symbol-creation)
4. [Expression Creation](#expression-creation)
5. [Operators](#operators)
6. [Mathematical Functions](#mathematical-functions)
7. [Algebraic Operations](#algebraic-operations)
8. [Calculus](#calculus)
9. [Equation Solving](#equation-solving)
10. [ODE Solving](#ode-solving)
11. [PDE Solving](#pde-solving)
12. [Parsing](#parsing)
13. [Display and Formatting](#display-and-formatting)
14. [Advanced Features](#advanced-features)
15. [Performance Tips](#performance-tips)

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
from mathhook import symbol, symbols, parse, solve, sin, cos, exp

# Create symbols
x = symbol('x')
y = symbol('y')

# Build expressions using Python operators
expr = x**2 + 2*x + 1

# Simplify
simplified = expr.simplify()

# Differentiate
derivative = expr.derivative(symbol('x'))

# Solve equations
solutions = solve(x**2 - 4, 'x')  # Returns [2, -2]
```

## Symbol Creation

### Single Symbol

```python
from mathhook import symbol

x = symbol('x')
y = symbol('y')
theta = symbol('theta')
```

### Multiple Symbols

```python
from mathhook import symbols

# Space-separated
x, y, z = symbols('x y z')

# Comma-separated
a, b, c = symbols('a,b,c')

# Range syntax (creates x0, x1, x2)
x0, x1, x2 = symbols('x0:3')
```

## Expression Creation

### Using Expression Class

```python
from mathhook import Expression

# Integers
two = Expression.integer(2)
forty_two = Expression.integer(42)

# Floats
pi_approx = Expression.float(3.14159)

# Rationals
half = Expression.rational(1, 2)
three_quarters = Expression.rational(3, 4)

# Constants
pi = Expression.pi()
phi = Expression.golden_ratio()

# Square root
sqrt_2 = Expression.sqrt(Expression.integer(2))

# Factorial
fact_5 = Expression.factorial(Expression.integer(5))

# Sets
s = Expression.set([Expression.integer(1), Expression.integer(2), Expression.integer(3)])

# Intervals
interval = Expression.interval(
    Expression.integer(0),
    Expression.integer(10),
    True,   # start inclusive
    False   # end exclusive
)
```

## Operators

MathHook supports Python operator overloading:

```python
from mathhook import symbol

x = symbol('x')
y = symbol('y')

# Addition
sum_expr = x + y

# Subtraction
diff_expr = x - y

# Multiplication
product = x * y

# Division
quotient = x / y

# Power/Exponentiation
squared = x ** 2
cubed = x ** 3

# Negation
negative = -x

# Compound expressions
polynomial = x**2 + 2*x + 1
rational = (x + 1) / (x - 1)
nested = ((x + y) ** 2) / (x - y)
```

## Mathematical Functions

### Trigonometric Functions

```python
from mathhook import symbol, sin, cos, tan, asin, acos, atan

x = symbol('x')

# Basic trig
sine = sin(x)
cosine = cos(x)
tangent = tan(x)

# Inverse trig
arcsine = asin(x)
arccosine = acos(x)
arctangent = atan(x)

# Pythagorean identity
identity = sin(x)**2 + cos(x)**2  # Simplifies to 1
```

### Hyperbolic Functions

```python
from mathhook import symbol, sinh, cosh, tanh

x = symbol('x')

hyperbolic_sine = sinh(x)
hyperbolic_cosine = cosh(x)
hyperbolic_tangent = tanh(x)
```

### Exponential and Logarithmic

```python
from mathhook import symbol, exp, log, ln, sqrt

x = symbol('x')

# Exponential
exponential = exp(x)

# Natural logarithm
natural_log = ln(x)

# Logarithm with base
log_base_10 = log(x, 10)
log_base_2 = log(x, 2)

# Square root
square_root = sqrt(x)
```

### Special Functions

```python
from mathhook import symbol, gamma, factorial, erf, erfc, zeta, digamma
from mathhook import bessel_j, bessel_y, beta, polygamma

x = symbol('x')
n = symbol('n')

# Gamma and factorial
gamma_x = gamma(x)
fact_n = factorial(n)

# Error functions
error_func = erf(x)
complementary_erf = erfc(x)

# Zeta function
riemann_zeta = zeta(x)

# Digamma function
psi = digamma(x)

# Polygamma
polygamma_1 = polygamma(1, x)

# Bessel functions
bessel_first = bessel_j(n, x)
bessel_second = bessel_y(n, x)

# Beta function
beta_func = beta(x, n)
```

### Utility Functions

```python
from mathhook import symbol, abs_expr, sign, floor, ceil, round
from mathhook import gcd, lcm, modulo, isprime

x = symbol('x')
a = symbol('a')
b = symbol('b')

# Absolute value (note: abs_expr to avoid conflict with built-in abs)
absolute = abs_expr(x)

# Sign function
sign_x = sign(x)

# Rounding functions
floor_x = floor(x)
ceil_x = ceil(x)
round_x = round(x)

# Number theory
gcd_ab = gcd(a, b)
lcm_ab = lcm(a, b)
mod_ab = modulo(a, b)
is_prime = isprime(a)
```

## Algebraic Operations

### Simplification

```python
from mathhook import symbol

x = symbol('x')

# Basic simplification
expr = x + x + x
simplified = expr.simplify()  # 3*x

# Trigonometric simplification
trig = (sin(x)**2 + cos(x)**2)
trig_simplified = trig.simplify_trigonometric()  # 1

# Rational simplification
rational = (x**2 - 1) / (x - 1)
rational_simplified = rational.simplify_rational()  # x + 1

# Logarithm simplification
log_expr = ln(exp(x))
log_simplified = log_expr.simplify_logarithms()  # x

# Complex simplification
from mathhook import Expression
complex_expr = Expression.simplify_complex(expr)

# Factorial simplification
fact_expr = factorial(n + 1) / factorial(n)
fact_simplified = fact_expr.simplify_factorial()  # n + 1

# Matrix simplification
matrix_simplified = matrix_expr.simplify_matrix()
```

### Expansion

```python
from mathhook import symbol

x = symbol('x')
y = symbol('y')

# Expand products
expr = (x + 1) * (x + 2)
expanded = expr.expand()  # x^2 + 3*x + 2

# Expand powers
power_expr = (x + y) ** 3
expanded_power = power_expr.expand()
```

### Factorization

```python
from mathhook import symbol

x = symbol('x')

# Factor polynomials
expr = x**2 - 1
factored = expr.factor()  # (x - 1)(x + 1)

# Factor out GCD
expr2 = 2*x**2 + 4*x
factored_gcd = expr2.factor_gcd()

# Factor common terms
expr3 = x*y + x*z
factored_common = expr3.factor_common()
```

### Substitution

```python
from mathhook import symbol, Expression

x = symbol('x')
y = symbol('y')

expr = x**2 + 2*x + 1

# Single substitution
result = expr.subs(x, Expression.integer(3))

# Multiple substitutions using dictionary
substitutions = {'x': Expression.integer(2), 'y': Expression.integer(3)}
result_multi = (x + y).substitute(substitutions)

# Substitute and simplify
result_simplified = (x + y).substitute_and_simplify(substitutions)
```

### Polynomial Operations

```python
from mathhook import symbol, degree, roots

x = symbol('x')

poly = x**3 + 2*x**2 + x + 1

# Get degree
deg = poly.degree(symbol('x'))  # 3

# Or use the function
deg = degree(poly, 'x')  # 3

# Find roots
poly_roots = roots(poly, 'x')

# Polynomial division
quotient = poly.polynomial_quotient(x - 1)
remainder = poly.polynomial_remainder(x - 1)

# GCD of polynomials
poly2 = x**2 - 1
gcd_poly = poly.polynomial_gcd(poly2)

# Primitive part and content
primitive = poly.primitive_part()
content = poly.polynomial_content()
```

## Calculus

### Derivatives

```python
from mathhook import symbol, sin, cos

x = symbol('x')

# First derivative
expr = x**3
derivative = expr.derivative(symbol('x'))  # 3*x^2

# Higher-order derivatives
second_deriv = expr.nth_derivative(symbol('x'), 2)  # 6*x
third_deriv = expr.nth_derivative(symbol('x'), 3)   # 6

# Using static method
from mathhook import Expression
deriv = Expression.derivative_with_expression_variable_order(expr, symbol('x'), 2)

# Step-by-step derivative explanation
steps = expr.derivative_with_steps(symbol('x'), 1)
for step in steps.steps:
    print(f"{step.title}: {step.description}")
    print(f"  Before: {step.before}")
    print(f"  After: {step.after}")
```

### Integration

```python
from mathhook import symbol

x = symbol('x')

# Indefinite integral
expr = x**2
integral = expr.integrate(symbol('x'), 0)  # x^3/3

# With depth parameter for complex integrals
complex_integral = sin(x).integrate(symbol('x'), 5)
```

### Limits

```python
from mathhook import symbol, Expression

x = symbol('x')

# Limit at a point
expr = (x**2 - 1) / (x - 1)
lim = expr.limit(symbol('x'), Expression.integer(1))  # 2

# Limit at infinity
expr2 = 1 / x
lim_inf = expr2.limit_at_infinity(symbol('x'))  # 0

# Limit at negative infinity
lim_neg_inf = expr2.limit_at_negative_infinity(symbol('x'))  # 0

# Directed limit (left or right)
from mathhook import LimitDirection
lim_right = expr.limit_directed(symbol('x'), Expression.integer(0), LimitDirection.Right)
```

### Series Expansion

```python
from mathhook import symbol

x = symbol('x')

# Taylor series
expr = sin(x)
series = expr.taylor_series(symbol('x'), Expression.integer(0), 5)

# Maclaurin series (Taylor at 0)
maclaurin = expr.maclaurin_series(symbol('x'), 5)

# Laurent series
laurent = expr.laurent_series(symbol('x'), Expression.integer(0), 5, 5)

# Puiseux series
puiseux = expr.puiseux_series(symbol('x'), Expression.integer(0), 5)

# Asymptotic series
asymptotic = expr.asymptotic_series(symbol('x'), 5)
```

### Summation

```python
from mathhook import symbol, Expression

n = symbol('n')
k = symbol('k')

# Finite sum
expr = k**2
finite_sum = expr.finite_sum(symbol('k'), Expression.integer(1), Expression.integer(10))

# Infinite sum
geometric = (Expression.rational(1, 2))**k
infinite_sum = geometric.infinite_sum(symbol('k'))

# Partial sum
partial = expr.partial_sum(symbol('k'), Expression.integer(5))
```

## Equation Solving

### Basic Equation Solving

```python
from mathhook import symbol, solve

x = symbol('x')

# Linear equations
solutions = solve(2*x + 3 - 7, 'x')  # [2]

# Quadratic equations
solutions = solve(x**2 - 5*x + 6, 'x')  # [2, 3]

# Complex solutions
solutions = solve(x**2 + 1, 'x')  # [i, -i]

# Higher degree polynomials
solutions = solve(x**3 - 8, 'x')
```

### Fast Path Solvers

```python
from mathhook import symbol

x = symbol('x')

# Direct linear solver (skips classification)
expr = 2*x + 3
result = expr.solve_linear(symbol('x'))

# Direct quadratic solver
expr2 = x**2 - 4
result2 = expr2.solve_quadratic(symbol('x'))

# Direct polynomial solver
expr3 = x**4 - 1
result3 = expr3.solve_polynomial(symbol('x'))

# Matrix equation solver
A = symbol('A')  # Matrix symbol
result_matrix = matrix_eq.solve_matrix_equation(symbol('X'))
```

### Solver Results

```python
from mathhook import symbol, solve

x = symbol('x')
result = solve(x**2 - 4, 'x')

# Result is a list of expressions
for solution in result:
    print(solution)

# Check number of solutions
print(f"Found {len(result)} solutions")
```

## ODE Solving

### Numerical ODE Methods

```python
from mathhook import PyODESolver, symbol, Expression

solver = PyODESolver()
x = symbol('x')
y = symbol('y')

# Define ODE: dy/dx = f(x, y)
ode = x * y  # dy/dx = x*y

# Euler method
solution_euler = solver.euler(
    ode,
    'x',        # independent variable name
    'y',        # dependent variable name
    0.0,        # x0 (initial x)
    1.0,        # y0 (initial y)
    1.0,        # x_end (final x)
    0.1         # step size
)

# Returns list of (x, y) tuples
for x_val, y_val in solution_euler:
    print(f"x = {x_val:.2f}, y = {y_val:.4f}")

# Runge-Kutta 4th order (RK4)
solution_rk4 = solver.runge_kutta_4(
    ode, 'x', 'y', 0.0, 1.0, 1.0, 0.1
)

# Adaptive Runge-Kutta-Fehlberg (RKF45)
solution_rkf45 = solver.runge_kutta_45(
    ode, 'x', 'y', 0.0, 1.0, 1.0,
    tolerance=1e-6,      # error tolerance
    initial_step=0.1     # initial step size
)
```

## PDE Solving

### PDE Solver

```python
from mathhook import PyPDESolver, symbol, Expression

solver = PyPDESolver()

# Heat equation: du/dt = alpha * d²u/dx²
alpha = Expression.float(0.01)
solution_heat = solver.solve_heat_equation(
    alpha,
    'x',                        # spatial variable
    't',                        # time variable
    Expression.integer(10)      # domain length
)

# Wave equation: d²u/dt² = c² * d²u/dx²
c = Expression.integer(1)  # wave speed
solution_wave = solver.solve_wave_equation(
    c,
    'x',
    't',
    Expression.integer(10)
)

# Laplace equation: d²u/dx² + d²u/dy² = 0
solution_laplace = solver.solve_laplace_equation(['x', 'y'])

# General PDE solving
u = symbol('u')
x = symbol('x')
t = symbol('t')
equation = u  # Your PDE equation (= 0)
solution = solver.solve(equation, 'u', ['x', 't'])
```

## Parsing

### Parse Mathematical Expressions

```python
from mathhook import parse

# Standard notation
expr = parse('x^2 + 2*x + 1')

# Implicit multiplication
expr = parse('2x')        # Parsed as 2*x
expr = parse('xy')        # Parsed as x*y
expr = parse('3xy^2')     # Parsed as 3*x*y^2

# Function calls
expr = parse('sin(x) + cos(x)')
expr = parse('exp(x) * log(y)')

# LaTeX notation
expr = parse(r'\frac{x^2}{2}')
expr = parse(r'\sqrt{x + 1}')

# Complex expressions
expr = parse('sin(x)^2 + cos(x)^2')
expr = parse('(x + 1) / (x - 1)')
```

## Display and Formatting

### Pretty Printing

```python
from mathhook import pprint, symbol

x = symbol('x')
expr = x**2 + 2*x + 1

# Pretty print with Unicode (default)
pprint(expr)  # Shows: x² + 2·x + 1

# Pretty print without Unicode
pprint(expr, use_unicode=False)  # Shows: x^2 + 2*x + 1
```

### Initialize Printing

```python
from mathhook import init_printing

# Enable LaTeX rendering (for Jupyter)
init_printing()

# Disable LaTeX
init_printing(use_latex=False)

# Use PNG rendering
init_printing(latex_mode='png')

# Use SVG rendering
init_printing(latex_mode='svg')

# Configure Unicode output
init_printing(unicode=True)
```

### String Representation

```python
from mathhook import symbol

x = symbol('x')
expr = x**2 + 1

# String representation
print(str(expr))   # x^2 + 1

# Debug representation
print(repr(expr))  # PyExpression(x^2 + 1)
```

## Advanced Features

### Groebner Basis

```python
from mathhook import PyGroebnerBasis, parse

# Create polynomials
f1 = parse('x^2 + y^2 - 1')
f2 = parse('x - y')

# Create Groebner basis
gb = PyGroebnerBasis(
    [f1, f2],           # polynomials
    ['x', 'y'],         # variables
    'lex'               # monomial ordering: 'lex', 'grlex', 'grevlex'
)

# Compute the basis
gb.compute()

# Get basis polynomials
basis = gb.get_basis()
for poly in basis:
    print(poly)

# Test ideal membership
test_poly = parse('x^2 - 1')
is_member = gb.contains(test_poly)

# Reduce to minimal form
gb.reduce()

# Check if reduced
is_reduced = gb.is_reduced()
```

### Pattern Matching

```python
from mathhook import PyPattern, symbol

x = symbol('x')

# Wildcard pattern (matches any expression)
pattern = PyPattern.wildcard('x')

# Exact pattern (matches specific expression)
pattern = PyPattern.exact(x**2)
```

### Fast Polynomial Operations (PolyZp)

```python
from mathhook import poly_zp, poly_gcd, poly_mul_fast

# Create polynomial over Z/pZ (integers mod p)
coeffs = [1, 2, 3]  # 1 + 2x + 3x^2
modulus = 7
poly = poly_zp(coeffs, modulus)

# Fast GCD of polynomials
gcd_result = poly_gcd(poly1, poly2, modulus)

# Fast multiplication
product = poly_mul_fast(poly1, poly2, modulus)
```

### Expression Analysis

```python
from mathhook import symbol

x = symbol('x')
y = symbol('y')

expr = x**2 + y**2 + x*y

# Find all variables
variables = expr.find_variables()
for var in variables:
    print(var)

# Get degree with respect to variable
deg_x = expr.degree(symbol('x'))  # 2

# Get total degree
total_deg = expr.total_degree()  # 2

# Find poles
poles = expr.find_poles(symbol('x'))
```

### Matrix Operations

```python
from mathhook import symbol, Expression

# Matrix operations (when using matrix expressions)
A = symbol('A')  # Matrix symbol

# Matrix transpose
A_T = A.matrix_transpose()

# Matrix addition
B = symbol('B')
sum_matrix = A.matrix_add(B)

# Matrix multiplication
product = A.matrix_multiply(B)

# Matrix determinant
det = A.matrix_determinant()

# Matrix inverse
inv = A.matrix_inverse()

# Eigenvalues
eigenvalues = A.matrix_eigenvalues()
```

### Residue Calculus

```python
from mathhook import symbol, Expression

z = symbol('z')

expr = 1 / (z * (z - 1))

# Calculate residue at a pole
residue = expr.residue(symbol('z'), Expression.integer(0))

# Find all poles
poles = expr.find_poles(symbol('z'))
```

## Performance Tips

### 1. Reuse Symbols

```python
from mathhook import symbol

# Create symbols once
x = symbol('x')
y = symbol('y')

# Reuse them
expr1 = x**2 + y
expr2 = x + y**2
```

### 2. Use Fast Path Solvers

```python
from mathhook import symbol

x = symbol('x')
expr = x**2 - 4

# Use fast path when you know the equation type
result = expr.solve_quadratic(symbol('x'))  # Faster than solve()
```

### 3. Simplify Incrementally

```python
from mathhook import symbol

x = symbol('x')

# Simplify at key stages, not on every operation
step1 = (x + 1) ** 10
step2 = step1.expand()
result = step2.simplify()  # Simplify once at the end
```

### 4. Use Appropriate Precision

```python
from mathhook import Expression

# Use integers when possible (exact)
two = Expression.integer(2)

# Use rationals for fractions (exact)
half = Expression.rational(1, 2)

# Use floats only when necessary (approximate)
pi_approx = Expression.float(3.14159)
```

### 5. Batch Operations

```python
from mathhook import symbols, parse

# Parse once
exprs = [parse(f'x^{i}') for i in range(10)]

# Process in batch
simplified = [e.simplify() for e in exprs]
```

## Common Patterns

### Quadratic Formula

```python
from mathhook import symbol, solve, sqrt

x = symbol('x')
a, b, c = symbol('a'), symbol('b'), symbol('c')

# Solve ax^2 + bx + c = 0
quadratic = a*x**2 + b*x + c
solutions = solve(quadratic, 'x')
```

### Taylor Series Approximation

```python
from mathhook import symbol, sin, Expression

x = symbol('x')

# Get Taylor series of sin(x) around 0
sin_approx = sin(x).taylor_series(symbol('x'), Expression.integer(0), 5)
print(sin_approx)  # x - x^3/6 + x^5/120
```

### Numerical Integration via ODE

```python
from mathhook import PyODESolver, symbol, Expression

# Integrate f(x) = x^2 from 0 to 1
# Use dy/dx = x^2, y(0) = 0
solver = PyODESolver()
ode = symbol('x') ** 2

result = solver.runge_kutta_4(
    ode, 'x', 'y', 0.0, 0.0, 1.0, 0.01
)
# Last y value is the integral
print(f"Integral: {result[-1][1]}")  # ~0.333
```

## Error Handling

```python
from mathhook import parse, symbol

# Parse errors
try:
    expr = parse('x +* y')  # Invalid syntax
except ValueError as e:
    print(f"Parse error: {e}")

# Symbol operations are safe
x = symbol('x')
y = symbol('y')
expr = x / y  # Works even when y might be 0 symbolically
```

## Further Reading

- [API Reference](https://mathhook.readthedocs.io)
- [Examples Repository](https://github.com/AhmedMashour/mathhook/tree/main/examples)
- [Rust Core Documentation](../../README.md)
- [Performance Guide](../../docs/performance.md)
