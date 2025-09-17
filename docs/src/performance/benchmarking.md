# Benchmarking Guide

Comprehensive guide to MathHook's performance benchmarking infrastructure across all supported platforms.

**Last Updated:** 2025-11-30T0300

## Usage

MathHook provides two unified entry points for performance and correctness validation:

| Script | Purpose | Location |
|--------|---------|----------|
| `./scripts/bench.sh` | Performance benchmarking (Rust, Python, Node.js) | Criterion + bindings |
| `./scripts/validate.sh` | Mathematical correctness (SymPy comparison) | Python validation scripts |

### Performance Benchmarking

```bash
# Run all Rust benchmarks
./scripts/bench.sh run

# Quick run (reduced samples for faster feedback)
./scripts/bench.sh quick

# Run specific benchmark group
./scripts/bench.sh rust polynomial_benchmarks

# Cross-platform benchmarks
./scripts/bench.sh python           # Python binding benchmarks
./scripts/bench.sh node             # Node.js binding benchmarks

# Baseline workflow (before/after comparison)
./scripts/bench.sh save before      # Save baseline
# ... make changes ...
./scripts/bench.sh compare before   # Compare against baseline

# CI mode (fails on >10% regression)
./scripts/bench.sh ci

# Check infrastructure status
./scripts/bench.sh status
```

### Mathematical Validation

```bash
# Run all SymPy validations
./scripts/validate.sh

# Specific validation suites
./scripts/validate.sh simplify      # Algebraic simplification
./scripts/validate.sh ode           # ODE solver correctness
./scripts/validate.sh summation     # Summation operations

# Get help
./scripts/validate.sh help
```

### Typical Development Workflow

```bash
# 1. Before making changes - save baseline
./scripts/bench.sh save my-feature

# 2. Make your changes...

# 3. Verify mathematical correctness
./scripts/validate.sh

# 4. Compare performance
./scripts/bench.sh compare my-feature

# 5. If adding new algorithm, add benchmarks
#    (see "Contributing Benchmarks" section)
```

## Quick Start

Get up and running with benchmarks in 30 seconds:

```bash
# Run all Rust benchmarks
./scripts/bench.sh run

# Quick run (reduced samples for faster feedback)
./scripts/bench.sh quick

# Run specific benchmark group
./scripts/bench.sh group polynomial_benchmarks

# Save baseline for your feature branch
./scripts/bench.sh save my-feature

# Compare against saved baseline
./scripts/bench.sh compare my-feature

# Check infrastructure status
./scripts/bench.sh status
```

**Direct Cargo Commands:**
```bash
# All benchmarks
cargo bench

# Specific group
cargo bench --bench core_performance

# Specific benchmark within group
cargo bench --bench polynomial_benchmarks -- gcd_algorithms

# Save and compare baselines
cargo bench -- --save-baseline main
cargo bench -- --baseline main
```

## Directory Structure

```
crates/mathhook-benchmarks/
├── benches/                      # Rust Criterion benchmarks
│   ├── core_performance.rs       # Core operations + parsing variants
│   ├── calculus_benchmarks.rs    # Derivatives, integrals
│   ├── simplification_benchmarks.rs
│   ├── solving_benchmarks.rs
│   ├── polynomial_benchmarks.rs
│   ├── function_evaluation_benchmarks.rs
│   └── parsing_benchmarks.rs
├── public/                       # Cross-platform benchmarks
│   ├── python/                   # Python binding benchmarks
│   └── node/                     # Node.js binding benchmarks
├── baselines/                    # Baseline storage
└── results/                      # Output files
```

## Benchmark Categories

### Core Performance (`core_performance.rs`)

Basic operation benchmarks establishing performance baselines:

**Operations Tested:**
- Expression creation (native vs parsing)
- Simplification (constant folding, algebraic reduction)
- Basic equation solving
- Polynomial operations
- Memory efficiency validation (32-byte Expression size)

**Example Benchmarks:**
```rust
expression_creation
expression_creation_with_parsing
simplification
simplification_with_parsing
basic_solving
basic_solving_with_parsing
polynomial_creation
polynomial_creation_with_parsing
```

### Calculus Operations (`calculus_benchmarks.rs`)

Symbolic calculus performance for derivatives and integrals:

**Derivatives Tested:**
- Power rule (varying degrees: 2, 5, 10, 20, 50)
- Product rule (with and without parsing)
- Chain rule (with and without parsing)
- Quotient rule
- Higher-order derivatives (1st through 5th)
- Trigonometric derivatives
- Exponential and logarithmic derivatives
- Multivariable partial derivatives

**Integrals Tested:**
- Power rule integration
- Trigonometric integrals (with and without parsing)
- Exponential integrals
- Rational function integration
- Polynomial integration (varying degrees)
- Substitution integrals

### Equation Solving (`solving_benchmarks.rs`)

Solver performance across equation types:

**Linear Equations:**
- Simple linear (2x + 3 = 0, with and without parsing)
- Large coefficient handling
- Rational coefficient equations

**Quadratic Equations:**
- Real roots (x^2 - 4 = 0)
- Complex roots (x^2 + 1 = 0)
- Perfect squares
- General quadratics with varying coefficients

**Polynomial Equations:**
- Cubic equations
- Quartic equations
- Higher-degree polynomials (degree 5, 7, 10)

**System Solving:**
- 2x2 linear systems
- 3x3 linear systems
- Nonlinear 2x2 systems

**Matrix Equations:**
- 2x2 matrix equations (AX = B)
- 3x3 matrix equations

**Differential Equations:**
- Separable ODEs
- Linear first-order ODEs
- PDE placeholders

### Simplification (`simplification_benchmarks.rs`)

Algebraic simplification performance:

**Polynomial Simplification:**
- Like term collection (with and without parsing)
- Product expansion
- Power combination
- Binomial expansion (powers: 2, 5, 10, 15)
- Multinomial simplification

**Trigonometric Simplification:**
- Pythagorean identity (sin^2(x) + cos^2(x) -> 1)
- Double angle formulas
- Reciprocal identities
- Quotient identities

**Logarithmic Simplification:**
- Product rule (log(x) + log(y) -> log(xy))
- Quotient rule (log(x) - log(y) -> log(x/y))
- Power rule (n*log(x) -> log(x^n))
- Combined rules

**Zero Detection:**
- Obvious zeros (x - x -> 0)
- Non-obvious zeros (algebraic cancellation)
- Identity simplification (x * 1 -> x, x + 0 -> x)

**Large Expressions:**
- Many-term polynomials (10, 25, 50, 100 terms)
- Throughput measurement (elements/second)

### Function Evaluation (`function_evaluation_benchmarks.rs`)

Elementary and special function performance:

**Elementary Functions:**
- Trigonometric: sin, cos, tan, arcsin, arctan (with parsing variants)
- Hyperbolic: sinh, cosh, tanh
- Exponential and logarithmic: exp, log, log with base
- Power and roots: sqrt, nth roots, rational exponents

**Special Functions:**
- Gamma function (symbolic and integer values)
- Bessel functions
- Zeta function

**Combinatorial Functions:**
- Factorial (values: 1, 5, 10, 15, 20)
- Binomial coefficients

**Function Composition:**
- Nested functions (sin(exp(x)), log(sin(x) + cos(x)))
- Deep nesting (f(g(h(i(x)))))
- Identity compositions (exp(log(x)), log(exp(x)))

### Polynomial Module (`polynomial_benchmarks.rs`)

Comprehensive polynomial operation benchmarks:

**GCD Algorithms:**
- Univariate GCD (simple with parsing, degree 10, large coefficients)
- Multivariate GCD (bivariate, trivariate, Zippel algorithm)
- Various configurations and edge cases

**Division Operations:**
- Long division (simple, degree 8)
- Exact division

**Factorization:**
- Common factor extraction
- Quadratic factoring
- Cubic factoring

**Resultant and Discriminant:**
- Quadratic resultants
- Degree-4 resultants

**Grobner Basis:**
- Simple 2-variable systems
- Different monomial orders (Lex, Grlex, Grevlex)

**Special Polynomial Families:**
- Legendre polynomials (symbolic expansion: 5, 10, 15, 20)
- Chebyshev polynomials (T_n)
- Hermite polynomials
- Laguerre polynomials
- Numerical evaluation at test points

**Finite Field Arithmetic:**
- PolyZp creation
- Multiplication
- GCD
- Division

### Parsing Benchmarks (`parsing_benchmarks.rs`)

Parser performance across expression complexity:

**Basic Parsing:**
- Single symbol, number
- Binary operations (+, *, ^)
- Function calls

**Complex Parsing:**
- Polynomial expressions
- Nested functions
- Rational expressions
- Trigonometric identities

**Implicit Multiplication:**
- Coefficient notation (2x)
- Parenthesized products (2(x+1))
- Adjacent factors ((a)(b))
- Function products (sin(x)cos(x))

## Interpreting Results

### What the Numbers Mean

**Nanoseconds (ns) to Real-World Performance:**

| Time | Operations/Second | Context |
|------|-------------------|---------|
| 1 ns | 1,000,000,000 | Memory access |
| 10 ns | 100,000,000 | Simple expression creation |
| 100 ns | 10,000,000 | Basic parsing |
| 1 us | 1,000,000 | Simple simplification |
| 10 us | 100,000 | Polynomial GCD |
| 100 us | 10,000 | Complex calculus |
| 1 ms | 1,000 | Large system solving |

### Criterion Output Explained

```
expression_creation      time:   [85.234 ns 86.127 ns 87.142 ns]
                         change: [-2.3421% -1.2345% +0.1234%] (p = 0.12 > 0.05)
                         No change in performance detected.
```

**Fields:**
- **time**: [lower bound, mean, upper bound] at 95% confidence
- **change**: Performance change vs baseline [lower, mean, upper]
- **p-value**: Statistical significance (p < 0.05 = significant)

**Interpretation:**
- `change: [-5%, -3%, -1%]` = Got faster (negative is better)
- `change: [+1%, +3%, +5%]` = Got slower (regression)
- `p > 0.05` = Change might be noise, not significant
- `p < 0.05` = Change is statistically significant

### Performance Expectations

**MathHook vs Competitors:**
```
Rust (native) > Node.js (NAPI) > Python (PyO3)
```

**Expected Binding Overhead:**
- **Python**: 2-5x slower than Rust (FFI overhead)
- **Node.js**: 1.5-3x slower than Rust (NAPI overhead)
- **Parsing**: Additional 10-30% overhead

**Target Performance Baselines:**

| Category | Target | Current |
|----------|--------|---------|
| Expression creation | < 100 ns | ~85 ns |
| Simple parsing | < 200 ns | ~150 ns |
| Simplification | < 500 ns | ~350 ns |
| Power rule derivative | < 1 us | ~700 ns |
| Simple GCD | < 5 us | ~3 us |
| Polynomial factoring | < 50 us | ~30 us |

### Identifying Regressions

**Regression Thresholds:**

| Severity | Threshold | Action |
|----------|-----------|--------|
| Critical | > 20% | CI fails, blocks merge |
| Warning | > 10% | Flagged for review |
| Noise | < 5% | Usually statistical noise |

**Common Regression Causes:**
1. Added allocations in hot path
2. Changed algorithm complexity
3. Removed SIMD optimization
4. Added unnecessary cloning
5. Changed cache access patterns

## Contributing Benchmarks

### When to Add Benchmarks

Add benchmarks when:
- Implementing new algorithm (e.g., new GCD method)
- Optimizing existing code (need before/after comparison)
- Adding new operation type (e.g., new function family)
- Performance is a key feature requirement

### Benchmark Template

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use mathhook_core::{parse, symbol, Expression};
use std::hint::black_box;

fn bench_my_operation(c: &mut Criterion) {
    let mut group = c.benchmark_group("my_operation_group");

    let x = symbol!(x);

    // Without parsing - measures pure algorithm
    group.bench_function("operation_native", |b| {
        let expr = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::integer(1),
        ]);
        b.iter(|| black_box(expr.clone().my_operation()))
    });

    // With parsing - measures end-to-end user experience
    group.bench_function("operation_with_parsing", |b| {
        b.iter(|| {
            let expr = parse("x + 1").unwrap();
            black_box(expr.my_operation())
        })
    });

    group.finish();
}

criterion_group!(benches, bench_my_operation);
criterion_main!(benches);
```

### Naming Conventions

| Pattern | Example | Usage |
|---------|---------|-------|
| `<operation>` | `simplification` | Pure algorithmic test |
| `<operation>_with_parsing` | `simplification_with_parsing` | End-to-end with parser |
| `<category>/<variant>` | `derivatives/power_rule` | Grouped variations |
| `<category>/<variant>/<param>` | `derivatives/power_rule/10` | Parameterized tests |

### Best Practices

1. **Always use `black_box`**: Prevents compiler from optimizing away work
2. **Test both patterns**: Native AND parsing variants for comparison
3. **Use meaningful groups**: Group related benchmarks together
4. **Document expected ranges**: Note what "good" performance looks like
5. **Test edge cases**: Empty, single element, maximum size
6. **Parameterize where useful**: Use BenchmarkId for varying inputs
7. **Set appropriate sample size**: Default is 100, adjust for slow benchmarks

### Adding to CI

New benchmarks are automatically included in CI. To add regression thresholds:

1. Run benchmark to establish baseline
2. Add expected range to documentation
3. CI will flag significant deviations

## Running Benchmarks

### Full Benchmark Suite

```bash
# All Rust benchmarks
cargo bench

# With minimal output (CI mode)
cargo bench -- --noplot

# Save results for comparison
cargo bench -- --save-baseline my-feature
```

### Specific Categories

```bash
cargo bench --bench core_performance
cargo bench --bench calculus_benchmarks
cargo bench --bench solving_benchmarks
cargo bench --bench simplification_benchmarks
cargo bench --bench function_evaluation_benchmarks
cargo bench --bench polynomial_benchmarks
cargo bench --bench parsing_benchmarks
```

### Specific Benchmarks

```bash
# Single benchmark
cargo bench --bench core_performance -- expression_creation

# Pattern matching
cargo bench --bench polynomial_benchmarks -- gcd

# Exclude pattern
cargo bench --bench calculus_benchmarks -- --skip integral
```

### Baseline Management

```bash
# Save current as baseline
cargo bench -- --save-baseline main

# Compare against baseline
cargo bench -- --baseline main

# Save with timestamp
cargo bench -- --save-baseline "$(date +%Y%m%d)"
```

### Python Benchmarks

```bash
cd crates/mathhook-benchmarks/public/python

# Run all
python3 bench_mathhook.py

# JSON output for automation
python3 bench_mathhook.py --json > results.json
```

### Node.js Benchmarks

```bash
cd crates/mathhook-benchmarks/public/node

# Run all
node bench_mathhook.js

# JSON output
node bench_mathhook.js --json > results.json
```

## CI/CD Integration

### Cross-Platform Benchmark Pipeline

The CI runs benchmarks across **all three platforms** in parallel:

```
Cross-Platform Benchmarks Workflow
├── rust-benchmarks      # Criterion benchmarks (native)
├── python-benchmarks    # PyO3 bindings via maturin
├── node-benchmarks      # NAPI bindings via napi-rs
├── summary              # Aggregate results + PR comment
└── update-baselines     # On push to main/master
```

### Automatic Regression Detection

Every pull request automatically:
1. Runs benchmarks on **Rust, Python, and Node.js**
2. Compares each platform against its baseline
3. Posts unified comparison report as PR comment
4. Reports per-platform status with details

### Workflow Triggers

| Event | Action |
|-------|--------|
| Pull Request | Run all platform benchmarks, compare, report |
| Push to main | Run all benchmarks, update all baselines |
| Manual dispatch | Full benchmark run with configurable threshold |

### Manual Trigger Options

```bash
# Trigger workflow manually from GitHub Actions UI
# - threshold: Regression threshold percentage (default: 10.0)
```

### Baseline Management

**Per-Platform Baselines:**
- `benchmarks/rust_baseline.json` - Rust Criterion results
- `benchmarks/python_baseline.json` - Python PyO3 results
- `benchmarks/node_baseline.json` - Node.js NAPI results

**On merge to main/master:**
1. All platform benchmarks run in release mode
2. Results exported to platform-specific baseline files
3. Baselines committed with `[skip ci]` tag

### PR Comment Format

Every PR receives a unified benchmark report:

```markdown
## Benchmark Results

**Regression Threshold:** 10.0%

### Platform Status

| Platform | Status | Details |
|----------|--------|---------|
| Rust | :white_check_mark: Passed | Criterion benchmarks |
| Python | :white_check_mark: Passed | PyO3 bindings |
| Node.js | :white_check_mark: Passed | NAPI bindings |

---

### Rust Comparison
[Detailed Criterion comparison...]

### Python Comparison
[Detailed Python benchmark comparison...]

### Node Comparison
[Detailed Node.js benchmark comparison...]
```

### Artifacts

Benchmark results are:
- Archived for 30 days per commit
- Separated by platform (`rust-benchmark-results`, `python-benchmark-results`, `node-benchmark-results`)
- Available for download from GitHub Actions
- Used for trend analysis

## Troubleshooting

### Common Issues

**Benchmarks fail to compile:**
```bash
cargo build --release
cargo bench --bench core_performance
```

**Inconsistent results:**
```bash
# Increase sample size
cargo bench -- --sample-size 200

# Disable CPU frequency scaling
sudo cpupower frequency-set --governor performance
```

**Very slow benchmarks:**
```bash
# Reduce sample size
cargo bench -- --sample-size 10

# Run specific benchmark only
cargo bench --bench core_performance -- expression_creation
```

**Python import errors:**
```bash
# Install bindings
cd crates/mathhook-python
maturin develop --release
```

**Node.js errors:**
```bash
cd crates/mathhook-node
npm install
npm run build
```

### Getting Help

- Check existing benchmark files for patterns
- Review CI logs for environment setup
- Open issue with benchmark output and system info

## Dashboard Auto-Discovery System

MathHook's benchmark dashboard **automatically discovers** all benchmarks from all platforms without any hardcoding. This means:

1. **Add a benchmark** to any platform (Rust, Python, Node.js)
2. **Run the CI** or local script
3. **New benchmark appears** in dashboard and comparison pages automatically

### How Auto-Discovery Works

The dashboard generator (`scripts/ci/generate_dashboard.py`) reads JSON benchmark results and:

1. **Discovers all benchmark IDs** from each platform's JSON file
2. **Auto-generates display names** from benchmark IDs (e.g., `gcd_simple` -> "GCD Simple")
3. **Auto-detects categories** from prefixes (e.g., `parse_*` -> "Parsing", `mul_*` -> "Multiplication")
4. **Merges results** across platforms for comparison

**No code changes required** - just add your benchmark and it appears everywhere.

### Auto-Generated Display Names

The system converts benchmark IDs to human-readable names:

| Benchmark ID | Auto-Generated Name |
|--------------|---------------------|
| `gcd_simple` | GCD Simple |
| `parse_large` | Parse Large |
| `mul_medium` | Multiply Medium |
| `expand_simple` | Expand Simple |
| `factor_quadratic` | Factor Quadratic |
| `simplify_polynomial` | Simplify Polynomial |

**Naming Convention**: Use `operation_size` or `operation_variant` format for best auto-generation.

### Category Auto-Detection

Categories are detected from benchmark ID prefixes:

| Prefix | Category |
|--------|----------|
| `parse_*` | Parsing |
| `gcd_*` | GCD |
| `mul_*` | Multiplication |
| `div_*` | Division |
| `expand_*` | Expansion |
| `simplify_*` | Simplification |
| `factor_*` | Factorization |
| Other | Miscellaneous |

## Adding New Benchmarks

### Step 1: Choose Your Platform

Decide where to add your benchmark:

| Platform | Location | Format |
|----------|----------|--------|
| Rust (Criterion) | `crates/mathhook-benchmarks/benches/*.rs` | Native Rust |
| Python (MathHook) | `crates/mathhook-benchmarks/public/python/bench_mathhook.py` | Python |
| Node.js (MathHook) | `crates/mathhook-benchmarks/public/node/bench_mathhook.js` | JavaScript |

### Step 2: Add the Benchmark

#### Rust (Criterion)

```rust
// In crates/mathhook-benchmarks/benches/polynomial_benchmarks.rs

fn bench_my_new_operation(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial");

    // Use snake_case naming: operation_variant
    group.bench_function("gcd_my_new_case", |b| {
        let x = symbol!(x);
        let f = expr!(x^3 - 1);
        let g = expr!(x^2 - 1);
        b.iter(|| black_box(polynomial_gcd(&f, &g)))
    });

    group.finish();
}
```

#### Python (MathHook Bindings)

```python
# In crates/mathhook-benchmarks/public/python/bench_mathhook.py

def bench_gcd_my_new_case():
    """GCD of my new test case."""
    f = mathhook.parse("x^3 - 1")
    g = mathhook.parse("x^2 - 1")
    return mathhook.gcd(f, g)

# Add to benchmarks dict:
benchmarks = {
    # ... existing benchmarks ...
    'gcd_my_new_case': bench_gcd_my_new_case,
}
```

#### Python (SymPy Comparison)

```python
def bench_gcd_my_new_case():
    """GCD of my new test case."""
    f = x**3 - 1
    g = x**2 - 1
    return gcd(f, g)

# Add to benchmarks dict:
benchmarks = {
    # ... existing benchmarks ...
    'gcd_my_new_case': bench_gcd_my_new_case,
}
```

#### Node.js (MathHook Bindings)

```javascript
// In crates/mathhook-benchmarks/public/node/bench_mathhook.js

const benchmarks = {
    // ... existing benchmarks ...

    gcd_my_new_case: () => {
        const f = mathhook.parse("x^3 - 1");
        const g = mathhook.parse("x^2 - 1");
        return mathhook.gcd(f, g);
    },
};
```

### Step 3: Run and Verify

```bash
# Run local CI to test all platforms
./scripts/ci/run_local.sh

# View dashboard
cd gh-pages && python3 -m http.server 8080
# Open http://localhost:8080
# Check comparison at http://localhost:8080/comparison.html
```

Your new benchmark `gcd_my_new_case` will automatically:
- Appear in the main dashboard under "GCD" category
- Be named "GCD My New Case" in the UI

### Step 4: Ensure Matching Benchmarks

For valid comparisons, **add the same benchmark to all platforms**:

```
gcd_my_new_case  ->  Rust (MathHook native)
gcd_my_new_case  ->  Python (MathHook PyO3)
gcd_my_new_case  ->  Node.js (MathHook NAPI)
```

The comparison page matches benchmarks by ID, so use **identical names** across platforms.

## JSON Output Format

All benchmark scripts output JSON in this format:

```json
{
  "platform": "python",
  "python_version": "3.11.0",
  "benchmarks": {
    "gcd_simple": {
      "mean_ns": 12345.67,
      "stdev_ns": 456.78,
      "median_ns": 12000.00,
      "min_ns": 11000,
      "max_ns": 14000,
      "iterations": 100
    },
    "gcd_medium": {
      "mean_ns": 23456.78,
      "stdev_ns": 567.89,
      "median_ns": 23000.00,
      "min_ns": 22000,
      "max_ns": 25000,
      "iterations": 100
    }
  }
}
```

**Required Fields:**
- `platform`: Platform identifier (e.g., "python", "node-mathhook")
- `benchmarks`: Dictionary of benchmark_id -> timing data
- `mean_ns`: Mean time in nanoseconds
- `iterations`: Number of iterations run

**Optional Fields:**
- `stdev_ns`, `median_ns`, `min_ns`, `max_ns`: Statistical data
- Version info (`python_version`, etc.)

## Running the Full CI Locally

```bash
# Run complete benchmark workflow (like GitHub CI)
./scripts/ci/run_local.sh

# With auto-serve option
./scripts/ci/run_local.sh --serve
```

This runs:
1. Rust Criterion benchmarks (uses existing reports)
2. Python MathHook benchmarks
3. Node.js MathHook benchmarks
6. Generates dashboard + comparison page

**Output:**
```
benchmark-results/
├── rust.json
├── python.json
├── node.json

gh-pages/
├── index.html          # Main dashboard
├── comparison.html     # Hidden comparison page
├── style.css
└── criterion/          # Criterion HTML reports
```

**Color Coding:**
- **Green**: MathHook faster
- **Red**: MathHook slower
- **Gray**: N/A (benchmark not available for that platform)

## Adding a Comparison Platform

To add a new comparison platform (e.g., Mathematica):

### 1. Create Benchmark Script

```python
# crates/mathhook-benchmarks/comparison/bench_mathematica.py
#!/usr/bin/env python3
"""Mathematica Benchmark Suite"""

import json
import time
import statistics
# ... Mathematica-specific imports

def benchmark(func, iterations=100, warmup=10):
    """Run benchmark with statistical analysis."""
    for _ in range(warmup):
        func()

    times = []
    for _ in range(iterations):
        start = time.perf_counter_ns()
        func()
        end = time.perf_counter_ns()
        times.append(end - start)

    return {
        'mean_ns': statistics.mean(times),
        'stdev_ns': statistics.stdev(times) if len(times) > 1 else 0,
        'median_ns': statistics.median(times),
        'min_ns': min(times),
        'max_ns': max(times),
        'iterations': iterations
    }

# Define benchmarks with same IDs as other platforms
benchmarks = {
    'gcd_simple': bench_gcd_simple,
    'parse_large': bench_parse_large,
    # ... use same IDs for matching
}

def run_all_benchmarks(iterations=100):
    results = {
        'platform': 'mathematica',
        'benchmarks': {}
    }
    for name, func in benchmarks.items():
        results['benchmarks'][name] = benchmark(func, iterations)
    return results

if __name__ == '__main__':
    results = run_all_benchmarks()
    print(json.dumps(results, indent=2))
```

### 2. Update run_local.sh

```bash
# Add in scripts/ci/run_local.sh

echo "[X/Y] Running Mathematica benchmarks..."
MATHEMATICA_SCRIPT="crates/mathhook-benchmarks/comparison/bench_mathematica.py"
if [ -f "$MATHEMATICA_SCRIPT" ]; then
    python3 "$MATHEMATICA_SCRIPT" --json --iterations 50 > "$ROOT/benchmark-results/mathematica.json"
fi
```

### 3. Update Dashboard Generator

The dashboard generator auto-discovers platforms from JSON files, so just add handling:

```python
# In scripts/ci/generate_dashboard.py
```

## Best Practices

### Benchmark Naming

**DO:**
- `gcd_simple` - operation_variant
- `parse_large` - operation_size
- `mul_sparse` - operation_type
- `factor_quadratic` - operation_specific

**DON'T:**
- `test_gcd` - don't use "test" prefix
- `MyBenchmark` - don't use CamelCase
- `gcd-simple` - don't use hyphens

### Consistent Operations

Ensure all platforms benchmark the **exact same operation**:

```
# All platforms should compute:
gcd(x^2 - 1, x - 1) = x - 1
```

Not:
```
# Platform A: gcd(x^2 - 1, x - 1)
# Platform B: gcd(x^2 - 1, x + 1)  <- Different!
```

### Warmup and Iterations

- Use 10+ warmup iterations
- Use 50-100 measurement iterations
- More for fast operations, fewer for slow ones

### Error Handling

```python
def bench_gcd_large():
    """GCD that may not be available on all platforms."""
    try:
        return Expression.gcd(f, g)
    except (AttributeError, TypeError) as e:
        return f  # Graceful fallback
```

## Related Documentation

- [Performance Architecture](./architecture.md) - System performance design
- [SIMD Operations](./simd.md) - Vectorization strategies
- [Caching Strategies](./caching.md) - Performance optimization techniques
- [Contributing Guide](../contributing/development.md) - Development workflow
