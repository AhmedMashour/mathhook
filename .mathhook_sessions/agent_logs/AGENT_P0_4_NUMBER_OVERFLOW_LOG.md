# Agent P0-4: Number Safety Engineer

**Task**: P0-4 - Add Number Type Arithmetic with Overflow Handling
**Status**: NOT_STARTED
**Progress**: 0%
**Priority**: P0 (CRITICAL - CORRECTNESS)
**Estimated Duration**: 1 week
**Started**: -
**Last Update**: -

---

## Mission Briefing

Implement safe arithmetic operations for the Number type with overflow detection and BigInt promotion. Currently, the Number type has ZERO arithmetic trait implementations and can silently overflow.

**Current Problem**:
- No `impl Add/Sub/Mul/Div for Number`
- Zero uses of `checked_add`, `checked_mul`, etc.
- Integer overflow can occur silently (DANGEROUS)

**CLAUDE.md Violation**: "Rational arithmetic overflow: Use checked arithmetic operations. On overflow, promote to arbitrary precision BigInt or return an error. NEVER silently wrap."

**Reference Material**:
- Task details: `.mathhook_sessions/0.1_RELEASE_READINESS_AI_AGENT.md` (TASK P0-4)
- Number type constraints: `CLAUDE.md` (Number Type section)
- File: `crates/mathhook-core/src/core/number.rs`

---

## Current Objective

Waiting for launch command...

---

## Implementation Plan

### Phase 1: Define Error Type (Day 1)
- [ ] Ensure `MathError` enum exists (coordinate with P0-5)
- [ ] Add `NumericOverflow` variant if missing
- [ ] Add `DivisionByZero` variant if missing

### Phase 2: Implement Add Trait (Day 1-2)
- [ ] Implement `Add for Number` with overflow checking
- [ ] Handle Integer + Integer (use `checked_add`)
- [ ] Promote to BigInt on overflow
- [ ] Handle Rational + Rational
- [ ] Handle Float + Float
- [ ] Handle mixed-type addition (type promotion)
- [ ] Add 5+ tests for addition

### Phase 3: Implement Mul Trait (Day 2-3)
- [ ] Implement `Mul for Number` with overflow checking
- [ ] Handle Integer * Integer (use `checked_mul`)
- [ ] Promote to BigInt on overflow
- [ ] Handle Rational * Rational
- [ ] Handle Float * Float
- [ ] Handle mixed-type multiplication
- [ ] Add 5+ tests for multiplication

### Phase 4: Implement Sub Trait (Day 3-4)
- [ ] Implement `Sub for Number` with overflow checking
- [ ] Handle Integer - Integer (use `checked_sub`)
- [ ] Promote to BigInt on underflow
- [ ] Handle Rational - Rational
- [ ] Handle Float - Float
- [ ] Handle mixed-type subtraction
- [ ] Add 5+ tests for subtraction

### Phase 5: Implement Div Trait (Day 4-5)
- [ ] Implement `Div for Number` with division by zero check
- [ ] Add `is_zero()` helper method
- [ ] Return error on division by zero
- [ ] Handle Integer / Integer (exact division or promote to Rational)
- [ ] Reduce rationals to lowest terms
- [ ] Handle Rational / Rational
- [ ] Handle Float / Float
- [ ] Handle mixed-type division
- [ ] Add 5+ tests for division

### Phase 6: Comprehensive Testing (Day 5-6)
- [ ] Test overflow promotion to BigInt
- [ ] Test division by zero error handling
- [ ] Test rational reduction
- [ ] Test mixed-type operations
- [ ] Test precision loss documentation (Rational → Float)
- [ ] Add property-based tests if time allows
- [ ] Target: 20+ total tests

### Phase 7: Documentation & Integration (Day 6-7)
- [ ] Document overflow behavior in type documentation
- [ ] Add examples showing overflow handling
- [ ] Update any code using Number arithmetic
- [ ] Verify no regressions

---

## Completed Work

_Nothing yet - waiting for launch_

---

## Files to Modify

- [ ] `crates/mathhook-core/src/core/number.rs` (main implementation)
- [ ] `crates/mathhook-core/src/error.rs` (coordinate with P0-5)
- [ ] `crates/mathhook-core/tests/number_arithmetic_tests.rs` (create)

---

## Required Trait Implementations

### Add Trait
```rust
impl Add for Number {
    type Output = Result<Number, MathError>;

    fn add(self, other: Number) -> Result<Number, MathError> {
        // Integer + Integer with checked arithmetic
        // Rational + Rational
        // Float + Float
        // Mixed cases with type promotion
    }
}
```

### Mul Trait
```rust
impl Mul for Number {
    type Output = Result<Number, MathError>;

    fn mul(self, other: Number) -> Result<Number, MathError> {
        // Similar structure to Add
    }
}
```

### Sub Trait
```rust
impl Sub for Number {
    type Output = Result<Number, MathError>;

    fn sub(self, other: Number) -> Result<Number, MathError> {
        // Similar structure to Add
    }
}
```

### Div Trait
```rust
impl Div for Number {
    type Output = Result<Number, MathError>;

    fn div(self, other: Number) -> Result<Number, MathError> {
        // Check for division by zero first
        // Integer division: exact or promote to Rational
        // Rational arithmetic with reduction
    }
}
```

### Helper Methods
```rust
impl Number {
    pub fn is_zero(&self) -> bool {
        // Check if number equals zero
    }
}
```

---

## Tests Status

**Target**: 20+ tests
**Current**: 0 tests

### Test Categories
- [ ] Integer overflow promotes to BigInt (3 tests)
- [ ] Multiplication overflow promotes to BigInt (3 tests)
- [ ] Division by zero returns error (3 tests)
- [ ] Rational reduction to lowest terms (3 tests)
- [ ] Mixed-type arithmetic (4 tests)
- [ ] Edge cases (4+ tests)

---

## Key Test Cases

### Test 1: Integer Overflow
```rust
#[test]
fn test_integer_overflow_promotes_to_bigint() {
    let a = Number::Integer(i64::MAX);
    let b = Number::Integer(1);
    let result = (a + b).unwrap();

    match result {
        Number::BigInteger(n) => {
            assert_eq!(*n, BigInt::from(i64::MAX) + BigInt::from(1));
        }
        _ => panic!("Expected BigInteger promotion"),
    }
}
```

### Test 2: Division By Zero
```rust
#[test]
fn test_division_by_zero_returns_error() {
    let a = Number::Integer(5);
    let b = Number::Integer(0);
    let result = a / b;

    assert!(result.is_err());
    match result {
        Err(MathError::DivisionByZero) => { /* OK */ }
        _ => panic!("Expected DivisionByZero error"),
    }
}
```

### Test 3: Rational Reduction
```rust
#[test]
fn test_rational_reduction() {
    let a = Number::Integer(6);
    let b = Number::Integer(4);
    let result = (a / b).unwrap();

    // Should reduce to 3/2
    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(3));
            assert_eq!(r.denom(), &BigInt::from(2));
        }
        _ => panic!("Expected rational 3/2"),
    }
}
```

---

## Blockers

**Current Blockers**:
- May need coordination with P0-5 (Domain Guardian) for `MathError` enum definition

_Will document any blockers as they arise_

---

## Next Steps

1. Await launch command
2. Check if `MathError` exists (coordinate with P0-5 if needed)
3. Implement `Add` trait first (with tests)
4. Implement remaining traits systematically

---

## Questions for Manager

- Should I coordinate with P0-5 agent for error type definition?
- Preference for return type: `Result<Number, MathError>` or panic on overflow?

---

## Verification Checklist

When marking COMPLETE, verify:
- [ ] All four arithmetic traits implemented (Add, Sub, Mul, Div)
- [ ] All operations use checked arithmetic (no silent overflow)
- [ ] Overflow promotes to BigInt correctly
- [ ] Division by zero returns error
- [ ] Rational arithmetic reduces to lowest terms
- [ ] Mixed-type operations handled correctly
- [ ] 20+ tests passing
- [ ] Overflow behavior documented in type docs
- [ ] Code follows CLAUDE.md guidelines
- [ ] No regressions in existing tests

---

**Agent Status**: STANDBY - Ready to launch
**Blocking**: Safe arithmetic operations system-wide
