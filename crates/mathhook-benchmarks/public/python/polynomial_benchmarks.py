"""
Polynomial Benchmarks

Mirrors: benches/polynomial_benchmarks.rs
Tests: GCD, division, factorization, special polynomials, finite field operations
"""

import time
import statistics
from typing import Dict, List

try:
    from mathhook import Expression, Symbol
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
# GCD Algorithm Benchmarks
# ============================================================================

def bench_gcd_univariate_simple_direct():
    """Benchmark univariate GCD simple (direct API): gcd(x^2-1, x-1)."""
    x = Symbol("x")
    f = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(2)),
        Expression.integer(-1)
    ])
    g = Expression.add([Expression.symbol(x), Expression.integer(-1)])
    result = Expression.gcd(f, g)
    return result


def bench_gcd_univariate_simple_with_parsing():
    """Benchmark univariate GCD simple (with parsing): gcd(x^2-1, x-1)."""
    f = Expression.parse("x^2 - 1")
    g = Expression.parse("x - 1")
    result = Expression.gcd(f, g)
    return result


def bench_gcd_univariate_degree_10_direct():
    """Benchmark univariate GCD degree 10 (direct API)."""
    x = Symbol("x")
    f_terms = [Expression.integer(-1)]
    for i in range(1, 11):
        f_terms.append(Expression.multiply([
            Expression.integer(i),
            Expression.pow(Expression.symbol(x), Expression.integer(i))
        ]))
    f = Expression.add(f_terms)
    g = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(5)),
        Expression.integer(-1)
    ])
    result = Expression.gcd(f, g)
    return result


def bench_gcd_univariate_degree_10_with_parsing():
    """Benchmark univariate GCD degree 10 (with parsing)."""
    f = Expression.parse("x^10 + 10*x^9 + 9*x^8 + 8*x^7 + 7*x^6 + 6*x^5 + 5*x^4 + 4*x^3 + 3*x^2 + 2*x - 1")
    g = Expression.parse("x^5 - 1")
    result = Expression.gcd(f, g)
    return result


def bench_gcd_bivariate_simple_direct():
    """Benchmark bivariate GCD simple (direct API): gcd(x*y, x*(y+1))."""
    x = Symbol("x")
    y = Symbol("y")
    f = Expression.multiply([Expression.symbol(x), Expression.symbol(y)])
    g = Expression.multiply([
        Expression.symbol(x),
        Expression.add([Expression.symbol(y), Expression.integer(1)])
    ])
    result = Expression.gcd(f, g)
    return result


def bench_gcd_bivariate_simple_with_parsing():
    """Benchmark bivariate GCD simple (with parsing): gcd(x*y, x*(y+1))."""
    f = Expression.parse("x * y")
    g = Expression.parse("x * (y + 1)")
    result = Expression.gcd(f, g)
    return result


# ============================================================================
# Polynomial Division Benchmarks
# ============================================================================

def bench_division_simple_direct():
    """Benchmark simple division (direct API): (x^2-1)/(x-1)."""
    x = Symbol("x")
    dividend = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(2)),
        Expression.integer(-1)
    ])
    divisor = Expression.add([Expression.symbol(x), Expression.integer(-1)])
    result = Expression.divide(dividend, divisor)
    return result


def bench_division_simple_with_parsing():
    """Benchmark simple division (with parsing): (x^2-1)/(x-1)."""
    expr = Expression.parse("(x^2 - 1) / (x - 1)")
    result = expr.simplify()
    return result


def bench_division_degree_8_direct():
    """Benchmark higher degree division (direct API): (x^8-1)/(x^2-1)."""
    x = Symbol("x")
    dividend = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(8)),
        Expression.integer(-1)
    ])
    divisor = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(2)),
        Expression.integer(-1)
    ])
    result = Expression.divide(dividend, divisor)
    return result


def bench_division_degree_8_with_parsing():
    """Benchmark higher degree division (with parsing): (x^8-1)/(x^2-1)."""
    expr = Expression.parse("(x^8 - 1) / (x^2 - 1)")
    result = expr.simplify()
    return result


# ============================================================================
# Factorization Benchmarks
# ============================================================================

def bench_factor_quadratic_direct():
    """Benchmark factor quadratic (direct API): factor(x^2-1)."""
    x = Symbol("x")
    poly = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(2)),
        Expression.integer(-1)
    ])
    result = Expression.factor(poly)
    return result


def bench_factor_quadratic_with_parsing():
    """Benchmark factor quadratic (with parsing): factor(x^2-1)."""
    poly = Expression.parse("x^2 - 1")
    result = Expression.factor(poly)
    return result


def bench_factor_cubic_direct():
    """Benchmark factor cubic (direct API): factor(x^3-1)."""
    x = Symbol("x")
    poly = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(3)),
        Expression.integer(-1)
    ])
    result = Expression.factor(poly)
    return result


def bench_factor_cubic_with_parsing():
    """Benchmark factor cubic (with parsing): factor(x^3-1)."""
    poly = Expression.parse("x^3 - 1")
    result = Expression.factor(poly)
    return result


def bench_common_factor_extraction_direct():
    """Benchmark common factor extraction (direct API): 6x^2 + 12x + 18."""
    x = Symbol("x")
    poly = Expression.add([
        Expression.multiply([
            Expression.integer(6),
            Expression.pow(Expression.symbol(x), Expression.integer(2))
        ]),
        Expression.multiply([Expression.integer(12), Expression.symbol(x)]),
        Expression.integer(18)
    ])
    result = Expression.factor(poly)
    return result


def bench_common_factor_extraction_with_parsing():
    """Benchmark common factor extraction (with parsing): 6x^2 + 12x + 18."""
    poly = Expression.parse("6*x^2 + 12*x + 18")
    result = Expression.factor(poly)
    return result


# ============================================================================
# Polynomial Multiplication Benchmarks
# ============================================================================

def bench_poly_multiply_small_direct():
    """Benchmark small polynomial multiplication (direct API): (x+1)*(x+2)."""
    x = Symbol("x")
    f = Expression.add([Expression.symbol(x), Expression.integer(1)])
    g = Expression.add([Expression.symbol(x), Expression.integer(2)])
    result = Expression.multiply([f, g])
    result = result.simplify()
    return result


def bench_poly_multiply_small_with_parsing():
    """Benchmark small polynomial multiplication (with parsing): (x+1)*(x+2)."""
    expr = Expression.parse("(x + 1) * (x + 2)")
    result = expr.simplify()
    return result


def bench_poly_multiply_medium_direct():
    """Benchmark medium polynomial multiplication (direct API)."""
    x = Symbol("x")
    f = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(2)),
        Expression.symbol(x),
        Expression.integer(1)
    ])
    g = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(2)),
        Expression.integer(-1)
    ])
    result = Expression.multiply([f, g])
    result = result.simplify()
    return result


def bench_poly_multiply_medium_with_parsing():
    """Benchmark medium polynomial multiplication (with parsing)."""
    expr = Expression.parse("(x^2 + x + 1) * (x^2 - 1)")
    result = expr.simplify()
    return result


# ============================================================================
# Special Polynomial Family Benchmarks
# ============================================================================

def bench_legendre_degree_5_direct():
    """Benchmark Legendre polynomial degree 5 (direct API)."""
    expr = Expression.function("legendre", [Expression.integer(5)])
    result = expr.simplify()
    return result


def bench_legendre_degree_5_with_parsing():
    """Benchmark Legendre polynomial degree 5 (with parsing)."""
    expr = Expression.parse("legendre(5)")
    result = expr.simplify()
    return result


def bench_chebyshev_degree_5_direct():
    """Benchmark Chebyshev polynomial degree 5 (direct API)."""
    expr = Expression.function("chebyshev_t", [Expression.integer(5)])
    result = expr.simplify()
    return result


def bench_chebyshev_degree_5_with_parsing():
    """Benchmark Chebyshev polynomial degree 5 (with parsing)."""
    expr = Expression.parse("chebyshev_t(5)")
    result = expr.simplify()
    return result


def bench_hermite_degree_5_direct():
    """Benchmark Hermite polynomial degree 5 (direct API)."""
    expr = Expression.function("hermite", [Expression.integer(5)])
    result = expr.simplify()
    return result


def bench_hermite_degree_5_with_parsing():
    """Benchmark Hermite polynomial degree 5 (with parsing)."""
    expr = Expression.parse("hermite(5)")
    result = expr.simplify()
    return result


# ============================================================================
# Resultant Benchmarks
# ============================================================================

def bench_resultant_quadratic_direct():
    """Benchmark resultant of quadratics (direct API)."""
    x = Symbol("x")
    f = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(2)),
        Expression.symbol(x),
        Expression.integer(1)
    ])
    g = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(2)),
        Expression.integer(-1)
    ])
    result = Expression.resultant(f, g)
    return result


def bench_resultant_quadratic_with_parsing():
    """Benchmark resultant of quadratics (with parsing)."""
    f = Expression.parse("x^2 + x + 1")
    g = Expression.parse("x^2 - 1")
    result = Expression.resultant(f, g)
    return result


# ============================================================================
# Benchmark Runner
# ============================================================================

def run_all_benchmarks(samples: int = 50) -> Dict[str, BenchmarkResult]:
    """Run all polynomial benchmarks."""
    results = {}

    benchmarks = [
        # GCD algorithms
        bench_gcd_univariate_simple_direct,
        bench_gcd_univariate_simple_with_parsing,
        bench_gcd_univariate_degree_10_direct,
        bench_gcd_univariate_degree_10_with_parsing,
        bench_gcd_bivariate_simple_direct,
        bench_gcd_bivariate_simple_with_parsing,

        # Division
        bench_division_simple_direct,
        bench_division_simple_with_parsing,
        bench_division_degree_8_direct,
        bench_division_degree_8_with_parsing,

        # Factorization
        bench_factor_quadratic_direct,
        bench_factor_quadratic_with_parsing,
        bench_factor_cubic_direct,
        bench_factor_cubic_with_parsing,
        bench_common_factor_extraction_direct,
        bench_common_factor_extraction_with_parsing,

        # Multiplication
        bench_poly_multiply_small_direct,
        bench_poly_multiply_small_with_parsing,
        bench_poly_multiply_medium_direct,
        bench_poly_multiply_medium_with_parsing,

        # Special polynomials
        bench_legendre_degree_5_direct,
        bench_legendre_degree_5_with_parsing,
        bench_chebyshev_degree_5_direct,
        bench_chebyshev_degree_5_with_parsing,
        bench_hermite_degree_5_direct,
        bench_hermite_degree_5_with_parsing,

        # Resultant
        bench_resultant_quadratic_direct,
        bench_resultant_quadratic_with_parsing,
    ]

    print("=" * 80)
    print("Polynomial Benchmarks")
    print("=" * 80)

    for bench_func in benchmarks:
        print(f"Running {bench_func.__name__}...", end=" ")
        result = benchmark(bench_func, samples=samples)
        results[bench_func.__name__] = result
        print(f"{result.mean_ns:.2f}ns")

    print("=" * 80)

    return results


def main():
    """Main entry point for polynomial benchmarks."""
    results = run_all_benchmarks(samples=50)

    print("\nDetailed Results:")
    print("-" * 80)
    for name, result in results.items():
        print(result)


if __name__ == "__main__":
    main()
