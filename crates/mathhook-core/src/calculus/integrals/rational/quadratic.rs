//! Integration of irreducible quadratic factors in partial fraction decomposition
//!
//! Implements both simple quadratic integration (power=1) and Ostrogradsky's
//! reduction formula for repeated quadratics (power=2).

use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;

/// Integrate repeated irreducible quadratic using Ostrogradsky's reduction
///
/// For `∫(Ax+B)/(x²+px+q)^m dx` where `m > 1`, uses Ostrogradsky's formula:
/// ```text
/// ∫f/D^m dx = g/D^(m-1) + ∫h/D dx
/// ```
///
/// # Mathematical Method (Ostrogradsky's Reduction)
///
/// 1. Set up reduction formula with unknown linear polynomial `g = ax+b`
/// 2. Use identity: `f = g'·D - (m-1)·g·D' + h·D^(m-1)`
/// 3. Expand and collect coefficients to solve for `a`, `b`, `h`
/// 4. Integrate remainder `h/D` using simple quadratic formula
///
/// # Current Implementation
///
/// Handles `m=2` case (most common). For higher powers, returns `None`.
/// Can be generalized using recursive Ostrogradsky reduction.
///
/// # Arguments
///
/// * `numerator` - Numerator expression (linear in `x`)
/// * `denominator` - Full denominator (for reference)
/// * `p` - Linear coefficient in `x²+px+q`
/// * `q` - Constant term in `x²+px+q`
/// * `power` - Exponent `m` (currently only `m=2` supported)
/// * `var` - Integration variable
///
/// # Returns
///
/// Integrated expression or `None` if unsupported power
///
/// # Examples
///
/// ```
/// use mathhook_core::{Expression, symbol};
/// use mathhook_core::calculus::integrals::rational::quadratic::integrate_repeated_quadratic;
///
/// let x = symbol!(x);
/// let numerator = Expression::integer(1);
/// let p = Expression::integer(0);
/// let q = Expression::integer(1);
/// let denominator = Expression::pow(
///     Expression::add(vec![
///         Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
///         q.clone(),
///     ]),
///     Expression::integer(2),
/// );
///
/// let result = integrate_repeated_quadratic(&numerator, &denominator, &p, &q, 2, &x);
/// assert!(result.is_some());
/// ```
///
/// # References
///
/// - Stewart, Calculus (8th ed), Section 7.4
/// - Bronstein, "Symbolic Integration I" (reduction formulas)
pub fn integrate_repeated_quadratic(
    numerator: &Expression,
    denominator: &Expression,
    p: &Expression,
    q: &Expression,
    power: i64,
    var: &Symbol,
) -> Option<Expression> {
    let d = Expression::add(vec![
        Expression::pow(Expression::symbol(var.clone()), Expression::integer(2)),
        Expression::mul(vec![p.clone(), Expression::symbol(var.clone())]),
        q.clone(),
    ]);

    if power != 2 {
        return None;
    }

    let (coeff_a, coeff_b) = extract_linear_coefficients(numerator, var)?;

    let denom_expr = Expression::add(vec![
        Expression::mul(vec![Expression::integer(4), q.clone()]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::pow(p.clone(), Expression::integer(2)),
        ]),
    ])
    .simplify();

    if denom_expr.is_zero() {
        return None;
    }

    let a = Expression::mul(vec![
        Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), coeff_b]),
            Expression::mul(vec![Expression::integer(-1), coeff_a.clone(), p.clone()]),
        ]),
        Expression::pow(denom_expr, Expression::integer(-1)),
    ])
    .simplify();

    let b = Expression::mul(vec![
        Expression::rational(1, 2),
        Expression::add(vec![
            Expression::mul(vec![a.clone(), p.clone()]),
            Expression::mul(vec![Expression::integer(-1), coeff_a]),
        ]),
    ])
    .simplify();

    let h_coeff = a.clone();

    let g = Expression::add(vec![
        Expression::mul(vec![a, Expression::symbol(var.clone())]),
        b,
    ]);

    let rational_part = Expression::mul(vec![g, Expression::pow(d, Expression::integer(-1))]);

    let transcendental_part = integrate_simple_quadratic(&h_coeff, denominator, p, q, var)?;

    Some(Expression::add(vec![rational_part, transcendental_part]).simplify())
}

/// Extract coefficients A and B from linear expression `Ax+B`
///
/// # Arguments
///
/// * `expr` - Expression to analyze
/// * `var` - Variable to extract coefficient for
///
/// # Returns
///
/// `Some((A, B))` if expression is linear, `None` otherwise
///
/// # Examples
///
/// ```
/// use mathhook_core::{Expression, symbol};
/// use mathhook_core::calculus::integrals::rational::quadratic::extract_linear_coefficients;
///
/// let x = symbol!(x);
///
/// let expr1 = Expression::add(vec![
///     Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
///     Expression::integer(5),
/// ]);
/// let (a, b) = extract_linear_coefficients(&expr1, &x).unwrap();
///
/// let expr2 = Expression::integer(7);
/// let (a2, b2) = extract_linear_coefficients(&expr2, &x).unwrap();
/// ```
pub fn extract_linear_coefficients(
    expr: &Expression,
    var: &Symbol,
) -> Option<(Expression, Expression)> {
    match expr {
        Expression::Number(_) => Some((Expression::integer(0), expr.clone())),

        Expression::Symbol(s) if *s == *var => {
            Some((Expression::integer(1), Expression::integer(0)))
        }

        Expression::Mul(factors) => {
            let mut coeff = Vec::new();
            let mut has_var = false;

            for factor in factors.iter() {
                if let Expression::Symbol(s) = factor {
                    if *s == *var {
                        has_var = true;
                        continue;
                    }
                }
                coeff.push(factor.clone());
            }

            if has_var {
                let a = if coeff.is_empty() {
                    Expression::integer(1)
                } else {
                    Expression::mul(coeff)
                };
                Some((a, Expression::integer(0)))
            } else {
                None
            }
        }

        Expression::Add(terms) => {
            let mut a = Expression::integer(0);
            let mut b = Expression::integer(0);

            for term in terms.iter() {
                match term {
                    Expression::Symbol(s) if *s == *var => {
                        a = Expression::add(vec![a, Expression::integer(1)]);
                    }
                    Expression::Mul(_) => {
                        if term.contains_variable(var) {
                            let (term_a, term_b) = extract_linear_coefficients(term, var)?;
                            a = Expression::add(vec![a, term_a]);
                            b = Expression::add(vec![b, term_b]);
                        } else {
                            b = Expression::add(vec![b, term.clone()]);
                        }
                    }
                    _ => {
                        if !term.contains_variable(var) {
                            b = Expression::add(vec![b, term.clone()]);
                        } else {
                            return None;
                        }
                    }
                }
            }

            Some((a.simplify(), b.simplify()))
        }

        _ => None,
    }
}

/// Integrate simple irreducible quadratic `(Bx+C)/(x²+px+q)`
///
/// # Mathematical Method
///
/// For `x²+px+q` with discriminant `Δ = p²-4q < 0` (irreducible):
///
/// 1. Complete the square: `x²+px+q = (x+p/2)² + a²` where `a² = q - p²/4`
/// 2. Split integral: `∫(Bx+C)/(x²+px+q) dx = B∫x/(x²+px+q) dx + C∫1/(x²+px+q) dx`
/// 3. Logarithmic part: `∫x/(x²+px+q) dx = (1/2)ln|x²+px+q| - (p/2)∫1/(x²+px+q) dx`
/// 4. Arctangent part: `∫1/(x²+px+q) dx = (1/a)arctan((x+p/2)/a)`
///
/// # Current Implementation
///
/// Simplified version assumes constant numerator (B=0, C=1).
///
/// # Arguments
///
/// * `_numerator` - Numerator (currently unused, assumes 1)
/// * `_denominator` - Full denominator (for reference)
/// * `p` - Linear coefficient in `x²+px+q`
/// * `q` - Constant term in `x²+px+q`
/// * `var` - Integration variable
///
/// # Returns
///
/// Integrated expression or `None` if quadratic is not irreducible
///
/// # Examples
///
/// ```
/// use mathhook_core::{Expression, symbol};
/// use mathhook_core::calculus::integrals::rational::quadratic::integrate_simple_quadratic;
///
/// let x = symbol!(x);
/// let numerator = Expression::integer(1);
/// let p = Expression::integer(0);
/// let q = Expression::integer(1);
/// let denominator = Expression::add(vec![
///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
///     q.clone(),
/// ]);
///
/// let result = integrate_simple_quadratic(&numerator, &denominator, &p, &q, &x);
/// assert!(result.is_some());
/// ```
pub fn integrate_simple_quadratic(
    _numerator: &Expression,
    _denominator: &Expression,
    p: &Expression,
    q: &Expression,
    var: &Symbol,
) -> Option<Expression> {
    let discriminant = Expression::add(vec![
        Expression::pow(p.clone(), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(-4), q.clone()]),
    ])
    .simplify();

    if let Expression::Number(Number::Integer(d)) = discriminant {
        if d >= 0 {
            return None;
        }
    }

    let a_squared = Expression::add(vec![
        q.clone(),
        Expression::mul(vec![
            Expression::rational(-1, 4),
            Expression::pow(p.clone(), Expression::integer(2)),
        ]),
    ])
    .simplify();

    let a = Expression::function("sqrt", vec![a_squared]);

    let shift = Expression::mul(vec![Expression::rational(1, 2), p.clone()]);
    let x_shifted = Expression::add(vec![Expression::symbol(var.clone()), shift]).simplify();

    Some(Expression::mul(vec![
        Expression::pow(a.clone(), Expression::integer(-1)),
        Expression::function(
            "atan",
            vec![Expression::mul(vec![
                Expression::pow(a, Expression::integer(-1)),
                x_shifted,
            ])],
        ),
    ]))
}
