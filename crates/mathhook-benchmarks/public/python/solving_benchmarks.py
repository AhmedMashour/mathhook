"""
Solving Benchmarks

Mirrors: benches/solving_benchmarks.rs
Tests: Linear, quadratic, polynomial, system equation solving

Last Updated: 2025-12-28T1200
"""

import time
import statistics
from typing import Dict, List

try:
    from mathhook import symbol, symbols, parse, solve
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
    x = symbol('x')
    equation = 2*x + 3
    result = solve(equation, 'x')
    return result


def bench_linear_simple_with_parsing():
    """Benchmark simple linear equation solving (with parsing): 2x + 3 = 0."""
    result = solve(parse("2*x + 3"), 'x')
    return result


def bench_linear_large_coeffs_direct():
    """Benchmark linear with large coefficients (direct API)."""
    x = symbol('x')
    equation = 1000*x + 500
    result = solve(equation, 'x')
    return result


def bench_linear_large_coeffs_with_parsing():
    """Benchmark linear with large coefficients (with parsing)."""
    result = solve(parse("1000*x + 500"), 'x')
    return result


# ============================================================================
# Quadratic Equation Solving Benchmarks
# ============================================================================

def bench_quadratic_simple_direct():
    """Benchmark simple quadratic (direct API): x^2 - 4 = 0."""
    x = symbol('x')
    equation = x**2 - 4
    result = solve(equation, 'x')
    return result


def bench_quadratic_simple_with_parsing():
    """Benchmark simple quadratic (with parsing): x^2 - 4 = 0."""
    result = solve(parse("x^2 - 4"), 'x')
    return result


def bench_quadratic_complex_roots_direct():
    """Benchmark quadratic with complex roots (direct API): x^2 + 1 = 0."""
    x = symbol('x')
    equation = x**2 + 1
    result = solve(equation, 'x')
    return result


def bench_quadratic_complex_roots_with_parsing():
    """Benchmark quadratic with complex roots (with parsing): x^2 + 1 = 0."""
    result = solve(parse("x^2 + 1"), 'x')
    return result


def bench_quadratic_general_direct():
    """Benchmark general quadratic (direct API): 2x^2 + 3x - 5 = 0."""
    x = symbol('x')
    equation = 2*x**2 + 3*x - 5
    result = solve(equation, 'x')
    return result


def bench_quadratic_general_with_parsing():
    """Benchmark general quadratic (with parsing): 2x^2 + 3x - 5 = 0."""
    result = solve(parse("2*x^2 + 3*x - 5"), 'x')
    return result


# ============================================================================
# Polynomial Equation Solving Benchmarks
# ============================================================================

def bench_cubic_equation_direct():
    """Benchmark cubic equation (direct API): x^3 - 6x^2 + 11x - 6 = 0."""
    x = symbol('x')
    equation = x**3 - 6*x**2 + 11*x - 6
    result = solve(equation, 'x')
    return result


def bench_cubic_equation_with_parsing():
    """Benchmark cubic equation (with parsing): x^3 - 6x^2 + 11x - 6 = 0."""
    result = solve(parse("x^3 - 6*x^2 + 11*x - 6"), 'x')
    return result


def bench_quartic_equation_direct():
    """Benchmark quartic equation (direct API): x^4 - 5x^2 + 4 = 0."""
    x = symbol('x')
    equation = x**4 - 5*x**2 + 4
    result = solve(equation, 'x')
    return result


def bench_quartic_equation_with_parsing():
    """Benchmark quartic equation (with parsing): x^4 - 5x^2 + 4 = 0."""
    result = solve(parse("x^4 - 5*x^2 + 4"), 'x')
    return result


# ============================================================================
# System of Equations Solving Benchmarks (using individual solves)
# ============================================================================

def bench_system_2_equations_direct():
    """Benchmark solving 2 equations sequentially (direct API)."""
    x, y = symbols('x y')
    # Solve x + y = 3 for x, then substitute
    eq1 = x + y - 3
    eq2 = 2*x - y

    # Solve first equation
    solutions1 = solve(eq1, 'x')
    # Solve second equation
    solutions2 = solve(eq2, 'x')
    return (solutions1, solutions2)


def bench_system_2_equations_with_parsing():
    """Benchmark solving 2 equations sequentially (with parsing)."""
    eq1 = parse("x + y - 3")
    eq2 = parse("2*x - y")

    solutions1 = solve(eq1, 'x')
    solutions2 = solve(eq2, 'x')
    return (solutions1, solutions2)


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
        bench_system_2_equations_direct,
        bench_system_2_equations_with_parsing,
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
