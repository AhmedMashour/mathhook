//! Polynomial Recurrence Evaluation Engine
//!
//! Generic three-term recurrence evaluation for all orthogonal polynomial families.
//! Implements numerically stable evaluation using recurrence relations from properties.

use crate::core::{Expression, Number};
use crate::functions::properties::PolynomialProperties;

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
        return evaluate_expression(&properties.recurrence.initial_conditions.0, x);
    }
    if n == 1 {
        return evaluate_expression(&properties.recurrence.initial_conditions.1, x);
    }

    let mut p_prev = evaluate_expression(&properties.recurrence.initial_conditions.0, x);
    let mut p_curr = evaluate_expression(&properties.recurrence.initial_conditions.1, x);

    for i in 1..n {
        let alpha = evaluate_coefficient(&properties.recurrence.alpha_coeff, i, x);
        let beta = evaluate_coefficient(&properties.recurrence.beta_coeff, i, x);
        let gamma = evaluate_coefficient(&properties.recurrence.gamma_coeff, i, x);

        let p_next = (alpha * x + beta) * p_curr + gamma * p_prev;
        p_prev = p_curr;
        p_curr = p_next;
    }

    p_curr
}

/// Evaluate coefficient expression to numerical value
///
/// Handles symbolic coefficients that depend on n (e.g., (2n+1)/(n+1) for Legendre)
/// and evaluates them to f64 for numerical computation.
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
fn evaluate_coefficient(expr: &Expression, n: usize, x: f64) -> f64 {
    match expr {
        Expression::Number(num) => number_to_f64(num),
        Expression::Symbol(sym) if sym.name() == "x" => x,
        Expression::Symbol(sym) if sym.name() == "n" => n as f64,
        Expression::Add(terms) => terms
            .iter()
            .map(|t| evaluate_coefficient(t, n, x))
            .sum(),
        Expression::Mul(factors) => factors
            .iter()
            .map(|f| evaluate_coefficient(f, n, x))
            .product(),
        Expression::Pow(base, exp) => {
            let base_val = evaluate_coefficient(base, n, x);
            let exp_val = evaluate_coefficient(exp, n, x);
            base_val.powf(exp_val)
        }
        Expression::Function { name, args } => evaluate_function_coefficient(name, args, n, x),
        _ => 0.0,
    }
}

/// Evaluate function call in coefficient expression
///
/// Handles special coefficient functions like legendre_alpha(n), hermite_gamma(n), etc.
///
/// # Arguments
///
/// * `name` - Function name
/// * `args` - Function arguments
/// * `n` - Current iteration index
/// * `x` - Evaluation point
///
/// # Returns
///
/// Numerical value of function at given n
fn evaluate_function_coefficient(name: &str, args: &[Expression], n: usize, x: f64) -> f64 {
    let n_f64 = n as f64;

    match name {
        "legendre_alpha" => (2.0 * n_f64 + 1.0) / (n_f64 + 1.0),
        "legendre_gamma" => -n_f64 / (n_f64 + 1.0),
        "laguerre_alpha" => -(1.0 / (n_f64 + 1.0)),
        "laguerre_beta" => (2.0 * n_f64 + 1.0) / (n_f64 + 1.0),
        "laguerre_gamma" => -n_f64 / (n_f64 + 1.0),
        _ => {
            if !args.is_empty() {
                evaluate_coefficient(&args[0], n, x)
            } else {
                0.0
            }
        }
    }
}

/// Evaluate expression to numerical value
///
/// Used for initial conditions and constant expressions in recurrence.
///
/// # Arguments
///
/// * `expr` - Expression to evaluate
/// * `x` - Evaluation point
///
/// # Returns
///
/// Numerical value of expression at x
fn evaluate_expression(expr: &Expression, x: f64) -> f64 {
    match expr {
        Expression::Number(num) => number_to_f64(num),
        Expression::Symbol(sym) if sym.name() == "x" => x,
        Expression::Add(terms) => terms.iter().map(|t| evaluate_expression(t, x)).sum(),
        Expression::Mul(factors) => factors.iter().map(|f| evaluate_expression(f, x)).product(),
        Expression::Pow(base, exp) => {
            let base_val = evaluate_expression(base, x);
            let exp_val = evaluate_expression(exp, x);
            base_val.powf(exp_val)
        }
        _ => 0.0,
    }
}

/// Convert Number to f64 for numerical computation
///
/// # Arguments
///
/// * `num` - Number to convert
///
/// # Returns
///
/// f64 representation of number
fn number_to_f64(num: &Number) -> f64 {
    match num {
        Number::Integer(i) => *i as f64,
        Number::Rational(r) => {
            use num_traits::ToPrimitive;
            let numer = r.numer().to_f64().unwrap_or(0.0);
            let denom = r.denom().to_f64().unwrap_or(1.0);
            numer / denom
        }
        Number::Float(f) => *f,
        _ => 0.0,
    }
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

    #[test]
    fn test_evaluate_recurrence_legendre_low_order() {
        let legendre = LegendreIntelligence::new();
        let props = legendre.get_properties();

        if let Some(FunctionProperties::Polynomial(legendre_props)) =
            props.get("legendre_p")
        {
            assert!((evaluate_recurrence(legendre_props, 0, 0.5) - 1.0).abs() < 1e-10);
            assert!((evaluate_recurrence(legendre_props, 1, 0.5) - 0.5).abs() < 1e-10);
        }
    }

    #[test]
    fn test_coefficient_evaluation() {
        let expr_n = Expression::symbol("n");
        let expr_2n_plus_1 = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol("n")]),
            Expression::integer(1),
        ]);

        assert!((evaluate_coefficient(&expr_n, 5, 0.0) - 5.0).abs() < 1e-10);
        assert!((evaluate_coefficient(&expr_2n_plus_1, 5, 0.0) - 11.0).abs() < 1e-10);
    }

    #[test]
    fn test_expression_evaluation() {
        let expr_1 = Expression::integer(1);
        let expr_x = Expression::symbol("x");
        let expr_2x = Expression::mul(vec![Expression::integer(2), Expression::symbol("x")]);

        assert!((evaluate_expression(&expr_1, 0.5) - 1.0).abs() < 1e-10);
        assert!((evaluate_expression(&expr_x, 0.5) - 0.5).abs() < 1e-10);
        assert!((evaluate_expression(&expr_2x, 0.5) - 1.0).abs() < 1e-10);
    }
}
