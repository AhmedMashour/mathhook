#!/usr/bin/env python3
"""
MathHook Python Basic Usage Examples

This example demonstrates the fundamental operations available
in the MathHook Python bindings.

Last Updated: 2025-12-28T1200
"""

from mathhook import (
    symbol, symbols, parse,
    sin, cos, tan, exp, log, sqrt, abs_expr,
    pprint, init_printing
)

def main():
    print("=" * 60)
    print("MathHook Python - Basic Usage Examples")
    print("=" * 60)

    # =========================================================================
    # Symbol Creation
    # =========================================================================
    print("\n1. Symbol Creation")
    print("-" * 40)

    # Single symbol
    x = symbol('x')
    print(f"Single symbol: x = {x}")

    # Multiple symbols
    a, b, c = symbols('a b c')
    print(f"Multiple symbols: a={a}, b={b}, c={c}")

    # =========================================================================
    # Expression Creation with Operators
    # =========================================================================
    print("\n2. Expression Creation (Operator Overloading)")
    print("-" * 40)

    # Arithmetic operations
    expr1 = x + 2
    print(f"x + 2 = {expr1}")

    expr2 = x - 3
    print(f"x - 3 = {expr2}")

    expr3 = x * 5
    print(f"x * 5 = {expr3}")

    expr4 = x / 2
    print(f"x / 2 = {expr4}")

    expr5 = x ** 2
    print(f"x ** 2 = {expr5}")

    # Complex expression: quadratic
    quadratic = a * x**2 + b * x + c
    print(f"Quadratic: a*x² + b*x + c = {quadratic}")

    # =========================================================================
    # Mathematical Functions
    # =========================================================================
    print("\n3. Mathematical Functions")
    print("-" * 40)

    print(f"sin(x) = {sin(x)}")
    print(f"cos(x) = {cos(x)}")
    print(f"tan(x) = {tan(x)}")
    print(f"exp(x) = {exp(x)}")
    print(f"log(x) = {log(x)}")
    print(f"sqrt(x) = {sqrt(x)}")
    print(f"abs(x) = {abs_expr(x)}")

    # =========================================================================
    # Simplification
    # =========================================================================
    print("\n4. Simplification")
    print("-" * 40)

    # Combine like terms
    expr = x + x + x
    print(f"x + x + x = {expr.simplify()}")

    # Arithmetic simplification
    expr = parse("2 + 3 + 5")
    print(f"2 + 3 + 5 = {expr.simplify()}")

    # Polynomial simplification
    expr = x**2 + 2*x + 1 + x**2 - 1
    print(f"x² + 2x + 1 + x² - 1 = {expr.simplify()}")

    # =========================================================================
    # Parsing Expressions
    # =========================================================================
    print("\n5. Parsing Expressions")
    print("-" * 40)

    expr1 = parse("x^2 + 2*x + 1")
    print(f"Parsed 'x^2 + 2*x + 1': {expr1}")

    expr2 = parse("sin(x) + cos(y)")
    print(f"Parsed 'sin(x) + cos(y)': {expr2}")

    expr3 = parse("2x + 3y")  # Implicit multiplication
    print(f"Parsed '2x + 3y': {expr3}")

    # =========================================================================
    # Expansion and Factoring
    # =========================================================================
    print("\n6. Expansion and Factoring")
    print("-" * 40)

    # Expand (x + 1)^2
    expr = (x + 1) ** 2
    expanded = expr.expand()
    print(f"(x + 1)² expanded = {expanded}")

    # Factor x^2 - 1
    expr = x**2 - 1
    factored = expr.factor()
    print(f"x² - 1 factored = {factored}")

    # =========================================================================
    # Output Formats
    # =========================================================================
    print("\n7. Output Formats")
    print("-" * 40)

    expr = x**2 + 2*x + 1

    # String representation
    print(f"String: {expr}")

    # LaTeX format
    latex = expr.to_latex()
    print(f"LaTeX: {latex}")

    # Pretty printing
    print("Pretty print:")
    pprint(expr)

    # =========================================================================
    # Summary
    # =========================================================================
    print("\n" + "=" * 60)
    print("Basic usage examples completed!")
    print("=" * 60)


if __name__ == "__main__":
    main()
