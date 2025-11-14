# Wave 3 Continuation Tasks: Number Theory & Polynomial Algorithms

**Mission**: Complete Gröbner bases and prime number functions to achieve Wave 3 completion (90-95%)

## Current Status

- **Gröbner Bases**: 70% (implemented but needs API fixes)
- **Number Theory**: 30% (intelligence system only, no implementations)
- **Overall Wave 3**: 60-70%

## Task Breakdown

### Priority 1: Fix Gröbner Basis API Compatibility (2-4 hours)

**Files to Update**:
- `crates/mathhook-core/src/algebra/groebner/buchberger.rs`
- `crates/mathhook-core/src/algebra/groebner/monomial_order.rs`
- `crates/mathhook-core/src/algebra/groebner/reduction.rs`
- `crates/mathhook-core/src/algebra/groebner/s_polynomial.rs`
- `crates/mathhook-core/src/algebra/groebner/mod.rs`

**Required Changes**:

1. **Iteration over Box<Vec<Expression>>** (occurs ~10 times):
   ```rust
   // OLD (doesn't compile):
   for factor in factors {
   
   // NEW (correct):
   for factor in factors.as_ref().iter() {
   // OR
   for factor in factors.iter() {
   ```

2. **Integer extraction from Expression** (occurs ~5 times):
   ```rust
   // OLD (doesn't compile):
   exp.to_integer().unwrap_or(0)
   
   // NEW (correct - pattern matching):
   if let Expression::Number(Number::Integer(n)) = exp.as_ref() {
       n.to_i64().unwrap_or(0)
   } else {
       0
   }
   ```

3. **Subtraction operation** (occurs 3 times):
   ```rust
   // OLD (doesn't compile):
   Expression::sub(expr1, expr2)
   
   // NEW (correct - use Sub trait):
   expr1 - expr2
   ```

4. **find_variables method** (occurs 1 time in mod.rs:187):
   ```rust
   // OLD (private method):
   poly.find_variables()
   
   // NEW (use public method or implement locally):
   // Option A: Make find_variables() public in Expression
   // Option B: Implement variable extraction locally in groebner module
   // Option C: Use existing collect module's variable extraction
   ```

**Testing After Fixes**:
```bash
cargo build -p mathhook-core
cargo test -p mathhook-core groebner::
```

### Priority 2: Implement Prime Number Functions (8-12 hours)

Create file: `crates/mathhook-core/src/algebra/prime_functions.rs`

**Required Functions**:

1. **Miller-Rabin Primality Test**:
   ```rust
   /// Miller-Rabin probabilistic primality test
   ///
   /// Uses k rounds of testing for probability (1 - 1/4^k) of correctness
   ///
   /// # Arguments
   /// * `n` - Number to test for primality
   /// * `k` - Number of rounds (default 40 for high confidence)
   ///
   /// # Examples
   /// ```rust
   /// assert!(is_prime_miller_rabin(&BigInt::from(17), 40));
   /// assert!(!is_prime_miller_rabin(&BigInt::from(15), 40));
   /// ```
   pub fn is_prime_miller_rabin(n: &BigInt, k: usize) -> bool {
       // Reference: SymPy ~/Documents/work/math/sympy/sympy/ntheory/primetest.py
       // TODO: Implement Miller-Rabin algorithm
   }
   ```

2. **Next Prime**:
   ```rust
   /// Find the next prime number ≥ n
   ///
   /// # Arguments
   /// * `n` - Starting number
   ///
   /// # Examples
   /// ```rust
   /// assert_eq!(next_prime(&BigInt::from(10)), BigInt::from(11));
   /// assert_eq!(next_prime(&BigInt::from(11)), BigInt::from(11));
   /// ```
   pub fn next_prime(n: &BigInt) -> BigInt {
       // Reference: SymPy ~/Documents/work/math/sympy/sympy/ntheory/generate.py
       // TODO: Implement next_prime using Miller-Rabin
   }
   ```

3. **Euler's Totient Function**:
   ```rust
   /// Euler's totient function φ(n)
   ///
   /// Counts numbers ≤ n that are coprime to n
   ///
   /// # Arguments
   /// * `n` - Input number
   ///
   /// # Examples
   /// ```rust
   /// assert_eq!(totient(&BigInt::from(9)), BigInt::from(6)); // φ(9) = 6
   /// assert_eq!(totient(&BigInt::from(10)), BigInt::from(4)); // φ(10) = 4
   /// ```
   pub fn totient(n: &BigInt) -> BigInt {
       // Reference: SymPy ~/Documents/work/math/sympy/sympy/ntheory/factor_.py
       // TODO: Implement using prime factorization: φ(n) = n * ∏(1 - 1/p)
   }
   ```

**Integration**:
- Add to `number_theory.rs` module
- Register functions in `NumberTheoryIntelligence`
- Update `functions/mod.rs` exports

**Testing**:
- Create `tests/prime_functions_tests.rs` with 50+ tests
- Validate against SymPy outputs
- Test edge cases: small primes, large primes, composite numbers

### Priority 3: SymPy Validation Testing (4-6 hours)

Create file: `tests/groebner_sympy_validation.rs`

**Test Categories**:

1. **Basic Gröbner Basis Computation** (10 tests):
   ```python
   # SymPy reference tests from ~/Documents/work/math/sympy/sympy/polys/tests/test_groebner.py
   
   # Test 1: Simple ideal <x - y, y^2 - 1>
   # Test 2: Circle and line intersection
   # Test 3: Polynomial system solving
   # etc.
   ```

2. **Monomial Orderings** (9 tests - 3 per ordering):
   - Lex ordering tests
   - Grlex ordering tests
   - Grevlex ordering tests

3. **Ideal Membership Testing** (10 tests):
   - Test `contains()` method correctness
   - Validate reduction algorithms

4. **S-Polynomial Tests** (10 tests):
   - Verify S-polynomial computation
   - Test leading term cancellation

5. **Buchberger Criterion** (10 tests):
   - Test optimization correctness
   - Verify relatively prime pair skipping

**SymPy Reference**:
```bash
# Primary reference:
~/Documents/work/math/sympy/sympy/polys/groebnertools.py
~/Documents/work/math/sympy/sympy/polys/tests/test_groebner.py

# Usage example:
from sympy import symbols, groebner
from sympy.polys.orderings import lex
x, y = symbols('x y')
F = [x**2 + y**2 - 1, x - y]
G = groebner(F, x, y, order=lex)
```

## Success Criteria

- ✅ All Gröbner tests compile and pass (23+ existing + 50+ new)
- ✅ Miller-Rabin, next_prime, totient implemented and tested
- ✅ 100+ total tests passing
- ✅ SymPy validation: 100% pass rate on comparison tests
- ✅ Zero compiler warnings
- ✅ Full documentation with doctests

## Estimated Timeline

- **Day 1 (4 hours)**: Fix Gröbner API compatibility, run existing tests
- **Day 2 (8 hours)**: Implement Miller-Rabin and next_prime with tests  
- **Day 3 (4 hours)**: Implement totient function and additional number theory features
- **Day 4 (4 hours)**: SymPy validation testing for all components
- **Day 5 (2 hours)**: Documentation, cleanup, final verification

**Total**: ~22 hours to complete Wave 3 to 90-95%

## Files Modified (Checklist)

**Gröbner Fixes**:
- [x] `crates/mathhook-core/src/algebra.rs` (already updated - added groebner module)
- [ ] `crates/mathhook-core/src/algebra/groebner/buchberger.rs` (API fixes needed)
- [ ] `crates/mathhook-core/src/algebra/groebner/monomial_order.rs` (API fixes needed)
- [ ] `crates/mathhook-core/src/algebra/groebner/reduction.rs` (API fixes needed)
- [ ] `crates/mathhook-core/src/algebra/groebner/s_polynomial.rs` (API fixes needed)
- [ ] `crates/mathhook-core/src/algebra/groebner/mod.rs` (find_variables issue)

**Number Theory**:
- [ ] `crates/mathhook-core/src/algebra/prime_functions.rs` (NEW - create this)
- [ ] `crates/mathhook-core/src/functions/number_theory.rs` (add implementations)

**Tests**:
- [ ] `tests/groebner_sympy_validation.rs` (NEW - create this)
- [ ] `tests/prime_functions_tests.rs` (NEW - create this)

## Notes for Continuation Agent

1. **SymPy is already available** at `~/Documents/work/math/sympy/`
2. **Symbolica reference** at `~/Documents/work/math/symbolica` (secondary reference)
3. **Existing test count baseline**: 677 tests (must not regress)
4. **CLAUDE.md compliance**: No emojis, full documentation, macro usage enforced
5. **32-byte Expression constraint**: Maintained throughout Gröbner implementation

## Quick Start Commands

```bash
# Fix Gröbner compilation
cargo build -p mathhook-core 2>&1 | grep error

# Run Gröbner tests (after fixes)
cargo test -p mathhook-core groebner

# Run number theory tests  
cargo test -p mathhook-core number_theory

# Full test suite
cargo test -p mathhook-core

# Check test count (should be ≥677)
cargo test -p mathhook-core 2>&1 | grep "test result"
```

## Reference Implementation Patterns

**How to extract integer from Expression**:
```rust
match exp.as_ref() {
    Expression::Number(Number::Integer(n)) => n.to_i64().unwrap_or(0),
    _ => 0
}
```

**How to iterate Box<Vec<Expression>>**:
```rust
for element in boxed_vec.iter() {
    // process element
}
```

**How to use Sub trait**:
```rust
let result = expr1 - expr2; // Sub trait is implemented
```

## Contact/Questions

This document created by Wave 3 Initial Assessment Agent.
Continuation agent should start with Priority 1 (Gröbner fixes) as it unblocks testing.
