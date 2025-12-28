#!/usr/bin/env python3
"""
MathHook Python Benchmark Suite

Comprehensive polynomial operation benchmarks using MathHook Python bindings.

Usage:
    python bench_mathhook.py [--json] [--iterations N]

Output: JSON for baseline comparison or human-readable report.

Last Updated: 2025-12-28T1200
"""

import json
import sys
import time
import statistics
import argparse
from typing import Dict, Any, Callable

try:
    from mathhook import parse, symbol, symbols, gcd
except ImportError:
    print("Error: MathHook Python bindings not installed", file=sys.stderr)
    print("Install with: pip install mathhook", file=sys.stderr)
    sys.exit(1)


def benchmark(func: Callable, iterations: int = 100, warmup: int = 10) -> Dict[str, float]:
    """Run benchmark with statistical analysis."""
    # Warmup
    for _ in range(warmup):
        func()

    # Collect timing samples
    times = []
    for _ in range(iterations):
        start = time.perf_counter_ns()
        func()
        end = time.perf_counter_ns()
        times.append(end - start)

    # Statistics in nanoseconds
    mean_ns = statistics.mean(times)
    stdev_ns = statistics.stdev(times) if len(times) > 1 else 0
    median_ns = statistics.median(times)
    min_ns = min(times)
    max_ns = max(times)

    return {
        'mean_ns': mean_ns,
        'stdev_ns': stdev_ns,
        'median_ns': median_ns,
        'min_ns': min_ns,
        'max_ns': max_ns,
        'iterations': iterations
    }


# GCD expressions (pre-parsed for fair benchmarking)
_gcd_simple_f = parse("x^2 - 1")
_gcd_simple_g = parse("x - 1")

_gcd_medium_f = parse("x^5 + x^4 + x^3 + x^2 + x + 1")
_gcd_medium_g = parse("x^3 - 1")

_gcd_large_f = parse("x^20 - 1")
_gcd_large_g = parse("x^10 - 1")

_gcd_content_f = parse("6*x^3 + 12*x^2 + 18*x")
_gcd_content_g = parse("9*x^2 + 18*x + 27")

_gcd_bivariate_f = parse("x*y + x")
_gcd_bivariate_g = parse("x*y")

_gcd_trivariate_f = parse("x*y*z")
_gcd_trivariate_g = parse("x*y")

# Multiplication expressions
_mul_simple_f = parse("x + 1")
_mul_simple_g = parse("x + 1")

_mul_medium_f = parse("x^10 + x^9 + x^8 + x^7 + x^6 + x^5 + x^4 + x^3 + x^2 + x + 1")
_mul_medium_g = parse("2*x^10 + 2*x^9 + 2*x^8 + 2*x^7 + 2*x^6 + 2*x^5 + 2*x^4 + 2*x^3 + 2*x^2 + 2*x + 2")

_mul_large_terms = " + ".join([f"x^{i}" for i in range(51)])
_mul_large_f = parse(_mul_large_terms)
_mul_large_g = parse(_mul_large_terms)

_mul_sparse_f = parse("1 + x^50 + x^100")
_mul_sparse_g = parse("1 + x^25 + x^75")

# Division expressions
_div_simple_f = parse("x^2 - 1")
_div_simple_g = parse("x - 1")

_div_medium_f = parse("x^10 + x^5 + x^2 + 1")
_div_medium_g = parse("x^3 + x + 1")

# Expansion expressions
_expand_simple = parse("(x + 1)^2")
_expand_medium = parse("(x + 1)^5")
_expand_large = parse("(x + 1)^10")

# Simplification expressions
_simplify_simple = parse("x + x + x")
_simplify_polynomial = parse("x^2 + 2*x + 1 + x^2 - 1")
_simplify_large = parse("(x+1)^2 - (x^2 + 2*x + 1)")

# Factorization expressions
_factor_simple = parse("x^2 - 1")
_factor_quadratic = parse("x^2 + 2*x + 1")


# ============================================================================
# PARSING BENCHMARKS (measure parsing time - this is expected to include overhead)
# ============================================================================

def bench_parse_simple():
    """Parse simple polynomial from string."""
    return parse("x^2 + 2*x + 1")

def bench_parse_medium():
    """Parse medium complexity polynomial."""
    return parse("x^5 + 3*x^4 - 2*x^3 + 7*x^2 - x + 5")

def bench_parse_large():
    """Parse large polynomial (degree 20)."""
    terms = " + ".join([f"{i+1}*x^{i}" for i in range(21)])
    return parse(terms)

def bench_parse_multivariate():
    """Parse multivariate polynomial."""
    return parse("x^2*y + x*y^2 + x*y + x + y + 1")


# ============================================================================
# GCD BENCHMARKS (using pre-parsed expressions for fair comparison)
# ============================================================================

def bench_gcd_simple():
    """GCD of simple polynomials: (x^2-1, x-1) -> x-1"""
    return gcd(_gcd_simple_f, _gcd_simple_g)

def bench_gcd_medium():
    """GCD of degree-5 polynomials."""
    return gcd(_gcd_medium_f, _gcd_medium_g)

def bench_gcd_large():
    """GCD of degree-20 polynomials."""
    return gcd(_gcd_large_f, _gcd_large_g)

def bench_gcd_with_content():
    """GCD with common content factor."""
    return gcd(_gcd_content_f, _gcd_content_g)

def bench_gcd_bivariate():
    """Bivariate GCD."""
    return gcd(_gcd_bivariate_f, _gcd_bivariate_g)

def bench_gcd_trivariate():
    """Trivariate GCD."""
    return gcd(_gcd_trivariate_f, _gcd_trivariate_g)


# ============================================================================
# POLYNOMIAL MULTIPLICATION BENCHMARKS (using pre-parsed expressions)
# ============================================================================

def bench_mul_simple():
    """Multiply (x+1)(x+1)."""
    return _mul_simple_f * _mul_simple_g

def bench_mul_medium():
    """Multiply degree-10 polynomials."""
    return _mul_medium_f * _mul_medium_g

def bench_mul_large():
    """Multiply degree-50 polynomials."""
    return _mul_large_f * _mul_large_g

def bench_mul_sparse():
    """Multiply sparse polynomials."""
    return _mul_sparse_f * _mul_sparse_g


# ============================================================================
# POLYNOMIAL DIVISION BENCHMARKS (using pre-parsed expressions)
# ============================================================================

def bench_div_simple():
    """Divide (x^2-1) by (x-1)."""
    return _div_simple_f / _div_simple_g

def bench_div_medium():
    """Divide degree-10 by degree-3."""
    return _div_medium_f / _div_medium_g


# ============================================================================
# EXPANSION BENCHMARKS (using pre-parsed expressions)
# ============================================================================

def bench_expand_simple():
    """Expand (x+1)^2."""
    return _expand_simple.expand()

def bench_expand_medium():
    """Expand (x+1)^5."""
    return _expand_medium.expand()

def bench_expand_large():
    """Expand (x+1)^10."""
    return _expand_large.expand()


# ============================================================================
# SIMPLIFICATION BENCHMARKS (using pre-parsed expressions)
# ============================================================================

def bench_simplify_simple():
    """Simplify x + x + x."""
    return _simplify_simple.simplify()

def bench_simplify_polynomial():
    """Simplify polynomial expression."""
    return _simplify_polynomial.simplify()

def bench_simplify_large():
    """Simplify (x+1)^2 - (x^2 + 2*x + 1)."""
    return _simplify_large.simplify()


# ============================================================================
# FACTORIZATION BENCHMARKS (using pre-parsed expressions)
# ============================================================================

def bench_factor_simple():
    """Factor x^2 - 1."""
    return _factor_simple.factor()

def bench_factor_quadratic():
    """Factor x^2 + 2x + 1."""
    return _factor_quadratic.factor()


# ============================================================================
# MAIN BENCHMARK RUNNER
# ============================================================================

def run_all_benchmarks(iterations: int = 100) -> Dict[str, Any]:
    """Run all benchmarks and return results."""

    benchmarks = {
        # Parsing (with string processing overhead - this is expected)
        'parse_simple': bench_parse_simple,
        'parse_medium': bench_parse_medium,
        'parse_large': bench_parse_large,
        'parse_multivariate': bench_parse_multivariate,

        # GCD (using pre-parsed expressions for fair comparison)
        'gcd_simple': bench_gcd_simple,
        'gcd_medium': bench_gcd_medium,
        'gcd_large': bench_gcd_large,
        'gcd_with_content': bench_gcd_with_content,
        'gcd_bivariate': bench_gcd_bivariate,
        'gcd_trivariate': bench_gcd_trivariate,

        # Multiplication (using pre-parsed expressions)
        'mul_simple': bench_mul_simple,
        'mul_medium': bench_mul_medium,
        'mul_large': bench_mul_large,
        'mul_sparse': bench_mul_sparse,

        # Division (using pre-parsed expressions)
        'div_simple': bench_div_simple,
        'div_medium': bench_div_medium,

        # Expansion (using pre-parsed expressions)
        'expand_simple': bench_expand_simple,
        'expand_medium': bench_expand_medium,
        'expand_large': bench_expand_large,

        # Simplification (using pre-parsed expressions)
        'simplify_simple': bench_simplify_simple,
        'simplify_polynomial': bench_simplify_polynomial,
        'simplify_large': bench_simplify_large,

        # Factorization (using pre-parsed expressions)
        'factor_simple': bench_factor_simple,
        'factor_quadratic': bench_factor_quadratic,
    }

    results = {
        'platform': 'python-mathhook',
        'binding': 'PyO3',
        'python_version': sys.version.split()[0],
        'benchmarks': {}
    }

    for name, func in benchmarks.items():
        try:
            print(f"Running {name}...", file=sys.stderr)
            result = benchmark(func, iterations=iterations)
            results['benchmarks'][name] = result
        except Exception as e:
            print(f"Error in {name}: {e}", file=sys.stderr)
            results['benchmarks'][name] = {'error': str(e)}

    return results


def print_human_readable(results: Dict[str, Any]):
    """Print results in human-readable format."""
    print("=" * 80)
    print("MathHook Python Benchmark Results")
    print("=" * 80)
    print(f"Binding: {results['binding']}")
    print(f"Python version: {results['python_version']}")

    categories = {
        'Parsing': ['parse_'],
        'GCD': ['gcd_'],
        'Multiplication': ['mul_'],
        'Division': ['div_'],
        'Expansion': ['expand_'],
        'Simplification': ['simplify_'],
        'Factorization': ['factor_'],
    }

    for category, prefixes in categories.items():
        print(f"\n{category}:")
        print("-" * 60)

        for name, data in results['benchmarks'].items():
            if any(name.startswith(p) for p in prefixes):
                if 'error' in data:
                    print(f"  {name:30s} ERROR: {data['error']}")
                else:
                    mean_us = data['mean_ns'] / 1000
                    stdev_us = data['stdev_ns'] / 1000
                    print(f"  {name:30s} {mean_us:12.2f} us  (stdev: {stdev_us:8.2f} us)")


def main():
    parser = argparse.ArgumentParser(description='MathHook Python Benchmarks')
    parser.add_argument('--json', action='store_true', help='Output JSON format')
    parser.add_argument('--iterations', type=int, default=100, help='Benchmark iterations')
    args = parser.parse_args()

    results = run_all_benchmarks(iterations=args.iterations)

    if args.json:
        print(json.dumps(results, indent=2))
    else:
        print_human_readable(results)


if __name__ == '__main__':
    main()
