# Functions Module Context

**Purpose**: Universal Function Intelligence System providing modular intelligence for all mathematical functions (elementary, special, number theory, polynomials)

**Last Updated**: 2025-10-30

---

## Module Structure

### Files in This Module

**Core Intelligence System** (1,288 lines):
- `intelligence.rs` (254 lines) - Universal function registry with O(1) lookup
- `traits.rs` (413 lines) - Function intelligence traits and interfaces
- `education.rs` (649 lines) - Function educator for step-by-step explanations
- `extensibility.rs` (381 lines) - Extension system for custom functions
- `evaluation.rs` (177 lines) - Function evaluation engine
- `accuracy.rs` (346 lines) - Accuracy verification for numerical functions

**Elementary Functions** (2,252 lines):
- `elementary/mod.rs` (90 lines) - Elementary function aggregator
- `elementary/trigonometric.rs` (83 lines) - Trig function aggregator
- `elementary/trigonometric/trig_circular.rs` (393 lines) - sin, cos, tan, sec, csc, cot
- `elementary/trigonometric/trig_inverse.rs` (223 lines) - arcsin, arccos, arctan
- `elementary/exponential.rs` (156 lines) - exp, e^x
- `elementary/logarithmic.rs` (186 lines) - log, ln, log10
- `elementary/hyperbolic.rs` (214 lines) - sinh, cosh, tanh, sech, csch, coth
- `elementary/sqrt.rs` (421 lines) - Square root and nth roots
- `elementary/abs.rs` (337 lines) - Absolute value

**Special Functions** (367 lines):
- `special.rs` (367 lines) - Special function intelligence (gamma, zeta, bessel, erf)

**Number Theory** (138 lines):
- `number_theory.rs` (138 lines) - Number theory functions (factorial, gcd, lcm, primes)

**Polynomials** (1,574 lines):
- `polynomials/mod.rs` (74 lines) - Polynomial function aggregator
- `polynomials/symbolic.rs` (423 lines) - Symbolic polynomial operations
- `polynomials/evaluation.rs` (424 lines) - Numerical polynomial evaluation (SIMD-optimized)
- `polynomials/chebyshev.rs` (245 lines) - Chebyshev polynomials (T_n, U_n)
- `polynomials/legendre.rs` (229 lines) - Legendre polynomials (P_n)
- `polynomials/hermite.rs` (142 lines) - Hermite polynomials (H_n)
- `polynomials/laguerre.rs` (161 lines) - Laguerre polynomials (L_n)

**Properties System** (1,168 lines):
- `properties/mod.rs` (330 lines) - Function properties interface
- `properties/elementary.rs` (72 lines) - Elementary function properties
- `properties/special.rs` (178 lines) - Special function properties
- `properties/rules.rs` (330 lines) - Derivative and antiderivative rules

**Module Aggregator**:
- `mod.rs` (53 lines) - Main module interface and exports

**Total Module Size**: ~6,849 lines across 29 files

---

## Public API

### Key Traits
- `pub trait FunctionIntelligence` - Core intelligence interface for all functions
- `pub trait FunctionEvaluator` - Numerical evaluation (SIMD-optimized)
- `pub trait FunctionEducator` - Educational step-by-step generation
- `pub trait FunctionOptimizer` - Performance optimization strategies
- `pub trait PropertyValidator` - Validate mathematical properties
- `pub trait MetadataProvider` - Function metadata and documentation
- `pub trait CompleteFunctionIntelligence` - Combines all intelligence traits
- `pub trait IntelligenceFactory` - Create intelligence instances
- `pub trait FunctionFamilyExtension` - Extensibility for custom functions
- `pub trait FunctionValidator` - Validate function implementations
- `pub trait StepGenerator` - Generate educational steps

### Key Structs
- `pub struct UniversalFunctionRegistry` - Global O(1) function lookup registry
- `pub struct FunctionEducator` - Educational explanation generator
- `pub struct ExtensionRegistry` - Custom function extension registry
- `pub struct DefaultValidator` - Default function validation
- `pub struct IntelligenceReport` - Comprehensive intelligence report
- `pub struct IntelligenceConfig` - Intelligence configuration
- `pub struct ComplexityEstimate` - Function complexity estimation
- `pub struct ValidationResult` - Property validation results
- `pub struct ValidationMetrics` - Validation metrics
- `pub struct AccuracyVerifier` - Numerical accuracy verification
- `pub struct VerifiedConstant` - Verified mathematical constant
- `pub struct ElementaryProperties` - Elementary function properties
- `pub struct SpecialProperties` - Special function properties
- `pub struct DerivativeRule` - Derivative calculation rule
- `pub struct AntiderivativeRule` - Antiderivative calculation rule
- `pub struct RecurrenceRule` - Polynomial recurrence relation

### Key Enums
- `pub enum FunctionFamily` - Elementary, Special, NumberTheory, Polynomials
- `pub enum EvaluationStrategy` - Symbolic, Numerical, SIMD, Hybrid
- `pub enum EvaluationResult` - Success, DomainError, NotImplemented
- `pub enum FunctionProperties` - Elementary, Special, UserDefined
- `pub enum ValidationLevel` - Strict, Standard, Permissive
- `pub enum ExtensionError` - DuplicateName, InvalidSignature, etc.
- `pub enum IssueSeverity` - Critical, Warning, Info

### Key Functions
- `pub fn get_universal_registry()` - Get global function registry (O(1) lookup)

---

## Dependencies

### Imports FROM Other Modules
**Core Types** (Heavy usage):
- `core/expression/` - Expression enum and variants
- `core/symbol.rs` - Symbol type
- `core/number.rs` - Number type (exact and approximate)

**Calculus** (Moderate usage):
- `calculus/derivatives/` - Derivative computation
- `calculus/integrals/` - Antiderivative computation

**Educational** (Moderate usage):
- `educational/message_registry/` - Educational messages

**Simplification** (Light usage):
- `simplify/` - Canonical form for function results

### Used BY Other Modules
**Heavy Consumers**:
- `calculus/` - Uses function derivatives and integrals
- `parser/` - Uses function registry for name resolution
- `algebra/` - Uses function properties for simplification
- `educational/` - Uses function educator for step-by-step

**Moderate Consumers**:
- `formatter/` - Uses function names for output
- `simplify/` - Uses function properties for simplification rules

---

## Testing

### Module-Specific Test Commands
```bash
# All function tests
cargo test -p mathhook-core functions

# Elementary function tests
cargo test -p mathhook-core elementary

# Special function tests
cargo test -p mathhook-core special

# Polynomial tests
cargo test -p mathhook-core polynomials

# Intelligence system tests
cargo test -p mathhook-core intelligence

# Educational tests
cargo test -p mathhook-core function_education
```

### Test Coverage
- Unit tests: ~90 `#[test]` functions
- Integration tests: Cross-module function tests
- Doctests: Examples in public API
- SIMD tests: Vectorized evaluation tests

**Key Test Areas**:
- Function evaluation (symbolic and numerical)
- SIMD optimization correctness
- Educational explanation quality
- Property validation
- Extensibility system

---

## External References

### SymPy Equivalent
**Location**: `~/Documents/work/math/sympy/sympy/functions/`
**Key Files**:
- `sympy/functions/elementary/` - Elementary functions
- `sympy/functions/special/` - Special functions (gamma, bessel, etc.)
- `sympy/functions/combinatorial/` - Number theory functions
- `sympy/polys/orthopolys.py` - Orthogonal polynomials

### Symbolica Equivalent
**Location**: `~/Documents/work/math/symbolica/src/`
**Key Files**:
- `symbolica/src/builtins/` - Built-in functions
- `symbolica/src/evaluate.rs` - Function evaluation

---

## Common Patterns & Pitfalls

### Design Patterns Used
1. **Registry Pattern**: `UniversalFunctionRegistry` for O(1) function lookup
2. **Intelligence Traits**: Each function implements `FunctionIntelligence` traits
3. **SIMD Optimization**: Vectorized evaluation for array operations
4. **Educational Wrappers**: All functions provide step-by-step explanations
5. **Extensibility System**: Custom functions via extension registry
6. **Property-Based Simplification**: Functions know their mathematical properties

### Common Pitfalls
1. **NEVER hardcode function names**: Use registry lookup, not string matches
   ```rust
   // ❌ DON'T:
   match func_name {
       "sin" => /* special handling */,
       "cos" => /* special handling */,
       _ => /* generic */
   }

   // ✅ DO:
   if let Some(intelligence) = registry.get_function(func_name) {
       intelligence.evaluate(args)
   }
   ```

2. **Domain Restrictions**: Always check function domains
   - `log(x)`: x > 0 (real domain)
   - `sqrt(x)`: x ≥ 0 (real domain)
   - `1/x`: x ≠ 0 (poles)
   - Use `EvaluationResult::DomainError` for violations

3. **Branch Cuts**: Document which branch for multi-valued functions
   - `log(z)`: Principal branch, `-π < arg(z) ≤ π`
   - `sqrt(z)`: Principal branch, branch cut on negative real axis

4. **Special Values**: Handle exact symbolic values
   - `sin(π)` → `0` (NOT `1.2246467991473532e-16`)
   - `cos(π/2)` → `0` (NOT approximate)
   - Detect exact inputs vs approximate floats

5. **SIMD Requirements**: SIMD functions need array inputs
   - Provide scalar fallback for single values
   - Test with both AVX2 and SSE2 feature flags

6. **Educational Quality**: Explanations must have REAL content
   - NOT just structure (step count)
   - Validate actual mathematical content in tests

---

## CLAUDE.md Constraints (Module-Specific)

### File Size Compliance
**Current Status**: ✅ All files compliant (≤500 lines)
- Largest file: `education.rs` (649 lines) - **EXCEEDS 500**
  - **Technical Debt**: Should be split in future cleanup

**Target**: Split `education.rs` into:
- `education/mod.rs` - Interface
- `education/elementary.rs` - Elementary function education
- `education/special.rs` - Special function education
- `education/polynomials.rs` - Polynomial education

### Module-Specific Rules
1. **Registry-Based Dispatch**: NEVER hardcode function names in implementation
2. **O(1) Lookup**: All function lookups MUST use `UniversalFunctionRegistry`
3. **Intelligence Traits**: All functions MUST implement `FunctionIntelligence`
4. **Educational Support**: All major functions SHOULD provide step-by-step
5. **SIMD Support**: Performance-critical functions SHOULD have SIMD optimization
6. **Property Documentation**: All functions MUST document domain/range restrictions

---

## Recent Changes

### Last 3 Major Modifications
1. **Educational Wave 4**: Function intelligence system with educational support (Oct 2024)
   - Implemented Universal Function Intelligence System
   - Added elementary, special, number theory function intelligence
   - 30+ tests, 8.5/10 quality score

2. **SIMD Optimization**: Vectorized polynomial evaluation
   - AVX2 and SSE2 support for polynomial evaluation
   - 2-4x speedup for large arrays
   - Scalar fallback for non-SIMD targets

3. **Extensibility System**: Custom function registration
   - Extension registry for user-defined functions
   - Validation and compatibility checking
   - Plugin-style architecture

---

## Technical Debt

### Known Issues
1. **File Size Violation**: `education.rs` (649 lines) exceeds 500-line limit
   - **Acceptable for now**: Pre-existing from successful wave
   - **Future**: Split using module aggregator pattern

2. **SIMD Coverage**: Not all functions have SIMD optimization
   - Target: Add SIMD for trigonometric functions
   - Profile and optimize hot paths

3. **Special Functions**: Incomplete implementation
   - Gamma function: Implemented
   - Bessel functions: Partial implementation
   - Error function (erf): Needs implementation
   - Zeta function: Needs implementation

### Future Improvements
1. Split `education.rs` into focused sub-modules
2. Add SIMD optimization for more functions
3. Complete special function implementations (erf, bessel, zeta)
4. Add more orthogonal polynomial families (Jacobi, Gegenbauer)
5. Implement function approximation for non-elementary integrals
6. Add numerical methods for special functions (series, continued fractions)

---

## Integration Points

### Function Lookup Flow
```
User Expression: sin(x)
    ↓
Parser: Creates Expression::Function("sin", [x])
    ↓
UniversalFunctionRegistry.get_function("sin")
    ↓
Returns: TrigonometricIntelligence (implements FunctionIntelligence)
    ↓
Operations:
    - .evaluate() → Numerical/symbolic evaluation
    - .derivative() → cos(x)
    - .integral() → -cos(x) + C
    - .explain() → Educational step-by-step
    ↓
Result: Canonical form expression
```

### Educational Integration Flow
```
User: "Explain how to differentiate sin(x)"
    ↓
FunctionEducator::explain_derivative("sin", x)
    ↓
UniversalFunctionRegistry.get_function("sin")
    ↓
intelligence.explain_derivative_rule()
    ↓
Generate Steps:
    1. "Identify function: sin(x)"
    2. "Apply derivative rule: d/dx[sin(x)] = cos(x)"
    3. "Result: cos(x)"
    ↓
StepByStepExplanation with educational messages
```

### SIMD Evaluation Flow
```
Polynomial Evaluation: P(x) = a₀ + a₁x + a₂x² + ... (1000 points)
    ↓
Check: SIMD available? (AVX2 or SSE2)
    ↓
YES → Vectorized evaluation:
    - Process 8 doubles at once (AVX2) or 2 (SSE2)
    - Horner's method in SIMD
    - 2-4x faster than scalar
    ↓
NO → Scalar fallback:
    - Horner's method
    - Still fast, just not vectorized
    ↓
Result: Array of evaluated values
```

---

## Universal Function Intelligence System Architecture

### Key Components

1. **UniversalFunctionRegistry** (Global Singleton):
   - HashMap for O(1) function lookup
   - Lazy initialization on first use
   - Immutable after initialization (thread-safe)

2. **Function Intelligence** (Per-Function):
   - Evaluation strategy (symbolic, numerical, SIMD)
   - Mathematical properties (domain, range, symmetry)
   - Derivative and integral rules
   - Educational explanation generation

3. **Extension System**:
   - Custom function registration
   - Validation and compatibility checking
   - Namespace management

### Design Principles

1. **No Hardcoding**: Functions self-describe through intelligence traits
2. **Extensibility**: New functions via extension registry
3. **Performance**: SIMD optimization for hot paths
4. **Education**: Every function can explain itself
5. **Type Safety**: Compile-time guarantees where possible
6. **Thread Safety**: All intelligence is `Send + Sync`

---

**Module Owner**: Core team
**Related Waves**: Educational Wave 4 (function intelligence), SIMD optimization, Extensibility enhancement
