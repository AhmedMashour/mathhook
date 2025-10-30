# Calculus Module Context

**Purpose**: Calculus operations including derivatives, integrals, limits, series expansion, residue calculus, and partial derivatives

**Last Updated**: 2025-10-30

---

## Module Structure

### Files in This Module

**Derivatives** (4,745 lines):
- `derivatives.rs` (101 lines) - Main derivative interface and trait
- `derivatives/basic.rs` (232 lines) - Power rule, constant rule, sum rule
- `derivatives/power_rule.rs` (266 lines) - Generalized power rule implementation
- `derivatives/chain_rule.rs` (332 lines) - Chain rule for composite functions
- `derivatives/product_rule.rs` (476 lines) - Product rule and quotient rule
- `derivatives/higher_order.rs` (488 lines) - Second, third, nth derivatives
- `derivatives/checker.rs` (288 lines) - Derivative correctness verification
- `derivatives/advanced_differentiation.rs` (133 lines) - Advanced techniques aggregator
- `derivatives/advanced_differentiation/implicit.rs` (572 lines) - Implicit differentiation
- `derivatives/advanced_differentiation/parametric.rs` (259 lines) - Parametric equations
- `derivatives/advanced_differentiation/vector_valued/mod.rs` (56 lines) - Vector calculus aggregator
- `derivatives/advanced_differentiation/vector_valued/components.rs` (322 lines) - Vector components
- `derivatives/advanced_differentiation/vector_valued/geometry.rs` (294 lines) - Geometric applications
- `derivatives/educational/mod.rs` (366 lines) - Educational explanations
- `derivatives/educational/basic_rules.rs` (281 lines) - Basic rule explanations
- `derivatives/educational/composition_rules.rs` (337 lines) - Chain rule explanations

**Partial Derivatives** (2,478 lines):
- `derivatives/partial.rs` (163 lines) - Partial derivative interface
- `derivatives/partial/gradient.rs` (401 lines) - Gradient vector computation
- `derivatives/partial/hessian.rs` (429 lines) - Hessian matrix (second partials)
- `derivatives/partial/jacobian.rs` (616 lines) - Jacobian matrix for multivariable functions
- `derivatives/partial/utils.rs` (530 lines) - Partial derivative utilities
- `derivatives/partial/vector_fields.rs` (13 lines) - Vector field operations aggregator
- `derivatives/partial/vector_fields/operations.rs` (197 lines) - Curl, divergence, Laplacian
- `derivatives/partial/vector_fields/conservative.rs` (154 lines) - Conservative field testing
- `derivatives/partial/vector_fields/fluid_dynamics.rs` (84 lines) - Fluid dynamics applications
- `derivatives/partial/vector_fields/tests.rs` (296 lines) - Vector calculus tests

**Integrals** (5,654 lines):
- `integrals.rs` (143 lines) - Main integration interface and trait
- `integrals/basic.rs` (404 lines) - Power rule, constant rule, sum rule integration
- `integrals/substitution.rs` (440 lines) - U-substitution technique
- `integrals/by_parts.rs` (300 lines) - Integration by parts
- `integrals/rational.rs` (492 lines) - Partial fraction decomposition
- `integrals/trigonometric.rs` (520 lines) - Trigonometric integration
- `integrals/function_integrals.rs` (310 lines) - Special function integration
- `integrals/table.rs` (499 lines) - Integration table lookup
- `integrals/strategy.rs` (225 lines) - Integration strategy selection
- `integrals/educational.rs` (670 lines) - Step-by-step integration explanations
- `integrals/risch/mod.rs` (125 lines) - Risch algorithm aggregator
- `integrals/risch/differential_extension.rs` (302 lines) - Differential field extensions
- `integrals/risch/hermite.rs` (211 lines) - Hermite reduction for rational functions
- `integrals/risch/rde.rs` (478 lines) - Risch differential equation solver
- `integrals/risch/helpers.rs` (42 lines) - Risch algorithm helpers

**Other Calculus Operations** (2,091 lines):
- `limits.rs` (1,096 lines) - Limit computation (one-sided, two-sided, at infinity)
- `series.rs` (455 lines) - Taylor series, Maclaurin series, power series
- `summation.rs` (430 lines) - Summation formulas and techniques
- `residues.rs` (468 lines) - Complex residue calculus, contour integration

**Total Module Size**: ~14,968 lines across 45 files

---

## Public API

### Key Traits
- `pub trait Derivative` - Differentiation interface
- `pub trait Integration` - Integration interface
- `pub trait Limits` - Limit computation interface
- `pub trait SeriesExpansion` - Series expansion interface
- `pub trait ResidueCalculus` - Residue and contour integration
- `pub trait ComplexAnalysis` - Complex analysis operations

### Key Structs
- `pub struct IntegrationMethods` - Integration implementation
- `pub struct BasicIntegrals` - Basic integration rules
- `pub struct IntegrationByParts` - Integration by parts implementation
- `pub struct PartialFractionDecomposition` - Rational function decomposition
- `pub struct FunctionIntegrals` - Special function integration
- `pub struct LimitMethods` - Limit computation methods
- `pub struct LimitEducation` - Educational limit explanations
- `pub struct SeriesMethods` - Series expansion methods
- `pub struct ResidueMethods` - Residue calculus methods
- `pub struct IntegrationExplanation` - Educational integration explanations

### Key Enums
- `pub enum LimitDirection` - Left, Right, Bidirectional limits
- `pub enum SeriesType` - Taylor, Maclaurin, Laurent, Fourier
- `pub enum SingularityType` - Removable, Pole, Essential
- `pub enum RischResult` - Elementary, NonElementary, Partial
- `pub enum DifferentialExtension` - Exponential, Logarithmic, Algebraic
- `pub enum PatternKey` - Integration pattern matching keys

### Key Functions
- `pub fn try_substitution()` - U-substitution attempt
- `pub fn try_table_lookup()` - Integration table lookup
- `pub fn is_rational_function()` - Rational function detection
- `pub fn integrate_rational()` - Rational function integration
- `pub fn try_risch_integration()` - Risch algorithm attempt
- `pub fn hermite_reduction()` - Hermite reduction for rational functions
- `pub fn explain_power_rule()` - Educational power rule explanation
- `pub fn explain_u_substitution()` - Educational substitution explanation
- `pub fn explain_integration_by_parts()` - Educational by-parts explanation

---

## Dependencies

### Imports FROM Other Modules
**Core Types** (Heavy usage):
- `core/expression/` - Expression, Add, Mul, Pow, Function variants
- `core/symbol.rs` - Symbol type for variables
- `core/number.rs` - Number type (exact rationals)

**Algebra** (Heavy usage):
- `algebra/` - Simplification, expansion, rational operations
- `algebra/solvers/` - Equation solving for definite integrals

**Functions** (Heavy usage):
- `functions/elementary/` - sin, cos, exp, log derivatives/integrals
- `functions/special/` - gamma, erf, bessel function calculus

**Simplification** (Critical):
- `simplify/` - Canonical form, simplification strategies
- `pattern/` - Pattern matching for integration

**Educational** (Moderate usage):
- `educational/message_registry/` - Step-by-step explanation messages

### Used BY Other Modules
**Primary Consumers**:
- `educational/` - Uses derivative/integral explanations for step-by-step
- `functions/` - Function calculus properties (derivatives, integrals)
- `algebra/` - Uses limits for zero detection and asymptotic behavior

**Secondary Consumers**:
- `parser/` - Uses calculus operators (∫, d/dx, ∂/∂x, ∑, lim)
- `formatter/` - Uses calculus notation for LaTeX output
- `matrix/` - Uses Jacobian for matrix calculus

---

## Testing

### Module-Specific Test Commands
```bash
# All calculus tests
cargo test -p mathhook-core calculus

# Derivative tests only
cargo test -p mathhook-core derivatives

# Integration tests only
cargo test -p mathhook-core integrals

# Limits tests
cargo test -p mathhook-core limits

# Vector calculus tests
cargo test -p mathhook-core vector_fields

# Educational tests
cargo test -p mathhook-core calculus_educational
```

### Test Coverage
- Unit tests: ~180 `#[test]` functions
- Integration tests: Cross-module calculus tests
- Doctests: Examples in public API documentation
- Educational tests: Content validation (not just structure)

**Key Test Files**:
- `derivatives/partial/vector_fields/tests.rs` (296 lines) - Vector calculus tests
- Educational Wave 3 added comprehensive derivative/integral tests

---

## External References

### SymPy Equivalent
**Location**: `~/Documents/work/math/sympy/sympy/calculus/`
**Key Files**:
- `sympy/calculus/singularities.py` - Singularity analysis
- `sympy/core/function.py` - Derivative implementation
- `sympy/integrals/integrals.py` - Integration interface
- `sympy/integrals/risch.py` - Risch algorithm
- `sympy/series/series.py` - Series expansion
- `sympy/calculus/limits.py` - Limit computation

### Symbolica Equivalent
**Location**: `~/Documents/work/math/symbolica/src/`
**Key Files**:
- `symbolica/src/derive.rs` - Derivative operations
- `symbolica/src/integrate.rs` - Integration operations
- `symbolica/src/series.rs` - Series expansion

---

## Common Patterns & Pitfalls

### Design Patterns Used
1. **Trait-based Polymorphism**: All operations use traits (Derivative, Integration, Limits)
2. **Strategy Pattern**: Integration uses strategy selection (`integrals/strategy.rs`)
3. **Table Lookup**: Integration table for common patterns
4. **Educational Wrappers**: Every major operation has educational explanation variant
5. **Rule Composition**: Derivative rules compose (chain rule uses product rule)

### Common Pitfalls
1. **Domain Restrictions**: Check for log(0), division by zero, sqrt of negatives
2. **Branch Cuts**: Complex functions have branch cuts (log, arcsin, etc.)
3. **Integration Constants**: Indefinite integrals need `+ C` (represented symbolically)
4. **Limit Direction**: One-sided vs two-sided limits matter (discontinuities)
5. **Simplification**: Always simplify results to canonical form
6. **Higher-Order Derivatives**: Use chain rule carefully for nth derivatives
7. **Partial Derivatives**: Order matters for mixed partials (verify equality conditions)
8. **Vector Calculus**: Curl and divergence order of operations
9. **Risch Algorithm**: Some integrals are non-elementary (erf, li)

---

## CLAUDE.md Constraints (Module-Specific)

### File Size Compliance
**Current Status**: ⚠️ 1 file exceeds 500 lines (pre-existing, documented)
- `limits.rs` (1,096 lines) - Technical debt, should be split

**Target**: Split `limits.rs` into:
- `limits/mod.rs` - Interface
- `limits/finite.rs` - Finite limits
- `limits/infinity.rs` - Limits at infinity
- `limits/educational.rs` - Educational explanations

### Module-Specific Rules
1. **Educational Integration**: All major calculus operations MUST have educational variants
2. **Simplification**: All results MUST be simplified to canonical form
3. **Exact Results**: Use exact symbolic integration, not numerical approximation
4. **Risch Algorithm**: Attempt Risch for transcendental integrals before giving up
5. **Message Registry**: Use educational registry for step-by-step explanations

---

## Recent Changes

### Last 3 Major Modifications
1. **Wave 3**: Calculus educational implementation (Educational Waves)
   - Added comprehensive derivative explanations
   - Added integration step-by-step with multiple strategies
   - 30+ tests, 9/10 quality score

2. **Advanced Differentiation**: Implicit and parametric differentiation
   - Implicit differentiation for implicit curves
   - Parametric derivatives for parametric equations
   - Vector-valued function calculus

3. **Risch Algorithm**: Symbolic integration enhancement
   - Hermite reduction for rational functions
   - Differential extension tower construction
   - RDE (Risch Differential Equation) solver

---

## Technical Debt

### Known Issues
1. **File Size Violation**: `limits.rs` (1,096 lines) needs splitting
   - **Future**: Split using module aggregator pattern in cleanup wave

2. **Integration Table**: Integration table could be more comprehensive
   - Target: Add 50+ more common integral patterns

3. **Risch Algorithm**: Incomplete implementation
   - Elementary transcendental integrals work
   - Non-elementary detection incomplete (needs more work)

4. **Performance**: Large expression integration can be slow
   - Consider caching intermediate results
   - Profile and optimize hot paths

### Future Improvements
1. Split `limits.rs` into focused sub-modules
2. Expand integration table with more patterns
3. Complete Risch algorithm for all elementary functions
4. Add Maclaurin series for more functions
5. Optimize integration strategy selection (machine learning?)
6. Add more vector calculus operations (line integrals, surface integrals)

---

## Integration Points

### Derivative Flow
```
User Expression → Derivative trait
    ↓
Check type:
    - Basic polynomial → power_rule
    - Composite function → chain_rule
    - Product → product_rule
    - Implicit → implicit differentiation
    - Partial → gradient/Jacobian/Hessian
    ↓
Simplify result → canonical form
    ↓
Optional: Educational explanation
```

### Integration Flow
```
User Expression → Integration trait
    ↓
Strategy Selection:
    1. Check integration table (common patterns)
    2. Try basic rules (power, constant, sum)
    3. Try u-substitution
    4. Try integration by parts
    5. Check if rational → partial fractions
    6. Try trigonometric techniques
    7. Attempt Risch algorithm (transcendental)
    ↓
Simplify result → canonical form
    ↓
Optional: Educational explanation with chosen strategy
```

### Educational Integration
Calculus operations with educational support:
- `Derivative` → `derivatives/educational/` for step-by-step
- `Integration` → `integrals/educational.rs` with strategy explanations
- `Limits` → `LimitEducation` for limit explanations
- Educational registry messages in `educational/message_registry/calculus.rs`

---

**Module Owner**: Core team
**Related Waves**: Educational Wave 3 (calculus implementation), Advanced Differentiation enhancement
