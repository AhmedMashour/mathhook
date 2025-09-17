"""
MathHook Python Benchmarks

Module-based benchmark suite for MathHook Python bindings.
Mirrors the Rust benchmark structure for cross-platform comparison.

Usage:
    python -m public.python.core_performance
    python -m public.python.run_all
    python -m public.python.update_baseline
"""

__version__ = "0.1.0"
__all__ = [
    "core_performance",
    "calculus_benchmarks",
    "solving_benchmarks",
    "simplification_benchmarks",
    "function_evaluation_benchmarks",
    "parsing_benchmarks",
    "polynomial_benchmarks",
    "educational_benchmarks",
    "run_all",
    "update_baseline",
    "compare_baseline",
]
