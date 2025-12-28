"""
Simplification Benchmarks

Mirrors: benches/simplification_benchmarks.rs
Tests: Polynomial, trigonometric, logarithmic, rational simplification

Last Updated: 2025-12-28T1200
"""

import time
import statistics
from typing import Dict, List

try:
    from mathhook import symbol, symbols, parse, sin, cos, log
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
# Polynomial Simplification Benchmarks
# ============================================================================

def bench_collect_like_terms_direct():
    """Benchmark collect like terms (direct API): 3x + 2x + x."""
    x = symbol('x')
    expr = 3*x + 2*x + x
    result = expr.simplify()
    return result


def bench_collect_like_terms_with_parsing():
    """Benchmark collect like terms (with parsing): 3x + 2x + x."""
    expr = parse("3*x + 2*x + x")
    result = expr.simplify()
    return result


def bench_expand_product_direct():
    """Benchmark expand product (direct API): (x + 1)(x + 2)."""
    x = symbol('x')
    expr = (x + 1) * (x + 2)
    result = expr.simplify()
    return result


def bench_expand_product_with_parsing():
    """Benchmark expand product (with parsing): (x + 1)(x + 2)."""
    expr = parse("(x + 1) * (x + 2)")
    result = expr.simplify()
    return result


def bench_combine_powers_direct():
    """Benchmark combine powers (direct API): x^2 * x^3."""
    x = symbol('x')
    expr = x**2 * x**3
    result = expr.simplify()
    return result


def bench_combine_powers_with_parsing():
    """Benchmark combine powers (with parsing): x^2 * x^3."""
    expr = parse("x^2 * x^3")
    result = expr.simplify()
    return result


def bench_binomial_expansion_direct():
    """Benchmark binomial expansion (direct API): (x + 1)^5."""
    x = symbol('x')
    expr = (x + 1) ** 5
    result = expr.simplify()
    return result


def bench_binomial_expansion_with_parsing():
    """Benchmark binomial expansion (with parsing): (x + 1)^5."""
    expr = parse("(x + 1)^5")
    result = expr.simplify()
    return result


# ============================================================================
# Trigonometric Simplification Benchmarks
# ============================================================================

def bench_pythagorean_identity_direct():
    """Benchmark Pythagorean identity (direct API): sin^2(x) + cos^2(x)."""
    x = symbol('x')
    expr = sin(x)**2 + cos(x)**2
    result = expr.simplify()
    return result


def bench_pythagorean_identity_with_parsing():
    """Benchmark Pythagorean identity (with parsing): sin^2(x) + cos^2(x)."""
    expr = parse("sin(x)^2 + cos(x)^2")
    result = expr.simplify()
    return result


def bench_double_angle_direct():
    """Benchmark double angle (direct API): 2*sin(x)*cos(x)."""
    x = symbol('x')
    expr = 2 * sin(x) * cos(x)
    result = expr.simplify()
    return result


def bench_double_angle_with_parsing():
    """Benchmark double angle (with parsing): 2*sin(x)*cos(x)."""
    expr = parse("2 * sin(x) * cos(x)")
    result = expr.simplify()
    return result


def bench_trig_quotient_direct():
    """Benchmark trig quotient (direct API): sin(x)/cos(x)."""
    x = symbol('x')
    expr = sin(x) / cos(x)
    result = expr.simplify()
    return result


def bench_trig_quotient_with_parsing():
    """Benchmark trig quotient (with parsing): sin(x)/cos(x)."""
    expr = parse("sin(x) / cos(x)")
    result = expr.simplify()
    return result


# ============================================================================
# Logarithmic Simplification Benchmarks
# ============================================================================

def bench_log_product_rule_direct():
    """Benchmark log product rule (direct API): log(x) + log(y)."""
    x, y = symbols('x y')
    expr = log(x) + log(y)
    result = expr.simplify()
    return result


def bench_log_product_rule_with_parsing():
    """Benchmark log product rule (with parsing): log(x) + log(y)."""
    expr = parse("log(x) + log(y)")
    result = expr.simplify()
    return result


def bench_log_quotient_rule_direct():
    """Benchmark log quotient rule (direct API): log(x) - log(y)."""
    x, y = symbols('x y')
    expr = log(x) - log(y)
    result = expr.simplify()
    return result


def bench_log_quotient_rule_with_parsing():
    """Benchmark log quotient rule (with parsing): log(x) - log(y)."""
    expr = parse("log(x) - log(y)")
    result = expr.simplify()
    return result


def bench_log_power_rule_direct():
    """Benchmark log power rule (direct API): 3*log(x)."""
    x = symbol('x')
    expr = 3 * log(x)
    result = expr.simplify()
    return result


def bench_log_power_rule_with_parsing():
    """Benchmark log power rule (with parsing): 3*log(x)."""
    expr = parse("3 * log(x)")
    result = expr.simplify()
    return result


# ============================================================================
# Rational Simplification Benchmarks
# ============================================================================

def bench_simple_rational_direct():
    """Benchmark simple rational (direct API): (x^2 - 1)/(x - 1)."""
    x = symbol('x')
    expr = (x**2 - 1) / (x - 1)
    result = expr.simplify()
    return result


def bench_simple_rational_with_parsing():
    """Benchmark simple rational (with parsing): (x^2 - 1)/(x - 1)."""
    expr = parse("(x^2 - 1) / (x - 1)")
    result = expr.simplify()
    return result


def bench_complex_rational_direct():
    """Benchmark complex rational (direct API): (x^3 - 8)/(x - 2)."""
    x = symbol('x')
    expr = (x**3 - 8) / (x - 2)
    result = expr.simplify()
    return result


def bench_complex_rational_with_parsing():
    """Benchmark complex rational (with parsing): (x^3 - 8)/(x - 2)."""
    expr = parse("(x^3 - 8) / (x - 2)")
    result = expr.simplify()
    return result


# ============================================================================
# Zero Detection Benchmarks
# ============================================================================

def bench_obvious_zero_direct():
    """Benchmark obvious zero (direct API): x - x."""
    x = symbol('x')
    expr = x - x
    result = expr.simplify()
    return result


def bench_obvious_zero_with_parsing():
    """Benchmark obvious zero (with parsing): x - x."""
    expr = parse("x - x")
    result = expr.simplify()
    return result


def bench_identity_simplification_direct():
    """Benchmark identity simplification (direct API): x * 1."""
    x = symbol('x')
    expr = x * 1
    result = expr.simplify()
    return result


def bench_identity_simplification_with_parsing():
    """Benchmark identity simplification (with parsing): x * 1."""
    expr = parse("x * 1")
    result = expr.simplify()
    return result


def bench_additive_identity_direct():
    """Benchmark additive identity (direct API): x + 0."""
    x = symbol('x')
    expr = x + 0
    result = expr.simplify()
    return result


def bench_additive_identity_with_parsing():
    """Benchmark additive identity (with parsing): x + 0."""
    expr = parse("x + 0")
    result = expr.simplify()
    return result


# ============================================================================
# Benchmark Runner
# ============================================================================

def run_all_benchmarks(samples: int = 100) -> Dict[str, BenchmarkResult]:
    """Run all simplification benchmarks."""
    results = {}

    benchmarks = [
        # Polynomial simplification
        bench_collect_like_terms_direct,
        bench_collect_like_terms_with_parsing,
        bench_expand_product_direct,
        bench_expand_product_with_parsing,
        bench_combine_powers_direct,
        bench_combine_powers_with_parsing,
        bench_binomial_expansion_direct,
        bench_binomial_expansion_with_parsing,

        # Trigonometric simplification
        bench_pythagorean_identity_direct,
        bench_pythagorean_identity_with_parsing,
        bench_double_angle_direct,
        bench_double_angle_with_parsing,
        bench_trig_quotient_direct,
        bench_trig_quotient_with_parsing,

        # Logarithmic simplification
        bench_log_product_rule_direct,
        bench_log_product_rule_with_parsing,
        bench_log_quotient_rule_direct,
        bench_log_quotient_rule_with_parsing,
        bench_log_power_rule_direct,
        bench_log_power_rule_with_parsing,

        # Rational simplification
        bench_simple_rational_direct,
        bench_simple_rational_with_parsing,
        bench_complex_rational_direct,
        bench_complex_rational_with_parsing,

        # Zero detection
        bench_obvious_zero_direct,
        bench_obvious_zero_with_parsing,
        bench_identity_simplification_direct,
        bench_identity_simplification_with_parsing,
        bench_additive_identity_direct,
        bench_additive_identity_with_parsing,
    ]

    print("=" * 80)
    print("Simplification Benchmarks")
    print("=" * 80)

    for bench_func in benchmarks:
        print(f"Running {bench_func.__name__}...", end=" ")
        result = benchmark(bench_func, samples=samples)
        results[bench_func.__name__] = result
        print(f"{result.mean_ns:.2f}ns")

    print("=" * 80)

    return results


def main():
    """Main entry point for simplification benchmarks."""
    results = run_all_benchmarks(samples=100)

    print("\nDetailed Results:")
    print("-" * 80)
    for name, result in results.items():
        print(result)


if __name__ == "__main__":
    main()
