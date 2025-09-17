#!/usr/bin/env python3
"""
Compare Against Python Baseline

Runs current benchmarks and compares against stored baseline.
Detects performance regressions and improvements.

Usage:
    python compare_baseline.py [--iterations N] [--threshold PERCENT]

Exit Codes:
    0: No regressions detected
    1: Performance regressions detected (slower than threshold)
    2: Baseline not found or error

Last Updated: 2025-11-29T2000
"""

import json
import sys
from pathlib import Path
import argparse

# Add bench script to path
sys.path.insert(0, str(Path(__file__).parent))
from bench_mathhook import run_all_benchmarks


def load_baseline(baselines_dir: Path) -> dict:
    """Load latest baseline from baselines directory."""
    latest_path = baselines_dir / 'latest.json'

    if not latest_path.exists():
        print(f"ERROR: No baseline found at {latest_path}", file=sys.stderr)
        print("Run: python update_baseline.py", file=sys.stderr)
        sys.exit(2)

    with open(latest_path, 'r') as f:
        return json.load(f)


def compare_benchmarks(current: dict, baseline: dict, threshold_percent: float) -> bool:
    """
    Compare current benchmarks against baseline.

    Returns: True if regressions detected, False otherwise
    """
    current_benches = current['benchmarks']
    baseline_benches = baseline['benchmarks']

    print("\n" + "=" * 80)
    print("Performance Comparison vs Baseline")
    print("=" * 80)
    print(f"Baseline: {baseline['metadata']['git_commit']} ({baseline['metadata']['timestamp']})")
    print(f"Threshold: {threshold_percent}% slowdown allowed")
    print()

    # Track regressions
    regressions = []
    improvements = []
    unchanged = []

    # Compare each benchmark
    for name, current_data in current_benches.items():
        if 'error' in current_data:
            print(f"SKIP {name:30s} (error in current)")
            continue

        if name not in baseline_benches:
            print(f"NEW  {name:30s} (not in baseline)")
            continue

        baseline_data = baseline_benches[name]
        if 'error' in baseline_data:
            print(f"SKIP {name:30s} (error in baseline)")
            continue

        # Calculate percentage change (using median for stability)
        current_ns = current_data['median_ns']
        baseline_ns = baseline_data['median_ns']

        percent_change = ((current_ns - baseline_ns) / baseline_ns) * 100

        # Categorize
        if percent_change > threshold_percent:
            regressions.append((name, percent_change, current_ns, baseline_ns))
        elif percent_change < -threshold_percent:
            improvements.append((name, percent_change, current_ns, baseline_ns))
        else:
            unchanged.append((name, percent_change, current_ns, baseline_ns))

    # Print results
    if regressions:
        print("\nREGRESSIONS (slower than baseline):")
        print("-" * 80)
        for name, pct, current, baseline in sorted(regressions, key=lambda x: x[1], reverse=True):
            print(f"  {name:30s} {pct:+7.2f}%  ({current/1000:.2f} us vs {baseline/1000:.2f} us)")

    if improvements:
        print("\nIMPROVEMENTS (faster than baseline):")
        print("-" * 80)
        for name, pct, current, baseline in sorted(improvements, key=lambda x: x[1]):
            print(f"  {name:30s} {pct:+7.2f}%  ({current/1000:.2f} us vs {baseline/1000:.2f} us)")

    if unchanged:
        print(f"\nUNCHANGED (within {threshold_percent}% threshold): {len(unchanged)} benchmarks")

    # Summary
    print("\n" + "=" * 80)
    print("Summary:")
    print(f"  Regressions: {len(regressions)}")
    print(f"  Improvements: {len(improvements)}")
    print(f"  Unchanged: {len(unchanged)}")
    print("=" * 80)

    return len(regressions) > 0


def main():
    parser = argparse.ArgumentParser(description='Compare Against Python Baseline')
    parser.add_argument('--iterations', type=int, default=100,
                        help='Number of benchmark iterations (default: 100)')
    parser.add_argument('--threshold', type=float, default=10.0,
                        help='Regression threshold percentage (default: 10.0)')
    args = parser.parse_args()

    # Determine baselines directory
    script_dir = Path(__file__).parent
    baselines_dir = script_dir.parent.parent / 'baselines' / 'python'

    print("=" * 80)
    print("MathHook Python Baseline Comparison")
    print("=" * 80)
    print(f"Iterations: {args.iterations}")
    print(f"Threshold: {args.threshold}%")
    print()

    # Load baseline
    print("Loading baseline...")
    baseline = load_baseline(baselines_dir)

    # Run current benchmarks
    print("Running current benchmarks...")
    current = run_all_benchmarks(iterations=args.iterations)

    # Compare
    has_regressions = compare_benchmarks(current, baseline, args.threshold)

    # Exit with appropriate code
    if has_regressions:
        print("\nFAILURE: Performance regressions detected!")
        sys.exit(1)
    else:
        print("\nSUCCESS: No performance regressions detected!")
        sys.exit(0)


if __name__ == '__main__':
    main()
