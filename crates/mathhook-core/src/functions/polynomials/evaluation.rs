//! Polynomial Recurrence Evaluation Engine
//!
//! Generic three-term recurrence evaluation for all orthogonal polynomial families.
//! Implements numerically stable evaluation using recurrence relations from properties.

use crate::core::Expression;
use crate::functions::properties::PolynomialProperties;
use std::collections::HashMap;

/// Evaluate polynomial using three-term recurrence relation
///
/// Generic implementation for all orthogonal polynomials that uses the
/// recurrence relation defined in PolynomialProperties. This is mathematically
/// the most stable approach for polynomial evaluation.
///
/// # Recurrence Form
///
/// For orthogonal polynomials, the three-term recurrence has the form:
/// ```text
/// P_{n+1}(x) = (alpha_n * x + beta_n) * P_n(x) + gamma_n * P_{n-1}(x)
/// ```
///
/// # Arguments
///
/// * `properties` - Polynomial properties containing recurrence coefficients
/// * `n` - Polynomial degree (must be non-negative)
/// * `x` - Evaluation point
///
/// # Returns
///
/// Numerical value of P_n(x) computed using recurrence relation
///
/// # Examples
///
/// ```rust
/// use mathhook_core::functions::polynomials::evaluation::evaluate_recurrence;
/// use mathhook_core::functions::polynomials::legendre::LegendreIntelligence;
/// use mathhook_core::functions::properties::FunctionProperties;
///
/// let legendre = LegendreIntelligence::new();
/// let props = legendre.get_properties();
/// if let Some(FunctionProperties::Polynomial(legendre_props)) = props.get("legendre_p") {
///     let result = evaluate_recurrence(legendre_props, 5, 0.5);
///     assert!((result - 0.08984375).abs() < 1e-10);
/// }
/// ```
pub fn evaluate_recurrence(properties: &PolynomialProperties, n: usize, x: f64) -> f64 {
    if n == 0 {
        return eval_expr_at_x_internal(&properties.recurrence.initial_conditions.0, x);
    }
    if n == 1 {
        return eval_expr_at_x_internal(&properties.recurrence.initial_conditions.1, x);
    }

    let mut p_prev = eval_expr_at_x_internal(&properties.recurrence.initial_conditions.0, x);
    let mut p_curr = eval_expr_at_x_internal(&properties.recurrence.initial_conditions.1, x);

    for i in 1..n {
        let alpha = eval_coeff_at_n_internal(&properties.recurrence.alpha_coeff, i, x);
        let beta = eval_coeff_at_n_internal(&properties.recurrence.beta_coeff, i, x);
        let gamma = eval_coeff_at_n_internal(&properties.recurrence.gamma_coeff, i, x);

        let p_next = (alpha * x + beta) * p_curr + gamma * p_prev;
        p_prev = p_curr;
        p_curr = p_next;
    }

    p_curr
}

/// Internal domain-specific evaluation for polynomial coefficients
///
/// Handles symbolic coefficients that depend on n (e.g., (2n+1)/(n+1) for Legendre)
/// and evaluates them to f64 for numerical computation.
///
/// This function substitutes 'n' and 'x' symbols with their values, then uses
/// Expression::evaluate_to_f64() for numerical conversion.
///
/// # Arguments
///
/// * `expr` - Coefficient expression (may contain 'n' symbol)
/// * `n` - Current iteration index
/// * `x` - Evaluation point (needed for some coefficient expressions)
///
/// # Returns
///
/// Numerical value of coefficient at given n
fn eval_coeff_at_n_internal(expr: &Expression, n: usize, x: f64) -> f64 {
    let mut substitutions = HashMap::new();
    substitutions.insert("n".to_owned(), Expression::integer(n as i64));
    substitutions.insert("x".to_owned(), Expression::float(x));

    expr.substitute(&substitutions)
        .evaluate_to_f64()
        .unwrap_or_else(|_| eval_func_coeff_internal(expr, n))
}

/// Internal fallback for special coefficient functions
///
/// Handles special coefficient functions like legendre_alpha(n), hermite_gamma(n), etc.
/// that cannot be handled by standard expression evaluation.
fn eval_func_coeff_internal(expr: &Expression, n: usize) -> f64 {
    match expr {
        Expression::Function { name, .. } => {
            let n_f64 = n as f64;
            match name.as_ref() {
                "legendre_alpha" => (2.0 * n_f64 + 1.0) / (n_f64 + 1.0),
                "legendre_gamma" => -n_f64 / (n_f64 + 1.0),
                "laguerre_alpha" => -(1.0 / (n_f64 + 1.0)),
                "laguerre_beta" => (2.0 * n_f64 + 1.0) / (n_f64 + 1.0),
                "laguerre_gamma" => -n_f64 / (n_f64 + 1.0),
                _ => 0.0,
            }
        }
        _ => 0.0,
    }
}

/// Internal domain-specific evaluation for polynomial expressions
///
/// Used for initial conditions and constant expressions in recurrence.
/// Substitutes 'x' symbol with the evaluation point and converts to f64.
///
/// # Arguments
///
/// * `expr` - Expression to evaluate
/// * `x` - Evaluation point
///
/// # Returns
///
/// Numerical value of expression at x
fn eval_expr_at_x_internal(expr: &Expression, x: f64) -> f64 {
    let mut substitutions = HashMap::new();
    substitutions.insert("x".to_owned(), Expression::float(x));

    expr.substitute(&substitutions)
        .evaluate_to_f64()
        .unwrap_or(0.0)
}

/// Numerical evaluator function for Legendre polynomials
///
/// Expected args: [n, x] where n is polynomial degree and x is evaluation point
pub fn evaluate_legendre_numerical(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        return vec![0.0];
    }
    let n = args[0] as usize;
    let x = args[1];

    vec![evaluate_legendre(n, x)]
}

/// Direct Legendre polynomial evaluation using recurrence relation
///
/// (n+1)P_{n+1}(x) = (2n+1)x P_n(x) - n P_{n-1}(x)
fn evaluate_legendre(n: usize, x: f64) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n == 1 {
        return x;
    }

    let mut p_prev = 1.0;
    let mut p_curr = x;

    for i in 1..n {
        let i_f64 = i as f64;
        let alpha = (2.0 * i_f64 + 1.0) / (i_f64 + 1.0);
        let gamma = -i_f64 / (i_f64 + 1.0);

        let p_next = alpha * x * p_curr + gamma * p_prev;
        p_prev = p_curr;
        p_curr = p_next;
    }

    p_curr
}

/// Numerical evaluator function for Hermite polynomials
///
/// Expected args: [n, x] where n is polynomial degree and x is evaluation point
pub fn evaluate_hermite_numerical(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        return vec![0.0];
    }
    let n = args[0] as usize;
    let x = args[1];

    vec![evaluate_hermite(n, x)]
}

/// Direct Hermite polynomial evaluation using recurrence relation
///
/// H_{n+1}(x) = 2x H_n(x) - 2n H_{n-1}(x)
fn evaluate_hermite(n: usize, x: f64) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n == 1 {
        return 2.0 * x;
    }

    let mut p_prev = 1.0;
    let mut p_curr = 2.0 * x;

    for i in 1..n {
        let i_f64 = i as f64;
        let alpha = 2.0 * x;
        let gamma = -2.0 * i_f64;

        let p_next = alpha * p_curr + gamma * p_prev;
        p_prev = p_curr;
        p_curr = p_next;
    }

    p_curr
}

/// Numerical evaluator function for Laguerre polynomials
///
/// Expected args: [n, x] where n is polynomial degree and x is evaluation point
pub fn evaluate_laguerre_numerical(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        return vec![0.0];
    }
    let n = args[0] as usize;
    let x = args[1];

    vec![evaluate_laguerre(n, x)]
}

/// Direct Laguerre polynomial evaluation using recurrence relation
///
/// (n+1)L_{n+1}(x) = (2n+1-x)L_n(x) - nL_{n-1}(x)
fn evaluate_laguerre(n: usize, x: f64) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n == 1 {
        return 1.0 - x;
    }

    let mut p_prev = 1.0;
    let mut p_curr = 1.0 - x;

    for i in 1..n {
        let i_f64 = i as f64;
        let alpha = -(1.0 / (i_f64 + 1.0));
        let beta = (2.0 * i_f64 + 1.0) / (i_f64 + 1.0);
        let gamma = -i_f64 / (i_f64 + 1.0);

        let p_next = (alpha * x + beta) * p_curr + gamma * p_prev;
        p_prev = p_curr;
        p_curr = p_next;
    }

    p_curr
}

/// Numerical evaluator function for Chebyshev first kind polynomials T_n(x)
///
/// Expected args: [n, x] where n is polynomial degree and x is evaluation point
pub fn evaluate_chebyshev_first_numerical(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        return vec![0.0];
    }
    let n = args[0] as usize;
    let x = args[1];

    vec![evaluate_chebyshev_first(n, x)]
}

/// Direct Chebyshev first kind polynomial evaluation using recurrence relation
///
/// T_{n+1}(x) = 2x T_n(x) - T_{n-1}(x)
fn evaluate_chebyshev_first(n: usize, x: f64) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n == 1 {
        return x;
    }

    let mut p_prev = 1.0;
    let mut p_curr = x;

    for _ in 1..n {
        let p_next = 2.0 * x * p_curr - p_prev;
        p_prev = p_curr;
        p_curr = p_next;
    }

    p_curr
}

/// Numerical evaluator function for Chebyshev second kind polynomials U_n(x)
///
/// Expected args: [n, x] where n is polynomial degree and x is evaluation point
pub fn evaluate_chebyshev_second_numerical(args: &[f64]) -> Vec<f64> {
    if args.len() != 2 {
        return vec![0.0];
    }
    let n = args[0] as usize;
    let x = args[1];

    vec![evaluate_chebyshev_second(n, x)]
}

/// Direct Chebyshev second kind polynomial evaluation using recurrence relation
///
/// U_{n+1}(x) = 2x U_n(x) - U_{n-1}(x)
fn evaluate_chebyshev_second(n: usize, x: f64) -> f64 {
    if n == 0 {
        return 1.0;
    }
    if n == 1 {
        return 2.0 * x;
    }

    let mut p_prev = 1.0;
    let mut p_curr = 2.0 * x;

    for _ in 1..n {
        let p_next = 2.0 * x * p_curr - p_prev;
        p_prev = p_curr;
        p_curr = p_next;
    }

    p_curr
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::functions::polynomials::legendre::LegendreIntelligence;
    use crate::functions::properties::FunctionProperties;
    use crate::{expr, symbol};

    #[test]
    fn test_evaluate_recurrence_legendre_low_order() {
        let legendre = LegendreIntelligence::new();
        let props = legendre.get_properties();

        if let Some(FunctionProperties::Polynomial(legendre_props)) = props.get("legendre_p") {
            assert!((evaluate_recurrence(legendre_props, 0, 0.5) - 1.0).abs() < 1e-10);
            assert!((evaluate_recurrence(legendre_props, 1, 0.5) - 0.5).abs() < 1e-10);
        }
    }

    #[test]
    fn test_coefficient_evaluation() {
        let n_sym = symbol!(n);
        let expr_2n_plus_1 = expr!((2 * n) + 1);

        assert!((eval_coeff_at_n_internal(&Expression::symbol(n_sym), 5, 0.0) - 5.0).abs() < 1e-10);
        assert!((eval_coeff_at_n_internal(&expr_2n_plus_1, 5, 0.0) - 11.0).abs() < 1e-10);
    }

    #[test]
    fn test_expression_evaluation() {
        let x_sym = symbol!(x);
        let expr_1 = expr!(1);
        let expr_x = Expression::symbol(x_sym);
        let expr_2x = expr!(2 * x);

        assert!((eval_expr_at_x_internal(&expr_1, 0.5) - 1.0).abs() < 1e-10);
        assert!((eval_expr_at_x_internal(&expr_x, 0.5) - 0.5).abs() < 1e-10);
        assert!((eval_expr_at_x_internal(&expr_2x, 0.5) - 1.0).abs() < 1e-10);
    }
}
