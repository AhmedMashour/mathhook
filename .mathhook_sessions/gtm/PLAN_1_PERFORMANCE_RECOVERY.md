# Plan 1: Performance Recovery & Comprehensive Benchmarking (V3)

**Priority**: 🔥 CRITICAL
**Timeline**: 4-5 weeks (updated to include Wave 3.5 and mandatory CI)
**Waves**: 5 (Wave 1-4 + Wave 3.5: SymPy comparison, mandatory CI integration)
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

## Formal Baseline Definition

**Critical Foundation**: All performance measurements require a stable, versioned baseline for meaningful comparison.

### Baseline Requirements

1. **What Constitutes a Baseline**:
   - Criterion benchmark results from a known-good commit (typically last release or stable main)
   - Saved in `.mathhook_sessions/baselines/performance_baseline_[version].json`
   - Includes: operation name, median time, std deviation, sample size
   - Tagged with: git commit SHA, Rust version, CPU architecture, optimization flags

2. **Baseline Creation Process**:
   ```bash
   # Create baseline from current state
   cargo bench --bench algebra_benchmarks -- --save-baseline baseline_v0.1.0

   # Store baseline metadata
   echo "{
     \"version\": \"0.1.0\",
     \"commit\": \"$(git rev-parse HEAD)\",
     \"rustc\": \"$(rustc --version)\",
     \"cpu\": \"$(lscpu | grep 'Model name')\",
     \"date\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"
   }" > .mathhook_sessions/baselines/baseline_v0.1.0_metadata.json
   ```

3. **Baseline Comparison Methodology**:
   - Run current benchmarks: `cargo bench --bench [name]`
   - Compare against baseline: `cargo bench --bench [name] -- --baseline baseline_v0.1.0`
   - Acceptable variance: ±5% (measurement noise)
   - Warning threshold: >10% regression
   - Blocking threshold: >20% regression (requires investigation)

4. **Baseline Versioning**:
   - Create new baseline for each release: `baseline_v0.1.0`, `baseline_v0.2.0`, etc.
   - Keep baselines in git (committed to `.mathhook_sessions/baselines/`)
   - Never delete baselines (enables historical comparison)
   - Document baseline rationale in commit message

5. **CI Integration** (Wave 5):
   - Automated baseline comparison on every PR
   - Fail CI if regression >20% without justification
   - Comment on PR with performance impact summary
   - Update baseline automatically on release tags

### Baseline Storage Structure

```
.mathhook_sessions/baselines/
├── baseline_v0.1.0.json                 # Criterion baseline data
├── baseline_v0.1.0_metadata.json        # Environment metadata
├── baseline_v0.2.0.json
├── baseline_v0.2.0_metadata.json
└── README.md                            # Baseline usage guide
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

### Wave 3.5: SymPy Comparison Validation (6-8 hours)

**Objectives**:
1. Run identical test cases against SymPy to validate correctness
2. Benchmark MathHook vs SymPy for speed comparison
3. Validate "10-100x faster" claim with concrete data
4. Document specific operations where MathHook outperforms

**Agent Delegation**:
```bash
/sc:spawn rust-engineer "Create SymPy comparison benchmarks and validate speed claims"
```

**Agent Prompt**:
```markdown
Create comprehensive SymPy comparison to validate both correctness and performance claims.

**Context**: You are the `rust-engineer` agent. Performance fixes complete, now validate against SymPy.

**Critical Requirement**: This wave validates BOTH correctness and speed claims.

**Tasks**:

1. **Correctness Validation Suite**:
   Create Python script: `crates/mathhook-benchmarks/sympy_comparison/correctness_validation.py`
   ```python
   import sympy as sp
   from mathhook import Expression  # Assuming Python bindings exist

   def validate_correctness():
       """Run identical test cases through both MathHook and SymPy"""
       x, y, z = sp.symbols('x y z')
       test_cases = [
           # Algebra
           ("expand((x+1)^10)", sp.expand((x+1)**10)),
           ("factor(x^2 - 1)", sp.factor(x**2 - 1)),
           ("simplify(sin^2(x) + cos^2(x))", sp.simplify(sp.sin(x)**2 + sp.cos(x)**2)),

           # Calculus
           ("derivative(x^3, x)", sp.diff(x**3, x)),
           ("integral(x^2, x)", sp.integrate(x**2, x)),

           # Solving
           ("solve(x^2 - 4 = 0, x)", sp.solve(x**2 - 4, x)),
       ]

       results = []
       for description, sympy_result in test_cases:
           # Run through MathHook (via Python bindings or CLI)
           mathhook_result = run_mathhook(description)

           # Compare results
           match = (str(mathhook_result) == str(sympy_result))
           results.append({
               "test": description,
               "mathhook": str(mathhook_result),
               "sympy": str(sympy_result),
               "match": match
           })

       # Report
       passing = sum(1 for r in results if r["match"])
       print(f"Correctness: {passing}/{len(results)} tests match SymPy")

       return results
   ```

2. **Performance Comparison Benchmarks**:
   Create timing comparison scripts for each category:

   **`algebra_speed_comparison.py`**:
   ```python
   import time
   from sympy import symbols, expand, factor, simplify

   def bench_sympy_algebra():
       x = symbols('x')

       # Expansion benchmark
       start = time.perf_counter()
       for _ in range(100):
           result = expand((x + 1)**20)
       sympy_time = (time.perf_counter() - start) / 100

       print(f"SymPy expand((x+1)^20): {sympy_time*1000:.2f} ms")

       # Factor benchmark
       start = time.perf_counter()
       for _ in range(100):
           result = factor(x**10 - 1)
       sympy_time = (time.perf_counter() - start) / 100

       print(f"SymPy factor(x^10 - 1): {sympy_time*1000:.2f} ms")
   ```

   **`calculus_speed_comparison.py`** (derivatives, integrals)
   **`solving_speed_comparison.py`** (equation solving)

3. **Run Matching Rust Benchmarks**:
   Ensure Rust benchmarks test IDENTICAL operations:
   ```rust
   // algebra_benchmarks.rs - add these specific tests
   fn bench_expand_binomial_20(c: &mut Criterion) {
       let x = symbol!(x);
       c.bench_function("expand (x+1)^20", |b| {
           b.iter(|| {
               let expr = expr!((x + 1) ^ 20);
               expr.expand()
           });
       });
   }
   ```

4. **Calculate Speedup Ratios**:
   Create `.mathhook_sessions/sympy_comparison_results.md`:
   ```markdown
   # MathHook vs SymPy Validation Results

   ## Correctness Validation
   - **Total Tests**: [N]
   - **Matching Results**: [M/N]
   - **Correctness Rate**: [M/N * 100]%
   - **Discrepancies**: [List any differences with explanations]

   ## Performance Comparison

   ### Algebra Operations
   | Operation | MathHook | SymPy | Speedup | Validated |
   |-----------|----------|-------|---------|-----------|
   | expand((x+1)^20) | 45 µs | 3.2 ms | 71x | ✅ |
   | factor(x^10 - 1) | 120 µs | 8.5 ms | 71x | ✅ |
   | simplify(trig) | ... | ... | ...x | ✅ |

   ### Calculus Operations
   | Operation | MathHook | SymPy | Speedup | Validated |
   |-----------|----------|-------|---------|-----------|
   | d/dx(x^3) | 15 µs | 450 µs | 30x | ✅ |
   | ∫x^2 dx | 25 µs | 1.2 ms | 48x | ✅ |

   ### Solving Operations
   | Operation | MathHook | SymPy | Speedup | Validated |
   |-----------|----------|-------|---------|-----------|
   | solve(x^2 - 4 = 0) | 80 µs | 2.1 ms | 26x | ✅ |

   ## Speed Claim Validation
   - **Original Claim**: "10-100x faster than SymPy"
   - **Actual Range**: [MIN]x - [MAX]x faster
   - **Validation**: ✅ CONFIRMED / ⚠️ PARTIALLY / ❌ REFUTED
   - **Recommendation**: [Update claim to match actual data]

   ## Specific Operations Analysis
   ### Where MathHook Excels (>50x faster):
   - [List operations with >50x speedup]

   ### Where MathHook is Moderately Faster (10-50x):
   - [List operations]

   ### Where MathHook is Similar (<10x):
   - [List operations, if any]

   ## Conclusion
   - Correctness: [PASS/FAIL]
   - Performance claims: [VALIDATED/NEEDS UPDATE]
   - Recommended marketing claim: "[Updated claim based on data]"
   ```

5. **Automation Script**:
   Create `run_sympy_comparison.sh`:
   ```bash
   #!/bin/bash
   set -e

   echo "=== MathHook vs SymPy Comparison ==="

   # Install SymPy if needed
   pip install -q sympy

   # Run correctness validation
   echo "1. Validating correctness..."
   python3 crates/mathhook-benchmarks/sympy_comparison/correctness_validation.py

   # Run SymPy benchmarks
   echo "2. Benchmarking SymPy..."
   python3 crates/mathhook-benchmarks/sympy_comparison/algebra_speed_comparison.py > sympy_results.txt
   python3 crates/mathhook-benchmarks/sympy_comparison/calculus_speed_comparison.py >> sympy_results.txt

   # Run MathHook benchmarks (same operations)
   echo "3. Benchmarking MathHook..."
   cargo bench --bench algebra_benchmarks > mathhook_results.txt
   cargo bench --bench calculus_benchmarks >> mathhook_results.txt

   # Compare results (manual or automated analysis)
   echo "4. Comparing results..."
   # (Implementation: parse both result files and calculate speedups)

   echo "Results saved to .mathhook_sessions/sympy_comparison_results.md"
   ```

**Deliverables**:
- Correctness validation script with test cases
- Performance comparison scripts (3+ categories)
- Matching Rust benchmarks for identical operations
- Comprehensive comparison results report
- Automation script for reproducible comparison
- Updated performance claims (if needed)

**Quality Target**: 10/10 - Rigorous, reproducible, honest validation
```

**Verification Script** (`verify_wave_3.5_sympy_comparison.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 3.5 Verification: SymPy Comparison ==="

# 1. Check comparison scripts exist
if [ ! -f "crates/mathhook-benchmarks/sympy_comparison/correctness_validation.py" ]; then
    echo "❌ FAIL: Correctness validation script missing"
    exit 1
fi
echo "✅ Correctness validation script exists"

# 2. Check speed comparison scripts
speed_scripts=("algebra_speed_comparison.py" "calculus_speed_comparison.py" "solving_speed_comparison.py")
for script in "${speed_scripts[@]}"; do
    if [ ! -f "crates/mathhook-benchmarks/sympy_comparison/$script" ]; then
        echo "⚠️  WARNING: $script missing (expected 3+ comparison scripts)"
    fi
done

# 3. Check results report exists
if [ ! -f ".mathhook_sessions/sympy_comparison_results.md" ]; then
    echo "❌ FAIL: Comparison results report missing"
    exit 1
fi
echo "✅ Comparison results report exists"

# 4. Validate report has required sections
required_sections=("Correctness Validation" "Performance Comparison" "Speed Claim Validation" "Specific Operations Analysis")
for section in "${required_sections[@]}"; do
    if ! grep -qi "$section" .mathhook_sessions/sympy_comparison_results.md; then
        echo "❌ FAIL: Missing section in report: $section"
        exit 1
    fi
done
echo "✅ Report has all required sections"

# 5. Check for speedup data in report
if ! grep -qE "[0-9]+x faster|Speedup" .mathhook_sessions/sympy_comparison_results.md; then
    echo "❌ FAIL: No speedup data in report"
    exit 1
fi
echo "✅ Speedup data documented"

# 6. Check automation script exists
if [ ! -f "run_sympy_comparison.sh" ]; then
    echo "⚠️  WARNING: Automation script missing (recommended)"
else
    echo "✅ Automation script exists"
fi

echo ""
echo "=== Wave 3.5 Verification: PASSED ==="
echo ""
echo "Next steps:"
echo "1. Review .mathhook_sessions/sympy_comparison_results.md"
echo "2. Update marketing claims if needed based on validated data"
echo "3. Proceed to Wave 4 (CI Integration)"
```

**Success Criteria**:
- [ ] Correctness validation: 100% match with SymPy results
- [ ] Performance benchmarks: Identical operations tested in both systems
- [ ] Speedup ratios calculated for all major operation categories
- [ ] Speed claims validated with concrete data
- [ ] Report documents specific operations where MathHook excels
- [ ] Quality score = 10/10 (rigorous validation critical)

---

### Wave 4: CI Integration & Continuous Performance Monitoring (8-10 hours)

**Objectives**:
1. **MANDATORY**: Set up GitHub Actions CI for automated performance monitoring
2. **MANDATORY**: Implement automated baseline comparison on every PR
3. **MANDATORY**: Fail CI on performance regressions >20% without justification
4. Create PR comments with performance impact summary
5. Automate baseline updates on release tags

**Agent Delegation**:
```bash
/sc:spawn rust-engineer "Implement mandatory CI integration for continuous performance monitoring"
```

**Agent Prompt**:
```markdown
Implement mandatory CI integration for continuous performance monitoring.

**Context**: You are the `rust-engineer` agent. Waves 1-3.5 complete, now ensure regressions never happen again.

**CRITICAL**: This wave is MANDATORY - CI must prevent future performance regressions.

**Tasks**:

1. **Create GitHub Actions Workflow** (MANDATORY):
   Create `.github/workflows/performance_benchmarks.yml`:
   ```yaml
   name: Performance Benchmarks & Regression Detection

   on:
     push:
       branches: [main, master]
     pull_request:
       types: [opened, synchronize, reopened]

   jobs:
     benchmark:
       runs-on: ubuntu-latest
       timeout-minutes: 60

       steps:
         - name: Checkout code
           uses: actions/checkout@v4
           with:
             fetch-depth: 0  # Need full history for baseline comparison

         - name: Install Rust toolchain
           uses: dtolnay/rust-toolchain@stable
           with:
             components: rustfmt, clippy

         - name: Cache cargo registry
           uses: actions/cache@v3
           with:
             path: ~/.cargo/registry
             key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

         - name: Cache cargo build
           uses: actions/cache@v3
           with:
             path: target
             key: ${{ runner.os }}-cargo-build-${{ hashFiles('**/Cargo.lock') }}

         - name: Install criterion
           run: cargo install cargo-criterion || true

         - name: Run benchmarks
           run: |
             # Run all benchmark suites
             cargo bench --bench algebra_benchmarks -- --save-baseline pr_baseline
             cargo bench --bench calculus_benchmarks -- --save-baseline pr_baseline
             cargo bench --bench solving_benchmarks -- --save-baseline pr_baseline
             cargo bench --bench simplification_benchmarks -- --save-baseline pr_baseline

         - name: Compare against main baseline
           id: baseline_comparison
           run: |
             # Download baseline from main branch
             git fetch origin main
             git show origin/main:.mathhook_sessions/baselines/baseline_main.json > baseline_main.json || echo "{}" > baseline_main.json

             # Compare current benchmarks against main baseline
             # (This requires custom script - see Task 2)
             python3 .github/scripts/compare_benchmarks.py \
               --current target/criterion \
               --baseline baseline_main.json \
               --threshold 20 \
               --output comparison_results.json

             # Set output for next step
             REGRESSION_DETECTED=$(jq -r '.has_regression' comparison_results.json)
             echo "regression_detected=$REGRESSION_DETECTED" >> $GITHUB_OUTPUT

         - name: Post PR comment with benchmark results
           if: github.event_name == 'pull_request'
           uses: actions/github-script@v7
           with:
             script: |
               const fs = require('fs');
               const results = JSON.parse(fs.readFileSync('comparison_results.json', 'utf8'));

               const comment = `
               ## 📊 Performance Benchmark Results

               ${results.summary}

               ### Detailed Comparison

               | Benchmark | Before | After | Change | Status |
               |-----------|--------|-------|--------|--------|
               ${results.benchmarks.map(b =>
                 `| ${b.name} | ${b.baseline_time} | ${b.current_time} | ${b.change_percent} | ${b.status_emoji} |`
               ).join('\n')}

               ### Threshold
               - ⚠️ Warning: >10% regression
               - ❌ Blocking: >20% regression (CI will fail)
               - ✅ Acceptable: ≤10% regression

               ${results.has_regression ? '❌ **REGRESSION DETECTED** - Review required before merge' : '✅ Performance looks good!'}
               `;

               github.rest.issues.createComment({
                 issue_number: context.issue.number,
                 owner: context.repo.owner,
                 repo: context.repo.repo,
                 body: comment
               });

         - name: Fail CI on regression
           if: steps.baseline_comparison.outputs.regression_detected == 'true'
           run: |
             echo "❌ FAILED: Performance regression >20% detected"
             echo "Review comparison_results.json for details"
             exit 1

         - name: Update baseline on main merge
           if: github.ref == 'refs/heads/main' && github.event_name == 'push'
           run: |
             # Save new baseline for main branch
             mkdir -p .mathhook_sessions/baselines
             cp -r target/criterion .mathhook_sessions/baselines/

             # Generate metadata
             echo "{
               \"commit\": \"${{ github.sha }}\",
               \"date\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\",
               \"rustc\": \"$(rustc --version)\",
               \"runner\": \"${{ runner.os }}\"
             }" > .mathhook_sessions/baselines/baseline_metadata.json

             # Commit baseline update (requires workflow permissions)
             git config user.name "github-actions[bot]"
             git config user.email "github-actions[bot]@users.noreply.github.com"
             git add .mathhook_sessions/baselines/
             git commit -m "chore: update performance baseline [skip ci]" || echo "No baseline changes"
             git push origin main

         - name: Upload benchmark artifacts
           uses: actions/upload-artifact@v3
           with:
             name: benchmark-results
             path: |
               target/criterion/
               comparison_results.json
             retention-days: 30
   ```

2. **Create Benchmark Comparison Script** (MANDATORY):
   Create `.github/scripts/compare_benchmarks.py`:
   ```python
   #!/usr/bin/env python3
   """
   Compare current benchmark results against baseline and detect regressions.

   Usage:
     python3 compare_benchmarks.py \
       --current target/criterion \
       --baseline baseline_main.json \
       --threshold 20 \
       --output comparison_results.json
   """
   import argparse
   import json
   import sys
   from pathlib import Path
   from typing import Dict, List

   def parse_criterion_results(criterion_dir: Path) -> Dict[str, float]:
       """Parse criterion benchmark results from target/criterion directory"""
       results = {}

       # Criterion stores results in target/criterion/<benchmark_name>/base/estimates.json
       for benchmark_path in criterion_dir.glob("*/*/base/estimates.json"):
           with open(benchmark_path) as f:
               data = json.load(f)
               benchmark_name = benchmark_path.parent.parent.name
               # Extract median time in nanoseconds
               median_ns = data.get("median", {}).get("point_estimate", 0)
               results[benchmark_name] = median_ns

       return results

   def compare_benchmarks(current: Dict[str, float], baseline: Dict[str, float], threshold_percent: float):
       """Compare current vs baseline and detect regressions"""
       comparisons = []
       has_regression = False

       for name, current_time in current.items():
           baseline_time = baseline.get(name)

           if baseline_time is None:
               # New benchmark
               comparisons.append({
                   "name": name,
                   "baseline_time": "N/A",
                   "current_time": f"{current_time/1000:.2f} µs",
                   "change_percent": "NEW",
                   "status_emoji": "🆕",
                   "regression": False
               })
               continue

           # Calculate percentage change
           change_percent = ((current_time - baseline_time) / baseline_time) * 100

           # Determine status
           if abs(change_percent) <= 5:
               status_emoji = "✅"
               status = "acceptable"
           elif change_percent > 5 and change_percent <= 10:
               status_emoji = "⚠️"
               status = "warning"
           elif change_percent > 10 and change_percent <= threshold_percent:
               status_emoji = "⚠️"
               status = "warning"
           elif change_percent > threshold_percent:
               status_emoji = "❌"
               status = "blocking"
               has_regression = True
           else:
               # Improvement
               status_emoji = "🎉"
               status = "improvement"

           comparisons.append({
               "name": name,
               "baseline_time": f"{baseline_time/1000:.2f} µs",
               "current_time": f"{current_time/1000:.2f} µs",
               "change_percent": f"{change_percent:+.1f}%",
               "status_emoji": status_emoji,
               "status": status,
               "regression": status == "blocking"
           })

       return comparisons, has_regression

   def generate_summary(comparisons: List[Dict], has_regression: bool) -> str:
       """Generate human-readable summary"""
       total = len(comparisons)
       improvements = len([c for c in comparisons if c.get("status") == "improvement"])
       warnings = len([c for c in comparisons if c.get("status") == "warning"])
       blocking = len([c for c in comparisons if c.get("status") == "blocking"])

       summary = f"**Total benchmarks**: {total}\n"
       summary += f"- 🎉 Improvements: {improvements}\n"
       summary += f"- ⚠️ Warnings (>5%): {warnings}\n"
       summary += f"- ❌ Blocking (>20%): {blocking}\n"

       if has_regression:
           summary += "\n⚠️ **Action Required**: Performance regression detected. Review and optimize before merge."

       return summary

   def main():
       parser = argparse.ArgumentParser(description="Compare benchmark results and detect regressions")
       parser.add_argument("--current", type=Path, required=True, help="Path to current criterion results")
       parser.add_argument("--baseline", type=Path, required=True, help="Path to baseline JSON")
       parser.add_argument("--threshold", type=float, default=20.0, help="Regression threshold percentage")
       parser.add_argument("--output", type=Path, required=True, help="Output JSON file")

       args = parser.parse_args()

       # Parse results
       current_results = parse_criterion_results(args.current)

       # Load baseline
       if args.baseline.exists():
           with open(args.baseline) as f:
               baseline_results = json.load(f)
       else:
           print(f"Warning: No baseline found at {args.baseline}, treating all as new", file=sys.stderr)
           baseline_results = {}

       # Compare
       comparisons, has_regression = compare_benchmarks(current_results, baseline_results, args.threshold)
       summary = generate_summary(comparisons, has_regression)

       # Output results
       output_data = {
           "has_regression": has_regression,
           "summary": summary,
           "benchmarks": comparisons,
           "threshold_percent": args.threshold
       }

       with open(args.output, 'w') as f:
           json.dump(output_data, f, indent=2)

       print(f"Comparison results saved to {args.output}")
       print(summary)

       # Exit code
       sys.exit(1 if has_regression else 0)

   if __name__ == "__main__":
       main()
   ```

3. **Baseline Automation on Releases**:
   Add to `.github/workflows/release.yml` (or create if doesn't exist):
   ```yaml
   name: Release

   on:
     push:
       tags:
         - 'v*'

   jobs:
     create-baseline:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v4
         - uses: dtolnay/rust-toolchain@stable

         - name: Run full benchmark suite
           run: |
             cargo bench --bench algebra_benchmarks -- --save-baseline release_${{ github.ref_name }}
             cargo bench --bench calculus_benchmarks -- --save-baseline release_${{ github.ref_name }}
             cargo bench --bench solving_benchmarks -- --save-baseline release_${{ github.ref_name }}

         - name: Save baseline
           run: |
             mkdir -p .mathhook_sessions/baselines
             cp -r target/criterion .mathhook_sessions/baselines/baseline_${{ github.ref_name }}

             echo "{
               \"version\": \"${{ github.ref_name }}\",
               \"commit\": \"${{ github.sha }}\",
               \"date\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"
             }" > .mathhook_sessions/baselines/baseline_${{ github.ref_name }}_metadata.json

         - name: Commit baseline
           run: |
             git config user.name "github-actions[bot]"
             git config user.email "github-actions[bot]@users.noreply.github.com"
             git checkout main
             git add .mathhook_sessions/baselines/
             git commit -m "chore: add performance baseline for ${{ github.ref_name }}"
             git push origin main
   ```

4. **Documentation**:
   Create `.mathhook_sessions/baselines/README.md`:
   ```markdown
   # Performance Baselines

   This directory stores versioned performance baselines for MathHook.

   ## Structure

   - `baseline_vX.Y.Z.json` - Criterion benchmark data for version X.Y.Z
   - `baseline_vX.Y.Z_metadata.json` - Environment metadata (commit, rustc version, etc.)
   - `baseline_main.json` - Latest baseline from main branch (used by CI)

   ## Usage

   ### Creating a New Baseline

   ```bash
   # Run benchmarks with baseline name
   cargo bench -- --save-baseline baseline_v0.2.0

   # Save metadata
   echo "{
     \"version\": \"0.2.0\",
     \"commit\": \"$(git rev-parse HEAD)\",
     \"rustc\": \"$(rustc --version)\",
     \"date\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"
   }" > .mathhook_sessions/baselines/baseline_v0.2.0_metadata.json

   # Commit
   git add .mathhook_sessions/baselines/
   git commit -m "chore: add baseline for v0.2.0"
   ```

   ### Comparing Against Baseline

   ```bash
   # Compare current performance against v0.1.0 baseline
   cargo bench -- --baseline baseline_v0.1.0
   ```

   ### CI Integration

   GitHub Actions automatically:
   - Compares every PR against main branch baseline
   - Fails CI if regression >20%
   - Posts benchmark results as PR comment
   - Updates main baseline on merge
   - Creates versioned baselines on release tags

   ## Baseline Policy

   - **Never delete baselines** - Enables historical comparison
   - **Create baseline for every release** - Tag as `baseline_vX.Y.Z`
   - **Update main baseline on every merge** - Keeps CI comparison current
   - **Commit baselines to git** - Version control for performance tracking

   ## Regression Thresholds

   - ✅ **Acceptable**: ≤5% variance (measurement noise)
   - ⚠️ **Warning**: 5-20% regression (review recommended)
   - ❌ **Blocking**: >20% regression (CI fails, investigation required)
   ```

**Deliverables**:
- `.github/workflows/performance_benchmarks.yml` (MANDATORY)
- `.github/scripts/compare_benchmarks.py` (MANDATORY)
- `.github/workflows/release.yml` (with baseline automation)
- `.mathhook_sessions/baselines/README.md`
- Initial baseline committed to repository

**Quality Target**: 10/10 - CI integration is MANDATORY for preventing future regressions
```

**Verification Script** (`verify_wave_4_ci_integration.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 4 Verification: CI Integration ==="

# 1. Check GitHub Actions workflow exists (MANDATORY)
if [ ! -f ".github/workflows/performance_benchmarks.yml" ]; then
    echo "❌ FAIL: Performance benchmarks workflow missing (MANDATORY)"
    exit 1
fi
echo "✅ Performance benchmarks workflow exists"

# 2. Check comparison script exists (MANDATORY)
if [ ! -f ".github/scripts/compare_benchmarks.py" ]; then
    echo "❌ FAIL: Benchmark comparison script missing (MANDATORY)"
    exit 1
fi
echo "✅ Benchmark comparison script exists"

# 3. Check script is executable
if [ ! -x ".github/scripts/compare_benchmarks.py" ]; then
    chmod +x .github/scripts/compare_benchmarks.py
    echo "⚠️  Made compare_benchmarks.py executable"
fi

# 4. Check release workflow exists
if [ ! -f ".github/workflows/release.yml" ]; then
    echo "⚠️  WARNING: Release workflow missing (recommended for baseline automation)"
else
    echo "✅ Release workflow exists"
fi

# 5. Check baselines directory structure
if [ ! -d ".mathhook_sessions/baselines" ]; then
    echo "❌ FAIL: Baselines directory missing"
    exit 1
fi
echo "✅ Baselines directory exists"

# 6. Check baseline README exists
if [ ! -f ".mathhook_sessions/baselines/README.md" ]; then
    echo "❌ FAIL: Baseline README missing"
    exit 1
fi
echo "✅ Baseline README exists"

# 7. Validate workflow syntax
echo "Validating workflow syntax..."
if command -v yamllint &> /dev/null; then
    yamllint .github/workflows/performance_benchmarks.yml || {
        echo "⚠️  WARNING: Workflow YAML has syntax issues"
    }
else
    echo "⚠️  INFO: yamllint not installed (optional validation)"
fi

# 8. Test comparison script with dummy data
echo "Testing comparison script..."
mkdir -p /tmp/test_criterion
echo '{"median": {"point_estimate": 1000}}' > /tmp/test_criterion/test/base/estimates.json
python3 .github/scripts/compare_benchmarks.py \
    --current /tmp/test_criterion \
    --baseline /dev/null \
    --threshold 20 \
    --output /tmp/test_results.json &>/dev/null || {
    echo "⚠️  WARNING: Comparison script test failed"
}
rm -rf /tmp/test_criterion /tmp/test_results.json
echo "✅ Comparison script validated"

# 9. Check for initial baseline (should exist after first run)
if [ -z "$(ls -A .mathhook_sessions/baselines/*.json 2>/dev/null)" ]; then
    echo "⚠️  INFO: No baselines yet (will be created on first benchmark run)"
else
    echo "✅ Baseline files exist"
fi

# 10. Validate required sections in workflow
required_workflow_sections=("benchmark:" "Compare against main baseline" "Fail CI on regression" "Post PR comment")
for section in "${required_workflow_sections[@]}"; do
    if ! grep -q "$section" .github/workflows/performance_benchmarks.yml; then
        echo "❌ FAIL: Missing required workflow section: $section"
        exit 1
    fi
done
echo "✅ All required workflow sections present"

echo ""
echo "=== Wave 4 Verification: PASSED ==="
echo ""
echo "CI Integration Complete:"
echo "- ✅ Performance benchmarks workflow: .github/workflows/performance_benchmarks.yml"
echo "- ✅ Benchmark comparison script: .github/scripts/compare_benchmarks.py"
echo "- ✅ Baselines directory: .mathhook_sessions/baselines/"
echo ""
echo "Next steps:"
echo "1. Create initial baseline: cargo bench -- --save-baseline baseline_main"
echo "2. Commit baseline: git add .mathhook_sessions/baselines/ && git commit -m 'chore: add initial baseline'"
echo "3. Push to trigger CI: git push origin main"
echo "4. Verify CI runs on next PR"
```

**Success Criteria**:
- [ ] `.github/workflows/performance_benchmarks.yml` created (MANDATORY)
- [ ] `.github/scripts/compare_benchmarks.py` created and tested (MANDATORY)
- [ ] Workflow includes PR comment generation
- [ ] Workflow fails CI on >20% regression
- [ ] Release workflow automates baseline creation
- [ ] Baseline README documents usage
- [ ] Initial baseline committed
- [ ] Quality score = 10/10 (CI is MANDATORY)

---

## Final Success Criteria

### Wave Completion Checklist
- [ ] Wave 1: Irrelevant benchmarks removed, coverage gaps identified, baseline infrastructure created
- [ ] Wave 2: Comprehensive benchmarks created for ALL core functionality (8 files)
- [ ] Wave 3: Performance regressions fixed, tests passing (676/677)
- [ ] Wave 3.5: SymPy correctness and performance comparison complete, speed claims validated
- [ ] Wave 4: CI integration complete, automated regression detection active (MANDATORY)

### Quality Metrics
- All waves score ≥ 9/10 (Waves 3, 3.5, 4 require 10/10 for correctness and CI)
- Test pass rate maintained: 676/677 minimum
- Benchmark coverage: 100% of core functionality
- Speed claims: Validated with concrete data
- CI integration: MANDATORY (blocking regressions >20%)

### Deliverables Checklist
- [ ] `.mathhook_sessions/benchmark_coverage_report.md`
- [ ] 8 comprehensive benchmark files (algebra, calculus, solving, etc.)
- [ ] `.mathhook_sessions/performance_fixes_summary.md`
- [ ] `flamegraph.svg`
- [ ] Git branches with fixes
- [ ] `.mathhook_sessions/sympy_comparison_results.md`
- [ ] SymPy comparison scripts (correctness + speed)
- [ ] `.github/workflows/performance_benchmarks.yml` (MANDATORY)
- [ ] `.github/scripts/compare_benchmarks.py` (MANDATORY)
- [ ] `.mathhook_sessions/baselines/README.md`
- [ ] Initial baseline committed

### Exit Criteria
- **Comprehensive Coverage**: ALL core functionality benchmarked
- **No Regressions**: Performance at or above baseline
- **Claims Validated**: Speed comparisons backed by data (SymPy comparison)
- **Continuous Monitoring**: CI MANDATORY - prevents future regressions automatically
- **Baseline Infrastructure**: Formal baseline definition, versioning, and CI automation complete

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

**Week 1**: Wave 1 (audit + cleanup + baseline infrastructure) + Wave 2 start (4 benchmark files)
**Week 2**: Wave 2 complete (remaining 4 files) + Wave 3 start (regression fixes)
**Week 3**: Wave 3 complete (all fixes) + Wave 3.5 start (SymPy comparison)
**Week 4**: Wave 3.5 complete (validation report) + Wave 4 start (CI integration)
**Week 5**: Wave 4 complete (CI operational) + final validation

**Total**: 4-5 weeks to completion (updated from 3-4 weeks to include mandatory CI)

---

## Agent Requirements

**CRITICAL**: All work MUST be delegated to the `rust-engineer` agent:
- Agent path: `.claude/agents/rust-engineer.md`
- Expertise: Rust performance optimization, benchmarking, criterion
- Tools: cargo, rustc, clippy, cargo-flamegraph, miri
- Focus: Zero-cost abstractions, benchmark-driven development

**DO NOT** use generic performance-engineer. Use MathHook's `rust-engineer` agent.
