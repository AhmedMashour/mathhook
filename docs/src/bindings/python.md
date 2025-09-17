# Python API Guide

Complete guide to using MathHook from Python via PyO3 bindings.

**Last Updated:** 2025-12-15T1500

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
from mathhook import PyExpression, symbols, parse

# Create symbols
x, y = symbols('x y')

# Build expressions using operators
expr = x.pow(2).add(x.multiply(2)).add(1)

# Or parse from string
expr = parse('x^2 + 2*x + 1')

# Simplify
simplified = expr.simplify()
print(simplified)  # x^2 + 2*x + 1

# Calculus
derivative = expr.derivative('x')
print(derivative)  # 2*x + 2
```

## Core Classes

### PyExpression

The main class for symbolic expressions. All mathematical operations return `PyExpression` instances.

#### Creating Expressions

**Static constructors:**

```python
from mathhook import PyExpression

# Integers and floats
num = PyExpression.integer(42)
pi_approx = PyExpression.float(3.14159)

# Rationals (exact fractions)
half = PyExpression.rational(1, 2)
third = PyExpression.rational(1, 3)

# Symbols
x = PyExpression.symbol('x')
theta = PyExpression.symbol('theta')

# Mathematical constants
pi = PyExpression.pi()
e = PyExpression.e()
i = PyExpression.i()  # Imaginary unit
phi = PyExpression.golden_ratio()
gamma = PyExpression.euler_gamma()

# Complex numbers
z = PyExpression.complex(PyExpression.integer(3), PyExpression.integer(4))  # 3 + 4i
```

**Using the `symbols()` function (recommended):**

```python
from mathhook import symbols

# Space-separated
x, y, z = symbols('x y z')

# Comma-separated
a, b, c = symbols('a, b, c')

# Range syntax
x0, x1, x2 = symbols('x0:3')
```

**Parsing from strings:**

```python
from mathhook import parse, PyExpression

# Standard notation
expr = parse('x^2 + 2*x + 1')

# LaTeX (auto-detected)
expr = parse(r'\frac{x^2 + 1}{x - 1}')

# Wolfram Language (auto-detected)
expr = parse('Sin[x] + Cos[y]')

# With configuration
expr = PyExpression.parse_with_config('2x + 3y', enable_implicit_multiplication=True)
```

#### Arithmetic Operations

```python
from mathhook import symbols

x, y = symbols('x y')

# Method chaining (returns new PyExpression)
result = x.add(y)           # x + y
result = x.subtract(y)      # x - y
result = x.multiply(y)      # x * y
result = x.divide(y)        # x / y
result = x.pow(2)           # x^2
result = x.negate()         # -x

# Auto-conversion from Python types
result = x.add(2)           # x + 2 (int auto-converted)
result = x.multiply(3.14)   # x * 3.14 (float auto-converted)

# Method chaining
quadratic = x.pow(2).add(x.multiply(2)).add(1)  # x^2 + 2*x + 1
```

#### Algebraic Operations

```python
from mathhook import symbols, parse

x, y = symbols('x y')

# Simplification
expr = parse('x + x + x')
simplified = expr.simplify()  # 3*x

# Expansion
expr = parse('(x + 1)^2')
expanded = expr.expand()  # x^2 + 2*x + 1

# Factorization
expr = parse('x^2 - 1')
factored = expr.factor()  # (x - 1)*(x + 1)

# Collect terms
expr = parse('x*y + x*z + x')
collected = expr.collect('x')  # x*(y + z + 1)

# Substitution
expr = parse('x^2 + y')
substitutions = {'x': PyExpression.integer(3), 'y': PyExpression.integer(1)}
result = expr.substitute(substitutions)  # 10

# Trigonometric simplification
expr = parse('sin(x)^2 + cos(x)^2')
result = expr.trigsimp()  # 1

# Expand trigonometric functions
expr = parse('sin(2*x)')
result = expr.expand_trig()  # 2*sin(x)*cos(x)

# Rational simplification
expr = parse('(x^2 - 1)/(x - 1)')
result = expr.rational_simplify()  # x + 1
```

#### Calculus Operations

```python
from mathhook import symbols, parse, PyExpression

x = symbols('x')[0]

# Derivatives
expr = parse('x^3')
df = expr.derivative('x')      # 3*x^2
d2f = expr.nth_derivative('x', 2)  # 6*x

# Check differentiability
is_diff = expr.is_differentiable('x')  # True

# Partial derivatives
expr = parse('x^2 * y^3')
result = expr.partial_derivative(['x', 'y'])  # 6*x*y^2

# Indefinite integration
expr = parse('x^2')
integral = expr.integrate('x')  # x^3/3

# Definite integration
result = expr.integrate_definite('x', PyExpression.integer(0), PyExpression.integer(2))

# Numerical integration methods
result = expr.integrate_simpson('x', 0.0, 2.0, 100)      # Simpson's rule
result = expr.integrate_gaussian('x', 0.0, 2.0, 10)     # Gaussian quadrature
result = expr.integrate_romberg('x', 0.0, 2.0, 1e-10)   # Romberg integration

# Limits
expr = parse('sin(x)/x')
limit_val = expr.limit('x', PyExpression.integer(0))  # 1
limit_inf = expr.limit_at_infinity('x')  # 0

# Taylor series
expr = parse('sin(x)')
series = expr.taylor_series('x', PyExpression.integer(0), 5)
```

#### Step-by-Step Explanations

```python
from mathhook import parse

expr = parse('(x + 1)^2')

# Get step-by-step simplification
explanation = expr.explain_simplification()
for step in explanation.steps:
    print(f"{step.title}: {step.expression}")
    print(f"  {step.description}")

# Get step-by-step derivative
explanation = expr.derivative_with_steps('x')
for step in explanation.steps:
    print(f"{step.title}: {step.expression}")
```

#### Output Formats

```python
from mathhook import parse

expr = parse('x^2 / 2')

# String representation
print(str(expr))        # x^2/2

# LaTeX
latex = expr.to_latex()
print(latex)            # \frac{x^{2}}{2}

# Simple notation
simple = expr.to_simple()
print(simple)           # x^2/2

# Wolfram Language
wolfram = expr.to_wolfram()
print(wolfram)          # Divide[Power[x, 2], 2]
```

#### Evaluation

```python
from mathhook import parse, PyExpression, EvalContext

expr = parse('x^2 + 2*x + 1')

# Basic evaluation (simplifies and evaluates numerics)
result = expr.evaluate()

# Evaluation with context
ctx = EvalContext.numeric({
    'x': PyExpression.integer(3)
})
result = expr.evaluate_with_context(ctx)  # 16

# Symbolic context (no numerical evaluation)
ctx = EvalContext.symbolic()
result = expr.evaluate_with_context(ctx)
```

### Matrix Operations

```python
from mathhook import PyExpression, symbols

x = symbols('x')[0]

# Create matrices
A = PyExpression.matrix([
    [PyExpression.integer(1), PyExpression.integer(2)],
    [PyExpression.integer(3), PyExpression.integer(4)]
])

# Special matrices
I = PyExpression.identity_matrix(3)
Z = PyExpression.zero_matrix(2, 3)

# Matrix operations
det = A.determinant()       # -2
inv = A.inverse()
trans = A.transpose()
tr = A.trace()

# Eigenvalues and eigenvectors
eigenvals = A.eigenvalues()
char_poly = A.characteristic_polynomial()

# Matrix decompositions
L, U, P = A.lu_decomposition()
Q, R = A.qr_decomposition()
U, S, V = A.svd()
L = A.cholesky_decomposition()  # For positive definite matrices

# Matrix properties
rank = A.rank()
is_pd = A.is_positive_definite()
cond = A.condition_number()

# Matrix functions
A_pow = A.matrix_power(3)
A_exp = A.matrix_exponential()
```

### Complex Number Operations

```python
from mathhook import PyExpression

# Create complex numbers
z = PyExpression.complex(PyExpression.integer(3), PyExpression.integer(4))

# Complex operations
conj = z.complex_conjugate()    # 3 - 4i
mod = z.complex_modulus()       # 5
arg = z.complex_argument()      # arctan(4/3)

# Polar form
r, theta = z.to_polar_form()

# Complex arithmetic
w = PyExpression.complex(PyExpression.integer(1), PyExpression.integer(2))
sum_z = z.complex_add(w)
diff = z.complex_subtract(w)
prod = z.complex_multiply(w)
quot = z.complex_divide(w)

# Type checks
z.is_real()           # False
z.is_imaginary()      # False
z.is_pure_imaginary() # False
```

### Summation and Products

```python
from mathhook import symbols, PyExpression

n, k = symbols('n k')

# Finite sum: Σ(k^2) for k=1 to n
expr = k.pow(2)
result = expr.finite_sum('k', PyExpression.integer(1), n)

# Infinite sum
result = expr.infinite_sum('k', PyExpression.integer(1))

# Finite product: Π(k) for k=1 to n
result = k.finite_product('k', PyExpression.integer(1), n)

# Infinite product
result = k.infinite_product('k', PyExpression.integer(1))
```

### Polynomial Operations

```python
from mathhook import parse, PyExpression, degree, roots

# Degree
poly = parse('x^3 + 2*x^2 + x + 1')
deg = degree(poly, 'x')  # 3

# Roots
poly = parse('x^2 - 5*x + 6')
r = roots(poly, 'x')  # Roots of x^2 - 5*x + 6

# Polynomial division
p1 = parse('x^3 - 1')
p2 = parse('x - 1')
quotient, remainder = PyExpression.polynomial_div(p1, p2, 'x')

# Quotient and remainder separately
q = PyExpression.polynomial_quo(p1, p2, 'x')
r = PyExpression.polynomial_rem(p1, p2, 'x')

# GCD of polynomials
gcd = PyExpression.multivariate_gcd([p1, p2], ['x'])

# Resultant
res = PyExpression.resultant(p1, p2, 'x')

# Discriminant
disc = PyExpression.discriminant(poly, 'x')
```

### Pattern Matching

```python
from mathhook import PyExpression, PyPattern

# Create patterns
wildcard = PyPattern.wildcard('a')  # Matches anything, binds to 'a'
exact = PyPattern.exact(PyExpression.integer(1))  # Matches exactly 1

# Match expressions
expr = parse('x + 1')
matches = expr.matches(wildcard)  # Returns dict of bindings or None

# Replace with pattern
replacement = PyPattern.exact(PyExpression.integer(2))
new_expr = expr.replace(wildcard, replacement)
```

## Mathematical Functions

### Trigonometric Functions

```python
from mathhook import sin, cos, tan, asin, acos, atan, symbols

x = symbols('x')[0]

# Basic trig
expr = sin(x)
expr = cos(x)
expr = tan(x)

# Inverse trig
expr = asin(x)
expr = acos(x)
expr = atan(x)
```

### Hyperbolic Functions

```python
from mathhook import sinh, cosh, tanh, symbols

x = symbols('x')[0]

expr = sinh(x)
expr = cosh(x)
expr = tanh(x)
```

### Exponential and Logarithmic

```python
from mathhook import exp, log, ln, sqrt, symbols

x = symbols('x')[0]

expr = exp(x)           # e^x
expr = ln(x)            # Natural logarithm
expr = log(x)           # Natural logarithm (same as ln)
expr = log(x, 10)       # Log base 10
expr = sqrt(x)          # Square root
```

### Rounding and Sign Functions

```python
from mathhook import floor, ceil, round, sign, abs_expr, symbols

x = symbols('x')[0]

expr = floor(x)         # Floor function
expr = ceil(x)          # Ceiling function
expr = round(x)         # Round to nearest
expr = sign(x)          # Sign function (-1, 0, or 1)
expr = abs_expr(x)      # Absolute value (named abs_expr to avoid Python builtin conflict)
```

### Special Functions

```python
from mathhook import gamma, factorial, digamma, zeta, erf, erfc, symbols

x = symbols('x')[0]

expr = gamma(x)         # Gamma function
expr = factorial(5)     # 120
expr = digamma(x)       # Digamma function (ψ)
expr = zeta(x)          # Riemann zeta function
expr = erf(x)           # Error function
expr = erfc(x)          # Complementary error function
```

### Advanced Special Functions

```python
from mathhook import polygamma, bessel_j, bessel_y, beta, symbols

x, n = symbols('x n')

expr = polygamma(n, x)  # Polygamma function
expr = bessel_j(n, x)   # Bessel function of first kind
expr = bessel_y(n, x)   # Bessel function of second kind
expr = beta(x, n)       # Beta function
```

### Number Theory Functions

```python
from mathhook import gcd, lcm, modulo, isprime

result = gcd(12, 18)    # 6
result = lcm(4, 6)      # 12
result = modulo(17, 5)  # 2
result = isprime(17)    # True (returns PyExpression)
```

## Equation Solving

### Using PyMathSolver

```python
from mathhook import PyMathSolver, PyExpression, parse, symbols

x = symbols('x')[0]

# Create solver
solver = PyMathSolver()

# Create equation (as Expression)
equation = PyExpression.equation(
    parse('x^2 - 4'),
    PyExpression.integer(0)
)

# Solve
result = solver.solve(equation, 'x')

# Check result
if result.has_solutions():
    print(f"Found {result.count()} solutions")
```

## ODE Solving

### Using PyODESolver

```python
from mathhook import PyODESolver, PyExpression

solver = PyODESolver()

# Euler method
# dy/dx = f(x, y), y(x0) = y0
result = solver.euler(
    f=PyExpression.symbol('y'),  # dy/dx = y
    x0=0.0,
    y0=1.0,
    h=0.01,     # Step size
    steps=100
)

# Runge-Kutta 4th order
result = solver.runge_kutta_4(
    f=PyExpression.symbol('y'),
    x0=0.0,
    y0=1.0,
    h=0.01,
    steps=100
)

# Adaptive Runge-Kutta-Fehlberg (RK45)
result = solver.runge_kutta_45(
    f=PyExpression.symbol('y'),
    x0=0.0,
    y0=1.0,
    x_end=1.0,
    tol=1e-6
)
```

## PDE Solving

### Using PyPDESolver

```python
from mathhook import PyPDESolver, PyExpression

solver = PyPDESolver()

# Heat equation: u_t = α * u_xx
result = solver.solve_heat_equation(
    initial_condition=PyExpression.symbol('x'),
    boundary_left=PyExpression.integer(0),
    boundary_right=PyExpression.integer(0),
    alpha=1.0,
    x_min=0.0,
    x_max=1.0,
    t_max=0.1,
    nx=50,
    nt=100
)

# Wave equation: u_tt = c^2 * u_xx
result = solver.solve_wave_equation(
    initial_position=PyExpression.symbol('x'),
    initial_velocity=PyExpression.integer(0),
    boundary_left=PyExpression.integer(0),
    boundary_right=PyExpression.integer(0),
    c=1.0,
    x_min=0.0,
    x_max=1.0,
    t_max=1.0,
    nx=50,
    nt=100
)

# Laplace equation
result = solver.solve_laplace_equation(['x', 'y'])
```

## Gröbner Basis

```python
from mathhook import PyGroebnerBasis, parse

# Create basis from polynomials
polys = [parse('x^2 + y - 1'), parse('x + y^2 - 1')]
basis = PyGroebnerBasis(polys, ['x', 'y'], 'grevlex')

# Compute the basis
basis.compute()

# Get result
result = basis.get_basis()

# Check membership
poly = parse('x + y')
is_member = basis.contains(poly)

# Reduce to minimal basis
basis.reduce()
is_reduced = basis.is_reduced()
```

## Printing and Display

```python
from mathhook import init_printing, pprint, symbols

# Configure printing
init_printing(use_latex=True, latex_mode='mathjax', unicode=True)

x, y = symbols('x y')
expr = x.pow(2).add(y)

# Pretty print to stdout
pprint(expr)                    # With Unicode
pprint(expr, use_unicode=False) # Without Unicode

# Jupyter notebooks: expressions auto-render as LaTeX via _repr_latex_()
```

## EvalContext

Control evaluation behavior with `EvalContext`:

```python
from mathhook import EvalContext, PyExpression

# Numeric context - substitutes variables and evaluates
ctx = EvalContext.numeric({
    'x': PyExpression.integer(3),
    'y': PyExpression.float(2.5)
})

# Symbolic context - no numerical evaluation
ctx = EvalContext.symbolic()

# Use with evaluate_with_context
expr = parse('x^2 + y')
result = expr.evaluate_with_context(ctx)
```

## Fast Polynomial Arithmetic (PolyZp)

For high-performance polynomial operations over finite fields:

```python
from mathhook import poly_zp, poly_gcd, poly_mul_fast, PyPolyZp

# Create polynomial over Z_p
coeffs = [1, 2, 1]  # 1 + 2x + x^2
modulus = 17
poly = poly_zp(coeffs, modulus)

# GCD of polynomials
gcd = poly_gcd(poly1, poly2)

# Fast multiplication
product = poly_mul_fast(poly1, poly2)
```

## Complete API Reference

### Classes

| Class | Purpose |
|-------|---------|
| `PyExpression` | Core symbolic expression - all math operations |
| `EvalContext` | Controlled evaluation context |
| `PyMathSolver` | Algebraic equation solver |
| `PySolverResult` | Solution container |
| `PyODESolver` | Numerical ODE methods (Euler, RK4, RKF45) |
| `PyPDESolver` | PDE solvers (heat, wave, Laplace) |
| `PyGroebnerBasis` | Gröbner basis computation |
| `PyPattern` | Pattern matching for expressions |
| `PyStep` | Single step in explanation |
| `PyStepByStepExplanation` | Full step-by-step explanation |
| `PyPolyZp` | Polynomial over finite field |

### Functions

| Function | Purpose |
|----------|---------|
| `symbols('x y z')` | Create multiple symbols |
| `parse('x^2')` | Parse expression from string |
| `init_printing()` | Configure display settings |
| `pprint(expr)` | Pretty print expression |
| `sin`, `cos`, `tan` | Trigonometric functions |
| `asin`, `acos`, `atan` | Inverse trigonometric |
| `sinh`, `cosh`, `tanh` | Hyperbolic functions |
| `exp`, `log`, `ln`, `sqrt` | Exponential/logarithmic |
| `floor`, `ceil`, `round`, `sign`, `abs_expr` | Rounding/sign |
| `gamma`, `factorial`, `digamma`, `zeta` | Special functions |
| `erf`, `erfc` | Error functions |
| `polygamma`, `bessel_j`, `bessel_y`, `beta` | Advanced special |
| `gcd`, `lcm`, `modulo`, `isprime` | Number theory |
| `degree`, `roots` | Polynomial functions |
| `poly_zp`, `poly_gcd`, `poly_mul_fast` | Fast polynomial ops |

## Performance Tips

### 1. Reuse Symbols

```python
# GOOD: Create symbols once
x, y = symbols('x y')
for i in range(1000):
    expr = x.pow(2).add(y.multiply(i))

# BAD: Create symbols repeatedly
for i in range(1000):
    x, y = symbols('x y')  # Unnecessary overhead
    expr = x.pow(2).add(y.multiply(i))
```

### 2. Use parse() for Complex Expressions

```python
# GOOD: Parse complex expressions
expr = parse('x^3 + 3*x^2 + 3*x + 1')

# VERBOSE: Build manually
x = symbols('x')[0]
expr = x.pow(3).add(x.pow(2).multiply(3)).add(x.multiply(3)).add(1)
```

### 3. Simplify Strategically

```python
# GOOD: Simplify once at end
expr = build_complex_expression()
# ... many operations ...
final = expr.simplify()

# BAD: Over-simplification
expr = x.add(1)
expr = expr.simplify()  # Unnecessary
expr = expr.multiply(2)
expr = expr.simplify()  # Unnecessary
```

## Links

- **PyPI**: https://pypi.org/project/mathhook/
- **GitHub**: https://github.com/AhmedMashour/mathhook
- **Documentation**: https://mathhook.readthedocs.io
