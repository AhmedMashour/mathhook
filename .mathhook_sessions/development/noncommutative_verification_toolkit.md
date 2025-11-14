# Noncommutative Algebra Verification Toolkit

**Purpose**: Independent verification tools for second-reviewer to validate orchestrator's work
**Reviewer**: Claude (second verifier)
**Date Created**: 2025-10-19

---

## How to Use This Toolkit

After orchestrator completes each wave, the user says:
```
"Verify Wave N using the toolkit"
```

I (second verifier) will then:
1. Read this toolkit
2. Run all verification commands for that wave
3. Check all success criteria
4. Report findings (pass/fail with specific issues)

---

## Wave 1: Core Type System & Symbol Enhancement

### Verification Commands

```bash
# 1. Check SymbolType enum exists
grep -n "pub enum SymbolType" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/symbol.rs

# 2. Check Commutativity enum exists
grep -n "pub enum Commutativity" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/commutativity.rs

# 3. Check Expression::Mul signature changed
grep -n "Mul(Box<Vec<Expression>>, Commutativity)" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/data_types.rs

# 4. Check Symbol constructors exist
grep -n "pub fn scalar\|pub fn matrix\|pub fn operator\|pub fn quaternion" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/symbol.rs

# 5. Check Expression size
cargo test --release -p mathhook-core -- expression_size --nocapture 2>&1 | grep "Expression size"

# 6. Run all tests
cargo test -p mathhook-core 2>&1 | grep "test result"
```

### Success Criteria Checklist

- [ ] SymbolType enum has 4 variants: Scalar, Matrix, Operator, Quaternion
- [ ] Commutativity enum has 3 variants: Commutative, Noncommutative, Unknown
- [ ] Expression::Mul has signature `Mul(Box<Vec<Expression>>, Commutativity)`
- [ ] Symbol has all 4 constructor methods
- [ ] Symbol has `commutativity()` method
- [ ] Expression size ≤ 48 bytes
- [ ] All existing tests pass (0 regressions)
- [ ] No emojis in code
- [ ] All files ≤ 500 lines
- [ ] Proper documentation on all public items

### Manual Verification (Read Code)

```bash
# Read Symbol implementation
cat /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/symbol.rs | head -100

# Read Commutativity implementation
cat /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/commutativity.rs

# Read Expression enum
cat /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/data_types.rs | grep -A 5 "pub enum Expression"
```

---

## Wave 2: Constructor & Accessor Updates

### Verification Commands

```bash
# 1. Check Expression::mul() auto-infers commutativity
grep -A 20 "pub fn mul(" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/constructors/basic.rs | grep -i "infer\|commutat"

# 2. Check NO explicit control method exists
! grep -n "mul_with_commutativity" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/constructors/basic.rs

# 3. Count Mul pattern match updates (should be ~100-150)
grep -r "Expression::Mul(" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/ | wc -l

# 4. Check commutativity() method exists for all variants
grep -n "pub fn commutativity(&self)" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/methods.rs

# 5. Check is_commutative() convenience method
grep -n "pub fn is_commutative(&self)" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/methods.rs

# 6. Run tests
cargo test -p mathhook-core 2>&1 | grep "test result"
```

### Success Criteria Checklist

- [ ] Expression::mul() automatically infers commutativity from factors
- [ ] NO `mul_with_commutativity()` method exists (we removed this)
- [ ] All pattern matches updated (count ≥ 100)
- [ ] Expression::commutativity() implemented for all variants
- [ ] Expression::is_commutative() exists
- [ ] Inference rules correct:
  - All Commutative → Commutative
  - Any Noncommutative → Noncommutative
  - Any Unknown → Unknown
- [ ] Build passes with zero errors
- [ ] All tests pass
- [ ] 30+ new tests for commutativity inference

### Test Cases to Validate

```bash
# Run specific commutativity tests
cargo test -p mathhook-core commutativity 2>&1 | grep "test result"
cargo test -p mathhook-core inference 2>&1 | grep "test result"
```

---

## Wave 3: Simplification Engine Updates

### Verification Commands

```bash
# 1. Check simplify_multiplication uses commutativity
grep -A 30 "pub fn simplify_multiplication" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/simplify/arithmetic/multiplication.rs | grep -i "commutat\|sort"

# 2. Check conditional sorting exists
grep -n "can_sort()\|Commutativity::Commutative" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/simplify/arithmetic/multiplication.rs

# 3. Check addition respects commutativity
grep -n "commutat" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/simplify/arithmetic/addition.rs

# 4. Run simplification tests
cargo test -p mathhook-core simplif 2>&1 | grep "test result"

# 5. Check NO sorting for noncommutative
cargo test -p mathhook-core -- --nocapture 2>&1 | grep -i "A\*B.*B\*A"
```

### Success Criteria Checklist

- [ ] simplify_multiplication() checks commutativity before sorting
- [ ] ONLY commutative factors are sorted
- [ ] Noncommutative factors preserve order
- [ ] Like-term collection: AB + BA stays separate if noncommutative
- [ ] Power rules: (AB)^2 ≠ A^2*B^2 for noncommutative
- [ ] 50+ tests for commutative vs noncommutative simplification
- [ ] Critical test: A*B + B*A does NOT simplify to 2*A*B for matrices
- [ ] All existing tests still pass

### Manual Test Cases

```bash
# Create test matrix symbols and verify no simplification
cargo test -p mathhook-core matrix_simplification -- --nocapture
```

---

## Wave 4: Calculus Integration

### Verification Commands

```bash
# 1. Check product rule preserves order
grep -A 20 "product_rule\|d.*AB" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/calculus/derivatives/rules.rs

# 2. Check all calculus files updated
for file in derivatives/rules.rs integrals/indefinite.rs limits.rs series.rs summation.rs; do
  echo "=== $file ==="
  grep -n "commutat\|Noncommutative" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/calculus/$file | head -5
done

# 3. Run calculus tests
cargo test -p mathhook-core calculus 2>&1 | grep "test result"

# 4. Count new tests
cargo test -p mathhook-core calculus -- --list 2>&1 | wc -l
```

### Success Criteria Checklist

- [ ] Product rule: d(AB)/dx = (dA/dx)B + A(dB/dx) with ORDER preserved
- [ ] Chain rule respects noncommutativity
- [ ] Integration preserves order
- [ ] Limits work for noncommutative expressions
- [ ] Series expansion: (A+B)^n preserves order in all terms
- [ ] Summation handles noncommutative terms
- [ ] 40+ new calculus tests
- [ ] SymPy validation for product rule, chain rule
- [ ] Critical: d(AB)/dx ≠ dA/dx * dB/dx for matrices

---

## Wave 5: Algebra Operations Integration

### Verification Commands

```bash
# 1. Check expand.rs handles (A+B)^2 correctly
grep -A 30 "expand\|(A+B)" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/expand.rs | grep -i "commutat\|order"

# 2. Check all algebra files
for file in expand.rs factor.rs collect.rs polynomial_division.rs rational.rs advanced_simplify.rs; do
  echo "=== $file ==="
  grep -n "commutat" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/$file | head -3
done

# 3. Run algebra tests
cargo test -p mathhook-core algebra 2>&1 | grep "test result"

# 4. Specific expansion test
cargo test -p mathhook-core expand_noncommutative -- --nocapture
```

### Success Criteria Checklist

- [ ] Expansion: (A+B)^2 = A^2 + AB + BA + B^2 (NOT A^2 + 2AB + B^2)
- [ ] Factoring preserves order (left vs right factoring)
- [ ] Collection: AB and BA are DIFFERENT terms
- [ ] Polynomial division with noncommutative coefficients works
- [ ] Rational expressions simplified correctly
- [ ] Advanced simplification respects commutativity
- [ ] 50+ tests comparing commutative vs noncommutative
- [ ] SymPy validation tests pass
- [ ] Critical: (A+B)^2 expansion is mathematically correct for matrices

---

## Wave 6: Pattern Matching & Substitution

### Verification Commands

```bash
# 1. Check pattern matching respects order
grep -A 20 "match\|pattern" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/pattern/matching/mod.rs | grep -i "commutat\|order"

# 2. Check substitution preserves positions
grep -A 20 "substitute\|replace" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/pattern/substitution/mod.rs | grep -i "commutat\|position"

# 3. Run pattern tests
cargo test -p mathhook-core pattern 2>&1 | grep "test result"
```

### Success Criteria Checklist

- [ ] Pattern: AB does NOT match BA for noncommutative
- [ ] Substitution: A→C in ABA becomes CBA (preserves positions)
- [ ] Ordered vs unordered pattern matching modes exist
- [ ] Wildcard matching respects commutativity
- [ ] Pattern-based rewrite rules check commutativity first
- [ ] 30+ tests for ordered pattern matching
- [ ] Documentation on pattern matching modes

---

## Wave 7: Matrix Operations Enhancement

### Verification Commands

```bash
# 1. Check transpose reverses order
grep -A 15 "transpose" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/matrix/operations.rs | grep -i "reverse\|order"

# 2. Check inverse reverses order
grep -A 15 "inverse" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/matrix/operations.rs | grep -i "reverse\|order"

# 3. Run matrix tests
cargo test -p mathhook-core matrix 2>&1 | grep "test result"
```

### Success Criteria Checklist

- [ ] Transpose: (AB)^T = B^T A^T (order REVERSES)
- [ ] Determinant works for symbolic matrices
- [ ] Inverse: (AB)^(-1) = B^(-1) A^(-1) (order REVERSES)
- [ ] Eigenvalue computation for symbolic matrices
- [ ] Decomposition methods work symbolically
- [ ] All matrix operations respect noncommutativity
- [ ] 40+ tests for symbolic matrix operations
- [ ] SymPy validation for transpose, inverse rules

---

## Wave 8: Parser Integration (LaTeX)

### Verification Commands

```bash
# 1. Check parser has \mathbf support
grep -n "mathbf\|MATHBF" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/parser/grammar.lalrpop

# 2. Check \hat support
grep -n "hat\|HAT" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/parser/grammar.lalrpop

# 3. Regenerate and test parser
cd /Users/ahmedmashhour/Documents/work/math/mathhook && lalrpop crates/mathhook-core/src/parser/grammar.lalrpop && cargo test -p mathhook-core parser 2>&1 | grep "test result"

# 4. Test parsing all four types
cargo test -p mathhook-core parse_matrix -- --nocapture
cargo test -p mathhook-core parse_operator -- --nocapture
```

### Success Criteria Checklist

- [ ] Parser recognizes \mathbf{A} → Matrix symbol
- [ ] Parser recognizes \hat{p} → Operator symbol
- [ ] Quaternion notation supported
- [ ] All four types parseable
- [ ] Lowercase letters stay scalar (commutative)
- [ ] Explicit notation always wins
- [ ] 20+ parser tests with matrix/operator notation
- [ ] Examples show automatic type inference
- [ ] Parser handles \mathbf{A}\mathbf{B} and plain AB correctly

---

## Wave 9: symbol! and symbols! Macro Enhancement

### Verification Commands

```bash
# 1. Check symbol! macro updated
grep -A 30 "macro_rules! symbol" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook/src/macros.rs

# 2. Check symbols! macro exists
grep -A 30 "macro_rules! symbols" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook/src/macros.rs

# 3. Test macro usage
cargo test -p mathhook macro -- --nocapture 2>&1 | grep -i "symbol"

# 4. Check all four types supported
grep -n "matrix\|operator\|quaternion" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook/src/macros.rs
```

### Success Criteria Checklist

- [ ] symbol!(x) creates Scalar (default, backward compatible)
- [ ] symbol!(A; matrix) creates Matrix
- [ ] symbol!(p; operator) creates Operator
- [ ] symbol!(q; quaternion) creates Quaternion
- [ ] symbols!("x y z") creates all scalars
- [ ] symbols!("A B C"; matrix) creates all matrices
- [ ] symbols!("p x h"; operator) creates all operators
- [ ] symbols!("i j k"; quaternion) creates all quaternions
- [ ] Commutator and anticommutator functions exist
- [ ] 25+ tests covering all four types
- [ ] Doctests show usage for all types
- [ ] CLAUDE.md updated with examples

---

## Wave 10: Equation Solvers Integration

### Verification Commands

```bash
# 1. Check solver distinguishes AX = B from XA = B
grep -A 20 "solve\|left\|right" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/solvers.rs | grep -i "commutat\|order"

# 2. Check linear solver
grep -n "commutat" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/solvers/linear.rs

# 3. Run solver tests
cargo test -p mathhook-core solver 2>&1 | grep "test result"
```

### Success Criteria Checklist

- [ ] Solver distinguishes AX = B from XA = B (different solutions!)
- [ ] Linear system solver handles matrix coefficients
- [ ] Left vs right division support
- [ ] Equation analyzer detects commutativity
- [ ] Symbolic solutions preserve order
- [ ] 35+ tests for matrix equations
- [ ] SymPy validation for matrix equation solving

---

## Wave 11: Message Registry, Educational & Formatters

### Verification Commands

```bash
# 1. Check message registry updates
for file in algebra.rs calculus.rs core.rs solvers.rs; do
  echo "=== message_registry/$file ==="
  grep -n "noncommut\|matrix\|order" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/educational/message_registry/$file | head -5
done

# 2. Check step-by-step
grep -n "noncommut" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/educational/step_by_step.rs

# 3. Check formatters
grep -n "mathbf\|matrix" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/formatter/latex/expressions.rs
grep -n "matrix\|operator" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/formatter/simple.rs
grep -n "matrix" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/formatter/wolfram.rs

# 4. Run formatter tests
cargo test -p mathhook-core format 2>&1 | grep "test result"
```

### Success Criteria Checklist

- [ ] Message registry has noncommutative algebra messages (4 files)
- [ ] Messages explain why AB ≠ BA for matrices
- [ ] Step-by-step explanations for noncommutative operations
- [ ] LaTeX formatter displays matrices as \mathbf{A}, operators as \hat{p}
- [ ] Simple formatter clearly distinguishes types
- [ ] Wolfram formatter uses proper matrix notation
- [ ] 25+ tests for formatting (LaTeX, simple, Wolfram)
- [ ] Educational examples exist
- [ ] All formatters distinguish matrix/operator/quaternion from scalar

---

## Wave 12: Examples, Documentation & Final Verification

### Verification Commands

```bash
# 1. Check examples exist
ls -la /Users/ahmedmashhour/Documents/work/math/mathhook/examples/ | grep -i "quantum\|matrix\|quaternion\|scalar"

# 2. Run examples
cargo run --example quantum_operators 2>&1 | head -20
cargo run --example matrix_algebra 2>&1 | head -20
cargo run --example quaternions 2>&1 | head -20
cargo run --example scalar_algebra 2>&1 | head -20

# 3. Check migration guide exists
ls -la /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/ | grep -i "migration"

# 4. Run ALL tests
cargo test 2>&1 | grep "test result"

# 5. Count total tests
cargo test -- --list 2>&1 | wc -l

# 6. Run benchmarks
cargo bench --no-run 2>&1 | grep -i "commutative\|scalar"
```

### Success Criteria Checklist

- [ ] quantum_operators.rs example exists and runs
- [ ] matrix_algebra.rs example exists and runs
- [ ] quaternions.rs example exists and runs
- [ ] scalar_algebra.rs example exists and runs (baseline)
- [ ] Migration guide document exists
- [ ] 60+ integration tests covering all four types
- [ ] Performance benchmarks show zero regression for commutative path
- [ ] Quality audit report shows 9.0+/10
- [ ] All 28 success criteria from orchestrator command met

---

## Final Comprehensive Verification

### Run Everything

```bash
#!/bin/bash
# Save as: verify_all_waves.sh

echo "=== COMPREHENSIVE NONCOMMUTATIVE ALGEBRA VERIFICATION ==="
echo ""

echo "1. Build Check"
cargo build --release 2>&1 | tail -5

echo ""
echo "2. Test Count"
BEFORE_TESTS=528  # Current test count
CURRENT_TESTS=$(cargo test -- --list 2>&1 | wc -l)
NEW_TESTS=$((CURRENT_TESTS - BEFORE_TESTS))
echo "Before: $BEFORE_TESTS tests"
echo "Current: $CURRENT_TESTS tests"
echo "New: $NEW_TESTS tests (Expected: 425+)"

echo ""
echo "3. Run All Tests"
cargo test 2>&1 | grep "test result"

echo ""
echo "4. Check Expression Size"
cargo test --release -p mathhook-core expression_size -- --nocapture 2>&1 | grep "Expression size"

echo ""
echo "5. Check No Emojis"
EMOJI_COUNT=$(grep -r "[\x{1F600}-\x{1F64F}\x{1F300}-\x{1F5FF}\x{1F680}-\x{1F6FF}]" /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/ 2>/dev/null | wc -l)
echo "Emoji count: $EMOJI_COUNT (Expected: 0)"

echo ""
echo "6. Check File Sizes"
find /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/ -name "*.rs" -exec wc -l {} \; | awk '$1 > 500 {print "VIOLATION: " $2 " has " $1 " lines (max 500)"}'

echo ""
echo "7. Run Examples"
cargo run --example matrix_algebra 2>&1 | head -10

echo ""
echo "8. Performance Benchmark"
cargo bench --no-run 2>&1 | grep -i "scalar\|commutative" | head -5

echo ""
echo "=== VERIFICATION COMPLETE ==="
```

### Success Criteria - All 28 Checkpoints

Run through the 28 success criteria from the orchestrator command and verify each one.

---

## How I (Second Verifier) Will Use This

When you say **"Verify Wave N"**, I will:

1. **Read this toolkit** for Wave N
2. **Run all verification commands** listed
3. **Check all success criteria** in the checklist
4. **Read modified files** to verify quality
5. **Report findings**:
   - ✅ PASS: All criteria met, ready for next wave
   - ⚠️ PARTIAL: Some issues found (list them)
   - ❌ FAIL: Critical issues (must fix before continuing)

---

## Example Verification Report Format

```
=== WAVE 1 VERIFICATION REPORT ===

Date: 2025-10-XX
Verifier: Claude (Second Reviewer)
Status: ✅ PASS / ⚠️ PARTIAL / ❌ FAIL

## Automated Checks:
✅ SymbolType enum found with 4 variants
✅ Commutativity enum found with 3 variants
✅ Expression::Mul signature updated
✅ Expression size: 48 bytes (acceptable)
✅ All tests pass: 558/558
✅ No emojis found
✅ All files ≤ 500 lines

## Manual Code Review:
✅ Symbol constructors well-documented
✅ Commutativity logic is correct
⚠️ Minor: Missing doctest in symbol.rs line 145

## Issues Found:
- None (or list specific issues)

## Recommendation:
✅ Wave 1 COMPLETE - proceed to Wave 2
(or: ⚠️ Fix minor issues before continuing)
(or: ❌ CRITICAL - must fix before Wave 2)
```

---

**This toolkit is ready to use. Say "Verify Wave N" after orchestrator completes each wave!**
