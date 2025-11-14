//! Integration strategy dispatcher
//!
//! Orchestrates all integration techniques in optimal order (fast to slow).
//!
//! # Strategy Layers
//!
//! 1. Table lookup (Wave 3) - 2. Rational functions (Wave 2) - 3. Function registry
//! 4. By parts - 5. Substitution (Wave 3) - 6. Trigonometric (Wave 4)
//! 7. Risch (Wave 5) - 8. Symbolic fallback

use crate::calculus::integrals::{
    basic::BasicIntegrals, by_parts::IntegrationByParts, function_integrals::FunctionIntegrals,
    rational, risch, substitution, table, trigonometric,
};
use crate::core::{Expression, Number, Symbol};

/// Main integration strategy dispatcher
///
/// Tries 8 techniques in order (fast to slow) until one succeeds.
pub fn integrate_with_strategy(expr: &Expression, var: Symbol, depth: usize) -> Expression {
    // Layer 1: Table lookup (Wave 3) - ACTIVE
    if let Some(result) = table::try_table_lookup(expr, &var) {
        return result;
    }

    // Layer 2: Rational functions (Wave 2 - Agent 2A implementing)
    if let Some(result) = try_rational_integration(expr, &var) {
        return result;
    }

    // Layer 3: Function registry (existing - already working)
    if let Some(result) = try_registry_integration(expr, &var) {
        return result;
    }

    // Layer 4: By parts (existing - already working)
    if let Some(result) = try_by_parts(expr, &var, depth) {
        return result;
    }

    // Layer 5: Substitution (Wave 3) - ACTIVE
    if let Some(result) = substitution::try_substitution(expr, var.clone()) {
        return result;
    }

    // Layer 6: Trigonometric (Wave 4) - ACTIVE
    if let Some(result) = trigonometric::try_trigonometric_integration(expr, var.clone()) {
        return result;
    }

    // Layer 7: Risch algorithm (Wave 5) - ACTIVE
    if let Some(result) = risch::try_risch_integration(expr, var.clone()) {
        return result;
    }

    // Layer 7.5: Try basic integration rules (power rule, constants, etc.)
    // These are fast and handle many common cases that don't fit above patterns
    if let Some(result) = try_basic_integration(expr, &var, depth) {
        return result;
    }

    // Layer 8: Fallback - return symbolic integral
    Expression::integral(expr.clone(), var)
}

/// Try rational function integration (Layer 2)
///
/// Delegates to rational.rs for P(x)/Q(x) patterns (Wave 2, Agent 2A).
pub fn try_rational_integration(expr: &Expression, var: &Symbol) -> Option<Expression> {
    // Check if expression matches rational function pattern P(x)/Q(x)
    if rational::is_rational_function(expr, var) {
        // Delegate to Agent 2A's rational function integrator
        rational::integrate_rational(expr, var)
    } else {
        None
    }
}

/// Try function registry integration (Layer 3)
///
/// Uses function intelligence registry for sin, cos, exp, ln, etc.
pub fn try_registry_integration(expr: &Expression, var: &Symbol) -> Option<Expression> {
    match expr {
        Expression::Function { name, args } => {
            let result = FunctionIntegrals::integrate(name, args, var.clone());

            // Check if result is symbolic integral (integration failed)
            if is_symbolic_integral(&result) {
                None
            } else {
                Some(result)
            }
        }
        _ => None,
    }
}

/// Try integration by parts (Layer 4)
///
/// Uses LIATE heuristic for products like x*exp(x).
pub fn try_by_parts(expr: &Expression, var: &Symbol, depth: usize) -> Option<Expression> {
    IntegrationByParts::integrate(expr, var.clone(), depth)
}


/// Check if expression is a polynomial in the given variable
///
/// Polynomial: only var, constants, +, *, and non-negative integer powers.
pub fn is_polynomial(expr: &Expression, _var: &Symbol) -> bool {
    match expr {
        // Constants are polynomials (degree 0)
        Expression::Number(_) | Expression::Constant(_) => true,

        // Variable itself is a polynomial (degree 1)
        Expression::Symbol(_sym) => {
            // Either the variable we're checking, or a different symbol (constant w.r.t. var)
            true
        }

        // Sum of polynomials is a polynomial
        Expression::Add(terms) => terms.iter().all(|t| is_polynomial(t, _var)),

        // Product of polynomials is a polynomial
        Expression::Mul(factors) => factors.iter().all(|f| is_polynomial(f, _var)),

        // Power is polynomial if base is polynomial and exponent is non-negative integer
        Expression::Pow(base, exp) => {
            // Check if exponent is a non-negative integer
            let valid_exponent = match &**exp {
                Expression::Number(Number::Integer(n)) => *n >= 0,
                _ => false,
            };

            valid_exponent && is_polynomial(base, _var)
        }

        // Functions, calculus operations, etc. are not polynomials
        _ => false,
    }
}

/// Try basic integration rules (Layer 7.5)
///
/// Power rule, constants, sums, products with constant factors.
pub fn try_basic_integration(expr: &Expression, var: &Symbol, depth: usize) -> Option<Expression> {
    let result = match expr {
        // Handle existing calculus expressions
        Expression::Calculus(data) => BasicIntegrals::handle_calculus(expr, data, var.clone()),

        // Basic cases
        Expression::Number(_) => BasicIntegrals::handle_constant(expr, var.clone()),
        Expression::Symbol(sym) => BasicIntegrals::handle_symbol(sym, var),
        Expression::Add(terms) => BasicIntegrals::handle_sum(terms, var.clone(), depth),
        Expression::Mul(factors) => BasicIntegrals::handle_product(factors, var.clone(), depth),
        Expression::Pow(base, exp) => BasicIntegrals::handle_power(base, exp, var.clone()),

        // Not a basic pattern
        _ => return None,
    };

    // Check if result is symbolic (failed)
    if is_symbolic_integral(&result) {
        None
    } else {
        Some(result)
    }
}

/// Check if an expression is a symbolic integral (integration failed)
fn is_symbolic_integral(expr: &Expression) -> bool {
    matches!(expr, Expression::Calculus(_))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_is_polynomial_constant() {
        let x = symbol!(x);
        assert!(is_polynomial(&Expression::integer(5), &x));
    }

    #[test]
    fn test_is_polynomial_variable() {
        let x = symbol!(x);
        assert!(is_polynomial(&Expression::symbol(x.clone()), &x));
    }

    #[test]
    fn test_is_polynomial_sum() {
        let x = symbol!(x);
        let poly = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(x.clone()),
            Expression::integer(1),
        ]);
        assert!(is_polynomial(&poly, &x));
    }

    #[test]
    fn test_is_polynomial_product() {
        let x = symbol!(x);
        let poly = Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]);
        assert!(is_polynomial(&poly, &x));
    }

    #[test]
    fn test_is_not_polynomial_negative_power() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));
        assert!(!is_polynomial(&expr, &x));
    }

    #[test]
    fn test_is_not_polynomial_function() {
        let x = symbol!(x);
        let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        assert!(!is_polynomial(&expr, &x));
    }
}
