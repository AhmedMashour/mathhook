"""
Parsing Benchmarks

Mirrors: benches/parsing_benchmarks.rs
Tests: Simple/complex parsing, implicit multiplication, formatting
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
# Simple Parsing Benchmarks
# ============================================================================

def bench_parse_variable():
    """Benchmark parsing a variable: x."""
    result = Expression.parse("x")
    return result


def bench_parse_number():
    """Benchmark parsing a number: 42."""
    result = Expression.parse("42")
    return result


def bench_parse_addition():
    """Benchmark parsing addition: x + y."""
    result = Expression.parse("x + y")
    return result


def bench_parse_multiplication():
    """Benchmark parsing multiplication: x * y."""
    result = Expression.parse("x * y")
    return result


def bench_parse_power():
    """Benchmark parsing power: x^2."""
    result = Expression.parse("x^2")
    return result


def bench_parse_sin():
    """Benchmark parsing sin function: sin(x)."""
    result = Expression.parse("sin(x)")
    return result


# ============================================================================
# Complex Parsing Benchmarks
# ============================================================================

def bench_parse_polynomial():
    """Benchmark parsing polynomial: x^3 + 2*x^2 - 5*x + 3."""
    result = Expression.parse("x^3 + 2*x^2 - 5*x + 3")
    return result


def bench_parse_nested_functions():
    """Benchmark parsing nested functions: sin(cos(x))."""
    result = Expression.parse("sin(cos(x))")
    return result


def bench_parse_complex_fraction():
    """Benchmark parsing complex fraction: (x + 1) / (x - 1)."""
    result = Expression.parse("(x + 1) / (x - 1)")
    return result


def bench_parse_trig_identity():
    """Benchmark parsing trig identity: sin(x)^2 + cos(x)^2."""
    result = Expression.parse("sin(x)^2 + cos(x)^2")
    return result


# ============================================================================
# Implicit Multiplication Parsing Benchmarks
# ============================================================================

def bench_parse_2x():
    """Benchmark parsing implicit multiplication: 2x."""
    result = Expression.parse("2x")
    return result


def bench_parse_2_paren_x_plus_1():
    """Benchmark parsing implicit multiplication: 2(x+1)."""
    result = Expression.parse("2(x+1)")
    return result


def bench_parse_paren_a_paren_b():
    """Benchmark parsing implicit multiplication: (a)(b)."""
    result = Expression.parse("(a)(b)")
    return result


def bench_parse_sin_x_cos_x():
    """Benchmark parsing implicit multiplication: sin(x)cos(x)."""
    result = Expression.parse("sin(x)cos(x)")
    return result


# ============================================================================
# Formatting Benchmarks
# ============================================================================

def bench_format_simple():
    """Benchmark formatting simple expression: x + 1."""
    x = Symbol("x")
    expr = Expression.add([Expression.symbol(x), Expression.integer(1)])
    result = expr.to_string()
    return result


def bench_format_polynomial():
    """Benchmark formatting polynomial: x^3 + 2*x^2 + x."""
    x = Symbol("x")
    expr = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(3)),
        Expression.multiply([
            Expression.integer(2),
            Expression.pow(Expression.symbol(x), Expression.integer(2))
        ]),
        Expression.symbol(x)
    ])
    result = expr.to_string()
    return result


def bench_format_nested_functions():
    """Benchmark formatting nested functions: sin(cos(x))."""
    x = Symbol("x")
    expr = Expression.function("sin", [
        Expression.function("cos", [Expression.symbol(x)])
    ])
    result = expr.to_string()
    return result


def bench_format_latex_simple():
    """Benchmark LaTeX formatting simple expression: x + 1."""
    x = Symbol("x")
    expr = Expression.add([Expression.symbol(x), Expression.integer(1)])
    result = expr.to_latex()
    return result


def bench_format_latex_polynomial():
    """Benchmark LaTeX formatting polynomial: x^3 + 2*x^2 + x."""
    x = Symbol("x")
    expr = Expression.add([
        Expression.pow(Expression.symbol(x), Expression.integer(3)),
        Expression.multiply([
            Expression.integer(2),
            Expression.pow(Expression.symbol(x), Expression.integer(2))
        ]),
        Expression.symbol(x)
    ])
    result = expr.to_latex()
    return result


def bench_format_latex_nested():
    """Benchmark LaTeX formatting nested functions: sin(cos(x))."""
    x = Symbol("x")
    expr = Expression.function("sin", [
        Expression.function("cos", [Expression.symbol(x)])
    ])
    result = expr.to_latex()
    return result


# ============================================================================
# Parsing Throughput Benchmarks
# ============================================================================

def bench_parse_sum_10_terms():
    """Benchmark parsing sum with 10 terms."""
    expr_str = " + ".join([f"x{i}" for i in range(10)])
    result = Expression.parse(expr_str)
    return result


def bench_parse_sum_20_terms():
    """Benchmark parsing sum with 20 terms."""
    expr_str = " + ".join([f"x{i}" for i in range(20)])
    result = Expression.parse(expr_str)
    return result


def bench_parse_sum_50_terms():
    """Benchmark parsing sum with 50 terms."""
    expr_str = " + ".join([f"x{i}" for i in range(50)])
    result = Expression.parse(expr_str)
    return result


# ============================================================================
# Benchmark Runner
# ============================================================================

def run_all_benchmarks(samples: int = 100) -> Dict[str, BenchmarkResult]:
    """Run all parsing benchmarks."""
    results = {}

    benchmarks = [
        # Simple parsing
        bench_parse_variable,
        bench_parse_number,
        bench_parse_addition,
        bench_parse_multiplication,
        bench_parse_power,
        bench_parse_sin,

        # Complex parsing
        bench_parse_polynomial,
        bench_parse_nested_functions,
        bench_parse_complex_fraction,
        bench_parse_trig_identity,

        # Implicit multiplication
        bench_parse_2x,
        bench_parse_2_paren_x_plus_1,
        bench_parse_paren_a_paren_b,
        bench_parse_sin_x_cos_x,

        # Formatting
        bench_format_simple,
        bench_format_polynomial,
        bench_format_nested_functions,
        bench_format_latex_simple,
        bench_format_latex_polynomial,
        bench_format_latex_nested,

        # Throughput
        bench_parse_sum_10_terms,
        bench_parse_sum_20_terms,
        bench_parse_sum_50_terms,
    ]

    print("=" * 80)
    print("Parsing Benchmarks")
    print("=" * 80)

    for bench_func in benchmarks:
        print(f"Running {bench_func.__name__}...", end=" ")
        result = benchmark(bench_func, samples=samples)
        results[bench_func.__name__] = result
        print(f"{result.mean_ns:.2f}ns")

    print("=" * 80)

    return results


def main():
    """Main entry point for parsing benchmarks."""
    results = run_all_benchmarks(samples=100)

    print("\nDetailed Results:")
    print("-" * 80)
    for name, result in results.items():
        print(result)


if __name__ == "__main__":
    main()
