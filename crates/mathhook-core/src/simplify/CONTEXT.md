# Simplify Module Context

**Purpose**: High-performance expression simplification with modular architecture and minimal overhead

**Last Updated**: 2025-10-30

---

## Module Structure

### Files in This Module

**Core Interface** (208 lines):
- `simplify.rs` (208 lines) - Main simplification trait and dispatcher

**Arithmetic Simplification** (1,426 lines):
- `arithmetic.rs` (15 lines) - Arithmetic module aggregator
- `arithmetic/multiplication.rs` (560 lines) - Multiplication simplification (EXCEEDS 500-line limit)
- `arithmetic/addition.rs` (506 lines) - Addition simplification (EXCEEDS 500-line limit)
- `arithmetic/power.rs` (290 lines) - Power simplification
- `arithmetic/helpers.rs` (131 lines) - Helper functions for arithmetic

**Function and Constant Simplification** (176 lines):
- `functions.rs` (139 lines) - Function simplification (sin, cos, exp, log, etc.)
- `constants.rs` (37 lines) - Mathematical constant simplification (π, e, i)

**Total Module Size**: ~1,678 lines across 7 files

---

## Public API

### Key Traits
- `pub trait Simplify` - Main simplification interface for all expressions

### Key Functions
- `pub fn simplify_addition(terms: &[Expression]) -> Expression` - Simplify sum of terms
- `pub fn simplify_multiplication(factors: &[Expression]) -> Expression` - Simplify product of factors
- `pub fn simplify_power(base: &Expression, exp: &Expression) -> Expression` - Simplify exponentiation
- `pub fn simplify_function(name: &str, args: &[Expression]) -> Expression` - Simplify function calls
- `pub fn simplify_constant(constant: &MathConstant) -> Expression` - Simplify mathematical constants

---

## Dependencies

### Imports FROM Other Modules
**Core Types** (Heavy usage):
- `core/expression/` - Expression enum (central to all simplification)
- `core/symbol.rs` - Symbol type for variable manipulation
- `core/number.rs` - Number type for exact arithmetic
- `core/constants.rs` - Mathematical constants (π, e, i)

**Matrix Operations** (Moderate usage):
- `matrix/operations.rs` - Matrix simplification delegation

**Functions** (Light usage):
- `functions/` - Function-specific simplification rules

### Used BY Other Modules
**CRITICAL DEPENDENCY - Used by EVERY major module**:
- `algebra/` - All algebraic operations use simplification
- `calculus/` - Derivatives and integrals simplify results
- `functions/` - Function evaluation simplifies output
- `parser/` - Parsed expressions often simplified
- `educational/` - Educational steps show simplified forms
- `matrix/` - Matrix operations simplify results
- `pattern/` - Pattern matching uses simplified expressions

---

## Testing

### Module-Specific Test Commands
```bash
# All simplify tests
cargo test -p mathhook-core simplify

# Arithmetic simplification tests
cargo test -p mathhook-core arithmetic

# Function simplification tests
cargo test -p mathhook-core simplify_function

# Constant simplification tests
cargo test -p mathhook-core simplify_constant
```

### Test Coverage
- Unit tests: ~35 `#[test]` functions
- Integration tests: Cross-module simplification tests
- Doctests: Examples in public API

**Key Test Areas**:
- Arithmetic operations: Addition, multiplication, power
- Identity elements: x + 0 → x, x * 1 → x, x^1 → x
- Zero detection: 0 * x → 0
- Function simplification: sin(0) → 0, cos(0) → 1, exp(0) → 1
- Constant simplification: π, e, i exact values
- Commutative sorting: y + x → x + y
- Associativity flattening: (a + b) + c → a + b + c

---

## External References

### SymPy Equivalent
**Location**: `~/Documents/work/math/sympy/sympy/simplify/`
**Key Files**:
- `sympy/simplify/simplify.py` - Main simplification interface
- `sympy/simplify/radsimp.py` - Radical simplification
- `sympy/simplify/trigsimp.py` - Trigonometric simplification
- `sympy/simplify/ratsimp.py` - Rational simplification
- `sympy/core/basic.py` - Basic simplification methods

### Symbolica Equivalent
**Location**: `~/Documents/work/math/symbolica/src/`
**Key Files**:
- `symbolica/src/poly/` - Polynomial simplification
- `symbolica/src/atom/` - Atomic simplification operations
- `symbolica/src/normalize.rs` - Normalization and canonicalization

---

## Common Patterns & Pitfalls

### Design Patterns Used
1. **Dispatcher Pattern**: `simplify.rs` dispatches to specialized modules based on expression type
2. **Canonical Form Enforcement**: All operations produce canonical forms automatically
3. **Modular Architecture**: Separate modules for arithmetic, functions, constants
4. **Performance Optimization**: `#[inline(always)]` on hot paths, minimal allocations
5. **Idempotency**: `simplify(simplify(x)) == simplify(x)` (simplification is idempotent)

### Common Pitfalls
1. **Infinite Recursion**: Be careful not to create simplification loops
   - Example: `simplify_addition` must not call itself indirectly
   - Always ensure forward progress or termination

2. **Loss of Mathematical Equivalence**: NEVER simplify in ways that change meaning
   - `sqrt(x^2)` ≠ `x` (could be `-x` if x < 0)
   - `log(a*b)` ≠ `log(a) + log(b)` (branch cuts for complex numbers)
   - Always preserve domain restrictions

3. **Over-Simplification**: Don't "simplify" expressions that become more complex
   - `x + x + x` → `3*x` ✅ (simpler)
   - `(x + 1)*(x - 1)` → `x^2 - 1` ⚠️ (depends on context, may be more complex)
   - Measure complexity before/after

4. **Identity Element Handling**: Must handle all identity cases correctly
   - Addition: `x + 0 → x`, `0 + x → x`
   - Multiplication: `x * 1 → x`, `1 * x → x`, `x * 0 → 0`, `0 * x → 0`
   - Exponentiation: `x^1 → x`, `x^0 → 1` (except `0^0`), `1^x → 1`, `0^x → 0` (x > 0)

5. **Commutative Sorting**: Sort order must be consistent and total
   - Lexicographic by symbol name
   - Numbers before symbols
   - Complex expressions sorted by hash or canonical form

6. **Associativity Flattening**: Flatten nested operations correctly
   - `Add(Add(a, b), c)` → `Add(a, b, c)` ✅
   - Don't create deeply nested structures

7. **Rational Number Simplification**: Always reduce fractions
   - `6/4` → `3/2` (not `1.5` unless explicitly requested)
   - Preserve exactness in symbolic context

8. **Function-Specific Rules**: Different functions have different simplification rules
   - Trigonometric: `sin(-x) → -sin(x)`, `cos(-x) → cos(x)`
   - Exponential: `exp(0) → 1`, `exp(log(x)) → x` (for x > 0)
   - Logarithmic: `log(1) → 0`, `log(exp(x)) → x` (principal branch)

---

## CLAUDE.md Constraints (Module-Specific)

### File Size Compliance
**Current Status**: ⚠️ **2 files exceed 500 lines** (technical debt)

**File Size Violations**:
- `arithmetic/multiplication.rs` (560 lines) - **EXCEEDS 500 by 60 lines**
  - **Target Split**:
    - `arithmetic/multiplication/mod.rs` - Interface (≤150 lines)
    - `arithmetic/multiplication/basic.rs` - Basic multiplication rules (≤200 lines)
    - `arithmetic/multiplication/symbolic.rs` - Symbolic multiplication (≤200 lines)
    - `arithmetic/multiplication/optimization.rs` - Performance optimization (≤150 lines)

- `arithmetic/addition.rs` (506 lines) - **EXCEEDS 500 by 6 lines**
  - **Target Split**:
    - `arithmetic/addition/mod.rs` - Interface (≤150 lines)
    - `arithmetic/addition/basic.rs` - Basic addition rules (≤200 lines)
    - `arithmetic/addition/symbolic.rs` - Symbolic addition (≤200 lines)

**Priority**: MODERATE - These files should be split in next cleanup wave

### Module-Specific Rules
1. **Idempotency**: Simplification MUST be idempotent
   - `expr.simplify().simplify() == expr.simplify()`
   - Test this property extensively

2. **Mathematical Equivalence**: Simplification MUST preserve mathematical meaning
   - NEVER change domain or range
   - NEVER introduce or remove discontinuities
   - Test against SymPy/Symbolica for correctness

3. **Canonical Form**: All operations MUST produce canonical form
   - Consistent sorting for commutative operations
   - Flattened associative operations
   - Reduced rational numbers

4. **Performance**: Simplification is in the HOT PATH
   - Use `#[inline(always)]` for dispatch
   - Minimize allocations
   - Benchmark all changes

5. **Zero Detection**: Must correctly detect and handle zero
   - `0 * x → 0` for all x
   - But NOT `0 / x → 0` (undefined at x = 0)

---

## Recent Changes

### Last 3 Major Modifications
1. **Modular Architecture Refactor**: Split into arithmetic, functions, constants (Date TBD)
   - Separated concerns for better maintainability
   - Performance-optimized dispatch
   - Reduced file sizes (though 2 still exceed 500)

2. **Complex and Matrix Delegation**: Added specialized simplification (Date TBD)
   - `Expression::Complex` → `Expression::simplify_complex()`
   - `Expression::Matrix` → `self.simplify_matrix()`
   - Delegation to specialized modules

3. **Calculus Expression Simplification**: Added support for calculus operations (Date TBD)
   - Derivative simplification
   - Integral simplification
   - Limit, Sum, Product simplification
   - Properly handles sub-expression recursion

---

## Technical Debt

### Known Issues
1. **File Size Violations** (MODERATE):
   - `arithmetic/multiplication.rs` (560 lines) exceeds 500-line limit
   - `arithmetic/addition.rs` (506 lines) exceeds 500-line limit
   - **Impact**: Harder to navigate and modify
   - **Priority**: MODERATE - Should be split in cleanup wave

2. **Limited Function Simplification**: Not all functions have complete simplification rules
   - Some special functions missing simplification
   - Transcendental function composition not fully simplified
   - **Future**: Expand function simplification coverage

3. **No Advanced Simplification**: Complex simplification strategies not implemented
   - Trigonometric identities (basic only)
   - Rational expression simplification (limited)
   - Radical simplification (basic only)
   - **Future**: Add advanced simplification module (see `algebra/advanced_simplify`)

4. **Performance Profiling**: Simplification is hot path but not fully profiled
   - Need benchmarks for common patterns
   - Identify optimization opportunities
   - **Future**: Add comprehensive benchmark suite

### Future Improvements
1. Split `arithmetic/multiplication.rs` and `arithmetic/addition.rs` using module pattern
2. Expand function simplification rules (more trigonometric identities, special functions)
3. Add advanced simplification strategies (see `algebra/advanced_simplify/`)
4. Implement complexity heuristics (don't "simplify" to more complex forms)
5. Add simplification caching for common sub-expressions
6. Profile and optimize hot paths (benchmark-driven optimization)
7. Add property-based testing (commutativity, associativity, distributivity)

---

## Integration Points

### Simplification Flow
```
Expression → Simplify trait
    ↓
Dispatcher (simplify.rs) matches expression type:
    - Number/Symbol → clone (already canonical)
    - Add → arithmetic::simplify_addition
    - Mul → arithmetic::simplify_multiplication
    - Pow → arithmetic::simplify_power
    - Function → functions::simplify_function
    - Constant → constants::simplify_constant
    - Complex → Expression::simplify_complex
    - Matrix → self.simplify_matrix
    - Relation/Piecewise/Set/Interval/Calculus/MethodCall → recursive simplification
    ↓
Specialized simplification:
    - Combine like terms (addition)
    - Combine like factors (multiplication)
    - Simplify exponents (power)
    - Apply function-specific rules (functions)
    - Evaluate exact constants (constants)
    ↓
Result: Canonical form expression
```

### Arithmetic Simplification Flow
```
Addition (simplify_addition):
    1. Flatten nested additions: (a + b) + c → a + b + c
    2. Simplify each term recursively
    3. Combine numeric terms: 2 + 3 → 5
    4. Collect like symbolic terms: 2x + 3x → 5x
    5. Sort terms in canonical order: y + x → x + y
    6. Remove identity: x + 0 → x
    7. Return canonical Add or single term

Multiplication (simplify_multiplication):
    1. Flatten nested multiplications: (a * b) * c → a * b * c
    2. Simplify each factor recursively
    3. Detect zero: 0 * x → 0
    4. Combine numeric factors: 2 * 3 → 6
    5. Collect powers of same base: x * x^2 → x^3
    6. Sort factors in canonical order: y * x → x * y
    7. Remove identity: x * 1 → x
    8. Return canonical Mul or single factor

Power (simplify_power):
    1. Simplify base and exponent recursively
    2. Handle identity cases:
       - x^1 → x
       - x^0 → 1 (except 0^0)
       - 1^x → 1
       - 0^x → 0 (for x > 0)
    3. Evaluate numeric powers: 2^3 → 8
    4. Simplify nested powers: (x^a)^b → x^(a*b)
    5. Return canonical Pow or simplified result
```

### Function Simplification Flow
```
Function simplification (simplify_function):
    1. Simplify all arguments recursively
    2. Check function registry for simplification rules
    3. Apply function-specific rules:
       - sin(0) → 0, cos(0) → 1
       - sin(-x) → -sin(x), cos(-x) → cos(x)
       - exp(0) → 1, exp(log(x)) → x
       - log(1) → 0, log(exp(x)) → x
       - sqrt(0) → 0, sqrt(1) → 1
    4. If no rule applies, return function with simplified args
```

### Canonical Form Properties
```
Canonical form ensures:
1. Idempotency: simplify(simplify(x)) == simplify(x)
2. Structural equality: If a == b mathematically, then simplify(a) == simplify(b) structurally
3. Consistency: Same expression always simplifies to same canonical form
4. Performance: Repeated simplification has no additional cost (O(1))
5. Determinism: Simplification result is deterministic (no randomness)
```

---

## Performance Optimization

### Hot Path Analysis
Simplification is called on EVERY mathematical operation result. Performance is critical.

**Optimization Strategies**:
1. **Inline Aggressively**: Use `#[inline(always)]` on dispatch and simple operations
2. **Avoid Allocations**: Reuse expressions where possible, avoid unnecessary clones
3. **Early Termination**: Return immediately for trivial cases (Number, Symbol)
4. **Lazy Evaluation**: Don't simplify until needed (though MathHook simplifies eagerly)
5. **Caching**: Cache simplified results for common sub-expressions (future improvement)

**Benchmarking Requirements**:
- Simplification of polynomial expressions (degree 2, 5, 10, 20)
- Simplification of trigonometric expressions
- Simplification of nested operations
- Simplification with many terms/factors (10, 100, 1000)

**Performance Targets**:
- Simple expressions (<10 nodes): <10µs
- Moderate expressions (10-100 nodes): <100µs
- Large expressions (100-1000 nodes): <1ms
- Must be faster than SymPy (Python) by 10-100x

---

## Correctness Validation

### Testing Against SymPy
All simplification results MUST be mathematically equivalent to SymPy results.

**Validation Process**:
1. Generate test cases (expressions)
2. Simplify in MathHook
3. Simplify in SymPy
4. Convert both to canonical string form
5. Compare results (allowing for notational differences)

**Critical Test Cases**:
- `2 + 3 → 5` (basic arithmetic)
- `x + 0 → x` (identity)
- `x * 1 → x` (identity)
- `0 * x → 0` (zero)
- `x^1 → x` (identity)
- `x^0 → 1` (identity, except 0^0)
- `sin(0) → 0` (function simplification)
- `cos(0) → 1` (function simplification)
- `exp(0) → 1` (function simplification)
- `log(1) → 0` (function simplification)
- `sqrt(4) → 2` (exact simplification)
- `2x + 3x → 5x` (like term collection)
- `x * x → x^2` (power collection)
- `(x + y) + z → x + y + z` (flattening)

---

**Module Owner**: Core team
**Related Waves**: Modular architecture refactor, Performance optimization waves
