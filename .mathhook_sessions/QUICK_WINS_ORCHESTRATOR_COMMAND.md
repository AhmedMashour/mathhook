# Quick Wins Bundle - Orchestrator Command (10/10 Quality Target)

**Created**: 2025-10-19
**Goal**: Implement high-value, low-effort features with perfect quality
**Target Quality**: 10/10 across all metrics
**Estimated Timeline**: 1 week (8-11 hours total)
**Priority**: MEDIUM-HIGH (Foundation for Month 1 roadmap)

---

## Orchestrator Bootstrap Command

```
Orchestrate Quick Wins Bundle (Elementary Functions + API Polish) using the proven methodology from Educational Waves 1-5:

Wave Structure:
- Wave 1: Absolute Value Function |x| (Elementary function with full intelligence)
- Wave 2: Square Root Function √x (Enhanced from x^(1/2) with better domain handling)
- Wave 3: Polynomial Division Public API (Documentation + trait convenience methods)

Target:
- Build 3 high-value, low-effort features
- Achieve 10/10 quality on ALL metrics
- ZERO regressions
- 100% SymPy validation
- Comprehensive test coverage (30+ tests total)
- Flawless CLAUDE.md compliance

Success Criteria:
- Quality Score: 10/10 minimum (perfect implementation)
- Tests: 30+ total (10+ per wave), 100% content validation
- CLAUDE.md: 100% compliance (files ≤500 lines, zero emojis, comprehensive docs)
- Build: 0 errors, 0 warnings
- Regressions: Zero (all 514 existing tests must pass)
- Mathematical Correctness: 100% SymPy validation
- Documentation: Production-quality with examples
- Integration: Seamless with UniversalFunctionRegistry

Standard orchestration protocol:
- You are orchestrator, maintain momentum between waves
- Create verification scripts for each wave BEFORE launching agents
- Launch agents with strict CLAUDE.md enforcement
- Verify everything before declaring complete
- Create comprehensive verification reports with honest quality scores
- Track progress with TodoWrite
- NO continuation agents needed (waves are small enough for single-agent completion)

Let's begin with Wave 1: Absolute Value Function
```

---

## Pre-Orchestration Analysis

### Why "Quick Wins Bundle"?

**Strategic Value**:
1. **Foundation for Roadmap**: These are Month 1, Week 1 in NEXT_PRIORITIES_ROADMAP.md
2. **High ROI**: Low effort (8-11 hours) with immediate value
3. **Quality Demonstration**: Perfect execution sets bar for future waves
4. **User-Facing**: All 3 features are directly user-visible

**Feasibility for 10/10**:
- Small scope per wave (2-4 hours each)
- Well-defined requirements
- Clear SymPy reference implementations
- No algorithmic complexity (unlike Risch integration)
- Existing patterns to follow (other elementary functions)

---

## Wave Breakdown

### Wave 1: Absolute Value Function |x|

**Estimated Effort**: 3-4 hours
**Quality Target**: 10/10

#### Scope
1. **Implementation File**: `functions/elementary/abs.rs` (~150-200 lines)
   - Function intelligence with full properties
   - Derivative rule: d/dx|x| = sgn(x) = x/|x| for x ≠ 0
   - Antiderivative: ∫|x|dx = x|x|/2 + C
   - Domain: ℝ (all real numbers)
   - Range: [0, ∞) (non-negative reals)
   - Special values: |0| = 0
   - Simplification rules:
     - |a*b| = |a|*|b|
     - |x²| = x²
     - |-x| = |x|
     - |x/y| = |x|/|y|

2. **API Addition**: `impl Expression` helper method
   ```rust
   pub fn abs(self) -> Expression {
       Expression::function("abs", vec![self])
   }
   ```

3. **Registry Integration**: Add to `UniversalFunctionRegistry`

4. **Tests**: `tests/abs_tests.rs` (~200-250 lines)
   - Numerical evaluation: |5| = 5, |-5| = 5, |0| = 0
   - Symbolic simplification: |-x| = |x|, |x²| = x²
   - Derivative: d/dx|x| at x=5 should be 1, at x=-5 should be -1
   - Antiderivative validation
   - Complex numbers: |a+bi| = √(a²+b²)
   - Edge cases: Division by zero in derivative at x=0
   - SymPy validation: All results match SymPy.Abs()

5. **Documentation**:
   - Module docs with mathematical definition
   - Function docs with examples
   - Domain/range restrictions clearly documented

#### Success Criteria
- ✅ File size: abs.rs ≤ 500 lines
- ✅ Zero emojis
- ✅ Build: 0 errors, 0 warnings
- ✅ Tests: 10+ tests, 100% passing, 100% content validation
- ✅ SymPy validation: 100% match for all operations
- ✅ Documentation: Complete with examples
- ✅ Quality: 10/10 (flawless implementation)

---

### Wave 2: Square Root Function √x

**Estimated Effort**: 3-4 hours
**Quality Target**: 10/10

#### Scope
1. **Implementation File**: `functions/elementary/sqrt.rs` (~200-250 lines)
   - Enhanced from current `x^(1/2)` representation
   - Function intelligence with full properties
   - Derivative: d/dx√x = 1/(2√x) for x > 0
   - Antiderivative: ∫√x dx = (2/3)x^(3/2) + C
   - Domain: [0, ∞) for real, ℂ for complex (with branch cut)
   - Range: [0, ∞) for real
   - Special values: √0 = 0, √1 = 1, √4 = 2
   - Simplification rules:
     - √(a²) = |a| (NOT a, due to sign)
     - √(ab) = √a·√b (for non-negative a, b)
     - √(a/b) = √a/√b
     - √(x⁴) = x²
     - √(-1) = i (in complex domain)

2. **Domain Handling**:
   - Real mode: √(-x) → error or symbolic
   - Complex mode: √(-x) → i√x

3. **LaTeX Output**: `\sqrt{x}` instead of `x^{1/2}`

4. **API Addition**:
   ```rust
   pub fn sqrt(self) -> Expression {
       Expression::function("sqrt", vec![self])
   }
   ```

5. **Registry Integration**: Add to `UniversalFunctionRegistry`

6. **Tests**: `tests/sqrt_tests.rs` (~250-300 lines)
   - Numerical: √4 = 2, √9 = 3, √2 ≈ 1.414...
   - Symbolic: √(x²) = |x|, √(x⁴) = x²
   - Derivative: d/dx√x matches SymPy
   - Antiderivative validation
   - Domain errors: √(-1) in real mode
   - Complex: √(-1) = i, √(-4) = 2i
   - Simplification: √(4x²) = 2|x|
   - SymPy validation: All results match SymPy.sqrt()

#### Success Criteria
- ✅ File size: sqrt.rs ≤ 500 lines
- ✅ Zero emojis
- ✅ Build: 0 errors, 0 warnings
- ✅ Tests: 10+ tests, 100% passing, 100% content validation
- ✅ SymPy validation: 100% match
- ✅ Domain handling: Proper real vs complex distinction
- ✅ LaTeX output: `\sqrt{x}` format
- ✅ Quality: 10/10 (flawless implementation)

---

### Wave 3: Polynomial Division Public API Enhancement

**Estimated Effort**: 2-3 hours
**Quality Target**: 10/10

#### Scope
1. **Documentation Enhancement**: `algebra/polynomial_division.rs`
   - Add comprehensive module-level docs (examples)
   - Document each function with examples
   - Add usage patterns (direct functions vs trait methods)

2. **Trait Convenience Methods**: `algebra/gcd.rs` (PolynomialGcd trait)
   ```rust
   /// Polynomial division returning (quotient, remainder)
   fn div_polynomial(&self, divisor: &Self, var: &Symbol)
       -> (Expression, Expression)
   {
       use crate::algebra::polynomial_division::polynomial_div;
       polynomial_div(self, divisor, var)
   }

   /// Polynomial quotient only
   fn quo_polynomial(&self, divisor: &Self, var: &Symbol) -> Expression {
       use crate::algebra::polynomial_division::polynomial_quo;
       polynomial_quo(self, divisor, var)
   }

   /// Polynomial remainder only
   fn rem_polynomial(&self, divisor: &Self, var: &Symbol) -> Expression {
       use crate::algebra::polynomial_division::polynomial_rem;
       polynomial_rem(self, divisor, var)
   }
   ```

3. **Examples File**: `examples/polynomial_division_usage.rs` (~100-150 lines)
   - Demonstrate standalone functions
   - Demonstrate trait methods
   - Show common patterns
   - Educational examples with comments

4. **Tests Enhancement**: `tests/polynomial_division_api_tests.rs` (~150-200 lines)
   - Test trait methods work correctly
   - Test ergonomics (method chaining)
   - Validate examples from documentation
   - Edge cases (division by zero, zero polynomial)

5. **Documentation**:
   - Update `algebra/mod.rs` module docs with polynomial division section
   - Add to high-level README (if exists)

#### Success Criteria
- ✅ Trait methods added and tested
- ✅ Examples file compiles and runs
- ✅ Documentation comprehensive with runnable examples
- ✅ Tests: 10+ tests for new API surface
- ✅ CLAUDE.md: Files ≤ 500 lines, zero emojis
- ✅ Build: 0 errors, 0 warnings
- ✅ Quality: 10/10 (perfect API design)

---

## Verification Script Template (Per Wave)

Each wave will have a custom verification script following this pattern:

```bash
#!/bin/bash

# Wave [N]: [NAME] Verification Script
# Purpose: Verify [FEATURE] implementation with 10/10 quality target
# Enforces CLAUDE.md compliance strictly

echo "========================================"
echo "WAVE [N]: [NAME] VERIFICATION"
echo "Target: 10/10 quality across all metrics"
echo "========================================"

FAILURES=0
WARNINGS=0

# CATEGORY 1: FILE SIZE VIOLATIONS
echo "========================================"
echo "CATEGORY 1: FILE SIZE COMPLIANCE"
echo "Target: All files ≤ 500 lines"
echo "========================================"

# Check new/modified files
# [Specific to wave]

# CATEGORY 2: EMOJI COMPLIANCE
echo "========================================"
echo "CATEGORY 2: EMOJI COMPLIANCE"
echo "Target: Zero emojis in source code"
echo "========================================"

# Check specific files for emojis

# CATEGORY 3: BUILD STATUS
echo "========================================"
echo "CATEGORY 3: BUILD STATUS"
echo "Target: 0 errors, 0 warnings"
echo "========================================"

BUILD_OUTPUT=$(cargo check -p mathhook-core 2>&1)
# Also check: cargo clippy

# CATEGORY 4: TEST PASS RATE
echo "========================================"
echo "CATEGORY 4: TEST PASS RATE"
echo "Target: 100% passing (including new tests)"
echo "========================================"

# Run tests for this wave
# Verify all tests pass

# CATEGORY 5: REGRESSION CHECK
echo "========================================"
echo "CATEGORY 5: REGRESSION CHECK"
echo "Target: All 514 existing tests still pass"
echo "========================================"

# Run full test suite
FULL_TEST=$(cargo test -p mathhook-core --lib 2>&1)
# Verify 514+ tests pass

# CATEGORY 6: SYMPY VALIDATION
echo "========================================"
echo "CATEGORY 6: SYMPY VALIDATION"
echo "Target: 100% match with SymPy reference"
echo "========================================"

# Check test output for SymPy validation
# Verify all SymPy comparisons pass

# CATEGORY 7: CONTENT VALIDATION RATIO
echo "========================================"
echo "CATEGORY 7: CONTENT VALIDATION RATIO"
echo "Target: 100% content validation (no structure-only tests)"
echo "========================================"

# Analyze tests for content validation
# Verify no false-positive tests

# CATEGORY 8: DOCUMENTATION QUALITY
echo "========================================"
echo "CATEGORY 8: DOCUMENTATION QUALITY"
echo "Target: Comprehensive docs with examples"
echo "========================================"

# Check for module docs, function docs, examples
# Verify doctests compile and run

# CATEGORY 9: REGISTRY INTEGRATION
echo "========================================"
echo "CATEGORY 9: REGISTRY INTEGRATION"
echo "Target: Proper UniversalFunctionRegistry integration"
echo "========================================"

# Verify function registered
# Check O(1) lookup works

# CATEGORY 10: MATHEMATICAL CORRECTNESS
echo "========================================"
echo "CATEGORY 10: MATHEMATICAL CORRECTNESS"
echo "Target: Flawless mathematical accuracy"
echo "========================================"

# Verify derivatives, integrals, simplifications
# Check domain/range handling
# Validate special cases

# SUMMARY
echo "========================================"
echo "VERIFICATION SUMMARY"
echo "========================================"

if [ $FAILURES -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✓ ALL CHECKS PASSED - 10/10 QUALITY ACHIEVED${NC}"
    echo "Wave [N]: [NAME] VERIFIED COMPLETE"
    exit 0
elif [ $FAILURES -eq 0 ]; then
    echo -e "${YELLOW}✓ PASSED WITH WARNINGS - 9/10 QUALITY${NC}"
    echo "$WARNINGS warning(s) found"
    exit 0
else
    echo -e "${RED}✗ VERIFICATION FAILED - QUALITY <10${NC}"
    echo "$FAILURES failure(s), $WARNINGS warning(s)"
    exit 1
fi
```

---

## Agent Prompt Template (Per Wave)

### Example: Wave 1 Agent Prompt

```markdown
# Wave 1: Absolute Value Function Implementation

## Mission: Implement |x| with 10/10 quality - flawless execution

You are Agent 1A within Quick Wins Bundle Wave 1. Your mission is to implement the absolute value function |x| as a first-class elementary function with complete mathematical intelligence, achieving PERFECT 10/10 quality.

## Critical Context

**Your Identity**: Agent 1A within Quick Wins Bundle Wave 1
**Orchestrator**: I am the orchestrator managing sequential waves with parallel agents
**Quality Target**: 10/10 (PERFECT) - This is a quick win, no excuse for anything less than flawless
**CLAUDE.md Enforcement**: MANDATORY compliance - the orchestrator WILL verify strictly

### Current Progress
- ✅ Polynomial work complete (9.25/10 quality, 103 tests, 514 total tests passing)
- ✅ Architecture audit complete (polynomial system rated 9.0/10)
- → Starting Quick Wins Bundle (Month 1, Week 1 of roadmap)

### Current Status
- Building elementary function foundation for calculus expansion
- abs(), sqrt(), polynomial API are prerequisites for Gamma function and integration work

## Your Scope: Absolute Value Function |x|

### Primary Deliverables

1. **Implementation**: `functions/elementary/abs.rs` (~150-200 lines)
   - Complete function intelligence
   - All mathematical properties
   - Domain/range handling
   - Simplification rules

2. **API**: `impl Expression` helper method
3. **Registry**: UniversalFunctionRegistry integration
4. **Tests**: `tests/abs_tests.rs` (~200-250 lines, 10+ tests)
5. **Documentation**: Production-quality with examples

### Detailed Requirements

#### File: `functions/elementary/abs.rs`

**Module Structure**:
```rust
//! Absolute value function |x| with complete mathematical intelligence
//!
//! Provides symbolic and numerical evaluation of |x|, with proper domain
//! handling, derivative/integral computation, and simplification.
//!
//! # Mathematical Definition
//!
//! |x| = { x   if x ≥ 0
//!      { -x  if x < 0
//!
//! For complex z = a + bi:
//! |z| = √(a² + b²)
//!
//! # Examples
//!
//! ```rust
//! use mathhook_core::{expr, symbol};
//!
//! let x = symbol!(x);
//!
//! // Numerical evaluation
//! let result = expr!(abs(-5)).simplify();
//! assert_eq!(result, expr!(5));
//!
//! // Symbolic properties
//! let neg = expr!(abs(-x));
//! assert_eq!(neg.simplify(), expr!(abs(x)));
//! ```

use crate::core::Expression;
use crate::functions::properties::{FunctionProperties, ElementaryProperties};
// ... imports

pub struct AbsIntelligence {
    properties: HashMap<String, FunctionProperties>,
}

impl AbsIntelligence {
    pub fn new() -> Self {
        // Implementation
    }

    pub fn get_properties(&self) -> HashMap<String, FunctionProperties> {
        // Return abs function properties
    }

    pub fn has_function(&self, name: &str) -> bool {
        name == "abs"
    }
}

// Function intelligence
// - derivative_rule: d/dx|x| = x/|x| for x ≠ 0, undefined at x=0
// - antiderivative_rule: ∫|x|dx = x|x|/2 + C
// - domain: Domain::Real (all reals), Domain::Complex (complex plane)
// - range: Range::NonNegative (≥0)
// - special_values: |0| = 0
// - simplification_rules:
//   - |a*b| = |a|*|b|
//   - |x²| = x², |x⁴| = x⁴, |x^(2n)| = x^(2n) for positive even powers
//   - |-x| = |x|
//   - |x/y| = |x|/|y|
```

**Numerical Evaluator**:
```rust
pub fn evaluate_abs_numerical(args: &[f64]) -> Vec<f64> {
    // Takes single argument, returns |x|
    // For complex: would need complex evaluator
}
```

**Simplification**:
```rust
pub fn simplify_abs(expr: &Expression) -> Expression {
    match expr {
        // |number| = abs(number)
        Expression::Number(n) => // simplify
        // |-x| = |x|
        Expression::Mul([Expression::Number(-1), x]) => abs(x)
        // |x²| = x²
        Expression::Pow(base, Expression::Number(n)) if n % 2 == 0 => expr.clone()
        // ... more rules
        _ => Expression::function("abs", vec![expr.clone()])
    }
}
```

#### File: `core/expression/methods.rs` (or appropriate location)

Add helper method:
```rust
impl Expression {
    /// Create absolute value expression |x|
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    ///
    /// let x = symbol!(x);
    /// let abs_x = x.abs();
    /// assert_eq!(abs_x, expr!(abs(x)));
    /// ```
    pub fn abs(self) -> Expression {
        Expression::function("abs", vec![self])
    }
}
```

#### File: `tests/abs_tests.rs`

**Required Tests** (10+ total, 100% content validation):

1. **Numerical Evaluation**:
   ```rust
   #[test]
   fn test_abs_numerical_positive() {
       let result = expr!(abs(5)).simplify();
       assert_eq!(result, expr!(5));
   }

   #[test]
   fn test_abs_numerical_negative() {
       let result = expr!(abs(-5)).simplify();
       assert_eq!(result, expr!(5));
   }

   #[test]
   fn test_abs_numerical_zero() {
       let result = expr!(abs(0)).simplify();
       assert_eq!(result, expr!(0));
   }
   ```

2. **Symbolic Simplification**:
   ```rust
   #[test]
   fn test_abs_negative_symbol() {
       let x = symbol!(x);
       let result = expr!(abs(-x)).simplify();
       assert_eq!(result, expr!(abs(x)));
   }

   #[test]
   fn test_abs_even_power() {
       let x = symbol!(x);
       let result = expr!(abs(x^2)).simplify();
       assert_eq!(result, expr!(x^2));
   }

   #[test]
   fn test_abs_product_rule() {
       let x = symbol!(x);
       let y = symbol!(y);
       // |xy| may simplify to |x||y| depending on implementation
       let result = expr!(abs(x * y)).simplify();
       // Validate structure
   }
   ```

3. **Derivative**:
   ```rust
   #[test]
   fn test_abs_derivative_positive() {
       let x = symbol!(x);
       let abs_x = x.clone().abs();
       let deriv = abs_x.derivative(&x, 1);

       // At x > 0: d/dx|x| = 1
       // Symbolic: d/dx|x| = x/|x|
       assert!(/* validate derivative form */);
   }
   ```

4. **Antiderivative**:
   ```rust
   #[test]
   fn test_abs_antiderivative() {
       let x = symbol!(x);
       let abs_x = x.clone().abs();
       let integral = abs_x.antiderivative(&x);

       // ∫|x|dx = x|x|/2 + C
       assert!(/* validate integral form */);
   }
   ```

5. **Complex Numbers**:
   ```rust
   #[test]
   fn test_abs_complex() {
       // |3+4i| = √(9+16) = 5
       let z = expr!(3 + 4*i);
       let result = z.abs().simplify();
       assert_eq!(result, expr!(5));
   }
   ```

6. **SymPy Validation**:
   ```rust
   #[test]
   fn test_abs_sympy_validation() {
       // Compare with SymPy Abs() for various inputs
       // |5| = 5
       // |-7| = 7
       // |0| = 0
       // Symbolic: |-x| = |x|
   }
   ```

7. **Edge Cases**:
   ```rust
   #[test]
   fn test_abs_derivative_at_zero() {
       // d/dx|x| is undefined at x=0
       // Should handle gracefully (symbolic or error)
   }
   ```

**ALL tests MUST validate actual content, not just structure.**

## CLAUDE.md Compliance Requirements (STRICTLY ENFORCED)

### File Size
- **Maximum 500 lines per file**
- abs.rs: ~150-200 lines (SAFE)
- abs_tests.rs: ~200-250 lines (SAFE)
- Check: `wc -l <file>.rs`

### No Emojis
- **Zero tolerance**
- Check before commit: `grep -r "✅\|❌\|⚠️\|🚀" functions/elementary/abs.rs tests/abs_tests.rs`

### Documentation
- `//!` for module-level ONLY
- `///` for functions, structs, traits
- Minimize inline `//` (only for mathematical formulas or complex logic)
- EVERY public function MUST have examples

### Build
- Must compile: `cargo check -p mathhook-core` with 0 errors, 0 warnings
- Clippy: `cargo clippy -p mathhook-core` with 0 warnings

### No Placeholders
- No `todo!()` macros
- No "for now" comments
- Every function fully implemented
- No symbolic placeholders

### Tests
- 100% content validation (NO structure-only tests)
- Use flexible string matching or mathematical validation
- NO tests like `assert!(result.steps.len() >= 5)` without content checks

## Success Criteria (MANDATORY FOR 10/10)

1. ✅ Implementation complete: abs.rs fully implements function intelligence
2. ✅ API ergonomic: `.abs()` method works as expected
3. ✅ Registry integration: abs registered and discoverable
4. ✅ Tests comprehensive: 10+ tests, all content validation
5. ✅ SymPy validation: 100% match with SymPy.Abs() for all test cases
6. ✅ CLAUDE.md: 100% compliance (file size, emojis, docs)
7. ✅ Build: 0 errors, 0 warnings
8. ✅ Regressions: All 514 existing tests still pass
9. ✅ Documentation: Production-quality with runnable examples
10. ✅ Mathematical correctness: Derivatives, integrals, simplifications all correct

## Verification Protocol

When you complete, the orchestrator WILL run:
```bash
bash .mathhook_sessions/verify_wave1_abs.sh
```

This script will verify:
- File sizes
- Emoji compliance
- Build status (0 errors, 0 warnings)
- Test pass rate (100%)
- Regression check (514+ tests passing)
- SymPy validation
- Content validation ratio
- Documentation quality
- Registry integration
- Mathematical correctness

**If any check fails, quality < 10/10.**

## Execution Protocol

1. **Read Reference Implementations**:
   - Look at existing elementary functions (sin, cos, exp, log)
   - Study their function intelligence pattern
   - Follow the same architecture

2. **Create abs.rs**:
   - Implement AbsIntelligence struct
   - Define all mathematical properties
   - Implement numerical evaluator
   - Implement simplification rules

3. **Add API Helper**:
   - Find appropriate location (Expression impl)
   - Add `.abs()` method
   - Document with example

4. **Registry Integration**:
   - Add abs to UniversalFunctionRegistry
   - Verify O(1) lookup works

5. **Create Tests**:
   - Write 10+ comprehensive tests
   - Ensure 100% content validation
   - Run SymPy validation comparisons
   - Test all edge cases

6. **Documentation**:
   - Module-level docs with examples
   - Function docs with examples
   - Ensure doctests compile and run

7. **Verify Locally**:
   - `cargo check -p mathhook-core` → 0 errors
   - `cargo clippy -p mathhook-core` → 0 warnings
   - `cargo test -p mathhook-core abs` → 100% passing
   - `cargo test -p mathhook-core --lib` → 514+ tests passing
   - Check file sizes
   - Check for emojis

8. **Self-Assessment**:
   - Review all success criteria
   - Confirm 10/10 quality
   - Prepare completion report

## Reporting Template

When complete, provide this report:

```markdown
# Agent 1A: Absolute Value Function - COMPLETE

## Implementation Summary

**Files Created**:
- `functions/elementary/abs.rs` ([X] lines)
- `tests/abs_tests.rs` ([Y] lines)

**Files Modified**:
- [API location]: Added `.abs()` method
- [Registry location]: Registered abs function

**Lines of Code**: [Total] lines added

## Mathematical Properties Implemented

- ✅ Derivative: d/dx|x| = x/|x| for x ≠ 0
- ✅ Antiderivative: ∫|x|dx = x|x|/2 + C
- ✅ Domain: ℝ (real), ℂ (complex)
- ✅ Range: [0, ∞)
- ✅ Special values: |0| = 0
- ✅ Simplification rules: |-x| = |x|, |x²| = x², |a*b| = |a|*|b|

## Test Summary

**Tests Created**: [N] tests
**Test Categories**:
- [X] Numerical evaluation tests
- [X] Symbolic simplification tests
- [X] Derivative tests
- [X] Antiderivative tests
- [X] Complex number tests
- [X] SymPy validation tests
- [X] Edge case tests

**Content Validation**: 100% (all tests validate actual content)

## Verification Results

**Local Verification**:
- ✅ File size: abs.rs = [X] lines (≤500)
- ✅ File size: abs_tests.rs = [Y] lines (≤500)
- ✅ Emojis: 0 found
- ✅ Build: 0 errors, 0 warnings
- ✅ Tests: [N]/[N] passing (100%)
- ✅ Regressions: 514+ tests passing (no regressions)
- ✅ SymPy validation: 100% match
- ✅ Documentation: Complete with examples
- ✅ Doctests: All compile and run
- ✅ Registry: abs function registered

**Quality Self-Assessment**: 10/10

**Justification**:
- Flawless implementation following existing patterns
- Comprehensive test coverage with content validation
- 100% CLAUDE.md compliance
- Perfect SymPy validation
- Production-quality documentation
- Zero regressions
- Mathematical correctness verified

**Ready for orchestrator verification**: YES

## Notes

[Any implementation notes, design decisions, or observations]
```

## Important Notes

1. **You are NOT the orchestrator** - you are Agent 1A, a specialized agent for abs()
2. **Focus ONLY on abs()** - don't modify sqrt, polynomial, or unrelated code
3. **Quality over speed** - take 3-4 hours if needed to achieve 10/10
4. **CLAUDE.md is law** - 100% compliance required, no exceptions
5. **Content validation** - every test must validate actual mathematical content
6. **SymPy reference** - use SymPy.Abs() as ground truth for correctness
7. **Follow patterns** - study existing elementary functions (sin, cos, exp)

## Begin Implementation

Start by reading 2-3 existing elementary function implementations to understand the pattern. Then implement abs() following that exact architecture.

**Return your final report** when all success criteria are met.
```

---

## Expected Outcomes

### Wave 1: Absolute Value
- **Quality**: 10/10
- **Files**: 2 new (abs.rs, abs_tests.rs), 2-3 modified (API, registry)
- **Tests**: 10-12 tests, 100% passing
- **Time**: 3-4 hours

### Wave 2: Square Root
- **Quality**: 10/10
- **Files**: 2 new (sqrt.rs, sqrt_tests.rs), 2-3 modified (API, registry)
- **Tests**: 10-12 tests, 100% passing
- **Time**: 3-4 hours

### Wave 3: Polynomial Division API
- **Quality**: 10/10
- **Files**: 1 new (examples/polynomial_division_usage.rs), 3-4 modified (docs, trait, tests)
- **Tests**: 10+ tests for new API
- **Time**: 2-3 hours

### Total Bundle
- **Quality**: 10/10 average
- **Tests**: 30-36 new tests
- **Total Time**: 8-11 hours
- **SymPy Coverage**: Incremental improvement
- **User Value**: High (immediate utility)

---

## Post-Bundle Verification

After all 3 waves complete, create:

1. **`QUICK_WINS_BUNDLE_COMPLETE.md`**:
   - Executive summary
   - Quality scores per wave (all 10/10)
   - Total tests added
   - Files created/modified
   - Verification results
   - Recommendation: APPROVED for Month 1 completion

2. **Update Roadmap**:
   - Mark Month 1, Week 1 as COMPLETE
   - Update status: Ready for Gamma function (Month 1, Weeks 2-4)

---

## Why This Will Achieve 10/10

1. **Small Scope**: Each wave is 2-4 hours, allowing perfect execution
2. **Clear Requirements**: Well-defined mathematical properties from SymPy
3. **Existing Patterns**: Can copy-paste-adapt from sin, cos, exp, log
4. **No Complexity**: No algorithms to implement (unlike Risch integration)
5. **Strict Verification**: 10-category verification script per wave
6. **Content Validation**: Mandatory 100% content validation in tests
7. **SymPy Ground Truth**: Clear reference for correctness
8. **CLAUDE.md**: Easy to comply (files <200 lines, no algorithmic complexity)

---

**This command is ready to execute. Paste it to the orchestrator to begin.**
