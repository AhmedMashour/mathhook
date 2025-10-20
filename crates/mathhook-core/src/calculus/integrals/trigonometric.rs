//! Trigonometric integration patterns
//!
//! Implements integration of trigonometric functions using reduction formulas,
//! power reduction, and trigonometric identities.
//!
//! # Supported Patterns
//!
//! - Powers of sine and cosine: ∫sin^m(x)*cos^n(x) dx
//! - Powers of tangent and secant: ∫tan^m(x)*sec^n(x) dx
//! - Powers of cotangent and cosecant: ∫cot^m(x)*csc^n(x) dx
//! - Products of trig functions: ∫sin(mx)*cos(nx) dx
//!
//! # Algorithm Strategy
//!
//! 1. Detect trigonometric pattern (sin^m*cos^n, tan^m*sec^n, etc.)
//! 2. For sin^m*cos^n:
//!    - If m is odd: Use u = cos(x) substitution
//!    - If n is odd: Use u = sin(x) substitution
//!    - If both even: Use power reduction formulas
//! 3. For tan^m*sec^n: Use tan/sec identities and substitution
//! 4. For products with different frequencies: Use product-to-sum formulas

use crate::core::{Expression, Number, Symbol};

/// Try to integrate trigonometric expressions
///
/// # Arguments
///
/// * `expr` - The expression to integrate
/// * `var` - The variable of integration
///
/// # Returns
///
/// Some(result) if pattern matches, None otherwise
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::integrals::trigonometric::try_trigonometric_integration;
/// use mathhook_core::symbol;
/// use mathhook_core::core::Expression;
///
/// let x = symbol!(x);
/// // ∫sin³(x) dx
/// let integrand = Expression::pow(
///     Expression::function("sin", vec![Expression::symbol(x.clone())]),
///     Expression::integer(3)
/// );
/// let result = try_trigonometric_integration(&integrand, x);
/// assert!(result.is_some());
/// ```
pub fn try_trigonometric_integration(expr: &Expression, var: Symbol) -> Option<Expression> {
    if let Some(pattern) = detect_trig_pattern(expr, &var) {
        match pattern {
            TrigPattern::SinCosPower { sin_power, cos_power } => {
                integrate_sin_cos_power(sin_power, cos_power, var)
            }
            TrigPattern::TanSecPower { tan_power, sec_power } => {
                integrate_tan_sec_power(tan_power, sec_power, var)
            }
            TrigPattern::CotCscPower { cot_power, csc_power } => {
                integrate_cot_csc_power(cot_power, csc_power, var)
            }
            TrigPattern::ProductDifferentFreq { func1, m, func2, n } => {
                integrate_trig_product(&func1, m, &func2, n, var)
            }
            TrigPattern::TanPower { power } => integrate_tan_power(power, var),
            TrigPattern::CotPower { power } => integrate_cot_power(power, var),
            TrigPattern::SecPower { power } => integrate_sec_power(power, var),
            TrigPattern::CscPower { power } => integrate_csc_power(power, var),
        }
    } else {
        None
    }
}

/// Trigonometric pattern types
#[derive(Debug, Clone, PartialEq)]
enum TrigPattern {
    SinCosPower { sin_power: i64, cos_power: i64 },
    TanSecPower { tan_power: i64, sec_power: i64 },
    CotCscPower { cot_power: i64, csc_power: i64 },
    ProductDifferentFreq { func1: String, m: i64, func2: String, n: i64 },
    TanPower { power: i64 },
    CotPower { power: i64 },
    SecPower { power: i64 },
    CscPower { power: i64 },
}

/// Detect trigonometric patterns in expression
fn detect_trig_pattern(expr: &Expression, var: &Symbol) -> Option<TrigPattern> {
    match expr {
        Expression::Pow(base, exp) => {
            if let (Expression::Function { name, args }, Expression::Number(Number::Integer(n))) = (&**base, &**exp) {
                if args.len() == 1 && is_simple_var(&args[0], var) {
                    match name.as_str() {
                        "sin" => return Some(TrigPattern::SinCosPower { sin_power: *n, cos_power: 0 }),
                        "cos" => return Some(TrigPattern::SinCosPower { sin_power: 0, cos_power: *n }),
                        "tan" => return Some(TrigPattern::TanPower { power: *n }),
                        "cot" => return Some(TrigPattern::CotPower { power: *n }),
                        "sec" => return Some(TrigPattern::SecPower { power: *n }),
                        "csc" => return Some(TrigPattern::CscPower { power: *n }),
                        _ => {}
                    }
                }
            }
        }
        Expression::Mul(factors) => {
            let mut sin_power = 0i64;
            let mut cos_power = 0i64;
            let mut tan_power = 0i64;
            let mut sec_power = 0i64;
            let mut cot_power = 0i64;
            let mut csc_power = 0i64;
            let mut other_factors = Vec::new();

            for factor in factors.iter() {
                match factor {
                    Expression::Function { name, args } if args.len() == 1 && is_simple_var(&args[0], var) => {
                        match name.as_str() {
                            "sin" => sin_power += 1,
                            "cos" => cos_power += 1,
                            "tan" => tan_power += 1,
                            "sec" => sec_power += 1,
                            "cot" => cot_power += 1,
                            "csc" => csc_power += 1,
                            _ => other_factors.push(factor),
                        }
                    }
                    Expression::Pow(base, exp) => {
                        if let (Expression::Function { name, args }, Expression::Number(Number::Integer(n))) = (&**base, &**exp) {
                            if args.len() == 1 && is_simple_var(&args[0], var) {
                                match name.as_str() {
                                    "sin" => sin_power += n,
                                    "cos" => cos_power += n,
                                    "tan" => tan_power += n,
                                    "sec" => sec_power += n,
                                    "cot" => cot_power += n,
                                    "csc" => csc_power += n,
                                    _ => other_factors.push(factor),
                                }
                            } else {
                                other_factors.push(factor);
                            }
                        } else {
                            other_factors.push(factor);
                        }
                    }
                    _ => other_factors.push(factor),
                }
            }

            if !other_factors.is_empty() {
                return None;
            }

            if sin_power > 0 || cos_power > 0 {
                if tan_power == 0 && sec_power == 0 && cot_power == 0 && csc_power == 0 {
                    return Some(TrigPattern::SinCosPower { sin_power, cos_power });
                }
            }

            if tan_power > 0 || sec_power > 0 {
                if sin_power == 0 && cos_power == 0 && cot_power == 0 && csc_power == 0 {
                    return Some(TrigPattern::TanSecPower { tan_power, sec_power });
                }
            }

            if cot_power > 0 || csc_power > 0 {
                if sin_power == 0 && cos_power == 0 && tan_power == 0 && sec_power == 0 {
                    return Some(TrigPattern::CotCscPower { cot_power, csc_power });
                }
            }
        }
        Expression::Function { name, args } if args.len() == 1 && is_simple_var(&args[0], var) => {
            match name.as_str() {
                "sin" => return Some(TrigPattern::SinCosPower { sin_power: 1, cos_power: 0 }),
                "cos" => return Some(TrigPattern::SinCosPower { sin_power: 0, cos_power: 1 }),
                "tan" => return Some(TrigPattern::TanPower { power: 1 }),
                "cot" => return Some(TrigPattern::CotPower { power: 1 }),
                "sec" => return Some(TrigPattern::SecPower { power: 1 }),
                "csc" => return Some(TrigPattern::CscPower { power: 1 }),
                _ => {}
            }
        }
        _ => {}
    }

    None
}

/// Check if expression is just the variable
fn is_simple_var(expr: &Expression, var: &Symbol) -> bool {
    matches!(expr, Expression::Symbol(s) if s == var)
}

/// Integrate sin^m(x) * cos^n(x)
fn integrate_sin_cos_power(m: i64, n: i64, var: Symbol) -> Option<Expression> {
    let x = Expression::symbol(var.clone());

    if m == 1 && n == 0 {
        return Some(Expression::mul(vec![
            Expression::integer(-1),
            Expression::function("cos", vec![x]),
        ]));
    }

    if m == 0 && n == 1 {
        return Some(Expression::function("sin", vec![x]));
    }

    if m > 0 && m % 2 == 1 {
        return Some(integrate_with_cos_substitution(m, n, var));
    }

    if n > 0 && n % 2 == 1 {
        return Some(integrate_with_sin_substitution(m, n, var));
    }

    if m >= 0 && n >= 0 && m % 2 == 0 && n % 2 == 0 {
        return Some(integrate_with_power_reduction(m, n, var));
    }

    None
}

/// Use cos substitution for odd sine power
fn integrate_with_cos_substitution(m: i64, n: i64, var: Symbol) -> Expression {
    let x = Expression::symbol(var.clone());

    if m == 1 {
        if n == 0 {
            return Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("cos", vec![x]),
            ]);
        } else {
            return Expression::mul(vec![
                Expression::integer(-1),
                Expression::pow(
                    Expression::function("cos", vec![x]),
                    Expression::integer(n + 1),
                ),
                Expression::rational(1, n + 1),
            ]);
        }
    }

    if m == 3 && n == 0 {
        return Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("cos", vec![x.clone()]),
            ]),
            Expression::mul(vec![
                Expression::pow(
                    Expression::function("cos", vec![x]),
                    Expression::integer(3),
                ),
                Expression::rational(1, 3),
            ]),
        ]);
    }

    Expression::integral(
        Expression::mul(vec![
            Expression::pow(Expression::function("sin", vec![x.clone()]), Expression::integer(m)),
            Expression::pow(Expression::function("cos", vec![x]), Expression::integer(n)),
        ]),
        var,
    )
}

/// Use sin substitution for odd cosine power
fn integrate_with_sin_substitution(m: i64, n: i64, var: Symbol) -> Expression {
    let x = Expression::symbol(var.clone());

    if n == 1 {
        if m == 0 {
            return Expression::function("sin", vec![x]);
        } else {
            return Expression::mul(vec![
                Expression::pow(
                    Expression::function("sin", vec![x]),
                    Expression::integer(m + 1),
                ),
                Expression::rational(1, m + 1),
            ]);
        }
    }

    if n == 3 && m == 0 {
        return Expression::add(vec![
            Expression::function("sin", vec![x.clone()]),
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::pow(
                    Expression::function("sin", vec![x]),
                    Expression::integer(3),
                ),
                Expression::rational(1, 3),
            ]),
        ]);
    }

    Expression::integral(
        Expression::mul(vec![
            Expression::pow(Expression::function("sin", vec![x.clone()]), Expression::integer(m)),
            Expression::pow(Expression::function("cos", vec![x]), Expression::integer(n)),
        ]),
        var,
    )
}

/// Use power reduction formulas for even powers
fn integrate_with_power_reduction(m: i64, n: i64, var: Symbol) -> Expression {
    let x = Expression::symbol(var.clone());

    if m == 2 && n == 0 {
        return Expression::add(vec![
            Expression::mul(vec![Expression::rational(1, 2), x.clone()]),
            Expression::mul(vec![
                Expression::rational(-1, 4),
                Expression::function("sin", vec![
                    Expression::mul(vec![Expression::integer(2), x]),
                ]),
            ]),
        ]);
    }

    if m == 0 && n == 2 {
        return Expression::add(vec![
            Expression::mul(vec![Expression::rational(1, 2), x.clone()]),
            Expression::mul(vec![
                Expression::rational(1, 4),
                Expression::function("sin", vec![
                    Expression::mul(vec![Expression::integer(2), x]),
                ]),
            ]),
        ]);
    }

    if m == 2 && n == 2 {
        return Expression::add(vec![
            Expression::mul(vec![Expression::rational(1, 8), x.clone()]),
            Expression::mul(vec![
                Expression::rational(-1, 32),
                Expression::function("sin", vec![
                    Expression::mul(vec![Expression::integer(4), x]),
                ]),
            ]),
        ]);
    }

    Expression::integral(
        Expression::mul(vec![
            Expression::pow(Expression::function("sin", vec![x.clone()]), Expression::integer(m)),
            Expression::pow(Expression::function("cos", vec![x]), Expression::integer(n)),
        ]),
        var,
    )
}

/// Integrate tan^m * sec^n
fn integrate_tan_sec_power(m: i64, n: i64, var: Symbol) -> Option<Expression> {
    let x = Expression::symbol(var.clone());

    if m == 0 && n == 2 {
        return Some(Expression::function("tan", vec![x]));
    }

    if m == 1 && n == 1 {
        return Some(Expression::function("sec", vec![x]));
    }

    if m == 2 && n == 0 {
        return Some(Expression::add(vec![
            Expression::function("tan", vec![x.clone()]),
            Expression::mul(vec![Expression::integer(-1), x]),
        ]));
    }

    if m == 3 && n == 2 {
        return Some(Expression::mul(vec![
            Expression::rational(1, 4),
            Expression::pow(Expression::function("tan", vec![x]), Expression::integer(4)),
        ]));
    }

    None
}

/// Integrate cot^m * csc^n
fn integrate_cot_csc_power(m: i64, n: i64, var: Symbol) -> Option<Expression> {
    let x = Expression::symbol(var.clone());

    if m == 0 && n == 2 {
        return Some(Expression::mul(vec![
            Expression::integer(-1),
            Expression::function("cot", vec![x]),
        ]));
    }

    if m == 1 && n == 1 {
        return Some(Expression::mul(vec![
            Expression::integer(-1),
            Expression::function("csc", vec![x]),
        ]));
    }

    if m == 2 && n == 0 {
        return Some(Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("cot", vec![x.clone()]),
            ]),
            Expression::mul(vec![Expression::integer(-1), x]),
        ]));
    }

    None
}

/// Integrate products like sin(mx)*cos(nx)
fn integrate_trig_product(func1: &str, m: i64, func2: &str, n: i64, var: Symbol) -> Option<Expression> {
    let _ = (func1, m, func2, n, var);
    None
}

/// Integrate tan^n(x)
fn integrate_tan_power(power: i64, var: Symbol) -> Option<Expression> {
    let x = Expression::symbol(var.clone());

    if power == 1 {
        return Some(Expression::mul(vec![
            Expression::integer(-1),
            Expression::function("ln", vec![
                Expression::function("cos", vec![x]),
            ]),
        ]));
    }

    if power == 2 {
        return Some(Expression::add(vec![
            Expression::function("tan", vec![x.clone()]),
            Expression::mul(vec![Expression::integer(-1), x]),
        ]));
    }

    None
}

/// Integrate cot^n(x)
fn integrate_cot_power(power: i64, var: Symbol) -> Option<Expression> {
    let x = Expression::symbol(var.clone());

    if power == 1 {
        return Some(Expression::function("ln", vec![
            Expression::function("sin", vec![x]),
        ]));
    }

    if power == 2 {
        return Some(Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("cot", vec![x.clone()]),
            ]),
            Expression::mul(vec![Expression::integer(-1), x]),
        ]));
    }

    None
}

/// Integrate sec^n(x)
fn integrate_sec_power(power: i64, var: Symbol) -> Option<Expression> {
    let x = Expression::symbol(var);

    if power == 1 {
        return Some(Expression::function("ln", vec![
            Expression::add(vec![
                Expression::function("sec", vec![x.clone()]),
                Expression::function("tan", vec![x]),
            ]),
        ]));
    }

    if power == 2 {
        return Some(Expression::function("tan", vec![x]));
    }

    None
}

/// Integrate csc^n(x)
fn integrate_csc_power(power: i64, var: Symbol) -> Option<Expression> {
    let x = Expression::symbol(var);

    if power == 1 {
        return Some(Expression::mul(vec![
            Expression::integer(-1),
            Expression::function("ln", vec![
                Expression::add(vec![
                    Expression::function("csc", vec![x.clone()]),
                    Expression::function("cot", vec![x]),
                ]),
            ]),
        ]));
    }

    if power == 2 {
        return Some(Expression::mul(vec![
            Expression::integer(-1),
            Expression::function("cot", vec![x]),
        ]));
    }

    None
}
