//! Integration of linear factors in partial fraction decomposition
//!
//! Implements Heaviside's method for computing coefficients of repeated linear factors.

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;

use super::helpers::{factorial, substitute_variable};

/// Integrate linear partial fraction using Heaviside's method
///
/// For simple pole (power=1): Uses cover-up method
/// For repeated pole (power>1): Uses Heaviside's method with derivatives
///
/// # Mathematical Basis
///
/// For `P(x)/Q(x)` with `Q(x) = (x-r)^n · R(x)`, we want:
/// ```text
/// P(x)/Q(x) = A₁/(x-r) + A₂/(x-r)² + ... + Aₙ/(x-r)ⁿ + (other terms)
/// ```
///
/// Heaviside's method: Define `g(x) = (x-r)ⁿ · P(x)/Q(x) = P(x)/R(x)`
/// Then: `Aₖ = (1/(n-k)!) · [d^(n-k)/dx^(n-k) g(x)]|ₓ₌ᵣ`
///
/// # Integration Formulas
///
/// - `∫A/(x-r) dx = A·ln|x-r| + C`
/// - `∫A/(x-r)ⁿ dx = -A/((n-1)(x-r)^(n-1)) + C` for `n > 1`
///
/// # Arguments
///
/// * `numerator` - Numerator polynomial `P(x)`
/// * `denominator` - Full denominator `Q(x)`
/// * `root` - The root `r` of the linear factor
/// * `power` - Multiplicity `n` of the root
/// * `var` - Integration variable
///
/// # Returns
///
/// Integrated expression or `None` if integration fails
///
/// # Examples
///
/// ```
/// use mathhook_core::{Expression, symbol};
/// use mathhook_core::calculus::integrals::rational::linear::integrate_linear_factor;
/// use mathhook_core::simplify::Simplify;
///
/// let x = symbol!(x);
///
/// let numerator = Expression::integer(1);
/// let denominator = Expression::pow(
///     Expression::add(vec![
///         Expression::symbol(x.clone()),
///         Expression::integer(-2),
///     ]),
///     Expression::integer(2),
/// );
/// let root = Expression::integer(2);
///
/// let result = integrate_linear_factor(&numerator, &denominator, &root, 2, &x);
/// assert!(result.is_some());
/// ```
pub fn integrate_linear_factor(
    numerator: &Expression,
    denominator: &Expression,
    root: &Expression,
    power: i64,
    var: &Symbol,
) -> Option<Expression> {
    let mut result = Expression::integer(0);

    for k in 1..=power {
        let coeff = if power == 1 {
            substitute_variable(numerator, var, root).simplify()
        } else {
            compute_heaviside_coefficient(numerator, denominator, root, power, k, var)?
        };

        let x_minus_r = Expression::add(vec![
            Expression::symbol(var.clone()),
            Expression::mul(vec![Expression::integer(-1), root.clone()]),
        ]);

        let term = if k == 1 {
            Expression::mul(vec![
                coeff,
                Expression::function("ln", vec![Expression::function("abs", vec![x_minus_r])]),
            ])
        } else {
            Expression::mul(vec![
                Expression::integer(-1),
                coeff,
                Expression::rational(1, k - 1),
                Expression::pow(x_minus_r, Expression::integer(-(k - 1))),
            ])
        };

        result = Expression::add(vec![result, term]);
    }

    Some(result)
}

/// Compute Heaviside coefficient for repeated linear factors
///
/// For `(x-r)^n` in denominator, coefficient k is:
/// ```text
/// Aₖ = (1/(n-k)!) · [d^(n-k)/dx^(n-k) of g(x)]|ₓ₌ᵣ
/// ```
/// where `g(x) = (x-r)^n · P(x)/Q(x)`
///
/// # Arguments
///
/// * `numerator` - Numerator polynomial `P(x)`
/// * `denominator` - Full denominator `Q(x)`
/// * `root` - The root `r`
/// * `total_power` - Multiplicity `n`
/// * `k` - Coefficient index (1 to n)
/// * `var` - Integration variable
///
/// # Returns
///
/// The coefficient `Aₖ` or `None` if computation fails
///
/// # Examples
///
/// ```
/// use mathhook_core::{Expression, symbol};
/// use mathhook_core::calculus::integrals::rational::linear::compute_heaviside_coefficient;
///
/// let x = symbol!(x);
/// let numerator = Expression::integer(1);
/// let denominator = Expression::pow(
///     Expression::add(vec![
///         Expression::symbol(x.clone()),
///         Expression::integer(-1),
///     ]),
///     Expression::integer(3),
/// );
/// let root = Expression::integer(1);
///
/// let coeff = compute_heaviside_coefficient(&numerator, &denominator, &root, 3, 1, &x);
/// assert!(coeff.is_some());
/// ```
pub fn compute_heaviside_coefficient(
    numerator: &Expression,
    denominator: &Expression,
    root: &Expression,
    total_power: i64,
    k: i64,
    var: &Symbol,
) -> Option<Expression> {
    let x_minus_r = Expression::add(vec![
        Expression::symbol(var.clone()),
        Expression::mul(vec![Expression::integer(-1), root.clone()]),
    ]);
    let x_minus_r_pow_n = Expression::pow(x_minus_r, Expression::integer(total_power));

    let g = Expression::mul(vec![
        x_minus_r_pow_n,
        Expression::mul(vec![
            numerator.clone(),
            Expression::pow(denominator.clone(), Expression::integer(-1)),
        ]),
    ])
    .simplify();

    let derivative_order = total_power - k;

    let derivative_result = if derivative_order == 0 {
        g
    } else {
        g.nth_derivative(var.clone(), derivative_order as u32)
    };

    let evaluated = substitute_variable(&derivative_result, var, root).simplify();

    let fact = factorial(derivative_order);
    Some(Expression::mul(vec![
        Expression::rational(1, fact),
        evaluated,
    ]))
}
