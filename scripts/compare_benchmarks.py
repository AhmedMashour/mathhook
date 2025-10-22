#!/usr/bin/env python3
"""
Compare benchmark results against baseline and detect regressions.

Usage:
    python scripts/compare_benchmarks.py \\
        --baseline benchmarks/baseline.json \\
        --current target/criterion \\
        --threshold 20 \\
        --output benchmarks/comparison.md
"""

import json
import argparse
from pathlib import Path
from typing import Dict, List, Tuple

def load_baseline(baseline_path: Path) -> Dict:
    """Load baseline benchmark results."""
    if not baseline_path.exists():
        return {}

    with open(baseline_path) as f:
        return json.load(f)

def load_current_results(criterion_dir: Path) -> Dict:
    """Parse Criterion benchmark results."""
    results = {}

    if not criterion_dir.exists():
        return results

    for bench_file in criterion_dir.rglob("base/estimates.json"):
        bench_name = bench_file.parent.parent.name
        with open(bench_file) as f:
            data = json.load(f)
            results[bench_name] = {
                'mean': data['mean']['point_estimate'],
                'std_dev': data['std_dev']['point_estimate']
            }

    return results

def compare_benchmarks(baseline: Dict, current: Dict, threshold: float) -> Tuple[List[Dict], bool]:
    """
    Compare current results with baseline.

    Returns:
        (comparison_data, has_regression)
    """
    comparison = []
    has_regression = False

    for bench_name, current_data in current.items():
        if bench_name not in baseline:
            comparison.append({
                'name': bench_name,
                'baseline': None,
                'current': current_data['mean'],
                'change_pct': None,
                'status': 'new'
            })
            continue

        baseline_mean = baseline[bench_name]['mean']
        current_mean = current_data['mean']
        change_pct = ((current_mean - baseline_mean) / baseline_mean) * 100

        if abs(change_pct) < 5:
            status = 'unchanged'
        elif change_pct > threshold:
            status = 'regression'
            has_regression = True
        elif change_pct < -5:
            status = 'improvement'
        else:
            status = 'minor_change'

        comparison.append({
            'name': bench_name,
            'baseline': baseline_mean,
            'current': current_mean,
            'change_pct': change_pct,
            'status': status
        })

    return comparison, has_regression

def format_time(nanoseconds: float) -> str:
    """Format time in human-readable units."""
    if nanoseconds < 1000:
        return f"{nanoseconds:.2f} ns"
    elif nanoseconds < 1_000_000:
        return f"{nanoseconds/1000:.2f} µs"
    elif nanoseconds < 1_000_000_000:
        return f"{nanoseconds/1_000_000:.2f} ms"
    else:
        return f"{nanoseconds/1_000_000_000:.2f} s"

def generate_markdown_report(comparison: List[Dict], has_regression: bool) -> str:
    """Generate Markdown report for PR comment."""

    status_emoji = {
        'regression': '🔴',
        'improvement': '🟢',
        'unchanged': '⚪',
        'minor_change': '🟡',
        'new': '🆕'
    }

    report = "## Benchmark Results\n\n"

    if has_regression:
        report += "⚠️ **Performance regression detected!** (>20% slower)\n\n"
    else:
        report += "✅ **No significant performance regressions**\n\n"

    report += "| Benchmark | Baseline | Current | Change | Status |\n"
    report += "|-----------|----------|---------|--------|--------|\n"

    for item in sorted(comparison, key=lambda x: x.get('change_pct', 0) if x.get('change_pct') is not None else -1000, reverse=True):
        emoji = status_emoji[item['status']]
        name = item['name']

        if item['baseline'] is None:
            baseline_str = "N/A"
            current_str = format_time(item['current'])
            change_str = "NEW"
        else:
            baseline_str = format_time(item['baseline'])
            current_str = format_time(item['current'])
            change_pct = item['change_pct']

            if change_pct > 0:
                change_str = f"+{change_pct:.1f}%"
            elif change_pct < 0:
                change_str = f"{change_pct:.1f}%"
            else:
                change_str = "0%"

        report += f"| {name} | {baseline_str} | {current_str} | {change_str} | {emoji} {item['status']} |\n"

    return report

def main():
    parser = argparse.ArgumentParser(description="Compare benchmark results")
    parser.add_argument("--baseline", type=Path, required=True)
    parser.add_argument("--current", type=Path, required=True)
    parser.add_argument("--threshold", type=float, default=20.0)
    parser.add_argument("--output", type=Path, required=True)

    args = parser.parse_args()

    baseline = load_baseline(args.baseline)
    current = load_current_results(args.current)

    if not current:
        print("No current benchmark results found!")
        return 1

    comparison, has_regression = compare_benchmarks(baseline, current, args.threshold)
    report = generate_markdown_report(comparison, has_regression)

    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(report)

    if has_regression:
        (args.output.parent / "regression_detected").touch()

    print(report)

    return 1 if has_regression else 0

if __name__ == "__main__":
    exit(main())
