"""
Solving Benchmarks

Mirrors: benches/solving_benchmarks.rs
Tests: Linear, quadratic, polynomial, system, matrix equation solving
"""

import time
import statistics
from typing import Dict, List

try:
    from mathhook import Expression, Symbol, MathSolver
except ImportError:
    print("ERROR: mathhook Python bindings not found. Install with: pip install mathhook")
    exit(1)


class BenchmarkResult:
    """Stores benchmark timing results."""

    def __init__(self, name: str, times_ns: List[int]):
        self.name = name
        self.times_ns = times_ns
        self.mean_ns = statistics.mean(times_ns)
        self.median_ns = statistics.median(times_ns)
        self.min_ns = min(times_ns)
        self.max_ns = max(times_ns)
        self.std_dev_ns = statistics.stdev(times_ns) if len(times_ns) > 1 else 0.0
        self.samples = len(times_ns)

    def __repr__(self):
        return (f"{self.name}: {self.mean_ns:.2f}ns ± {self.std_dev_ns:.2f}ns "
                f"(median: {self.median_ns:.2f}ns)")


def benchmark(func, samples: int = 100, warmup: int = 10) -> BenchmarkResult:
    """Benchmark a function with warmup and multiple samples."""
    for _ in range(warmup):
        func()

    times_ns = []
    for _ in range(samples):
        start = time.perf_counter_ns()
        func()
        end = time.perf_counter_ns()
        times_ns.append(end - start)

    return BenchmarkResult(func.__name__, times_ns)


# ============================================================================
# Linear Equation Solving Benchmarks
# ============================================================================

def bench_linear_simple_direct():
    """Benchmark simple linear equation solving (direct API): 2x + 3 = 0."""
    solver = MathSolver()
    x = Symbol("x")
    equation = Expression.add([
        Expression.multiply([Expression.integer(2), Expression.symbol(x)]),
        Expression.integer(3)
    ])
    result = solver.solve(equation, x)
    return result


def bench_linear_simple_with_parsing():
    """Benchmark simple linear equation solving (with parsing): 2x + 3 = 0."""
    solver = MathSolver()
    equation = Expression.parse("2*x + 3")
    x = Symbol("x")
    result = solver.solve(equation, x)
    return result


def bench_linear_large_coeffs_direct():
    """Benchmark linear with large coefficients (direct API)."""
    solver = MathSolver()
    x = Symbol("x")
    equation = Expression.add([
        Expression.multiply([Expression.integer(1000), Expression.symbol(x)]),
        Expression.integer(500)
    ])
    result = solver.solve(equation, x)
    return result


def bench_linear_large_coeffs_with_parsing():
    """Benchmark linear with large coefficients (with parsing)."""
    solver = MathSolver()
    equation = Expression.parse("1000*x + 500")
    x = Symbol("x")
    result = solver.solve(equation, x)
    return result


# ============================================================================
# Quadratic Equation Solving Benchmarks
# ============================================================================

def bench_quadratic_simple_direct():
    """Benchmark simple quadratic (direct API): x^2 - 4 = 0."""
    solver = MathSolver()
    x = Symbol("x")
    equation = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(2)),
        Expression.integer(-4)
    ])
    result = solver.solve(equation, x)
    return result


def bench_quadratic_simple_with_parsing():
    """Benchmark simple quadratic (with parsing): x^2 - 4 = 0."""
    solver = MathSolver()
    equation = Expression.parse("x^2 - 4")
    x = Symbol("x")
    result = solver.solve(equation, x)
    return result


def bench_quadratic_complex_roots_direct():
    """Benchmark quadratic with complex roots (direct API): x^2 + 1 = 0."""
    solver = MathSolver()
    x = Symbol("x")
    equation = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(2)),
        Expression.integer(1)
    ])
    result = solver.solve(equation, x)
    return result


def bench_quadratic_complex_roots_with_parsing():
    """Benchmark quadratic with complex roots (with parsing): x^2 + 1 = 0."""
    solver = MathSolver()
    equation = Expression.parse("x^2 + 1")
    x = Symbol("x")
    result = solver.solve(equation, x)
    return result


def bench_quadratic_general_direct():
    """Benchmark general quadratic (direct API): 2x^2 + 3x - 5 = 0."""
    solver = MathSolver()
    x = Symbol("x")
    equation = Expression.add([
        Expression.multiply([
            Expression.integer(2),
            Expression.pow(Expression.symbol(x), Expression.integer(2))
        ]),
        Expression.multiply([Expression.integer(3), Expression.symbol(x)]),
        Expression.integer(-5)
    ])
    result = solver.solve(equation, x)
    return result


def bench_quadratic_general_with_parsing():
    """Benchmark general quadratic (with parsing): 2x^2 + 3x - 5 = 0."""
    solver = MathSolver()
    equation = Expression.parse("2*x^2 + 3*x - 5")
    x = Symbol("x")
    result = solver.solve(equation, x)
    return result


# ============================================================================
# Polynomial Equation Solving Benchmarks
# ============================================================================

def bench_cubic_equation_direct():
    """Benchmark cubic equation (direct API): x^3 - 6x^2 + 11x - 6 = 0."""
    solver = MathSolver()
    x = Symbol("x")
    equation = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(3)),
        Expression.multiply([
            Expression.integer(-6),
            Expression.pow(Expression.symbol(x), Expression.integer(2))
        ]),
        Expression.multiply([Expression.integer(11), Expression.symbol(x)]),
        Expression.integer(-6)
    ])
    result = solver.solve(equation, x)
    return result


def bench_cubic_equation_with_parsing():
    """Benchmark cubic equation (with parsing): x^3 - 6x^2 + 11x - 6 = 0."""
    solver = MathSolver()
    equation = Expression.parse("x^3 - 6*x^2 + 11*x - 6")
    x = Symbol("x")
    result = solver.solve(equation, x)
    return result


def bench_quartic_equation_direct():
    """Benchmark quartic equation (direct API): x^4 - 5x^2 + 4 = 0."""
    solver = MathSolver()
    x = Symbol("x")
    equation = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(4)),
        Expression.multiply([
            Expression.integer(-5),
            Expression.pow(Expression.symbol(x), Expression.integer(2))
        ]),
        Expression.integer(4)
    ])
    result = solver.solve(equation, x)
    return result


def bench_quartic_equation_with_parsing():
    """Benchmark quartic equation (with parsing): x^4 - 5x^2 + 4 = 0."""
    solver = MathSolver()
    equation = Expression.parse("x^4 - 5*x^2 + 4")
    x = Symbol("x")
    result = solver.solve(equation, x)
    return result


# ============================================================================
# System of Equations Solving Benchmarks
# ============================================================================

def bench_system_2x2_direct():
    """Benchmark 2x2 linear system (direct API): x + y = 3, 2x - y = 0."""
    solver = MathSolver()
    x = Symbol("x")
    y = Symbol("y")
    eq1 = Expression.add([
        Expression.symbol(x),
        Expression.symbol(y),
        Expression.integer(-3)
    ])
    eq2 = Expression.add([
        Expression.multiply([Expression.integer(2), Expression.symbol(x)]),
        Expression.multiply([Expression.integer(-1), Expression.symbol(y)])
    ])
    result = solver.solve_system([eq1, eq2], [x, y])
    return result


def bench_system_2x2_with_parsing():
    """Benchmark 2x2 linear system (with parsing): x + y = 3, 2x - y = 0."""
    solver = MathSolver()
    eq1 = Expression.parse("x + y - 3")
    eq2 = Expression.parse("2*x - y")
    x = Symbol("x")
    y = Symbol("y")
    result = solver.solve_system([eq1, eq2], [x, y])
    return result


def bench_system_3x3_direct():
    """Benchmark 3x3 linear system (direct API)."""
    solver = MathSolver()
    x = Symbol("x")
    y = Symbol("y")
    z = Symbol("z")
    eq1 = Expression.add([
        Expression.symbol(x),
        Expression.symbol(y),
        Expression.symbol(z),
        Expression.integer(-6)
    ])
    eq2 = Expression.add([
        Expression.multiply([Expression.integer(2), Expression.symbol(x)]),
        Expression.symbol(y),
        Expression.integer(-3)
    ])
    eq3 = Expression.add([
        Expression.symbol(x),
        Expression.multiply([Expression.integer(-1), Expression.symbol(y)]),
        Expression.multiply([Expression.integer(2), Expression.symbol(z)]),
        Expression.integer(-1)
    ])
    result = solver.solve_system([eq1, eq2, eq3], [x, y, z])
    return result


def bench_system_3x3_with_parsing():
    """Benchmark 3x3 linear system (with parsing)."""
    solver = MathSolver()
    eq1 = Expression.parse("x + y + z - 6")
    eq2 = Expression.parse("2*x + y - 3")
    eq3 = Expression.parse("x - y + 2*z - 1")
    x = Symbol("x")
    y = Symbol("y")
    z = Symbol("z")
    result = solver.solve_system([eq1, eq2, eq3], [x, y, z])
    return result


# ============================================================================
# Benchmark Runner
# ============================================================================

def run_all_benchmarks(samples: int = 100) -> Dict[str, BenchmarkResult]:
    """Run all solving benchmarks."""
    results = {}

    benchmarks = [
        # Linear equations
        bench_linear_simple_direct,
        bench_linear_simple_with_parsing,
        bench_linear_large_coeffs_direct,
        bench_linear_large_coeffs_with_parsing,

        # Quadratic equations
        bench_quadratic_simple_direct,
        bench_quadratic_simple_with_parsing,
        bench_quadratic_complex_roots_direct,
        bench_quadratic_complex_roots_with_parsing,
        bench_quadratic_general_direct,
        bench_quadratic_general_with_parsing,

        # Polynomial equations
        bench_cubic_equation_direct,
        bench_cubic_equation_with_parsing,
        bench_quartic_equation_direct,
        bench_quartic_equation_with_parsing,

        # System of equations
        bench_system_2x2_direct,
        bench_system_2x2_with_parsing,
        bench_system_3x3_direct,
        bench_system_3x3_with_parsing,
    ]

    print("=" * 80)
    print("Solving Benchmarks")
    print("=" * 80)

    for bench_func in benchmarks:
        print(f"Running {bench_func.__name__}...", end=" ")
        result = benchmark(bench_func, samples=samples)
        results[bench_func.__name__] = result
        print(f"{result.mean_ns:.2f}ns")

    print("=" * 80)

    return results


def main():
    """Main entry point for solving benchmarks."""
    results = run_all_benchmarks(samples=100)

    print("\nDetailed Results:")
    print("-" * 80)
    for name, result in results.items():
        print(result)


if __name__ == "__main__":
    main()
