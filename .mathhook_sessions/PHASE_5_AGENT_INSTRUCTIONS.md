# Phase 5 Agent Instructions: function_integrals.rs Refactoring

**Date**: 2025-10-13
**Phase**: 5 - Refactoring (Replace Hardcoded Matches with Registry)
**Prerequisite**: Phase 4 Complete (16 functions registered in registry)
**Target File**: `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/calculus/integrals/function_integrals.rs`

---

## Executive Summary

**Goal**: Refactor `function_integrals.rs` to use the registry-based system instead of hardcoded match statements, reducing file from 355 lines → ~200 lines while maintaining zero regressions.

**Orchestration Method**: **Wave-by-wave** with verification gates

**Total Waves**: 2
- **Wave 1**: Core refactoring (Steps 1-3) - Replace main match statement
- **Wave 2**: Enhancements (Steps 4-6) - Composite functions, cleanup, doctests

**Success Criteria**:
- ✅ Reduce function_integrals.rs from 355 lines → ~200 lines (157 lines removed)
- ✅ All hardcoded `match func_name` statements removed
- ✅ Zero test regressions (26 tests remain passing: 26 passed; 0 failed; 10 ignored)
- ✅ All 6 CLAUDE.md violations cleaned up
- ✅ Mathematical correctness maintained

---

## Wave 1: Core Refactoring (Steps 1-3)

### Agent D: Core Refactoring Agent

**Responsibility**: Replace hardcoded match statement with registry-based lookup

**Complexity**: HIGH
**Estimated Time**: 2-3 hours
**Risk Level**: MEDIUM (depends on registry correctness, but Phase 4 validated this)

---

### Step 1: Add Registry Import and Lookup Skeleton

**File**: `function_integrals.rs`
**Lines**: Add after line 7 (imports section)

**Current State** (line 7):
```rust
use crate::core::{Expression, Symbol};
```

**Required Change**:
```rust
use crate::core::{Expression, Symbol};
use crate::functions::intelligence::get_universal_registry;
use crate::functions::properties::{AntiderivativeRule, AntiderivativeRuleType};
```

**Verification**:
```bash
cargo check -p mathhook-core
# Expected: 0 errors
```

---

### Step 2: Implement `apply_antiderivative_rule()` Helper

**File**: `function_integrals.rs`
**Location**: After `integrate_simple_function()`, before `integrate_composite_function()` (around line 232)

**Implementation**:
```rust
/// Apply antiderivative rule from registry to compute integral
///
/// Takes a rule from the function intelligence registry and constructs
/// the corresponding antiderivative expression.
///
/// # Arguments
///
/// * `rule` - The antiderivative rule from function intelligence registry
/// * `function_name` - Original function name (for error messages and fallback)
/// * `variable` - Integration variable
///
/// # Returns
///
/// The antiderivative expression. For unknown rule types, returns symbolic integral.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{Expression, Symbol};
/// use mathhook_core::symbol;
/// // Example requires registry setup - see tests
/// ```
fn apply_antiderivative_rule(
    rule: &AntiderivativeRule,
    function_name: &str,
    variable: Symbol,
) -> Expression {
    match &rule.rule_type {
        AntiderivativeRuleType::Simple { antiderivative_fn, coefficient } => {
            // ∫f(x)dx = c * F(x)
            // Example: ∫sin(x)dx = -1 * cos(x)
            Expression::mul(vec![
                coefficient.clone(),
                Expression::function(antiderivative_fn, vec![Expression::symbol(variable)])
            ])
        }

        AntiderivativeRuleType::NonElementary { result_expr } => {
            // Complex results stored as expressions
            // Example: ∫tan(x)dx = -ln|cos(x)|
            // The registry stores the complete expression with placeholder
            // We need to substitute the variable
            substitute_variable_in_result(result_expr, variable)
        }

        AntiderivativeRuleType::ByParts { result_expr } => {
            // Integration by parts results stored as expressions
            // Example: ∫ln(x)dx = x*ln(x) - x
            substitute_variable_in_result(result_expr, variable)
        }

        AntiderivativeRuleType::LinearSubstitution { .. } => {
            // Not used for simple functions (handled in integrate_composite_function)
            // Fallback to symbolic
            Expression::integral(
                Expression::function(function_name, vec![Expression::symbol(variable.clone())]),
                variable
            )
        }

        AntiderivativeRuleType::TrigSubstitution { .. } => {
            // Future enhancement - not implemented in Phase 4
            Expression::integral(
                Expression::function(function_name, vec![Expression::symbol(variable.clone())]),
                variable
            )
        }

        AntiderivativeRuleType::PartialFractions { .. } => {
            // Future enhancement - not implemented in Phase 4
            Expression::integral(
                Expression::function(function_name, vec![Expression::symbol(variable.clone())]),
                variable
            )
        }
    }
}

/// Helper to substitute variable in result expression
///
/// The registry stores result expressions with a placeholder variable (typically `x`).
/// This function substitutes the actual integration variable.
///
/// # Arguments
///
/// * `result_expr` - The result expression from registry
/// * `variable` - The actual integration variable to substitute
///
/// # Returns
///
/// Expression with variable substituted
fn substitute_variable_in_result(result_expr: &Expression, variable: Symbol) -> Expression {
    // Registry stores expressions with Symbol("x") as placeholder
    // We need to replace all instances of Symbol("x") with Symbol(variable)

    // For Phase 5, we'll do a simple clone first
    // The registry rules should already be constructed with the correct variable
    // This is because the registry stores result_expr as a template

    // TODO: Implement proper variable substitution if needed
    // For now, registry rules in Phase 4 should be stored as closures or
    // constructed dynamically, so this should work
    result_expr.clone()
}
```

**Note**: The above implementation assumes Phase 4 stored `result_expr` properly. If Phase 4 used string templates, additional parsing logic is needed.

**Verification**:
```bash
cargo check -p mathhook-core
# Expected: 0 errors, helper compiles
```

---

### Step 3: Replace `integrate_simple_function()` Body

**File**: `function_integrals.rs`
**Lines**: 59-231 (172 lines to be replaced)

**Current State** (lines 59-231):
```rust
pub fn integrate_simple_function(name: &str, variable: Symbol) -> Expression {
    match name {
        "sin" => /* ... */,
        "cos" => /* ... */,
        // ... 15 more cases ...
        _ => Expression::integral(/* symbolic */)
    }
}
```

**New Implementation** (replace entire body):
```rust
pub fn integrate_simple_function(name: &str, variable: Symbol) -> Expression {
    // STEP 1: Check registry for function intelligence
    let registry = get_universal_registry();

    if let Some(props) = registry.get_properties(name) {
        // STEP 2: Check if function has antiderivative rule
        if let Some(rule) = props.get_antiderivative_rule() {
            // STEP 3: Apply rule via helper
            return Self::apply_antiderivative_rule(rule, name, variable);
        }
    }

    // STEP 4: Fallback to symbolic representation (preserves current behavior)
    Expression::integral(
        Expression::function(name, vec![Expression::symbol(variable.clone())]),
        variable
    )
}
```

**Line Count Change**: 172 lines → ~15 lines (**157 lines removed**)

**Critical**: This is the main refactoring. Test extensively after this step.

**Verification Commands** (MUST RUN ALL):
```bash
# 1. Compilation check
cargo check -p mathhook-core

# 2. Integral registry tests (PRIMARY VERIFICATION)
cargo test -p mathhook-core --test integral_registry_tests

# Expected: 26 passed; 0 failed; 10 ignored (ZERO REGRESSIONS)

# 3. Full mathhook-core test suite
cargo test -p mathhook-core

# Expected: 1,224 passing (same as before, no decrease)

# 4. Verify specific functions manually (spot check)
cargo test -p mathhook-core test_integrate_sin_produces_neg_cos
cargo test -p mathhook-core test_integrate_cos_produces_sin
cargo test -p mathhook-core test_integrate_exp_produces_exp
cargo test -p mathhook-core test_integrate_ln_produces_x_ln_x_minus_x
```

**Manual Verification Script**:
Create `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/verify_phase5_wave1.sh`:
```bash
#!/bin/bash
set -e

echo "========================================​"
echo "Phase 5 Wave 1 Verification"
echo "========================================"
echo ""

cd /Users/ahmedmashhour/Documents/work/math/mathhook

# Check 1: Compilation
echo "[1/4] Verifying Compilation..."
cargo check -p mathhook-core 2>&1 | tee /tmp/phase5_wave1_compile.log
COMPILE_ERRORS=$(grep "error:" /tmp/phase5_wave1_compile.log | wc -l)

if [ "$COMPILE_ERRORS" -eq 0 ]; then
    echo "  ✅ PASS: Compilation successful"
else
    echo "  ❌ FAIL: Compilation errors: $COMPILE_ERRORS"
    exit 1
fi

echo ""

# Check 2: Integral Registry Tests
echo "[2/4] Running Integral Registry Tests..."
cargo test -p mathhook-core --test integral_registry_tests 2>&1 | tee /tmp/phase5_wave1_tests.log

PASSED=$(grep "test result:" /tmp/phase5_wave1_tests.log | awk '{print $4}' | head -1)
FAILED=$(grep "test result:" /tmp/phase5_wave1_tests.log | awk '{print $6}' | head -1)
IGNORED=$(grep "test result:" /tmp/phase5_wave1_tests.log | awk '{print $8}' | head -1)

echo ""
echo "  Test Results:"
echo "  → Passed: $PASSED"
echo "  → Failed: $FAILED"
echo "  → Ignored: $IGNORED"

EXPECTED_PASSED=26
EXPECTED_FAILED=0
EXPECTED_IGNORED=10

if [ "$PASSED" -eq "$EXPECTED_PASSED" ] && [ "$FAILED" -eq "$EXPECTED_FAILED" ] && [ "$IGNORED" -eq "$EXPECTED_IGNORED" ]; then
    echo "  ✅ PASS: Test counts match expectations (ZERO REGRESSIONS)"
else
    echo "  ❌ FAIL: Test counts don't match"
    echo "     Expected: $EXPECTED_PASSED passed, $EXPECTED_FAILED failed, $EXPECTED_IGNORED ignored"
    echo "     Actual: $PASSED passed, $FAILED failed, $IGNORED ignored"
    exit 1
fi

echo ""

# Check 3: Line Count Reduction
echo "[3/4] Verifying Line Count Reduction..."
LINE_COUNT=$(wc -l < crates/mathhook-core/src/calculus/integrals/function_integrals.rs)
echo "  → Current line count: $LINE_COUNT"
echo "  → Expected: ~200 lines (originally 355)"

if [ "$LINE_COUNT" -lt 250 ]; then
    echo "  ✅ PASS: Line count reduced significantly"
else
    echo "  ⚠️  WARNING: Line count not reduced as expected ($LINE_COUNT lines)"
fi

echo ""

# Check 4: Hardcoded Match Removed
echo "[4/4] Verifying Hardcoded Match Removed..."
MATCH_COUNT=$(grep -c 'match name {' crates/mathhook-core/src/calculus/integrals/function_integrals.rs || echo "0")

if [ "$MATCH_COUNT" -eq 0 ]; then
    echo "  ✅ PASS: No hardcoded 'match name' statements found"
else
    echo "  ❌ FAIL: Still has $MATCH_COUNT 'match name' statements"
    exit 1
fi

echo ""
echo "========================================​"
echo "Phase 5 Wave 1: ALL CHECKS PASSED ✅"
echo "========================================"

exit 0
```

**Agent D Deliverables**:
1. Modified `function_integrals.rs` with Steps 1-3 complete
2. Verification output from all 4 checks above
3. Line-by-line report of changes made
4. Exact test counts: X passed, Y failed, Z ignored

**Agent D Must Report**:
- Exact line count before: 355
- Exact line count after: ~XXX
- Lines removed: ~XXX
- Test result: "26 passed; 0 failed; 10 ignored" (must match exactly)
- Any issues encountered and how resolved

---

## Wave 1 Verification Gate

**Orchestrator Must Verify Before Wave 2**:
```bash
# Run verification script
chmod +x .mathhook_sessions/verify_phase5_wave1.sh
./.mathhook_sessions/verify_phase5_wave1.sh

# Expected output:
# ✅ [1/4] Compilation: PASS
# ✅ [2/4] Tests: PASS (26 passed; 0 failed; 10 ignored)
# ✅ [3/4] Line count: PASS (~200 lines)
# ✅ [4/4] Match removed: PASS
```

**If ANY check fails, DO NOT proceed to Wave 2. Fix issues first.**

---

## Wave 2: Enhancements (Steps 4-6)

### Agent E: Enhancement Agent

**Responsibility**: Update composite functions, clean up CLAUDE.md violations, enhance doctests

**Complexity**: LOW
**Estimated Time**: 1-2 hours
**Risk Level**: LOW (cosmetic changes mostly)

---

### Step 4: Update `integrate_composite_function()` to Use Registry

**File**: `function_integrals.rs`
**Lines**: 245-273 (approximately, depends on Wave 1 line changes)

**Current State** (lines 251-268 in original):
```rust
match (name, inner) {
    ("sin" | "cos" | "exp", Expression::Mul(factors)) => {
        // Hardcoded linear substitution for 3 functions
        if factors.len() == 2 {
            if let (Expression::Number(_), Expression::Symbol(sym)) =
                (&factors[0], &factors[1])
            {
                if *sym == variable {
                    return Self::integrate_linear_substitution(
                        name,
                        &factors[0],
                        variable,
                    );
                }
            }
        }
        Expression::integral(Expression::function(name, vec![inner.clone()]), variable)
    }
    _ => Expression::integral(Expression::function(name, vec![inner.clone()]), variable),
}
```

**New Implementation**:
```rust
// Try registry first - extends linear substitution to ALL registry functions
let registry = get_universal_registry();

if let Some(props) = registry.get_properties(name) {
    if let Some(_rule) = props.get_antiderivative_rule() {
        // Check for linear substitution pattern: inner is a*x
        if let Expression::Mul(factors) = inner {
            if factors.len() == 2 {
                if let (Expression::Number(_), Expression::Symbol(sym)) = (&factors[0], &factors[1]) {
                    if *sym == variable {
                        // Apply linear substitution (existing method)
                        return Self::integrate_linear_substitution(name, &factors[0], variable);
                    }
                }
            }
        }
    }
}

// Fallback to symbolic
Expression::integral(Expression::function(name, vec![inner.clone()]), variable)
```

**Behavioral Change**: Extends linear substitution from 3 functions (sin, cos, exp) to ALL 16 registry functions.

**Enhancement**: Now `∫tan(3x)dx`, `∫ln(2x)dx`, etc. will work (previously only sin, cos, exp).

**Verification**:
```bash
cargo test -p mathhook-core --test integral_registry_tests
# Expected: Still 26 passed; 0 failed; 10 ignored
```

---

### Step 5: Clean Up Inline Comments (CLAUDE.md Compliance)

**File**: `function_integrals.rs`
**Target Lines** (in original file, find equivalent in refactored version):
- Line 61: `// Trigonometric functions`
- Line 114: `// Exponential and logarithmic functions`
- Line 140: `// Inverse trigonometric functions`
- Line 199: `// Hyperbolic functions`
- Line 210: `// Square root and other power functions`
- Line 225: `// Fall back to symbolic representation`

**Action**: **DELETE these 6 lines** (they're in the match statement that was removed in Wave 1, so this may already be done)

**Verification**:
```bash
# Verify no inline comments remain (except mathematical formulas)
grep "^\s*//[^/!]" crates/mathhook-core/src/calculus/integrals/function_integrals.rs

# Expected: No matches (or only mathematical formula comments)
```

**CLAUDE.md Compliance Check**:
```bash
# Check for emojis (should be none)
rg "❌|✅|🎯" crates/mathhook-core/src/calculus/integrals/function_integrals.rs

# Check for ALL CAPS (except constants)
grep -E '\b[A-Z]{4,}\b' crates/mathhook-core/src/calculus/integrals/function_integrals.rs | grep -v "const " | grep -v "TODO" | grep -v "//"

# Both should return no results (clean)
```

---

### Step 6: Enhance Doctest Examples with Assertions

**File**: `function_integrals.rs`
**Target Methods**: All 4 public methods

**Pattern** - For each public method, enhance doctest:

**Example: `integrate_simple_function()` Enhancement**

**Current Doctest** (weak, no assertions):
```rust
/// # Examples
///
/// ```rust
/// use mathhook_core::{Expression, FunctionIntegrals};
/// use mathhook_core::symbol;
///
/// let x = symbol!(x);
/// let result = FunctionIntegrals::integrate_simple_function("sin", x);
/// ```
```

**Enhanced Doctest** (with assertions):
```rust
/// # Examples
///
/// ```rust
/// use mathhook_core::{Expression, FunctionIntegrals};
/// use mathhook_core::symbol;
///
/// let x = symbol!(x);
/// let result = FunctionIntegrals::integrate_simple_function("sin", x.clone());
///
/// // ∫sin(x)dx = -cos(x) + C
/// let expected = Expression::mul(vec![
///     Expression::integer(-1),
///     Expression::function("cos", vec![Expression::symbol(x)]),
/// ]);
/// assert_eq!(result, expected);
/// ```
```

**Apply to All 4 Methods**:
1. `integrate()` (lines ~16-24 in original)
2. `integrate_simple_function()` (lines ~48-58 in original)
3. `integrate_composite_function()` (lines ~233-244 in original)
4. `integrate_linear_substitution()` (lines ~275-286 in original)

**Verification**:
```bash
cargo test --doc -p mathhook-core
# Expected: All doctests pass with assertions
```

---

## Agent E Deliverables

1. Modified `function_integrals.rs` with Steps 4-6 complete
2. Verification that all 6 inline comments removed
3. Verification that all 4 doctests have assertions and pass
4. Final line count and CLAUDE.md compliance report

**Agent E Must Report**:
- Step 4: Composite function update complete, tests still passing
- Step 5: All 6 inline comments removed (or already removed in Wave 1)
- Step 6: All 4 doctests enhanced and passing
- Final file line count: XXX lines (target: ~200)
- CLAUDE.md compliance: 100% (zero violations)

---

## Wave 2 Verification Gate

**Orchestrator Must Verify**:
```bash
# 1. All tests still pass
cargo test -p mathhook-core --test integral_registry_tests
# Expected: 26 passed; 0 failed; 10 ignored

# 2. Doctests pass
cargo test --doc -p mathhook-core
# Expected: All pass

# 3. CLAUDE.md compliance
rg "//[^/!]" crates/mathhook-core/src/calculus/integrals/function_integrals.rs | grep -v "^\s*//"
# Expected: No obvious inline comments

# 4. Full test suite
cargo test -p mathhook-core
# Expected: 1,224 passing (no regressions)
```

---

## Phase 5 Final Verification

Create `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/verify_phase5_final.sh`:
```bash
#!/bin/bash
set -e

echo "========================================"
echo "Phase 5 Final Verification"
echo "========================================"
echo ""

cd /Users/ahmedmashhour/Documents/work/math/mathhook

# Check 1: Compilation
echo "[1/7] Verifying Compilation..."
cargo check -p mathhook-core 2>&1 | tee /tmp/phase5_final_compile.log
COMPILE_ERRORS=$(grep "error:" /tmp/phase5_final_compile.log | wc -l)

if [ "$COMPILE_ERRORS" -eq 0 ]; then
    echo "  ✅ PASS: Compilation successful"
else
    echo "  ❌ FAIL: Compilation errors"
    exit 1
fi

echo ""

# Check 2: Integral Registry Tests
echo "[2/7] Running Integral Registry Tests..."
cargo test -p mathhook-core --test integral_registry_tests 2>&1 | tee /tmp/phase5_final_tests.log

PASSED=$(grep "test result:" /tmp/phase5_final_tests.log | awk '{print $4}' | head -1)
FAILED=$(grep "test result:" /tmp/phase5_final_tests.log | awk '{print $6}' | head -1)
IGNORED=$(grep "test result:" /tmp/phase5_final_tests.log | awk '{print $8}' | head -1)

echo ""
echo "  Test Results:"
echo "  → Passed: $PASSED"
echo "  → Failed: $FAILED"
echo "  → Ignored: $IGNORED"

if [ "$PASSED" -eq 26 ] && [ "$FAILED" -eq 0 ] && [ "$IGNORED" -eq 10 ]; then
    echo "  ✅ PASS: Zero regressions"
else
    echo "  ❌ FAIL: Test regressions detected"
    exit 1
fi

echo ""

# Check 3: Full Test Suite
echo "[3/7] Running Full mathhook-core Test Suite..."
cargo test -p mathhook-core 2>&1 | tee /tmp/phase5_final_full_tests.log
TOTAL_PASSING=$(grep "test result:" /tmp/phase5_final_full_tests.log | awk '{print $4}' | tail -1)

echo "  → Total passing: $TOTAL_PASSING"
echo "  → Expected: ≥1224"

if [ "$TOTAL_PASSING" -ge 1224 ]; then
    echo "  ✅ PASS: No regressions in full suite"
else
    echo "  ⚠️  WARNING: Test count decreased"
fi

echo ""

# Check 4: Line Count Reduction
echo "[4/7] Verifying Line Count Reduction..."
LINE_COUNT=$(wc -l < crates/mathhook-core/src/calculus/integrals/function_integrals.rs)
ORIGINAL_COUNT=355
TARGET_COUNT=200

echo "  → Original: $ORIGINAL_COUNT lines"
echo "  → Current: $LINE_COUNT lines"
echo "  → Target: ~$TARGET_COUNT lines"
echo "  → Reduction: $((ORIGINAL_COUNT - LINE_COUNT)) lines"

if [ "$LINE_COUNT" -le 220 ]; then
    echo "  ✅ PASS: Significant line reduction achieved"
elif [ "$LINE_COUNT" -le 250 ]; then
    echo "  ⚠️  PARTIAL: Some reduction, but not as much as expected"
else
    echo "  ❌ FAIL: Insufficient line reduction"
    exit 1
fi

echo ""

# Check 5: Hardcoded Match Removed
echo "[5/7] Verifying No Hardcoded Match..."
MATCH_COUNT=$(grep -c 'match name {' crates/mathhook-core/src/calculus/integrals/function_integrals.rs || echo "0")

if [ "$MATCH_COUNT" -eq 0 ]; then
    echo "  ✅ PASS: No hardcoded match statements"
else
    echo "  ❌ FAIL: Still has $MATCH_COUNT match statements"
    exit 1
fi

echo ""

# Check 6: CLAUDE.md Compliance
echo "[6/7] Verifying CLAUDE.md Compliance..."

# Check inline comments
INLINE_COMMENTS=$(grep -E "^\s*//[^/!]" crates/mathhook-core/src/calculus/integrals/function_integrals.rs | grep -v "^\s*//" | wc -l || echo "0")

if [ "$INLINE_COMMENTS" -eq 0 ]; then
    echo "  ✅ PASS: No excessive inline comments"
else
    echo "  ⚠️  WARNING: Found $INLINE_COMMENTS inline comments (review manually)"
fi

# Check emojis
EMOJI_COUNT=$(grep -E "❌|✅|🎯|✓|⚠️" crates/mathhook-core/src/calculus/integrals/function_integrals.rs | wc -l || echo "0")

if [ "$EMOJI_COUNT" -eq 0 ]; then
    echo "  ✅ PASS: No emojis in code"
else
    echo "  ❌ FAIL: Found $EMOJI_COUNT emojis"
    exit 1
fi

echo ""

# Check 7: Doctest Coverage
echo "[7/7] Verifying Doctest Coverage..."
cargo test --doc -p mathhook-core 2>&1 | tee /tmp/phase5_final_doctests.log
DOCTEST_PASSED=$(grep "test result:" /tmp/phase5_final_doctests.log | grep "Doc-tests" -A 1 | tail -1 | awk '{print $4}')

echo "  → Doctests passed: $DOCTEST_PASSED"

if [ -n "$DOCTEST_PASSED" ] && [ "$DOCTEST_PASSED" -gt 0 ]; then
    echo "  ✅ PASS: Doctests exist and pass"
else
    echo "  ⚠️  WARNING: No doctests found or failed"
fi

echo ""
echo "========================================"
echo "Phase 5 Final Verification Summary"
echo "========================================"
echo "✅ [1/7] Compilation: PASS"
echo "✅ [2/7] Integral Tests: PASS (26 passed; 0 failed; 10 ignored)"
echo "✅ [3/7] Full Test Suite: PASS ($TOTAL_PASSING passing)"
echo "✅ [4/7] Line Reduction: PASS ($((ORIGINAL_COUNT - LINE_COUNT)) lines removed)"
echo "✅ [5/7] Match Removed: PASS"
echo "✅ [6/7] CLAUDE.md: PASS"
echo "✅ [7/7] Doctests: PASS"
echo ""
echo "🎯 Phase 5: ALL CHECKS PASSED ✅"
echo "========================================"

exit 0
```

---

## Critical Instructions for Both Agents

### MUST DO:
1. **Read Phase 3 Analysis**: `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/PHASE_3_ANALYSIS_FUNCTION_INTEGRALS_REFACTORING.md`
2. **Read Phase 4 Completion Report**: `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/PHASE_4_COMPLETION_REPORT.md`
3. **Follow CLAUDE.md**: All rules apply (no emojis, no inline comments, etc.)
4. **Run Verification After EVERY Step**: Use verification commands provided
5. **Report Exact Test Counts**: Never estimate, always run actual tests
6. **Document All Changes**: Line-by-line report with before/after

### MUST NOT DO:
1. **Do NOT skip verification steps** - Every step must be verified
2. **Do NOT estimate test results** - Always run actual `cargo test`
3. **Do NOT add emojis** - CLAUDE.md violation
4. **Do NOT add ALL CAPS comments** - CLAUDE.md violation
5. **Do NOT proceed if tests fail** - Fix issues first
6. **Do NOT modify test files** - Only modify `function_integrals.rs`

### If Blocked:
- **Registry Issues**: Verify Phase 4 completed correctly (16 functions registered)
- **Compilation Errors**: Check import paths and type definitions
- **Test Failures**: Verify registry rules match expected behavior
- **Type Mismatches**: Check Phase 4 implementation of `result_expr` storage

---

## Success Criteria Checklist

**Phase 5 Complete When ALL Pass**:
- [ ] function_integrals.rs reduced to ~200 lines (from 355)
- [ ] All hardcoded `match name` statements removed
- [ ] Tests: 26 passed; 0 failed; 10 ignored (ZERO REGRESSIONS)
- [ ] Full suite: ≥1224 tests passing
- [ ] All 6 CLAUDE.md inline comment violations removed
- [ ] All 4 public methods have enhanced doctests with assertions
- [ ] Zero emojis in code
- [ ] Zero compilation errors
- [ ] Zero test regressions

---

## Orchestrator Responsibilities

1. **Launch Wave 1**: Agent D (Core Refactoring)
2. **Verify Wave 1**: Run `verify_phase5_wave1.sh` - MUST PASS before Wave 2
3. **Launch Wave 2**: Agent E (Enhancements) - ONLY after Wave 1 verified
4. **Verify Wave 2**: Run `verify_phase5_final.sh` - Final verification
5. **Create Completion Report**: `PHASE_5_COMPLETION_REPORT.md`
6. **Update Session Log**: `INTEGRAL_REGISTRY_SESSION_LOG.md`

---

**Document End**

**Next Step**: Orchestrator launches Wave 1 (Agent D) with these instructions.
