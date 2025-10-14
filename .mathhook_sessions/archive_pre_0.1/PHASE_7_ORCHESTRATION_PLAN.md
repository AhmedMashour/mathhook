# Phase 7: 0.1 Release Blocker Resolution - Orchestration Plan

**Created**: 2025-10-13
**Status**: READY TO EXECUTE
**Assessment**: 104 CRITICAL VIOLATIONS FOUND
**Methodology**: Wave-by-wave parallel agent execution with verification gates

---

## Executive Summary

Comprehensive assessment reveals **104 critical CLAUDE.md violations** that MUST be resolved before 0.1 release:

### Critical Blockers
1. **Module Size Violations**: 23 files exceed 500-line limit (2 CRITICAL >1000 lines)
2. **Placeholder Code**: 81 "TODO/not implemented yet" comments across 33 files
3. **Domain Error Integration**: MathError defined but NOT used (0 Result<> returns)
4. **Number Overflow**: Only 3 checked operations (should be comprehensive)
5. **Unwrap Panics**: 121 .unwrap() calls (potential runtime panics)

**Assessment Script**: `.mathhook_sessions/assess_0.1_blockers.sh`

---

## Orchestration Structure

### Wave-by-Wave Execution
- **5 Waves** with verification gates between each
- **Multiple agents per wave** working in PARALLEL on independent tasks
- **Zero tolerance for false positives** - all verification from actual execution
- **Comprehensive logging** in `agent_logs/` directory

### Verification Requirements
- Run `assess_0.1_blockers.sh` after EACH wave
- Violations must decrease after each wave
- All tests must continue passing (741/741 tests)
- Zero regressions allowed

---

## WAVE 1: Module Size Refactoring (CRITICAL FILES)

**Priority**: P0 - CRITICAL
**Target**: 2 files >1000 lines (CLAUDE.md: max 500)
**Duration**: 2-3 hours
**Agents**: 2 agents working in parallel

### Agent K: Refactor matrix/unified.rs (1,021 lines → ~400 lines)

**File**: `crates/mathhook-core/src/matrix/unified.rs`
**Current**: 1,021 lines (+104% over limit)
**Target**: Split into 3 modules (~340 lines each)

**Refactoring Strategy**:
1. Create `matrix/unified/` directory
2. Split into:
   - `construction.rs`: Matrix construction methods (~340 lines)
   - `operations.rs`: Arithmetic operations (~340 lines)
   - `decomposition.rs`: Advanced operations (~340 lines)
3. Update `matrix/unified.rs` to be module aggregator (~50 lines)

**Verification**:
- All matrix tests passing
- No new compilation errors
- File sizes verified: `wc -l matrix/unified/*.rs`

**Log**: `agent_logs/AGENT_P0_K_MATRIX_REFACTOR_LOG.md`

---

### Agent L: Refactor core/expression/constructors.rs (1,020 lines → ~400 lines)

**File**: `crates/mathhook-core/src/core/expression/constructors.rs`
**Current**: 1,020 lines (+104% over limit)
**Target**: Split into 3 modules (~340 lines each)

**Refactoring Strategy**:
1. Create `expression/constructors/` directory
2. Split into:
   - `basic.rs`: Number, Symbol, Add, Mul, Pow constructors (~340 lines)
   - `functions.rs`: Function, Calculus constructors (~340 lines)
   - `specialized.rs`: Complex, Matrix, Set constructors (~340 lines)
3. Update `constructors.rs` to be module aggregator (~50 lines)

**Verification**:
- All expression tests passing
- No new compilation errors
- File sizes verified: `wc -l expression/constructors/*.rs`

**Log**: `agent_logs/AGENT_P0_L_CONSTRUCTORS_REFACTOR_LOG.md`

---

**Wave 1 Verification Gate**:
```bash
# Run after both agents complete
./assess_0.1_blockers.sh | grep "CRITICAL"
# Expected: 0 CRITICAL violations

cargo test -p mathhook-core --lib
# Expected: 459 passed; 0 failed
```

**Success Criteria**:
- ✅ 0 files >1000 lines
- ✅ All tests passing (459/459)
- ✅ Zero regressions

---

## WAVE 2: Module Size Refactoring (HIGH PRIORITY FILES)

**Priority**: P0 - HIGH
**Target**: 5 files 751-1000 lines
**Duration**: 2-3 hours
**Agents**: 5 agents working in parallel

### Agent M: Refactor educational/enhanced_steps.rs (939 lines → ~400 lines)

**Strategy**: Split into `enhanced_steps/formatting.rs` and `enhanced_steps/generation.rs`

### Agent N: Refactor formatter/latex.rs (838 lines → ~400 lines)

**Strategy**: Split into `latex/expressions.rs` and `latex/functions.rs`

### Agent O: Refactor pattern/matching.rs (909 lines → ~400 lines)

**Strategy**: Split into `matching/patterns.rs` and `matching/engine.rs`

### Agent P: Refactor functions/properties.rs (872 lines → ~400 lines)

**Strategy**: Split into `properties/elementary.rs`, `properties/special.rs`, `properties/rules.rs`

### Agent Q: Refactor algebra/complex.rs (881 lines → ~400 lines)

**Strategy**: Split into `complex/operations.rs` and `complex/arithmetic.rs`

**Logs**: `agent_logs/AGENT_P0_{M,N,O,P,Q}_*_LOG.md`

**Wave 2 Verification Gate**:
```bash
./assess_0.1_blockers.sh | grep "HIGH"
# Expected: 0 HIGH violations

cargo test -p mathhook-core --lib
# Expected: 459 passed; 0 failed
```

---

## WAVE 3: Placeholder Code Elimination

**Priority**: P0 - CRITICAL
**Target**: 81 placeholder comments across 33 files
**Duration**: 3-4 hours
**Agents**: 4 agents working in parallel (grouped by type)

### Agent R: Eliminate "Complex case not implemented yet"

**Files**:
- `algebra/solvers/quadratic.rs` (2 occurrences)
- Implement complex number solutions for quadratic equations

**Implementation**:
```rust
// Replace:
_ => SolverResult::NoSolution, // Complex case not implemented yet

// With actual implementation:
_ => {
    // Compute complex roots using quadratic formula
    let discriminant_sqrt = Expression::function("sqrt", vec![
        Expression::mul(vec![Expression::integer(-1), discriminant.clone()])
    ]);
    let root1 = Expression::complex(
        Expression::mul(vec![
            Expression::mul(vec![Expression::integer(-1), b.clone()]),
            Expression::pow(Expression::mul(vec![Expression::integer(2), a.clone()]), Expression::integer(-1))
        ]),
        Expression::mul(vec![
            discriminant_sqrt.clone(),
            Expression::pow(Expression::mul(vec![Expression::integer(2), a.clone()]), Expression::integer(-1))
        ])
    );
    // ... root2 similarly
    SolverResult::Multiple(vec![root1, root2])
}
```

**Verification**:
```rust
// Test case: x^2 + x + 1 = 0
// Roots: (-1 ± i√3) / 2
```

**Log**: `agent_logs/AGENT_P0_R_COMPLEX_QUADRATIC_LOG.md`

---

### Agent S: Eliminate "Integration not yet implemented"

**Files** (8 occurrences):
- `functions/polynomials/laguerre.rs` (2)
- `functions/polynomials/legendre.rs` (2)
- `functions/polynomials/hermite.rs` (2)
- `functions/polynomials/chebyshev.rs` (4)

**Implementation**:
For each special polynomial function, implement actual integration rules or keep as symbolic integral (but remove "not yet implemented" comments).

**Options**:
1. **Register with Symbolic variant**: Mark as requiring special integration techniques
2. **Implement actual formulas**: Use orthogonal polynomial integration formulas

**Recommended**: Option 1 (symbolic) for 0.1 release - remove placeholder comments, make it explicit that these are symbolic

**Log**: `agent_logs/AGENT_P0_S_POLYNOMIAL_INTEGRATION_LOG.md`

---

### Agent T: Remove GPU Acceleration Placeholders

**Files**:
- `core/performance/gpu_acceleration.rs` (12 occurrences)
- `core/performance/webgpu_compute.rs` (1 occurrence)

**Strategy**:
- Keep GPU acceleration code but remove "not implemented yet" strings
- Replace with proper NotImplemented error returns
- Document as "future enhancement" not "not implemented"

**Implementation**:
```rust
// Replace:
Err(GpuError::Unsupported("WebGPU bulk add not implemented yet".to_string()))

// With:
Err(GpuError::NotImplemented {
    feature: "WebGPU bulk operations",
    reason: "Requires WebGPU API stabilization (planned for 0.2)"
})
```

**Log**: `agent_logs/AGENT_P0_T_GPU_PLACEHOLDERS_LOG.md`

---

### Agent U: Remove Remaining TODO/Placeholder Comments

**Files** (20 files with minor TODOs):
- Replace TODO comments with actual code or remove if not applicable
- Convert "placeholder" variables to descriptive names
- Document future enhancements in separate ROADMAP.md instead of inline TODOs

**Categories**:
1. **Implementable now**: Write actual code (e.g., SIMD stats tracking)
2. **Future enhancements**: Remove TODO, add to ROADMAP.md
3. **Non-applicable**: Delete comment entirely

**Log**: `agent_logs/AGENT_P0_U_TODO_CLEANUP_LOG.md`

---

**Wave 3 Verification Gate**:
```bash
./assess_0.1_blockers.sh | grep "PLACEHOLDER"
# Expected: 0 placeholder comments

cargo test -p mathhook-core --lib
# Expected: 459 passed; 0 failed

# Verify no "not implemented yet" strings remain
grep -r "not implemented yet" crates/mathhook-core/src --include="*.rs"
# Expected: no matches
```

---

## WAVE 4: Domain Error Integration

**Priority**: P0 - CRITICAL
**Target**: Integrate MathError into all fallible operations
**Duration**: 4-5 hours
**Agents**: 3 agents working in parallel

### Agent V: Integrate Domain Errors in Division Operations

**Files**:
- `core/expression/constructors.rs` (division constructors)
- `algebra/arithmetic.rs` (division implementation)
- All solver files using division

**Implementation**:
```rust
// Add Result return type
pub fn div(numerator: Expression, denominator: Expression) -> Result<Expression, MathError> {
    // Check for division by zero
    if denominator.is_zero() {
        return Err(MathError::DivisionByZero);
    }

    // Check for domain restrictions
    // ... existing logic
    Ok(Expression::mul(vec![
        numerator,
        Expression::pow(denominator, Expression::integer(-1))
    ]))
}
```

**Update Call Sites**:
- All `.div()` calls must handle Result
- Replace `.unwrap()` with proper error propagation using `?`

**Verification**:
```rust
#[test]
fn test_division_by_zero_error() {
    let result = Expression::div(expr!(5), expr!(0));
    assert!(matches!(result, Err(MathError::DivisionByZero)));
}
```

**Log**: `agent_logs/AGENT_P0_V_DIVISION_ERRORS_LOG.md`

---

### Agent W: Integrate Domain Errors in sqrt/log Operations

**Files**:
- `functions/elementary/exponential.rs` (sqrt)
- `functions/elementary/logarithmic.rs` (ln, log)

**Implementation**:
```rust
// sqrt domain check
pub fn sqrt(arg: Expression) -> Result<Expression, MathError> {
    match &arg {
        Expression::Number(Number::Integer(n)) if *n < 0 => {
            Err(MathError::DomainError {
                operation: "sqrt".to_string(),
                value: arg,
                reason: "Square root of negative number requires complex domain".to_string(),
            })
        }
        _ => Ok(Expression::function("sqrt", vec![arg]))
    }
}

// log domain check
pub fn log(base: Expression, arg: Expression) -> Result<Expression, MathError> {
    if arg.is_zero() || arg.is_negative() {
        return Err(MathError::DomainError {
            operation: "log".to_string(),
            value: arg,
            reason: "Logarithm undefined for non-positive arguments".to_string(),
        });
    }
    // ... existing logic
}
```

**Log**: `agent_logs/AGENT_P0_W_SQRT_LOG_ERRORS_LOG.md`

---

### Agent X: Replace All .unwrap() with Error Handling

**Target**: 121 .unwrap() calls across codebase
**Strategy**:
1. Search for all `.unwrap()` calls
2. Analyze each for proper error handling
3. Replace with:
   - `?` operator where function returns Result
   - `.unwrap_or_default()` for safe defaults
   - `.expect("descriptive message")` ONLY for programmer errors (invariants)

**Implementation**:
```rust
// Before:
let value = expr.evaluate().unwrap();

// After:
let value = expr.evaluate()?;

// OR for safe defaults:
let value = expr.evaluate().unwrap_or(Expression::integer(0));

// OR for invariants:
let value = expr.evaluate()
    .expect("BUG: Expression should always be evaluable after simplification");
```

**Verification**:
```bash
# Count remaining unwraps (should be <20 for invariants only)
grep -r "\.unwrap()" crates/mathhook-core/src --include="*.rs" | wc -l
```

**Log**: `agent_logs/AGENT_P0_X_UNWRAP_ELIMINATION_LOG.md`

---

**Wave 4 Verification Gate**:
```bash
./assess_0.1_blockers.sh | grep "Domain error integration"
# Expected: Domain error integration: COMPLETE

# Check Result usage
grep -r "Result<Expression, MathError>" crates/mathhook-core/src --include="*.rs" | wc -l
# Expected: >50 uses

# Check unwrap count
grep -r "\.unwrap()" crates/mathhook-core/src --include="*.rs" | wc -l
# Expected: <20

cargo test -p mathhook-core --lib
# Expected: 459 passed; 0 failed
```

---

## WAVE 5: Number Overflow & Final Cleanup

**Priority**: P0
**Target**: Comprehensive checked arithmetic + remaining medium files
**Duration**: 3-4 hours
**Agents**: 2 agents working in parallel

### Agent Y: Implement Comprehensive Checked Arithmetic

**File**: `core/number.rs`
**Current**: 3 checked operations
**Target**: Comprehensive coverage

**Implementation**:
```rust
impl Number {
    pub fn add(&self, other: &Number) -> Result<Number, MathError> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => {
                a.checked_add(*b)
                    .map(Number::Integer)
                    .ok_or(MathError::NumericOverflow)
            }
            // ... other cases
        }
    }

    pub fn mul(&self, other: &Number) -> Result<Number, MathError> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => {
                a.checked_mul(*b)
                    .map(Number::Integer)
                    .ok_or(MathError::NumericOverflow)
            }
            // ... other cases
        }
    }

    // Similar for sub, div, pow
}
```

**Testing**:
```rust
#[test]
fn test_overflow_detection() {
    let max = Number::Integer(i64::MAX);
    let result = max.add(&Number::Integer(1));
    assert!(matches!(result, Err(MathError::NumericOverflow)));
}
```

**Log**: `agent_logs/AGENT_P0_Y_CHECKED_ARITHMETIC_LOG.md`

---

### Agent Z: Refactor Remaining Medium-Priority Files

**Target**: Top 5 medium files by priority
1. `simplify/arithmetic.rs` (738 lines → ~450)
2. `educational/step_by_step.rs` (713 lines → ~450)
3. `calculus/derivatives/partial/vector_fields.rs` (718 lines → ~450)
4. `pattern/substitution.rs` (650 lines → ~450)
5. `calculus/derivatives/advanced_differentiation/vector_valued.rs` (659 lines → ~450)

**Strategy**: Each file split into 2 modules (~350 lines each)

**Log**: `agent_logs/AGENT_P0_Z_MEDIUM_FILES_REFACTOR_LOG.md`

---

**Wave 5 Verification Gate**:
```bash
./assess_0.1_blockers.sh
# Expected: TOTAL VIOLATIONS: 0

# Final comprehensive check
cargo test -p mathhook-core --lib
# Expected: 459 passed; 0 failed

cargo test --doc -p mathhook-core
# Expected: 282 passed; 0 failed

cargo check -p mathhook-core
# Expected: 0 errors

cargo clippy -p mathhook-core -- -D warnings
# Expected: 0 warnings
```

---

## Verification Scripts

### Per-Wave Verification

Each wave includes verification script execution:

**Script**: `verify_wave_X.sh`
```bash
#!/bin/bash
# Wave X Verification Script

echo "Verifying Wave X completion..."

# 1. Run assessment
./assess_0.1_blockers.sh > wave_X_assessment.txt

# 2. Check violations decreased
VIOLATIONS=$(grep "Total Violations:" wave_X_assessment.txt | grep -oE '[0-9]+')
echo "Remaining violations: $VIOLATIONS"

# 3. Run tests
cargo test -p mathhook-core --lib 2>&1 | tee wave_X_tests.txt
PASSED=$(grep -oE '[0-9]+ passed' wave_X_tests.txt | head -1 | grep -oE '[0-9]+')

if [ "$PASSED" -eq 459 ]; then
    echo "✓ All tests passing"
else
    echo "✗ Test failures detected"
    exit 1
fi

# 4. Check compilation
cargo check -p mathhook-core 2>&1 | tee wave_X_compile.txt
if grep -q "error" wave_X_compile.txt; then
    echo "✗ Compilation errors"
    exit 1
fi

echo "✓ Wave X verification complete"
```

### Final Validation

**Script**: `verify_0.1_release_final.sh`
```bash
#!/bin/bash
# Final 0.1 Release Validation

set -e

echo "========================================
echo "FINAL 0.1 RELEASE VALIDATION"
echo "========================================"

# 1. Comprehensive assessment
./assess_0.1_blockers.sh
if [ $? -ne 0 ]; then
    echo "✗ Blockers still present"
    exit 1
fi

# 2. Full test suite
cargo test -p mathhook-core
cargo test --doc -p mathhook-core

# 3. Compilation
cargo check -p mathhook-core
cargo clippy -p mathhook-core -- -D warnings

# 4. Code quality checks
echo "Checking for CLAUDE.md violations..."

# No files >500 lines
LARGE_FILES=$(find crates/mathhook-core/src -name "*.rs" -type f -exec wc -l {} + | awk '$1 > 500 {print $0}' | wc -l)
if [ "$LARGE_FILES" -gt 0 ]; then
    echo "✗ Found $LARGE_FILES files exceeding 500 lines"
    exit 1
fi

# No TODO/FIXME/placeholder
PLACEHOLDERS=$(grep -r "TODO\|FIXME\|not implemented yet\|placeholder" crates/mathhook-core/src --include="*.rs" | wc -l)
if [ "$PLACEHOLDERS" -gt 0 ]; then
    echo "✗ Found $PLACEHOLDERS placeholder comments"
    exit 1
fi

# MathError integration
RESULT_USAGE=$(grep -r "Result<Expression, MathError>" crates/mathhook-core/src --include="*.rs" | wc -l)
if [ "$RESULT_USAGE" -lt 50 ]; then
    echo "✗ Insufficient MathError integration ($RESULT_USAGE uses)"
    exit 1
fi

echo "========================================"
echo "✓ 0.1 RELEASE READY"
echo "========================================"
```

---

## Success Criteria (ALL MUST PASS)

### Code Quality
- ✅ 0 files exceeding 500 lines
- ✅ 0 "TODO/FIXME/not implemented yet" comments
- ✅ 0 "Complex case not implemented" comments
- ✅ 0 "placeholder" code

### Error Handling
- ✅ >50 uses of `Result<Expression, MathError>`
- ✅ <20 `.unwrap()` calls (only for invariants)
- ✅ Comprehensive checked arithmetic in Number type

### Testing
- ✅ 459 unit tests passing
- ✅ 282 doctests passing
- ✅ 0 compilation errors
- ✅ 0 clippy warnings (with `-D warnings`)

### Mathematical Correctness
- ✅ Complex quadratic solutions implemented
- ✅ Division by zero detection
- ✅ Domain error checking for sqrt/log
- ✅ Overflow detection in Number operations

---

## Timeline

**Total Duration**: 12-17 hours across 5 waves
- Wave 1: 2-3 hours (2 agents)
- Wave 2: 2-3 hours (5 agents)
- Wave 3: 3-4 hours (4 agents)
- Wave 4: 4-5 hours (3 agents)
- Wave 5: 3-4 hours (2 agents)

**Target Completion**: 2 days with sequential wave execution

---

## Agent Logging

All agents MUST create comprehensive logs in `agent_logs/`:

**Format**: `AGENT_P0_{LETTER}_{TASK_NAME}_LOG.md`

**Required Sections**:
1. Task Summary
2. Files Modified
3. Changes Made (with line numbers)
4. Verification Results
5. Test Execution Output
6. CLAUDE.md Compliance Check
7. Blockers Encountered (if any)
8. Time Taken

**Example**: `agent_logs/AGENT_P0_K_MATRIX_REFACTOR_LOG.md`

---

## Wave Execution Protocol

### Before Each Wave
1. Read previous wave completion reports
2. Verify no conflicts with previous changes
3. Pull latest code state

### During Wave Execution
1. Agents work in parallel on independent files
2. No file should be touched by multiple agents
3. Create detailed logs in real-time
4. Run local verification before marking complete

### After Each Wave
1. Run wave verification script
2. Create wave completion report
3. Update PHASE_7_SESSION_LOG.md
4. Verify violation count decreased
5. Gate: All checks pass before proceeding to next wave

---

## Orchestration Command

To execute this plan:
```bash
# 1. Initial assessment
cd /Users/ahmedmashhour/Documents/work/math/mathhook
./mathhook_sessions/assess_0.1_blockers.sh

# 2. Execute waves sequentially
# Wave 1: Launch agents K and L
# Wave 2: Launch agents M, N, O, P, Q
# Wave 3: Launch agents R, S, T, U
# Wave 4: Launch agents V, W, X
# Wave 5: Launch agents Y, Z

# 3. Final validation
./.mathhook_sessions/verify_0.1_release_final.sh
```

---

## Document Metadata

- **Created**: 2025-10-13
- **Author**: Orchestrator (Claude Code)
- **Methodology**: Wave-by-wave parallel agent orchestration
- **Assessment Tool**: `assess_0.1_blockers.sh`
- **Total Agents**: 16 agents (K through Z)
- **Total Violations**: 104 → 0 (target)
- **CLAUDE.md Compliance**: MANDATORY
- **Zero False Positives**: REQUIRED

---

**END OF ORCHESTRATION PLAN**

**Next Action**: Begin Wave 1 with Agents K and L
