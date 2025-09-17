# MathHook Benchmarking Suite

Comprehensive cross-platform performance benchmarking system for MathHook CAS.

## Quick Start

```bash
# Update baselines (single command per platform)
./public/rust/baseline_updater.sh
python public/python/update_baseline.py
node public/node/update_baseline.js

# Compare against baselines
python public/python/compare_baseline.py
node public/node/compare_baseline.js
cargo bench --baseline master

# Run all benchmarks
./ci/run_benchmarks.sh
```

## Documentation

- **[BENCHMARKING.md](BENCHMARKING.md)**: Complete user guide
- **[DIRECTORY_STRUCTURE.md](DIRECTORY_STRUCTURE.md)**: Directory layout and organization
- **[baselines/schema.json](baselines/schema.json)**: JSON baseline format specification

## Directory Overview

```
mathhook-benchmarks/
├── benches/                # Criterion benchmarks (Rust)
├── public/                 # Public cross-platform benchmarks
│   ├── rust/              # Rust baseline scripts
│   ├── python/            # Python benchmarks + baseline mgmt
│   └── node/              # Node.js benchmarks + baseline mgmt
├── comparison/            # External library comparisons (GITIGNORED)
├── baselines/             # Performance baselines (JSON)
├── results/               # Temporary results (GITIGNORED)
├── ci/                    # CI/CD scripts
└── scripts/               # Utility scripts
```

## Benchmarking Philosophy

2. **Single Command**: One command per platform to update baselines
3. **Automatic Metadata**: Git commit, version, system info auto-tagged
4. **Historical Tracking**: Baselines archived with version/commit info
5. **CI-Friendly**: GitHub Actions integration with regression detection

## Platform Support

### Rust (Native)

Uses Criterion for statistical analysis and built-in baseline management.

```bash
cd public/rust
./baseline_updater.sh
cargo bench --baseline master
```

### Python (PyO3 Bindings)

Custom baseline management with JSON storage.

```bash
cd public/python
python update_baseline.py
python compare_baseline.py
```

### Node.js (NAPI Bindings)

Custom baseline management with JSON storage.

```bash
cd public/node
node update_baseline.js
node compare_baseline.js
```

## CI/CD Integration

GitHub Actions workflow: `.github/workflows/benchmark_regression.yml`

**Triggers**:
- Pull requests to main/master
- Manual workflow dispatch

**Default Threshold**: 10% slowdown allowed (configurable)

**Jobs**:
- Rust Criterion benchmarks
- Python baseline comparison
- Node.js baseline comparison
- Aggregate summary

## Benchmark Categories

All platforms benchmark:
- **Parsing**: String → Expression (with/without LaTeX)
- **GCD**: Univariate, multivariate, with content extraction
- **Multiplication**: Dense, sparse, various sizes
- **Division**: Exact division, polynomial long division
- **Expansion**: Power expansion, binomial expansion
- **Simplification**: Algebraic reduction, canonical forms
- **Factorization**: Polynomial factorization

Platform-specific:
- **Rust**: Special polynomials (Legendre, Chebyshev, Hermite, Laguerre)
- **Rust**: Groebner basis computation
- **Rust**: Finite field arithmetic

## Performance Expectations

Typical speedups vs SymPy:

| Operation | Speedup | Notes |
|-----------|---------|-------|
| Parsing | 10-100x | String processing overhead |
| GCD (simple) | 5-20x | Univariate polynomials |
| Multiplication | 10-50x | Dense polynomials |
| Factorization | 5-15x | Polynomial factorization |

Rust (native) is fastest; Python/Node.js have binding overhead (~2-5x slower than Rust).

## Baseline Management

Baselines are JSON files with performance data + git metadata:

```json
{
  "metadata": {
    "timestamp": "2025-11-29T20:00:00Z",
    "git_commit": "abc123d",
    "platform": "python",
    "version": "0.1.0"
  },
  "benchmarks": {
    "parse_simple": {
      "mean_ns": 123456.78,
      "median_ns": 122000.0
    }
  }
}
```

Storage:
- `baselines/<platform>/latest.json`: Current baseline
- `baselines/<platform>/history/`: Historical snapshots

## Migration from Old Structure

If you have existing benchmarks in `cross_platform/`:

```bash
./scripts/migrate_to_new_structure.sh --dry-run
./scripts/migrate_to_new_structure.sh
```

## Comparison Benchmarks

Setup requires external dependencies:

## Contributing

When adding benchmarks:

1. Add to appropriate platform in `public/`
2. Follow naming convention: `bench_<operation>_<size>`
3. Update baseline after implementation
4. Ensure CI passes with new benchmarks
5. Update documentation

## Troubleshooting

See [BENCHMARKING.md - Troubleshooting](BENCHMARKING.md#troubleshooting) for:
- Baseline not found
- Dirty working directory warnings
- Performance variance
- CI failures
- Missing dependencies

## Resources

- [Criterion Documentation](https://bheisler.github.io/criterion.rs/book/)
- [GitHub Actions Workflows](https://docs.github.com/en/actions/using-workflows)
- MathHook Documentation: `../../README.md`

## Last Updated

2025-11-29T2000
