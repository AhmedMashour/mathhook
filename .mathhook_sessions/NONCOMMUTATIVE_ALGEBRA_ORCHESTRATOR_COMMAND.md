# Noncommutative Algebra Support - Orchestrator Bootstrap Command

**Purpose**: Add comprehensive noncommutative algebra support (matrices, quantum operators, quaternions) to MathHook
**Date Created**: 2025-10-19
**Last Updated**: 2025-10-19 (Expanded to 12 waves for full module coverage)
**Based On**: SymPy's proven `commutative=False` approach, extensive architecture discussion

**Scope**: This is a **comprehensive 12-wave implementation** covering:
- **Foundation** (Waves 1-3): Core types, constructors, simplification
- **Integration** (Waves 4-7): Calculus, algebra, patterns, matrix operations
- **User-Facing** (Waves 8-9): Parser, macros
- **Advanced** (Waves 10-11): Solvers, educational, formatters
- **Completion** (Wave 12): Examples, documentation, final verification

**Why 12 Waves**: Noncommutative algebra isn't just a feature - it affects **every mathematical operation** in MathHook. This plan ensures correctness across all modules (37 files modified, including message registry and all formatters).

**Estimated Effort**: 158-197 hours of agent work (~185-230 hours with orchestration)
**Timeline**: 4-6 weeks of focused work

---

## Copy-Paste This Entire Block Into New Claude Code Session

```
You are the Orchestrator for the Noncommutative Algebra Implementation project.

CRITICAL FIRST STEP - Read these files in order and line by line to learn the proven methodology:

1. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md
   - This is the SINGLE SOURCE OF TRUTH for all development rules
   - Contains architectural constraints, coding standards, and non-negotiables
   - CLAUDE.md ALWAYS overrides any other documentation
   - Pay special attention to: Expression size constraints (32-byte target), mathematical correctness, canonical forms

2. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md
   - Complete orchestration methodology from Educational Waves 1-5
   - Contains wave templates, agent prompts, verification patterns
   - Shows exactly how to structure work, launch agents, verify results

3. Read: /Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/UPDATED_ANALYSIS_POST_COMPLETION.md
   - Section on "Noncommutative Algebra Support" (line 499-732)
   - Contains architectural approaches and design decisions
   - Shows why this is critical for mathematical correctness

MANDATORY ORCHESTRATION RULES (From Proven Methodology):

1. You Are Always The Orchestrator
   - You plan, launch agents, verify, and make decisions
   - Agents execute specific tasks; you maintain control and continuity
   - NEVER delegate orchestration responsibilities to agents

2. Sequential Waves, Parallel Agents
   - Work proceeds in waves: Wave 1 → verify → Wave 2 → verify → ...
   - Within a wave, launch multiple agents in parallel when possible
   - NEVER skip verification between waves

3. Mandatory Verification Protocol
   - Create verification script BEFORE launching agents (bash script with 8-10 categories)
   - Run verification script AFTER agents complete
   - Create comprehensive verification report
   - NEVER declare work complete without running verification

4. Strict CLAUDE.md Enforcement
   - All agent prompts MUST include CLAUDE.md requirements explicitly
   - Enforce: max 500 lines/file, no emojis, proper docs, no placeholders, build passes
   - Zero tolerance for violations
   - CLAUDE.md overrides ALL other guidance

5. Maintain Momentum
   - Don't stop between waves unless verification fails
   - Use TodoWrite to track progress through all waves
   - Keep user informed of progress without asking unnecessary questions

MATHEMATICAL CORRECTNESS - HIGHEST PRIORITY:

From CLAUDE.md: "Mathematical Correctness First: Every mathematical operation must be correct in ALL cases. No exceptions."

**Critical Problem We're Solving**:
Current MathHook assumes ALL multiplication is commutative (A*B = B*A).
This is MATHEMATICALLY WRONG for:
- Matrices: A*B ≠ B*A (order matters!)
- Quantum operators: [x,p] = xp - px ≠ 0
- Quaternions: ij = k, ji = -k (different!)

**SymPy's Proven Approach** (studied from ~/Documents/work/math/sympy/):
1. **Default to commutative=True** (95% of cases)
2. **Explicit opt-in** for noncommutative: `symbols("A B", commutative=False)`
3. **Parser always creates commutative symbols** (user must declare noncommutative explicitly)
4. **Separates commutative and noncommutative factors** in multiplication
5. **Sorts only commutative factors**, preserves order for noncommutative

CONFIRMATION REQUIRED:

After reading all files above line by line, respond with:

1. "I have read and understood the orchestration methodology"
2. "I understand SymPy's commutative=False approach and why we're following it"
3. Summarize the 5 mandatory orchestration rules in your own words
4. List the 5 phases of a standard wave
5. State: "The default is commutative=True, noncommutative requires explicit declaration"
6. Say: "I am ready to orchestrate. Awaiting goal confirmation."

Then WAIT for the user to provide the goal confirmation and any modifications.

DO NOT proceed with any work until you have:
- Read all required files line by line
- Confirmed understanding
- Received goal confirmation from the user
```

---

## Goal Statement (Provide After Orchestrator Confirms)

```
The goal is: Implement Noncommutative Algebra Support Following SymPy's Proven Design

Context: MathHook currently assumes all multiplication is commutative. This is mathematically incorrect for matrices, quantum operators, and quaternions. We need to support noncommutative algebra while maintaining backward compatibility.

Design Philosophy (From SymPy):
- Default: commutative=True (like SymPy)
- Explicit opt-in for noncommutative (user must declare it)
- Parser creates commutative symbols by default
- Zero user complexity for 95% of cases (scalars)
- LaTeX notation infers types (\mathbf{A} → matrix → noncommutative)

Structure - 12 Waves for Comprehensive Integration Across All Modules:

**Foundation (Waves 1-3)**: Core infrastructure and simplification
**Integration (Waves 4-7)**: Calculus, algebra, patterns, matrix operations
**User-Facing (Waves 8-9)**: Parser and macros
**Advanced (Waves 10-11)**: Solvers, educational, formatter
**Completion (Wave 12)**: Examples, documentation, final verification

Wave 1: Core Type System & Symbol Enhancement (8-10 hours)
- Scope: Add commutativity tracking to Symbol and Expression types
- Priority: HIGHEST (foundation for all other waves)
- Objectives:
  1. Add SymbolType enum (Scalar, Matrix, Operator, Quaternion) to Symbol
  2. Update Expression::Mul from Mul(Box<Vec<Expression>>) to Mul(Box<Vec<Expression>>, Commutativity)
  3. Create Commutativity enum (Commutative, Noncommutative, Unknown)
  4. Update Symbol with symbol_type field (defaults to Scalar)
  5. Add Symbol::scalar(), Symbol::matrix(), Symbol::operator(), Symbol::quaternion() constructors
  6. Implement Symbol::commutativity() method (Scalar→Commutative, Matrix/Operator/Quaternion→Noncommutative)
  7. Verify Expression size stays ≤ 48 bytes (acceptable per document)
  8. NO user-facing API changes yet (internal only)
- Deliverables:
  - Updated Symbol type in /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/symbol.rs
  - Updated Expression enum in /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/data_types.rs
  - Commutativity module in /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/commutativity.rs
  - All existing tests pass (zero regressions)
  - Expression size verified ≤ 48 bytes
- CRITICAL: This is a breaking change to Expression::Mul signature - all pattern matches must be updated

Wave 2: Constructor & Accessor Updates (10-12 hours)
- Scope: Update Expression constructors and add commutativity propagation
- Priority: HIGH (enables the system to work)
- Objectives:
  1. Update Expression::mul() to ALWAYS infer commutativity from factors (no explicit control)
  2. Implement inference rules:
     - All factors Commutative → Mul is Commutative
     - Any factor Noncommutative → Mul is Noncommutative
     - Any factor Unknown → Mul is Unknown (conservative)
  3. Update Expression::commutativity() method for all variants
  4. Add Expression::is_commutative() convenience method
  5. Update ALL pattern matches on Mul throughout codebase (~100-150 sites)
  6. Add comprehensive tests for commutativity inference
  7. Document inference rules in code comments
- Deliverables:
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/constructors/basic.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/methods.rs
  - All Mul pattern matches updated across codebase
  - 30+ tests for commutativity propagation
  - Build passes with zero errors
- CRITICAL: Search for ALL occurrences of "Expression::Mul(" pattern matching and update

Wave 3: Simplification Engine Updates (15-18 hours)
- Scope: Update simplification to respect commutativity
- Priority: HIGH (correctness of results depends on this)
- Objectives:
  1. Update simplify_multiplication() to take commutativity parameter
  2. ONLY sort factors if commutativity.can_sort() returns true
  3. Update like-term combining: (A*B) + (B*A) stays separate if noncommutative
  4. Update power simplification: (A*B)^2 ≠ A^2*B^2 if noncommutative
  5. Update pattern matching to check commutativity before reordering
  6. Ensure Matrix expressions are always noncommutative
  7. Validate against SymPy behavior
- Deliverables:
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/simplify/arithmetic/multiplication.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/simplify/arithmetic/addition.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/simplify/arithmetic/power.rs
  - 50+ tests comparing commutative vs noncommutative simplification
  - SymPy validation tests
- CRITICAL: Test that A*B + B*A does NOT simplify to 2*A*B for noncommutative A, B

Wave 4: Calculus Integration (18-22 hours)
- Scope: Update calculus operations to respect noncommutativity
- Priority: HIGH (critical for correctness in derivatives, integrals)
- Objectives:
  1. Update derivative product rule: d(AB)/dx = (dA/dx)B + A(dB/dx) - ORDER MATTERS
  2. Update chain rule for noncommutative compositions
  3. Update integral operations to preserve order
  4. Update limits module for noncommutative expressions
  5. Update series expansion: (A+B)^n expansion preserves order
  6. Update summation module for noncommutative terms
  7. Validate against SymPy calculus behavior
- Deliverables:
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/calculus/derivatives/rules.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/calculus/integrals/indefinite.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/calculus/limits.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/calculus/series.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/calculus/summation.rs
  - 40+ tests for noncommutative calculus
  - SymPy validation for product rule, chain rule
- CRITICAL: d(AB)/dx ≠ dA/dx * dB/dx for matrices - must use product rule preserving order

Wave 5: Algebra Operations Integration (20-25 hours)
- Scope: Update algebra module to respect noncommutativity
- Priority: HIGH (affects expand, factor, collect operations)
- Objectives:
  1. Update expand.rs: (A+B)^2 = A^2 + AB + BA + B^2 (NOT A^2 + 2AB + B^2)
  2. Update factor.rs: Factoring preserves order (left vs right factoring)
  3. Update collect.rs: AB and BA are DIFFERENT terms (don't combine)
  4. Update polynomial_division.rs: Division with noncommutative coefficients
  5. Update rational.rs: Simplification of rational expressions with matrices
  6. Update advanced_simplify.rs: Respect commutativity in all rules
  7. Validate against SymPy algebra behavior
- Deliverables:
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/expand.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/factor.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/collect.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/polynomial_division.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/rational.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/advanced_simplify.rs
  - 50+ tests comparing commutative vs noncommutative algebra
  - SymPy validation tests
- CRITICAL: (A+B)^2 must expand correctly for matrices - cannot assume commutativity

Wave 6: Pattern Matching & Substitution (12-15 hours)
- Scope: Update pattern matching to respect order in noncommutative expressions
- Priority: MEDIUM-HIGH (affects pattern-based simplifications)
- Objectives:
  1. Update pattern matching: AB should NOT match BA for noncommutative
  2. Update substitution: A→C in ABA must preserve positions (CBA, not ACB or BAC)
  3. Add ordered vs unordered pattern matching modes
  4. Update wildcard matching to respect commutativity
  5. Ensure pattern-based rewrite rules check commutativity first
  6. Add tests for matrix pattern matching
- Deliverables:
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/pattern/matching/mod.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/pattern/substitution/mod.rs
  - 30+ tests for ordered pattern matching
  - Documentation on pattern matching modes
- CRITICAL: Substitution must preserve positional information for noncommutative symbols

Wave 7: Matrix Operations Enhancement (15-18 hours)
- Scope: Enhance matrix module for full noncommutative support
- Priority: MEDIUM-HIGH (matrices are the primary noncommutative type)
- Objectives:
  1. Update transpose: (AB)^T = B^T A^T - ORDER REVERSES
  2. Update determinant calculation for symbolic matrices
  3. Update inverse: (AB)^(-1) = B^(-1) A^(-1) - ORDER REVERSES
  4. Update eigenvalue computation for symbolic matrices
  5. Update decomposition methods (LU, QR, SVD) for symbolic
  6. Ensure all matrix operations respect noncommutativity
  7. Add comprehensive matrix algebra tests
- Deliverables:
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/matrix/operations.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/matrix/decomposition/mod.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/matrix/eigenvalues/mod.rs
  - 40+ tests for symbolic matrix operations
  - SymPy validation for transpose, inverse rules
- CRITICAL: Transpose and inverse must reverse order of products

Wave 8: Parser Integration (LaTeX) (12-15 hours)
- Scope: Add LaTeX notation support for type inference
- Priority: MEDIUM-HIGH (user-facing convenience)
- Objectives:
  1. Add \\mathbf token to LALRPOP grammar (already exists at line 116)
  2. Add \\mathbf{A} → Symbol::matrix(A) rule (3 lines)
  3. Update \\hat{p} → Symbol::operator(p) rule (existing, modify)
  4. Add support for quaternions via explicit \\mathbb{H} or similar notation
  5. Test parser with all four types: scalar (x), matrix (\\mathbf{A}), operator (\\hat{p}), quaternion
  6. Ensure lowercase letters stay scalar (commutative)
  7. Ensure explicit notation always wins
  8. Add examples to documentation
- Deliverables:
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/parser/grammar.lalrpop
  - Parser regenerated (lalrpop command)
  - 20+ parser tests with matrix/operator notation
  - Examples showing automatic type inference
- CRITICAL: Parser must handle both \mathbf{A}\mathbf{B} and plain AB correctly

Wave 9: symbol! and symbols! Macro Enhancement (6-8 hours)
- Scope: User-friendly symbol creation for all four types (Scalar, Matrix, Operator, Quaternion)
- Priority: MEDIUM (ergonomics)
- Objectives:
  1. UPDATE singular symbol!() macro to support optional type parameters:
     - symbol!(x) → Scalar (default, backward compatible)
     - symbol!(A; matrix) → Matrix symbol
     - symbol!(p; operator) → Operator symbol
     - symbol!(q; quaternion) → Quaternion symbol
  2. CREATE plural symbols!() macro for bulk creation (matching SymPy's symbols() API):
     - symbols!("x y z") → All scalars (default)
     - symbols!("A B C"; matrix) → All matrices (noncommutative)
     - symbols!("p x h"; operator) → All operators (noncommutative)
     - symbols!("i j k"; quaternion) → All quaternions (noncommutative)
  3. Add commutator() and anticommutator() functions
  4. Add comprehensive documentation and examples for all four types
  5. Ensure backward compatibility (existing symbol!(x) calls unchanged)
- Deliverables:
  - Updated symbol!() macro in /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook/src/macros.rs
  - New symbols!() macro in /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook/src/macros.rs
  - Commutator/anticommutator in /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/constructors/specialized.rs
  - 25+ tests covering all four types (singular and plural macros)
  - Doctests showing usage for all four types
  - Updated CLAUDE.md with comprehensive examples
- CRITICAL: Default is Scalar (commutative). Matrix, Operator, Quaternion are explicitly noncommutative.

Wave 10: Equation Solvers Integration (18-22 hours)
- Scope: Update equation solvers to handle noncommutative algebra
- Priority: HIGH (matrix equations are fundamentally noncommutative)
- Objectives:
  1. Distinguish AX = B from XA = B (different solutions!)
  2. Update linear system solver for matrix coefficients
  3. Add left vs right division support
  4. Update equation analyzer to detect commutativity
  5. Ensure symbolic solutions preserve order
  6. Add matrix equation solving examples
  7. Validate against SymPy solvers
- Deliverables:
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/solvers.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/solvers/linear.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/equation_analyzer.rs
  - 35+ tests for matrix equations
  - SymPy validation for matrix equation solving
- CRITICAL: AX = B and XA = B have different solutions - must distinguish left vs right multiplication

Wave 11: Educational, Message Registry & Formatter Integration (12-15 hours)
- Scope: Update educational explanations, message registry, and output formatting
- Priority: MEDIUM-HIGH (user education and output quality)
- Objectives:
  1. Update message registry for algebra/calculus/core/solvers to explain noncommutative operations
  2. Add messages for why AB ≠ BA for matrices
  3. Update step-by-step explanations for noncommutative operations
  4. Update LaTeX formatter: display matrices as \mathbf{A}, operators as \hat{p}, quaternions appropriately
  5. Update simple.rs formatter: clear distinction between types
  6. Update wolfram.rs formatter: proper Wolfram notation for matrices
  7. Add pedagogical examples for matrix algebra
  8. Document common pitfalls (assuming commutativity)
- Deliverables:
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/educational/message_registry/algebra.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/educational/message_registry/calculus.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/educational/message_registry/core.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/educational/message_registry/solvers.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/educational/step_by_step.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/formatter/latex/expressions.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/formatter/latex/functions.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/formatter/simple.rs
  - Updated /Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/formatter/wolfram.rs
  - 25+ tests for formatting (LaTeX, simple, Wolfram)
  - Educational examples and explanations
- CRITICAL: All formatters must clearly distinguish matrix/operator/quaternion from scalar symbols

Wave 12: Examples, Documentation & Final Verification (12-15 hours)
- Scope: Complete integration testing and documentation for all four symbol types
- Priority: HIGH (production readiness)
- Objectives:
  1. Create quantum mechanics example (Operator type: commutator [x,p] = iℏ)
  2. Create matrix algebra example (Matrix type: A*B ≠ B*A)
  3. Create quaternion example (Quaternion type: ij = k, ji = -k)
  4. Create scalar example (Scalar type: x*y = y*x, default behavior)
  5. Update CLAUDE.md with comprehensive noncommutative section
  6. Create migration guide for Expression::Mul changes
  7. Add 50+ integration tests covering all four types
  8. Performance benchmarks (commutative path should have zero overhead)
  9. Final quality audit
- Deliverables:
  - /Users/ahmedmashhour/Documents/work/math/mathhook/examples/quantum_operators.rs (Operator type)
  - /Users/ahmedmashhour/Documents/work/math/mathhook/examples/matrix_algebra.rs (Matrix type)
  - /Users/ahmedmashhour/Documents/work/math/mathhook/examples/quaternions.rs (Quaternion type)
  - /Users/ahmedmashhour/Documents/work/math/mathhook/examples/scalar_algebra.rs (Scalar type baseline)
  - Updated CLAUDE.md sections with all four types
  - Migration guide document
  - 60+ integration tests (15 per type)
  - Performance benchmarks showing zero regression
  - Quality audit report (target 9.0+/10)
- CRITICAL: Verify that scalar algebra performance is unchanged (commutative path must be just as fast)

Target Metrics:
- Quality Score: 9.0+/10 (this is core infrastructure)
- Test Count: Add 425+ tests (30 + 50 + 50 + 40 + 50 + 30 + 40 + 20 + 25 + 35 + 25 + 60)
- SymPy Validation: 100% behavior match for noncommutative operations
- Build: Zero errors, zero regressions
- CLAUDE.md: 100% compliance
- Expression Size: ≤ 48 bytes (documented increase from 32)
- Performance: Commutative path zero overhead
- Breaking Changes: Documented and mitigated
- Module Coverage: 100% of math operations respect commutativity

Success Criteria (Comprehensive - All 12 Waves):

**Foundation:**
1. ✅ Symbol has SymbolType enum with all four types (Scalar, Matrix, Operator, Quaternion)
2. ✅ Expression::Mul tracks Commutativity (inferred from factors, no explicit control)
3. ✅ Simplification respects commutativity (no sorting if noncommutative)

**Calculus:**
4. ✅ Product rule: d(AB)/dx = (dA/dx)B + A(dB/dx) preserves order
5. ✅ Series expansion: (A+B)^n preserves order in all terms

**Algebra:**
6. ✅ Expansion: (A+B)^2 = A^2 + AB + BA + B^2 (correct for matrices)
7. ✅ Collection: AB and BA treated as different terms
8. ✅ Factoring preserves order (left vs right factoring)

**Patterns & Matrix:**
9. ✅ Pattern matching: AB does NOT match BA for noncommutative
10. ✅ Substitution preserves positions
11. ✅ Transpose: (AB)^T = B^T A^T (order reverses)
12. ✅ Inverse: (AB)^(-1) = B^(-1) A^(-1) (order reverses)

**User-Facing:**
13. ✅ Parser infers types from LaTeX notation (\mathbf{A} → Matrix, \hat{p} → Operator)
14. ✅ Both symbol!() and symbols!() macros support all four types

**Solvers & Output:**
15. ✅ Solvers distinguish AX = B from XA = B (different solutions)
16. ✅ Message registry has noncommutative algebra messages (algebra, calculus, core, solvers)
17. ✅ LaTeX formatter displays matrices as \mathbf{A}, operators as \hat{p}
18. ✅ Simple formatter clearly distinguishes types
19. ✅ Wolfram formatter uses proper matrix notation
20. ✅ Educational step-by-step explanations for why AB ≠ BA

**Examples:**
21. ✅ Quantum operator example: [x,p] ≠ 0 (Operator type)
22. ✅ Matrix example: A*B + B*A NOT simplified (Matrix type)
23. ✅ Quaternion example: ij ≠ ji (Quaternion type)
24. ✅ Scalar example: x*y = y*x simplified correctly (Scalar type, default)

**Quality:**
25. ✅ All existing tests pass (zero regressions)
26. ✅ 425+ new tests all passing
27. ✅ Performance benchmarks show zero overhead for commutative
28. ✅ SymPy validation: 100% behavior match

Mathematical Correctness Emphasis:
- Every operation validated against SymPy behavior
- Matrix multiplication order preservation tested
- Commutator identities verified: [A,B] = -[B,A]
- No silent failures (explicit is better than implicit)
- Default to safe (commutative) unless explicitly noncommutative

Standard orchestration protocol:
- You are orchestrator, maintain momentum
- Create verification scripts per wave (verify_wave_N.sh)
- Launch agents with strict CLAUDE.md enforcement
- Verify everything before declaring complete
- Create comprehensive verification reports
- Track with TodoWrite
- Compare against SymPy for mathematical validation

Let's begin with Wave 1: Core Type System & Symbol Enhancement
```

---

## What This Command Achieves (12-Wave Comprehensive Implementation)

### Wave 1: Core Type System (8-10 hours)
**Critical**: Foundation for all noncommutative algebra
**Changes**: Symbol gets SymbolType, Expression::Mul gets Commutativity
**Output**: Internal type system ready

### Wave 2: Constructor Updates (10-12 hours)
**Critical**: Makes the system functional
**Changes**: Update ~100-150 Mul pattern matches, add automatic commutativity inference
**Output**: System can track commutativity automatically (no explicit control)

### Wave 3: Simplification Engine (15-18 hours)
**Critical**: Correctness of results
**Changes**: Only sort commutative factors, preserve order for noncommutative
**Output**: A*B + B*A stays unsimplified for matrices

### Wave 4: Calculus Integration (18-22 hours)
**Critical**: Derivatives and integrals for matrices
**Changes**: Product rule, chain rule, series expansion respect order
**Output**: d(AB)/dx = (dA/dx)B + A(dB/dx) ✅

### Wave 5: Algebra Operations (20-25 hours)
**Critical**: Expand, factor, collect for matrices
**Changes**: (A+B)^2 expands correctly, AB and BA are different terms
**Output**: All algebra operations preserve order

### Wave 6: Pattern Matching (12-15 hours)
**Important**: Pattern-based simplifications
**Changes**: AB doesn't match BA, substitution preserves positions
**Output**: Correct pattern matching for noncommutative

### Wave 7: Matrix Operations (15-18 hours)
**Important**: Symbolic matrix algebra
**Changes**: Transpose, inverse reverse order: (AB)^T = B^T A^T
**Output**: Full symbolic matrix support

### Wave 8: Parser Integration (12-15 hours)
**User-Facing**: LaTeX notation → automatic type inference
**Changes**: \mathbf{A} → Matrix, \hat{p} → Operator
**Output**: Users write standard notation, system infers types

### Wave 9: symbol! and symbols! Macros (6-8 hours)
**User-Facing**: Ergonomic symbol creation
**Changes**: Both macros support all four types
**Output**: symbol!(A; matrix), symbols!("i j k"; quaternion)

### Wave 10: Equation Solvers (18-22 hours)
**Critical**: Matrix equation solving
**Changes**: Distinguish AX = B from XA = B
**Output**: Correct solutions for matrix equations

### Wave 11: Message Registry, Educational & Formatters (12-15 hours)
**Important**: User education, messages, and output quality
**Changes**: Message registry updates (4 files), step-by-step, 3 formatters (LaTeX, simple, Wolfram)
**Output**: Clear messages, educational value, proper formatting across all output formats

### Wave 12: Examples & Final Verification (12-15 hours)
**Production-Ready**: Complete integration
**Changes**: Examples for all types, comprehensive testing
**Output**: Production-ready noncommutative algebra across ALL modules

---

## Key Design Decisions Baked Into This Plan

### 1. Follow SymPy's Proven Approach
- Default: commutative=True (safe, matches user expectations)
- Explicit opt-in for noncommutative
- Parser always creates commutative symbols
- Zero complexity for 95% of users (scalars)

### 2. Minimal Breaking Changes
- Expression::Mul signature changes (unavoidable)
- All other types stay backward compatible
- Migration path documented
- Existing tests must pass

### 3. LaTeX Notation as Type Hint
- \mathbf{A} → matrix (standard mathematical notation)
- \hat{p} → operator (quantum mechanics convention)
- Plain x, y, z → scalar (default)
- Matches what users already know

### 4. Performance Safety
- Commutative path must have zero overhead
- Noncommutative path adds minimal cost (skip sort step)
- Benchmarks required to prove no regression

### 5. Mathematical Correctness
- SymPy behavior is reference
- All operations validated
- Edge cases tested thoroughly
- No silent failures

### 6. Comprehensive Type System

| Type | Commutativity | Use Case | Creation Syntax | Parser Notation | Example |
|------|---------------|----------|-----------------|-----------------|---------|
| **Scalar** | Commutative | Default algebra, variables | `symbol!(x)` <br> `symbols!("x y z")` | `x`, `y`, `z` (plain) | x*y = y*x ✅ |
| **Matrix** | Noncommutative | Linear algebra, matrices | `symbol!(A; matrix)` <br> `symbols!("A B C"; matrix)` | `\mathbf{A}` (LaTeX bold) | A*B ≠ B*A ✅ |
| **Operator** | Noncommutative | Quantum mechanics, operators | `symbol!(p; operator)` <br> `symbols!("p x h"; operator)` | `\hat{p}` (LaTeX hat) | [x,p] ≠ 0 ✅ |
| **Quaternion** | Noncommutative | 3D rotations, quaternions | `symbol!(i; quaternion)` <br> `symbols!("i j k"; quaternion)` | Explicit only (no standard LaTeX) | ij = k, ji = -k ✅ |

**Key Properties:**
- **Default**: Scalar (95% of use cases, matches SymPy)
- **Explicit**: Matrix, Operator, Quaternion require opt-in
- **Parser**: Infers type from LaTeX notation where possible
- **Safety**: No silent failures - noncommutative must be explicit
- **SymPy Alignment**: Matches `symbols("x y", commutative=False)` API

---

## Files The Orchestrator Will Reference (Comprehensive)

**Core Foundation (Waves 1-3):**
1. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/symbol.rs` - Add SymbolType
2. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/data_types.rs` - Update Mul variant
3. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/commutativity.rs` - NEW file
4. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/core/expression/constructors/basic.rs` - Update mul()
5. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/simplify/arithmetic/multiplication.rs` - Conditional sorting
6. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/simplify/arithmetic/addition.rs` - Collect like terms
7. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/simplify/arithmetic/power.rs` - Power rules

**Calculus (Wave 4):**
8. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/calculus/derivatives/rules.rs` - Product rule, chain rule
9. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/calculus/integrals/indefinite.rs` - Integration
10. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/calculus/limits.rs` - Limits
11. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/calculus/series.rs` - Series expansion
12. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/calculus/summation.rs` - Summation

**Algebra (Wave 5):**
13. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/expand.rs` - Expansion
14. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/factor.rs` - Factoring
15. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/collect.rs` - Collection
16. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/polynomial_division.rs` - Division
17. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/rational.rs` - Rational expressions
18. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/advanced_simplify.rs` - Advanced simplification

**Pattern & Matrix (Waves 6-7):**
19. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/pattern/matching/mod.rs` - Pattern matching
20. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/pattern/substitution/mod.rs` - Substitution
21. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/matrix/operations.rs` - Matrix operations
22. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/matrix/decomposition/mod.rs` - Decomposition
23. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/matrix/eigenvalues/mod.rs` - Eigenvalues

**User-Facing (Waves 8-9):**
24. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/parser/grammar.lalrpop` - Parser
25. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook/src/macros.rs` - Macros

**Solvers (Wave 10):**
26. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/solvers.rs` - Solvers
27. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/solvers/linear.rs` - Linear solvers
28. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/algebra/equation_analyzer.rs` - Equation analyzer

**Educational & Formatters (Wave 11):**
29. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/educational/message_registry/algebra.rs` - Algebra messages
30. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/educational/message_registry/calculus.rs` - Calculus messages
31. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/educational/message_registry/core.rs` - Core messages
32. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/educational/message_registry/solvers.rs` - Solver messages
33. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/educational/step_by_step.rs` - Step-by-step
34. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/formatter/latex/expressions.rs` - LaTeX expressions
35. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/formatter/latex/functions.rs` - LaTeX functions
36. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/formatter/simple.rs` - Simple formatter
37. `/Users/ahmedmashhour/Documents/work/math/mathhook/crates/mathhook-core/src/formatter/wolfram.rs` - Wolfram formatter

**Reference Documentation:**
38. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/UPDATED_ANALYSIS_POST_COMPLETION.md` - Design rationale
39. `/Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md` - All rules

**SymPy Reference:**
40. `~/Documents/work/math/sympy/sympy/core/symbol.py` - Symbol with commutative flag
41. `~/Documents/work/math/sympy/sympy/core/mul.py` - How Mul separates commutative/noncommutative
42. `~/Documents/work/math/sympy/sympy/core/tests/test_noncommutative.py` - Expected behavior

**Total: 37 files to modify + 5 reference files = 42 files**

---

## Expected Timeline (12 Waves - Comprehensive)

**Foundation (Waves 1-3)**: 33-40 hours
- **Wave 1**: 8-10 hours (core types)
- **Wave 2**: 10-12 hours (constructors + 100-150 pattern match updates)
- **Wave 3**: 15-18 hours (simplification engine)

**Integration (Waves 4-7)**: 65-80 hours
- **Wave 4**: 18-22 hours (calculus integration)
- **Wave 5**: 20-25 hours (algebra operations)
- **Wave 6**: 12-15 hours (pattern matching)
- **Wave 7**: 15-18 hours (matrix operations)

**User-Facing (Waves 8-9)**: 18-23 hours
- **Wave 8**: 12-15 hours (parser integration)
- **Wave 9**: 6-8 hours (macros)

**Advanced (Waves 10-11)**: 30-37 hours
- **Wave 10**: 18-22 hours (equation solvers)
- **Wave 11**: 12-15 hours (message registry, educational, 3 formatters)

**Completion (Wave 12)**: 12-15 hours
- **Wave 12**: 12-15 hours (examples + docs + final verification)

**Total Agent Work**: 158-197 hours
**With Orchestration Overhead**: ~185-230 hours total

**Parallelization Opportunities**:
- **Wave 2**: Multiple agents for pattern matching (tedious but parallelizable)
- **Waves 4-7**: Can partially parallelize (calculus independent of algebra)
- **Waves 8-9**: Can run in parallel after Wave 3
- **Waves 10-11**: Can run in parallel after Wave 7

**Realistic Timeline**: 4-6 weeks of focused work

---

## Quality Targets

| Metric | Target | Justification |
|--------|--------|---------------|
| Quality Score | 9.0+/10 | Core infrastructure (higher standard) |
| Tests Added | 165+ | Comprehensive coverage |
| SymPy Match | 100% | Reference implementation |
| CLAUDE.md Compliance | 100% | Non-negotiable |
| Regressions | 0 | Zero tolerance |
| Performance Overhead | 0% | Commutative path unchanged |

---

## Success Definition

At the end of Wave 6, we should be able to:

```rust
use mathhook_core::prelude::*;

// ============================================================================
// TYPE 1: Scalar (Default - Commutative)
// ============================================================================
let x = symbol!(x);  // Scalar type (default)
let y = symbol!(y);
let expr = expr!(x*y + y*x);
assert_eq!(expr.simplify(), expr!(2*x*y));  // ✅ Simplified (x*y = y*x)

// Bulk scalar creation
let (a, b, c) = symbols!("a b c");  // All scalars (commutative)

// ============================================================================
// TYPE 2: Matrix (Noncommutative)
// ============================================================================
// Explicit creation
let A = Symbol::matrix("A");
let B = Symbol::matrix("B");
let expr = Expression::mul(vec![A.into(), B.into()])
         + Expression::mul(vec![B.into(), A.into()]);
assert_ne!(expr.simplify(), expr!(2*A*B));  // ✅ Does NOT simplify (A*B ≠ B*A)

// Macro creation
let M = symbol!(M; matrix);  // Single matrix
let (X, Y, Z) = symbols!("X Y Z"; matrix);  // Bulk matrices

// Parser: LaTeX notation → automatic Matrix type
let parser = Parser::new();
let expr = parser.parse(r"\mathbf{A}\mathbf{B} + \mathbf{B}\mathbf{A}")?;
// ✅ Parser creates Matrix symbols, does NOT simplify

// ============================================================================
// TYPE 3: Operator (Noncommutative - Quantum Mechanics)
// ============================================================================
// Explicit creation
let x = Symbol::operator("x");
let p = Symbol::operator("p");
let commutator = Expression::commutator(x.into(), p.into());
assert_ne!(commutator.simplify(), Expression::integer(0));  // ✅ [x,p] ≠ 0

// Macro creation
let x_op = symbol!(x; operator);  // Single operator
let (p, h, L) = symbols!("p h L"; operator);  // Bulk operators

// Parser: LaTeX notation → automatic Operator type
let expr = parser.parse(r"\hat{p}\hat{x} - \hat{x}\hat{p}")?;
// ✅ Parser creates Operator symbols, preserves order

// ============================================================================
// TYPE 4: Quaternion (Noncommutative)
// ============================================================================
// Explicit creation
let i = Symbol::quaternion("i");
let j = Symbol::quaternion("j");
let k = Symbol::quaternion("k");

let ij = Expression::mul(vec![i.into(), j.into()]);
let ji = Expression::mul(vec![j.into(), i.into()]);
assert_ne!(ij, ji);  // ✅ i*j ≠ j*i
// i*j = k, j*i = -k (quaternion multiplication rules)

// Macro creation
let q = symbol!(q; quaternion);  // Single quaternion
let (i, j, k) = symbols!("i j k"; quaternion);  // Bulk quaternions

// ============================================================================
// Mixed Operations - Type Safety
// ============================================================================
let x_scalar = symbol!(x);  // Scalar (commutative)
let A_matrix = symbol!(A; matrix);  // Matrix (noncommutative)

// Scalar * Matrix preserves noncommutativity
let expr = Expression::mul(vec![x_scalar.into(), A_matrix.into()]);
assert!(expr.commutativity() == Commutativity::Noncommutative);  // ✅ Inherits noncommutative

// ============================================================================
// Verification Summary
// ============================================================================
// ✅ All four types work: Scalar, Matrix, Operator, Quaternion
// ✅ Default to Scalar (commutative) matches SymPy
// ✅ Explicit opt-in for noncommutative types
// ✅ Parser infers from LaTeX notation
// ✅ Both singular symbol!() and plural symbols!() support all types
// ✅ Mathematical correctness: A*B ≠ B*A for noncommutative
```

---

## Why This Plan Will Succeed

1. **Based on SymPy's proven design** - 15+ years of battle-testing
2. **Minimal user complexity** - Default behavior unchanged for 95% of cases
3. **Explicit is better than implicit** - Matches Rust philosophy
4. **Incremental implementation** - Each wave builds on previous
5. **Mathematical correctness first** - SymPy validation built into every wave
6. **Performance safety** - Benchmarks prove zero overhead
7. **Backward compatible** - Existing code works (with migration for Mul pattern matching)
8. **CLAUDE.md enforced** - Quality and standards maintained

---

## Breaking Changes & Migration

### Breaking Change: Expression::Mul Signature

**Before:**
```rust
Expression::Mul(Box<Vec<Expression>>)
```

**After:**
```rust
Expression::Mul(Box<Vec<Expression>>, Commutativity)
```

### Migration Required:

All pattern matches on Mul must be updated:

```rust
// OLD (breaks):
match expr {
    Expression::Mul(factors) => { /* ... */ }
}

// NEW (works):
match expr {
    Expression::Mul(factors, commutativity) => { /* ... */ }
    // OR if commutativity not needed:
    Expression::Mul(factors, _) => { /* ... */ }
}
```

**Estimate**: ~100-150 match sites across codebase (Wave 2 handles this)

---

## SymPy Behavior Reference

From studying `~/Documents/work/math/sympy/`:

```python
# SymPy defaults to commutative=True
x, y = symbols("x y")  # commutative by default
assert x*y + y*x == 2*x*y  # ✅ Simplified

# Explicit noncommutative
A, B = symbols("A B", commutative=False)
assert A*B + B*A != 2*A*B  # ✅ NOT simplified
assert A*B != B*A  # ✅ Order matters

# Parser always creates commutative
from sympy.parsing.sympy_parser import parse_expr
expr = parse_expr("A*B")  # A and B are commutative by default
# User must explicitly create noncommutative symbols
```

**MathHook will match this behavior exactly.**

---

---

## Critical Design Decision Made During Planning

**Question Raised**: "Why are we giving explicit control?" (regarding `Expression::mul_with_commutativity()`)

**Answer**: We're NOT. This was removed from the design.

**Final Decision**:
- **NO explicit commutativity control** - commutativity is ALWAYS inferred from factors
- Symbol type determines commutativity (Scalar → Commutative, Matrix/Operator/Quaternion → Noncommutative)
- Expression::mul() automatically infers based on factors
- This matches SymPy's approach and prevents user error
- Simpler API = safer and more maintainable

**Rationale**: Commutativity is a property of the **symbols**, not the **operation**. The operation should infer it automatically.

---

**This orchestrator command is ready to use. Copy the bootstrap block and goal statement into a new Claude Code session.**

**Document Status**: ✅ Comprehensive 12-wave plan covering all MathHook modules
**Files Modified**: 37 core files across 8 major modules (core, simplify, calculus, algebra, pattern, matrix, parser, solvers, educational, formatters)
**Tests Added**: 425+ comprehensive tests
**Timeline**: 158-197 hours (4-6 weeks)
**Quality Target**: 9.0+/10
**Success Criteria**: 28 comprehensive checkpoints covering foundation, integration, user-facing, and quality
