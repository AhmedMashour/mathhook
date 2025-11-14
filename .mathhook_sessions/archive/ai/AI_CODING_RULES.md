# 🎯 AI CODING RULES - HOW WE CODE IN MATHHOOK

## 📋 **AI INSTRUCTIONS: READ THIS BEFORE ANY CODING**

**THESE ARE THE FUNDAMENTAL RULES FOR ALL CODING IN MATHHOOK**
- NEVER violate these rules
- Always check against these rules before implementing
- Update these rules only with explicit user approval
- These rules define our coding culture and quality standards

---

## 🏗️ **ARCHITECTURE RULES**

### **MAGIC BULLETS (NEVER COMPROMISE):**
1. **Magic Bullet #1**: CompactNumber (16-byte optimized numbers)
   - ALWAYS use `CompactNumber` instead of raw numeric types
   - SmallInt(i64) for common cases, Box<BigInt> for large numbers
   - Preserve performance optimizations

2. **Magic Bullet #2**: CompactExpression (32-byte optimized expressions)
   - Expression IS the compact implementation (no separate types)
   - ALWAYS use `Box<Vec<Expression>>` for Add/Mul/Function variants
   - Maintain 32-byte size constraint

3. **Magic Bullet #3**: Performance Normalization
   - Single optimized path for all operations
   - NO separate "fast" and "slow" implementations
   - Consistent performance across all operations

4. **Magic Bullet #4**: SIMD Integration
   - Use SIMD for bulk operations (16+ elements threshold)
   - Manual loop unrolling where beneficial
   - Smart thresholds to avoid overhead

5. **Magic Bullet #5**: Hot Path + Memory Optimization
   - `#[inline(always)]` for hot paths
   - Arena allocation for large expression trees
   - Pre-allocation strategies where possible

### **MEMORY MANAGEMENT RULES:**
- Box large enum variants to keep enum size small
- Use arena allocation for temporary large structures
- Prefer stack allocation for small, short-lived objects
- Always measure memory usage impact

### **PERFORMANCE RULES:**
- ALWAYS benchmark new functionality
- Maintain 4.5M+ operations per second minimum
- Use `#[inline(always)]` judiciously on hot paths
- Profile before optimizing, measure after optimizing

---

## 🧪 **TDD RULES (MANDATORY)**

### **TEST-DRIVEN DEVELOPMENT CYCLE:**
1. **RED**: Write failing test first
   - Test MUST fail initially (compilation error or assertion failure)
   - Write comprehensive tests covering all expected functionality
   - Document expected behavior clearly

2. **GREEN**: Make test pass with minimal code
   - Implement just enough to make the test pass
   - Don't over-engineer the solution
   - Focus on correctness first, optimization later

3. **REFACTOR**: Clean up while keeping tests green
   - Apply Magic Bullets optimizations
   - Improve code structure and readability
   - Maintain all tests passing

### **TEST QUALITY RULES:**
- EVERY new module MUST use TDD approach
- Test coverage >95% for all new code
- Include edge cases and error conditions
- Test performance regressions
- Document test purpose and expected behavior

### **TEST ORGANIZATION:**
- Separate test files for each module
- Group related tests in modules
- Use descriptive test names
- Include both unit and integration tests

---

## 📚 **DOCUMENTATION RULES**

### **USER REQUIREMENT PRESERVATION:**
- ALWAYS preserve user's exact words in documentation
- Quote user requirements verbatim
- Trace implementation back to user requirements
- Document decision rationale

### **STEP-BY-STEP DOCUMENTATION:**
- Document EVERY implementation step
- Explain WHY decisions were made
- Record failed approaches and lessons learned
- Maintain implementation timeline

### **SESSION MANAGEMENT:**
- Update checklists immediately after changes
- Create comprehensive session summaries
- Prepare restoration commands for next session
- Archive completed work properly

### **CODE DOCUMENTATION:**
- Document all public APIs
- Explain complex algorithms
- Include usage examples
- Document performance characteristics

---

## 🔧 **CODE QUALITY RULES**

### **RUST BEST PRACTICES:**
- Follow The Rust Programming Language Book guidelines
- ZERO compilation warnings (mandatory)
- Use `#[allow(dead_code)]` only for planned future features
- Prefer `_` prefix for intentionally unused variables

### **NAMING CONVENTIONS:**
- Use clear, descriptive names
- Follow Rust naming conventions (snake_case, PascalCase)
- Avoid abbreviations unless widely understood
- Use consistent terminology throughout codebase

### **ERROR HANDLING:**
- Use Result types for fallible operations
- Create specific error types for each module
- Provide helpful error messages
- Handle errors at appropriate levels

### **IMPORT ORGANIZATION:**
- Only import what you actually use
- Group imports logically (std, external, internal)
- Use explicit imports over glob imports
- Keep import lists clean and minimal

---

## 🎯 **SYMPY COMPATIBILITY RULES**

### **API COMPATIBILITY:**
- Match SymPy function signatures where possible
- Use similar result formats and types
- Maintain mathematical correctness
- Document compatibility levels

### **MATHEMATICAL ACCURACY:**
- Verify results against SymPy
- Handle edge cases consistently
- Maintain numerical stability
- Document algorithmic differences

### **FEATURE PARITY:**
- Implement core SymPy functionality
- Prioritize commonly used features
- Document missing features clearly
- Plan implementation roadmap

---

## 📊 **PERFORMANCE RULES**

### **BENCHMARKING REQUIREMENTS:**
- Benchmark all new functionality
- Compare against previous performance
- Set performance regression thresholds
- Document performance characteristics

### **OPTIMIZATION PRIORITIES:**
1. Correctness first (must be mathematically correct)
2. Performance second (maintain 4.5M+ ops/sec)
3. Memory efficiency third (maintain Magic Bullets)
4. Code clarity fourth (readable and maintainable)

### **PERFORMANCE MONITORING:**
- Regular performance regression testing
- Monitor memory usage patterns
- Profile hot paths regularly
- Maintain performance metrics database

---

## 🔄 **MAINTENANCE RULES**

### **CHECKLIST MAINTENANCE:**
- Update checklists immediately after changes
- Mark completed items with timestamps
- Cross-reference between different checklists
- Keep checklists synchronized

### **SESSION MANAGEMENT:**
- Document every session comprehensively
- Create clear restoration commands
- Maintain session continuity
- Archive completed sessions

### **VERSION CONTROL:**
- Commit frequently with descriptive messages
- Tag major milestones
- Maintain clean git history
- Document breaking changes

---

## 🚨 **CRITICAL RULES (NEVER VIOLATE)**

### **ZERO TOLERANCE RULES:**
1. **ZERO WARNINGS**: No compilation warnings allowed
2. **TDD MANDATORY**: All new modules must use TDD
3. **USER VOICE**: Always preserve user's exact requirements
4. **MAGIC BULLETS**: Never compromise on performance optimizations
5. **DOCUMENTATION**: Every step must be documented

### **QUALITY GATES:**
- All tests must pass before committing
- Performance must meet or exceed targets
- Documentation must be complete
- User requirements must be fulfilled
- Code review checklist must be satisfied

---

## 🎯 **AI SELF-CHECK RULES**

### **BEFORE ANY IMPLEMENTATION:**
1. Have I read and understood user requirements?
2. Am I following TDD approach?
3. Are all Magic Bullets being preserved?
4. Is this documented step-by-step?
5. Will this maintain zero warnings?

### **AFTER ANY IMPLEMENTATION:**
1. Are all tests passing?
2. Is performance maintained or improved?
3. Are checklists updated?
4. Is documentation complete?
5. Are user requirements fulfilled?

### **BEFORE SESSION END:**
1. Is all work documented?
2. Are restoration commands prepared?
3. Are checklists up to date?
4. Is next session prepared?
5. Is user satisfied with progress?

---

## 📝 **RULE VIOLATION PROTOCOL**

### **IF RULES MUST BE BROKEN:**
1. Get explicit user approval first
2. Document the violation and rationale
3. Plan remediation if possible
4. Update rules if permanently changed
5. Notify user of any impacts

### **RULE UPDATE PROCESS:**
1. Only update with explicit user request
2. Document what changed and why
3. Update all related documentation
4. Ensure team alignment
5. Archive old rules for reference

---

*These rules define our coding excellence - follow them religiously!* 🎯
