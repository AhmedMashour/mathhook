#!/usr/bin/env python3
"""
MathHook Python Advanced Usage Examples

This example demonstrates advanced mathematical operations,
complex expressions, and real-world use cases for the
MathHook Python bindings.

Last Updated: 2025-12-28T1200
"""

import time
from mathhook import (
    symbol, symbols, parse, solve,
    sin, cos, tan, exp, log, sqrt, abs_expr,
    factorial, gamma,
    pprint, init_printing,
    gcd, lcm,
    PyODESolver, PyExpression
)

def main():
    print("=" * 70)
    print("MathHook Python - Advanced Usage Examples")
    print("=" * 70)

    # =========================================================================
    # Complex Mathematical Expressions
    # =========================================================================
    print("\n1. Complex Mathematical Expressions")
    print("-" * 50)

    x, y, z = symbols('x y z')
    a, b, c = symbols('a b c')

    # Multi-variable polynomial: ax² + bxy + cy²
    multi_poly = a * x**2 + b * x * y + c * y**2
    print(f"Multi-variable polynomial: {multi_poly}")
    print(f"Simplified: {multi_poly.simplify()}")

    # Nested operations: (x + y)³
    cubed = (x + y) ** 3
    print(f"\n(x + y)³ = {cubed}")
    print(f"Expanded: {cubed.expand()}")

    # Complex fraction: (x² + 2x + 1) / (x + 1)
    numerator = x**2 + 2*x + 1
    denominator = x + 1
    fraction = numerator / denominator
    print(f"\n(x² + 2x + 1) / (x + 1) = {fraction}")
    print(f"Simplified: {fraction.simplify()}")

    # =========================================================================
    # Calculus Operations
    # =========================================================================
    print("\n2. Calculus Operations")
    print("-" * 50)

    # Derivative
    expr = x**3 + 2*x**2 + x + 1
    print(f"f(x) = {expr}")
    derivative = expr.derivative('x')
    print(f"f'(x) = {derivative}")

    # Second derivative
    second_derivative = expr.nth_derivative('x', 2)
    print(f"f''(x) = {second_derivative}")

    # Integration
    integral = expr.integrate('x')
    print(f"∫f(x)dx = {integral}")

    # Trigonometric calculus
    trig_expr = sin(x) * cos(x)
    print(f"\ng(x) = sin(x)cos(x)")
    print(f"g'(x) = {trig_expr.derivative('x')}")

    # Chain rule: d/dx[sin(x²)]
    chain_expr = sin(x**2)
    print(f"\nh(x) = sin(x²)")
    print(f"h'(x) = {chain_expr.derivative('x')}")

    # Product rule: d/dx[x²sin(x)]
    product_expr = x**2 * sin(x)
    print(f"\np(x) = x²sin(x)")
    print(f"p'(x) = {product_expr.derivative('x')}")

    # =========================================================================
    # Equation Solving
    # =========================================================================
    print("\n3. Equation Solving")
    print("-" * 50)

    # Linear equation: 2x + 3 = 7
    solutions = solve(2*x + 3 - 7, 'x')
    print(f"2x + 3 = 7 → x = {solutions}")

    # Quadratic equation: x² - 5x + 6 = 0
    solutions = solve(x**2 - 5*x + 6, 'x')
    print(f"x² - 5x + 6 = 0 → x = {solutions}")

    # Quadratic with complex roots: x² + 1 = 0
    solutions = solve(x**2 + 1, 'x')
    print(f"x² + 1 = 0 → x = {solutions}")

    # Cubic equation: x³ - 6x² + 11x - 6 = 0
    solutions = solve(x**3 - 6*x**2 + 11*x - 6, 'x')
    print(f"x³ - 6x² + 11x - 6 = 0 → x = {solutions}")

    # =========================================================================
    # Substitution
    # =========================================================================
    print("\n4. Substitution")
    print("-" * 50)

    expr = x**2 + 2*x + 1
    print(f"f(x) = {expr}")

    # Substitute x = 3
    result = expr.substitute({'x': PyExpression.integer(3)})
    print(f"f(3) = {result}")

    # Substitute x = y + 1
    result = expr.substitute({'x': y + 1})
    print(f"f(y + 1) = {result}")
    print(f"f(y + 1) expanded = {result.expand()}")

    # =========================================================================
    # Limits
    # =========================================================================
    print("\n5. Limits")
    print("-" * 50)

    # Limit of sin(x)/x as x → 0
    expr = sin(x) / x
    limit_result = expr.limit('x', PyExpression.integer(0))
    print(f"lim(x→0) sin(x)/x = {limit_result}")

    # Limit of (x² - 1)/(x - 1) as x → 1
    expr = (x**2 - 1) / (x - 1)
    limit_result = expr.limit('x', PyExpression.integer(1))
    print(f"lim(x→1) (x² - 1)/(x - 1) = {limit_result}")

    # =========================================================================
    # Polynomial Operations
    # =========================================================================
    print("\n6. Polynomial Operations")
    print("-" * 50)

    p1 = x**2 - 1
    p2 = x - 1

    # GCD
    result = gcd(p1, p2)
    print(f"gcd(x² - 1, x - 1) = {result}")

    # Multiplication
    product = p1 * p2
    print(f"(x² - 1)(x - 1) = {product.expand()}")

    # Division
    quotient = p1 / p2
    print(f"(x² - 1)/(x - 1) = {quotient.simplify()}")

    # =========================================================================
    # Trigonometric Identities
    # =========================================================================
    print("\n7. Trigonometric Identities")
    print("-" * 50)

    # Pythagorean identity: sin²(x) + cos²(x) = 1
    identity = sin(x)**2 + cos(x)**2
    print(f"sin²(x) + cos²(x) = {identity.simplify()}")

    # Double angle: 2sin(x)cos(x) = sin(2x)
    double_angle = 2 * sin(x) * cos(x)
    print(f"2sin(x)cos(x) = {double_angle.simplify()}")

    # =========================================================================
    # Logarithmic Operations
    # =========================================================================
    print("\n8. Logarithmic Operations")
    print("-" * 50)

    # Log product rule: log(xy) = log(x) + log(y)
    log_sum = log(x) + log(y)
    print(f"log(x) + log(y) = {log_sum.simplify()}")

    # Log power rule: log(x^n) = n*log(x)
    log_power = log(x**3)
    print(f"log(x³) = {log_power.simplify()}")

    # =========================================================================
    # Special Functions
    # =========================================================================
    print("\n9. Special Functions")
    print("-" * 50)

    n = symbol('n')

    print(f"factorial(5) = {factorial(5)}")
    print(f"gamma(5) = {gamma(5)}")
    print(f"exp(log(x)) = {exp(log(x)).simplify()}")

    # =========================================================================
    # ODE Solving (Numerical)
    # =========================================================================
    print("\n10. ODE Solving (Numerical)")
    print("-" * 50)

    # Solve dy/dx = -2y with y(0) = 1
    t = symbol('t')
    y_var = symbol('y')

    # Using PyODESolver for numerical ODE solving
    ode_solver = PyODESolver()

    print("ODE: dy/dt = -2y, y(0) = 1")
    print("Analytical solution: y(t) = e^(-2t)")

    # The numerical solver would be used for more complex ODEs
    # For simple exponential decay, we can verify:
    analytical = exp(-2 * t)
    result = analytical.substitute({'t': PyExpression.integer(1)})
    print(f"At t=1: y(1) = {result}")

    # =========================================================================
    # Performance Demo
    # =========================================================================
    print("\n11. Performance Demo")
    print("-" * 50)

    start = time.time()
    expressions = []
    for i in range(1000):
        expr = x * i + y**2 + i * 2
        expressions.append(expr.simplify())
    elapsed = (time.time() - start) * 1000

    print(f"Created and simplified 1000 expressions in {elapsed:.2f}ms")
    print(f"Average: {elapsed/1000:.4f}ms per expression")

    # =========================================================================
    # Summary
    # =========================================================================
    print("\n" + "=" * 70)
    print("Advanced usage examples completed!")
    print("=" * 70)


if __name__ == "__main__":
    main()
