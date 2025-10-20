# Wave 11: Educational, Message Registry & Formatter for Noncommutative Algebra

**Goal**: Update educational message registries and LaTeX formatter to support noncommutative algebra

**Priority**: MEDIUM-HIGH (Educational features are a key differentiator)
**Effort**: 2-3 hours
**Impact**: Enables proper educational explanations and LaTeX output for matrix/operator equations

---

## Problem with Current Educational System

### Current State

The existing educational system and formatters assume commutativity:

```rust
// Current educational messages don't distinguish:
"Multiply both sides by 1/a"  // Assumes division is commutative

// But for matrices, this is ambiguous:
"Multiply both sides by A^(-1)"  // LEFT or RIGHT multiplication?
```

**Issues**:
1. ❌ Educational messages don't specify left vs right division
2. ❌ LaTeX formatter doesn't use proper notation for matrices (\mathbf{A})
3. ❌ LaTeX formatter doesn't use proper notation for operators (\hat{p})
4. ❌ No educational explanations for why order matters
5. ❌ Step-by-step solver doesn't explain left/right division choice

---

## Wave 11 Scope

### 1. Educational Message Registry Updates

**Files**: `crates/mathhook-core/src/educational/messages/`

**Changes Needed**:
- Add messages for left division: "Multiply both sides on the LEFT by A^(-1)"
- Add messages for right division: "Multiply both sides on the RIGHT by A^(-1)"
- Add explanation messages: "Order matters because A is noncommutative"
- Update existing division messages to be commutativity-aware

**New Message Categories**:
```rust
// Left division messages
"left_multiply_inverse" -> "Multiply both sides on the LEFT by {inverse}"
"left_division_explanation" -> "For equation A*X = B, we multiply LEFT by A^(-1) to isolate X"

// Right division messages
"right_multiply_inverse" -> "Multiply both sides on the RIGHT by {inverse}"
"right_division_explanation" -> "For equation X*A = B, we multiply RIGHT by A^(-1) to isolate X"

// Commutativity messages
"noncommutative_warning" -> "Order matters: {A} is noncommutative ({type})"
"commutator_explanation" -> "[A,B] = AB - BA measures how much A and B fail to commute"
```

### 2. LaTeX Formatter Updates

**File**: `crates/mathhook-core/src/formatter/latex.rs` (or similar)

**Changes Needed**:
- Format Matrix symbols as `\mathbf{A}` (bold)
- Format Operator symbols as `\hat{p}` (hat notation)
- Format Quaternion symbols as `\mathbb{H}_{i}` or just `i, j, k`
- Update expression formatting to preserve order for noncommutative operations

**Examples**:
```rust
// Current (commutative):
x -> "x"
A -> "A"

// New (type-aware):
x (Scalar) -> "x"
A (Matrix) -> "\\mathbf{A}"
p (Operator) -> "\\hat{p}"
i (Quaternion) -> "i"  // or "\\mathbb{H}_i"
```

**Equation Formatting**:
```rust
// Matrix equation:
A*X = B  ->  "\\mathbf{A} \\mathbf{X} = \\mathbf{B}"

// Operator equation:
H*psi = E*psi  ->  "\\hat{H} \\psi = E \\psi"

// Mixed:
2*A*X = B  ->  "2 \\mathbf{A} \\mathbf{X} = \\mathbf{B}"
```

### 3. Step-by-Step Educational Explanations

**File**: `crates/mathhook-core/src/educational/step_by_step/`

**Update Existing**:
- `MatrixEquationSolver` already generates step-by-step (from Wave 10)
- Enhance with educational messages from registry
- Add explanations of why order matters

**New Explanations**:
```rust
// Left division steps:
Step 1: "Identify equation form: A*X = B (left multiplication)"
Step 2: "Multiply both sides on the LEFT by A^(-1)"
Step 3: "Simplify: A^(-1)*(A*X) = A^(-1)*B"
Step 4: "Use associativity: (A^(-1)*A)*X = A^(-1)*B"
Step 5: "A^(-1)*A = I (identity matrix)"
Step 6: "Solution: X = A^(-1)*B"

// With explanation of why:
Explanation: "We multiply on the LEFT because the variable X is on the RIGHT of A.
              For matrices, A^(-1)*B ≠ B*A^(-1), so position matters."
```

### 4. Test Suite Updates

**File**: `crates/mathhook-core/tests/educational_noncommutative_tests.rs` (create new)

**Test Categories** (25+ tests total):
1. **Message registry tests** (8 tests)
2. **LaTeX formatter tests** (8 tests)
3. **Step-by-step explanation tests** (9 tests)

---

## Implementation Strategy

### Phase 1: Message Registry Updates (45 min)

1. **Locate message registry files**
   - Find educational message definitions
   - Understand current structure

2. **Add noncommutative messages**
   - Left/right division messages
   - Commutativity explanation messages
   - Order-matters warnings

3. **Update existing messages**
   - Make division messages context-aware
   - Add commutativity parameters

### Phase 2: LaTeX Formatter Updates (45 min)

1. **Add symbol type detection**
   - Check Symbol::symbol_type()
   - Route to appropriate formatter

2. **Implement type-specific formatting**
   - Matrix: `\mathbf{A}`
   - Operator: `\hat{p}`
   - Quaternion: `i, j, k` (simple)
   - Scalar: unchanged

3. **Test LaTeX output**
   - Verify correct notation for each type
   - Test mixed expressions

### Phase 3: Step-by-Step Enhancements (45 min)

1. **Review MatrixEquationSolver step generation**
   - Already has basic steps from Wave 10
   - Enhance with registry messages

2. **Add educational explanations**
   - Why order matters
   - Left vs right choice explanation
   - Mathematical properties

3. **Test step-by-step output**
   - Verify clarity of explanations
   - Test with all symbol types

### Phase 4: Testing (30 min)

1. **Create comprehensive test suite**
   - 25+ tests covering all features
   - Test message retrieval
   - Test LaTeX formatting
   - Test step-by-step explanations

---

## Success Criteria

1. ✅ Message registry has left/right division messages
2. ✅ Message registry has commutativity explanation messages
3. ✅ LaTeX formatter outputs `\mathbf{A}` for Matrix symbols
4. ✅ LaTeX formatter outputs `\hat{p}` for Operator symbols
5. ✅ LaTeX formatter handles Quaternion symbols
6. ✅ Step-by-step explanations include "multiply LEFT" or "multiply RIGHT"
7. ✅ Step-by-step explanations explain why order matters
8. ✅ 25+ tests covering all educational features
9. ✅ Build passes with 0 errors
10. ✅ Zero regressions (existing educational tests pass)

---

## Testing Strategy

### Test Categories (25+ tests):

**Message Registry Tests (8 tests)**:
1. Test left division message retrieval
2. Test right division message retrieval
3. Test noncommutative warning message
4. Test commutator explanation message
5. Test message formatting with parameters
6. Test message exists for all division types
7. Test backward compatibility (scalar messages unchanged)
8. Test message clarity (no ambiguous wording)

**LaTeX Formatter Tests (8 tests)**:
9. Test Matrix symbol formats as `\mathbf{A}`
10. Test Operator symbol formats as `\hat{p}`
11. Test Quaternion symbol formats correctly
12. Test Scalar symbol unchanged
13. Test mixed expression: `2*A*X = B`
14. Test operator equation: `H*psi = E*psi`
15. Test quaternion multiplication: `i*j = k`
16. Test complex nested expressions

**Step-by-Step Explanation Tests (9 tests)**:
17. Test left division explanation clarity
18. Test right division explanation clarity
19. Test explanation includes "LEFT" or "RIGHT"
20. Test explanation explains why order matters
21. Test explanation for Matrix equations
22. Test explanation for Operator equations
23. Test explanation for Quaternion equations
24. Test explanation for commutative (scalar) equations (unchanged)
25. Test full step-by-step output for `A*X = B`

---

## Verification Script

**File**: `.mathhook_sessions/verify_wave_11_educational.sh`

**Categories**:
1. File size compliance (all files ≤500 lines)
2. Emoji compliance (zero emojis)
3. Build status (passes with 0 errors)
4. Message registry has left/right messages
5. LaTeX formatter has type-aware formatting
6. Step-by-step uses registry messages
7. Test count (≥25 tests)
8. All educational tests pass
9. Documentation quality
10. Zero regressions (existing educational tests pass)

---

## Files to Modify/Create

### Modify:
1. **Message registry files** (location TBD - need to find)
   - Add left/right division messages
   - Add commutativity messages
   - ~50-100 lines added

2. **LaTeX formatter** (crates/mathhook-core/src/formatter/latex.rs or similar)
   - Add symbol type detection
   - Add type-specific formatting
   - ~50-100 lines modified

3. **Step-by-step educational** (if exists)
   - Integrate with message registry
   - Add explanations
   - ~30-50 lines modified

### Create:
4. **crates/mathhook-core/tests/educational_noncommutative_tests.rs** (NEW)
   - 25+ comprehensive tests
   - ~400-500 lines

---

## Mathematical Educational Content

### Key Concepts to Explain

**1. Why Order Matters**:
```
In matrix algebra, AB ≠ BA in general.
This means the solution to A*X = B is different from X*A = B.
```

**2. Left Division**:
```
To solve A*X = B:
- Multiply BOTH sides on the LEFT by A^(-1)
- A^(-1)*(A*X) = A^(-1)*B
- (A^(-1)*A)*X = A^(-1)*B
- I*X = A^(-1)*B
- X = A^(-1)*B
```

**3. Right Division**:
```
To solve X*A = B:
- Multiply BOTH sides on the RIGHT by A^(-1)
- (X*A)*A^(-1) = B*A^(-1)
- X*(A*A^(-1)) = B*A^(-1)
- X*I = B*A^(-1)
- X = B*A^(-1)
```

**4. Commutator**:
```
The commutator [A,B] = AB - BA measures how much A and B fail to commute.
If [A,B] = 0, then A and B commute (AB = BA).
If [A,B] ≠ 0, then order matters.
```

---

## CLAUDE.md Compliance Requirements

1. **File Size**: All files ≤500 lines
2. **No Emojis**: Zero emojis in code/comments/docs
3. **Tests**: 25+ comprehensive tests
4. **Build**: Must pass with 0 errors
5. **Regressions**: Zero (all existing educational tests pass)
6. **Documentation**: All new functions have `///` docs with examples
7. **Module Docs**: Use `//!` for module-level documentation
8. **Educational Quality**: Clear, pedagogical explanations
9. **Mathematical Correctness**: All explanations mathematically accurate
10. **Backward Compatibility**: Existing educational features still work

---

## Agent 11A Prompt Structure (When Ready)

**Agent 11A**: Educational & Message Registry Integration for Noncommutative Algebra

**Task**:
- Update message registry with left/right division messages
- Update LaTeX formatter with type-aware symbol formatting
- Enhance step-by-step explanations for noncommutative equations
- Create 25+ comprehensive tests
- Maintain zero regressions

**Deliverables**:
1. Updated message registry with noncommutative messages
2. Updated LaTeX formatter with `\mathbf{A}`, `\hat{p}` notation
3. Enhanced step-by-step explanations
4. New `tests/educational_noncommutative_tests.rs` with 25+ tests
5. All files ≤500 lines
6. Build passes, zero regressions
7. Documentation updated

**Target Quality**: 9.5+/10

---

## Timeline

**Phase 1**: Message Registry Updates (45 min)
**Phase 2**: LaTeX Formatter Updates (45 min)
**Phase 3**: Step-by-Step Enhancements (45 min)
**Phase 4**: Testing and Verification (30 min)

**Total**: 2.5-3 hours

---

**This Wave 11 will enable MathHook to provide excellent educational explanations for noncommutative algebra, distinguishing it from other CAS systems!**
