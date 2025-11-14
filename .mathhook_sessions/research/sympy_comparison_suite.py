#!/usr/bin/env python3
"""
SymPy Comparison Test Oracle Generator
Wave 0 Research Phase for MathHook Core Mathematical Features

Generates comprehensive test cases by running SymPy and saving expected outputs.
This creates a test oracle for validating MathHook's mathematical correctness.

Usage:
    python3 sympy_comparison_suite.py

Output:
    .research/test_oracle.json - Test cases with SymPy expected outputs
"""

import sys
import json
import traceback
from typing import Dict, List, Any, Tuple
from datetime import datetime

try:
    import sympy as sp
    from sympy import symbols, Function, Eq, dsolve, solve, Matrix, factor, series, integrate, diff
    from sympy import sin, cos, exp, log, sqrt, pi, E, I
    from sympy.matrices import QRdecomposition, LUdecomposition
    from sympy.polys import groebner
except ImportError:
    print("Error: SymPy not found. Please install:")
    print("  pip install sympy")
    sys.exit(1)


def generate_ode_first_order_tests() -> List[Dict[str, Any]]:
    """Generate first-order ODE test cases"""
    x, y, t, C1 = symbols('x y t C1')
    f = Function('f')

    test_cases = []

    # Separable ODEs
    separable_odes = [
        ("dy/dx = x", Eq(y.diff(x), x)),
        ("dy/dx = y", Eq(y.diff(x), y)),
        ("dy/dx = x*y", Eq(y.diff(x), x*y)),
        ("dy/dx = y/x", Eq(y.diff(x), y/x)),
        ("dy/dx = x^2 + 1", Eq(y.diff(x), x**2 + 1)),
        ("dy/dx = sin(x)", Eq(y.diff(x), sin(x))),
        ("dy/dx = e^x", Eq(y.diff(x), exp(x))),
        ("dy/dx = 1/(1+x^2)", Eq(y.diff(x), 1/(1+x**2))),
    ]

    for desc, ode in separable_odes:
        try:
            solution = dsolve(ode, y)
            test_cases.append({
                "type": "ode_first_order_separable",
                "description": desc,
                "input": {
                    "ode": str(ode),
                    "dependent_var": "y",
                    "independent_var": "x"
                },
                "expected_output": str(solution),
                "sympy_version": sp.__version__,
                "difficulty": "simple"
            })
        except Exception as e:
            print(f"Warning: Failed to solve {desc}: {e}")

    # Linear first-order ODEs
    linear_first_odes = [
        ("dy/dx + y = 0", Eq(y.diff(x) + y, 0)),
        ("dy/dx + 2*y = 0", Eq(y.diff(x) + 2*y, 0)),
        ("dy/dx - y = x", Eq(y.diff(x) - y, x)),
        ("dy/dx + y/x = 1", Eq(y.diff(x) + y/x, 1)),
        ("dy/dx + 2*x*y = x", Eq(y.diff(x) + 2*x*y, x)),
    ]

    for desc, ode in linear_first_odes:
        try:
            solution = dsolve(ode, y)
            test_cases.append({
                "type": "ode_first_order_linear",
                "description": desc,
                "input": {
                    "ode": str(ode),
                    "dependent_var": "y",
                    "independent_var": "x"
                },
                "expected_output": str(solution),
                "sympy_version": sp.__version__,
                "difficulty": "medium"
            })
        except Exception as e:
            print(f"Warning: Failed to solve {desc}: {e}")

    # Edge cases
    edge_cases = [
        ("dy/dx = 0", Eq(y.diff(x), 0), "trivial_constant"),
        ("dy/dx = 1", Eq(y.diff(x), 1), "trivial_linear"),
        ("dy/dx = y^2", Eq(y.diff(x), y**2), "bernoulli_n2"),
    ]

    for desc, ode, case_type in edge_cases:
        try:
            solution = dsolve(ode, y)
            test_cases.append({
                "type": f"ode_first_order_edge_{case_type}",
                "description": desc,
                "input": {
                    "ode": str(ode),
                    "dependent_var": "y",
                    "independent_var": "x"
                },
                "expected_output": str(solution),
                "sympy_version": sp.__version__,
                "difficulty": "edge_case"
            })
        except Exception as e:
            print(f"Warning: Failed to solve edge case {desc}: {e}")

    return test_cases


def generate_ode_second_order_tests() -> List[Dict[str, Any]]:
    """Generate second-order ODE test cases"""
    x, y, t, C1, C2 = symbols('x y t C1 C2')

    test_cases = []

    # Constant coefficients - homogeneous
    const_coeff_homogeneous = [
        ("y'' + y = 0", Eq(y.diff(x, 2) + y, 0)),
        ("y'' - y = 0", Eq(y.diff(x, 2) - y, 0)),
        ("y'' + 4*y = 0", Eq(y.diff(x, 2) + 4*y, 0)),
        ("y'' + 2*y' + y = 0", Eq(y.diff(x, 2) + 2*y.diff(x) + y, 0)),
        ("y'' + 4*y' + 4*y = 0", Eq(y.diff(x, 2) + 4*y.diff(x) + 4*y, 0)),
        ("y'' - 3*y' + 2*y = 0", Eq(y.diff(x, 2) - 3*y.diff(x) + 2*y, 0)),
    ]

    for desc, ode in const_coeff_homogeneous:
        try:
            solution = dsolve(ode, y)
            test_cases.append({
                "type": "ode_second_order_const_coeff_homogeneous",
                "description": desc,
                "input": {
                    "ode": str(ode),
                    "dependent_var": "y",
                    "independent_var": "x"
                },
                "expected_output": str(solution),
                "sympy_version": sp.__version__,
                "difficulty": "medium"
            })
        except Exception as e:
            print(f"Warning: Failed to solve {desc}: {e}")

    # Constant coefficients - non-homogeneous
    const_coeff_nonhomogeneous = [
        ("y'' + y = x", Eq(y.diff(x, 2) + y, x)),
        ("y'' + y = sin(x)", Eq(y.diff(x, 2) + y, sin(x))),
        ("y'' + y = e^x", Eq(y.diff(x, 2) + y, exp(x))),
    ]

    for desc, ode in const_coeff_nonhomogeneous:
        try:
            solution = dsolve(ode, y)
            test_cases.append({
                "type": "ode_second_order_const_coeff_nonhomogeneous",
                "description": desc,
                "input": {
                    "ode": str(ode),
                    "dependent_var": "y",
                    "independent_var": "x"
                },
                "expected_output": str(solution),
                "sympy_version": sp.__version__,
                "difficulty": "hard"
            })
        except Exception as e:
            print(f"Warning: Failed to solve {desc}: {e}")

    return test_cases


def generate_matrix_eigenvalue_tests() -> List[Dict[str, Any]]:
    """Generate eigenvalue/eigenvector test cases"""
    test_cases = []

    # 2x2 matrices
    matrices_2x2 = [
        ("Identity 2x2", Matrix([[1, 0], [0, 1]])),
        ("Diagonal 2x2", Matrix([[2, 0], [0, 3]])),
        ("Symmetric 2x2", Matrix([[2, 1], [1, 2]])),
        ("General 2x2", Matrix([[1, 2], [3, 4]])),
        ("Rotation-like 2x2", Matrix([[0, -1], [1, 0]])),
    ]

    for desc, matrix in matrices_2x2:
        try:
            eigenvals = matrix.eigenvals()
            eigenvects = matrix.eigenvects()

            test_cases.append({
                "type": "matrix_eigenvalues_2x2",
                "description": desc,
                "input": {
                    "matrix": str(matrix.tolist())
                },
                "expected_output": {
                    "eigenvalues": {str(k): v for k, v in eigenvals.items()},
                    "eigenvectors": str(eigenvects)
                },
                "sympy_version": sp.__version__,
                "difficulty": "simple"
            })
        except Exception as e:
            print(f"Warning: Failed eigenvalue computation for {desc}: {e}")

    # 3x3 matrices
    matrices_3x3 = [
        ("Identity 3x3", Matrix([[1, 0, 0], [0, 1, 0], [0, 0, 1]])),
        ("Diagonal 3x3", Matrix([[1, 0, 0], [0, 2, 0], [0, 0, 3]])),
        ("Symmetric 3x3", Matrix([[2, 1, 0], [1, 2, 1], [0, 1, 2]])),
    ]

    for desc, matrix in matrices_3x3:
        try:
            eigenvals = matrix.eigenvals()

            test_cases.append({
                "type": "matrix_eigenvalues_3x3",
                "description": desc,
                "input": {
                    "matrix": str(matrix.tolist())
                },
                "expected_output": {
                    "eigenvalues": {str(k): v for k, v in eigenvals.items()}
                },
                "sympy_version": sp.__version__,
                "difficulty": "medium"
            })
        except Exception as e:
            print(f"Warning: Failed eigenvalue computation for {desc}: {e}")

    return test_cases


def generate_matrix_decomposition_tests() -> List[Dict[str, Any]]:
    """Generate matrix decomposition test cases"""
    test_cases = []

    # QR decomposition
    qr_matrices = [
        ("QR 2x2", Matrix([[1, 2], [3, 4]])),
        ("QR 3x3", Matrix([[1, 2, 3], [4, 5, 6], [7, 8, 10]])),
    ]

    for desc, matrix in qr_matrices:
        try:
            Q, R = matrix.QRdecomposition()

            test_cases.append({
                "type": "matrix_qr_decomposition",
                "description": desc,
                "input": {
                    "matrix": str(matrix.tolist())
                },
                "expected_output": {
                    "Q": str(Q.tolist()),
                    "R": str(R.tolist())
                },
                "sympy_version": sp.__version__,
                "difficulty": "medium"
            })
        except Exception as e:
            print(f"Warning: Failed QR decomposition for {desc}: {e}")

    # LU decomposition
    lu_matrices = [
        ("LU 2x2", Matrix([[1, 2], [3, 4]])),
        ("LU 3x3", Matrix([[2, 1, 0], [1, 2, 1], [0, 1, 2]])),
    ]

    for desc, matrix in lu_matrices:
        try:
            L, U, perm = matrix.LUdecomposition()

            test_cases.append({
                "type": "matrix_lu_decomposition",
                "description": desc,
                "input": {
                    "matrix": str(matrix.tolist())
                },
                "expected_output": {
                    "L": str(L.tolist()),
                    "U": str(U.tolist()),
                    "permutation": str(perm)
                },
                "sympy_version": sp.__version__,
                "difficulty": "medium"
            })
        except Exception as e:
            print(f"Warning: Failed LU decomposition for {desc}: {e}")

    return test_cases


def generate_polynomial_factorization_tests() -> List[Dict[str, Any]]:
    """Generate polynomial factorization test cases"""
    x, y = symbols('x y')

    test_cases = []

    # Univariate polynomials
    univariate_polys = [
        ("x^2 - 1", x**2 - 1),
        ("x^2 + 2*x + 1", x**2 + 2*x + 1),
        ("x^3 - 1", x**3 - 1),
        ("x^4 - 1", x**4 - 1),
        ("x^3 + x^2 - x - 1", x**3 + x**2 - x - 1),
        ("x^4 + 4", x**4 + 4),
    ]

    for desc, poly in univariate_polys:
        try:
            factored = factor(poly)

            test_cases.append({
                "type": "polynomial_factorization_univariate",
                "description": desc,
                "input": {
                    "polynomial": str(poly),
                    "variables": ["x"]
                },
                "expected_output": str(factored),
                "sympy_version": sp.__version__,
                "difficulty": "simple"
            })
        except Exception as e:
            print(f"Warning: Failed factorization for {desc}: {e}")

    # Multivariate polynomials
    multivariate_polys = [
        ("x^2 - y^2", x**2 - y**2),
        ("x^2 + 2*x*y + y^2", x**2 + 2*x*y + y**2),
        ("x^3 - y^3", x**3 - y**3),
    ]

    for desc, poly in multivariate_polys:
        try:
            factored = factor(poly)

            test_cases.append({
                "type": "polynomial_factorization_multivariate",
                "description": desc,
                "input": {
                    "polynomial": str(poly),
                    "variables": ["x", "y"]
                },
                "expected_output": str(factored),
                "sympy_version": sp.__version__,
                "difficulty": "medium"
            })
        except Exception as e:
            print(f"Warning: Failed factorization for {desc}: {e}")

    return test_cases


def generate_series_expansion_tests() -> List[Dict[str, Any]]:
    """Generate series expansion test cases"""
    x = symbols('x')

    test_cases = []

    # Taylor series
    taylor_functions = [
        ("e^x", exp(x), 0, 10),
        ("sin(x)", sin(x), 0, 10),
        ("cos(x)", cos(x), 0, 10),
        ("ln(1+x)", log(1+x), 0, 10),
        ("(1+x)^(-1)", (1+x)**(-1), 0, 10),
        ("sqrt(1+x)", sqrt(1+x), 0, 10),
    ]

    for desc, func, point, order in taylor_functions:
        try:
            taylor = series(func, x, point, order)

            test_cases.append({
                "type": "series_taylor",
                "description": f"Taylor series of {desc} at x={point}, order {order}",
                "input": {
                    "function": str(func),
                    "variable": "x",
                    "point": point,
                    "order": order
                },
                "expected_output": str(taylor),
                "sympy_version": sp.__version__,
                "difficulty": "medium"
            })
        except Exception as e:
            print(f"Warning: Failed series expansion for {desc}: {e}")

    return test_cases


def generate_integration_tests() -> List[Dict[str, Any]]:
    """Generate symbolic integration test cases"""
    x = symbols('x')

    test_cases = []

    # Definite and indefinite integrals
    integrals = [
        ("∫ x dx", x, None, None, "polynomial"),
        ("∫ x^2 dx", x**2, None, None, "polynomial"),
        ("∫ sin(x) dx", sin(x), None, None, "trigonometric"),
        ("∫ cos(x) dx", cos(x), None, None, "trigonometric"),
        ("∫ e^x dx", exp(x), None, None, "exponential"),
        ("∫ 1/x dx", 1/x, None, None, "logarithmic"),
        ("∫_0^1 x dx", x, 0, 1, "definite"),
        ("∫_0^π sin(x) dx", sin(x), 0, pi, "definite_trig"),
    ]

    for desc, func, lower, upper, int_type in integrals:
        try:
            if lower is None and upper is None:
                result = integrate(func, x)
            else:
                result = integrate(func, (x, lower, upper))

            test_cases.append({
                "type": f"integration_{int_type}",
                "description": desc,
                "input": {
                    "integrand": str(func),
                    "variable": "x",
                    "lower_limit": str(lower) if lower is not None else None,
                    "upper_limit": str(upper) if upper is not None else None
                },
                "expected_output": str(result),
                "sympy_version": sp.__version__,
                "difficulty": "medium"
            })
        except Exception as e:
            print(f"Warning: Failed integration for {desc}: {e}")

    return test_cases


def generate_test_oracle() -> Dict[str, Any]:
    """Generate complete test oracle"""
    print("Generating test oracle from SymPy...")
    print(f"SymPy version: {sp.__version__}\n")

    oracle = {
        "metadata": {
            "generated_at": datetime.now().isoformat(),
            "sympy_version": sp.__version__,
            "purpose": "Mathematical correctness validation oracle for MathHook",
            "waves": {
                "wave_1": "ODEs",
                "wave_2": "Linear Algebra",
                "wave_3": "Polynomials",
                "wave_4": "Series & Special Functions",
                "wave_5": "PDEs",
                "wave_6": "Numerical Methods"
            }
        },
        "test_cases": {}
    }

    # Wave 1: ODEs
    print("Generating ODE test cases...")
    oracle["test_cases"]["ode_first_order"] = generate_ode_first_order_tests()
    oracle["test_cases"]["ode_second_order"] = generate_ode_second_order_tests()
    print(f"  Generated {len(oracle['test_cases']['ode_first_order'])} first-order ODE tests")
    print(f"  Generated {len(oracle['test_cases']['ode_second_order'])} second-order ODE tests")

    # Wave 2: Linear Algebra
    print("\nGenerating linear algebra test cases...")
    oracle["test_cases"]["matrix_eigenvalues"] = generate_matrix_eigenvalue_tests()
    oracle["test_cases"]["matrix_decompositions"] = generate_matrix_decomposition_tests()
    print(f"  Generated {len(oracle['test_cases']['matrix_eigenvalues'])} eigenvalue tests")
    print(f"  Generated {len(oracle['test_cases']['matrix_decompositions'])} decomposition tests")

    # Wave 3: Polynomials
    print("\nGenerating polynomial test cases...")
    oracle["test_cases"]["polynomial_factorization"] = generate_polynomial_factorization_tests()
    print(f"  Generated {len(oracle['test_cases']['polynomial_factorization'])} factorization tests")

    # Wave 4: Series
    print("\nGenerating series expansion test cases...")
    oracle["test_cases"]["series_expansions"] = generate_series_expansion_tests()
    oracle["test_cases"]["integration"] = generate_integration_tests()
    print(f"  Generated {len(oracle['test_cases']['series_expansions'])} series expansion tests")
    print(f"  Generated {len(oracle['test_cases']['integration'])} integration tests")

    # Calculate total
    total_tests = sum(
        len(tests) for tests in oracle["test_cases"].values()
    )

    oracle["metadata"]["total_test_cases"] = total_tests
    print(f"\n{'='*60}")
    print(f"Total test cases generated: {total_tests}")
    print(f"{'='*60}\n")

    return oracle


def main():
    """Main execution"""
    try:
        oracle = generate_test_oracle()

        output_file = ".research/test_oracle.json"
        print(f"Writing test oracle to {output_file}...")

        with open(output_file, 'w') as f:
            json.dump(oracle, f, indent=2)

        print(f"✓ Test oracle successfully generated!")
        print(f"✓ Total test cases: {oracle['metadata']['total_test_cases']}")
        print(f"✓ SymPy version: {oracle['metadata']['sympy_version']}")

        return 0

    except Exception as e:
        print(f"\nError generating test oracle: {e}")
        traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(main())
