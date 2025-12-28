"""
Core Performance Benchmarks

Mirrors: benches/core_performance.rs
Tests: Expression creation, simplification, basic solving, polynomial operations

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
                f"(median: {self.median_ns:.2f}ns, range: [{self.min_ns:.2f}, {self.max_ns:.2f}])")


def benchmark(func, samples: int = 100, warmup: int = 10) -> BenchmarkResult:
    """
    Benchmark a function with warmup and multiple samples.

    Args:
        func: Function to benchmark (no arguments)
        samples: Number of timing samples to collect
        warmup: Number of warmup iterations

    Returns:
        BenchmarkResult with timing statistics
    """
    # Warmup
    for _ in range(warmup):
        func()

    # Collect samples
    times_ns = []
    for _ in range(samples):
        start = time.perf_counter_ns()
        func()
        end = time.perf_counter_ns()
        times_ns.append(end - start)

    return BenchmarkResult(func.__name__, times_ns)


# ============================================================================
# Arithmetic Operations Benchmarks
# ============================================================================

def bench_expression_creation_direct():
    """Benchmark expression creation (direct API, no parsing)."""
    x = symbol('x')
    expr = x + 42
    return expr


def bench_expression_creation_with_parsing():
    """Benchmark expression creation (with parsing)."""
    expr = parse("x + 42")
    return expr


def bench_simplification_direct():
    """Benchmark simplification (direct API, no parsing)."""
    x = symbol('x')
    expr = x + x + x
    result = expr.simplify()
    return result


def bench_simplification_with_parsing():
    """Benchmark simplification (with parsing)."""
    expr = parse("2 + 3 + 5")
    result = expr.simplify()
    return result


# ============================================================================
# Solver Operations Benchmarks
# ============================================================================

def bench_basic_solving_direct():
    """Benchmark basic equation solving (direct API, no parsing)."""
    x = symbol('x')
    equation = x - 42  # x = 42
    solutions = solve(equation, 'x')
    return solutions


def bench_basic_solving_with_parsing():
    """Benchmark basic equation solving (with parsing)."""
    solutions = solve(parse("x - 42"), 'x')
    return solutions


# ============================================================================
# Polynomial Operations Benchmarks
# ============================================================================

def bench_polynomial_creation_direct():
    """Benchmark polynomial creation (direct API, no parsing)."""
    x = symbol('x')
    # Create polynomial: x^10 + 2x^9 + ... + 10x + 11
    poly = (x**10 + 2*x**9 + 3*x**8 + 4*x**7 + 5*x**6 +
            6*x**5 + 7*x**4 + 8*x**3 + 9*x**2 + 10*x + 11)
    return poly


def bench_polynomial_creation_with_parsing():
    """Benchmark polynomial creation (with parsing)."""
    poly = parse("x^10 + 2*x^9 + 3*x^8 + 4*x^7 + 5*x^6 + 6*x^5 + 7*x^4 + 8*x^3 + 9*x^2 + 10*x + 11")
    return poly


def bench_polynomial_simplification_direct():
    """Benchmark polynomial simplification (direct API, no parsing)."""
    x = symbol('x')
    poly = x**2 - 5*x + 6
    result = poly.simplify()
    return result


def bench_polynomial_simplification_with_parsing():
    """Benchmark polynomial simplification (with parsing)."""
    poly = parse("x^2 - 5*x + 6")
    result = poly.simplify()
    return result


# ============================================================================
# Memory Efficiency Benchmarks
# ============================================================================

def bench_expression_size_verification():
    """Benchmark expression size verification."""
    # Python objects don't have direct size like Rust's 32-byte constraint,
    # but we can benchmark object creation overhead
    x = symbol('x')
    expr = x ** 2
    return expr


# ============================================================================
# Benchmark Runner
# ============================================================================

def run_all_benchmarks(samples: int = 100) -> Dict[str, BenchmarkResult]:
    """
    Run all core performance benchmarks.

    Args:
        samples: Number of samples per benchmark

    Returns:
        Dictionary mapping benchmark name to BenchmarkResult
    """
    results = {}

    benchmarks = [
        # Arithmetic operations
        bench_expression_creation_direct,
        bench_expression_creation_with_parsing,
        bench_simplification_direct,
        bench_simplification_with_parsing,

        # Solver operations
        bench_basic_solving_direct,
        bench_basic_solving_with_parsing,

        # Polynomial operations
        bench_polynomial_creation_direct,
        bench_polynomial_creation_with_parsing,
        bench_polynomial_simplification_direct,
        bench_polynomial_simplification_with_parsing,

        # Memory efficiency
        bench_expression_size_verification,
    ]

    print("=" * 80)
    print("Core Performance Benchmarks")
    print("=" * 80)

    for bench_func in benchmarks:
        print(f"Running {bench_func.__name__}...", end=" ")
        result = benchmark(bench_func, samples=samples)
        results[bench_func.__name__] = result
        print(f"{result.mean_ns:.2f}ns")

    print("=" * 80)

    return results


def main():
    """Main entry point for core performance benchmarks."""
    results = run_all_benchmarks(samples=100)

    print("\nDetailed Results:")
    print("-" * 80)
    for name, result in results.items():
        print(result)

    # Calculate parsing overhead
    print("\nParsing Overhead Analysis:")
    print("-" * 80)

    overhead_pairs = [
        ("expression_creation", "bench_expression_creation_direct", "bench_expression_creation_with_parsing"),
        ("simplification", "bench_simplification_direct", "bench_simplification_with_parsing"),
        ("solving", "bench_basic_solving_direct", "bench_basic_solving_with_parsing"),
        ("polynomial_creation", "bench_polynomial_creation_direct", "bench_polynomial_creation_with_parsing"),
        ("polynomial_simplification", "bench_polynomial_simplification_direct", "bench_polynomial_simplification_with_parsing"),
    ]

    for label, direct, with_parsing in overhead_pairs:
        if direct in results and with_parsing in results:
            direct_time = results[direct].mean_ns
            parsing_time = results[with_parsing].mean_ns
            overhead_ns = parsing_time - direct_time
            overhead_pct = (overhead_ns / direct_time) * 100 if direct_time > 0 else 0
            print(f"{label:30s}: {overhead_ns:10.2f}ns ({overhead_pct:6.2f}%)")


if __name__ == "__main__":
    main()
