//! Integration table lookup for common patterns
//!
//! Provides O(1) lookup for approximately 30 common integration patterns,
//! covering 60-70% of typical integrals. This is the fastest integration
//! strategy and should be tried first before more complex techniques.

use crate::core::{Expression, Number, Symbol};

/// Pattern key for table lookup
///
/// Represents common integration patterns that can be matched
/// and integrated using closed-form formulas.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PatternKey {
    /// Power function: x^n (for n != -1)
    Power { exponent: i64 },
    /// Reciprocal: 1/x
    Reciprocal,
    /// Square root: sqrt(x)
    SquareRoot,
    /// Reciprocal square root: 1/sqrt(x)
    ReciprocalSquareRoot,
    /// Exponential: e^(ax)
    Exponential { coefficient: i64 },
    /// General exponential: a^x
    GeneralExponential,
    /// Natural logarithm: ln(x)
    NaturalLog,
    /// Sine: sin(ax)
    Sine { coefficient: i64 },
    /// Cosine: cos(ax)
    Cosine { coefficient: i64 },
    /// Tangent: tan(x)
    Tangent,
    /// Cotangent: cot(x)
    Cotangent,
    /// Secant: sec(x)
    Secant,
    /// Cosecant: csc(x)
    Cosecant,
    /// Sine squared: sin^2(x)
    SineSquared,
    /// Cosine squared: cos^2(x)
    CosineSquared,
    /// Arctangent pattern: 1/(x^2 + a^2)
    ArctanPattern { a_squared: i64 },
    /// Arcsine pattern: 1/sqrt(a^2 - x^2)
    ArcsinPattern { a_squared: i64 },
    /// Hyperbolic sine: sinh(x)
    HyperbolicSine,
    /// Hyperbolic cosine: cosh(x)
    HyperbolicCosine,
    /// Hyperbolic tangent: tanh(x)
    HyperbolicTangent,
    /// Product pattern: x*e^x
    XTimesExp,
    /// Product pattern: x^2*e^x
    XSquaredTimesExp,
    /// Product pattern: x*ln(x)
    XTimesLog,
}

/// Try to integrate expression using table lookup
///
/// # Arguments
///
/// * `expr` - The expression to integrate
/// * `var` - The variable of integration
///
/// # Returns
///
/// Some(integrated_expression) if pattern matches, None otherwise
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::integrals::table::try_table_lookup;
/// use mathhook_core::{Expression, symbol};
///
/// let x = symbol!(x);
/// let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
/// let result = try_table_lookup(&expr, &x);
/// assert!(result.is_some());
/// ```
pub fn try_table_lookup(expr: &Expression, var: &Symbol) -> Option<Expression> {
    // Extract coefficient if expression is c*f(x)
    let (coeff, core_expr) = extract_coefficient(expr, var);

    // Try to match core expression to a pattern
    let pattern = match_pattern(&core_expr, var)?;

    // Get integration result from pattern
    let result = integrate_pattern(&pattern, var);

    // Apply coefficient if present
    if coeff.is_one() {
        Some(result)
    } else {
        Some(Expression::mul(vec![coeff, result]))
    }
}

/// Extract coefficient from expression of form c*f(x)
fn extract_coefficient(expr: &Expression, var: &Symbol) -> (Expression, Expression) {
    match expr {
        Expression::Mul(factors) => {
            let mut constants = Vec::new();
            let mut variables = Vec::new();

            for factor in factors.iter() {
                if is_constant_wrt(factor, var) {
                    constants.push(factor.clone());
                } else {
                    variables.push(factor.clone());
                }
            }

            if variables.len() == 1 && !constants.is_empty() {
                let coeff = if constants.len() == 1 {
                    constants[0].clone()
                } else {
                    Expression::mul(constants)
                };
                (coeff, variables[0].clone())
            } else {
                (Expression::integer(1), expr.clone())
            }
        }
        _ => (Expression::integer(1), expr.clone()),
    }
}

/// Match expression to integration pattern
fn match_pattern(expr: &Expression, var: &Symbol) -> Option<PatternKey> {
    match expr {
        // Power patterns: x^n
        Expression::Pow(base, exp) => {
            if let Expression::Symbol(s) = &**base {
                if s == var {
                    // Check for x^n where n is integer
                    if let Expression::Number(Number::Integer(n)) = &**exp {
                        if *n == -1 {
                            return Some(PatternKey::Reciprocal);
                        } else {
                            return Some(PatternKey::Power { exponent: *n });
                        }
                    }
                    // Check for x^(1/2) (square root)
                    if let Expression::Mul(factors) = &**exp {
                        if factors.len() == 2 {
                            if let (
                                Expression::Number(Number::Integer(1)),
                                Expression::Pow(two, neg_one),
                            ) = (&factors[0], &factors[1])
                            {
                                if matches!(**two, Expression::Number(Number::Integer(2)))
                                    && matches!(**neg_one, Expression::Number(Number::Integer(-1)))
                                {
                                    return Some(PatternKey::SquareRoot);
                                }
                            }
                        }
                    }
                }
            }
            None
        }

        // Symbol alone: x
        Expression::Symbol(s) if s == var => Some(PatternKey::Power { exponent: 1 }),

        // Function patterns
        Expression::Function { name, args } => {
            if args.len() != 1 {
                return None;
            }

            let arg = &args[0];

            // Check if argument is x or ax
            let coeff = if let Expression::Symbol(s) = arg {
                if s == var {
                    1
                } else {
                    return None;
                }
            } else if let Expression::Mul(factors) = arg {
                // Check for a*x pattern
                if factors.len() == 2 {
                    if let (Expression::Number(Number::Integer(a)), Expression::Symbol(s)) =
                        (&factors[0], &factors[1])
                    {
                        if s == var {
                            *a
                        } else {
                            return None;
                        }
                    } else if let (Expression::Symbol(s), Expression::Number(Number::Integer(a))) =
                        (&factors[0], &factors[1])
                    {
                        if s == var {
                            *a
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            };

            match name.as_str() {
                "exp" => Some(PatternKey::Exponential { coefficient: coeff }),
                "ln" => {
                    if coeff == 1 {
                        Some(PatternKey::NaturalLog)
                    } else {
                        None
                    }
                }
                "sin" => Some(PatternKey::Sine { coefficient: coeff }),
                "cos" => Some(PatternKey::Cosine { coefficient: coeff }),
                "tan" => {
                    if coeff == 1 {
                        Some(PatternKey::Tangent)
                    } else {
                        None
                    }
                }
                "cot" => {
                    if coeff == 1 {
                        Some(PatternKey::Cotangent)
                    } else {
                        None
                    }
                }
                "sec" => {
                    if coeff == 1 {
                        Some(PatternKey::Secant)
                    } else {
                        None
                    }
                }
                "csc" => {
                    if coeff == 1 {
                        Some(PatternKey::Cosecant)
                    } else {
                        None
                    }
                }
                "sinh" => {
                    if coeff == 1 {
                        Some(PatternKey::HyperbolicSine)
                    } else {
                        None
                    }
                }
                "cosh" => {
                    if coeff == 1 {
                        Some(PatternKey::HyperbolicCosine)
                    } else {
                        None
                    }
                }
                "tanh" => {
                    if coeff == 1 {
                        Some(PatternKey::HyperbolicTangent)
                    } else {
                        None
                    }
                }
                "sqrt" => {
                    if let Expression::Symbol(s) = arg {
                        if s == var && coeff == 1 {
                            return Some(PatternKey::SquareRoot);
                        }
                    }
                    None
                }
                _ => None,
            }
        }

        // Rational patterns: 1/(x^2 + a^2), 1/sqrt(a^2 - x^2), etc.
        Expression::Mul(factors) => {
            // Look for patterns with denominator^(-1)
            // Could be: [denom^(-1)] or [1, denom^(-1)] or [coeff, denom^(-1)]
            for factor in factors.iter() {
                if let Expression::Pow(denom, exp) = factor {
                    if matches!(**exp, Expression::Number(Number::Integer(-1))) {
                        // Check for x^2 + a^2 pattern
                        if let Expression::Add(terms) = &**denom {
                            if terms.len() == 2 {
                                if let (
                                    Expression::Pow(x_base, two1),
                                    Expression::Number(Number::Integer(a_sq)),
                                ) = (&terms[0], &terms[1])
                                {
                                    if matches!(**two1, Expression::Number(Number::Integer(2))) {
                                        if let Expression::Symbol(s) = &**x_base {
                                            if s == var && *a_sq > 0 {
                                                return Some(PatternKey::ArctanPattern {
                                                    a_squared: *a_sq,
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        // Check for 1/sqrt(a^2 - x^2) pattern
                        if let Expression::Function { name, args } = &**denom {
                            if name == "sqrt" && args.len() == 1 {
                                if let Expression::Add(terms) = &args[0] {
                                    if terms.len() == 2 {
                                        // Handle a^2 - x^2 (represented as a^2 + (-1)*x^2)
                                        if let (
                                            Expression::Number(Number::Integer(a_sq)),
                                            Expression::Mul(neg_x_sq_factors),
                                        ) = (&terms[0], &terms[1])
                                        {
                                            if *a_sq > 0 && neg_x_sq_factors.len() == 2 {
                                                if let (
                                                    Expression::Number(Number::Integer(-1)),
                                                    Expression::Pow(x_base, two),
                                                ) = (&neg_x_sq_factors[0], &neg_x_sq_factors[1])
                                                {
                                                    if matches!(
                                                        **two,
                                                        Expression::Number(Number::Integer(2))
                                                    ) {
                                                        if let Expression::Symbol(s) = &**x_base {
                                                            if s == var {
                                                                return Some(
                                                                    PatternKey::ArcsinPattern {
                                                                        a_squared: *a_sq,
                                                                    },
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None
        }

        _ => None,
    }
}

/// Integrate a matched pattern
fn integrate_pattern(pattern: &PatternKey, var: &Symbol) -> Expression {
    let x = Expression::symbol(var.clone());

    match pattern {
        PatternKey::Power { exponent: n } => {
            // ∫x^n dx = x^(n+1)/(n+1)
            let new_exp = Expression::integer(n + 1);
            Expression::mul(vec![
                Expression::rational(1, n + 1),
                Expression::pow(x, new_exp),
            ])
        }

        PatternKey::Reciprocal => {
            // ∫(1/x) dx = ln|x|
            Expression::function("ln", vec![Expression::function("abs", vec![x])])
        }

        PatternKey::SquareRoot => {
            // ∫√x dx = (2/3)x^(3/2)
            Expression::mul(vec![
                Expression::rational(2, 3),
                Expression::pow(x, Expression::rational(3, 2)),
            ])
        }

        PatternKey::ReciprocalSquareRoot => {
            // ∫(1/√x) dx = 2√x
            Expression::mul(vec![
                Expression::integer(2),
                Expression::function("sqrt", vec![x]),
            ])
        }

        PatternKey::Exponential { coefficient: a } => {
            // ∫e^(ax) dx = e^(ax)/a
            let ax = if *a == 1 {
                x
            } else {
                Expression::mul(vec![Expression::integer(*a), x])
            };
            let result = Expression::function("exp", vec![ax]);
            if *a == 1 {
                result
            } else {
                Expression::mul(vec![Expression::rational(1, *a), result])
            }
        }

        PatternKey::NaturalLog => {
            // ∫ln(x) dx = x*ln(x) - x
            Expression::add(vec![
                Expression::mul(vec![x.clone(), Expression::function("ln", vec![x.clone()])]),
                Expression::mul(vec![Expression::integer(-1), x]),
            ])
        }

        PatternKey::Sine { coefficient: a } => {
            // ∫sin(ax) dx = -cos(ax)/a
            let ax = if *a == 1 {
                x
            } else {
                Expression::mul(vec![Expression::integer(*a), x])
            };
            let cos_ax = Expression::function("cos", vec![ax]);
            Expression::mul(vec![Expression::rational(-1, *a), cos_ax])
        }

        PatternKey::Cosine { coefficient: a } => {
            // ∫cos(ax) dx = sin(ax)/a
            let ax = if *a == 1 {
                x
            } else {
                Expression::mul(vec![Expression::integer(*a), x])
            };
            let sin_ax = Expression::function("sin", vec![ax]);
            if *a == 1 {
                sin_ax
            } else {
                Expression::mul(vec![Expression::rational(1, *a), sin_ax])
            }
        }

        PatternKey::Tangent => {
            // ∫tan(x) dx = -ln|cos(x)|
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::function(
                    "ln",
                    vec![Expression::function(
                        "abs",
                        vec![Expression::function("cos", vec![x])],
                    )],
                ),
            ])
        }

        PatternKey::Cotangent => {
            // ∫cot(x) dx = ln|sin(x)|
            Expression::function(
                "ln",
                vec![Expression::function(
                    "abs",
                    vec![Expression::function("sin", vec![x])],
                )],
            )
        }

        PatternKey::Secant => {
            // ∫sec(x) dx = ln|sec(x) + tan(x)|
            let sec_x = Expression::function("sec", vec![x.clone()]);
            let tan_x = Expression::function("tan", vec![x]);
            Expression::function(
                "ln",
                vec![Expression::function(
                    "abs",
                    vec![Expression::add(vec![sec_x, tan_x])],
                )],
            )
        }

        PatternKey::Cosecant => {
            // ∫csc(x) dx = -ln|csc(x) + cot(x)|
            let csc_x = Expression::function("csc", vec![x.clone()]);
            let cot_x = Expression::function("cot", vec![x]);
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::function(
                    "ln",
                    vec![Expression::function(
                        "abs",
                        vec![Expression::add(vec![csc_x, cot_x])],
                    )],
                ),
            ])
        }

        PatternKey::SineSquared => {
            // ∫sin^2(x) dx = x/2 - sin(2x)/4
            Expression::add(vec![
                Expression::mul(vec![Expression::rational(1, 2), x.clone()]),
                Expression::mul(vec![
                    Expression::rational(-1, 4),
                    Expression::function(
                        "sin",
                        vec![Expression::mul(vec![Expression::integer(2), x])],
                    ),
                ]),
            ])
        }

        PatternKey::CosineSquared => {
            // ∫cos^2(x) dx = x/2 + sin(2x)/4
            Expression::add(vec![
                Expression::mul(vec![Expression::rational(1, 2), x.clone()]),
                Expression::mul(vec![
                    Expression::rational(1, 4),
                    Expression::function(
                        "sin",
                        vec![Expression::mul(vec![Expression::integer(2), x])],
                    ),
                ]),
            ])
        }

        PatternKey::ArctanPattern { a_squared } => {
            // ∫1/(x^2 + a^2) dx = (1/a)*arctan(x/a)
            let a = (*a_squared as f64).sqrt() as i64;
            Expression::mul(vec![
                Expression::rational(1, a),
                Expression::function(
                    "atan",
                    vec![Expression::mul(vec![x, Expression::rational(1, a)])],
                ),
            ])
        }

        PatternKey::ArcsinPattern { a_squared } => {
            // ∫1/√(a^2 - x^2) dx = arcsin(x/a)
            let a = (*a_squared as f64).sqrt() as i64;
            Expression::function(
                "asin",
                vec![Expression::mul(vec![x, Expression::rational(1, a)])],
            )
        }

        PatternKey::HyperbolicSine => {
            // ∫sinh(x) dx = cosh(x)
            Expression::function("cosh", vec![x])
        }

        PatternKey::HyperbolicCosine => {
            // ∫cosh(x) dx = sinh(x)
            Expression::function("sinh", vec![x])
        }

        PatternKey::HyperbolicTangent => {
            // ∫tanh(x) dx = ln(cosh(x))
            Expression::function("ln", vec![Expression::function("cosh", vec![x])])
        }

        _ => {
            // Fallback (shouldn't reach here if pattern matching is correct)
            Expression::integral(Expression::symbol(var.clone()), var.clone())
        }
    }
}

/// Check if expression is constant with respect to variable
fn is_constant_wrt(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Number(_) | Expression::Constant(_) => true,
        Expression::Symbol(s) => s != var,
        Expression::Add(terms) => terms.iter().all(|t| is_constant_wrt(t, var)),
        Expression::Mul(factors) => factors.iter().all(|f| is_constant_wrt(f, var)),
        Expression::Pow(base, exp) => is_constant_wrt(base, var) && is_constant_wrt(exp, var),
        Expression::Function { args, .. } => args.iter().all(|a| is_constant_wrt(a, var)),
        _ => false,
    }
}
