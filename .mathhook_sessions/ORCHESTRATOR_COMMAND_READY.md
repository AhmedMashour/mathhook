# Orchestrator Command Ready for Number Theory & Polynomial Completion

**Created**: 2025-10-19
**Purpose**: Bootstrap new orchestrator session to complete the 4 critical objectives
**Status**: READY TO USE

---

## What I've Created For You

### 📄 Main Orchestrator Command Document

**Location**: `.mathhook_sessions/NUMBER_THEORY_POLYNOMIAL_ORCHESTRATOR_COMMAND.md`

This document contains:

1. **Bootstrap Command** (copy-paste into new Claude Code session)
   - Forces orchestrator to read all required methodology files
   - Requires confirmation of understanding before starting
   - Includes all 5 mandatory orchestration rules
   - Emphasizes mathematical correctness (SymPy validation)

2. **Goal Statement** (provide after orchestrator confirms)
   - Complete 4-wave structure
   - 38 hours total estimated work
   - Clear success criteria and metrics
   - Based on Foundation → Implementation → Verification strategy

3. **Implementation Plan**:
   - **Wave 1**: Fix LCM Bug & Verify Number Theory (1-2 hours)
   - **Wave 2**: Polynomial Recurrence Evaluation (12-15 hours)
   - **Wave 3**: Symbolic Polynomial Expansion (6-8 hours)
   - **Wave 4**: Complete Polynomial GCD (18-20 hours)

---

## The 4 Objectives (From Analysis)

Based on deep code analysis in `NUMBER_THEORY_POLYNOMIAL_ANALYSIS.md`:

### 1. Fix LCM Bug (CRITICAL - 1 hour)
**Current State**: Returns `a*b` instead of `LCM(a,b)`
**Location**: `/crates/mathhook-core/src/algebra/gcd.rs` lines 40-53
**Impact**: `LCM(6,8)` returns 48 instead of 24
**Fix**: One line change to divide by GCD

### 2. Implement Polynomial Evaluation (CRITICAL - 15 hours)
**Current State**: 100% properties defined, 0% evaluation capability
**Gap**: Cannot compute `P_5(0.5)`, `H_3(2.0)`, `T_10(0.7)`, `L_4(1.5)`
**Solution**: Generic recurrence evaluator using existing property definitions
**Files**: `/crates/mathhook-core/src/functions/polynomials/*.rs`

### 3. Verify MOD/is_prime (2 hours)
**Current State**: Property definitions exist, actual implementations uncertain
**Task**: Search codebase, verify status, document findings
**Outcome**: Either working implementations found or marked as missing

### 4. Complete Polynomial GCD (20 hours)
**Current State**: Integer GCD works, polynomial GCD only handles simple cases
**Gap**: No polynomial long division, incomplete Euclidean algorithm
**Solution**: Full division algorithm + complete GCD for univariate polynomials

---

## Wave Structure (4 Waves, ~50 hours total)

### Wave 1: Fix LCM & Verify Number Theory (1-2 hours)
**Agent Focus**: Critical bug fix + status verification
**Deliverables**:
- Fixed LCM implementation
- MOD/is_prime status documented
- 15+ new tests with SymPy validation

### Wave 2: Polynomial Recurrence Evaluation (12-15 hours)
**Agent Focus**: Generic evaluation engine
**Deliverables**:
- `evaluate(n, x)` working for all 5 polynomials
- 25+ tests vs SymPy
- Performance <1ms for n≤100

### Wave 3: Symbolic Polynomial Expansion (6-8 hours)
**Agent Focus**: Generate explicit formulas
**Deliverables**:
- `expand_symbolic(n)` for all polynomials
- 15+ tests comparing symbolic vs numerical
- Examples: `P_3(x) = (5x³-3x)/2`

### Wave 4: Complete Polynomial GCD (18-20 hours)
**Agent Focus**: Full Euclidean algorithm
**Deliverables**:
- Polynomial division: `div()`, `quo()`, `rem()`
- Complete Euclidean GCD
- 20+ tests with SymPy validation
- Final quality audit

---

## Key Design Decisions Baked In

### 1. Mathematical Correctness First
Every wave requires:
- SymPy validation for all operations
- Edge case testing (zero, infinity, complex)
- Domain restriction documentation
- No approximations without documentation

### 2. Leverage Existing Architecture
- Wave 2 uses existing polynomial properties (recurrence already defined)
- Wave 3 integrates with Expression system
- Wave 4 integrates with factorization system
- No reinvention of wheels

### 3. Foundation Before Features
- Wave 1 fixes critical bug before building new capabilities
- Waves 2-3 depend on working foundation
- Wave 4 adds advanced capability on solid base

### 4. CLAUDE.md Strict Enforcement
- Max 500 lines/file
- No emojis anywhere
- Proper documentation style
- Build must pass with zero errors
- Content validation tests required

### 5. Proven Methodology
Based on Educational Waves 1-5 success:
- Sequential waves with verification between
- Verification scripts before agents launch
- Comprehensive reporting after completion
- 8.5+/10 quality target

---

## Success Metrics (Based on Educational Waves)

| Metric | Target | Based On |
|--------|--------|----------|
| **Quality Score** | 8.5+/10 avg | Educational Waves: 8.5/10 |
| **Tests Added** | 75+ new tests | Educational: Added 110 tests |
| **Content Validation** | 80%+ ratio | Educational Wave 5: 82% |
| **CLAUDE.md** | 100% compliant | Non-negotiable |
| **Regressions** | 0 | Zero tolerance |
| **SymPy Validation** | 100% ops | Mathematical correctness |
| **Build** | 0 errors | Must compile |

---

## How To Use This

### Step 1: Start New Claude Code Session
Open a fresh Claude Code session

### Step 2: Copy Bootstrap Command
Open: `.mathhook_sessions/NUMBER_THEORY_POLYNOMIAL_ORCHESTRATOR_COMMAND.md`
Copy: The entire gray block starting with "You are the Orchestrator..."

### Step 3: Paste and Wait
Paste into new session
Wait for orchestrator to:
- Read all 6 required files
- Confirm understanding
- Summarize the 5 rules
- List the 4 objectives
- Say "I am ready to orchestrate. Awaiting goal confirmation."

### Step 4: Confirm Goal
Copy the goal statement from the same document
Paste it to confirm
Orchestrator begins Wave 1 immediately

---

## What Orchestrator Will Do

### Automatically Execute:
1. Create `verify_wave_1_number_theory.sh` script (8-10 categories)
2. Launch Agent 1 with comprehensive prompt
3. Wait for Agent 1 completion
4. Run verification script
5. Create Wave 1 verification report
6. Move to Wave 2 (repeat process)
7. Continue through all 4 waves
8. Create final quality audit

### You Don't Need To:
- Manage agents (orchestrator handles it)
- Create verification scripts (orchestrator creates them)
- Run verifications (orchestrator runs them)
- Write reports (orchestrator writes them)
- Track progress (TodoWrite auto-updated)

### You Just:
- Start the session with bootstrap command
- Confirm the goal
- Monitor progress
- Receive final report

---

## Expected Timeline

With orchestration overhead:
- **Wave 1**: 1-2 hours (bug fix + verification)
- **Wave 2**: 14-17 hours (evaluation + verification)
- **Wave 3**: 7-10 hours (expansion + verification)
- **Wave 4**: 20-24 hours (GCD + verification + audit)

**Total**: ~42-53 hours (includes all verification/reporting)

Can run in single session (orchestrator maintains momentum) or split across multiple sessions (orchestrator picks up where left off).

---

## Files Referenced

### Analysis (Context)
1. `NUMBER_THEORY_POLYNOMIAL_ANALYSIS.md` - Detailed status
2. `ANALYSIS_SUMMARY.md` - Executive summary
3. `SYMPY_FEATURE_COMPARISON.md` - Gap analysis

### Methodology (Guidance)
4. `ORCHESTRATION_METHODOLOGY.md` - Proven patterns
5. `EDUCATIONAL_WAVE_5_VERIFICATION_REPORT.md` - Example
6. `EDUCATIONAL_QUALITY_AUDIT.md` - Quality standards

### Code (Targets)
7. `/crates/mathhook-core/src/algebra/gcd.rs` - LCM bug
8. `/crates/mathhook-core/src/functions/polynomials/*.rs` - Evaluation
9. `/crates/mathhook-core/src/functions/number_theory.rs` - Verification

### External (Validation)
10. `~/Documents/work/math/sympy/` - Primary reference
11. `~/Documents/work/math/symbolica/` - Secondary reference

---

## Why This Will Succeed

✅ **Based on proven methodology** (Educational Waves 1-5: 100% success)
✅ **Analysis-driven** (Deep understanding of what's broken)
✅ **Foundation-first** (Fix bugs before features)
✅ **Mathematical correctness** (SymPy validation built in)
✅ **Realistic estimates** (Based on actual code analysis)
✅ **Clear success criteria** (Measurable outcomes)
✅ **Leverages existing work** (Uses defined properties)
✅ **CLAUDE.md enforced** (Quality standards maintained)

---

## Success Definition

After Wave 4 completes, we should have:

```rust
// ✅ Number Theory - ALL WORKING
assert_eq!(Expression::integer(12).lcm(&Expression::integer(8)),
           Expression::integer(24));  // FIXED (was 96)

// ✅ Polynomial Evaluation - ALL WORKING
assert_eq!(legendre.evaluate(5, 0.5), 0.08984375);  // NEW
assert_eq!(hermite.evaluate(3, 2.0), 40.0);         // NEW

// ✅ Symbolic Expansion - ALL WORKING
assert_eq!(legendre.expand_symbolic(3).to_string(),
           "(5*x^3 - 3*x)/2");                      // NEW

// ✅ Polynomial GCD - ALL WORKING
assert_eq!(parse("x^2-1").gcd(&parse("x^2-2x+1")),
           parse("x-1"));                           // NEW
```

---

## The Command Is Ready

Everything is prepared. The orchestrator command in `NUMBER_THEORY_POLYNOMIAL_ORCHESTRATOR_COMMAND.md` is:

- ✅ Complete with all context
- ✅ Based on proven methodology
- ✅ Aligned with CLAUDE.md
- ✅ Ready to copy-paste
- ✅ Will run autonomously once started

**Next step**: Copy bootstrap command into new Claude Code session and begin.

---

**Document Created**: 2025-10-19
**Status**: READY TO USE
**Confidence**: HIGH (based on Educational Waves 1-5 success pattern)
