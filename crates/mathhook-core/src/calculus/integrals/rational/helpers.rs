//! Helper utilities for rational function integration

use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;

/// Check if expression is a polynomial in the given variable
///
/// # Arguments
///
/// * `expr` - The expression to check
/// * `var` - The variable
///
/// # Examples
///
/// ```
/// use mathhook_core::{Expression, symbol};
/// use mathhook_core::calculus::integrals::rational::helpers::is_polynomial;
///
/// let x = symbol!(x);
/// let poly = Expression::add(vec![
///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
///     Expression::symbol(x.clone()),
///     Expression::integer(1),
/// ]);
///
/// assert!(is_polynomial(&poly, &x));
///
/// let non_poly = Expression::function("sin", vec![Expression::symbol(x.clone())]);
/// assert!(!is_polynomial(&non_poly, &x));
/// ```
pub fn is_polynomial(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Number(_) => true,
        Expression::Symbol(s) => s == var || !expr.contains_variable(var),
        Expression::Add(terms) => terms.iter().all(|t| is_polynomial(t, var)),
        Expression::Mul(factors) => factors.iter().all(|f| is_polynomial(f, var)),
        Expression::Pow(base, exp) => {
            if let Expression::Symbol(s) = base.as_ref() {
                if s == var {
                    matches!(exp.as_ref(), Expression::Number(Number::Integer(n)) if *n >= 0)
                } else {
                    is_polynomial(base, var)
                }
            } else {
                is_polynomial(base, var)
                    && matches!(exp.as_ref(), Expression::Number(Number::Integer(n)) if *n >= 0)
            }
        }
        _ => false,
    }
}

/// Get polynomial degree with respect to a variable
///
/// # Arguments
///
/// * `expr` - The polynomial expression
/// * `var` - The variable
///
/// # Examples
///
/// ```
/// use mathhook_core::{Expression, symbol};
/// use mathhook_core::calculus::integrals::rational::helpers::polynomial_degree;
///
/// let x = symbol!(x);
/// let cubic = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
/// assert_eq!(polynomial_degree(&cubic, &x), 3);
///
/// let linear = Expression::symbol(x.clone());
/// assert_eq!(polynomial_degree(&linear, &x), 1);
///
/// let constant = Expression::integer(5);
/// assert_eq!(polynomial_degree(&constant, &x), 0);
/// ```
pub fn polynomial_degree(expr: &Expression, var: &Symbol) -> i64 {
    match expr {
        Expression::Symbol(s) if s == var => 1,
        Expression::Number(_) => 0,
        Expression::Pow(base, exp) => {
            if let (Expression::Symbol(s), Expression::Number(Number::Integer(e))) =
                (base.as_ref(), exp.as_ref())
            {
                if s == var {
                    return *e;
                }
            }
            0
        }
        Expression::Add(terms) => terms
            .iter()
            .map(|t| polynomial_degree(t, var))
            .max()
            .unwrap_or(0),
        Expression::Mul(factors) => factors.iter().map(|f| polynomial_degree(f, var)).sum(),
        _ => 0,
    }
}

/// Substitute a value for a variable in an expression
///
/// # Arguments
///
/// * `expr` - The expression
/// * `var` - The variable to substitute
/// * `value` - The value to substitute
///
/// # Examples
///
/// ```
/// use mathhook_core::{Expression, symbol};
/// use mathhook_core::calculus::integrals::rational::helpers::substitute_variable;
/// use mathhook_core::simplify::Simplify;
///
/// let x = symbol!(x);
/// let expr = Expression::add(vec![
///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
///     Expression::integer(1),
/// ]);
///
/// let result = substitute_variable(&expr, &x, &Expression::integer(3));
/// assert_eq!(result.simplify(), Expression::integer(10));
/// ```
pub fn substitute_variable(expr: &Expression, var: &Symbol, value: &Expression) -> Expression {
    match expr {
        Expression::Symbol(s) if s == var => value.clone(),
        Expression::Add(terms) => Expression::add(
            terms
                .iter()
                .map(|t| substitute_variable(t, var, value))
                .collect(),
        ),
        Expression::Mul(factors) => Expression::mul(
            factors
                .iter()
                .map(|f| substitute_variable(f, var, value))
                .collect(),
        ),
        Expression::Pow(base, exp) => Expression::pow(
            substitute_variable(base, var, value),
            substitute_variable(exp, var, value),
        ),
        Expression::Function { name, args } => Expression::function(
            name,
            args.iter()
                .map(|a| substitute_variable(a, var, value))
                .collect(),
        ),
        _ => expr.clone(),
    }
}

/// Compute factorial of a non-negative integer
///
/// # Arguments
///
/// * `n` - Non-negative integer
///
/// # Examples
///
/// ```
/// use mathhook_core::calculus::integrals::rational::helpers::factorial;
///
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(5), 120);
/// assert_eq!(factorial(10), 3628800);
/// ```
pub fn factorial(n: i64) -> i64 {
    if n <= 0 {
        1
    } else {
        (1..=n).product()
    }
}

/// Try to extract quadratic coefficients from x² + px + q
///
/// Returns Some((p, q)) if expression matches x² + px + q pattern where the
/// coefficient of x² is exactly 1.
///
/// # Arguments
///
/// * `expr` - The expression to analyze
/// * `var` - The variable to match against
///
/// # Examples
///
/// ```
/// use mathhook_core::{Expression, symbol};
/// use mathhook_core::calculus::integrals::rational::helpers::try_extract_quadratic;
///
/// let x = symbol!(x);
///
/// let quadratic = Expression::add(vec![
///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
///     Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
///     Expression::integer(1),
/// ]);
///
/// let result = try_extract_quadratic(&quadratic, &x);
/// assert!(result.is_some());
/// let (p, q) = result.unwrap();
/// assert_eq!(p, Expression::integer(2));
/// assert_eq!(q, Expression::integer(1));
/// ```
pub fn try_extract_quadratic(expr: &Expression, var: &Symbol) -> Option<(Expression, Expression)> {
    if let Expression::Add(terms) = expr {
        let mut x_squared_coeff = None;
        let mut x_coeff = Expression::integer(0);
        let mut constant = Expression::integer(0);

        for term in terms.iter() {
            match term {
                Expression::Pow(base, exp) => {
                    if let (Expression::Symbol(s), Expression::Number(Number::Integer(2))) =
                        (base.as_ref(), exp.as_ref())
                    {
                        if *s == *var && x_squared_coeff.is_none() {
                            x_squared_coeff = Some(Expression::integer(1));
                            continue;
                        }
                    }
                    return None;
                }
                Expression::Mul(factors) => {
                    let mut has_x_squared = false;
                    let mut has_x = false;
                    let mut coeff = Expression::integer(1);

                    for factor in factors.iter() {
                        if let Expression::Pow(base, exp) = factor {
                            if let (Expression::Symbol(s), Expression::Number(Number::Integer(2))) =
                                (base.as_ref(), exp.as_ref())
                            {
                                if *s == *var {
                                    has_x_squared = true;
                                    continue;
                                }
                            }
                        }
                        if let Expression::Symbol(s) = factor {
                            if *s == *var {
                                has_x = true;
                                continue;
                            }
                        }
                        coeff = Expression::mul(vec![coeff, factor.clone()]);
                    }

                    if has_x_squared && x_squared_coeff.is_none() {
                        x_squared_coeff = Some(coeff);
                    } else if has_x {
                        x_coeff = Expression::add(vec![x_coeff, coeff]);
                    } else {
                        constant = Expression::add(vec![constant, term.clone()]);
                    }
                }
                Expression::Symbol(s) if *s == *var => {
                    x_coeff = Expression::add(vec![x_coeff, Expression::integer(1)]);
                }
                _ => {
                    constant = Expression::add(vec![constant, term.clone()]);
                }
            }
        }

        if x_squared_coeff == Some(Expression::integer(1)) {
            return Some((x_coeff.simplify(), constant.simplify()));
        }
    }

    None
}
