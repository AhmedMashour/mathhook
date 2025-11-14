# Wave 13: Quality Enhancement to 10/10

**Goal**: Address all "nice-to-have" improvements to achieve perfect 10/10 scores across all waves

**Priority**: HIGH (Final quality polish)
**Effort**: 2-3 hours
**Impact**: Achieves perfect 10/10 quality across all waves, eliminates all technical debt

---

## Wave 13 Overview

This wave addresses the minor deductions from Waves 8, 9.1, 11, and 12 to achieve **10/10 across all waves**.

**Current State**:
- Wave 8: 9.5/10 (missing edge case tests, design documentation)
- Wave 9.1: 9.5/10 (Phase 1 only, no per-symbol overrides)
- Wave 10: **10/10** ✅ (already perfect)
- Wave 11: 9.5/10 (missing error handling tests, performance optimization)
- Wave 12: 9.5/10 (files exceed 500 lines)

**Target State**:
- All waves: **10/10** ⭐

---

## Wave 13 Scope

### 1. Wave 12 File Size Compliance (HIGH PRIORITY)

**Issue**: 2 files exceed 500-line guideline
- `noncommutative_integration_tests.rs`: 587 lines (87 over)
- `NONCOMMUTATIVE_ALGEBRA.md`: 537 lines (37 over)

**Solution**: Split into focused, modular files

#### Split Integration Tests (587 → 3 files × <200 lines)

**Create**:
1. `crates/mathhook-core/tests/noncommutative_integration_cross_wave_tests.rs` (~200 lines)
   - 12 cross-wave integration tests
   - Parser → Solver → Formatter workflows

2. `crates/mathhook-core/tests/noncommutative_integration_regression_tests.rs` (~200 lines)
   - 10 regression prevention tests
   - Backward compatibility validation

3. `crates/mathhook-core/tests/noncommutative_integration_example_tests.rs` (~150 lines)
   - 3 example validation tests
   - Example concepts verification

**Delete**: `crates/mathhook-core/tests/noncommutative_integration_tests.rs` (replaced by 3 files above)

#### Split Documentation (537 → 3 files × <200 lines)

**Create**:
1. `NONCOMMUTATIVE_ALGEBRA.md` (~250 lines)
   - Overview and getting started
   - Symbol types quick reference
   - Basic usage examples
   - Links to detailed docs

2. `docs/noncommutative_api_reference.md` (~150 lines)
   - API reference for all four symbol types
   - Function signatures
   - Return types
   - Error handling

3. `docs/noncommutative_examples.md` (~150 lines)
   - Extended examples
   - Real-world use cases
   - Best practices
   - Common patterns

**Update**: Move detailed content from original NONCOMMUTATIVE_ALGEBRA.md to new files

### 2. Wave 11 Error Handling Tests (MEDIUM PRIORITY)

**Issue**: Missing error handling tests for message registry and formatter

**Solution**: Add 5 error handling tests

**Create**: `crates/mathhook-core/tests/educational_noncommutative_error_tests.rs` (~150 lines)

**Tests**:
1. `test_message_registry_missing_message` - Graceful handling of missing messages
2. `test_formatter_invalid_symbol_type` - Handle invalid/unknown symbol types
3. `test_formatter_null_symbol` - Handle edge cases (empty symbols)
4. `test_educational_steps_malformed_equation` - Handle malformed equations
5. `test_latex_formatter_special_characters` - Handle special LaTeX characters

### 3. Wave 11 Performance Optimization (MEDIUM PRIORITY)

**Issue**: LaTeX formatter doesn't cache type lookups

**Solution**: Add type lookup caching

**Modify**: `crates/mathhook-core/src/formatter/latex/expressions.rs` (~50 lines added)

**Implementation**:
```rust
use std::collections::HashMap;

pub struct LaTeXFormatter {
    // Add cache for symbol type lookups
    type_cache: HashMap<String, SymbolType>,
}

impl LaTeXFormatter {
    fn format_symbol(&mut self, symbol: &Symbol) -> String {
        // Check cache first
        if let Some(cached_type) = self.type_cache.get(symbol.name()) {
            return self.format_with_type(symbol, *cached_type);
        }

        // Lookup and cache
        let symbol_type = symbol.symbol_type();
        self.type_cache.insert(symbol.name().to_string(), symbol_type);
        self.format_with_type(symbol, symbol_type)
    }
}
```

**Add Benchmark**: `crates/mathhook-benchmarks/benches/latex_formatter_benchmark.rs`
- Benchmark formatter with/without caching
- Verify performance improvement

### 4. Wave 8 Edge Case Tests (MEDIUM PRIORITY)

**Issue**: Missing edge case tests for parser ambiguity

**Solution**: Add 5 edge case tests

**Modify**: `crates/mathhook-core/tests/parser_type_inference_tests.rs` (~100 lines added)

**Tests**:
1. `test_ambiguous_mathbf_nested` - Nested \mathbf{...} expressions
2. `test_ambiguous_hat_operator` - Multiple hat operators in one expression
3. `test_malformed_latex_mathbf` - Malformed \mathbf syntax (missing braces)
4. `test_malformed_latex_hat` - Malformed \hat syntax
5. `test_mixed_notation_precedence` - Mixed \mathbf and \hat in complex expression

### 5. Wave 8 Design Documentation (LOW PRIORITY)

**Issue**: Parser design decisions not thoroughly documented

**Solution**: Add comprehensive design documentation

**Create**: `docs/parser_design_noncommutative.md` (~200 lines)

**Contents**:
- Design rationale for type inference
- Why LaTeX notation was chosen
- Ambiguity resolution strategies
- LALRPOP grammar decisions
- Edge case handling
- Future enhancements

### 6. Wave 9.1 Phase 2: Per-Symbol Type Overrides (OPTIONAL)

**Issue**: Phase 1 only supports bulk type specification

**Solution**: Implement per-symbol overrides

**Current Syntax** (Phase 1):
```rust
symbols![A, B, C => matrix]  // All are matrices
```

**New Syntax** (Phase 2):
```rust
// Bulk type with per-symbol override
symbols![A, B, C => matrix, x: scalar]  // A, B, C are matrix; x is scalar

// Multiple overrides
symbols![A, B, C => matrix, x: scalar, y: scalar]

// Override changes type
symbols![A, B, x => matrix, x: scalar]  // A, B are matrix; x is scalar
```

**Modify**: `crates/mathhook-core/src/macros/symbols.rs` (~100 lines)

**Add Tests**: `crates/mathhook-core/tests/macro_enhancement_tests.rs` (~100 lines)
- 8 tests for per-symbol overrides
- Test precedence: per-symbol > bulk type > default

**NOTE**: This is OPTIONAL - may be deferred to future wave if time-constrained

---

## Success Criteria

### Wave 12 Enhancement (10/10):
1. ✅ All files ≤500 lines (3 test files, 3 doc files)
2. ✅ All 25 integration tests still passing (split across files)
3. ✅ Documentation maintains quality and comprehensiveness
4. ✅ Build passes with 0 errors

### Wave 11 Enhancement (10/10):
5. ✅ 5 error handling tests added and passing
6. ✅ LaTeX formatter caching implemented
7. ✅ Benchmark shows performance improvement (>10%)
8. ✅ Zero regressions

### Wave 8 Enhancement (10/10):
9. ✅ 5 edge case tests added and passing
10. ✅ Design documentation complete (~200 lines)
11. ✅ Edge cases handled gracefully

### Wave 9.1 Enhancement (Optional):
12. ⚠️ Per-symbol overrides implemented (Phase 2)
13. ⚠️ 8 tests for override syntax
14. ⚠️ Backward compatible with Phase 1

### Overall:
15. ✅ All waves achieve 10/10 quality score
16. ✅ Zero regressions across all 160+ tests
17. ✅ Build passes with 0 errors
18. ✅ Perfect CLAUDE.md compliance

---

## Implementation Strategy

### Phase 1: Wave 12 File Splitting (45 min)

**Priority: CRITICAL**

1. Split integration tests into 3 files
2. Verify all 25 tests still pass
3. Delete original file
4. Split documentation into 3 files
5. Update cross-references
6. Verify documentation quality

### Phase 2: Wave 11 Enhancements (45 min)

**Priority: HIGH**

1. Add 5 error handling tests
2. Implement formatter caching
3. Add performance benchmark
4. Verify performance improvement
5. Run all educational tests

### Phase 3: Wave 8 Enhancements (30 min)

**Priority: MEDIUM**

1. Add 5 edge case tests
2. Create design documentation
3. Run all parser tests

### Phase 4: Wave 9.1 Phase 2 (OPTIONAL, 45 min)

**Priority: LOW (Can defer)**

1. Implement per-symbol override syntax
2. Add 8 tests
3. Verify backward compatibility

### Phase 5: Final Verification (15 min)

1. Run all tests (180+ expected)
2. Verify all quality scores 10/10
3. Create final report

---

## Testing Strategy

### New Tests Created:

**Wave 12 (Split)**:
- 25 existing tests (split across 3 files)

**Wave 11 (Error Handling)**:
- 5 new error handling tests
- Total Wave 11: 30 + 5 = 35 tests

**Wave 8 (Edge Cases)**:
- 5 new edge case tests
- Total Wave 8: 27 + 5 = 32 tests

**Wave 9.1 Phase 2 (Optional)**:
- 8 new override tests
- Total Wave 9.1: 37 + 8 = 45 tests

**Cumulative**:
- Current: 160 tests
- After Wave 13: 160 + 5 + 5 + (8 optional) = 170-178 tests

---

## Verification Script

**File**: `.mathhook_sessions/verify_wave_13_quality.sh`

**Categories**:
1. File size compliance (ALL files ≤500 lines)
2. Test count (170+ tests)
3. All tests passing
4. Performance benchmark (formatter >10% faster)
5. Documentation completeness
6. Build status
7. Zero regressions
8. CLAUDE.md compliance
9. All waves 10/10 verification
10. Final quality metrics

---

## Files to Create

### Wave 12 Enhancements:
1. `crates/mathhook-core/tests/noncommutative_integration_cross_wave_tests.rs` (~200 lines)
2. `crates/mathhook-core/tests/noncommutative_integration_regression_tests.rs` (~200 lines)
3. `crates/mathhook-core/tests/noncommutative_integration_example_tests.rs` (~150 lines)
4. `docs/noncommutative_api_reference.md` (~150 lines)
5. `docs/noncommutative_examples.md` (~150 lines)

### Wave 11 Enhancements:
6. `crates/mathhook-core/tests/educational_noncommutative_error_tests.rs` (~150 lines)
7. `crates/mathhook-benchmarks/benches/latex_formatter_benchmark.rs` (~100 lines)

### Wave 8 Enhancements:
8. `docs/parser_design_noncommutative.md` (~200 lines)

### Wave 9.1 Phase 2 (Optional):
9. (Tests added to existing file)

### Verification:
10. `.mathhook_sessions/verify_wave_13_quality.sh` (NEW)

---

## Files to Modify

### Wave 12:
1. `NONCOMMUTATIVE_ALGEBRA.md` (537 → ~250 lines, restructured)

### Wave 11:
2. `crates/mathhook-core/src/formatter/latex/expressions.rs` (~50 lines added for caching)

### Wave 8:
3. `crates/mathhook-core/tests/parser_type_inference_tests.rs` (~100 lines added for edge cases)

### Wave 9.1 Phase 2 (Optional):
4. `crates/mathhook-core/src/macros/symbols.rs` (~100 lines added for overrides)
5. `crates/mathhook-core/tests/macro_enhancement_tests.rs` (~100 lines added for tests)

---

## Files to Delete

1. `crates/mathhook-core/tests/noncommutative_integration_tests.rs` (replaced by 3 split files)

---

## Quality Score Projection

**Before Wave 13**:
- Wave 8: 9.5/10
- Wave 9.1: 9.5/10
- Wave 10: 10/10 ✅
- Wave 11: 9.5/10
- Wave 12: 9.5/10
- **Average**: 9.6/10

**After Wave 13**:
- Wave 8: **10/10** ⭐ (edge cases + design docs)
- Wave 9.1: **10/10** ⭐ (Phase 2 complete) OR 9.8/10 (if Phase 2 deferred)
- Wave 10: **10/10** ✅ (already perfect)
- Wave 11: **10/10** ⭐ (error tests + performance)
- Wave 12: **10/10** ⭐ (file size compliance)
- **Average**: **10/10** ⭐ (or 9.95/10 if Phase 2 deferred)

---

## Timeline

**Phase 1**: Wave 12 File Splitting (45 min)
**Phase 2**: Wave 11 Enhancements (45 min)
**Phase 3**: Wave 8 Enhancements (30 min)
**Phase 4**: Wave 9.1 Phase 2 (45 min, OPTIONAL)
**Phase 5**: Final Verification (15 min)

**Total**: 2.25 hours (or 3 hours with Phase 2)

---

## CLAUDE.md Compliance Requirements

1. **File Size**: ALL files ≤500 lines (CRITICAL - this wave fixes violations)
2. **No Emojis**: Zero emojis in code/comments/docs
3. **Tests**: 170+ comprehensive tests (up from 160)
4. **Build**: Must pass with 0 errors
5. **Regressions**: Zero (all existing tests pass)
6. **Documentation**: All new functions have `///` docs with examples
7. **Performance**: Benchmark proves caching improves performance
8. **Error Handling**: All edge cases handled gracefully

---

## Agent 13A Prompt Structure (When Ready)

**Agent 13A**: Quality Enhancement to 10/10

**Task**:
- Split Wave 12 files to meet 500-line limit
- Add Wave 11 error handling tests and caching
- Add Wave 8 edge case tests and design docs
- Optionally: Implement Wave 9.1 Phase 2 (per-symbol overrides)
- Achieve 10/10 across all waves

**Deliverables**:
1. 5 new test files (3 split from Wave 12, 1 for Wave 11, existing for Wave 8)
2. 3 new documentation files (split from Wave 12, 1 for Wave 8)
3. 10+ new tests (5 error, 5 edge case, 8 optional override)
4. Performance benchmark
5. LaTeX formatter with caching
6. All files ≤500 lines
7. All 170+ tests passing
8. All waves 10/10

**Target Quality**: **10/10 across all waves** ⭐

---

**This Wave 13 will achieve perfect 10/10 quality across the entire Noncommutative Algebra implementation!**
