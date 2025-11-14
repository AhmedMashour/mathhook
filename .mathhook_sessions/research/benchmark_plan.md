# Performance Benchmark Plan
**Wave 0 Research Phase**
**Date**: October 22, 2025

---

## Executive Summary

This document defines the performance benchmarking strategy for MathHook Core Mathematical Features. We establish baseline measurements from SymPy and set targets for MathHook (10-100x speedup goal).

---

## Benchmarking Methodology

### Tools
- **SymPy Baseline**: Python timeit for accurate measurements
- **MathHook**: Criterion.rs for Rust benchmarking
- **Comparison**: Direct comparison at equivalent complexity levels

### Environment
- **Hardware**: Record CPU model, RAM, OS
- **Software**: SymPy version, Python version, Rust version
- **Isolation**: Dedicated benchmarking environment, minimal background processes

---

## Wave 1: ODE Solving Benchmarks

### First-Order ODEs

| Test Case | SymPy Baseline | MathHook Target | Speedup Goal |
|-----------|---------------|-----------------|--------------|
| Separable (simple: dy/dx = x) | ~20ms | <2ms | 10x |
| Separable (medium: dy/dx = x*y) | ~50ms | <5ms | 10x |
| Linear 1st order (dy/dx + y = 0) | ~40ms | <4ms | 10x |
| Linear 1st order (dy/dx + y/x = 1) | ~80ms | <8ms | 10x |
| Homogeneous (dy/dx = y/x) | ~60ms | <6ms | 10x |

### Second-Order ODEs

| Test Case | SymPy Baseline | MathHook Target | Speedup Goal |
|-----------|---------------|-----------------|--------------|
| Constant coeff (y'' + y = 0) | ~100ms | <10ms | 10x |
| Constant coeff (y'' + 2y' + y = 0) | ~150ms | <15ms | 10x |
| Constant coeff complex roots (y'' + 4y = 0) | ~120ms | <12ms | 10x |
| Cauchy-Euler (x²y'' + xy' + y = 0) | ~200ms | <20ms | 10x |

### Benchmark Script Template (SymPy)

```python
import timeit
import sympy as sp

def benchmark_ode_separable():
    x, y = sp.symbols('x y')
    ode = sp.Eq(y.diff(x), x*y)

    def solve_fn():
        return sp.dsolve(ode, y)

    # Warm-up
    for _ in range(3):
        solve_fn()

    # Benchmark (100 iterations)
    time = timeit.timeit(solve_fn, number=100)
    print(f"Average time: {time/100*1000:.2f} ms")

benchmark_ode_separable()
```

### Benchmark Script Template (MathHook)

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mathhook_core::ode::first_order::solve_separable;
use mathhook_core::{symbol, expr};

fn benchmark_ode_separable(c: &mut Criterion) {
    let x = symbol!(x);
    let y = symbol!(y);
    let ode = expr!(y.diff(x) = x * y);

    c.bench_function("ode_separable_simple", |b| {
        b.iter(|| {
            solve_separable(black_box(&ode), black_box(&y), black_box(&x))
        })
    });
}

criterion_group!(benches, benchmark_ode_separable);
criterion_main!(benches);
```

---

## Wave 2: Linear Algebra Benchmarks

### Matrix Operations

| Test Case | Matrix Size | SymPy Baseline | MathHook Target | Speedup Goal |
|-----------|-------------|---------------|-----------------|--------------|
| QR decomposition | 3×3 | ~50ms | <5ms | 10x |
| QR decomposition | 5×5 | ~150ms | <15ms | 10x |
| QR decomposition | 10×10 | ~800ms | <80ms | 10x |
| LU decomposition | 3×3 | ~40ms | <4ms | 10x |
| LU decomposition | 5×5 | ~120ms | <12ms | 10x |
| SVD | 3×3 | ~100ms | <10ms | 10x |
| SVD | 5×5 | ~400ms | <40ms | 10x |

### Eigenvalue Computation

| Test Case | Matrix Size | SymPy Baseline | MathHook Target | Speedup Goal |
|-----------|-------------|---------------|-----------------|--------------|
| Eigenvalues (symbolic) | 2×2 | ~30ms | <3ms | 10x |
| Eigenvalues (symbolic) | 3×3 | ~100ms | <10ms | 10x |
| Eigenvalues (symbolic) | 4×4 | ~500ms | <50ms | 10x |
| Eigenvalues (numerical) | 10×10 | ~200ms | <20ms | 10x |
| Eigenvalues (numerical) | 50×50 | ~5s | <500ms | 10x |

---

## Wave 3: Polynomial & Number Theory Benchmarks

### Polynomial Operations

| Test Case | SymPy Baseline | MathHook Target | Speedup Goal |
|-----------|---------------|-----------------|--------------|
| Univariate factorization (x^4 - 1) | ~30ms | <3ms | 10x |
| Univariate factorization (x^10 - 1) | ~150ms | <15ms | 10x |
| Multivariate factorization (x^2 - y^2) | ~80ms | <8ms | 10x |
| Polynomial GCD (x^3-1, x^2-1) | ~40ms | <4ms | 10x |
| Polynomial GCD (degree 10) | ~200ms | <20ms | 10x |

### Number Theory

| Test Case | SymPy Baseline | MathHook Target | Speedup Goal |
|-----------|---------------|-----------------|--------------|
| Integer factorization (10^6) | ~10ms | <1ms | 10x |
| Integer factorization (10^9) | ~100ms | <10ms | 10x |
| Prime test (10^6) | ~5ms | <0.5ms | 10x |
| Totient function (10^6) | ~15ms | <1.5ms | 10x |

### Gröbner Basis

| Test Case | Variables | SymPy Baseline | MathHook Target | Speedup Goal |
|-----------|-----------|---------------|-----------------|--------------|
| Simple ideal (2 vars, 2 polys) | 2 | ~500ms | <50ms | 10x |
| Medium ideal (3 vars, 3 polys) | 3 | ~2s | <200ms | 10x |

---

## Wave 4: Series & Special Functions Benchmarks

### Series Expansions

| Test Case | SymPy Baseline | MathHook Target | Speedup Goal |
|-----------|---------------|-----------------|--------------|
| Taylor series (e^x, order 10) | ~40ms | <4ms | 10x |
| Taylor series (sin(x), order 20) | ~80ms | <8ms | 10x |
| Laurent series (1/x, order 10) | ~100ms | <10ms | 10x |
| Fourier series (order 10) | ~150ms | <15ms | 10x |

### Special Functions

| Test Case | SymPy Baseline | MathHook Target | Speedup Goal |
|-----------|---------------|-----------------|--------------|
| Gamma(5) | ~10ms | <1ms | 10x |
| Gamma(0.5) numerical | ~20ms | <2ms | 10x |
| Bessel J0(x) | ~50ms | <5ms | 10x |
| erf(x) numerical | ~15ms | <1.5ms | 10x |
| Hypergeometric 2F1 | ~100ms | <10ms | 10x |

---

## Wave 5: PDE Benchmarks

| Test Case | SymPy Baseline | MathHook Target | Speedup Goal |
|-----------|---------------|-----------------|--------------|
| Heat equation (separation) | ~200ms | <20ms | 10x |
| Wave equation (separation) | ~250ms | <25ms | 10x |
| Laplace equation | ~150ms | <15ms | 10x |

---

## Wave 6: Numerical Methods Benchmarks

### Numerical Integration

| Test Case | Points | SymPy/SciPy Baseline | MathHook Target | Speedup Goal |
|-----------|--------|---------------------|-----------------|--------------|
| Gaussian quadrature (10 points) | 10 | ~5ms | <0.5ms | 10x |
| Adaptive Simpson (∫ x² dx) | Variable | ~20ms | <2ms | 10x |
| Romberg integration | Variable | ~30ms | <3ms | 10x |

### Numerical ODE Solving

| Test Case | Steps | SymPy/SciPy Baseline | MathHook Target | Speedup Goal |
|-----------|-------|---------------------|-----------------|--------------|
| RK4 (simple ODE, 100 steps) | 100 | ~50ms | <5ms | 10x |
| RK45 adaptive (complex ODE) | Variable | ~150ms | <15ms | 10x |

---

## Benchmark Execution Strategy

### Phase 1: Establish SymPy Baselines (Week 1)
1. Install SymPy in isolated environment
2. Run all benchmark scripts
3. Record results in `.research/sympy_baseline_results.json`
4. Document hardware/software configuration

### Phase 2: Implement MathHook Benchmarks (Ongoing)
1. Create Criterion benchmarks for each operation
2. Run after implementation
3. Compare against baseline
4. Iterate until targets met

### Phase 3: Continuous Monitoring
1. CI integration for benchmark regression detection
2. Alert on >10% performance regression
3. Track performance improvements over time

---

## Benchmark Execution Scripts

### SymPy Baseline Runner

```python
#!/usr/bin/env python3
"""
SymPy Baseline Benchmark Runner
Measures performance of all operations planned for MathHook
"""

import json
import timeit
from datetime import datetime
import sympy as sp
import platform

def benchmark_suite():
    results = {
        "metadata": {
            "timestamp": datetime.now().isoformat(),
            "sympy_version": sp.__version__,
            "python_version": platform.python_version(),
            "platform": platform.platform(),
            "processor": platform.processor(),
        },
        "benchmarks": {}
    }

    # ODE benchmarks
    x, y = sp.symbols('x y')

    # Separable ODE: dy/dx = x*y
    ode_separable = sp.Eq(y.diff(x), x*y)
    time_ode_sep = timeit.timeit(
        lambda: sp.dsolve(ode_separable, y),
        number=100
    ) / 100 * 1000  # ms

    results["benchmarks"]["ode_separable_simple"] = {
        "operation": "solve dy/dx = x*y",
        "time_ms": time_ode_sep,
        "iterations": 100
    }

    # ... add more benchmarks

    return results

if __name__ == "__main__":
    results = benchmark_suite()
    with open(".research/sympy_baseline_results.json", "w") as f:
        json.dump(results, f, indent=2)
    print(f"Baseline benchmarks complete. Results saved.")
```

### MathHook Benchmark Comparison

```rust
// crates/mathhook-benchmarks/src/wave_comparison.rs

use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct BaselineResults {
    benchmarks: HashMap<String, BenchmarkResult>,
}

#[derive(Deserialize)]
struct BenchmarkResult {
    time_ms: f64,
}

#[derive(Serialize)]
struct ComparisonReport {
    operation: String,
    sympy_time_ms: f64,
    mathhook_time_ms: f64,
    speedup: f64,
    target_met: bool,
}

pub fn compare_with_baseline(
    operation_name: &str,
    mathhook_time_ms: f64
) -> ComparisonReport {
    // Load SymPy baseline
    let baseline_json = fs::read_to_string(".research/sympy_baseline_results.json")
        .expect("Baseline results not found");
    let baseline: BaselineResults = serde_json::from_str(&baseline_json).unwrap();

    let sympy_time = baseline.benchmarks
        .get(operation_name)
        .expect("Benchmark not found in baseline")
        .time_ms;

    let speedup = sympy_time / mathhook_time_ms;
    let target_met = speedup >= 10.0;

    ComparisonReport {
        operation: operation_name.to_string(),
        sympy_time_ms: sympy_time,
        mathhook_time_ms,
        speedup,
        target_met,
    }
}
```

---

## Performance Regression Prevention

### CI Integration

```yaml
# .github/workflows/benchmark.yml
name: Performance Benchmarks

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run benchmarks
        run: cargo bench --all
      - name: Compare with baseline
        run: |
          cargo run --bin compare_benchmarks
          # Fail if regression >10%
      - name: Upload results
        uses: actions/upload-artifact@v2
        with:
          name: benchmark-results
          path: target/criterion
```

---

## Optimization Tracking

### Performance Log

Track optimization efforts and their impact:

```markdown
# .research/optimization_log.md

## ODE Separable Optimization

### Baseline (Week 3)
- Time: 15ms
- Speedup vs SymPy: 3.3x
- Status: Below target

### Optimization 1: Arena Allocation (Week 4)
- Change: Use arena for temporary expressions
- Time: 8ms
- Speedup vs SymPy: 6.25x
- Improvement: 47% faster
- Status: Still below target

### Optimization 2: SIMD Integration (Week 5)
- Change: SIMD-optimized numerical evaluation
- Time: 4ms
- Speedup vs SymPy: 12.5x
- Improvement: 50% faster
- Status: TARGET MET ✓

### Optimization 3: Expression Caching (Week 6)
- Change: Cache repeated derivative computations
- Time: 2ms
- Speedup vs SymPy: 25x
- Improvement: 50% faster
- Status: Exceeds target
```

---

## Success Criteria

### Per-Operation Targets
- Minimum 10x speedup for all operations
- Stretch goal: 100x for simple operations

### Overall Goals
- Average speedup: >20x
- No operation slower than 2x SymPy
- All benchmarks run in <1 second total

---

## Conclusion

This benchmark plan provides:
1. **Clear targets** for each operation
2. **Methodology** for accurate measurements
3. **Scripts** for automated benchmarking
4. **Tracking** for continuous improvement
5. **Regression prevention** through CI

Implementation teams should reference this plan when optimizing each wave.
