# Plan 1: Performance Recovery & Comprehensive Benchmarking (V2)

**Priority**: 🔥 CRITICAL
**Timeline**: 3-4 weeks
**Waves**: 4 (updated from 3)
**Orchestrator**: `/sc:task`
**Agent**: `rust-engineer` (MathHook project agent)

## Executive Summary

**Critical Finding**: Performance regressions + incomplete benchmark coverage
- ❌ **Regressions**: 30-45% across existing benchmarks
- ❌ **Coverage Gaps**: Missing benchmarks for calculus, solving, educational features
- ⚠️  **Irrelevant Benchmarks**: `symbolica_challenge`, `mathhook_iq_test_suite` don't test core functionality
- ❌ **Unregistered**: `realistic_cas_benchmarks.rs` exists but not in Cargo.toml

**Goal**: Fix regressions + create comprehensive benchmark suite covering ALL core functionality

---

## Bootstrap Command

```bash
/sc:task "Execute Wave-Based Performance Recovery & Comprehensive Benchmarking for MathHook using rust-engineer agent"
```

**Orchestrator Prompt**:

```markdown
You are the Orchestrator for **MathHook Performance Recovery & Comprehensive Benchmarking**.

**Context**: MathHook has 30-45% performance regressions AND incomplete benchmark coverage. Current benchmarks don't cover calculus, solving, or educational features.

**Your Mission**: Execute a 4-wave plan using the MathHook `rust-engineer` agent to fix regressions and create comprehensive benchmarks for ALL core functionality.

**Mandatory Reading** (in this order):
1. `/Users/ahmedmashhour/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md`
2. `/Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md`
3. `/Users/ahmedmashhour/Documents/work/math/mathhook/.claude/agents/rust-engineer.md`
4. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/PLAN_1_PERFORMANCE_RECOVERY_V2.md`

**5 Mandatory Rules**:
1. **You Are Always The Orchestrator** - Never implement directly
2. **Sequential Waves, Parallel Agents** - Complete waves in order
3. **Mandatory Verification** - Each wave ends with verification script
4. **Strict CLAUDE.md Enforcement** - 32-byte Expression, 16-byte Number constraints
5. **Maintain Momentum** - Report after each wave

**Agent to Use**: ALWAYS delegate to `rust-engineer` agent (defined in `.claude/agents/rust-engineer.md`)

**Wave Structure**: 4 waves following 5-phase template

Begin by confirming you understand and reading the mandatory files.
```

---

## Wave Breakdown

### Wave 1: Benchmark Audit & Cleanup (6-8 hours)

**Objectives**:
1. Audit existing benchmarks for relevance to core functionality
2. Remove irrelevant benchmarks (symbolica_challenge, mathhook_iq_test_suite)
3. Register realistic_cas_benchmarks.rs in Cargo.toml
4. Document benchmark coverage gaps

**Agent Delegation**:
```bash
/sc:spawn rust-engineer "Audit MathHook benchmarks and identify coverage gaps"
```

**Agent Prompt**:
```markdown
Audit and clean up MathHook benchmark suite.

**Context**: You are the `rust-engineer` agent for MathHook CAS project.

**Current Benchmarks** (in `crates/mathhook-benchmarks/benches/`):
- ✅ `core_performance.rs` - Basic operations (KEEP, needs expansion)
- ❌ `symbolica_challenge.rs` - Symbolica comparison (REMOVE - not our functionality)
- ❌ `mathhook_iq_test_suite.rs` - IQ test (REMOVE - not core functionality)
- ⚠️  `realistic_cas_benchmarks.rs` - Good but NOT registered in Cargo.toml

**Tasks**:

1. **Remove Irrelevant Benchmarks**:
   ```bash
   # Delete non-core benchmarks
   rm crates/mathhook-benchmarks/benches/symbolica_challenge.rs
   rm crates/mathhook-benchmarks/benches/mathhook_iq_test_suite.rs

   # Remove from Cargo.toml
   # Delete [[bench]] entries for symbolica_challenge and mathhook_iq_test_suite
   ```

2. **Register realistic_cas_benchmarks**:
   ```toml
   # Add to crates/mathhook-benchmarks/Cargo.toml
   [[bench]]
   name = "realistic_cas_benchmarks"
   harness = false
   ```

3. **Audit Current Coverage**:
   Read all remaining benchmarks and document what's covered:
   - Expression operations (add, mul, pow): ✅
   - Polynomial operations: ✅
   - Matrix operations: ✅
   - **MISSING**: Calculus (derivatives, integrals, limits)
   - **MISSING**: Equation solving (linear, quadratic, polynomial, systems)
   - **MISSING**: Simplification strategies
   - **MISSING**: Educational step-by-step generation
   - **MISSING**: LaTeX parsing/formatting
   - **MISSING**: Function evaluation (sin, cos, exp, log, etc.)

4. **Create Coverage Report**:
   Document in `.mathhook_sessions/benchmark_coverage_report.md`:
   - What's currently benchmarked
   - What's missing
   - Prioritized list for Wave 2

**Deliverables**:
- Irrelevant benchmarks deleted
- realistic_cas_benchmarks registered
- Coverage report with gaps identified

**Quality Target**: 9+/10 - Thorough audit, clear gaps identified
```

**Verification Script** (`verify_wave_1_benchmark_audit.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 1 Verification: Benchmark Audit ==="

# 1. Check irrelevant benchmarks removed
if [ -f "crates/mathhook-benchmarks/benches/symbolica_challenge.rs" ]; then
    echo "❌ FAIL: symbolica_challenge.rs still exists"
    exit 1
fi

if [ -f "crates/mathhook-benchmarks/benches/mathhook_iq_test_suite.rs" ]; then
    echo "❌ FAIL: mathhook_iq_test_suite.rs still exists"
    exit 1
fi
echo "✅ Irrelevant benchmarks removed"

# 2. Check realistic_cas_benchmarks registered
if ! grep -q "realistic_cas_benchmarks" crates/mathhook-benchmarks/Cargo.toml; then
    echo "❌ FAIL: realistic_cas_benchmarks not registered"
    exit 1
fi
echo "✅ realistic_cas_benchmarks registered"

# 3. Check coverage report exists
if [ ! -f ".mathhook_sessions/benchmark_coverage_report.md" ]; then
    echo "❌ FAIL: Coverage report not found"
    exit 1
fi
echo "✅ Coverage report created"

# 4. Verify coverage report has required sections
required_sections=("Currently Covered" "Missing Coverage" "Priority List")
for section in "${required_sections[@]}"; do
    if ! grep -qi "$section" .mathhook_sessions/benchmark_coverage_report.md; then
        echo "❌ FAIL: Missing section: $section"
        exit 1
    fi
done
echo "✅ Coverage report complete"

echo ""
echo "=== Wave 1 Verification: PASSED ==="
```

**Success Criteria**:
- [ ] Irrelevant benchmarks removed
- [ ] realistic_cas_benchmarks registered and runs
- [ ] Coverage gaps documented
- [ ] Quality score ≥ 9/10

---

### Wave 2: Comprehensive Core Functionality Benchmarks (12-16 hours)

**Objectives**:
1. Create benchmarks for ALL core mathematical operations
2. Organize by functionality category (algebra, calculus, solving, etc.)
3. Ensure benchmarks test real-world use cases
4. Add SymPy comparison baselines

**Agent Delegation**:
```bash
/sc:spawn rust-engineer "Create comprehensive benchmark suite for all MathHook core functionality"
```

**Agent Prompt**:
```markdown
Create comprehensive benchmarks for all MathHook core functionality.

**Context**: You are the `rust-engineer` agent. Wave 1 identified coverage gaps.

**Reference**: `.mathhook_sessions/benchmark_coverage_report.md` for priorities

**Benchmark Organization** (create separate files per category):

1. **`algebra_benchmarks.rs`**:
   ```rust
   // Expression operations
   - Expression::add (2, 10, 50, 100 terms)
   - Expression::mul (2, 10, 50 terms)
   - Expression::pow (x^2, x^10, x^100)
   - Expression::simplify (various complexity levels)
   - Expression::expand ((x+1)^n for n=2,5,10,20)
   - Expression::factor (factorization of polynomials)
   - Expression::collect (collect like terms)
   ```

2. **`calculus_benchmarks.rs`** (NEW):
   ```rust
   // Derivatives
   - derivative(x^n, x) for n=2,5,10,20
   - derivative(sin(x)*cos(x), x)
   - derivative(nested functions, x)
   - derivative(rational functions, x)

   // Integrals
   - integrate(x^n, x) for n=2,5,10
   - integrate(sin(x), x)
   - integrate(rational functions, x)

   // Limits
   - limit(expression, x, point) various cases

   // Series
   - series(expression, x, point, order)
   ```

3. **`solving_benchmarks.rs`** (NEW):
   ```rust
   // Equation solving
   - solve_linear(a*x + b = 0)
   - solve_quadratic(a*x^2 + b*x + c = 0)
   - solve_polynomial(degree 3, 5, 10)
   - solve_system(2x2, 3x3, 4x4 linear systems)
   - solve_rational_equation
   - solve_matrix_equation (A*X = B)
   ```

4. **`simplification_benchmarks.rs`** (NEW):
   ```rust
   // Simplification strategies
   - simplify_polynomial (various degrees)
   - simplify_rational_expression
   - simplify_trigonometric (sin^2 + cos^2 → 1)
   - simplify_logarithmic (log(a) + log(b) → log(ab))
   - simplify_nested_expression
   ```

5. **`function_evaluation_benchmarks.rs`** (NEW):
   ```rust
   // Elementary functions
   - sin/cos/tan evaluation (symbolic and numeric)
   - exp/log evaluation
   - sqrt evaluation
   - Hyperbolic functions

   // Special functions (if implemented)
   - gamma function
   - bessel functions
   ```

6. **`educational_benchmarks.rs`** (NEW):
   ```rust
   // Educational features
   - generate_step_by_step_solution (various problem types)
   - explain_simplification
   - explain_solving
   - message_registry_lookup
   ```

7. **`parsing_benchmarks.rs`** (NEW):
   ```rust
   // Parsing
   - parse_latex (simple, complex expressions)
   - parse_standard_notation
   - format_to_latex
   - format_to_wolfram
   ```

8. **`matrix_benchmarks.rs`** (expand existing):
   ```rust
   // Matrix operations
   - matrix_add (2x2, 4x4, 8x8, 16x16)
   - matrix_mul (2x2, 4x4, 8x8)
   - matrix_determinant (2x2, 3x3, 4x4)
   - matrix_inverse (2x2, 3x3, 4x4)
   - matrix_eigenvalues (if implemented)
   ```

**Implementation Pattern** (example):
```rust
// algebra_benchmarks.rs
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use mathhook_core::{Expression, Symbol, Simplify};

fn bench_expression_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("expression_operations");

    // Test addition with varying term counts
    for count in [2, 10, 50, 100].iter() {
        let terms: Vec<Expression> = (1..=*count)
            .map(|i| Expression::integer(i as i64))
            .collect();

        group.bench_with_input(
            BenchmarkId::new("add", count),
            count,
            |b, _| b.iter(|| Expression::add(terms.clone()))
        );
    }

    group.finish();
}

criterion_group!(benches, bench_expression_operations);
criterion_main!(benches);
```

**Register All Benchmarks**:
```toml
# Add to Cargo.toml
[[bench]]
name = "algebra_benchmarks"
harness = false

[[bench]]
name = "calculus_benchmarks"
harness = false

[[bench]]
name = "solving_benchmarks"
harness = false

[[bench]]
name = "simplification_benchmarks"
harness = false

[[bench]]
name = "function_evaluation_benchmarks"
harness = false

[[bench]]
name = "educational_benchmarks"
harness = false

[[bench]]
name = "parsing_benchmarks"
harness = false

[[bench]]
name = "matrix_benchmarks"
harness = false
```

**Deliverables**:
- 8 comprehensive benchmark files
- All registered in Cargo.toml
- All benchmarks compile and run
- Coverage for ALL core functionality

**Quality Target**: 9+/10 - Comprehensive, realistic, well-organized
```

**Verification Script** (`verify_wave_2_comprehensive_benchmarks.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 2 Verification: Comprehensive Benchmarks ==="

# 1. Check all benchmark files exist
benchmarks=("algebra_benchmarks" "calculus_benchmarks" "solving_benchmarks" "simplification_benchmarks" "function_evaluation_benchmarks" "educational_benchmarks" "parsing_benchmarks" "matrix_benchmarks")

for bench in "${benchmarks[@]}"; do
    if [ ! -f "crates/mathhook-benchmarks/benches/${bench}.rs" ]; then
        echo "❌ FAIL: ${bench}.rs not found"
        exit 1
    fi
done
echo "✅ All 8 benchmark files exist"

# 2. Check all registered in Cargo.toml
for bench in "${benchmarks[@]}"; do
    if ! grep -q "name = \"$bench\"" crates/mathhook-benchmarks/Cargo.toml; then
        echo "❌ FAIL: $bench not registered in Cargo.toml"
        exit 1
    fi
done
echo "✅ All benchmarks registered"

# 3. Compile all benchmarks
echo "Compiling benchmarks..."
cargo bench --no-run -p mathhook-benchmarks --quiet
if [ $? -ne 0 ]; then
    echo "❌ FAIL: Benchmarks don't compile"
    exit 1
fi
echo "✅ All benchmarks compile"

# 4. Run quick smoke test (1 iteration each)
echo "Running smoke tests..."
for bench in "${benchmarks[@]}"; do
    cargo bench --bench $bench -- --test --quick &>/dev/null
    if [ $? -ne 0 ]; then
        echo "❌ FAIL: $bench smoke test failed"
        exit 1
    fi
    echo "  ✓ $bench works"
done
echo "✅ All benchmarks run successfully"

echo ""
echo "=== Wave 2 Verification: PASSED ==="
```

**Success Criteria**:
- [ ] 8 comprehensive benchmark files created
- [ ] All registered in Cargo.toml
- [ ] All compile and run
- [ ] Cover ALL core functionality
- [ ] Quality score ≥ 9/10

---

### Wave 3: Regression Fixes & Optimization (12-16 hours)

**Objectives**:
1. Fix 30-45% performance regressions identified in existing benchmarks
2. Profile hot paths with flamegraph
3. Optimize without breaking mathematical correctness
4. Validate improvements with before/after benchmarks

**Agent Delegation**:
```bash
/sc:spawn rust-engineer "Fix MathHook performance regressions using profiling and optimization"
```

**Agent Prompt**:
```markdown
Fix performance regressions in MathHook.

**Context**: You are the `rust-engineer` agent. Benchmarks show 30-45% regressions.

**Input**: Current benchmark results showing regressions in:
- basic_solving: +36,203%
- polynomial_creation: +37.750%
- polynomial_simplification: +44.974%
- expression_size_verification: +15.562%

**Tasks**:

1. **Profile Hot Paths**:
   ```bash
   cargo install flamegraph
   cargo flamegraph --bench core_performance
   # Analyze flamegraph.svg for bottlenecks
   ```

2. **Git Blame Analysis**:
   ```bash
   # Find commits introducing regressions
   git log --oneline --since="3 months ago" crates/mathhook-core/src/
   # For each hot path file, check recent changes
   git blame <hot_path_file>
   ```

3. **Fix Regressions** (one at a time):
   - Create branch: `git checkout -b fix/performance-regression-[issue]`
   - Implement fix targeting root cause
   - **CRITICAL**: Run tests after EVERY fix: `cargo test -p mathhook-core`
   - **CRITICAL**: Verify 676/677 tests still pass (mathematical correctness first!)
   - Benchmark before/after: `cargo bench --bench core_performance`
   - Document fix rationale in commit message
   - Repeat for each regression

4. **Optimization Strategies** (from CLAUDE.md):
   - Use `#[inline]` for small hot functions (<10 lines)
   - Avoid allocations in tight loops
   - Use arena allocation for bulk operations
   - Consider SIMD for vectorizable operations (matrix ops, polynomial eval)
   - Profile after each optimization

5. **Document Fixes**:
   Create `.mathhook_sessions/performance_fixes_summary.md`:
   - Root cause for each regression
   - Fix implemented
   - Before/after benchmark data
   - Test results (must be 676/677 passing)

**Constraints** (CRITICAL from CLAUDE.md):
- **Mathematical Correctness First**: NEVER sacrifice correctness for speed
- **Zero Tolerance for Test Regressions**: 676/677 tests MUST still pass
- **32-byte Expression constraint**: Don't add fields to Expression type
- **16-byte Number constraint**: Don't modify Number size
- **Measure Everything**: Benchmark before AND after each optimization

**Deliverables**:
- Git branches with fixes (one per regression)
- Flamegraph analysis
- Before/after benchmark comparisons
- Test suite results (all passing)
- Performance fixes summary

**Quality Target**: 10/10 - Correct, measured, documented, no test regressions
```

**Verification Script** (`verify_wave_3_regression_fixes.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 3 Verification: Regression Fixes ==="

# 1. Run full test suite
echo "Running test suite..."
cargo test -p mathhook-core --quiet 2>&1 | tee test_results.txt

passing=$(grep -oP '\d+(?= passed)' test_results.txt || echo "0")
if [ "$passing" -lt 676 ]; then
    echo "❌ FAIL: Only $passing tests passing (need ≥676)"
    exit 1
fi
echo "✅ Test suite: $passing/677 tests passing"

# 2. Run benchmarks and check for improvements
echo "Running benchmarks..."
cargo bench --bench core_performance 2>&1 | tee bench_results.txt

# 3. Check for performance improvements (should not have massive regressions)
if grep -q "change:.*+3[0-9][0-9][0-9][0-9]%" bench_results.txt; then
    echo "⚠️  WARNING: Still showing >30,000% regression"
    echo "Check bench_results.txt for details"
fi

if grep -q "improved" bench_results.txt; then
    echo "✅ Performance improvements detected"
else
    echo "⚠️  WARNING: No improvements detected - check if fixes applied"
fi

# 4. Check flamegraph generated
if [ ! -f "flamegraph.svg" ]; then
    echo "⚠️  WARNING: Flamegraph not found (optional)"
else
    echo "✅ Flamegraph analysis completed"
fi

# 5. Check fixes documented
if [ ! -f ".mathhook_sessions/performance_fixes_summary.md" ]; then
    echo "❌ FAIL: Performance fixes summary not found"
    exit 1
fi
echo "✅ Fixes documented"

# 6. Check git branches created
fix_branches=$(git branch | grep "fix/performance-regression" | wc -l)
if [ "$fix_branches" -lt 1 ]; then
    echo "⚠️  WARNING: No fix branches created"
else
    echo "✅ Fix branches: $fix_branches"
fi

echo ""
echo "=== Wave 3 Verification: PASSED ==="
echo ""
echo "Performance Status:"
echo "- Tests passing: $passing/677"
echo "- Fix branches: $fix_branches"
echo "- Review bench_results.txt for detailed performance data"
```

**Success Criteria**:
- [ ] All regressions analyzed with flamegraph
- [ ] Fixes implemented with before/after data
- [ ] Test suite passes (676/677 minimum)
- [ ] Performance improvements documented
- [ ] Quality score = 10/10 (correctness critical)

---

### Wave 4: SymPy Comparison & Performance Validation (8-10 hours)

**Objectives**:
1. Create Python scripts benchmarking SymPy for same operations
2. Compare MathHook vs SymPy performance
3. Validate "10-100x faster than SymPy" claim
4. Set up CI for continuous performance monitoring

**Agent Delegation**:
```bash
/sc:spawn rust-engineer "Create SymPy comparison benchmarks and validate speed claims"
```

**Agent Prompt**:
```markdown
Create SymPy comparison and validate MathHook speed claims.

**Context**: You are the `rust-engineer` agent. Performance fixes complete, now validate claims.

**Tasks**:

1. **Create SymPy Comparison Scripts**:
   ```python
   # crates/mathhook-benchmarks/sympy_comparison/algebra_comparison.py
   import time
   from sympy import symbols, simplify, expand, factor

   def bench_sympy_operations():
       x = symbols('x')

       # Expression addition (match algebra_benchmarks.rs)
       start = time.perf_counter()
       for _ in range(1000):
           result = sum([x + i for i in range(100)])
       end = time.perf_counter()
       print(f"SymPy add (100 terms): {(end - start) * 1000:.2f} ms")

       # ... more benchmarks matching Rust ones

   if __name__ == "__main__":
       bench_sympy_operations()
   ```

   Create scripts for:
   - `algebra_comparison.py`
   - `calculus_comparison.py`
   - `solving_comparison.py`
   - `simplification_comparison.py`

2. **Run Comparisons**:
   ```bash
   # Install SymPy
   pip install sympy

   # Run Rust benchmarks
   cargo bench --bench algebra_benchmarks > rust_results.txt

   # Run SymPy benchmarks
   python sympy_comparison/algebra_comparison.py > sympy_results.txt

   # Compare results
   ```

3. **Calculate Speedup Ratios**:
   Create `.mathhook_sessions/performance_validation_report.md`:
   ```markdown
   # MathHook vs SymPy Performance Comparison

   ## Algebra Operations
   | Operation | MathHook | SymPy | Speedup |
   |-----------|----------|-------|---------|
   | Add (100 terms) | 26 µs | 2.3 ms | 88x faster |
   | Multiply (50 terms) | ... | ... | ...x faster |

   ## Calculus Operations
   | Operation | MathHook | SymPy | Speedup |
   |-----------|----------|-------|---------|
   | Derivative (x^10) | ... | ... | ...x faster |

   ## Overall Performance
   - **Claim**: "10-100x faster than SymPy"
   - **Validated**: YES / NO / PARTIALLY
   - **Actual Range**: Xx - Yx faster
   - **Recommendation**: Update claim to accurate range
   ```

4. **CI Integration** (optional but recommended):
   ```yaml
   # .github/workflows/benchmarks.yml
   name: Performance Benchmarks

   on:
     push:
       branches: [main, master]
     pull_request:

   jobs:
     benchmark:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v3
         - uses: dtolnay/rust-toolchain@stable

         - name: Run benchmarks
           run: cargo bench --bench algebra_benchmarks

         - name: Check for regressions
           run: |
             # Fail if performance regresses >10%
             # (Implementation details TBD)
   ```

**Deliverables**:
- SymPy comparison scripts (4+ files)
- Performance validation report with speedup ratios
- Honest assessment of speed claims
- CI workflow (optional)

**Quality Target**: 9+/10 - Rigorous, honest, reproducible
```

**Verification Script** (`verify_wave_4_sympy_comparison.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 4 Verification: SymPy Comparison ==="

# 1. Check SymPy comparison scripts exist
comparison_scripts=("algebra_comparison.py" "calculus_comparison.py" "solving_comparison.py" "simplification_comparison.py")
for script in "${comparison_scripts[@]}"; do
    if [ ! -f "crates/mathhook-benchmarks/sympy_comparison/$script" ]; then
        echo "❌ FAIL: $script not found"
        exit 1
    fi
done
echo "✅ All SymPy comparison scripts exist"

# 2. Check SymPy installed
python3 -c "import sympy" 2>/dev/null
if [ $? -ne 0 ]; then
    echo "⚠️  WARNING: SymPy not installed (pip install sympy)"
else
    echo "✅ SymPy available"
fi

# 3. Check performance validation report
if [ ! -f ".mathhook_sessions/performance_validation_report.md" ]; then
    echo "❌ FAIL: Performance validation report not found"
    exit 1
fi
echo "✅ Performance validation report exists"

# 4. Check report has speedup data
if ! grep -q "Speedup" .mathhook_sessions/performance_validation_report.md; then
    echo "❌ FAIL: No speedup data in report"
    exit 1
fi
echo "✅ Speedup data documented"

# 5. Check CI workflow (optional)
if [ -f ".github/workflows/benchmarks.yml" ]; then
    echo "✅ CI workflow created"
else
    echo "⚠️  INFO: CI workflow not created (optional)"
fi

echo ""
echo "=== Wave 4 Verification: PASSED ==="
```

**Success Criteria**:
- [ ] SymPy comparison scripts created and run
- [ ] Speedup ratios calculated for all operation categories
- [ ] Speed claims validated or revised based on data
- [ ] Performance validation report complete
- [ ] Quality score ≥ 9/10

---

## Final Success Criteria

### Wave Completion Checklist
- [ ] Wave 1: Irrelevant benchmarks removed, coverage gaps identified
- [ ] Wave 2: Comprehensive benchmarks created for ALL core functionality (8 files)
- [ ] Wave 3: Performance regressions fixed, tests passing (676/677)
- [ ] Wave 4: SymPy comparison complete, speed claims validated

### Quality Metrics
- All waves score ≥ 9/10 (Wave 3 requires 10/10 for correctness)
- Test pass rate maintained: 676/677 minimum
- Benchmark coverage: 100% of core functionality
- Speed claims: Validated with evidence

### Deliverables Checklist
- [ ] `.mathhook_sessions/benchmark_coverage_report.md`
- [ ] 8 comprehensive benchmark files (algebra, calculus, solving, etc.)
- [ ] `.mathhook_sessions/performance_fixes_summary.md`
- [ ] `flamegraph.svg`
- [ ] Git branches with fixes
- [ ] SymPy comparison scripts (4+)
- [ ] `.mathhook_sessions/performance_validation_report.md`
- [ ] CI workflow (optional)

### Exit Criteria
- **Comprehensive Coverage**: ALL core functionality benchmarked
- **No Regressions**: Performance at or above baseline
- **Claims Validated**: Speed comparisons backed by data
- **Continuous Monitoring**: CI prevents future regressions

---

## Benchmark Coverage Summary (Target)

| Category | Benchmarks | Coverage |
|----------|-----------|----------|
| **Algebra** | Expression ops, polynomial ops, matrix ops | 100% |
| **Calculus** | Derivatives, integrals, limits, series | 100% |
| **Solving** | Linear, quadratic, polynomial, systems, matrix equations | 100% |
| **Simplification** | Polynomial, rational, trigonometric, logarithmic | 100% |
| **Functions** | Elementary functions, special functions | 100% |
| **Educational** | Step-by-step generation, explanations | 100% |
| **Parsing** | LaTeX, standard notation, formatting | 100% |
| **Matrix** | Operations, determinant, inverse, eigenvalues | 100% |

**Total**: 8 benchmark files covering 100% of core functionality

---

## Timeline

**Week 1**: Wave 1 (audit + cleanup) + Wave 2 start (4 benchmark files)
**Week 2**: Wave 2 complete (remaining 4 files) + Wave 3 start (regression fixes)
**Week 3**: Wave 3 complete (all fixes) + Wave 4 start (SymPy comparison)
**Week 4**: Wave 4 complete (validation report) + CI setup

**Total**: 3-4 weeks to completion

---

## Agent Requirements

**CRITICAL**: All work MUST be delegated to the `rust-engineer` agent:
- Agent path: `.claude/agents/rust-engineer.md`
- Expertise: Rust performance optimization, benchmarking, criterion
- Tools: cargo, rustc, clippy, cargo-flamegraph, miri
- Focus: Zero-cost abstractions, benchmark-driven development

**DO NOT** use generic performance-engineer. Use MathHook's `rust-engineer` agent.
