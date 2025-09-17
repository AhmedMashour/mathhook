//! Tangent, Secant, Cotangent, and Cosecant integration
//!
//! Implements integration of tan, sec, cot, csc and their powers.

use crate::core::{Expression, Symbol};

/// Integrate tan^m * sec^n
///
/// # Mathematical Background
///
/// **Key Identities:**
/// - tan²(x) = sec²(x) - 1
/// - d/dx[tan(x)] = sec²(x)
/// - d/dx[sec(x)] = sec(x)tan(x)
///
/// **Common Cases:**
/// - ∫sec²(x) dx = tan(x)
/// - ∫tan(x)sec(x) dx = sec(x)
/// - ∫tan²(x) dx = tan(x) - x
/// - ∫tan³(x)sec²(x) dx = tan⁴(x)/4
pub fn integrate_tan_sec_power(m: i64, n: i64, var: Symbol) -> Option<Expression> {
    let x = Expression::symbol(var);

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
///
/// # Mathematical Background
///
/// **Key Identities:**
/// - cot²(x) = csc²(x) - 1
/// - d/dx[cot(x)] = -csc²(x)
/// - d/dx[csc(x)] = -csc(x)cot(x)
///
/// **Common Cases:**
/// - ∫csc²(x) dx = -cot(x)
/// - ∫cot(x)csc(x) dx = -csc(x)
/// - ∫cot²(x) dx = -cot(x) - x
pub fn integrate_cot_csc_power(m: i64, n: i64, var: Symbol) -> Option<Expression> {
    let x = Expression::symbol(var);

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

/// Integrate tan^n(x)
///
/// # Mathematical Formulas
///
/// - ∫tan(x) dx = -ln|cos(x)| = ln|sec(x)|
/// - ∫tan²(x) dx = tan(x) - x
///
/// **General Strategy:** Use reduction formula for higher powers
pub fn integrate_tan_power(power: i64, var: Symbol) -> Option<Expression> {
    let x = Expression::symbol(var);

    if power == 1 {
        return Some(Expression::mul(vec![
            Expression::integer(-1),
            Expression::function("ln", vec![Expression::function("cos", vec![x])]),
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
///
/// # Mathematical Formulas
///
/// - ∫cot(x) dx = ln|sin(x)|
/// - ∫cot²(x) dx = -cot(x) - x
///
/// **General Strategy:** Use reduction formula for higher powers
pub fn integrate_cot_power(power: i64, var: Symbol) -> Option<Expression> {
    let x = Expression::symbol(var);

    if power == 1 {
        return Some(Expression::function(
            "ln",
            vec![Expression::function("sin", vec![x])],
        ));
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
///
/// # Mathematical Formulas
///
/// - ∫sec(x) dx = ln|sec(x) + tan(x)|
/// - ∫sec²(x) dx = tan(x)
///
/// **Derivation for ∫sec(x) dx:**
/// Multiply by (sec(x) + tan(x))/(sec(x) + tan(x)):
/// - Numerator becomes: sec(x)[sec(x) + tan(x)]
/// - This is the derivative of the denominator
/// - Result: ln|sec(x) + tan(x)|
pub fn integrate_sec_power(power: i64, var: Symbol) -> Option<Expression> {
    let x = Expression::symbol(var);

    if power == 1 {
        return Some(Expression::function(
            "ln",
            vec![Expression::add(vec![
                Expression::function("sec", vec![x.clone()]),
                Expression::function("tan", vec![x]),
            ])],
        ));
    }

    if power == 2 {
        return Some(Expression::function("tan", vec![x]));
    }

    None
}

/// Integrate csc^n(x)
///
/// # Mathematical Formulas
///
/// - ∫csc(x) dx = -ln|csc(x) + cot(x)| = ln|csc(x) - cot(x)|
/// - ∫csc²(x) dx = -cot(x)
///
/// **Derivation for ∫csc(x) dx:**
/// Multiply by (csc(x) + cot(x))/(csc(x) + cot(x)):
/// - Numerator becomes: -csc(x)[csc(x) + cot(x)]
/// - This is the derivative of the denominator
/// - Result: -ln|csc(x) + cot(x)|
pub fn integrate_csc_power(power: i64, var: Symbol) -> Option<Expression> {
    let x = Expression::symbol(var);

    if power == 1 {
        return Some(Expression::mul(vec![
            Expression::integer(-1),
            Expression::function(
                "ln",
                vec![Expression::add(vec![
                    Expression::function("csc", vec![x.clone()]),
                    Expression::function("cot", vec![x]),
                ])],
            ),
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
