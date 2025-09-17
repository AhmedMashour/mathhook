//! Product-to-sum integration for trigonometric functions
//!
//! Handles integration of products like sin(mx)*cos(nx), sin(mx)*sin(nx), cos(mx)*cos(nx)
//! using product-to-sum formulas.

use crate::core::{Expression, Symbol};

/// Integrate products like sin(mx)*cos(nx), sin(mx)*sin(nx), cos(mx)*cos(nx)
///
/// Uses product-to-sum formulas:
/// - sin(mx)cos(nx) = [sin((m-n)x) + sin((m+n)x)]/2
/// - sin(mx)sin(nx) = [cos((m-n)x) - cos((m+n)x)]/2
/// - cos(mx)cos(nx) = [cos((m-n)x) + cos((m+n)x)]/2
///
/// # Mathematical Background
///
/// Product-to-sum identities convert products into sums for easier integration:
///
/// **Formula for sin(mx)cos(nx):**
/// - If m ≠ n: ∫sin(mx)cos(nx) dx = -cos((m-n)x)/(2(m-n)) - cos((m+n)x)/(2(m+n))
/// - If m = n: ∫sin(mx)cos(mx) dx = sin²(mx)/(2m) = x/2 - sin(2mx)/(4m)
///
/// **Formula for sin(mx)sin(nx):**
/// - If m ≠ n: ∫sin(mx)sin(nx) dx = sin((m-n)x)/(2(m-n)) - sin((m+n)x)/(2(m+n))
/// - If m = n: ∫sin²(mx) dx = x/2 - sin(2mx)/(4m)
///
/// **Formula for cos(mx)cos(nx):**
/// - If m ≠ n: ∫cos(mx)cos(nx) dx = sin((m-n)x)/(2(m-n)) + sin((m+n)x)/(2(m+n))
/// - If m = n: ∫cos²(mx) dx = x/2 + sin(2mx)/(4m)
///
/// # Domain
/// m, n ∈ ℤ (integer frequencies), m, n ≠ 0
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::integrals::trigonometric::integrate_trig_product;
/// use mathhook_core::symbol;
/// use mathhook_core::core::Expression;
///
/// let x = symbol!(x);
/// // ∫sin(2x)cos(3x) dx
/// let result = integrate_trig_product("sin", 2, "cos", 3, x.clone());
/// assert!(result.is_some());
/// ```
///
/// # References
/// Validated against SymPy integration results (January 2025)
///
/// # Architectural Note
///
/// Uses hardcoded function name matching ("sin", "cos") for product pattern dispatch.
/// This is acceptable because:
/// 1. Product-to-sum formulas are specific to sin/cos (not extensible to user functions)
/// 2. Pattern matching is O(1) vs registry lookup overhead
/// 3. Mathematical formulas are fixed (sin/cos product identities are not parameterizable)
pub fn integrate_trig_product(
    func1: &str,
    m: i64,
    func2: &str,
    n: i64,
    var: Symbol,
) -> Option<Expression> {
    let x = Expression::symbol(var);

    match (func1, func2) {
        ("sin", "cos") => Some(integrate_sin_cos_product(m, n, x)),
        ("cos", "sin") => Some(integrate_sin_cos_product(n, m, x)),
        ("sin", "sin") => Some(integrate_sin_sin_product(m, n, x)),
        ("cos", "cos") => Some(integrate_cos_cos_product(m, n, x)),
        _ => None,
    }
}

/// Integrate sin(mx)*cos(nx)
///
/// # Formula
///
/// If m = n: ∫sin(mx)cos(mx) dx = -cos(2mx)/(4m)
/// If m ≠ n: ∫sin(mx)cos(nx) dx = -cos((m-n)x)/(2(m-n)) - cos((m+n)x)/(2(m+n))
fn integrate_sin_cos_product(m: i64, n: i64, x: Expression) -> Expression {
    if m == n {
        Expression::mul(vec![
            Expression::rational(-1, 4 * m),
            Expression::function(
                "cos",
                vec![Expression::mul(vec![Expression::integer(2 * m), x])],
            ),
        ])
    } else {
        let m_minus_n = m - n;
        let m_plus_n = m + n;

        Expression::add(vec![
            Expression::mul(vec![
                Expression::rational(-1, 2 * m_minus_n),
                Expression::function(
                    "cos",
                    vec![Expression::mul(vec![
                        Expression::integer(m_minus_n),
                        x.clone(),
                    ])],
                ),
            ]),
            Expression::mul(vec![
                Expression::rational(-1, 2 * m_plus_n),
                Expression::function(
                    "cos",
                    vec![Expression::mul(vec![Expression::integer(m_plus_n), x])],
                ),
            ]),
        ])
    }
}

/// Integrate sin(mx)*sin(nx)
///
/// # Formula
///
/// If m = n: ∫sin²(mx) dx = x/2 - sin(2mx)/(4m)
/// If m ≠ n: ∫sin(mx)sin(nx) dx = sin((m-n)x)/(2(m-n)) - sin((m+n)x)/(2(m+n))
fn integrate_sin_sin_product(m: i64, n: i64, x: Expression) -> Expression {
    if m == n {
        Expression::add(vec![
            Expression::mul(vec![Expression::rational(1, 2), x.clone()]),
            Expression::mul(vec![
                Expression::rational(-1, 4 * m),
                Expression::function(
                    "sin",
                    vec![Expression::mul(vec![Expression::integer(2 * m), x])],
                ),
            ]),
        ])
    } else {
        let m_minus_n = m - n;
        let m_plus_n = m + n;

        Expression::add(vec![
            Expression::mul(vec![
                Expression::rational(1, 2 * m_minus_n),
                Expression::function(
                    "sin",
                    vec![Expression::mul(vec![
                        Expression::integer(m_minus_n),
                        x.clone(),
                    ])],
                ),
            ]),
            Expression::mul(vec![
                Expression::rational(-1, 2 * m_plus_n),
                Expression::function(
                    "sin",
                    vec![Expression::mul(vec![Expression::integer(m_plus_n), x])],
                ),
            ]),
        ])
    }
}

/// Integrate cos(mx)*cos(nx)
///
/// # Formula
///
/// If m = n: ∫cos²(mx) dx = x/2 + sin(2mx)/(4m)
/// If m ≠ n: ∫cos(mx)cos(nx) dx = sin((m-n)x)/(2(m-n)) + sin((m+n)x)/(2(m+n))
fn integrate_cos_cos_product(m: i64, n: i64, x: Expression) -> Expression {
    if m == n {
        Expression::add(vec![
            Expression::mul(vec![Expression::rational(1, 2), x.clone()]),
            Expression::mul(vec![
                Expression::rational(1, 4 * m),
                Expression::function(
                    "sin",
                    vec![Expression::mul(vec![Expression::integer(2 * m), x])],
                ),
            ]),
        ])
    } else {
        let m_minus_n = m - n;
        let m_plus_n = m + n;

        Expression::add(vec![
            Expression::mul(vec![
                Expression::rational(1, 2 * m_minus_n),
                Expression::function(
                    "sin",
                    vec![Expression::mul(vec![
                        Expression::integer(m_minus_n),
                        x.clone(),
                    ])],
                ),
            ]),
            Expression::mul(vec![
                Expression::rational(1, 2 * m_plus_n),
                Expression::function(
                    "sin",
                    vec![Expression::mul(vec![Expression::integer(m_plus_n), x])],
                ),
            ]),
        ])
    }
}
