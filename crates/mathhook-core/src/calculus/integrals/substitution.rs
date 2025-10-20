//! U-substitution for integration
//!
//! Implements automatic u-substitution detection and execution for composite
//! functions. Handles patterns like f'(g(x)) * g'(x) by substituting u = g(x).
//!
//! # Algorithm
//!
//! 1. Identify candidate substitutions u = g(x) from the integrand structure
//! 2. Compute du = g'(x) dx for each candidate
//! 3. Check if integrand can be rewritten as f(u) * du (possibly with constant factor)
//! 4. Integrate f(u) with respect to u
//! 5. Substitute back u = g(x) in the result
//!
//! # Supported Patterns
//!
//! - Polynomial inner functions: ∫2x*sin(x²) dx = -cos(x²)
//! - Exponential compositions: ∫e^x*sin(e^x) dx = -cos(e^x)
//! - Logarithmic patterns: ∫1/(x*ln(x)) dx = ln|ln(x)|
//! - Rational functions: ∫x/(x²+1) dx = (1/2)*ln(x²+1)

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;

/// Try to integrate using u-substitution
///
/// Automatically detects composite function patterns and applies substitution.
///
/// # Arguments
///
/// * `expr` - The integrand expression
/// * `var` - The variable of integration
///
/// # Returns
///
/// Some(result) if substitution succeeds, None if no suitable substitution found
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::integrals::substitution::try_substitution;
/// use mathhook_core::symbol;
/// use mathhook_core::core::Expression;
///
/// let x = symbol!(x);
/// // ∫2x*sin(x²) dx
/// let integrand = Expression::mul(vec![
///     Expression::integer(2),
///     Expression::symbol(x.clone()),
///     Expression::function("sin", vec![
///         Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))
///     ])
/// ]);
///
/// let result = try_substitution(&integrand, x);
/// assert!(result.is_some());
/// ```
pub fn try_substitution(expr: &Expression, var: Symbol) -> Option<Expression> {
    let candidates = find_substitution_candidates(expr, &var);

    for candidate in candidates {
        let g_prime = candidate.derivative(var.clone());

        if let Some((f_of_u, constant_factor)) =
            check_derivative_match(expr, &candidate, &g_prime, &var)
        {
            let u_symbol = Symbol::scalar("u");
            let u_expr = Expression::symbol(u_symbol.clone());

            let integrated = integrate_in_u(&f_of_u, u_symbol)?;

            let result = substitute_back(&integrated, &u_expr, &candidate);

            let final_result = if (constant_factor - 1.0).abs() > 1e-10 {
                if constant_factor.abs() < 1.0 {
                    let denom = (1.0 / constant_factor) as i64;
                    Expression::mul(vec![Expression::rational(1, denom), result])
                } else {
                    let numer = constant_factor as i64;
                    Expression::mul(vec![Expression::integer(numer), result])
                }
            } else {
                result
            };

            return Some(final_result);
        }
    }

    None
}

/// Find candidate expressions for substitution u = g(x)
///
/// Looks for inner functions, polynomial expressions, exponential/logarithm arguments.
fn find_substitution_candidates(expr: &Expression, var: &Symbol) -> Vec<Expression> {
    let mut candidates = Vec::new();

    collect_candidates_recursive(expr, var, &mut candidates);

    candidates.sort_by_key(|c| std::cmp::Reverse(expression_complexity(c)));
    candidates.dedup_by(|a, b| expressions_equivalent(a, b));

    candidates
}

/// Recursively collect substitution candidates from expression tree
fn collect_candidates_recursive(
    expr: &Expression,
    var: &Symbol,
    candidates: &mut Vec<Expression>,
) {
    match expr {
        Expression::Function { name: _, args } => {
            for arg in args.iter() {
                if contains_variable(arg, var) && !is_simple_variable(arg, var) {
                    candidates.push(arg.clone());
                }
                collect_candidates_recursive(arg, var, candidates);
            }
        }
        Expression::Pow(base, exp) => {
            if contains_variable(base, var) && !is_simple_variable(base, var) {
                candidates.push((**base).clone());
            }
            if contains_variable(exp, var) && !is_simple_variable(exp, var) {
                candidates.push((**exp).clone());
            }
            collect_candidates_recursive(base, var, candidates);
            collect_candidates_recursive(exp, var, candidates);
        }
        Expression::Add(terms) => {
            for term in terms.iter() {
                collect_candidates_recursive(term, var, candidates);
            }
        }
        Expression::Mul(factors) => {
            for factor in factors.iter() {
                collect_candidates_recursive(factor, var, candidates);
            }
        }
        _ => {}
    }
}

/// Check if expression contains the given variable
fn contains_variable(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Symbol(s) => s == var,
        Expression::Add(terms) => terms.iter().any(|t| contains_variable(t, var)),
        Expression::Mul(factors) => factors.iter().any(|f| contains_variable(f, var)),
        Expression::Pow(base, exp) => contains_variable(base, var) || contains_variable(exp, var),
        Expression::Function { name: _, args } => args.iter().any(|a| contains_variable(a, var)),
        _ => false,
    }
}

/// Check if expression is just the variable itself
fn is_simple_variable(expr: &Expression, var: &Symbol) -> bool {
    matches!(expr, Expression::Symbol(s) if s == var)
}

/// Measure complexity of expression (for prioritizing candidates)
fn expression_complexity(expr: &Expression) -> usize {
    match expr {
        Expression::Number(_) | Expression::Symbol(_) | Expression::Constant(_) => 1,
        Expression::Add(terms) => terms.iter().map(expression_complexity).sum::<usize>() + 1,
        Expression::Mul(factors) => factors.iter().map(expression_complexity).sum::<usize>() + 1,
        Expression::Pow(base, exp) => expression_complexity(base) + expression_complexity(exp) + 1,
        Expression::Function { name: _, args } => {
            args.iter().map(expression_complexity).sum::<usize>() + 2
        }
        _ => 1,
    }
}

/// Check if two expressions are equivalent
fn expressions_equivalent(a: &Expression, b: &Expression) -> bool {
    a == b
}

/// Check if derivative g'(x) appears in the integrand
///
/// Returns Some((f(u), constant_factor)) if a match is found, where:
/// - f(u) is the expression in terms of u
/// - constant_factor accounts for numerical differences between g'(x) and actual factor
fn check_derivative_match(
    expr: &Expression,
    g: &Expression,
    g_prime: &Expression,
    var: &Symbol,
) -> Option<(Expression, f64)> {
    let expr_simplified = expr.clone().simplify();
    let g_prime_simplified = g_prime.clone().simplify();

    if let Expression::Mul(factors) = &expr_simplified {
        let u_symbol = Symbol::scalar("u");
        let u_expr = Expression::symbol(u_symbol.clone());

        let (derivative_factors, other_factors): (Vec<_>, Vec<_>) = factors
            .iter()
            .partition(|f| factor_matches_derivative(f, &g_prime_simplified, var));

        if derivative_factors.is_empty() {
            return None;
        }

        let derivative_product = if derivative_factors.len() == 1 {
            derivative_factors[0].clone()
        } else {
            Expression::mul(derivative_factors.iter().map(|f| (*f).clone()).collect())
        };

        let constant_factor = compute_constant_ratio(&derivative_product, &g_prime_simplified)?;

        let remaining = if other_factors.is_empty() {
            Expression::integer(1)
        } else if other_factors.len() == 1 {
            other_factors[0].clone()
        } else {
            Expression::mul(other_factors.iter().map(|f| (*f).clone()).collect())
        };

        let f_of_u = substitute_back(&remaining, g, &u_expr);

        Some((f_of_u, constant_factor))
    } else {
        let constant_factor = compute_constant_ratio(&expr_simplified, &g_prime_simplified)?;
        let f_of_u = Expression::integer(1);
        Some((f_of_u, constant_factor))
    }
}

/// Check if a factor matches the derivative (possibly with constant multiple)
fn factor_matches_derivative(factor: &Expression, derivative: &Expression, var: &Symbol) -> bool {
    if factor == derivative {
        return true;
    }

    let factor_simplified = factor.clone().simplify();
    let derivative_simplified = derivative.clone().simplify();

    if factor_simplified == derivative_simplified {
        return true;
    }

    if let (Expression::Mul(f_factors), Expression::Mul(d_factors)) =
        (&factor_simplified, &derivative_simplified)
    {
        let f_non_const: Vec<_> = f_factors
            .iter()
            .filter(|f| contains_variable(f, var))
            .collect();
        let d_non_const: Vec<_> = d_factors
            .iter()
            .filter(|f| contains_variable(f, var))
            .collect();

        if f_non_const.len() == d_non_const.len() {
            return f_non_const
                .iter()
                .zip(d_non_const.iter())
                .all(|(f, d)| f == d);
        }
    }

    match (&factor_simplified, &derivative_simplified) {
        (Expression::Symbol(f_sym), Expression::Symbol(d_sym)) => f_sym == d_sym,
        (Expression::Pow(f_base, f_exp), Expression::Pow(d_base, d_exp)) => {
            f_base == d_base && f_exp == d_exp
        }
        _ => false,
    }
}

/// Compute constant ratio between two expressions
fn compute_constant_ratio(expr: &Expression, target: &Expression) -> Option<f64> {
    if expr == target {
        return Some(1.0);
    }

    match (expr, target) {
        (Expression::Number(n1), Expression::Number(n2)) => {
            let v1 = number_to_f64(n1)?;
            let v2 = number_to_f64(n2)?;
            if v2.abs() > 1e-10 {
                Some(v1 / v2)
            } else {
                None
            }
        }
        (Expression::Mul(factors), _) => {
            let (constants, rest): (Vec<_>, Vec<_>) =
                factors.iter().partition(|f| matches!(f, Expression::Number(_)));

            let constant_product = if constants.is_empty() {
                1.0
            } else {
                constants
                    .iter()
                    .filter_map(|c| {
                        if let Expression::Number(n) = c {
                            number_to_f64(n)
                        } else {
                            None
                        }
                    })
                    .product()
            };

            let remaining = if rest.is_empty() {
                Expression::integer(1)
            } else if rest.len() == 1 {
                rest[0].clone()
            } else {
                Expression::mul(rest.iter().map(|f| (*f).clone()).collect())
            };

            if remaining == *target || remaining.clone().simplify() == target.clone().simplify() {
                Some(constant_product)
            } else {
                None
            }
        }
        _ => {
            if expr.clone().simplify() == target.clone().simplify() {
                Some(1.0)
            } else {
                None
            }
        }
    }
}

/// Convert Number to f64
fn number_to_f64(num: &Number) -> Option<f64> {
    match num {
        Number::Integer(i) => Some(*i as f64),
        Number::Rational(r) => {
            use num_traits::ToPrimitive;
            r.to_f64()
        }
        Number::Float(f) => Some(*f),
        _ => None,
    }
}

/// Integrate expression with respect to u
fn integrate_in_u(expr: &Expression, u: Symbol) -> Option<Expression> {
    use crate::calculus::integrals::strategy::integrate_with_strategy;
    let result = integrate_with_strategy(expr, u);

    if matches!(result, Expression::Calculus(_)) {
        None
    } else {
        Some(result)
    }
}

/// Substitute u = g(x) back into the result
fn substitute_back(expr: &Expression, u: &Expression, g: &Expression) -> Expression {
    match expr {
        Expression::Symbol(s) => {
            if Expression::symbol(s.clone()) == *u {
                g.clone()
            } else {
                expr.clone()
            }
        }
        Expression::Add(terms) => {
            Expression::add(terms.iter().map(|t| substitute_back(t, u, g)).collect())
        }
        Expression::Mul(factors) => {
            Expression::mul(factors.iter().map(|f| substitute_back(f, u, g)).collect())
        }
        Expression::Pow(base, exp) => Expression::pow(
            substitute_back(base, u, g),
            substitute_back(exp, u, g),
        ),
        Expression::Function { name, args } => Expression::function(
            name,
            args.iter().map(|a| substitute_back(a, u, g)).collect(),
        ),
        _ => expr.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_contains_variable() {
        let x = symbol!(x);
        let y = symbol!(y);

        assert!(contains_variable(&Expression::symbol(x.clone()), &x));
        assert!(!contains_variable(&Expression::symbol(y.clone()), &x));
        assert!(!contains_variable(&Expression::integer(5), &x));
    }

    #[test]
    fn test_is_simple_variable() {
        let x = symbol!(x);

        assert!(is_simple_variable(&Expression::symbol(x.clone()), &x));
        assert!(!is_simple_variable(&Expression::integer(5), &x));
        assert!(!is_simple_variable(
            &Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            &x
        ));
    }

    #[test]
    fn test_expression_complexity() {
        let x = symbol!(x);

        assert_eq!(expression_complexity(&Expression::integer(5)), 1);
        assert_eq!(expression_complexity(&Expression::symbol(x.clone())), 1);

        let x_squared = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        assert_eq!(expression_complexity(&x_squared), 3);

        let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        assert_eq!(expression_complexity(&sin_x), 3);
    }

    #[test]
    fn test_find_substitution_candidates_basic() {
        let x = symbol!(x);
        let x_squared = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let sin_x_squared = Expression::function("sin", vec![x_squared.clone()]);

        let candidates = find_substitution_candidates(&sin_x_squared, &x);

        assert!(!candidates.is_empty());
        assert!(candidates.contains(&x_squared));
    }
}
