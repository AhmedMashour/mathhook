"""
Function Evaluation Benchmarks

Mirrors: benches/function_evaluation_benchmarks.rs
Tests: Elementary functions (trig, exp, log), special functions, composition
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
# Elementary Trigonometric Function Benchmarks
# ============================================================================

def bench_sin_symbolic_direct():
    """Benchmark sin symbolic (direct API): sin(x)."""
    x = Symbol("x")
    expr = Expression.function("sin", [Expression.symbol(x)])
    result = expr.simplify()
    return result


def bench_sin_symbolic_with_parsing():
    """Benchmark sin symbolic (with parsing): sin(x)."""
    expr = Expression.parse("sin(x)")
    result = expr.simplify()
    return result


def bench_cos_symbolic_direct():
    """Benchmark cos symbolic (direct API): cos(x)."""
    x = Symbol("x")
    expr = Expression.function("cos", [Expression.symbol(x)])
    result = expr.simplify()
    return result


def bench_cos_symbolic_with_parsing():
    """Benchmark cos symbolic (with parsing): cos(x)."""
    expr = Expression.parse("cos(x)")
    result = expr.simplify()
    return result


def bench_tan_symbolic_direct():
    """Benchmark tan symbolic (direct API): tan(x)."""
    x = Symbol("x")
    expr = Expression.function("tan", [Expression.symbol(x)])
    result = expr.simplify()
    return result


def bench_tan_symbolic_with_parsing():
    """Benchmark tan symbolic (with parsing): tan(x)."""
    expr = Expression.parse("tan(x)")
    result = expr.simplify()
    return result


def bench_nested_trig_direct():
    """Benchmark nested trig (direct API): sin(cos(x))."""
    x = Symbol("x")
    expr = Expression.function("sin", [
        Expression.function("cos", [Expression.symbol(x)])
    ])
    result = expr.simplify()
    return result


def bench_nested_trig_with_parsing():
    """Benchmark nested trig (with parsing): sin(cos(x))."""
    expr = Expression.parse("sin(cos(x))")
    result = expr.simplify()
    return result


def bench_arcsin_symbolic_direct():
    """Benchmark arcsin symbolic (direct API): arcsin(x)."""
    x = Symbol("x")
    expr = Expression.function("arcsin", [Expression.symbol(x)])
    result = expr.simplify()
    return result


def bench_arcsin_symbolic_with_parsing():
    """Benchmark arcsin symbolic (with parsing): arcsin(x)."""
    expr = Expression.parse("arcsin(x)")
    result = expr.simplify()
    return result


# ============================================================================
# Hyperbolic Function Benchmarks
# ============================================================================

def bench_sinh_symbolic_direct():
    """Benchmark sinh symbolic (direct API): sinh(x)."""
    x = Symbol("x")
    expr = Expression.function("sinh", [Expression.symbol(x)])
    result = expr.simplify()
    return result


def bench_sinh_symbolic_with_parsing():
    """Benchmark sinh symbolic (with parsing): sinh(x)."""
    expr = Expression.parse("sinh(x)")
    result = expr.simplify()
    return result


def bench_cosh_symbolic_direct():
    """Benchmark cosh symbolic (direct API): cosh(x)."""
    x = Symbol("x")
    expr = Expression.function("cosh", [Expression.symbol(x)])
    result = expr.simplify()
    return result


def bench_cosh_symbolic_with_parsing():
    """Benchmark cosh symbolic (with parsing): cosh(x)."""
    expr = Expression.parse("cosh(x)")
    result = expr.simplify()
    return result


def bench_tanh_symbolic_direct():
    """Benchmark tanh symbolic (direct API): tanh(x)."""
    x = Symbol("x")
    expr = Expression.function("tanh", [Expression.symbol(x)])
    result = expr.simplify()
    return result


def bench_tanh_symbolic_with_parsing():
    """Benchmark tanh symbolic (with parsing): tanh(x)."""
    expr = Expression.parse("tanh(x)")
    result = expr.simplify()
    return result


# ============================================================================
# Exponential and Logarithmic Function Benchmarks
# ============================================================================

def bench_exp_symbolic_direct():
    """Benchmark exp symbolic (direct API): exp(x)."""
    x = Symbol("x")
    expr = Expression.function("exp", [Expression.symbol(x)])
    result = expr.simplify()
    return result


def bench_exp_symbolic_with_parsing():
    """Benchmark exp symbolic (with parsing): exp(x)."""
    expr = Expression.parse("exp(x)")
    result = expr.simplify()
    return result


def bench_log_symbolic_direct():
    """Benchmark log symbolic (direct API): log(x)."""
    x = Symbol("x")
    expr = Expression.function("log", [Expression.symbol(x)])
    result = expr.simplify()
    return result


def bench_log_symbolic_with_parsing():
    """Benchmark log symbolic (with parsing): log(x)."""
    expr = Expression.parse("log(x)")
    result = expr.simplify()
    return result


def bench_log_base_10_direct():
    """Benchmark log base 10 (direct API): log(x, 10)."""
    x = Symbol("x")
    expr = Expression.function("log", [Expression.symbol(x), Expression.integer(10)])
    result = expr.simplify()
    return result


def bench_log_base_10_with_parsing():
    """Benchmark log base 10 (with parsing): log(x, 10)."""
    expr = Expression.parse("log(x, 10)")
    result = expr.simplify()
    return result


def bench_exp_log_identity_direct():
    """Benchmark exp(log(x)) identity (direct API)."""
    x = Symbol("x")
    expr = Expression.function("exp", [
        Expression.function("log", [Expression.symbol(x)])
    ])
    result = expr.simplify()
    return result


def bench_exp_log_identity_with_parsing():
    """Benchmark exp(log(x)) identity (with parsing)."""
    expr = Expression.parse("exp(log(x))")
    result = expr.simplify()
    return result


def bench_nested_exp_direct():
    """Benchmark nested exp (direct API): exp(exp(x))."""
    x = Symbol("x")
    expr = Expression.function("exp", [
        Expression.function("exp", [Expression.symbol(x)])
    ])
    result = expr.simplify()
    return result


def bench_nested_exp_with_parsing():
    """Benchmark nested exp (with parsing): exp(exp(x))."""
    expr = Expression.parse("exp(exp(x))")
    result = expr.simplify()
    return result


# ============================================================================
# Power and Root Function Benchmarks
# ============================================================================

def bench_sqrt_symbolic_direct():
    """Benchmark sqrt symbolic (direct API): sqrt(x)."""
    x = Symbol("x")
    expr = Expression.function("sqrt", [Expression.symbol(x)])
    result = expr.simplify()
    return result


def bench_sqrt_symbolic_with_parsing():
    """Benchmark sqrt symbolic (with parsing): sqrt(x)."""
    expr = Expression.parse("sqrt(x)")
    result = expr.simplify()
    return result


def bench_sqrt_square_direct():
    """Benchmark sqrt(x^2) simplification (direct API)."""
    x = Symbol("x")
    expr = Expression.function("sqrt", [
        Expression.pow(Expression.symbol(x), Expression.integer(2))
    ])
    result = expr.simplify()
    return result


def bench_sqrt_square_with_parsing():
    """Benchmark sqrt(x^2) simplification (with parsing)."""
    expr = Expression.parse("sqrt(x^2)")
    result = expr.simplify()
    return result


# ============================================================================
# Absolute Value and Sign Function Benchmarks
# ============================================================================

def bench_abs_symbolic_direct():
    """Benchmark abs symbolic (direct API): abs(x)."""
    x = Symbol("x")
    expr = Expression.function("abs", [Expression.symbol(x)])
    result = expr.simplify()
    return result


def bench_abs_symbolic_with_parsing():
    """Benchmark abs symbolic (with parsing): abs(x)."""
    expr = Expression.parse("abs(x)")
    result = expr.simplify()
    return result


def bench_nested_abs_direct():
    """Benchmark nested abs (direct API): abs(abs(x))."""
    x = Symbol("x")
    expr = Expression.function("abs", [
        Expression.function("abs", [Expression.symbol(x)])
    ])
    result = expr.simplify()
    return result


def bench_nested_abs_with_parsing():
    """Benchmark nested abs (with parsing): abs(abs(x))."""
    expr = Expression.parse("abs(abs(x))")
    result = expr.simplify()
    return result


# ============================================================================
# Factorial and Combinatorial Function Benchmarks
# ============================================================================

def bench_factorial_small_direct():
    """Benchmark factorial of small number (direct API): factorial(5)."""
    expr = Expression.function("factorial", [Expression.integer(5)])
    result = expr.simplify()
    return result


def bench_factorial_small_with_parsing():
    """Benchmark factorial of small number (with parsing): factorial(5)."""
    expr = Expression.parse("factorial(5)")
    result = expr.simplify()
    return result


def bench_factorial_symbolic_direct():
    """Benchmark factorial symbolic (direct API): factorial(n)."""
    n = Symbol("n")
    expr = Expression.function("factorial", [Expression.symbol(n)])
    result = expr.simplify()
    return result


def bench_factorial_symbolic_with_parsing():
    """Benchmark factorial symbolic (with parsing): factorial(n)."""
    expr = Expression.parse("factorial(n)")
    result = expr.simplify()
    return result


def bench_binomial_coefficient_direct():
    """Benchmark binomial coefficient (direct API): binomial(10, 5)."""
    expr = Expression.function("binomial", [Expression.integer(10), Expression.integer(5)])
    result = expr.simplify()
    return result


def bench_binomial_coefficient_with_parsing():
    """Benchmark binomial coefficient (with parsing): binomial(10, 5)."""
    expr = Expression.parse("binomial(10, 5)")
    result = expr.simplify()
    return result


# ============================================================================
# Special Function Benchmarks
# ============================================================================

def bench_gamma_symbolic_direct():
    """Benchmark gamma symbolic (direct API): gamma(x)."""
    x = Symbol("x")
    expr = Expression.function("gamma", [Expression.symbol(x)])
    result = expr.simplify()
    return result


def bench_gamma_symbolic_with_parsing():
    """Benchmark gamma symbolic (with parsing): gamma(x)."""
    expr = Expression.parse("gamma(x)")
    result = expr.simplify()
    return result


def bench_gamma_integer_direct():
    """Benchmark gamma at integer (direct API): gamma(5)."""
    expr = Expression.function("gamma", [Expression.integer(5)])
    result = expr.simplify()
    return result


def bench_gamma_integer_with_parsing():
    """Benchmark gamma at integer (with parsing): gamma(5)."""
    expr = Expression.parse("gamma(5)")
    result = expr.simplify()
    return result


# ============================================================================
# Function Composition Benchmarks
# ============================================================================

def bench_sin_exp_direct():
    """Benchmark sin(exp(x)) composition (direct API)."""
    x = Symbol("x")
    expr = Expression.function("sin", [
        Expression.function("exp", [Expression.symbol(x)])
    ])
    result = expr.simplify()
    return result


def bench_sin_exp_with_parsing():
    """Benchmark sin(exp(x)) composition (with parsing)."""
    expr = Expression.parse("sin(exp(x))")
    result = expr.simplify()
    return result


def bench_log_trig_sum_direct():
    """Benchmark log(sin(x) + cos(x)) (direct API)."""
    x = Symbol("x")
    expr = Expression.function("log", [
        Expression.add([
            Expression.function("sin", [Expression.symbol(x)]),
            Expression.function("cos", [Expression.symbol(x)])
        ])
    ])
    result = expr.simplify()
    return result


def bench_log_trig_sum_with_parsing():
    """Benchmark log(sin(x) + cos(x)) (with parsing)."""
    expr = Expression.parse("log(sin(x) + cos(x))")
    result = expr.simplify()
    return result


def bench_deeply_nested_direct():
    """Benchmark deeply nested functions (direct API): sin(cos(exp(log(x))))."""
    x = Symbol("x")
    expr = Expression.function("sin", [
        Expression.function("cos", [
            Expression.function("exp", [
                Expression.function("log", [Expression.symbol(x)])
            ])
        ])
    ])
    result = expr.simplify()
    return result


def bench_deeply_nested_with_parsing():
    """Benchmark deeply nested functions (with parsing): sin(cos(exp(log(x))))."""
    expr = Expression.parse("sin(cos(exp(log(x))))")
    result = expr.simplify()
    return result


# ============================================================================
# Benchmark Runner
# ============================================================================

def run_all_benchmarks(samples: int = 100) -> Dict[str, BenchmarkResult]:
    """Run all function evaluation benchmarks."""
    results = {}

    benchmarks = [
        # Elementary trigonometric
        bench_sin_symbolic_direct,
        bench_sin_symbolic_with_parsing,
        bench_cos_symbolic_direct,
        bench_cos_symbolic_with_parsing,
        bench_tan_symbolic_direct,
        bench_tan_symbolic_with_parsing,
        bench_nested_trig_direct,
        bench_nested_trig_with_parsing,
        bench_arcsin_symbolic_direct,
        bench_arcsin_symbolic_with_parsing,

        # Hyperbolic functions
        bench_sinh_symbolic_direct,
        bench_sinh_symbolic_with_parsing,
        bench_cosh_symbolic_direct,
        bench_cosh_symbolic_with_parsing,
        bench_tanh_symbolic_direct,
        bench_tanh_symbolic_with_parsing,

        # Exponential and logarithmic
        bench_exp_symbolic_direct,
        bench_exp_symbolic_with_parsing,
        bench_log_symbolic_direct,
        bench_log_symbolic_with_parsing,
        bench_log_base_10_direct,
        bench_log_base_10_with_parsing,
        bench_exp_log_identity_direct,
        bench_exp_log_identity_with_parsing,
        bench_nested_exp_direct,
        bench_nested_exp_with_parsing,

        # Power and root
        bench_sqrt_symbolic_direct,
        bench_sqrt_symbolic_with_parsing,
        bench_sqrt_square_direct,
        bench_sqrt_square_with_parsing,

        # Absolute value
        bench_abs_symbolic_direct,
        bench_abs_symbolic_with_parsing,
        bench_nested_abs_direct,
        bench_nested_abs_with_parsing,

        # Factorial and combinatorial
        bench_factorial_small_direct,
        bench_factorial_small_with_parsing,
        bench_factorial_symbolic_direct,
        bench_factorial_symbolic_with_parsing,
        bench_binomial_coefficient_direct,
        bench_binomial_coefficient_with_parsing,

        # Special functions
        bench_gamma_symbolic_direct,
        bench_gamma_symbolic_with_parsing,
        bench_gamma_integer_direct,
        bench_gamma_integer_with_parsing,

        # Function composition
        bench_sin_exp_direct,
        bench_sin_exp_with_parsing,
        bench_log_trig_sum_direct,
        bench_log_trig_sum_with_parsing,
        bench_deeply_nested_direct,
        bench_deeply_nested_with_parsing,
    ]

    print("=" * 80)
    print("Function Evaluation Benchmarks")
    print("=" * 80)

    for bench_func in benchmarks:
        print(f"Running {bench_func.__name__}...", end=" ")
        result = benchmark(bench_func, samples=samples)
        results[bench_func.__name__] = result
        print(f"{result.mean_ns:.2f}ns")

    print("=" * 80)

    return results


def main():
    """Main entry point for function evaluation benchmarks."""
    results = run_all_benchmarks(samples=100)

    print("\nDetailed Results:")
    print("-" * 80)
    for name, result in results.items():
        print(result)


if __name__ == "__main__":
    main()
