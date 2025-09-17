"""
Run All Python Benchmarks

Runs all benchmark categories and outputs combined JSON results.
"""

import argparse
import json
import sys
from typing import Dict, Any

try:
    from mathhook import Expression
except ImportError:
    print("ERROR: mathhook Python bindings not found. Install with: pip install mathhook")
    exit(1)

# Import all benchmark modules
import core_performance
import calculus_benchmarks
import solving_benchmarks
import simplification_benchmarks
import function_evaluation_benchmarks
import polynomial_benchmarks
import parsing_benchmarks


BENCHMARK_CATEGORIES = {
    "core": (core_performance, "Core Performance"),
    "calculus": (calculus_benchmarks, "Calculus"),
    "solving": (solving_benchmarks, "Solving"),
    "simplification": (simplification_benchmarks, "Simplification"),
    "functions": (function_evaluation_benchmarks, "Function Evaluation"),
    "polynomials": (polynomial_benchmarks, "Polynomials"),
    "parsing": (parsing_benchmarks, "Parsing"),
}


def format_result_to_dict(result) -> Dict[str, Any]:
    """Convert BenchmarkResult to dictionary for JSON serialization."""
    return {
        "name": result.name,
        "mean_ns": result.mean_ns,
        "median_ns": result.median_ns,
        "min_ns": result.min_ns,
        "max_ns": result.max_ns,
        "std_dev_ns": result.std_dev_ns,
        "samples": result.samples,
    }


def run_category(category_name: str, module, samples: int = 100) -> Dict[str, Any]:
    """Run benchmarks for a specific category."""
    print(f"\n{'=' * 80}")
    print(f"Running {BENCHMARK_CATEGORIES[category_name][1]} Benchmarks")
    print(f"{'=' * 80}")

    results = module.run_all_benchmarks(samples=samples)

    # Convert to JSON-serializable format
    json_results = {
        name: format_result_to_dict(result)
        for name, result in results.items()
    }

    return json_results


def run_all_benchmarks(categories=None, samples: int = 100) -> Dict[str, Any]:
    """
    Run all or specified benchmark categories.

    Args:
        categories: List of category names to run, or None for all
        samples: Number of samples per benchmark

    Returns:
        Dictionary with all benchmark results
    """
    if categories is None:
        categories = list(BENCHMARK_CATEGORIES.keys())

    all_results = {}

    for category in categories:
        if category not in BENCHMARK_CATEGORIES:
            print(f"WARNING: Unknown category '{category}', skipping...")
            continue

        module, _ = BENCHMARK_CATEGORIES[category]
        category_results = run_category(category, module, samples=samples)
        all_results[category] = category_results

    return all_results


def print_summary(results: Dict[str, Any]):
    """Print summary statistics for all benchmarks."""
    print("\n" + "=" * 80)
    print("BENCHMARK SUMMARY")
    print("=" * 80)

    for category, category_results in results.items():
        category_name = BENCHMARK_CATEGORIES[category][1]
        print(f"\n{category_name}:")
        print("-" * 80)

        # Calculate category statistics
        all_means = [result["mean_ns"] for result in category_results.values()]
        if all_means:
            min_mean = min(all_means)
            max_mean = max(all_means)
            avg_mean = sum(all_means) / len(all_means)

            print(f"  Benchmarks: {len(category_results)}")
            print(f"  Fastest:    {min_mean:.2f}ns")
            print(f"  Slowest:    {max_mean:.2f}ns")
            print(f"  Average:    {avg_mean:.2f}ns")

    print("\n" + "=" * 80)


def main():
    """Main entry point for running all benchmarks."""
    parser = argparse.ArgumentParser(
        description="Run MathHook Python benchmarks",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s                              # Run all benchmarks
  %(prog)s --category core              # Run only core benchmarks
  %(prog)s --category core calculus     # Run core and calculus benchmarks
  %(prog)s --samples 50                 # Run with 50 samples per benchmark
  %(prog)s --output results.json        # Save results to JSON file
        """
    )

    parser.add_argument(
        "--category",
        "-c",
        nargs="+",
        choices=list(BENCHMARK_CATEGORIES.keys()),
        help="Benchmark categories to run (default: all)",
    )

    parser.add_argument(
        "--samples",
        "-s",
        type=int,
        default=100,
        help="Number of samples per benchmark (default: 100)",
    )

    parser.add_argument(
        "--output",
        "-o",
        type=str,
        help="Output JSON file path (optional)",
    )

    parser.add_argument(
        "--quiet",
        "-q",
        action="store_true",
        help="Suppress detailed output, only show summary",
    )

    args = parser.parse_args()

    # Run benchmarks
    results = run_all_benchmarks(categories=args.category, samples=args.samples)

    # Print summary
    if not args.quiet:
        print_summary(results)

    # Save to JSON if requested
    if args.output:
        with open(args.output, "w") as f:
            json.dump(results, f, indent=2)
        print(f"\nResults saved to: {args.output}")

    # Return success
    return 0


if __name__ == "__main__":
    sys.exit(main())
