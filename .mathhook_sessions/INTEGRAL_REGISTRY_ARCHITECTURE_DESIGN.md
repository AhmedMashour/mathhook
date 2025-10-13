# Integral/Antiderivative Registry Architecture Design

**Author**: Claude Code
**Date**: 2025-10-13
**Status**: Phase 1 COMPLETE - Phase 2 COMPLETE - Phase 3 COMPLETE - Ready for Phase 4
**Last Updated**: 2025-10-13 06:46:25
**Goal**: Extend `UniversalFunctionRegistry` to support registry-based integral computation

---

## Executive Summary

This design document proposes extending the existing `UniversalFunctionRegistry` and `FunctionProperties` system to support antiderivatives/integrals in a parallel architecture to the existing derivative system. The goal is to eliminate hardcoded integral rules from `calculus/integrals/function_integrals.rs` and replace them with declarative, registry-based rules similar to how derivatives currently work.

**Status Update (2025-10-13 06:46:25)**:
- Phase 1 (Type System): ✅ COMPLETE - All types defined in properties.rs, 4/4 tests passing
- Phase 2 (Test Infrastructure): ✅ COMPLETE - 36 tests created, 26 passing, 10 awaiting implementation
- Phase 3 (Analysis): ✅ COMPLETE - Full refactoring analysis document created (PHASE_3_ANALYSIS_FUNCTION_INTEGRALS_REFACTORING.md)
- Phase 4 (Registry Population): ⏳ READY TO START - All prerequisites met
- Phase 5 (Refactoring): ⏳ PENDING - Blocked on Phase 4

**Key Benefits:**
1. **Consistency**: Mirrors the proven derivative architecture
2. **Extensibility**: Easy to add new function integrals
3. **Performance**: O(1) lookup for function integral rules
4. **Maintainability**: Centralized integral knowledge in function properties
5. **Educational**: Each integral rule can include step-by-step explanations

---

## 1. Current State Analysis

### 1.1 Derivative System (Current Implementation)

**Location**: `crates/mathhook-core/src/functions/properties.rs`

**Key Types:**
```rust
// Derivative rule for automatic differentiation
pub struct DerivativeRule {
    pub rule_type: DerivativeRuleType,
    pub result_template: String,
}

pub enum DerivativeRuleType {
    Simple(String),        // d/dx sin(x) = cos(x)
    ChainRule(String),     // d/dx sin(u) = cos(u) * du/dx
    ProductRule,           // d/dx (uv) = u'v + uv'
    QuotientRule,          // d/dx (u/v) = (u'v - uv')/v²
}
```

**Integration Pattern:**
- Each function's `ElementaryProperties` contains `derivative_rule: Option<DerivativeRule>`
- Registry lookup: O(1) via `UniversalFunctionRegistry`
- Usage in `calculus/derivatives/chain_rule.rs`: Hardcoded fallback (lines 69-331)

**Critical Observation:**
The current derivative implementation is **hybrid**: registry stores metadata, but actual derivative computation is still hardcoded in `FunctionDerivatives::get()`. This is **NOT** fully registry-driven.

### 1.2 Current Integral System

**Location**: `crates/mathhook-core/src/calculus/integrals/function_integrals.rs`

**Current Approach:**
- Hardcoded match statements for each function (lines 59-230)
- Special cases: simple functions, composite functions, linear substitution
- No registry integration

**Example Hardcoded Rules:**
```rust
match name {
    "sin" => Expression::mul(vec![
        Expression::integer(-1),
        Expression::function("cos", vec![Expression::symbol(variable)]),
    ]),
    "cos" => Expression::function("sin", vec![Expression::symbol(variable)]),
    // ... 30+ more cases
}
```

**Complexity:**
- 70+ lines of integral logic
- Composite function handling (substitution, linear transformation)
- No reusability across different integral contexts

---

## 2. Proposed Type Definitions

### 2.1 Core Antiderivative Types

**File**: `crates/mathhook-core/src/functions/properties.rs`

```rust
/// Antiderivative rule for automatic integration
///
/// Stores the antiderivative formula for a function, analogous to DerivativeRule.
/// Supports simple antiderivatives, substitution patterns, and special techniques.
#[derive(Debug, Clone)]
pub struct AntiderivativeRule {
    /// Rule type for efficient computation
    pub rule_type: AntiderivativeRuleType,

    /// Result expression template (for documentation and validation)
    /// Example: "∫sin(x)dx = -cos(x) + C"
    pub result_template: String,

    /// Constant of integration behavior
    pub constant_handling: ConstantOfIntegration,
}

/// Types of antiderivative rules for performance optimization
#[derive(Debug, Clone)]
pub enum AntiderivativeRuleType {
    /// Simple substitution: ∫sin(x)dx = -cos(x) + C
    /// Contains the antiderivative function name
    Simple {
        /// Name of the antiderivative function
        antiderivative_fn: String,

        /// Multiplicative coefficient (e.g., -1 for sin → -cos)
        coefficient: Expression,
    },

    /// Linear substitution: ∫f(ax)dx = (1/a)F(ax) + C
    /// Used for patterns like ∫sin(3x)dx = -(1/3)cos(3x) + C
    LinearSubstitution {
        /// Antiderivative of f(x)
        base_antiderivative: Box<AntiderivativeRule>,
    },

    /// Composite function requiring u-substitution
    /// ∫f(g(x))g'(x)dx = F(g(x)) + C
    USubstitution {
        /// Pattern to match g'(x)
        derivative_pattern: String,

        /// Antiderivative in terms of u
        antiderivative_of_u: String,
    },

    /// Integration by parts: ∫u dv = uv - ∫v du
    ByParts {
        /// Choice of u (the part to differentiate)
        u_pattern: String,

        /// Choice of dv (the part to integrate)
        dv_pattern: String,
    },

    /// Trigonometric substitution patterns
    /// Used for integrals like ∫1/√(1-x²)dx = arcsin(x) + C
    TrigSubstitution {
        /// Pattern to match (e.g., √(1-x²), √(x²+1))
        pattern: String,

        /// Substitution to use (e.g., x = sin(θ), x = tan(θ))
        substitution: String,

        /// Resulting antiderivative
        result: String,
    },

    /// Partial fraction decomposition (for rational functions)
    PartialFractions {
        /// Degree constraint (only if denominator degree > numerator degree)
        requires_proper_fraction: bool,
    },

    /// Reduction formula (recursive integration)
    /// Example: ∫sin^n(x)dx in terms of ∫sin^(n-2)(x)dx
    ReductionFormula {
        /// Recursion relation
        recursion: String,

        /// Base cases
        base_cases: Vec<(usize, String)>,
    },

    /// Special function integral (e.g., ∫e^(-x²)dx = √π/2 erf(x) + C)
    SpecialFunction {
        /// Name of special function result
        special_fn: String,

        /// Coefficients and transformations
        coefficients: Vec<Expression>,
    },

    /// Not integrable in elementary functions
    /// Returns symbolic integral expression
    NonElementary,
}

/// Constant of integration handling
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstantOfIntegration {
    /// Automatically add +C to result
    AddConstant,

    /// Definite integral (no constant)
    DefiniteIntegral,

    /// User will handle constant explicitly
    UserHandled,
}
```

### 2.2 Extended ElementaryProperties

**File**: `crates/mathhook-core/src/functions/properties.rs`

```rust
/// Elementary function properties (sin, cos, exp, log)
///
/// Performance-optimized layout with hot path data first
/// for cache-friendly access patterns.
#[derive(Debug, Clone)]
pub struct ElementaryProperties {
    /// Most frequently accessed property (hot path data first)
    pub derivative_rule: Option<DerivativeRule>,

    /// ADDED: Antiderivative rule for integration
    /// Placed second for cache locality with derivative_rule
    pub antiderivative_rule: Option<AntiderivativeRule>,

    /// Special values for exact computation
    pub special_values: Vec<SpecialValue>,

    // ... rest of fields unchanged
}
```

**Memory Impact:**
- Current `ElementaryProperties` size: ≤256 bytes (per test on line 611)
- Adding `Option<AntiderivativeRule>`: +8 bytes (pointer to boxed data)
- New estimated size: ~264 bytes (well within acceptable range)

### 2.3 Function Properties Query Methods

**File**: `crates/mathhook-core/src/functions/properties.rs`

```rust
impl FunctionProperties {
    /// Check if function has antiderivative rule
    ///
    /// Hot path method for performance-critical operations
    #[inline(always)]
    pub fn has_antiderivative(&self) -> bool {
        match self {
            FunctionProperties::Elementary(props) => props.antiderivative_rule.is_some(),
            FunctionProperties::Special(props) => props.has_antiderivative,
            FunctionProperties::Polynomial(_props) => true, // All polynomials are integrable
            FunctionProperties::UserDefined(_) => false,
        }
    }

    /// Get antiderivative rule if available
    #[inline(always)]
    pub fn get_antiderivative_rule(&self) -> Option<&AntiderivativeRule> {
        match self {
            FunctionProperties::Elementary(props) => props.antiderivative_rule.as_ref(),
            _ => None,
        }
    }
}
```

### 2.4 Special and Polynomial Properties Extensions

```rust
/// Special function properties (gamma, bessel, zeta, etc.)
#[derive(Debug, Clone)]
pub struct SpecialProperties {
    pub has_derivative: bool,

    /// ADDED: Quick antiderivative check
    pub has_antiderivative: bool,

    /// ADDED: Antiderivative rule (if known)
    pub antiderivative_rule: Option<AntiderivativeRule>,

    // ... rest unchanged
}

/// Polynomial function properties (legendre, hermite, laguerre, etc.)
#[derive(Debug, Clone)]
pub struct PolynomialProperties {
    // ... existing fields

    /// ADDED: Antiderivative rule (for polynomial integration)
    /// All polynomials are integrable, so this is always Some(...)
    pub antiderivative_rule: AntiderivativeRule,
}
```

---

## 3. Integration Point Design

### 3.1 Registry Lookup in Integration Code

**File**: `crates/mathhook-core/src/calculus/integrals/function_integrals.rs`

**Current Implementation (Hardcoded):**
```rust
impl FunctionIntegrals {
    pub fn integrate_simple_function(name: &str, variable: Symbol) -> Expression {
        match name {
            "sin" => /* hardcoded */,
            "cos" => /* hardcoded */,
            // ... 30+ cases
            _ => Expression::integral(/* symbolic */)
        }
    }
}
```

**Proposed Registry-Based Implementation:**
```rust
use crate::functions::intelligence::get_universal_registry;

impl FunctionIntegrals {
    /// Integrate simple functions f(x) using registry-based antiderivatives
    pub fn integrate_simple_function(name: &str, variable: Symbol) -> Expression {
        // STEP 1: Check registry for function intelligence
        let registry = get_universal_registry();

        if let Some(props) = registry.get_properties(name) {
            // STEP 2: Check if function has antiderivative rule
            if let Some(rule) = props.get_antiderivative_rule() {
                // STEP 3: Apply rule based on type
                return Self::apply_antiderivative_rule(rule, name, variable);
            }
        }

        // STEP 4: Fallback to symbolic representation
        Expression::integral(
            Expression::function(name, vec![Expression::symbol(variable.clone())]),
            variable
        )
    }

    /// Apply antiderivative rule to compute integral
    fn apply_antiderivative_rule(
        rule: &AntiderivativeRule,
        function_name: &str,
        variable: Symbol,
    ) -> Expression {
        match &rule.rule_type {
            AntiderivativeRuleType::Simple { antiderivative_fn, coefficient } => {
                // ∫f(x)dx = c * F(x) + C
                Expression::mul(vec![
                    coefficient.clone(),
                    Expression::function(antiderivative_fn, vec![Expression::symbol(variable)])
                ])
            }

            AntiderivativeRuleType::LinearSubstitution { base_antiderivative } => {
                // Handle ∫f(ax)dx = (1/a)F(ax) + C
                // Requires coefficient extraction from argument
                todo!("Implement linear substitution")
            }

            AntiderivativeRuleType::USubstitution { .. } => {
                // Complex pattern matching required
                todo!("Implement u-substitution pattern matching")
            }

            AntiderivativeRuleType::ByParts { .. } => {
                // Delegate to by_parts module
                todo!("Delegate to integration by parts")
            }

            AntiderivativeRuleType::NonElementary => {
                // Cannot integrate in elementary functions
                Expression::integral(
                    Expression::function(function_name, vec![Expression::symbol(variable.clone())]),
                    variable
                )
            }

            _ => {
                // Other rule types
                todo!("Implement remaining rule types")
            }
        }
    }
}
```

### 3.2 Composite Function Integration

**Challenge**: Current code handles `∫f(g(x))dx` with pattern matching

**Solution**: Use registry + pattern matching hybrid
```rust
impl FunctionIntegrals {
    pub fn integrate_composite_function(
        name: &str,
        inner: &Expression,
        variable: Symbol,
    ) -> Expression {
        let registry = get_universal_registry();

        if let Some(props) = registry.get_properties(name) {
            if let Some(rule) = props.get_antiderivative_rule() {
                // Check if inner expression matches rule patterns
                match &rule.rule_type {
                    AntiderivativeRuleType::LinearSubstitution { .. } => {
                        // Check if inner is linear: ax + b
                        if let Some(coeff) = Self::extract_linear_coefficient(inner, variable.clone()) {
                            return Self::apply_linear_substitution(name, &coeff, variable);
                        }
                    }

                    AntiderivativeRuleType::USubstitution { derivative_pattern, .. } => {
                        // Pattern match against known substitution patterns
                        // This is complex and may require pattern matching engine
                        todo!("Pattern matching for u-substitution")
                    }

                    _ => {}
                }
            }
        }

        // Fallback
        Expression::integral(Expression::function(name, vec![inner.clone()]), variable)
    }
}
```

---

## 4. Population of Registry

### 4.1 Trigonometric Functions Example

**File**: `crates/mathhook-core/src/functions/elementary/trigonometric.rs`

```rust
impl TrigonometricIntelligence {
    fn initialize_sin_cos(&mut self) {
        // Sin function with complete mathematical properties
        self.properties.insert(
            "sin".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("cos".to_string()),
                    result_template: "cos(x)".to_string(),
                }),

                // ADDED: Antiderivative rule
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Simple {
                        antiderivative_fn: "cos".to_string(),
                        coefficient: Expression::integer(-1),
                    },
                    result_template: "-cos(x) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),

                // ... rest of properties
            })),
        );

        // Cos function
        self.properties.insert(
            "cos".to_string(),
            FunctionProperties::Elementary(Box::new(ElementaryProperties {
                derivative_rule: Some(DerivativeRule {
                    rule_type: DerivativeRuleType::Simple("-sin".to_string()),
                    result_template: "-sin(x)".to_string(),
                }),

                // ADDED: Antiderivative rule
                antiderivative_rule: Some(AntiderivativeRule {
                    rule_type: AntiderivativeRuleType::Simple {
                        antiderivative_fn: "sin".to_string(),
                        coefficient: Expression::integer(1),
                    },
                    result_template: "sin(x) + C".to_string(),
                    constant_handling: ConstantOfIntegration::AddConstant,
                }),

                // ... rest of properties
            })),
        );
    }
}
```

### 4.2 Exponential and Logarithmic Functions

**File**: `crates/mathhook-core/src/functions/elementary/exponential.rs`

```rust
// Exponential function: ∫e^x dx = e^x + C
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Simple {
        antiderivative_fn: "exp".to_string(),
        coefficient: Expression::integer(1),
    },
    result_template: "exp(x) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
})
```

**File**: `crates/mathhook-core/src/functions/elementary/logarithmic.rs`

```rust
// Natural log: ∫ln(x) dx = x*ln(x) - x + C (by parts)
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::ByParts {
        u_pattern: "ln(x)".to_string(),
        dv_pattern: "1".to_string(),
    },
    result_template: "x*ln(x) - x + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
})
```

### 4.3 Inverse Trigonometric Functions

**File**: `crates/mathhook-core/src/functions/elementary/trigonometric.rs`

```rust
// arcsin: ∫arcsin(x) dx = x*arcsin(x) + √(1-x²) + C
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::ByParts {
        u_pattern: "arcsin(x)".to_string(),
        dv_pattern: "1".to_string(),
    },
    result_template: "x*arcsin(x) + √(1-x²) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
})

// 1/√(1-x²) → arcsin(x) (derivative relationship)
// This would be a separate function entry for "1/√(1-x²)" pattern
```

---

## 5. Architecture Diagram (Text-Based)

```
┌─────────────────────────────────────────────────────────────────────┐
│                     UniversalFunctionRegistry                        │
│  ┌────────────────────────────────────────────────────────────────┐ │
│  │  properties: HashMap<String, FunctionProperties>                │ │
│  │  ┌──────────────────────────────────────────────────────────┐  │ │
│  │  │  FunctionProperties::Elementary(ElementaryProperties)     │  │ │
│  │  │  ┌────────────────────────────────────────────────────┐  │  │ │
│  │  │  │  derivative_rule: Option<DerivativeRule>            │  │  │ │
│  │  │  │  antiderivative_rule: Option<AntiderivativeRule>    │  │  │ │
│  │  │  │  special_values: Vec<SpecialValue>                  │  │  │ │
│  │  │  │  identities: Box<Vec<MathIdentity>>                 │  │  │ │
│  │  │  │  domain_range: Box<DomainRangeData>                 │  │  │ │
│  │  │  │  periodicity: Option<Expression>                    │  │  │ │
│  │  │  └────────────────────────────────────────────────────┘  │  │ │
│  │  └──────────────────────────────────────────────────────────┘  │ │
│  └────────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ O(1) lookup
                                    ▼
         ┌──────────────────────────────────────────────────┐
         │   calculus/derivatives/chain_rule.rs             │
         │   ┌──────────────────────────────────────────┐   │
         │   │  FunctionDerivatives::get(name, arg)     │   │
         │   │  → registry.get_properties(name)          │   │
         │   │  → props.derivative_rule                  │   │
         │   └──────────────────────────────────────────┘   │
         └──────────────────────────────────────────────────┘
                                    │
                                    │ Parallel
                                    ▼
         ┌──────────────────────────────────────────────────┐
         │   calculus/integrals/function_integrals.rs       │
         │   ┌──────────────────────────────────────────┐   │
         │   │  FunctionIntegrals::integrate(name, arg) │   │
         │   │  → registry.get_properties(name)          │   │
         │   │  → props.antiderivative_rule              │   │
         │   └──────────────────────────────────────────┘   │
         └──────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│                      AntiderivativeRule Types                        │
├─────────────────────────────────────────────────────────────────────┤
│  Simple                      │  ∫sin(x)dx = -cos(x) + C             │
│  LinearSubstitution          │  ∫sin(3x)dx = -(1/3)cos(3x) + C     │
│  USubstitution               │  ∫f(g(x))g'(x)dx = F(g(x)) + C      │
│  ByParts                     │  ∫x*ln(x)dx = ...                    │
│  TrigSubstitution            │  ∫1/√(1-x²)dx = arcsin(x) + C       │
│  PartialFractions            │  ∫(2x+3)/(x²+1)dx = ...             │
│  ReductionFormula            │  ∫sin^n(x)dx in terms of sin^(n-2)  │
│  SpecialFunction             │  ∫e^(-x²)dx = √π/2 erf(x) + C       │
│  NonElementary               │  ∫e^(x²)dx (symbolic only)           │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 6. Files That Need Modification

### 6.1 Core Type Definitions
- **`crates/mathhook-core/src/functions/properties.rs`**
  - Add `AntiderivativeRule` struct
  - Add `AntiderivativeRuleType` enum
  - Add `ConstantOfIntegration` enum
  - Extend `ElementaryProperties` with `antiderivative_rule` field
  - Extend `SpecialProperties` with `has_antiderivative` and `antiderivative_rule`
  - Extend `PolynomialProperties` with `antiderivative_rule`
  - Add `has_antiderivative()` and `get_antiderivative_rule()` methods

### 6.2 Function Intelligence Modules
- **`crates/mathhook-core/src/functions/elementary/trigonometric.rs`**
  - Add antiderivative rules for: sin, cos, tan, sec, csc, cot
  - Add antiderivative rules for inverse trig functions

- **`crates/mathhook-core/src/functions/elementary/exponential.rs`**
  - Add antiderivative rule for exp

- **`crates/mathhook-core/src/functions/elementary/logarithmic.rs`**
  - Add antiderivative rule for ln (by parts)

- **`crates/mathhook-core/src/functions/elementary/hyperbolic.rs`**
  - Add antiderivative rules for: sinh, cosh, tanh, sech, csch, coth

### 6.3 Integration Code
- **`crates/mathhook-core/src/calculus/integrals/function_integrals.rs`**
  - Refactor `integrate_simple_function()` to use registry
  - Refactor `integrate_composite_function()` to use registry
  - Add `apply_antiderivative_rule()` method
  - Add pattern matching helpers for substitution

### 6.4 Tests
- **`crates/mathhook-core/src/functions/properties.rs`** (tests module)
  - Add test for `has_antiderivative()`
  - Add test for memory size constraints

- **`crates/mathhook-core/tests/integral_registry_tests.rs`** (new file)
  - Test registry-based integration for all elementary functions
  - Test composite function integration
  - Test edge cases and fallbacks

### 6.5 Documentation
- **`CLAUDE.md`**
  - Document the antiderivative registry system
  - Add examples of how to add new function integrals

---

## 7. Migration Strategy

### Phase 1: Foundation (Week 1)
1. **Define core types** in `properties.rs`
   - `AntiderivativeRule`
   - `AntiderivativeRuleType`
   - `ConstantOfIntegration`

2. **Extend existing types**
   - Add `antiderivative_rule` to `ElementaryProperties`
   - Add `has_antiderivative()` method to `FunctionProperties`

3. **Write comprehensive tests**
   - Test type definitions
   - Test memory size constraints
   - Test query methods

### Phase 2: Population (Week 2)
1. **Populate elementary functions** with antiderivative rules
   - Trigonometric functions (sin, cos, tan, etc.)
   - Exponential and logarithmic functions
   - Hyperbolic functions
   - Inverse trigonometric functions

2. **Test each function family**
   - Verify rules are stored correctly
   - Verify O(1) lookup performance

### Phase 3: Integration (Week 3)
1. **Refactor `function_integrals.rs`**
   - Replace hardcoded match with registry lookup
   - Implement `apply_antiderivative_rule()`
   - Handle simple cases first

2. **Test integration**
   - Compare results with old hardcoded implementation
   - Ensure no regressions

### Phase 4: Advanced Features (Week 4)
1. **Implement complex rule types**
   - Linear substitution
   - U-substitution pattern matching
   - By parts (delegate to existing module)

2. **Composite function integration**
   - Pattern matching for substitution
   - Coefficient extraction

3. **Comprehensive testing**
   - Test all rule types
   - Test edge cases
   - Performance benchmarks

### Phase 5: Cleanup (Week 5)
1. **Remove old hardcoded implementations**
   - Remove old match statements from `function_integrals.rs`
   - Ensure all tests still pass

2. **Documentation**
   - Update CLAUDE.md
   - Add inline documentation
   - Add examples

---

## 8. Potential Risks and Challenges

### 8.1 Technical Challenges

**Challenge 1: Pattern Matching Complexity**
- **Issue**: U-substitution and other techniques require sophisticated pattern matching
- **Risk Level**: HIGH
- **Mitigation**:
  - Start with simple cases (direct antiderivatives)
  - Add pattern matching incrementally
  - Consider separate pattern matching engine

**Challenge 2: Expression Template Evaluation**
- **Issue**: `result_template` is a string, not an executable expression
- **Risk Level**: MEDIUM
- **Mitigation**:
  - Store templates as `Expression` types instead of strings
  - Use closures: `Fn(&Expression, Symbol) -> Expression`
  - Trade-off: Increased memory usage vs flexibility

**Challenge 3: Composite Function Integration**
- **Issue**: Detecting when to apply chain rule in reverse
- **Risk Level**: HIGH
- **Mitigation**:
  - Heuristic-based detection (check if derivative is present)
  - Table of common patterns
  - Fallback to symbolic representation

**Challenge 4: By Parts and Advanced Techniques**
- **Issue**: By parts requires strategic choices of u and dv
- **Risk Level**: MEDIUM
- **Mitigation**:
  - Store heuristics in registry
  - Use existing `by_parts.rs` module
  - Registry provides metadata, module provides algorithm

### 8.2 Performance Risks

**Risk 1: Memory Bloat**
- **Issue**: Adding antiderivative rules to all functions increases memory
- **Current Size**: `ElementaryProperties` ≤256 bytes
- **Estimated Increase**: +8 bytes (pointer) + rule data (heap)
- **Mitigation**:
  - Box complex rules
  - Lazy initialization for rarely-used rules
  - Benchmark memory usage

**Risk 2: Registry Lookup Overhead**
- **Issue**: O(1) HashMap lookup is fast but not free
- **Mitigation**:
  - Cache frequently-used rules
  - Inline hot path lookups
  - Benchmark against current hardcoded implementation

### 8.3 Correctness Risks

**Risk 1: Mathematical Errors in Rules**
- **Issue**: Incorrect antiderivative rules cause wrong results
- **Mitigation**:
  - Validate all rules against SymPy
  - Extensive test suite
  - Derivative-integral round-trip tests (∫(d/dx f) = f + C)

**Risk 2: Domain Restrictions**
- **Issue**: Some antiderivatives have domain restrictions (e.g., ln|x| vs ln(x))
- **Mitigation**:
  - Include domain information in `AntiderivativeRule`
  - Use existing `DomainRangeData` system
  - Document restrictions clearly

---

## 9. Success Metrics

### 9.1 Functional Metrics
- [ ] All 30+ hardcoded integral rules migrated to registry
- [ ] Zero regressions in existing integral tests
- [ ] New functions can be added by defining rules only (no code changes)

### 9.2 Performance Metrics
- [ ] Registry lookup: <100ns per lookup (similar to derivative lookup)
- [ ] Integration speed: within 5% of current hardcoded implementation
- [ ] Memory overhead: <10% increase in `ElementaryProperties` size

### 9.3 Code Quality Metrics
- [ ] `function_integrals.rs`: Reduce from 350+ lines to <150 lines
- [ ] Eliminate all hardcoded match statements for function integrals
- [ ] 100% test coverage for registry-based integration

---

## 10. Open Questions

### Q1: Template Representation
**Question**: Should `result_template` be a `String` or `Expression`?

**Options:**
- **String**: Simpler, for documentation only
- **Expression**: Executable, but harder to parameterize
- **Closure**: `Box<dyn Fn(&Expression, Symbol) -> Expression>` - most flexible

**Recommendation**: Start with `String` for documentation, add closure field for execution:
```rust
pub struct AntiderivativeRule {
    pub rule_type: AntiderivativeRuleType,
    pub result_template: String,  // Documentation
    pub evaluator: Option<Box<dyn Fn(&Expression, Symbol) -> Expression>>, // Execution
}
```

### Q2: How to Handle Complex Patterns?
**Question**: How to detect patterns like ∫f(g(x))g'(x)dx?

**Options:**
1. **Pattern matching engine**: Build a general expression pattern matcher
2. **Heuristic detection**: Check if derivative of inner function appears
3. **Explicit registration**: User must explicitly mark substitution candidates

**Recommendation**: Start with heuristic detection, evolve to pattern engine if needed.

### Q3: Definite vs Indefinite Integrals
**Question**: Should the registry distinguish between definite and indefinite integrals?

**Recommendation**:
- Registry stores indefinite integral rules (with +C)
- Definite integral module uses registry rules + boundary evaluation
- `ConstantOfIntegration` enum handles this distinction

### Q4: Multi-Variable Functions
**Question**: How to handle ∫f(x,y)dx (partial integration)?

**Recommendation**:
- Treat as single-variable problem (integrate with respect to specified variable)
- Other variables are constants
- Registry rules are variable-agnostic

---

## 11. Future Enhancements

### 11.1 Pattern Matching Engine
- General expression pattern matching (beyond simple substitution)
- Used for u-substitution, trig substitution, partial fractions
- Similar to SymPy's pattern matching system

### 11.2 Integral Table Database
- Comprehensive table of common integrals
- Searchable by pattern
- User-extensible

### 11.3 Symbolic Integration Algorithm
- Risch algorithm for symbolic integration
- Fallback when registry rules don't apply
- Research-level complexity

### 11.4 Numerical Integration Integration
- Registry also stores numerical integration hints
- Preferred quadrature method for each function
- Error bounds and convergence properties

---

## 12. Conclusion

This design extends the proven `UniversalFunctionRegistry` architecture to support antiderivatives in a parallel fashion to derivatives. The key advantages are:

1. **Consistency**: Mirrors the existing derivative system
2. **Maintainability**: Centralized integral knowledge
3. **Extensibility**: New functions require only registry updates
4. **Performance**: O(1) lookup with minimal overhead

The main challenges are pattern matching complexity and ensuring mathematical correctness. A phased migration approach minimizes risk and allows for incremental validation.

**Recommendation**: Proceed with implementation in 5 phases over 5 weeks, starting with foundation types and simple cases, gradually adding complexity.

---

## Appendix A: Complete Example

### Example: Adding √x Integral to Registry

**Step 1: Define the rule in `elementary/power_functions.rs` (hypothetical)**
```rust
// Square root: ∫√x dx = (2/3)x^(3/2) + C
antiderivative_rule: Some(AntiderivativeRule {
    rule_type: AntiderivativeRuleType::Simple {
        antiderivative_fn: "power".to_string(),  // Or directly construct Expression
        coefficient: Expression::rational(2, 3),
    },
    result_template: "(2/3)x^(3/2) + C".to_string(),
    constant_handling: ConstantOfIntegration::AddConstant,
})
```

**Step 2: Registry automatically includes it**
```rust
// No code changes needed - registry initialization handles it
```

**Step 3: Integration code automatically uses it**
```rust
let x = symbol!(x);
let expr = Expression::function("sqrt", vec![Expression::symbol(x.clone())]);
let integral = FunctionIntegrals::integrate("sqrt", &[x], x);
// Returns: (2/3) * x^(3/2)
```

**Step 4: Validation**
```rust
// Derivative-integral round-trip test
let derivative = integral.derivative(x);
let simplified = derivative.simplify();
assert_eq!(simplified, expr);  // Validates correctness
```

---

**End of Design Document**
