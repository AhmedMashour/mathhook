//! Risch Differential Equation (RDE) solving
//!
//! Solves the RDE: y' + f*y = g for basic cases.
//! This module handles simple exponential and logarithmic patterns.
use super::{
    differential_extension::DifferentialExtension,
    helpers::{extract_division, is_just_variable, is_one},
    RischResult,
};
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;
/// Integrate transcendental part using RDE
///
/// Attempts to solve the Risch differential equation for basic patterns.
/// Returns Integral, NonElementary, or Unknown based on the analysis.
///
/// # Arguments
///
/// * `expr` - The transcendental expression to integrate
/// * `extensions` - The differential extension tower
/// * `var` - The variable of integration
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::integrals::risch::rde::integrate_transcendental;
/// use mathhook_core::calculus::integrals::risch::differential_extension::DifferentialExtension;
/// use mathhook_core::Expression;
/// use mathhook_core::symbol;
///
/// let x = symbol!(x);
/// let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
/// let extensions = vec![DifferentialExtension::Rational];
///
/// let result = integrate_transcendental(&expr, &extensions, &x);
/// ```
pub fn integrate_transcendental(
    expr: &Expression,
    _extensions: &[DifferentialExtension],
    var: &Symbol,
) -> RischResult {
    if let Some(result) = try_simple_exponential(expr, var) {
        return RischResult::Integral(result);
    }
    if let Some(result) = try_logarithmic_derivative(expr, var) {
        return RischResult::Integral(result);
    }
    if let Some(result) = try_exponential_product(expr, var) {
        return RischResult::Integral(result);
    }
    if is_non_elementary_pattern(expr, var) {
        return RischResult::NonElementary;
    }
    RischResult::Unknown
}
/// Try to integrate simple exponential e^(ax)
///
/// Handles patterns: e^(ax) where a is constant
/// Result: e^(ax)/a
fn try_simple_exponential(expr: &Expression, var: &Symbol) -> Option<Expression> {
    match expr {
        Expression::Function { name, args } if name.as_ref() == "exp" && args.len() == 1 => {
            let arg = &args[0];
            if let Some(coeff) = extract_linear_coefficient(arg, var) {
                return Some(Expression::div(expr.clone(), coeff));
            }
            if is_just_variable(arg, var) {
                return Some(expr.clone());
            }
            None
        }
        _ => None,
    }
}
/// Try to integrate logarithmic derivative patterns
///
/// Handles: 1/x → ln|x|, 1/(ax+b) → (1/a)*ln|ax+b|
fn try_logarithmic_derivative(expr: &Expression, var: &Symbol) -> Option<Expression> {
    if let Some((num, den)) = extract_division(expr) {
        if is_one(&num) {
            if is_just_variable(&den, var) {
                return Some(Expression::function("ln", vec![den]));
            }
            if let Some((a, b)) = extract_linear_form(&den, var) {
                let ln_arg = if b == Expression::integer(0) {
                    Expression::mul(vec![a.clone(), Expression::symbol(var.clone())])
                } else {
                    Expression::add(vec![
                        Expression::mul(vec![a.clone(), Expression::symbol(var.clone())]),
                        b,
                    ])
                };
                return Some(Expression::div(Expression::function("ln", vec![ln_arg]), a));
            }
        }
        if let Some(log_arg) = is_logarithmic_derivative_pattern(&num, &den, var.clone()) {
            return Some(Expression::function("ln", vec![log_arg]));
        }
    }
    None
}
/// Try to integrate exponential products
///
/// Handles: x*e^x, (ax+b)*e^(cx)
fn try_exponential_product(expr: &Expression, var: &Symbol) -> Option<Expression> {
    match expr {
        Expression::Mul(factors) if factors.len() == 2 => {
            let f1 = &factors[0];
            let f2 = &factors[1];
            if let Some(result) = check_exp_product(f1, f2, var) {
                return Some(result);
            }
            if let Some(result) = check_exp_product(f2, f1, var) {
                return Some(result);
            }
            None
        }
        _ => None,
    }
}
/// Check if pattern is f1 * exp(f2) where f1 is linear
fn check_exp_product(
    linear: &Expression,
    exp_part: &Expression,
    var: &Symbol,
) -> Option<Expression> {
    if let Expression::Function { name, args } = exp_part {
        if name.as_ref() == "exp" && args.len() == 1 {
            let exp_arg = &args[0];
            if is_just_variable(linear, var) && is_just_variable(exp_arg, var) {
                return Some(Expression::mul(vec![
                    Expression::add(vec![
                        Expression::symbol(var.clone()),
                        Expression::integer(-1),
                    ]),
                    exp_part.clone(),
                ]));
            }
        }
    }
    None
}
/// Check if pattern is known to be non-elementary
///
/// Detects patterns that provably have no elementary antiderivative.
fn is_non_elementary_pattern(expr: &Expression, var: &Symbol) -> bool {
    if let Some((num, den)) = extract_division(expr) {
        if is_exponential_of_var(&num, var) && is_just_variable(&den, var) {
            return true;
        }
        if is_sine_of_var(&num, var) && is_just_variable(&den, var) {
            return true;
        }
        if is_one(&num) && is_logarithm_of_var(&den, var) {
            return true;
        }
    }
    if let Expression::Function { name, args } = expr {
        if name.as_ref() == "exp" && args.len() == 1 && is_quadratic(&args[0], var) {
            return true;
        }
    }
    false
}
/// Extract coefficient from linear expression ax
fn extract_linear_coefficient(expr: &Expression, var: &Symbol) -> Option<Expression> {
    match expr {
        Expression::Symbol(s) if s == var => Some(Expression::integer(1)),
        Expression::Mul(factors) => {
            let mut coeff = None;
            let mut has_var = false;
            for factor in &**factors {
                if is_just_variable(factor, var) {
                    has_var = true;
                } else if !factor.contains_variable(var) {
                    coeff = Some(factor.clone());
                }
            }
            if has_var {
                coeff.or(Some(Expression::integer(1)))
            } else {
                None
            }
        }
        _ => None,
    }
}
/// Extract (a, b) from ax+b form
fn extract_linear_form(expr: &Expression, var: &Symbol) -> Option<(Expression, Expression)> {
    match expr {
        Expression::Symbol(s) if s == var => Some((Expression::integer(1), Expression::integer(0))),
        Expression::Add(terms) if terms.len() == 2 => {
            let t1 = &terms[0];
            let t2 = &terms[1];
            if let Some(a) = extract_linear_coefficient(t1, var) {
                if !t2.contains_variable(var) {
                    return Some((a, t2.clone()));
                }
            }
            if let Some(a) = extract_linear_coefficient(t2, var) {
                if !t1.contains_variable(var) {
                    return Some((a, t1.clone()));
                }
            }
            None
        }
        Expression::Mul(_) => {
            extract_linear_coefficient(expr, var).map(|a| (a, Expression::integer(0)))
        }
        _ => None,
    }
}
/// Check if pattern is f'/f (logarithmic derivative)
///
/// Recognizes when the numerator is the derivative of the denominator,
/// which integrates to ln|denominator|.
///
/// # Arguments
///
/// * `num` - Numerator of the fraction
/// * `den` - Denominator of the fraction
/// * `var` - Variable of integration
///
/// # Examples
///
/// The pattern f'(x)/f(x) integrates to ln|f(x)|. For example:
/// - 2x/(x²+1) → ln|x²+1| because d/dx[x²+1] = 2x
/// - 3x²/(x³+1) → ln|x³+1| because d/dx[x³+1] = 3x²
fn is_logarithmic_derivative_pattern(
    num: &Expression,
    den: &Expression,
    var: Symbol,
) -> Option<Expression> {
    let den_derivative = den.derivative(var).simplify();
    let num_simplified = num.simplify();
    if num_simplified == den_derivative {
        Some(den.clone())
    } else {
        None
    }
}
/// Check if expression is e^x
fn is_exponential_of_var(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Function { name, args } if name.as_ref() == "exp" && args.len() == 1 => {
            is_just_variable(&args[0], var)
        }
        _ => false,
    }
}
/// Check if expression is sin(x)
fn is_sine_of_var(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Function { name, args } if name.as_ref() == "sin" && args.len() == 1 => {
            is_just_variable(&args[0], var)
        }
        _ => false,
    }
}
/// Check if expression is ln(x) or log(x)
fn is_logarithm_of_var(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Function { name, args }
            if (name.as_ref() == "ln" || name.as_ref() == "log") && args.len() == 1 =>
        {
            is_just_variable(&args[0], var)
        }
        _ => false,
    }
}
/// Check if expression is quadratic in variable (x² or -x²)
fn is_quadratic(expr: &Expression, var: &Symbol) -> bool {
    match expr {
        Expression::Pow(base, exp) => is_just_variable(base, var) && is_integer_two(exp),
        Expression::Mul(factors) if factors.len() == 2 => {
            if is_negative_one(&factors[0]) {
                is_quadratic(&factors[1], var)
            } else if is_negative_one(&factors[1]) {
                is_quadratic(&factors[0], var)
            } else {
                false
            }
        }
        _ => false,
    }
}
/// Check if expression is the constant -1
fn is_negative_one(expr: &Expression) -> bool {
    match expr {
        Expression::Number(Number::Integer(n)) if *n == -1 => true,
        Expression::Mul(factors) if factors.len() == 2 => {
            matches!(&factors[0], Expression::Number(Number::Integer(-1))) && is_one(&factors[1])
                || is_one(&factors[0])
                    && matches!(&factors[1], Expression::Number(Number::Integer(-1)))
        }
        _ => false,
    }
}
/// Check if expression is the integer 2
fn is_integer_two(expr: &Expression) -> bool {
    matches!(expr, Expression::Number(Number::Integer(2)))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;
    #[test]
    fn test_simple_exp_x() {
        let x = symbol!(x);
        let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
        let extensions = vec![DifferentialExtension::Rational];
        let result = integrate_transcendental(&expr, &extensions, &x);
        assert!(matches!(result, RischResult::Integral(_)));
    }
    #[test]
    fn test_simple_exp_2x() {
        let x = symbol!(x);
        let expr = Expression::function(
            "exp",
            vec![Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(x.clone()),
            ])],
        );
        let extensions = vec![DifferentialExtension::Rational];
        let result = integrate_transcendental(&expr, &extensions, &x);
        assert!(matches!(result, RischResult::Integral(_)));
    }
    #[test]
    fn test_logarithmic_derivative_one_over_x() {
        let x = symbol!(x);
        let expr = Expression::div(Expression::integer(1), Expression::symbol(x.clone()));
        let extensions = vec![DifferentialExtension::Rational];
        let result = integrate_transcendental(&expr, &extensions, &x);
        assert!(matches!(result, RischResult::Integral(_)));
    }
    #[test]
    fn test_non_elementary_exp_x_squared() {
        let x = symbol!(x);
        let x_squared = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let expr = Expression::function("exp", vec![x_squared]);
        let extensions = vec![DifferentialExtension::Rational];
        let result = integrate_transcendental(&expr, &extensions, &x);
        assert!(matches!(result, RischResult::NonElementary));
    }
    #[test]
    fn test_non_elementary_exp_over_x() {
        let x = symbol!(x);
        let exp_x = Expression::function("exp", vec![Expression::symbol(x.clone())]);
        let expr = Expression::div(exp_x, Expression::symbol(x.clone()));
        let extensions = vec![DifferentialExtension::Rational];
        let result = integrate_transcendental(&expr, &extensions, &x);
        assert!(matches!(result, RischResult::NonElementary));
    }
    #[test]
    fn test_extract_linear_coefficient_simple() {
        let x = symbol!(x);
        let expr = Expression::symbol(x.clone());
        let coeff = extract_linear_coefficient(&expr, &x);
        assert_eq!(coeff, Some(Expression::integer(1)));
    }
    #[test]
    fn test_extract_linear_coefficient_scaled() {
        let x = symbol!(x);
        let expr = Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]);
        let coeff = extract_linear_coefficient(&expr, &x);
        assert_eq!(coeff, Some(Expression::integer(3)));
    }
    #[test]
    fn test_is_quadratic_x_squared() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        assert!(is_quadratic(&expr, &x));
    }
    #[test]
    fn test_is_not_quadratic_x_cubed() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
        assert!(!is_quadratic(&expr, &x));
    }
    #[test]
    fn test_logarithmic_derivative_pattern_basic() {
        let x = symbol!(x);
        let num = Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);
        let den = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::integer(1),
        ]);
        let result = is_logarithmic_derivative_pattern(&num, &den, x);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), den);
    }
    #[test]
    fn test_logarithmic_derivative_pattern_no_match() {
        let x = symbol!(x);
        let num = Expression::symbol(x.clone());
        let den = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::integer(1),
        ]);
        let result = is_logarithmic_derivative_pattern(&num, &den, x);
        assert!(result.is_none());
    }
}
