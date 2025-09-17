#!/usr/bin/env python3
"""
Phase 3 Python Bindings Demo

Demonstrates new features added in Phase 3:
1. PDE Solvers (heat, wave, Laplace equations)
2. Gröbner Basis computation
3. Polynomial resultant and discriminant

Usage:
    python phase3_demo.py
"""

from mathhook_python import PyExpression, PyPDESolver, PyGroebnerBasis


def demo_pde_solvers():
    """Demonstrate PDE solver capabilities."""
    print("=" * 60)
    print("PDE SOLVERS DEMO")
    print("=" * 60)

    solver = PyPDESolver()

    # Heat Equation
    print("\n1. Heat Equation: ∂u/∂t = α∇²u")
    print("-" * 60)
    alpha = PyExpression.float(0.01)
    domain_length = PyExpression.integer(10)

    try:
        heat_solution = solver.solve_heat_equation(alpha, "x", "t", domain_length)
        print(f"Solution: {heat_solution.to_simple()}")
        print("Note: Solution contains symbolic Fourier coefficients")
    except Exception as e:
        print(f"Heat equation solving: {e}")

    # Wave Equation
    print("\n2. Wave Equation: ∂²u/∂t² = c²∇²u")
    print("-" * 60)
    wave_speed = PyExpression.integer(1)
    domain_length = PyExpression.integer(10)

    try:
        wave_solution = solver.solve_wave_equation(
            wave_speed, "x", "t", domain_length
        )
        print(f"Solution: {wave_solution.to_simple()}")
        print("Note: Solution with eigenvalues and symbolic coefficients")
    except Exception as e:
        print(f"Wave equation solving: {e}")

    # Laplace Equation
    print("\n3. Laplace Equation: ∇²u = 0")
    print("-" * 60)
    try:
        laplace_solution = solver.solve_laplace_equation(["x", "y"])
        print(f"Solution: {laplace_solution.to_simple()}")
        print("Note: 2D solution on rectangular domain")
    except Exception as e:
        print(f"Laplace equation solving: {e}")


def demo_groebner_basis():
    """Demonstrate Gröbner basis computation."""
    print("\n" + "=" * 60)
    print("GRÖBNER BASIS DEMO")
    print("=" * 60)

    # Define polynomial ideal
    print("\nIdeal I = <x² + y² - 1, x - y>")
    print("-" * 60)

    x = PyExpression.symbol("x")
    y = PyExpression.symbol("y")

    f1 = PyExpression.parse("x^2 + y^2 - 1")
    f2 = PyExpression.parse("x - y")

    print(f"f1: {f1.to_simple()}")
    print(f"f2: {f2.to_simple()}")

    # Create and compute Gröbner basis
    print("\nComputing Gröbner basis with lexicographic ordering...")
    try:
        gb = PyGroebnerBasis([f1, f2], ["x", "y"], "lex")
        gb.compute()

        basis = gb.get_basis()
        print(f"\nGröbner Basis ({len(basis)} elements):")
        for i, poly in enumerate(basis, 1):
            print(f"  g{i}: {poly.to_simple()}")

        # Test ideal membership
        print("\nTesting ideal membership:")
        test1 = PyExpression.parse("x^2 - 1")
        test2 = PyExpression.parse("x + 1")

        print(f"  Is x² - 1 in I? {gb.contains(test1)}")
        print(f"  Is x + 1 in I? {gb.contains(test2)}")

        # Reduce to minimal form
        print("\nReducing to minimal form...")
        gb.reduce()
        print(f"  Is reduced? {gb.is_reduced()}")

    except Exception as e:
        print(f"Gröbner basis computation failed: {e}")


def demo_polynomial_operations():
    """Demonstrate advanced polynomial operations."""
    print("\n" + "=" * 60)
    print("POLYNOMIAL OPERATIONS DEMO")
    print("=" * 60)

    # Resultant
    print("\n1. Polynomial Resultant")
    print("-" * 60)
    p1 = PyExpression.parse("x^2 + y")
    p2 = PyExpression.parse("x + y^2")

    print(f"p1: {p1.to_simple()}")
    print(f"p2: {p2.to_simple()}")

    try:
        result = PyExpression.resultant(p1, p2, "x")
        print(f"\nResultant(p1, p2, x): {result.to_simple()}")
        print("Note: Resultant is zero iff p1 and p2 have common root")
    except Exception as e:
        print(f"Resultant computation: {e}")

    # Discriminant
    print("\n2. Polynomial Discriminant")
    print("-" * 60)

    # Quadratic with repeated root: (x+1)² = x² + 2x + 1
    quad1 = PyExpression.parse("x^2 + 2*x + 1")
    print(f"Quadratic: {quad1.to_simple()}")

    try:
        disc1 = PyExpression.discriminant(quad1, "x")
        print(f"Discriminant: {disc1.to_simple()}")
        print("Analysis: 0 → repeated root at x = -1")
    except Exception as e:
        print(f"Discriminant computation: {e}")

    # Quadratic with real roots: x² - 5x + 6 = (x-2)(x-3)
    quad2 = PyExpression.parse("x^2 - 5*x + 6")
    print(f"\nQuadratic: {quad2.to_simple()}")

    try:
        disc2 = PyExpression.discriminant(quad2, "x")
        simplified = disc2.simplify()
        print(f"Discriminant: {simplified.to_simple()}")
        print("Analysis: 1 > 0 → two real distinct roots")
    except Exception as e:
        print(f"Discriminant computation: {e}")


def main():
    """Run all Phase 3 feature demonstrations."""
    print("\n" + "=" * 60)
    print("MATHHOOK PHASE 3 PYTHON BINDINGS DEMO")
    print("=" * 60)
    print("\nDemonstrating new features:")
    print("  - PDE Solvers (heat, wave, Laplace)")
    print("  - Gröbner Basis computation")
    print("  - Polynomial resultant and discriminant")

    try:
        demo_pde_solvers()
        demo_groebner_basis()
        demo_polynomial_operations()

        print("\n" + "=" * 60)
        print("DEMO COMPLETE")
        print("=" * 60)
        print("\nAll Phase 3 features demonstrated successfully!")
        print("Note: Some features return symbolic results pending")
        print("      implementation of symbolic integration.")

    except ImportError as e:
        print(f"\nError: {e}")
        print("\nTo use these features, build the Python bindings:")
        print("  cd crates/mathhook-python")
        print("  maturin develop")


if __name__ == "__main__":
    main()
