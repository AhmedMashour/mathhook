#!/usr/bin/env python3
"""Benchmark Python bindings performance for hand-written vs macro-generated functions.

This benchmarks the FULL STACK: Python → PyO3 → Rust
"""

import timeit
import statistics
import mathhook

def bench_function(func_name, iterations=1000000):
    """Benchmark a function call with statistical analysis."""
    # Create a symbol once
    x = mathhook.symbols("x")[0]

    # Get the function
    func = getattr(mathhook, func_name)

    # Warmup
    for _ in range(1000):
        func(x)

    # Benchmark
    times = []
    for _ in range(100):
        start = timeit.default_timer()
        for _ in range(iterations // 100):
            func(x)
        end = timeit.default_timer()
        times.append((end - start) / (iterations // 100) * 1e9)  # ns per call

    mean = statistics.mean(times)
    stdev = statistics.stdev(times)
    median = statistics.median(times)

    return {
        'mean': mean,
        'stdev': stdev,
        'median': median,
        'min': min(times),
        'max': max(times)
    }

def main():
    print("=" * 80)
    print("Python Bindings Performance Benchmark")
    print("=" * 80)
    print()
    print("Testing: Python → PyO3 → Rust (full stack)")
    print("Iterations: 1,000,000 per function")
    print()

    # Benchmark hand-written functions
    print("Hand-Written Bindings (Current Approach):")
    print("-" * 80)

    hand_written = ['sin', 'cos', 'tan']
    hw_results = {}

    for func_name in hand_written:
        result = bench_function(func_name)
        hw_results[func_name] = result
        print(f"{func_name:20s} {result['mean']:8.2f} ns/call  "
              f"(σ={result['stdev']:6.2f} ns, median={result['median']:8.2f} ns)")

    print()

    # Benchmark macro-generated functions
    print("Macro-Generated Bindings (Proposed Approach):")
    print("-" * 80)

    macro_generated = ['sin_macro_generated', 'cos_macro_generated', 'tan_macro_generated']
    mg_results = {}

    for func_name in macro_generated:
        result = bench_function(func_name)
        mg_results[func_name] = result
        print(f"{func_name:20s} {result['mean']:8.2f} ns/call  "
              f"(σ={result['stdev']:6.2f} ns, median={result['median']:8.2f} ns)")

    print()
    print("=" * 80)
    print("Comparison Analysis:")
    print("=" * 80)
    print()

    # Compare corresponding functions
    comparisons = [
        ('sin', 'sin_macro_generated'),
        ('cos', 'cos_macro_generated'),
        ('tan', 'tan_macro_generated')
    ]

    for hw_name, mg_name in comparisons:
        hw_mean = hw_results[hw_name]['mean']
        mg_mean = mg_results[mg_name]['mean']
        diff_ns = mg_mean - hw_mean
        diff_pct = (diff_ns / hw_mean) * 100

        print(f"{hw_name} vs {mg_name}:")
        print(f"  Hand-written:     {hw_mean:8.2f} ns/call")
        print(f"  Macro-generated:  {mg_mean:8.2f} ns/call")
        print(f"  Difference:       {diff_ns:+8.2f} ns ({diff_pct:+.2f}%)")

        if abs(diff_pct) < 5:
            print(f"  Verdict:          ✅ IDENTICAL (within 5% noise)")
        elif diff_pct < 0:
            print(f"  Verdict:          ✅ MACRO FASTER by {-diff_pct:.2f}%")
        else:
            print(f"  Verdict:          ⚠️  MACRO SLOWER by {diff_pct:.2f}%")
        print()

    # Overall summary
    print("=" * 80)
    print("Overall Summary:")
    print("=" * 80)

    avg_hw = statistics.mean([r['mean'] for r in hw_results.values()])
    avg_mg = statistics.mean([r['mean'] for r in mg_results.values()])
    avg_diff = avg_mg - avg_hw
    avg_diff_pct = (avg_diff / avg_hw) * 100

    print(f"Average hand-written:     {avg_hw:8.2f} ns/call")
    print(f"Average macro-generated:  {avg_mg:8.2f} ns/call")
    print(f"Average difference:       {avg_diff:+8.2f} ns ({avg_diff_pct:+.2f}%)")
    print()

    if abs(avg_diff_pct) < 5:
        print("✅ RESULT: Macro-generated bindings have ZERO overhead")
        print("   Performance is identical to hand-written (within measurement noise)")
    elif avg_diff_pct < 0:
        print(f"✅ RESULT: Macro-generated bindings are {-avg_diff_pct:.2f}% FASTER")
    else:
        print(f"⚠️  RESULT: Macro-generated bindings are {avg_diff_pct:.2f}% SLOWER")

if __name__ == '__main__':
    main()
