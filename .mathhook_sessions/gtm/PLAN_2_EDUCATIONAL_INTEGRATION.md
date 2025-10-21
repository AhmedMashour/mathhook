# Plan 2: Educational System Integration

**Priority**: 📚 HIGH
**Timeline**: 7-9 weeks
**Waves**: 6
**Orchestrator**: `/sc:spawn`
**Version**: V3 (Added Wave 0: Pilot Testing & API Prototype)

## Executive Summary

**Current State**: Educational system is 30% complete
- ✅ **Architecture**: 8/10 - MessageRegistry, EducationalMessageGenerator, Step struct well-designed
- ❌ **Integration**: 2/10 - EducationalOperation trait has ZERO real-world implementations
- ⚠️  **Code Quality**: ~780 lines DUPLICATED in step_by_step.rs (lines 322-427 duplicate 1336-1443)
- ✅ **Messages**: 65+ messages implemented and tested

**Goal**: Complete educational system integration so all mathematical operations provide step-by-step explanations automatically.

**Unique Value Proposition**: This is MathHook's differentiator - **explainable symbolic mathematics** for education and neuro-symbolic AI (regulatory compliance).

---

## Root Cause Analysis: Code Duplication

**Problem**: 780 lines of duplicated code in `step_by_step.rs` (lines 322-427 duplicate 1336-1443)

**Root Causes Identified**:

1. **Rapid Prototyping Without Refactoring**:
   - Initial implementation focused on "making it work" for different mathematical operations
   - Each operation type (solve, simplify, expand, etc.) got its own implementation
   - Refactoring was deferred and never completed

2. **Lack of Clear Abstraction Pattern**:
   - `EducationalOperation` trait was designed but never enforced
   - No clear template for how to integrate educational features into new operations
   - Each developer created their own pattern, leading to duplication

3. **Integration Gaps**:
   - Educational system built in isolation from core operations
   - Core operations (solvers, simplification, calculus) didn't use `EducationalOperation` trait
   - Result: Educational code duplicated for each integration attempt

4. **Missing Validation Gates**:
   - No code review specifically checking for duplication
   - No automated detection of code similarity
   - No architectural enforcement of the trait-based pattern

**Prevention Strategy** (for this plan and future development):

1. **Mandatory Wave 0**: Prototype API and validate with users BEFORE full integration
2. **Trait Enforcement**: All new operations MUST implement `EducationalOperation` trait
3. **Code Review Gates**: Check for duplication before merging
4. **Integration Template**: Document standard pattern in CLAUDE.md for future reference
5. **User Validation**: Test with real users early to avoid building wrong abstractions

**Key Insight**: The duplication exists because we built the solution before fully understanding the problem. Wave 0 pilot testing will prevent this pattern from recurring.

---

## Bootstrap Command

Use this command to start the orchestrator:

```bash
/sc:spawn rust-engineer "Execute Wave-Based Educational System Integration for MathHook"
```

**Orchestrator Prompt**:

```markdown
You are the Orchestrator for **MathHook Educational System Integration**.

**Context**: You are the `rust-engineer` agent from `.claude/agents/rust-engineer.md` - Expert Rust developer specializing in systems programming, memory safety, and zero-cost abstractions for MathHook CAS.

MathHook has a well-designed educational architecture (MessageRegistry, EducationalOperation trait, 65+ messages) but it's NOT integrated into actual solvers and operations. Current assessment: 30% complete.

**Your Mission**: Execute a 6-wave plan to integrate step-by-step explanations into all core mathematical operations (Wave 0 focuses on user validation).

**Mandatory Reading** (in this order):
1. `/Users/ahmedmashhour/.claude/agents/rust-engineer.md` - Your agent specification
2. `/Users/ahmedmashhour/.mathhook_sessions/ORCHESTRATION_METHODOLOGY.md` - Proven wave-based methodology
3. `/Users/ahmedmashhour/Documents/work/math/mathhook/CLAUDE.md` - Project constraints
4. `/Users/ahmedmashhour/Documents/work/math/mathhook/.mathhook_sessions/PLAN_2_EDUCATIONAL_INTEGRATION.md` - This plan

**5 Mandatory Rules**:
1. **You Are Always The Orchestrator** - Delegate to rust-engineer agents
2. **Sequential Waves, Parallel Agents** - Complete waves in order
3. **Mandatory Verification** - Each wave ends with verification
4. **Strict CLAUDE.md Enforcement** - Follow documentation standards
5. **Maintain Momentum** - Report after each wave

**Wave Structure**: 6 waves starting with user validation (Wave 0), then targeting different integration points

**Success Criteria**:
- EducationalOperation trait implemented for all core operations
- `Expression.explain()` API works end-to-end
- Step-by-step quality improved based on user testing
- Zero code duplication

Begin by confirming understanding and reading mandatory files.
```

---

## Wave Breakdown

### Wave 0: Pilot Testing & API Prototype (1 week)

**Objective**: Validate educational approach with real users BEFORE full integration to prevent building wrong abstractions.

**Critical Success Criteria**:
- ✅ 3-5 pilot users recruited (mix of students and teachers)
- ✅ Prototype `Expression.explain()` API implemented (basic version)
- ✅ User feedback clarity score ≥7/10 average
- ✅ API design validated and frozen before Wave 1

**Tasks**:

1. **Recruit Pilot Users** (1 day):
   - Target: 3-5 users (mix of high school students, undergrad math students, teachers)
   - Channels: r/learnmath, math.stackexchange.com, local tutoring centers
   - Incentive: Free early access, acknowledgment in docs
   - Screening: Users comfortable with basic algebra and willing to provide detailed feedback

2. **Implement Prototype API** (2 days):
   - Create minimal `Expression.explain()` implementation
   - Support 3-4 operation types as proof-of-concept:
     - Linear equation solving: `2x + 3 = 7`
     - Quadratic solving: `x^2 - 5x + 6 = 0`
     - Polynomial expansion: `(x + 2)(x - 3)`
     - Basic simplification: `2x + 3x`
   - Output format: Step-by-step explanation with LaTeX
   - Example:
     ```rust
     let expr = parse("2x + 3 = 7");
     let explanation = expr.explain();
     // Returns: [
     //   "Step 1: Subtract 3 from both sides: 2x = 4",
     //   "Step 2: Divide both sides by 2: x = 2",
     //   "Solution: x = 2"
     // ]
     ```

3. **User Testing Protocol** (2 days):
   - Create standardized test cases (10 problems across 4 operation types)
   - Ask users to:
     a. Solve problems manually (without MathHook)
     b. Compare their steps to MathHook explanations
     c. Rate clarity (1-10 scale) for each explanation
     d. Provide specific feedback on confusing steps
   - Collect feedback via Google Forms + 15-min follow-up interviews
   - Document common confusion patterns and clarity scores

4. **API Design Freeze** (1 day):
   - Analyze user feedback and identify critical issues
   - Decision gate: **Only proceed to Wave 1 if average clarity score ≥7/10**
   - If score <7/10: Iterate on API design and re-test with 2-3 users
   - Once validated: **Freeze API signature** (critical for Plans 3 & 4 dependencies)
   - Document final API contract in `EDUCATIONAL_API.md`

**Agent Delegation**:
```bash
/sc:spawn rust-engineer "Implement Wave 0: Pilot Testing & API Prototype"
```

**Agent Prompt**:
```markdown
**Context**: You are the `rust-engineer` agent for MathHook CAS project.

Implement pilot testing and API prototype for educational features.

**Goal**: Validate educational approach with 3-5 real users before full integration.

**Tasks**:

1. **Recruit 3-5 pilot users**:
   - Post on r/learnmath, math.stackexchange.com
   - Target: Mix of students (high school/college) and teachers
   - Screening criteria: Comfortable with algebra, willing to give detailed feedback
   - Timeline: 1 day

2. **Implement prototype Expression.explain()**:
   - Create minimal implementation in `crates/mathhook-core/src/educational/prototype.rs`
   - Support 4 operation types: linear solving, quadratic solving, expansion, simplification
   - Use existing MessageRegistry where possible
   - Output: Step-by-step explanations as Vec<String>
   - Timeline: 2 days

3. **User testing protocol**:
   - Create 10 test problems (standardized across all users)
   - User workflow: Solve manually → Compare to MathHook → Rate clarity (1-10)
   - Collect feedback via Google Forms + 15-min interview
   - Document results in `.mathhook_sessions/pilot_testing_results.md`
   - Timeline: 2 days

4. **API design freeze**:
   - Analyze feedback, calculate average clarity score
   - Decision gate: ≥7/10 average → proceed to Wave 1
   - If <7/10 → iterate and re-test
   - Document final API in `EDUCATIONAL_API.md`
   - Timeline: 1 day

**Deliverables**:
- Prototype implementation (prototype.rs)
- Pilot testing results (.mathhook_sessions/pilot_testing_results.md)
- Frozen API specification (EDUCATIONAL_API.md)
- Go/No-Go decision for Wave 1

**Quality Target**: 9+/10 - Thorough user validation, clear API design
```

**Verification Script** (`verify_wave_0_pilot_testing.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 0 Verification: Pilot Testing ==="

# 1. Check prototype implementation
if [ ! -f "crates/mathhook-core/src/educational/prototype.rs" ]; then
    echo "❌ FAIL: Prototype implementation not found"
    exit 1
fi
echo "✅ Prototype implemented"

# 2. Run prototype tests
cargo test -p mathhook-core prototype --quiet
if [ $? -ne 0 ]; then
    echo "❌ FAIL: Prototype tests failing"
    exit 1
fi
echo "✅ Prototype tests passing"

# 3. Check pilot results
if [ ! -f ".mathhook_sessions/pilot_testing_results.md" ]; then
    echo "❌ FAIL: Pilot testing results not found"
    exit 1
fi

# Verify results have user ratings
if ! grep -q "Clarity Score:" .mathhook_sessions/pilot_testing_results.md; then
    echo "❌ FAIL: No clarity scores in results"
    exit 1
fi
echo "✅ Pilot testing results documented"

# 4. Check API specification
if [ ! -f "EDUCATIONAL_API.md" ]; then
    echo "❌ FAIL: API specification not found"
    exit 1
fi

if ! grep -q "Expression.explain()" EDUCATIONAL_API.md; then
    echo "❌ FAIL: API not documented"
    exit 1
fi
echo "✅ API specification frozen"

# 5. Validate average clarity score ≥7/10
avg_score=$(grep "Average Clarity Score:" .mathhook_sessions/pilot_testing_results.md | awk '{print $4}')
if (( $(echo "$avg_score < 7.0" | bc -l) )); then
    echo "❌ FAIL: Average clarity score $avg_score < 7.0"
    echo "⚠️  Cannot proceed to Wave 1 without user validation"
    exit 1
fi
echo "✅ Average clarity score: $avg_score/10 (≥7.0 required)"

echo ""
echo "=== Wave 0 Verification: PASSED ==="
echo "✅ Proceed to Wave 1: Integration Analysis & Cleanup"
```

**Deliverables**:
- `crates/mathhook-core/src/educational/prototype.rs`: Basic `Expression.explain()` implementation
- `.mathhook_sessions/pilot_testing_results.md`: User feedback summary with clarity scores
- `EDUCATIONAL_API.md`: Frozen API contract for `Expression.explain()`
- Decision: Go/No-Go for Wave 1 based on user validation

**Exit Criteria**:
- [ ] ≥3 pilot users tested prototype
- [ ] Average clarity score ≥7/10
- [ ] API design frozen and documented
- [ ] Common confusion patterns identified and documented

**Risks**:
- Low clarity scores may require API redesign (add 3-5 days if needed)
- Hard to recruit pilot users (mitigation: offer incentives, use existing contacts)
- User feedback may be too vague (mitigation: structured questionnaire + interviews)

**Dependencies**: None (Wave 0 is foundation for all subsequent waves)

**Unblocks**: Wave 1 (only proceed if pilot validates approach)

**Critical Insight**: This wave prevents the pattern that caused 780 lines of duplication - building the solution before understanding the problem. User validation FIRST, then full integration.

---

### Wave 1: Integration Analysis & Cleanup (4-6 hours)

**Objectives**:
1. Delete 780 lines of duplicated code in step_by_step.rs
2. Audit all uses of EducationalOperation trait
3. Create integration roadmap for core operations
4. Document current vs target state

**Agent Delegation**:
```bash
/sc:spawn rust-engineer "Clean up educational system codebase and analyze integration gaps"
```

**Agent Prompt**:
```markdown
**Context**: You are the `rust-engineer` agent for MathHook CAS project.

Clean up and analyze MathHook educational system integration.

**Critical Issue**: `crates/mathhook-core/src/educational/step_by_step.rs` has ~780 lines of DUPLICATED code (lines 322-427 duplicate 1336-1443).

**Tasks**:

1. **Delete Duplicated Code**:
   - Read: `crates/mathhook-core/src/educational/step_by_step.rs`
   - Identify exact duplicate sections
   - Delete duplicates, keep one canonical version
   - Run tests: `cargo test -p mathhook-core educational`
   - Commit: "refactor: remove 780 lines of duplicated code in step_by_step.rs"

2. **Audit EducationalOperation Trait**:
   - Find all files importing EducationalOperation
   - Count real vs test implementations
   - Document why trait exists but isn't used

3. **Integration Gap Analysis**:
   - List core operations lacking educational integration:
     - Equation solvers (linear, quadratic, polynomial)
     - Simplification strategies
     - Calculus operations (derivatives, integrals)
     - Matrix operations
   - For each, document: "Has educational messages? Has EducationalOperation impl?"

4. **Create Roadmap**:
   - Document: `.mathhook_sessions/educational_integration_roadmap.md`
   - Prioritize operations by user value (equation solving highest)
   - Estimate effort for each integration

**Deliverables**:
- Cleaned step_by_step.rs (no duplication)
- Educational trait audit report
- Integration gap analysis
- Roadmap for waves 2-5

**Quality Target**: 9+/10 - Thorough, actionable analysis
```

**Verification Script** (`verify_wave_1_educational_cleanup.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 1 Verification: Educational Cleanup ==="

# 1. Check for duplication removal
duplicate_lines=$(awk 'NR==322,NR==427' crates/mathhook-core/src/educational/step_by_step.rs | wc -l)
if [ "$duplicate_lines" -gt 0 ]; then
    # Verify these aren't the actual duplicates anymore
    if diff <(awk 'NR==322,NR==427' crates/mathhook-core/src/educational/step_by_step.rs) \
            <(awk 'NR==1336,NR==1443' crates/mathhook-core/src/educational/step_by_step.rs) &>/dev/null; then
        echo "❌ FAIL: Duplication still exists"
        exit 1
    fi
fi
echo "✅ Duplication removed"

# 2. Run educational tests
echo "Running educational tests..."
cargo test -p mathhook-core educational --quiet
if [ $? -ne 0 ]; then
    echo "❌ FAIL: Educational tests failing"
    exit 1
fi
echo "✅ Educational tests passing"

# 3. Check roadmap exists
if [ ! -f ".mathhook_sessions/educational_integration_roadmap.md" ]; then
    echo "❌ FAIL: Integration roadmap not found"
    exit 1
fi
echo "✅ Integration roadmap created"

# 4. Validate roadmap has required sections
required_sections=("Current State" "Target State" "Integration Gaps" "Wave 2-5 Plan")
for section in "${required_sections[@]}"; do
    if ! grep -qi "$section" .mathhook_sessions/educational_integration_roadmap.md; then
        echo "❌ FAIL: Missing section: $section"
        exit 1
    fi
done
echo "✅ All required sections present"

echo ""
echo "=== Wave 1 Verification: PASSED ==="
```

**Success Criteria**:
- [ ] 780 lines of duplication deleted
- [ ] Tests still passing
- [ ] Integration gaps documented
- [ ] Roadmap created with priorities

---

### Wave 2: Solver Integration (8-12 hours)

**Objectives**:
1. Implement EducationalOperation for equation solvers
2. Integrate step-by-step explanations into solve() function
3. Test with real equation-solving workflows
4. Document educational API patterns for future integrations

**Agent Delegation**:
```bash
/sc:spawn rust-engineer "Integrate educational features into equation solvers"
```

**Agent Prompt**:
```markdown
**Context**: You are the `rust-engineer` agent for MathHook CAS project.

Integrate step-by-step explanations into MathHook equation solvers.

**Context**: Equation solving is the HIGHEST value operation for students. Must have excellent educational output.

**Reference Files**:
- `crates/mathhook-core/src/solvers/` - Equation solvers
- `crates/mathhook-core/src/educational/message_registry/` - Messages
- `crates/mathhook-core/src/educational/step_by_step.rs` - Step infrastructure

**Tasks**:

1. **Implement EducationalOperation for Solvers**:
   ```rust
   // In solvers/linear.rs
   impl EducationalOperation for LinearSolver {
       fn explain_operation(&self, input: &Expression) -> StepByStepExplanation {
           // Use MessageRegistry to generate steps
           // Track each solving step (isolate variable, simplify, etc.)
       }
   }

   // Repeat for QuadraticSolver, PolynomialSolver
   ```

2. **Integrate into solve() Function**:
   ```rust
   pub fn solve(
       equation: &Expression,
       variable: &Symbol,
       show_steps: bool
   ) -> SolveResult {
       // If show_steps, call solver.explain_operation()
       // Return both solutions and explanation
   }
   ```

3. **Educational Message Enhancement**:
   - Review existing messages in message_registry
   - Add missing messages for edge cases:
     - No solutions case
     - Infinite solutions case
     - Complex solutions case
   - Improve clarity based on educational standards

4. **Testing**:
   - Create tests in `crates/mathhook-core/tests/educational/solver_explanations.rs`
   - Test each solver type with show_steps=true
   - Validate explanation quality (clear, accurate, complete)

5. **Documentation**:
   - Document pattern in `.mathhook_sessions/educational_integration_pattern.md`
   - Provide template for integrating other operations

**Deliverables**:
- EducationalOperation impls for all solver types
- solve() function with show_steps parameter
- Enhanced educational messages
- Comprehensive tests
- Integration pattern documentation

**Quality Target**: 9+/10 - Clear, accurate, pedagogically sound explanations
```

**Verification Script** (`verify_wave_2_solver_integration.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 2 Verification: Solver Integration ==="

# 1. Check EducationalOperation implementations
echo "Checking EducationalOperation implementations..."
if ! grep -q "impl EducationalOperation for LinearSolver" crates/mathhook-core/src/solvers/linear.rs; then
    echo "❌ FAIL: LinearSolver doesn't implement EducationalOperation"
    exit 1
fi
echo "✅ Solver implementations found"

# 2. Run solver educational tests
echo "Running solver educational tests..."
cargo test -p mathhook-core solver_explanations --quiet
if [ $? -ne 0 ]; then
    echo "❌ FAIL: Solver explanation tests failing"
    exit 1
fi
echo "✅ Solver tests passing"

# 3. Test manual workflow
echo "Testing step-by-step solving..."
cargo run --example educational_solving 2>&1 | tee solving_output.txt

if ! grep -q "Step 1:" solving_output.txt; then
    echo "❌ FAIL: No step-by-step output generated"
    exit 1
fi
echo "✅ Step-by-step output working"

# 4. Check documentation
if [ ! -f ".mathhook_sessions/educational_integration_pattern.md" ]; then
    echo "❌ FAIL: Integration pattern not documented"
    exit 1
fi
echo "✅ Integration pattern documented"

# 5. Count educational messages
message_count=$(grep -r "pub fn.*_steps" crates/mathhook-core/src/educational/message_registry/ | wc -l)
echo "Educational message generators: $message_count"
if [ "$message_count" -lt 10 ]; then
    echo "⚠️  WARNING: Low message count, may need more coverage"
fi

echo ""
echo "=== Wave 2 Verification: PASSED ==="
```

**Success Criteria**:
- [ ] All solver types implement EducationalOperation
- [ ] solve() function has show_steps parameter
- [ ] Tests validate explanation quality
- [ ] Integration pattern documented

---

### Wave 3: Expression.explain() API (10-14 hours)

**Objectives**:
1. Design Expression.explain() public API
2. Implement for all Expression variants
3. Route to appropriate educational handlers
4. Create examples demonstrating API usage

**Agent Delegation**:
```bash
/sc:spawn rust-engineer "Implement Expression.explain() API for end-to-end educational features"
```

**Agent Prompt**:
```markdown
**Context**: You are the `rust-engineer` agent for MathHook CAS project.

Implement unified Expression.explain() API for educational features.

**Goal**: Users should be able to call `expr.explain()` on any expression and get educational output.

**API Design**:
```rust
impl Expression {
    /// Generate step-by-step explanation for this expression
    pub fn explain(&self, operation: EducationalContext) -> StepByStepExplanation {
        match operation {
            EducationalContext::Simplify => self.explain_simplification(),
            EducationalContext::Expand => self.explain_expansion(),
            EducationalContext::Factor => self.explain_factorization(),
            EducationalContext::Solve(var) => self.explain_solving(var),
            EducationalContext::Differentiate(var) => self.explain_differentiation(var),
            EducationalContext::Integrate(var) => self.explain_integration(var),
        }
    }
}

pub enum EducationalContext {
    Simplify,
    Expand,
    Factor,
    Solve(Symbol),
    Differentiate(Symbol),
    Integrate(Symbol),
}
```

**Tasks**:

1. **Define Public API**:
   - Add to `crates/mathhook-core/src/core/expression/mod.rs`
   - Design EducationalContext enum
   - Implement routing logic

2. **Connect to Existing Infrastructure**:
   - Route Simplify → step_by_step.rs simplification
   - Route Solve → solver EducationalOperation implementations
   - Route Differentiate → calculus educational handlers (Wave 4)
   - Route Integrate → calculus educational handlers (Wave 4)

3. **Error Handling**:
   - Handle cases where operation doesn't apply
   - Provide helpful error messages
   - Example: "Cannot solve for variable 'x' as it doesn't appear in expression"

4. **Documentation & Examples**:
   - Add rustdoc examples to Expression.explain()
   - Create `examples/educational_api.rs` showing all contexts
   - Document in CLAUDE.md

5. **Python Bindings**:
   - Add to `crates/mathhook-python/src/lib.rs`:
     ```python
     @pymethods
     impl PyExpression {
         pub fn explain(&self, context: &str) -> String {
             // Call Rust Expression.explain()
             // Return JSON with steps
         }
     }
     ```

**Deliverables**:
- Expression.explain() API implemented
- EducationalContext enum with all operation types
- Routing logic connecting to existing educational handlers
- Comprehensive examples
- Python bindings for explain()

**Quality Target**: 9+/10 - Ergonomic, well-documented, comprehensive API
```

**Verification Script** (`verify_wave_3_explain_api.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 3 Verification: Expression.explain() API ==="

# 1. Check API exists
echo "Checking Expression.explain() API..."
if ! grep -q "pub fn explain" crates/mathhook-core/src/core/expression/mod.rs; then
    echo "❌ FAIL: Expression.explain() not found"
    exit 1
fi
echo "✅ explain() API exists"

# 2. Check EducationalContext enum
if ! grep -q "pub enum EducationalContext" crates/mathhook-core/src/educational/mod.rs; then
    echo "❌ FAIL: EducationalContext enum not found"
    exit 1
fi
echo "✅ EducationalContext defined"

# 3. Run examples
echo "Running educational API example..."
cargo run --example educational_api 2>&1 | tee api_example_output.txt

if ! grep -q "Simplification" api_example_output.txt; then
    echo "❌ FAIL: Example doesn't show simplification"
    exit 1
fi
echo "✅ Example works"

# 4. Check Python bindings
if ! grep -q "pub fn explain" crates/mathhook-python/src/lib.rs; then
    echo "❌ FAIL: Python explain() binding not found"
    exit 1
fi
echo "✅ Python binding exists"

# 5. Run doctests
echo "Running Expression.explain() doctests..."
cargo test --doc expression::explain --quiet
if [ $? -ne 0 ]; then
    echo "❌ FAIL: Doctests failing"
    exit 1
fi
echo "✅ Doctests passing"

echo ""
echo "=== Wave 3 Verification: PASSED ==="
```

**Success Criteria**:
- [ ] Expression.explain() API implemented
- [ ] All EducationalContext variants work
- [ ] Examples demonstrate all use cases
- [ ] Python bindings available

---

### Wave 4: Message Quality Improvement (8-10 hours)

**Objectives**:
1. User test current educational messages
2. Improve clarity and pedagogical quality
3. Add visual explanations (ASCII diagrams where helpful)
4. Ensure consistency across message types

**Agent Delegation**:
```bash
/sc:spawn rust-engineer "Improve educational message quality based on user testing"
```

**Agent Prompt**:
```markdown
**Context**: You are the `rust-engineer` agent for MathHook CAS project.

Improve quality of educational messages based on user testing.

**Context**: 65+ messages exist but quality varies. Need consistent, clear, pedagogically sound messages.

**Tasks**:

1. **User Testing Protocol**:
   - Select 10 representative math problems (algebra, calculus, linear algebra)
   - Generate step-by-step explanations using current system
   - Ask 3-5 users (students/teachers) to rate:
     - Clarity (1-10)
     - Completeness (1-10)
     - Pedagogical quality (1-10)
   - Document feedback in `.mathhook_sessions/educational_user_testing.md`

2. **Message Improvement**:
   - For messages scoring <7/10, rewrite based on feedback
   - Add "Why this step?" explanations
   - Include common mistakes/misconceptions
   - Add ASCII diagrams where visual aids help (e.g., fraction simplification)

3. **Consistency Audit**:
   - Ensure all messages follow same structure:
     1. What we're doing
     2. Why we're doing it
     3. How to do it
     4. Result
   - Use consistent terminology across messages
   - Maintain appropriate reading level (high school math)

4. **Special Cases**:
   - Add messages for edge cases:
     - Division by zero
     - No solutions
     - Infinite solutions
     - Domain restrictions
   - Ensure error messages are educational, not just "failed"

5. **A/B Testing** (if time permits):
   - Create two versions of key messages
   - Test with users to find clearer version
   - Document winning patterns

**Deliverables**:
- User testing report with ratings
- Improved messages (target: all ≥7/10)
- Consistency guidelines document
- Edge case message coverage

**Quality Target**: 9+/10 - Clear, pedagogically sound, user-validated messages
```

**Verification Script** (`verify_wave_4_message_quality.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 4 Verification: Message Quality ==="

# 1. Check user testing report
if [ ! -f ".mathhook_sessions/educational_user_testing.md" ]; then
    echo "❌ FAIL: User testing report not found"
    exit 1
fi
echo "✅ User testing report exists"

# 2. Validate user testing data
if ! grep -q "Clarity:" .mathhook_sessions/educational_user_testing.md; then
    echo "❌ FAIL: No clarity ratings in report"
    exit 1
fi
echo "✅ User ratings documented"

# 3. Check for message improvements
message_files=$(find crates/mathhook-core/src/educational/message_registry -name "*.rs" | wc -l)
echo "Message files: $message_files"

# 4. Run educational tests
echo "Running educational message tests..."
cargo test -p mathhook-core message_registry --quiet
if [ $? -ne 0 ]; then
    echo "❌ FAIL: Message tests failing"
    exit 1
fi
echo "✅ Message tests passing"

# 5. Check consistency guidelines
if [ ! -f ".mathhook_sessions/educational_message_guidelines.md" ]; then
    echo "⚠️  WARNING: Consistency guidelines not documented"
else
    echo "✅ Guidelines documented"
fi

# 6. Sample message quality check
echo ""
echo "Sample message quality check:"
cargo run --example message_quality_check 2>&1 | head -20

echo ""
echo "=== Wave 4 Verification: PASSED ==="
```

**Success Criteria**:
- [ ] User testing completed with ≥3 users
- [ ] All messages score ≥7/10
- [ ] Consistency guidelines established
- [ ] Edge cases covered

---

### Wave 5: End-to-End Examples & Documentation (6-8 hours)

**Objectives**:
1. Create comprehensive examples showing educational features
2. Update CLAUDE.md with educational API patterns
3. Create tutorial documentation for users
4. Prepare marketing materials highlighting educational differentiator

**Agent Delegation**:
```bash
/sc:spawn rust-engineer "Create comprehensive documentation for educational features"
```

**Agent Prompt**:
```markdown
**Context**: You are the `rust-engineer` agent for MathHook CAS project.

Create comprehensive documentation for MathHook educational features.

**Goal**: Make educational features discoverable and easy to use.

**Tasks**:

1. **Create Examples**:
   - `examples/educational_solving.rs` - Equation solving with explanations
   - `examples/educational_calculus.rs` - Derivatives and integrals with steps
   - `examples/educational_simplification.rs` - Expression simplification
   - `examples/educational_python.py` - Python API usage
   - Each example should be runnable and well-commented

2. **Update CLAUDE.md**:
   - Add "Educational Features" section
   - Document Expression.explain() API
   - Document EducationalOperation trait pattern
   - Show integration examples for future operations

3. **Create Tutorial**:
   - `docs/tutorials/educational_features.md`
   - Target audience: Users wanting step-by-step math
   - Cover: Basic usage, customization, error handling
   - Include screenshots of output (if applicable)

4. **Marketing Materials**:
   - Create `.mathhook_sessions/educational_differentiator.md`
   - Highlight unique value: "Explainable symbolic mathematics"
   - Target markets:
     - Education (students/teachers)
     - Neuro-symbolic AI (regulatory explainability)
     - Research (reproducible mathematical workflows)
   - Competitive analysis: SymPy doesn't have this depth of educational integration

5. **API Reference**:
   - Ensure all public educational APIs have rustdoc
   - Generate docs: `cargo doc --no-deps --open`
   - Review for completeness and clarity

**Deliverables**:
- 4+ comprehensive examples
- Updated CLAUDE.md
- Tutorial documentation
- Marketing materials
- Complete API reference

**Quality Target**: 9+/10 - Clear, comprehensive, user-friendly documentation
```

**Verification Script** (`verify_wave_5_documentation.sh`):
```bash
#!/bin/bash
set -e

echo "=== Wave 5 Verification: Documentation ==="

# 1. Check examples exist
examples=("educational_solving" "educational_calculus" "educational_simplification" "educational_python")
for example in "${examples[@]}"; do
    if [ ! -f "examples/${example}.rs" ] && [ ! -f "examples/${example}.py" ]; then
        echo "❌ FAIL: Example not found: $example"
        exit 1
    fi
done
echo "✅ All examples exist"

# 2. Run examples
echo "Running educational examples..."
for example in educational_solving educational_calculus educational_simplification; do
    cargo run --example $example &>/dev/null
    if [ $? -ne 0 ]; then
        echo "❌ FAIL: Example failed: $example"
        exit 1
    fi
    echo "  ✓ $example works"
done
echo "✅ Examples execute successfully"

# 3. Check CLAUDE.md update
if ! grep -q "Educational Features" CLAUDE.md; then
    echo "❌ FAIL: CLAUDE.md not updated with educational section"
    exit 1
fi
echo "✅ CLAUDE.md updated"

# 4. Check tutorial
if [ ! -f "docs/tutorials/educational_features.md" ]; then
    echo "❌ FAIL: Tutorial not created"
    exit 1
fi
echo "✅ Tutorial created"

# 5. Check marketing materials
if [ ! -f ".mathhook_sessions/educational_differentiator.md" ]; then
    echo "❌ FAIL: Marketing materials not created"
    exit 1
fi
echo "✅ Marketing materials ready"

# 6. Generate and check API docs
echo "Generating API documentation..."
cargo doc --no-deps --quiet
if [ ! -d "target/doc/mathhook_core/educational" ]; then
    echo "❌ FAIL: Educational API docs not generated"
    exit 1
fi
echo "✅ API documentation complete"

echo ""
echo "=== Wave 5 Verification: PASSED ==="
```

**Success Criteria**:
- [ ] All examples work and are well-commented
- [ ] CLAUDE.md has comprehensive educational section
- [ ] Tutorial suitable for new users
- [ ] Marketing materials ready
- [ ] API reference complete

---

## Final Success Criteria

### Wave Completion Checklist
- [ ] Wave 0: Pilot testing complete, API design frozen (≥7/10 clarity score)
- [ ] Wave 1: Duplication removed, integration roadmap created
- [ ] Wave 2: Solver integration complete, EducationalOperation implemented
- [ ] Wave 3: Expression.explain() API working end-to-end
- [ ] Wave 4: Message quality validated with users (≥7/10 all messages)
- [ ] Wave 5: Comprehensive documentation and examples

### Quality Metrics
- All waves score ≥ 8/10
- User testing shows ≥7/10 for message quality
- Zero code duplication
- 100% EducationalOperation coverage for core operations

### Deliverables Checklist
- [ ] Wave 0: Prototype implementation, pilot testing results, frozen API specification
- [ ] Cleaned step_by_step.rs (no duplication)
- [ ] EducationalOperation impls for all solver types
- [ ] Expression.explain() API with all contexts
- [ ] Improved messages (user-validated)
- [ ] Comprehensive examples (4+)
- [ ] Updated CLAUDE.md
- [ ] Tutorial documentation
- [ ] Marketing materials

### Exit Criteria
- **Unique Differentiator**: Educational features work end-to-end
- **API Ergonomics**: expr.explain() is discoverable and easy to use
- **Quality Validated**: User testing confirms pedagogical soundness
- **Marketing Ready**: Differentiator materials prepared

---

## Competitive Advantage

**SymPy**: Has some educational features but not integrated system-wide
**Mathematica**: Has excellent step-by-step but costs $25K/year
**MathHook**: Open-source with integrated educational features - **unique position**

**Target Markets**:
1. **Education**: Students, teachers, online learning platforms
2. **Neuro-Symbolic AI**: Regulatory compliance requires explainable reasoning
3. **Research**: Reproducible workflows with documented steps

This plan establishes MathHook's core differentiator in the CAS market.
