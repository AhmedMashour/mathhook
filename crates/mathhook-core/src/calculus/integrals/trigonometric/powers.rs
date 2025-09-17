//! Sin and Cos integration strategies
//!
//! Implements integration of sin^m(x) * cos^n(x) using:
//! - Substitution (for odd powers)
//! - Power reduction (for even powers)

use crate::core::{Expression, Symbol};

/// Integrate sin^m(x) * cos^n(x)
///
/// # Strategy
///
/// 1. If m is odd: Use u = cos(x) substitution
/// 2. If n is odd: Use u = sin(x) substitution
/// 3. If both even: Use power reduction formulas
///
/// # Mathematical Background
///
/// **Odd Power Substitution:**
/// - sin^(2k+1)(x) = sin(x) * [sin²(x)]^k = sin(x) * [1 - cos²(x)]^k
/// - Substitute u = cos(x), du = -sin(x) dx
///
/// **Power Reduction:**
/// - sin²(x) = (1 - cos(2x))/2
/// - cos²(x) = (1 + cos(2x))/2
///
/// # Domain
/// x ∈ ℝ
pub fn integrate_sin_cos_power(m: i64, n: i64, var: Symbol) -> Option<Expression> {
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
///
/// # Mathematical Formula
///
/// For ∫sin^(2k+1)(x) * cos^n(x) dx:
/// - sin^(2k+1)(x) = sin(x) * [1 - cos²(x)]^k
/// - Substitute u = cos(x), du = -sin(x) dx
/// - Result: -∫(1 - u²)^k * u^n du
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
                Expression::pow(Expression::function("cos", vec![x]), Expression::integer(3)),
                Expression::rational(1, 3),
            ]),
        ]);
    }

    Expression::integral(
        Expression::mul(vec![
            Expression::pow(
                Expression::function("sin", vec![x.clone()]),
                Expression::integer(m),
            ),
            Expression::pow(Expression::function("cos", vec![x]), Expression::integer(n)),
        ]),
        var,
    )
}

/// Use sin substitution for odd cosine power
///
/// # Mathematical Formula
///
/// For ∫sin^m(x) * cos^(2k+1)(x) dx:
/// - cos^(2k+1)(x) = cos(x) * [1 - sin²(x)]^k
/// - Substitute u = sin(x), du = cos(x) dx
/// - Result: ∫u^m * (1 - u²)^k du
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
                Expression::pow(Expression::function("sin", vec![x]), Expression::integer(3)),
                Expression::rational(1, 3),
            ]),
        ]);
    }

    Expression::integral(
        Expression::mul(vec![
            Expression::pow(
                Expression::function("sin", vec![x.clone()]),
                Expression::integer(m),
            ),
            Expression::pow(Expression::function("cos", vec![x]), Expression::integer(n)),
        ]),
        var,
    )
}

/// Use power reduction formulas for even powers
///
/// # Mathematical Formulas
///
/// **Power Reduction Identities:**
/// - sin²(x) = (1 - cos(2x))/2
/// - cos²(x) = (1 + cos(2x))/2
///
/// **Common Cases:**
/// - ∫sin²(x) dx = x/2 - sin(2x)/4
/// - ∫cos²(x) dx = x/2 + sin(2x)/4
/// - ∫sin²(x)cos²(x) dx = x/8 - sin(4x)/32
fn integrate_with_power_reduction(m: i64, n: i64, var: Symbol) -> Expression {
    let x = Expression::symbol(var.clone());

    if m == 2 && n == 0 {
        return Expression::add(vec![
            Expression::mul(vec![Expression::rational(1, 2), x.clone()]),
            Expression::mul(vec![
                Expression::rational(-1, 4),
                Expression::function(
                    "sin",
                    vec![Expression::mul(vec![Expression::integer(2), x])],
                ),
            ]),
        ]);
    }

    if m == 0 && n == 2 {
        return Expression::add(vec![
            Expression::mul(vec![Expression::rational(1, 2), x.clone()]),
            Expression::mul(vec![
                Expression::rational(1, 4),
                Expression::function(
                    "sin",
                    vec![Expression::mul(vec![Expression::integer(2), x])],
                ),
            ]),
        ]);
    }

    if m == 2 && n == 2 {
        return Expression::add(vec![
            Expression::mul(vec![Expression::rational(1, 8), x.clone()]),
            Expression::mul(vec![
                Expression::rational(-1, 32),
                Expression::function(
                    "sin",
                    vec![Expression::mul(vec![Expression::integer(4), x])],
                ),
            ]),
        ]);
    }

    Expression::integral(
        Expression::mul(vec![
            Expression::pow(
                Expression::function("sin", vec![x.clone()]),
                Expression::integer(m),
            ),
            Expression::pow(Expression::function("cos", vec![x]), Expression::integer(n)),
        ]),
        var,
    )
}
