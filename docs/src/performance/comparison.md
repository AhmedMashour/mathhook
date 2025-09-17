# Performance Comparison

MathHook is designed for high performance while maintaining mathematical correctness. This page shows how MathHook compares to other computer algebra systems.

## MathHook vs SymPy

SymPy is the most widely used Python computer algebra system. MathHook, written in Rust, provides significant performance improvements while maintaining compatibility.

### Derivative Performance

| Operation | SymPy | MathHook | Speedup |
|-----------|-------|----------|---------|
| d/dx(x^2) | 54.9ms | 0.035ms | **1580x** |
| d/dx(x^3) | 2.5ms | 0.019ms | **131x** |
| d/dx(x^4) | 3.3ms | 0.017ms | **189x** |
| d/dx(sin(x)) | 1.6ms | 0.048ms | **34x** |
| d/dx(cos(x)) | 1.6ms | 0.038ms | **42x** |
| d/dx(tan(x)) | 3.5ms | 0.035ms | **101x** |
| d/dx(exp(x)) | 1.8ms | 0.049ms | **36x** |
| d/dx(x*sin(x)) | 2.6ms | 0.079ms | **34x** |
| d/dx(sin(x)/x) | 7.3ms | 0.129ms | **56x** |
| d/dx(sin(x^2)) | 2.4ms | 0.042ms | **57x** |
| d/dx(x^2+2x+1) | 2.7ms | 0.014ms | **190x** |
| d/dx(x^2*exp(x)) | 3.3ms | 0.106ms | **31x** |
| d/dx(1/x) | 1.0ms | 0.006ms | **167x** |

### Summary Statistics

| Metric | Value |
|--------|-------|
| Minimum Speedup | 17x |
| Maximum Speedup | 1580x |
| Average Speedup | **179x** |
| Correctness Rate | 86.7% |

### Why MathHook is Faster

1. **Compiled Language**: Rust compiles to native machine code, eliminating interpreter overhead
2. **Zero-Cost Abstractions**: Rust's ownership system enables efficient memory management without garbage collection
3. **Cache-Optimized Structures**: Expression type is exactly 32 bytes, fitting two expressions per CPU cache line
4. **SIMD Operations**: Vectorized operations for numerical evaluation
5. **Lazy Evaluation**: Computations are deferred until results are needed

## Core Operation Performance

Based on Criterion benchmarks:

| Operation | Time | Target |
|-----------|------|--------|
| Expression Creation | ~159 ns | < 200 ns |
| Basic Simplification | ~8 ns | < 50 ns |
| Expression with Parsing | ~441 us | < 1 ms |

## Running Benchmarks

### Quick Benchmark

```bash
./scripts/bench.sh quick
```

### Full Benchmark Suite

```bash
./scripts/bench.sh run
```

### Specific Benchmark Groups

```bash
./scripts/bench.sh rust core_performance
./scripts/bench.sh rust calculus_benchmarks
./scripts/bench.sh rust polynomial_benchmarks
./scripts/bench.sh rust simplification_benchmarks
```

### Compare Against Baseline

```bash
# Save current as baseline
./scripts/bench.sh save my-baseline

# Run and compare
./scripts/bench.sh compare my-baseline
```

## Validation Against SymPy

MathHook includes validation scripts to verify mathematical correctness against SymPy:

```bash
./scripts/validate.sh           # Run all validations
./scripts/validate.sh simplify  # Simplification only
./scripts/validate.sh ode       # ODE solver only
```

## Continuous Integration

Performance is monitored in CI:
- Benchmarks run on every PR
- Regressions > 10% fail the build
- Results are posted as PR comments
- Baselines are updated on merge to main

See the [benchmarking guide](./benchmarking.md) for detailed information.
