//! Differential extension tower construction
//!
//! Builds a tower of differential extensions for integrand expressions.
//! Each extension represents a transcendental function (exponential or logarithmic).

use super::helpers::is_one;
use crate::core::{Expression, Symbol};

/// Differential extension tower element
#[derive(Debug, Clone, PartialEq)]
pub enum DifferentialExtension {
    /// Base field (rational functions)
    Rational,

    /// Exponential extension: e^(argument)
    Exponential {
        argument: Box<Expression>,
        derivative: Box<Expression>,
    },

    /// Logarithmic extension: ln(argument)
    Logarithmic {
        argument: Box<Expression>,
        derivative: Box<Expression>,
    },
}

/// Build differential extension tower for expression
///
/// Analyzes the expression structure and identifies transcendental
/// extensions (exponentials and logarithms).
///
/// # Arguments
///
/// * `expr` - The expression to analyze
/// * `var` - The variable of integration
///
/// # Examples
///
/// ```rust
/// use mathhook_core::calculus::integrals::risch::differential_extension::build_extension_tower;
/// use mathhook_core::Expression;
/// use mathhook_core::symbol;
///
/// let x = symbol!(x);
/// let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
/// let tower = build_extension_tower(&expr, x);
/// assert!(tower.is_some());
/// ```
pub fn build_extension_tower(expr: &Expression, var: Symbol) -> Option<Vec<DifferentialExtension>> {
    let mut extensions = vec![DifferentialExtension::Rational];

    // Detect exponential extensions
    if let Some(exp_ext) = detect_exponential_extension(expr, var.clone()) {
        extensions.push(exp_ext);
    }

    // Detect logarithmic extensions
    if let Some(log_ext) = detect_logarithmic_extension(expr, var) {
        extensions.push(log_ext);
    }

    Some(extensions)
}

/// Detect exponential extension in expression
///
/// Looks for patterns like exp(x), exp(ax), exp(ax+b).
fn detect_exponential_extension(expr: &Expression, var: Symbol) -> Option<DifferentialExtension> {
    match expr {
        Expression::Function { name, args } if name == "exp" && args.len() == 1 => {
            let arg = &args[0];

            // Check if argument contains the variable
            if arg.contains_variable(&var) {
                // For basic implementation: handle e^x, e^(ax), e^(ax+b)
                Some(DifferentialExtension::Exponential {
                    argument: Box::new(arg.clone()),
                    derivative: Box::new(compute_exponential_derivative(arg, var)),
                })
            } else {
                None
            }
        }
        Expression::Mul(factors) => {
            // Check factors for exponential
            for factor in &**factors {
                if let Some(ext) = detect_exponential_extension(factor, var.clone()) {
                    return Some(ext);
                }
            }
            None
        }
        _ => None,
    }
}

/// Detect logarithmic extension in expression
///
/// Looks for patterns like ln(x), ln(ax), 1/x patterns.
fn detect_logarithmic_extension(expr: &Expression, var: Symbol) -> Option<DifferentialExtension> {
    use super::helpers::extract_division;

    match expr {
        Expression::Function { name, args }
            if (name == "ln" || name == "log") && args.len() == 1 =>
        {
            let arg = &args[0];

            if arg.contains_variable(&var) {
                Some(DifferentialExtension::Logarithmic {
                    argument: Box::new(arg.clone()),
                    derivative: Box::new(compute_logarithmic_derivative(arg, var)),
                })
            } else {
                None
            }
        }
        Expression::Mul(_) => {
            // Check for division pattern: numerator * denominator^(-1)
            if let Some((num, den)) = extract_division(expr) {
                // Check for 1/x pattern (logarithmic derivative)
                if is_one(&num) && den.contains_variable(&var) {
                    return Some(DifferentialExtension::Logarithmic {
                        argument: Box::new(den.clone()),
                        derivative: Box::new(Expression::div(Expression::integer(1), den)),
                    });
                }
            }
            None
        }
        Expression::Pow(_, _) => {
            // Check for den^(-1) pattern (represents 1/den)
            if let Some((num, den)) = extract_division(expr) {
                // Check for 1/x pattern (logarithmic derivative)
                if is_one(&num) && den.contains_variable(&var) {
                    return Some(DifferentialExtension::Logarithmic {
                        argument: Box::new(den.clone()),
                        derivative: Box::new(Expression::div(Expression::integer(1), den)),
                    });
                }
            }
            None
        }
        _ => None,
    }
}

/// Compute derivative of exponential extension
///
/// For t = e^g, derivative is g' * e^g = g' * t
fn compute_exponential_derivative(arg: &Expression, var: Symbol) -> Expression {
    // Derivative of e^g is g' * e^g
    let arg_derivative = derivative_of(arg, var);
    Expression::mul(vec![
        arg_derivative,
        Expression::function("exp", vec![arg.clone()]),
    ])
}

/// Compute derivative of logarithmic extension
///
/// For t = ln(g), derivative is g'/g
fn compute_logarithmic_derivative(arg: &Expression, var: Symbol) -> Expression {
    // Derivative of ln(g) is g'/g
    let arg_derivative = derivative_of(arg, var);
    Expression::div(arg_derivative, arg.clone())
}

/// Compute simple derivative (basic cases only)
///
/// This is a simplified derivative computation for the Risch algorithm.
/// For more complex cases, use the full derivative system.
fn derivative_of(expr: &Expression, var: Symbol) -> Expression {
    match expr {
        Expression::Symbol(s) if *s == var => Expression::integer(1),
        Expression::Number(_) | Expression::Constant(_) => Expression::integer(0),
        Expression::Symbol(_) => Expression::integer(0),
        Expression::Mul(factors) => {
            // Product rule: (fg)' = f'g + fg'
            if factors.len() == 2 {
                let f = &factors[0];
                let g = &factors[1];
                let f_prime = derivative_of(f, var.clone());
                let g_prime = derivative_of(g, var);
                Expression::add(vec![
                    Expression::mul(vec![f_prime, g.clone()]),
                    Expression::mul(vec![f.clone(), g_prime]),
                ])
            } else {
                // For simplicity, handle basic cases
                Expression::integer(0)
            }
        }
        Expression::Add(terms) => {
            // Sum rule: (f+g)' = f' + g'
            Expression::add(
                terms
                    .iter()
                    .map(|t| derivative_of(t, var.clone()))
                    .collect(),
            )
        }
        Expression::Pow(base, exp) => {
            // Power rule for constant exponent
            if !exp.contains_variable(&var) {
                // (x^n)' = n*x^(n-1) * x'
                let base_derivative = derivative_of(base, var);
                Expression::mul(vec![
                    (**exp).clone(),
                    Expression::pow(
                        (**base).clone(),
                        Expression::add(vec![(**exp).clone(), Expression::integer(-1)]),
                    ),
                    base_derivative,
                ])
            } else {
                Expression::integer(0)
            }
        }
        _ => Expression::integer(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_detect_exponential_simple() {
        let x = symbol!(x);
        let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);

        let ext = detect_exponential_extension(&expr, x);
        assert!(ext.is_some());
        assert!(matches!(
            ext.unwrap(),
            DifferentialExtension::Exponential { .. }
        ));
    }

    #[test]
    fn test_detect_logarithmic_simple() {
        let x = symbol!(x);
        let expr = Expression::function("ln", vec![Expression::symbol(x.clone())]);

        let ext = detect_logarithmic_extension(&expr, x);
        assert!(ext.is_some());
        assert!(matches!(
            ext.unwrap(),
            DifferentialExtension::Logarithmic { .. }
        ));
    }

    #[test]
    fn test_detect_logarithmic_derivative() {
        let x = symbol!(x);
        let expr = Expression::div(Expression::integer(1), Expression::symbol(x.clone()));

        let ext = detect_logarithmic_extension(&expr, x);
        assert!(ext.is_some());
        assert!(matches!(
            ext.unwrap(),
            DifferentialExtension::Logarithmic { .. }
        ));
    }

    #[test]
    fn test_build_tower_exponential() {
        let x = symbol!(x);
        let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);

        let tower = build_extension_tower(&expr, x);
        assert!(tower.is_some());
        let extensions = tower.unwrap();
        assert!(extensions.len() >= 2);
    }
}
