# Educational Module Context

**Purpose**: Step-by-step mathematical explanations and educational message system for interactive learning

**Last Updated**: 2025-10-30

---

## Module Structure

### Files in This Module

**Core Educational System** (2,282 lines):
- `step_by_step.rs` (1,995 lines) - Main step-by-step explanation engine
- `traits.rs` (297 lines) - Educational traits and operation context
- `enhanced_steps.rs` (87 lines) - Enhanced steps module aggregator

**Enhanced Steps** (786 lines):
- `enhanced_steps/generation.rs` (502 lines) - Smart step generation with difficulty adaptation
- `enhanced_steps/formatting.rs` (284 lines) - Step formatting and presentation

**Message Registry** (2,063 lines):
- `message_registry/mod.rs` (261 lines) - Educational message registry interface
- `message_registry/calculus.rs` (484 lines) - Calculus-specific educational messages
- `message_registry/core.rs` (364 lines) - Core linear/quadratic/system messages
- `message_registry/algebra.rs` (356 lines) - Algebraic operation messages
- `message_registry/noncommutative.rs` (261 lines) - Noncommutative algebra messages (Wave 11)
- `message_registry/solvers.rs` (240 lines) - Equation solver messages

**Total Module Size**: ~5,131 lines across 11 files

---

## Public API

### Key Traits
- `pub trait EducationalOperation` - Interface for operations with educational support
- `pub trait EducationalExt` - Extension trait for expressions to generate explanations
- `pub trait StepByStep` - Generate step-by-step explanations for expressions

### Key Structs
- `pub struct Step` - Single explanation step with content and metadata
- `pub struct StepByStepExplanation` - Complete multi-step explanation
- `pub struct StepByStepBuilder` - Builder pattern for creating explanations
- `pub struct EnhancedStep` - Advanced step with API data and difficulty level
- `pub struct StepFactory` - Factory for creating steps from templates
- `pub struct SmartStepBuilder` - Intelligent step generation with adaptation
- `pub struct EnhancedStepBuilder` - Builder for enhanced steps
- `pub struct FormatContext` - Formatting context for step presentation
- `pub struct PresentationHints` - Hints for how to present explanations
- `pub struct EnhancedStepExplanation` - Complete enhanced explanation
- `pub struct ExplanationMetadata` - Metadata about explanation (difficulty, duration, etc.)
- `pub struct ExplanationSummary` - Summary of explanation content
- `pub struct EducationalMessageGenerator` - Generates educational messages
- `pub struct MessageOptimizer` - Optimizes message clarity and length
- `pub struct MessageKey` - Key for looking up educational messages
- `pub struct MessageTemplate` - Template for educational message generation
- `pub struct MessageBuilder` - Builder for creating messages
- `pub struct MessageHashSystem` - Hash system for message caching
- `pub struct OperationContext` - Context for educational operations
- `pub struct StepApiData` - API data embedded in steps
- `pub struct MathContext` - Mathematical context for step generation
- `pub struct EducationalResult` - Result of educational operation

### Key Enums
- `pub enum DifficultyLevel` - Beginner, Intermediate, Advanced
- `pub enum MessageCategory` - Concept, Strategy, Explanation, Hint, Warning
- `pub enum MessageType` - Linear, Quadratic, System, Calculus, Algebra, NonCommutative

### Key Functions
- `pub fn initialize_solver_messages()` - Initialize equation solver messages
- `pub fn initialize_noncommutative_messages()` - Initialize noncommutative algebra messages (Wave 11)
- `pub fn initialize_calculus_messages()` - Initialize calculus operation messages
- `pub fn initialize_algebra_messages()` - Initialize algebraic operation messages
- `pub fn initialize_linear_messages()` - Initialize linear equation messages
- `pub fn initialize_quadratic_messages()` - Initialize quadratic equation messages
- `pub fn initialize_system_messages()` - Initialize system equation messages
- `pub fn initialize_general_messages()` - Initialize general educational messages

---

## Dependencies

### Imports FROM Other Modules
**Core Types** (Heavy usage):
- `core/expression/` - Expression enum for mathematical operations
- `core/symbol.rs` - Symbol type for variables
- `core/number.rs` - Number type for exact values

**Calculus** (Moderate usage):
- `calculus/derivatives/` - Derivative educational integration
- `calculus/integrals/` - Integration educational integration

**Algebra** (Moderate usage):
- `algebra/solvers/` - Solver step-by-step explanations
- `algebra/simplify/` - Simplification explanations

**Functions** (Light usage):
- `functions/` - Function-specific educational messages

### Used BY Other Modules
**Primary Consumers**:
- `calculus/` - Uses educational traits for derivative/integral explanations
- `algebra/solvers/` - Uses message registry for solver step-by-step
- `functions/` - Uses step-by-step for function evaluations
- `parser/` - May use educational messages for parse error guidance

**Secondary Consumers**:
- `formatter/` - Uses presentation hints for output formatting
- User-facing APIs - All educational features exposed through high-level API

---

## Testing

### Module-Specific Test Commands
```bash
# All educational tests
cargo test -p mathhook-core educational

# Step-by-step tests
cargo test -p mathhook-core step_by_step

# Message registry tests
cargo test -p mathhook-core message_registry

# Educational traits tests
cargo test -p mathhook-core educational_traits
```

### Test Coverage
- Unit tests: ~21 `#[test]` functions
- Integration tests: Cross-module educational integration
- Doctests: Examples in public API

**Key Test Areas**:
- Step generation and formatting
- Message template rendering
- Difficulty level adaptation
- Educational operation trait implementations
- Noncommutative algebra explanations (Wave 11)

---

## External References

### SymPy Equivalent
**Location**: `~/Documents/work/math/sympy/sympy/interactive/`
**Key Files**:
- `sympy/interactive/session.py` - Interactive educational session
- `sympy/printing/pretty/` - Pretty printing for educational output

**Note**: SymPy doesn't have a dedicated educational module; explanations are ad-hoc

### Symbolica Equivalent
**Location**: N/A
**Note**: Symbolica doesn't provide educational features; it's focused on raw performance

---

## Common Patterns & Pitfalls

### Design Patterns Used
1. **Builder Pattern**: `StepByStepBuilder`, `EnhancedStepBuilder`, `MessageBuilder` for flexible step creation
2. **Factory Pattern**: `StepFactory` for creating steps from templates
3. **Template Pattern**: `MessageTemplate` for parameterized message generation
4. **Registry Pattern**: `MessageRegistry` for O(1) message lookup
5. **Trait Extension**: `EducationalExt` adds step-by-step to all expressions
6. **Strategy Pattern**: Different educational strategies based on `DifficultyLevel`

### Common Pitfalls
1. **Empty Steps**: Ensure every step has meaningful content
   - NOT just structure (step count)
   - Validate actual mathematical content in tests

2. **Message Template Errors**: Template parameters must match provided data
   - Example: `"Solution: {x}"` requires `x` in data map
   - Missing parameters cause runtime errors

3. **Difficulty Level Mismatch**: Educational explanations must match declared difficulty
   - Beginner: Simple language, detailed steps
   - Advanced: Concise, assumes prior knowledge

4. **Circular Dependencies**: Educational module uses calculus/algebra, which use educational
   - Avoid tight coupling
   - Use traits to break circular imports

5. **Message Registry Initialization**: Registry must be initialized before use
   - Call all `initialize_*_messages()` functions at startup
   - Missing initialization causes message lookup failures

6. **Step Order**: Steps must be in logical mathematical order
   - Test that step progression makes sense
   - Each step should build on previous steps

7. **LaTeX in Messages**: Educational messages may contain LaTeX
   - Ensure LaTeX is valid and well-formed
   - Test LaTeX rendering in formatter

---

## CLAUDE.md Constraints (Module-Specific)

### File Size Compliance
**Current Status**: ⚠️ **2 files exceed 500 lines** (significant technical debt)

**File Size Violations**:
- `step_by_step.rs` (1,995 lines) - **EXCEEDS 500 by 1,495 lines** ⚠️
  - **Critical**: This is nearly 4x the limit
  - **Target Split**:
    - `step_by_step/mod.rs` - Interface (≤200 lines)
    - `step_by_step/builder.rs` - Builder pattern (≤300 lines)
    - `step_by_step/explanation.rs` - Explanation struct (≤300 lines)
    - `step_by_step/traits.rs` - Trait implementations (≤300 lines)
    - `step_by_step/formatting.rs` - Formatting logic (≤300 lines)

- `enhanced_steps/generation.rs` (502 lines) - **EXCEEDS 500 by 2 lines**
  - **Minor**: Just over the limit
  - **Target Split**:
    - `enhanced_steps/generation/mod.rs` - Interface (≤200 lines)
    - `enhanced_steps/generation/smart_builder.rs` - Smart builder (≤200 lines)
    - `enhanced_steps/generation/factory.rs` - Factory pattern (≤200 lines)

**Priority**: HIGH - These files should be split in next major refactor

### Module-Specific Rules
1. **Content Quality**: Educational explanations MUST have real mathematical content
   - NOT just structure (step 1, step 2, ...)
   - Test content quality, not just presence

2. **Message Registry**: All mathematical operations SHOULD have educational messages
   - Linear equations ✅
   - Quadratic equations ✅
   - Systems ✅
   - Calculus ✅
   - Algebra ✅
   - Noncommutative ✅ (Wave 11)

3. **Difficulty Adaptation**: Educational content MUST adapt to difficulty level
   - Beginner: Maximum detail, simple language
   - Intermediate: Balanced detail, standard terminology
   - Advanced: Concise, assumes understanding

4. **Step Progression**: Steps MUST follow logical mathematical progression
   - Each step builds on previous
   - No logical gaps
   - Test step order validity

5. **LaTeX Integration**: All educational content SHOULD support LaTeX rendering
   - Mathematical notation in steps
   - Formula presentation
   - Work with formatter module

---

## Recent Changes

### Last 3 Major Modifications
1. **Wave 11**: Educational integration for noncommutative algebra (Oct 2024)
   - Added `message_registry/noncommutative.rs` (261 lines)
   - Educational explanations for matrix equations
   - Left vs right division explanations
   - Why order matters for matrices/operators
   - 30 tests, 9.5/10 quality score

2. **Enhanced Steps System**: Smart step generation with difficulty adaptation
   - Added `enhanced_steps/generation.rs` (502 lines)
   - `SmartStepBuilder` for intelligent step creation
   - `DifficultyLevel` enum for learner adaptation
   - `StepFactory` for template-based generation

3. **Message Registry Expansion**: Comprehensive message system
   - Calculus messages (derivatives, integrals, limits, series)
   - Algebra messages (expand, factor, simplify, collect)
   - Solver messages (linear, quadratic, polynomial, systems)
   - Template-based message generation

---

## Technical Debt

### Known Issues
1. **File Size Violations** (CRITICAL):
   - `step_by_step.rs` (1,995 lines) - 4x the 500-line limit
   - `enhanced_steps/generation.rs` (502 lines) - Slightly over limit
   - **Impact**: Hard to navigate, understand, and modify
   - **Priority**: HIGH - Should be split in next cleanup wave

2. **Message Template Validation**: No compile-time validation of templates
   - Template parameters checked at runtime
   - Missing parameter causes panic
   - **Future**: Consider proc macro for compile-time template validation

3. **Difficulty Level Testing**: Insufficient testing of difficulty adaptation
   - Tests verify steps exist, but not that difficulty matches
   - **Future**: Add tests that validate beginner vs advanced content quality

4. **Educational Content Quality**: Tests verify structure, not content richness
   - Tests check step count, not mathematical depth
   - **Future**: Add content quality metrics (formula count, explanation depth)

### Future Improvements
1. **Split `step_by_step.rs`**: Break into focused sub-modules using module aggregator pattern
2. **Compile-time Template Validation**: Use proc macros to validate message templates at compile time
3. **Interactive Educational Mode**: Socratic questioning, user-guided explanations
4. **Adaptive Learning**: Track user understanding, adjust difficulty dynamically
5. **Multi-language Support**: Educational messages in multiple languages
6. **Visual Educational Aids**: Integrate with formatter for diagrams, graphs
7. **Educational Test Harness**: Framework for testing educational content quality

---

## Integration Points

### Educational Flow
```
User Expression → Solver/Calculator/Simplifier
    ↓
Operation uses EducationalExt trait
    ↓
StepByStepBuilder creates explanation
    ↓
MessageRegistry provides educational messages
    ↓
EnhancedStepBuilder adapts to difficulty level
    ↓
FormatContext applies presentation hints
    ↓
Result: Comprehensive step-by-step explanation
```

### Message Registry Flow
```
Initialization:
    initialize_linear_messages()
    initialize_quadratic_messages()
    initialize_calculus_messages()
    initialize_algebra_messages()
    initialize_noncommutative_messages()
    ↓
Registry populated with templates
    ↓
Operation lookup:
    MessageKey → MessageTemplate → Rendered message
    ↓
Template parameters filled from operation context
    ↓
Result: Educational message for step
```

### Difficulty Adaptation Flow
```
User selects DifficultyLevel (Beginner/Intermediate/Advanced)
    ↓
SmartStepBuilder receives level
    ↓
StepFactory selects appropriate templates
    ↓
For Beginner:
    - More steps
    - Detailed explanations
    - Simple language
    - Formula breakdowns

For Advanced:
    - Fewer steps
    - Concise explanations
    - Mathematical terminology
    - Assumes understanding
    ↓
Result: Adapted educational content
```

### Cross-Module Integration
```
Calculus Module:
    Derivative calculation → derivatives/educational.rs
    → Uses educational/message_registry/calculus.rs
    → Generates step-by-step derivative explanation

Algebra Module:
    Equation solving → algebra/solvers/polynomial/educational.rs
    → Uses educational/message_registry/solvers.rs
    → Generates step-by-step solution

Functions Module:
    Function evaluation → functions/education.rs
    → Uses educational/message_registry/core.rs
    → Generates function-specific explanation
```

---

## Educational Design Principles

### Core Philosophy
1. **Content Over Structure**: Real mathematical insight, not just procedural steps
2. **Progressive Disclosure**: Start simple, add complexity as needed
3. **Multiple Representations**: Symbolic, numerical, graphical where applicable
4. **Why Before How**: Explain rationale before procedure
5. **Active Learning**: Encourage user engagement, not passive reading

### Message Design Guidelines
1. **Clear Language**: Accessible to target difficulty level
2. **Mathematical Rigor**: Correct terminology and notation
3. **Contextual Help**: Explain "why" not just "what"
4. **Consistent Style**: Uniform voice across all messages
5. **LaTeX Integration**: Mathematical notation where appropriate

### Step Generation Guidelines
1. **Atomic Steps**: Each step is a single logical operation
2. **Explicit Reasoning**: State the rule or principle used
3. **Forward Progress**: Each step moves toward solution
4. **Verification**: Show that intermediate results are correct
5. **Summary**: Conclude with clear answer statement

---

**Module Owner**: Core team
**Related Waves**: Educational Waves 1-5 (100% success rate), Wave 11 (noncommutative algebra)
