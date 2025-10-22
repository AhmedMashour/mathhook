# Benchmark CI Integration

This directory contains the benchmark baseline and CI integration for automated performance monitoring.

## Overview

The MathHook project uses **automated benchmark comparison in CI** to prevent performance regressions. Every pull request is automatically benchmarked against a baseline, and CI fails if performance degrades by more than 20%.

## Files

- **`baseline.json`**: Current performance baseline (105 benchmarks)
  - Updated automatically on main/master branch merges
  - Contains mean, std_dev, and median for each benchmark
  - Tracked in Git for historical analysis

- **`comparison.md`**: Generated comparison report (temporary, in PR workflow)
  - Created during PR benchmark runs
  - Posted as PR comment
  - Not committed to repository

## Benchmark Categories

The baseline tracks 105 benchmarks across:

1. **Core Operations** (20 benchmarks)
   - Expression construction
   - Simplification
   - Evaluation

2. **Calculus** (27 benchmarks)
   - Derivatives (14)
   - Integrals (13)

3. **Algebra** (25 benchmarks)
   - Solving
   - Factorization
   - Expansion

4. **Parsing** (15 benchmarks)
   - LaTeX parsing
   - Implicit multiplication
   - Expression formatting

5. **Advanced** (18 benchmarks)
   - Matrix operations
   - SIMD bulk operations
   - Memoization

## CI Workflow

### Pull Requests

When you create a PR:

1. CI runs all benchmarks
2. Compares results against `baseline.json`
3. Posts comparison table as PR comment
4. **Fails CI if any benchmark regresses >20%**

### Main Branch Merges

When PR merges to main/master:

1. CI runs all benchmarks
2. Exports results to `baseline.json`
3. Commits updated baseline to repository
4. New baseline becomes reference for future PRs

## Understanding Results

PR comments show results like this:

```markdown
## Benchmark Results

✅ **No significant performance regressions**

| Benchmark | Baseline | Current | Change | Status |
|-----------|----------|---------|--------|--------|
| polynomial_simplification | 5.71 µs | 4.89 µs | -14.4% | 🟢 improvement |
| expression_creation | 516.26 ns | 512.33 ns | -0.8% | ⚪ unchanged |
| derivative_chain_rule | 3.21 µs | 3.89 µs | +21.2% | 🔴 regression |
```

### Status Meanings

- 🟢 **Improvement**: >5% faster than baseline
- ⚪ **Unchanged**: <5% change from baseline
- 🟡 **Minor change**: 5-20% change from baseline
- 🔴 **Regression**: >20% slower (FAILS CI)
- 🆕 **New**: Benchmark not in baseline

## Local Testing

### Run Benchmarks

```bash
cargo bench -p mathhook-benchmarks
```

### Generate Baseline

```bash
python3 scripts/export_baseline.py \
  --input target/criterion \
  --output benchmarks/baseline.json
```

### Compare with Baseline

```bash
python3 scripts/compare_benchmarks.py \
  --baseline benchmarks/baseline.json \
  --current target/criterion \
  --threshold 20 \
  --output /tmp/comparison.md

cat /tmp/comparison.md
```

## Fixing Regressions

If CI fails due to regression:

1. **Identify the problem**:
   - Check which benchmark regressed
   - Look at the percentage change
   - Consider if the change is expected

2. **Profile the code**:
   ```bash
   cargo bench -p mathhook-benchmarks -- <benchmark_name>
   cargo flamegraph --bench <benchmark_file>
   ```

3. **Optimize**:
   - Use profiling to find hot spots
   - Apply targeted optimizations
   - Re-run benchmarks to verify improvement

4. **Justify or fix**:
   - If regression is justified (e.g., correctness fix), document in PR
   - If unjustified, optimize until regression is eliminated
   - Consider adjusting threshold for specific benchmarks if needed

## Adding New Benchmarks

1. Add benchmark to `crates/mathhook-benchmarks/benches/`
2. Run `cargo bench` locally to test
3. PR will show benchmark as "NEW" (🆕)
4. After merge, benchmark is included in baseline
5. Future PRs compare against this benchmark

## Maintenance

### Updating Baseline Manually (Rare)

Only needed for major architectural changes:

```bash
cargo bench -p mathhook-benchmarks
python3 scripts/export_baseline.py \
  --input target/criterion \
  --output benchmarks/baseline.json
git add benchmarks/baseline.json
git commit -m "Update benchmark baseline after [reason]"
```

### Threshold Adjustment

Current threshold: **20% regression triggers CI failure**

To adjust for specific benchmarks:
1. Modify `scripts/compare_benchmarks.py`
2. Add per-benchmark threshold configuration
3. Document the change in PR

## Configuration

### Environment Variables

- `CARGO_TERM_COLOR`: Color output (default: `always`)
- `RUST_BACKTRACE`: Backtrace on errors (default: `1`)

### CI Workflow

Located in `.github/workflows/benchmark.yml`

Key settings:
- Trigger: PRs and main/master pushes
- Threshold: 20% regression
- Retention: 30 days for artifacts

## Troubleshooting

### Baseline Missing

If baseline.json is missing:
```bash
cargo bench -p mathhook-benchmarks
python3 scripts/export_baseline.py \
  --input target/criterion \
  --output benchmarks/baseline.json
```

### CI Failing on First PR

First PR after CI setup may not have a baseline:
- CI creates empty baseline automatically
- All benchmarks marked as "NEW"
- Baseline updated after merge

### Inconsistent Results

Benchmark results can vary due to:
- System load during CI
- Different CPU architectures
- Thermal throttling

Mitigation:
- 20% threshold accounts for variance
- Run benchmarks multiple times
- Use dedicated CI runners for stability

## References

- GitHub Actions Workflow: `.github/workflows/benchmark.yml`
- Comparison Script: `scripts/compare_benchmarks.py`
- Export Script: `scripts/export_baseline.py`
- Verification: `scripts/verify_wave4.sh`
- Documentation: `.mathhook_sessions/gtm/wave4/WAVE4_COMPLETION_REPORT.md`

## Questions?

For issues with CI or benchmarks:
1. Check the completion report for detailed documentation
2. Run `./scripts/verify_wave4.sh` to test setup
3. Review GitHub Actions logs for errors
4. Consult `.mathhook_sessions/gtm/wave4/` for implementation details
