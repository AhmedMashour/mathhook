# Wave 3 Phase 3 - Optimization #1: Inline Directives

**Date**: 2025-10-22
**Optimization**: Add `#[inline]` attributes to hot-path functions
**Expected Impact**: 150-200ns improvement for expression_creation benchmark

## Changes Made

### 1. Expression Constructors (`basic.rs`)

Added `#[inline]` to simple, frequently-called constructors:

**Inlined Functions**:
- `Expression::number()` - Number wrapper (1 line)
- `Expression::integer()` - Integer constructor (1 line)
- `Expression::big_integer()` - BigInt constructor (small with branch)
- `Expression::rational()` - Rational constructor (2 lines)
- `Expression::float()` - Float constructor (1 line)
- `Expression::symbol()` - Symbol wrapper (1 line)
- `Expression::constant()` - Constant wrapper (1 line)
- `Expression::pi()` - Pi constant (1 line)
- `Expression::e()` - e constant (1 line)
- `Expression::i()` - Imaginary unit (1 line)
- `Expression::infinity()` - Infinity constant (1 line)
- `Expression::negative_infinity()` - Negative infinity (1 line)
- `Expression::undefined()` - Undefined constant (1 line)
- `Expression::golden_ratio()` - Golden ratio (1 line)
- `Expression::euler_gamma()` - Euler gamma (1 line)
- `Expression::equation()` - Equation constructor (small)
- `Expression::relation()` - Relation constructor (small)
- `Expression::div()` - Division constructor (delegates to mul/pow)
- `Expression::div_checked()` - Checked division (small with branch)

**NOT Inlined** (too complex or call non-inlined functions):
- `Expression::add()` - Delegates to `simplify_addition()` (complex)
- `Expression::mul()` - Delegates to `simplify_multiplication()` (complex)
- `Expression::pow()` - Delegates to `simplify_power()` (complex)

### 2. Symbol Constructors (`symbol.rs`)

Added `#[inline]` to getters and simple wrappers:

**Inlined Functions**:
- `Symbol::new()` - Wrapper for `scalar()` (1 line)
- `Symbol::name()` - Getter (1 line)
- `Symbol::symbol_type()` - Getter (1 line)
- `Symbol::commutativity()` - Small match (3 lines)

**NOT Inlined** (too complex, has locking):
- `Symbol::scalar()` - Complex with string interning and OnceLock
- `Symbol::matrix()` - Calls `intern_symbol()` with mutex lock
- `Symbol::operator()` - Calls `intern_symbol()` with mutex lock
- `Symbol::quaternion()` - Calls `intern_symbol()` with mutex lock
- `Symbol::intern_symbol()` - Mutex operations (never inline)

## Rationale

### When to Use `#[inline]`

**DO inline:**
1. **Trivial wrappers** (1-2 lines): `Expression::symbol()`, `Symbol::new()`
2. **Simple getters** (field access): `symbol.name()`, `symbol.symbol_type()`
3. **Small constructors** (<10 lines, no complex logic): `Expression::integer()`
4. **Frequently called in hot paths**: Expression creation, symbol access

**DON'T inline:**
1. **Complex logic** (>10 lines): Simplification functions
2. **Functions with locks**: `intern_symbol()` with Mutex
3. **Functions calling non-inlined functions**: `add()`, `mul()`, `pow()`
4. **Rarely called functions**: Specialized constructors

### Expected Performance Impact

From Phase 2 analysis:
- **expression_creation** regression: +70.7% (749ns vs 500ns baseline)
- **Root cause**: Symbol interning overhead + function call overhead
- **Inlining benefit**: Eliminate function call overhead for hot paths

**Estimated improvement**:
- Expression constructors: ~50-100ns (eliminate call overhead)
- Symbol getters: ~30-50ns (inline field access)
- **Total**: ~150-200ns improvement on expression_creation benchmark

### Why Not More Aggressive Inlining?

1. **Code bloat**: Inlining too much increases binary size
2. **Instruction cache misses**: Large inlined functions hurt I-cache
3. **Compilation time**: More inlining = slower compilation
4. **Diminishing returns**: Only hot paths benefit from inlining

## Verification

```bash
# Build with optimizations
cargo build --release -p mathhook-core

# Run tests
cargo test --workspace

# Run benchmarks
cargo bench --bench expression_creation
```

## Next Steps

After verifying tests pass:
1. Run benchmarks to measure actual improvement
2. Compare results against Phase 2 baseline (749ns)
3. Decide if further optimizations needed or move to optimization #2
