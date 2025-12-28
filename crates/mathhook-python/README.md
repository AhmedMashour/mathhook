# MathHook Python Bindings

[![PyPI](https://img.shields.io/pypi/v/mathhook.svg)](https://pypi.org/project/mathhook/)
[![Python Versions](https://img.shields.io/pypi/pyversions/mathhook.svg)](https://pypi.org/project/mathhook/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](../../LICENSE)

High-performance computer algebra system for Python, powered by Rust.

## Features

- **High Performance**: Rust-powered core targeting 10-100x speedup over SymPy
- **SymPy-like API**: Familiar interface for Python users
- **Symbolic Mathematics**: Expressions, algebra, calculus, and matrix operations
- **Multiple Input/Output Formats**: Parse and emit LaTeX, Wolfram Language, and standard notation
- **Educational**: Step-by-step explanations for simplification and derivatives
- **Memory Efficient**: Rust-powered core with minimal Python overhead

## Installation

```bash
pip install mathhook
```

Requires Python 3.8 or higher.

### From Source

```bash
# Install maturin
pip install maturin

# Clone and build
git clone https://github.com/AhmedMashour/mathhook.git
cd mathhook/crates/mathhook-python
maturin develop --release
```

## Quick Start

```python
from mathhook import symbol, symbols, parse, solve, sin, cos, exp

# Create symbols
x = symbol('x')
y = symbol('y')

# Build expressions with Python operators
expr = x**2 + 2*x + 1

# Simplify
simplified = expr.simplify()
print(simplified)  # x^2 + 2*x + 1

# Solve equations
solutions = solve(x**2 - 4, x)
print(solutions)  # Solutions: x = 2 and x = -2

# Calculus
derivative = expr.diff(x)
print(derivative)  # 2*x + 2

integral = expr.integrate(x)
print(integral)  # x^3/3 + x^2 + x
```

## Expression Creation

### Basic Expressions

```python
from mathhook import Expression, symbol, symbols

# Integers
num = Expression.integer(42)

# Floats
pi_approx = Expression.float(3.14159)

# Rationals (exact fractions)
half = Expression.rational(1, 2)

# Single symbol
x = symbol('x')

# Multiple symbols
a, b, c = symbols('a b c')
```

### Operator Overloading

Python bindings support natural mathematical notation:

```python
from mathhook import symbol

x = symbol('x')
y = symbol('y')

# All standard operators work
expr = (x + y) * (x - y)  # Difference of squares
expr = x**2 + 2*x + 1      # Polynomial
expr = (x + 1) / (x - 1)   # Rational function
```

### Functions

```python
from mathhook import symbol, sin, cos, tan, exp, log, sqrt, ln

x = symbol('x')

# Trigonometric functions
trig = sin(x)**2 + cos(x)**2  # = 1

# Exponential and logarithmic
exponential = exp(x)
natural_log = ln(x)
log_base_10 = log(x, 10)

# Square root
root = sqrt(x**2 + 1)
```

### Constants

```python
from mathhook import Expression, symbol

r = symbol('r')

# Mathematical constants
pi = Expression.pi()
e = Expression.e()
i = Expression.i()  # imaginary unit

# Use in expressions
circle_area = pi * r**2
```

## Algebraic Operations

### Simplification

```python
from mathhook import symbol, sin, cos

x = symbol('x')

# Algebraic simplification
expr = x + x + x
print(expr.simplify())  # 3*x

# Trigonometric identities
expr = sin(x)**2 + cos(x)**2
print(expr.simplify())  # 1

# Rational expressions
expr = (x**2 - 1) / (x - 1)
print(expr.simplify())  # x + 1
```

### Expansion

```python
from mathhook import symbol

x = symbol('x')
y = symbol('y')

# Expand products
expr = (x + 1) * (x + 2)
print(expr.expand())  # x^2 + 3*x + 2

# Expand powers
expr = (x + y)**2
print(expr.expand())  # x^2 + 2*x*y + y^2
```

### Factorization

```python
from mathhook import symbol

x = symbol('x')

# Factor polynomials
expr = x**2 - 1
print(expr.factor())  # (x + 1)*(x - 1)

expr = x**2 + 5*x + 6
print(expr.factor())  # (x + 2)*(x + 3)
```

### Substitution

```python
from mathhook import symbol, Expression

x = symbol('x')
y = symbol('y')

expr = x**2 + 2*x + 1

# Substitute value
result = expr.subs(x, Expression.integer(3))
print(result)  # 16

# Substitute expression
result = expr.subs(x, y + 1)
print(result)  # (y + 1)^2 + 2*(y + 1) + 1
```

## Calculus

### Derivatives

```python
from mathhook import symbol, sin, cos, exp

x = symbol('x')

# First derivative
expr = x**3
print(expr.diff(x))  # 3*x^2

# Higher-order derivatives
print(expr.diff(x, 2))  # 6*x
print(expr.diff(x, 3))  # 6

# Chain rule
expr = sin(x**2)
print(expr.diff(x))  # 2*x*cos(x^2)

# Product rule
expr = x * exp(x)
print(expr.diff(x))  # x*exp(x) + exp(x)
```

### Integrals

```python
from mathhook import symbol, sin, cos

x = symbol('x')

# Indefinite integrals
expr = x**2
print(expr.integrate(x))  # x^3/3

# Definite integrals
expr = x**2
result = expr.integrate((x, 0, 1))
print(result)  # 1/3

# Trigonometric integrals
expr = sin(x)
print(expr.integrate(x))  # -cos(x)
```

### Limits

```python
from mathhook import symbol, sin, Expression

x = symbol('x')

# Finite limits
expr = sin(x) / x
limit = expr.limit(x, Expression.integer(0))
print(limit)  # 1

# Infinite limits
expr = 1 / x
print(expr.limit(x, Expression.infinity()))  # 0
```

### Series Expansions

```python
from mathhook import symbol, sin, cos, exp, Expression

x = symbol('x')

# Taylor series around x=0
series = sin(x).series(x, Expression.integer(0), 6)
print(series)  # x - x^3/6 + x^5/120 + O(x^6)

# Around different points
series = exp(x).series(x, Expression.integer(1), 4)
print(series)  # e + e*(x-1) + e*(x-1)^2/2 + ...
```

## Equation Solving

### Algebraic Equations

```python
from mathhook import symbol, solve

x = symbol('x')

# Linear equations
solutions = solve(2*x + 3 - 7, x)
print(solutions)  # [2]

# Quadratic equations
solutions = solve(x**2 - 5*x + 6, x)
print(solutions)  # [2, 3]

# With complex roots
solutions = solve(x**2 + 1, x)
print(solutions)  # [i, -i]
```

### Systems of Equations

```python
from mathhook import symbol, symbols, solve

x, y = symbols('x y')

# Solve system
solutions = solve([
    x + y - 5,
    x - y - 1
], [x, y])
print(solutions)  # {x: 3, y: 2}
```

## Parsing

### Multi-Format Parser

```python
from mathhook import parse

# Standard notation
expr = parse("2*x + sin(y)")

# LaTeX (auto-detected)
expr = parse(r"\frac{x}{2} + y^2")
expr = parse(r"\sin(x) + \cos(y)")

# Wolfram Language (auto-detected)
expr = parse("Sin[x] + Cos[y]")

# Implicit multiplication
expr = parse("2x + 3y")

# Complex expressions
expr = parse("sin(2*pi*x) + exp(-x^2/2)")
```

### Format Conversion

```python
from mathhook import symbol

x = symbol('x')
expr = x**2 / 2

# Convert to LaTeX
latex = expr.to_latex()
print(latex)  # \frac{x^{2}}{2}

# Convert to Wolfram
wolfram = expr.to_wolfram()
print(wolfram)  # Divide[Power[x, 2], 2]

# String representation
print(str(expr))  # x^2/2
```

## Special Functions

```python
from mathhook import (
    symbol, Expression,
    # Trigonometric
    sin, cos, tan, asin, acos, atan,
    # Hyperbolic
    sinh, cosh, tanh,
    # Exponential/Logarithmic
    exp, ln, log,
    # Elementary
    sqrt, abs_expr as abs, sign, floor, ceil, round,
    # Special
    gamma, factorial, digamma, zeta, erf, erfc,
    # Bessel
    bessel_j, bessel_y,
    # Number theory
    gcd, lcm, isprime,
    # Polynomial
    degree, roots
)

x = symbol('x')

# Trigonometric
trig = sin(x)

# Hyperbolic
hyp = sinh(x)

# Special functions
g = gamma(Expression.integer(5))  # 24 (4!)
f = factorial(5)                   # 120
e = erf(x)                         # Error function
j = bessel_j(0, x)                 # Bessel function of first kind
z = zeta(Expression.integer(2))   # Riemann zeta: π²/6

# Number theory
g2 = gcd(12, 18)   # 6
l = lcm(4, 6)      # 12
p = isprime(17)    # True

# Rounding functions
s = sign(Expression.integer(-5))   # -1
fl = floor(Expression.float(3.7))  # 3
cl = ceil(Expression.float(3.2))   # 4
```

## Pretty Printing

```python
from mathhook import symbol, init_printing, pprint

# Initialize pretty printing
init_printing()

x = symbol('x')
expr = x**2 + 2*x + 1

# Pretty print expression
pprint(expr)
```

## Real-World Examples

### Quadratic Formula

```python
from mathhook import symbol, symbols, sqrt, solve

x = symbol('x')
a, b, c = symbols('a b c')

# General quadratic equation
equation = a*x**2 + b*x + c

# Discriminant
discriminant = b**2 - 4*a*c

# Solutions
x1 = (-b + sqrt(discriminant)) / (2*a)
x2 = (-b - sqrt(discriminant)) / (2*a)

print(f"x₁ = {x1}")
print(f"x₂ = {x2}")

# Solve specific equation: x² - 5x + 6 = 0
solutions = solve(x**2 - 5*x + 6, x)
print(f"Solutions: {solutions}")  # [2, 3]
```

### Taylor Series Approximation

```python
from mathhook import symbol, sin, cos, Expression

x = symbol('x')

# Approximate sin(x) with Taylor series
sin_approx = sin(x).series(x, Expression.integer(0), 10)
print(sin_approx)

# Compare with exact value at x=0.1
print(f"sin(0.1) exact: {sin(Expression.float(0.1))}")
```

### Pythagorean Distance

```python
from mathhook import symbol, sqrt

x = symbol('x')
y = symbol('y')

# Distance = √(x² + y²)
distance = sqrt(x**2 + y**2)
print(f"Distance: {distance}")
```

## Type Hints

MathHook provides full type hints for excellent IDE support:

```python
from mathhook import Expression, solve
from typing import List, Union

def quadratic(a: Union[int, float], b: Union[int, float],
              c: Union[int, float], x: Expression) -> Expression:
    """Create a quadratic expression."""
    return a*x**2 + b*x + c

def find_roots(expr: Expression, var: Expression) -> List[Expression]:
    """Find roots of an expression."""
    return solve(expr, var)
```

## API Quick Reference

### Main Functions

| Function | Purpose |
|----------|---------|
| `symbol(name)` | Create a single symbol |
| `symbols(names)` | Create multiple symbols from space-separated string |
| `parse(str)` | Parse expression from string (auto-detects format) |
| `solve(expr, var)` | Solve equation for variable |
| `pprint(expr)` | Pretty print expression |
| `init_printing()` | Initialize pretty printing |

### Expression Methods

| Method | Description |
|--------|-------------|
| `.simplify()` | Simplify expression |
| `.expand()` | Expand products and powers |
| `.factor()` | Factor expression |
| `.diff(var)` | Differentiate with respect to variable |
| `.diff(var, n)` | Nth derivative |
| `.integrate(var)` | Indefinite integral |
| `.integrate((var, a, b))` | Definite integral from a to b |
| `.limit(var, val)` | Compute limit |
| `.series(var, point, n)` | Taylor series expansion |
| `.subs(var, val)` | Substitute value for variable |
| `.to_latex()` | Convert to LaTeX string |
| `.to_wolfram()` | Convert to Wolfram Language string |

### Expression Static Methods

| Method | Description |
|--------|-------------|
| `.integer(n)` | Create integer expression |
| `.float(x)` | Create float expression |
| `.rational(num, den)` | Create rational expression |
| `.pi()` | Pi constant |
| `.e()` | Euler's number |
| `.i()` | Imaginary unit |
| `.infinity()` | Infinity |

### Mathematical Functions

| Function | Description |
|----------|-------------|
| `sin`, `cos`, `tan` | Trigonometric |
| `asin`, `acos`, `atan` | Inverse trigonometric |
| `sinh`, `cosh`, `tanh` | Hyperbolic |
| `exp`, `ln`, `log` | Exponential/logarithmic |
| `sqrt`, `abs_expr` | Square root, absolute value |
| `gamma`, `factorial` | Special functions |
| `erf`, `erfc` | Error functions |
| `bessel_j`, `bessel_y` | Bessel functions |
| `zeta`, `digamma`, `polygamma` | Zeta and related |
| `gcd`, `lcm`, `isprime`, `modulo` | Number theory |
| `sign`, `floor`, `ceil`, `round` | Numeric |
| `degree`, `roots`, `beta` | Polynomial/special |

## Common Issues

### Import Errors

If you see `ImportError: No module named 'mathhook'`:
- Ensure mathhook is installed: `pip install mathhook`
- Check Python version: `python --version` (must be 3.8+)

### Building from Source

If you need to build from source:
```bash
pip install maturin
cd mathhook/crates/mathhook-python
maturin develop --release
```

### Performance Issues

For better performance:
- Use the Rust-native functions when possible
- Process expressions in bulk when possible
- Cache frequently used expressions

## Performance Comparison

Benchmark results vs SymPy (lower is better):

| Operation | MathHook | SymPy | Speedup |
|-----------|----------|-------|---------|
| Expression Creation | 0.1μs | 2.0μs | 20x |
| Simplification | 1.0μs | 50μs | 50x |
| Differentiation | 2.0μs | 100μs | 50x |
| Matrix Multiplication | 10μs | 500μs | 50x |

*Benchmarks on Apple M1, Python 3.11*

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

## License

MathHook is dual-licensed under MIT OR Apache-2.0. See [LICENSE](../../LICENSE) for details.

## Links

- **PyPI**: https://pypi.org/project/mathhook/
- **GitHub**: https://github.com/AhmedMashour/mathhook
- **Documentation**: https://mathhook.readthedocs.io
- **Issue Tracker**: https://github.com/AhmedMashour/mathhook/issues
