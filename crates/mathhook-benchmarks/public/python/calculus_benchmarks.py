"""
Calculus Benchmarks

Mirrors: benches/calculus_benchmarks.rs
Tests: Derivatives (power, product, chain, quotient rules), Integrals, Multi-variable

Last Updated: 2025-12-28T1200
"""

import time
import statistics
from typing import Dict, List

try:
    from mathhook import symbol, symbols, parse, sin, cos, exp, log
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
# Derivative Benchmarks
# ============================================================================

def bench_derivative_power_rule_direct():
    """Benchmark power rule derivative (direct API): d/dx(x^5)."""
    x = symbol('x')
    expr = x ** 5
    result = expr.derivative('x')
    return result


def bench_derivative_power_rule_with_parsing():
    """Benchmark power rule derivative (with parsing): d/dx(x^5)."""
    x = symbol('x')
    expr = parse("x^5")
    result = expr.derivative('x')
    return result


def bench_derivative_product_rule_direct():
    """Benchmark product rule derivative (direct API): d/dx(x^2 * sin(x))."""
    x = symbol('x')
    expr = x**2 * sin(x)
    result = expr.derivative('x')
    return result


def bench_derivative_product_rule_with_parsing():
    """Benchmark product rule derivative (with parsing): d/dx(x^2 * sin(x))."""
    x = symbol('x')
    expr = parse("x^2 * sin(x)")
    result = expr.derivative('x')
    return result


def bench_derivative_chain_rule_direct():
    """Benchmark chain rule derivative (direct API): d/dx(sin(x^2))."""
    x = symbol('x')
    expr = sin(x ** 2)
    result = expr.derivative('x')
    return result


def bench_derivative_chain_rule_with_parsing():
    """Benchmark chain rule derivative (with parsing): d/dx(sin(x^2))."""
    x = symbol('x')
    expr = parse("sin(x^2)")
    result = expr.derivative('x')
    return result


def bench_derivative_quotient_rule_direct():
    """Benchmark quotient rule derivative (direct API): d/dx((x^2+1)/(x-1))."""
    x = symbol('x')
    expr = (x**2 + 1) / (x - 1)
    result = expr.derivative('x')
    return result


def bench_derivative_quotient_rule_with_parsing():
    """Benchmark quotient rule derivative (with parsing): d/dx((x^2+1)/(x-1))."""
    x = symbol('x')
    expr = parse("(x^2 + 1) / (x - 1)")
    result = expr.derivative('x')
    return result


def bench_derivative_trigonometric_direct():
    """Benchmark trigonometric derivative (direct API): d/dx(sin(x) + cos(x))."""
    x = symbol('x')
    expr = sin(x) + cos(x)
    result = expr.derivative('x')
    return result


def bench_derivative_trigonometric_with_parsing():
    """Benchmark trigonometric derivative (with parsing): d/dx(sin(x) + cos(x))."""
    x = symbol('x')
    expr = parse("sin(x) + cos(x)")
    result = expr.derivative('x')
    return result


def bench_derivative_exponential_direct():
    """Benchmark exponential derivative (direct API): d/dx(exp(2x))."""
    x = symbol('x')
    expr = exp(2 * x)
    result = expr.derivative('x')
    return result


def bench_derivative_exponential_with_parsing():
    """Benchmark exponential derivative (with parsing): d/dx(exp(2x))."""
    x = symbol('x')
    expr = parse("exp(2*x)")
    result = expr.derivative('x')
    return result


def bench_derivative_logarithmic_direct():
    """Benchmark logarithmic derivative (direct API): d/dx(log(x^2))."""
    x = symbol('x')
    expr = log(x ** 2)
    result = expr.derivative('x')
    return result


def bench_derivative_logarithmic_with_parsing():
    """Benchmark logarithmic derivative (with parsing): d/dx(log(x^2))."""
    x = symbol('x')
    expr = parse("log(x^2)")
    result = expr.derivative('x')
    return result


# ============================================================================
# Integration Benchmarks
# ============================================================================

def bench_integral_power_rule_direct():
    """Benchmark power rule integration (direct API): ∫x^5 dx."""
    x = symbol('x')
    expr = x ** 5
    result = expr.integrate('x')
    return result


def bench_integral_power_rule_with_parsing():
    """Benchmark power rule integration (with parsing): ∫x^5 dx."""
    x = symbol('x')
    expr = parse("x^5")
    result = expr.integrate('x')
    return result


def bench_integral_trigonometric_sin_direct():
    """Benchmark trigonometric integration (direct API): ∫sin(x) dx."""
    x = symbol('x')
    expr = sin(x)
    result = expr.integrate('x')
    return result


def bench_integral_trigonometric_sin_with_parsing():
    """Benchmark trigonometric integration (with parsing): ∫sin(x) dx."""
    x = symbol('x')
    expr = parse("sin(x)")
    result = expr.integrate('x')
    return result


def bench_integral_exponential_direct():
    """Benchmark exponential integration (direct API): ∫exp(x) dx."""
    x = symbol('x')
    expr = exp(x)
    result = expr.integrate('x')
    return result


def bench_integral_exponential_with_parsing():
    """Benchmark exponential integration (with parsing): ∫exp(x) dx."""
    x = symbol('x')
    expr = parse("exp(x)")
    result = expr.integrate('x')
    return result


# ============================================================================
# Multi-variable Derivatives
# ============================================================================

def bench_partial_derivative_x_direct():
    """Benchmark partial derivative (direct API): ∂/∂x(x^2 + y^2)."""
    x, y = symbols('x y')
    expr = x**2 + y**2
    result = expr.derivative('x')
    return result


def bench_partial_derivative_x_with_parsing():
    """Benchmark partial derivative (with parsing): ∂/∂x(x^2 + y^2)."""
    x = symbol('x')
    expr = parse("x^2 + y^2")
    result = expr.derivative('x')
    return result


def bench_partial_derivative_y_direct():
    """Benchmark partial derivative (direct API): ∂/∂y(x^2 + y^2)."""
    x, y = symbols('x y')
    expr = x**2 + y**2
    result = expr.derivative('y')
    return result


def bench_partial_derivative_y_with_parsing():
    """Benchmark partial derivative (with parsing): ∂/∂y(x^2 + y^2)."""
    y = symbol('y')
    expr = parse("x^2 + y^2")
    result = expr.derivative('y')
    return result


# ============================================================================
# Benchmark Runner
# ============================================================================

def run_all_benchmarks(samples: int = 100) -> Dict[str, BenchmarkResult]:
    """Run all calculus benchmarks."""
    results = {}

    benchmarks = [
        # Derivatives
        bench_derivative_power_rule_direct,
        bench_derivative_power_rule_with_parsing,
        bench_derivative_product_rule_direct,
        bench_derivative_product_rule_with_parsing,
        bench_derivative_chain_rule_direct,
        bench_derivative_chain_rule_with_parsing,
        bench_derivative_quotient_rule_direct,
        bench_derivative_quotient_rule_with_parsing,
        bench_derivative_trigonometric_direct,
        bench_derivative_trigonometric_with_parsing,
        bench_derivative_exponential_direct,
        bench_derivative_exponential_with_parsing,
        bench_derivative_logarithmic_direct,
        bench_derivative_logarithmic_with_parsing,

        # Integrals
        bench_integral_power_rule_direct,
        bench_integral_power_rule_with_parsing,
        bench_integral_trigonometric_sin_direct,
        bench_integral_trigonometric_sin_with_parsing,
        bench_integral_exponential_direct,
        bench_integral_exponential_with_parsing,

        # Multi-variable
        bench_partial_derivative_x_direct,
        bench_partial_derivative_x_with_parsing,
        bench_partial_derivative_y_direct,
        bench_partial_derivative_y_with_parsing,
    ]

    print("=" * 80)
    print("Calculus Benchmarks")
    print("=" * 80)

    for bench_func in benchmarks:
        print(f"Running {bench_func.__name__}...", end=" ")
        result = benchmark(bench_func, samples=samples)
        results[bench_func.__name__] = result
        print(f"{result.mean_ns:.2f}ns")

    print("=" * 80)

    return results


def main():
    """Main entry point for calculus benchmarks."""
    results = run_all_benchmarks(samples=100)

    print("\nDetailed Results:")
    print("-" * 80)
    for name, result in results.items():
        print(result)


if __name__ == "__main__":
    main()
