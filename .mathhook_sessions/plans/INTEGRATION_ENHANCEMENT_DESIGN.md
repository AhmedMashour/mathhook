# Integration Enhancement Design: Waves 2-6 Implementation Roadmap

## Executive Summary

This document provides the comprehensive implementation roadmap for enhancing MathHook's symbolic integration from 75 percent to 93-95 percent coverage across Waves 2-6. The design focuses on layered strategy dispatch, modular technique implementation, and maintainability. Each wave adds specific capabilities that compose into a complete integration system rivaling SymPy while leveraging Rust's performance advantages.

## Overall Architecture

### Layered Strategy Dispatcher

The core architectural pattern is a fallthrough strategy dispatcher where each layer handles a subset of integrals:

```
∫f(x) dx → Layer 1: Table Lookup (O(1), 20-30% coverage)
         → Layer 2: Rational Functions (O(n³), 15-20% coverage)
         → Layer 3: Registry Functions (O(1), 20-30% coverage, existing)
         → Layer 4: By Parts (O(n), 5-10% coverage, existing)
         → Layer 5: Substitution (O(n²), 10-15% coverage)
         → Layer 6: Trigonometric (O(n), 5-8% coverage)
         → Layer 7: Risch Algorithm (O(n⁴), 3-5% coverage)
         → Layer 8: Symbolic (unable to integrate, ~5%)
```

**Design Principle**: Fast techniques first (milliseconds), slow techniques as fallback (seconds).

### Module Structure

```
crates/mathhook-core/src/calculus/integrals/
├── mod.rs                   # Integration trait + dispatcher
├── strategy.rs              # Layer orchestration (NEW - Wave 2)
├── basic.rs                 # Power, constant, sum rules (EXISTING)
├── by_parts.rs              # Integration by parts (EXISTING)
├── function_integrals.rs    # Registry-based (EXISTING)
├── educational.rs           # Step-by-step explanations (EXISTING)
├── table.rs                 # Pattern-based lookup (NEW - Wave 3)
├── substitution.rs          # U-substitution (NEW - Wave 3)
├── rational.rs              # Partial fractions (NEW - Wave 2)
├── trigonometric.rs         # Sin^m·cos^n patterns (NEW - Wave 4)
├── risch/                   # Risch algorithm (NEW - Wave 5)
│   ├── mod.rs
│   ├── tower.rs
│   ├── rde.rs
│   ├── hermite.rs
│   ├── logarithmic_part.rs
│   └── back_substitution.rs
└── definite.rs              # Definite integrals (NEW - Wave 6)
```

## Wave 2: Rational Functions + Strategy Dispatcher

### Goal
Implement partial fraction decomposition for rational functions P(x)/Q(x) and create the strategy dispatcher infrastructure. Target: 75% → 85% coverage.

### Module 1: `rational.rs`

**Data Structures**:

```rust
/// Represents a partial fraction decomposition
pub struct PartialFractionDecomposition {
    /// Polynomial part (from improper fraction division)
    polynomial_part: Polynomial,

    /// Linear factor terms: A/(x-α)^k
    linear_terms: Vec<LinearTerm>,

    /// Irreducible quadratic terms: (Bx+C)/(x²+px+q)^k
    quadratic_terms: Vec<QuadraticTerm>,
}

#[derive(Debug, Clone)]
pub struct LinearTerm {
    coefficient: Expression,
    root: Expression,          // α in (x-α)
    multiplicity: usize,       // k in (x-α)^k
}

#[derive(Debug, Clone)]
pub struct QuadraticTerm {
    numerator_coeff_x: Expression,   // B in (Bx+C)
    numerator_const: Expression,      // C in (Bx+C)
    quadratic: Expression,            // x²+px+q
    multiplicity: usize,              // k in (x²+px+q)^k
}
```

**Core Algorithm**:

```rust
impl RationalIntegrals {
    /// Integrate rational function P(x)/Q(x)
    pub fn integrate(numerator: &Expression, denominator: &Expression, variable: Symbol)
        -> Expression
    {
        // Step 1: Check if proper fraction (deg(P) < deg(Q))
        let (poly_part, remainder) = if Self::is_proper(numerator, denominator, &variable) {
            (Expression::integer(0), numerator.clone())
        } else {
            // Polynomial division for improper fractions
            Self::polynomial_division(numerator, denominator, &variable)
        };

        // Step 2: Partial fraction decomposition of remainder/denominator
        let decomp = Self::partial_fraction_decompose(&remainder, denominator, &variable)?;

        // Step 3: Integrate polynomial part (trivial)
        let poly_integral = poly_part.integrate(variable.clone());

        // Step 4: Integrate each partial fraction
        let mut result = poly_integral;
        for term in decomp.linear_terms {
            result = result + Self::integrate_linear_term(&term, variable.clone());
        }
        for term in decomp.quadratic_terms {
            result = result + Self::integrate_quadratic_term(&term, variable.clone());
        }

        result.simplify()
    }

    /// Partial fraction decomposition using Heaviside cover-up method
    fn partial_fraction_decompose(p: &Expression, q: &Expression, var: &Symbol)
        -> Result<PartialFractionDecomposition, Error>
    {
        // Step 1: Factor denominator Q(x) = ∏(x-αᵢ)^mᵢ · ∏Qⱼ(x)^nⱼ
        let factored = Self::factor_denominator(q, var)?;

        // Step 2: Separate linear and quadratic factors
        let (linear_factors, quadratic_factors) = Self::classify_factors(&factored);

        // Step 3: Compute coefficients for each term
        let linear_terms = Self::compute_linear_coefficients(p, &linear_factors, var);
        let quadratic_terms = Self::compute_quadratic_coefficients(p, &quadratic_factors, var);

        Ok(PartialFractionDecomposition {
            polynomial_part: Polynomial::zero(),
            linear_terms,
            quadratic_terms,
        })
    }

    /// Integrate A/(x-α)^k
    fn integrate_linear_term(term: &LinearTerm, variable: Symbol) -> Expression {
        if term.multiplicity == 1 {
            // ∫A/(x-α) dx = A·ln|x-α|
            Expression::mul(vec![
                term.coefficient.clone(),
                Expression::function("ln", vec![
                    Expression::function("abs", vec![
                        Expression::add(vec![
                            Expression::symbol(variable),
                            Expression::mul(vec![
                                Expression::integer(-1),
                                term.root.clone()
                            ])
                        ])
                    ])
                ])
            ])
        } else {
            // ∫A/(x-α)^k dx = -A/((k-1)(x-α)^(k-1)) for k > 1
            let k = term.multiplicity as i64;
            Expression::mul(vec![
                Expression::integer(-1),
                term.coefficient.clone(),
                Expression::pow(
                    Expression::integer(k - 1),
                    Expression::integer(-1)
                ),
                Expression::pow(
                    Expression::add(vec![
                        Expression::symbol(variable),
                        Expression::mul(vec![
                            Expression::integer(-1),
                            term.root.clone()
                        ])
                    ]),
                    Expression::integer(-(k - 1))
                )
            ])
        }
    }

    /// Integrate (Bx+C)/(x²+px+q)^k
    fn integrate_quadratic_term(term: &QuadraticTerm, variable: Symbol) -> Expression {
        if term.multiplicity == 1 {
            // Split: (Bx+C)/(x²+px+q) = B/2 · 2x/(x²+px+q) + C/(x²+px+q)
            // First part: (B/2)·ln(x²+px+q)
            // Second part: C·arctan-based formula (completing the square)
            Self::integrate_simple_quadratic(term, variable)
        } else {
            // Use reduction formula for k > 1
            Self::integrate_repeated_quadratic(term, variable)
        }
    }
}
```

**Dependencies**:
- Polynomial division (recently implemented, verify compatibility)
- GCD computation (recently implemented, for factorization)
- Polynomial factorization (need to verify availability or implement)

**Tests**: 35 tests covering proper/improper fractions, linear factors, repeated factors, quadratic factors.

### Module 2: `strategy.rs`

**Purpose**: Orchestrate all integration techniques in optimal order.

**Data Structures**:

```rust
/// Strategy result with metadata
pub enum StrategyResult {
    Success {
        result: Expression,
        technique: TechniqueUsed,
        time_ms: u64,
    },
    Failed {
        reason: String,
    },
}

#[derive(Debug, Clone)]
pub enum TechniqueUsed {
    TableLookup,
    Rational,
    Registry,
    ByParts,
    Substitution,
    Trigonometric,
    Risch,
    Symbolic,
}

/// Strategy dispatcher configuration
pub struct IntegrationStrategy {
    /// Enable/disable each layer
    config: StrategyConfig,

    /// Performance tracking
    stats: StrategyStats,
}

pub struct StrategyConfig {
    enable_table: bool,         // Default: true
    enable_rational: bool,      // Default: true
    enable_registry: bool,      // Default: true
    enable_by_parts: bool,      // Default: true
    enable_substitution: bool,  // Default: true (Wave 3+)
    enable_trigonometric: bool, // Default: true (Wave 4+)
    enable_risch: bool,         // Default: true (Wave 5+)
    risch_timeout_ms: u64,      // Default: 10000 (10 seconds)
}
```

**Core Algorithm**:

```rust
impl IntegrationStrategy {
    pub fn integrate(&mut self, expr: &Expression, variable: Symbol) -> StrategyResult {
        let start = Instant::now();

        // Layer 1: Table lookup (O(1))
        if self.config.enable_table {
            if let Some(result) = table::try_lookup(expr, &variable) {
                return StrategyResult::Success {
                    result,
                    technique: TechniqueUsed::TableLookup,
                    time_ms: start.elapsed().as_millis() as u64,
                };
            }
        }

        // Layer 2: Rational functions (specialized algorithm)
        if self.config.enable_rational && Self::is_rational_function(expr, &variable) {
            match rational::integrate(expr, &variable) {
                Ok(result) => {
                    return StrategyResult::Success {
                        result,
                        technique: TechniqueUsed::Rational,
                        time_ms: start.elapsed().as_millis() as u64,
                    };
                }
                Err(_) => { /* Continue to next layer */ }
            }
        }

        // Layer 3: Registry-based function integration (existing)
        if self.config.enable_registry {
            if let Some(result) = function_integrals::try_integrate(expr, &variable) {
                return StrategyResult::Success {
                    result,
                    technique: TechniqueUsed::Registry,
                    time_ms: start.elapsed().as_millis() as u64,
                };
            }
        }

        // Layer 4: Integration by parts (existing)
        if self.config.enable_by_parts {
            if let Some(result) = by_parts::IntegrationByParts::integrate(expr, variable.clone()) {
                return StrategyResult::Success {
                    result,
                    technique: TechniqueUsed::ByParts,
                    time_ms: start.elapsed().as_millis() as u64,
                };
            }
        }

        // Layer 5: U-substitution (Wave 3+)
        if self.config.enable_substitution {
            if let Some(result) = substitution::try_substitution(expr, &variable) {
                return StrategyResult::Success {
                    result,
                    technique: TechniqueUsed::Substitution,
                    time_ms: start.elapsed().as_millis() as u64,
                };
            }
        }

        // Layer 6: Trigonometric patterns (Wave 4+)
        if self.config.enable_trigonometric && Self::is_trigonometric_pattern(expr, &variable) {
            if let Some(result) = trigonometric::integrate(expr, &variable) {
                return StrategyResult::Success {
                    result,
                    technique: TechniqueUsed::Trigonometric,
                    time_ms: start.elapsed().as_millis() as u64,
                };
            }
        }

        // Layer 7: Risch algorithm (Wave 5+, SLOW)
        if self.config.enable_risch {
            match risch::integrate_with_timeout(expr, &variable, self.config.risch_timeout_ms) {
                Ok(risch::IntegrationResult::Elementary(result)) => {
                    return StrategyResult::Success {
                        result,
                        technique: TechniqueUsed::Risch,
                        time_ms: start.elapsed().as_millis() as u64,
                    };
                }
                Ok(risch::IntegrationResult::NonElementary { .. }) => {
                    // Fall through to symbolic
                }
                Err(_) => {
                    // Timeout or error, fall through
                }
            }
        }

        // Layer 8: Symbolic (fallback)
        StrategyResult::Success {
            result: Expression::integral(expr.clone(), variable),
            technique: TechniqueUsed::Symbolic,
            time_ms: start.elapsed().as_millis() as u64,
        }
    }
}
```

**Tests**: 10 tests verifying strategy ordering, fallback behavior, performance tracking.

### Expected Outcome (Wave 2)

- **Coverage**: 75% → 85% (rational functions add 10-15%, strategy dispatcher optimizes)
- **Performance**: Fast path < 10ms for 85% of integrals
- **Tests**: 45 new tests (35 rational + 10 strategy)

## Wave 3: Integration Table + Enhanced Substitution

### Goal
Implement pattern-based integration table for O(1) lookups and general u-substitution. Target: 85% → 90% coverage.

### Module 1: `table.rs`

**Purpose**: Fast O(1) lookup for 50+ common integration patterns.

**Data Structures**:

```rust
/// Integration pattern with match and result templates
pub struct IntegrationPattern {
    /// Pattern to match against
    pattern: PatternMatcher,

    /// Result template (may contain placeholders)
    result_template: Expression,

    /// Domain restrictions
    domain: Option<DomainRestriction>,

    /// Educational description
    description: String,
}

pub enum PatternMatcher {
    /// Exact match: f(x) must equal pattern exactly
    Exact(Expression),

    /// Structural match: f(x) matches pattern with wildcards
    Structural {
        template: Expression,
        wildcards: HashMap<String, WildcardConstraint>,
    },

    /// Functional match: Custom matching function
    Functional(Box<dyn Fn(&Expression, &Symbol) -> Option<HashMap<String, Expression>>>),
}

pub enum WildcardConstraint {
    Any,                           // Matches anything
    Constant(Symbol),              // Constant with respect to variable
    Integer,                       // Must be integer
    Positive,                      // Must be positive
    Function(String),              // Must be specific function
}
```

**Table Contents** (50+ patterns):

```rust
impl IntegrationTable {
    pub fn build() -> Self {
        let mut table = IntegrationTable::new();

        // Power functions
        table.add_pattern(PatternMatcher::power_rule());          // ∫x^n dx
        table.add_pattern(PatternMatcher::reciprocal());          // ∫1/x dx

        // Trigonometric
        table.add_pattern(PatternMatcher::tan_pattern());         // ∫tan(x) dx
        table.add_pattern(PatternMatcher::sec_pattern());         // ∫sec(x) dx
        table.add_pattern(PatternMatcher::csc_pattern());         // ∫csc(x) dx

        // Inverse trigonometric forms
        table.add_pattern(PatternMatcher::arcsin_form());         // ∫1/√(a²-x²) dx
        table.add_pattern(PatternMatcher::arctan_form());         // ∫1/(a²+x²) dx
        table.add_pattern(PatternMatcher::arcsec_form());         // ∫1/(x√(x²-a²)) dx

        // Exponential and logarithmic
        table.add_pattern(PatternMatcher::exp_linear());          // ∫e^(ax) dx
        table.add_pattern(PatternMatcher::exp_quadratic());       // ∫x·e^(ax) dx
        table.add_pattern(PatternMatcher::ln_pattern());          // ∫ln(x) dx

        // Rational forms
        table.add_pattern(PatternMatcher::rational_linear_denom());    // ∫1/(ax+b) dx
        table.add_pattern(PatternMatcher::rational_quadratic());       // ∫1/(ax²+bx+c) dx

        // Hyperbolic
        table.add_pattern(PatternMatcher::tanh_pattern());        // ∫tanh(x) dx
        table.add_pattern(PatternMatcher::sech_pattern());        // ∫sech²(x) dx

        // Special combinations
        table.add_pattern(PatternMatcher::sqrt_linear());         // ∫√(ax+b) dx
        table.add_pattern(PatternMatcher::sqrt_quadratic());      // ∫√(a²-x²) dx

        // ... 30+ more patterns

        table
    }

    pub fn lookup(&self, expr: &Expression, variable: &Symbol) -> Option<Expression> {
        for pattern in &self.patterns {
            if let Some(substitutions) = pattern.matches(expr, variable) {
                return Some(pattern.result_template.substitute_all(&substitutions));
            }
        }
        None
    }
}
```

**Performance**: O(n) where n = number of patterns (~50), but each pattern match is O(1) structural comparison.

**Tests**: 30 tests covering all table patterns.

### Module 2: Enhanced `substitution.rs`

**Purpose**: Automatic u-substitution detection and application.

**Core Algorithm**:

```rust
impl SubstitutionIntegration {
    /// Detect and apply u-substitution
    pub fn try_substitution(expr: &Expression, variable: &Symbol) -> Option<Expression> {
        // Find candidate substitutions
        let candidates = Self::find_substitution_candidates(expr, variable);

        for candidate in candidates {
            // Check if derivative appears in integrand
            if Self::derivative_appears_in_integrand(expr, &candidate, variable) {
                // Apply substitution
                if let Some(result) = Self::apply_substitution(expr, &candidate, variable) {
                    return Some(result);
                }
            }
        }

        None
    }

    /// Find inner functions that might be good u candidates
    fn find_substitution_candidates(expr: &Expression, variable: &Symbol) -> Vec<Expression> {
        let mut candidates = Vec::new();

        // Visit expression tree
        Self::visit_expression(expr, &mut |subexpr| {
            match subexpr {
                // Functions: sin(g(x)), ln(g(x)), etc.
                Expression::Function { args, .. } if args.len() == 1 => {
                    let inner = &args[0];
                    if Self::contains_variable(inner, variable) && !Self::is_trivial(inner, variable) {
                        candidates.push(inner.clone());
                    }
                }

                // Powers: (g(x))^n
                Expression::Pow(base, _) if Self::contains_variable(base, variable) => {
                    candidates.push((**base).clone());
                }

                // Products: Look for g(x)·g'(x) patterns
                Expression::Mul(factors) => {
                    for factor in factors {
                        if Self::contains_variable(factor, variable) {
                            candidates.push(factor.clone());
                        }
                    }
                }

                _ => {}
            }
        });

        // Score and sort candidates by likelihood of success
        candidates.sort_by_key(|c| Self::score_substitution_candidate(c, expr, variable));
        candidates
    }

    /// Check if g'(x) appears in integrand (possibly with constant factor)
    fn derivative_appears_in_integrand(expr: &Expression, candidate: &Expression, variable: &Symbol) -> bool {
        let derivative = candidate.derivative(variable.clone());

        // Check for exact match: f(g)·g'
        if Self::contains_factor(expr, &derivative) {
            return true;
        }

        // Check for constant multiple: f(g)·c·g' where c is constant
        if let Some(_constant) = Self::find_constant_multiple(expr, &derivative, variable) {
            return true;
        }

        false
    }

    /// Apply u-substitution: ∫f(g(x))·g'(x) dx = ∫f(u) du where u = g(x)
    fn apply_substitution(expr: &Expression, u_expr: &Expression, variable: &Symbol)
        -> Option<Expression>
    {
        let u = Symbol::new("u");
        let u_derivative = u_expr.derivative(variable.clone());

        // Replace g(x) with u and g'(x)·dx with du
        let integrand_in_u = Self::substitute_and_simplify(expr, u_expr, &u, &u_derivative, variable)?;

        // Integrate with respect to u
        let result_in_u = integrand_in_u.integrate(u.clone());

        // Back-substitute u = g(x)
        let result = result_in_u.substitute(&u, u_expr).simplify();

        Some(result)
    }
}
```

**Tests**: 30 tests covering chain rule patterns, composite functions, various substitutions.

### Expected Outcome (Wave 3)

- **Coverage**: 85% → 90% (table adds 20-30% fast path, substitution adds 10-15%)
- **Performance**: Fast path < 1ms for 60% (table), < 10ms for 90%
- **Tests**: 60 new tests (30 table + 30 substitution)

## Wave 4: Trigonometric Integrals

### Goal
Implement specialized handler for sin^m(x)·cos^n(x) patterns. Target: 90% → 92% coverage.

### Module: `trigonometric.rs`

**Data Structures**:

```rust
pub struct TrigonometricPattern {
    /// Power of sin
    sin_power: Expression,

    /// Power of cos
    cos_power: Expression,

    /// Coefficient (a in sin(ax), cos(ax))
    coefficient: Expression,
}

impl TrigonometricIntegration {
    pub fn integrate(expr: &Expression, variable: &Symbol) -> Option<Expression> {
        // Detect sin^m(ax)·cos^n(ax) pattern
        let pattern = Self::match_pattern(expr, variable)?;

        // Determine integration strategy based on powers
        if pattern.is_odd_sin_power() {
            Self::integrate_odd_sin(&pattern, variable)
        } else if pattern.is_odd_cos_power() {
            Self::integrate_odd_cos(&pattern, variable)
        } else if pattern.is_even_powers() {
            Self::integrate_even_powers(&pattern, variable)
        } else {
            None
        }
    }

    /// Case 1: Odd sin power - substitute u = cos(ax)
    fn integrate_odd_sin(pattern: &TrigonometricPattern, variable: &Symbol) -> Option<Expression> {
        // ∫sin^(2k+1)(ax)·cos^n(ax) dx
        // = ∫sin^(2k)(ax)·cos^n(ax)·sin(ax) dx
        // = ∫(1-cos²(ax))^k·cos^n(ax)·sin(ax) dx
        // Let u = cos(ax), du = -a·sin(ax) dx
        // = -(1/a)∫(1-u²)^k·u^n du
        // ...
    }

    /// Case 2: Odd cos power - substitute u = sin(ax)
    fn integrate_odd_cos(pattern: &TrigonometricPattern, variable: &Symbol) -> Option<Expression> {
        // Similar to odd sin, but u = sin(ax)
        // ...
    }

    /// Case 3: Both even powers - use half-angle formulas
    fn integrate_even_powers(pattern: &TrigonometricPattern, variable: &Symbol) -> Option<Expression> {
        // Use: sin²(x) = (1 - cos(2x))/2
        //      cos²(x) = (1 + cos(2x))/2
        // Recursively expand and integrate
        // ...
    }
}
```

**Reduction Formulas**:

```rust
impl TrigonometricIntegration {
    /// Reduction formula for ∫sin^n(x) dx
    fn sin_power_reduction(n: i64, variable: &Symbol) -> Expression {
        if n == 0 {
            Expression::symbol(variable.clone())
        } else if n == 1 {
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("cos", vec![Expression::symbol(variable.clone())])
            ])
        } else {
            // ∫sin^n(x) dx = -sin^(n-1)(x)·cos(x)/n + (n-1)/n·∫sin^(n-2)(x) dx
            // ...
        }
    }

    /// Reduction formula for ∫cos^n(x) dx
    fn cos_power_reduction(n: i64, variable: &Symbol) -> Expression {
        // Similar to sin reduction
        // ...
    }
}
```

**Tests**: 35 tests covering odd powers, even powers, mixed products, reduction formulas.

### Expected Outcome (Wave 4)

- **Coverage**: 90% → 92% (trigonometric patterns add 2-3%)
- **Performance**: Fast path < 10ms for trigonometric integrals
- **Tests**: 35 new tests

## Wave 5: Risch Algorithm

(See `RISCH_ALGORITHM_DESIGN.md` for complete design)

**Summary**:
- Exponential extensions
- Logarithmic extensions
- Mixed towers
- Non-elementary detection

**Expected Outcome**: 92% → 95% coverage, tests: 30 new tests

## Wave 6: Testing, Documentation, Definite Integrals

### Goal
Validate 93-95% coverage, optimize performance, implement definite integrals. Final polish.

### Module: `definite.rs`

**Purpose**: Evaluate definite integrals using Fundamental Theorem of Calculus.

```rust
pub struct DefiniteIntegration;

impl DefiniteIntegration {
    pub fn evaluate(integrand: &Expression, variable: Symbol, lower: Expression, upper: Expression)
        -> Result<Expression, Error>
    {
        // Step 1: Compute indefinite integral F(x)
        let antiderivative = integrand.integrate(variable.clone());

        // Step 2: Check for symbolic integral (couldn't integrate)
        if Self::is_symbolic_integral(&antiderivative) {
            return Err(Error::UnableToIntegrate);
        }

        // Step 3: Evaluate F(upper) - F(lower)
        let upper_eval = antiderivative.substitute(&variable, &upper).simplify();
        let lower_eval = antiderivative.substitute(&variable, &lower).simplify();

        let result = Expression::add(vec![
            upper_eval,
            Expression::mul(vec![Expression::integer(-1), lower_eval])
        ]).simplify();

        Ok(result)
    }

    /// Check for domain restrictions in [lower, upper]
    fn check_domain(integrand: &Expression, variable: &Symbol, lower: &Expression, upper: &Expression)
        -> Result<(), DomainError>
    {
        // Check for discontinuities (e.g., 1/x at x=0)
        // Check for undefined regions (e.g., √x for x<0)
        // ...
    }
}
```

### Validation Suite (150+ SymPy Tests)

**Test Categories**:
1. Basic integrals (30 tests)
2. Rational functions (35 tests)
3. Trigonometric (35 tests)
4. Substitution (30 tests)
5. Table patterns (30 tests)
6. Risch cases (30 tests)
7. Edge cases (20 tests)

**Validation Method**:
```rust
#[test]
fn validate_against_sympy() {
    for case in SYMPY_TEST_CASES {
        let mathhook_result = integrate(&case.integrand, case.variable);
        let sympy_result = run_sympy(&case.sympy_expression);
        assert_equivalent(&mathhook_result, &sympy_result);
    }
}
```

### Performance Benchmarks

**Metrics**:
- Average integration time by technique
- Technique hit rates (% coverage per layer)
- Memory usage profiling
- Cache miss rates

**Targets**:
- Fast path (table + rational): < 1ms, 85% coverage
- Medium path (substitution + trig): < 10ms, 7% coverage
- Slow path (Risch): < 1s, 5% coverage
- Timeout: 10s maximum

### Expected Outcome (Wave 6)

- **Coverage**: 93-95% validated against SymPy
- **Performance**: Documented and optimized
- **Tests**: 150+ validation tests, 20 definite integral tests
- **Documentation**: Complete API docs, tutorial, algorithm explanations

## Integration with Existing Systems

### Function Registry

**Enhancement**: Add more antiderivative rules to registry during Waves 2-4:
- Wave 2: Register rational function patterns
- Wave 3: Register table patterns
- Wave 4: Register trigonometric reduction formulas

### Simplification

**Integration Point**: Automatically simplify integration results:
```rust
pub fn integrate_and_simplify(expr: &Expression, variable: Symbol) -> Expression {
    let result = strategy.integrate(expr, variable);
    result.simplify()
}
```

### Educational Module

**Enhancement**: Generate step-by-step explanations for each technique:
- Partial fraction decomposition steps
- U-substitution variable choice explanation
- Trigonometric reduction steps
- Risch tower construction explanation

## Conclusion

This design provides a clear roadmap for Waves 2-6:

**Wave 2** (2 weeks): Rational functions + strategy dispatcher (75% → 85%)
**Wave 3** (3 weeks): Integration table + substitution (85% → 90%)
**Wave 4** (2 weeks): Trigonometric integrals (90% → 92%)
**Wave 5** (5 weeks): Risch algorithm (92% → 95%)
**Wave 6** (2 weeks): Testing, documentation, definite integrals

**Total**: 14 weeks to achieve 93-95% coverage with comprehensive testing and documentation.

Each wave is modular and builds on previous work. The layered strategy dispatcher ensures optimal performance while the comprehensive test suite validates mathematical correctness against SymPy.
