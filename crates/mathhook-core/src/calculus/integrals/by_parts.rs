//! Integration by parts implementation
//!
//! Implements the integration by parts formula:
//! ∫ u dv = uv - ∫ v du
//!
//! This module provides automatic selection of u and dv based on heuristics
//! (LIATE rule: Logarithmic, Inverse trig, Algebraic, Trigonometric, Exponential)

use crate::calculus::derivatives::Derivative;
use crate::calculus::integrals::Integration;
use crate::core::{Expression, Symbol};

/// Integration by parts handler
pub struct IntegrationByParts;

impl IntegrationByParts {
    /// Attempt integration by parts on an expression
    ///
    /// Uses LIATE rule to select u and dv:
    /// - L: Logarithmic functions (ln, log)
    /// - I: Inverse trigonometric functions (arcsin, arctan, etc.)
    /// - A: Algebraic functions (polynomials, powers)
    /// - T: Trigonometric functions (sin, cos, tan)
    /// - E: Exponential functions (e^x, a^x)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, IntegrationByParts};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::integrals::Integration;
    ///
    /// let x = symbol!(x);
    /// // ∫ x·e^x dx
    /// let expr = Expression::mul(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::function("exp", vec![Expression::symbol(x.clone())])
    /// ]);
    /// let result = IntegrationByParts::integrate(&expr, x);
    /// ```
    pub fn integrate(expr: &Expression, variable: Symbol) -> Option<Expression> {
        // Try to identify if this is a product suitable for integration by parts
        if let Expression::Mul(factors) = expr {
            if factors.len() == 2 {
                // Try both orderings: (u=f0, dv=f1) and (u=f1, dv=f0)
                if let Some(result) = Self::try_by_parts(&factors[0], &factors[1], variable.clone())
                {
                    return Some(result);
                }
                if let Some(result) = Self::try_by_parts(&factors[1], &factors[0], variable) {
                    return Some(result);
                }
            }
        }
        None
    }

    /// Try integration by parts with specific u and dv
    ///
    /// ∫ u dv = uv - ∫ v du
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, IntegrationByParts};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let u = Expression::symbol(x.clone());
    /// let dv = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    /// let result = IntegrationByParts::try_by_parts(&u, &dv, x);
    /// ```
    pub fn try_by_parts(u: &Expression, dv: &Expression, variable: Symbol) -> Option<Expression> {
        // Check if this is a good choice based on LIATE
        if !Self::is_good_u_choice(u, &variable) {
            return None;
        }

        // Compute du = derivative of u
        let du = u.derivative(variable.clone());

        // Compute v = integral of dv
        let v = dv.integrate(variable.clone());

        // Check if v is simpler (not just symbolic integral)
        if Self::is_symbolic_integral(&v) {
            return None;
        }

        // Compute ∫ v du
        let v_du = Expression::mul(vec![v.clone(), du]);
        let integral_v_du = v_du.integrate(variable);

        // Return uv - ∫ v du
        Some(Expression::add(vec![
            Expression::mul(vec![u.clone(), v]),
            Expression::mul(vec![Expression::integer(-1), integral_v_du]),
        ]))
    }

    /// Determine if an expression is a good choice for u (LIATE priority)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, IntegrationByParts};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::function("ln", vec![Expression::symbol(x.clone())]);
    /// let is_good = IntegrationByParts::is_good_u_choice(&expr, &x);
    /// ```
    pub fn is_good_u_choice(expr: &Expression, variable: &Symbol) -> bool {
        match expr {
            // Logarithmic (highest priority)
            Expression::Function { name, .. } if name == "ln" || name == "log" => true,

            // Inverse trigonometric
            Expression::Function { name, .. }
                if name == "arcsin"
                    || name == "arccos"
                    || name == "arctan"
                    || name == "arcsec"
                    || name == "arccsc"
                    || name == "arccot" =>
            {
                true
            }

            // Algebraic (polynomials, powers of x)
            Expression::Symbol(sym) if sym == variable => true,
            Expression::Pow(base, _) => {
                if let Expression::Symbol(sym) = &**base {
                    sym == variable
                } else {
                    false
                }
            }

            // Don't choose trigonometric or exponential as u (lower priority)
            Expression::Function { name, .. }
                if name == "sin"
                    || name == "cos"
                    || name == "tan"
                    || name == "exp"
                    || name == "sinh"
                    || name == "cosh" =>
            {
                false
            }

            _ => false,
        }
    }

    /// Check if an expression is just a symbolic integral (integration failed)
    fn is_symbolic_integral(expr: &Expression) -> bool {
        matches!(expr, Expression::Calculus(_))
    }

    /// Apply integration by parts multiple times (for cases like ∫ x²·e^x dx)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, IntegrationByParts};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// // ∫ x²·e^x dx requires two applications of by parts
    /// let expr = Expression::mul(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::function("exp", vec![Expression::symbol(x.clone())])
    /// ]);
    /// let result = IntegrationByParts::integrate_repeated(&expr, x, 2);
    /// ```
    pub fn integrate_repeated(
        expr: &Expression,
        variable: Symbol,
        max_iterations: usize,
    ) -> Option<Expression> {
        let mut current = expr.clone();

        for _ in 0..max_iterations {
            if let Some(result) = Self::integrate(&current, variable.clone()) {
                // Check if the result still contains integrals
                if Self::contains_integral(&result) {
                    current = result;
                } else {
                    return Some(result);
                }
            } else {
                break;
            }
        }

        None
    }

    /// Check if expression contains a symbolic integral
    fn contains_integral(expr: &Expression) -> bool {
        match expr {
            Expression::Calculus(_) => true,
            Expression::Add(terms) => terms.iter().any(|t| Self::contains_integral(t)),
            Expression::Mul(factors) => factors.iter().any(|f| Self::contains_integral(f)),
            Expression::Pow(base, exp) => {
                Self::contains_integral(base) || Self::contains_integral(exp)
            }
            Expression::Function { args, .. } => args.iter().any(|a| Self::contains_integral(a)),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_by_parts_x_times_exp() {
        let x = symbol!(x);
        // ∫ x·e^x dx = x·e^x - e^x + C
        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::function("exp", vec![Expression::symbol(x.clone())]),
        ]);

        let result = IntegrationByParts::integrate(&expr, x);
        assert!(result.is_some());
    }

    #[test]
    fn test_by_parts_x_times_sin() {
        let x = symbol!(x);
        // ∫ x·sin(x) dx = -x·cos(x) + sin(x) + C
        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
        ]);

        let result = IntegrationByParts::integrate(&expr, x);
        assert!(result.is_some());
    }

    #[test]
    fn test_by_parts_ln() {
        let x = symbol!(x);
        // ∫ ln(x) dx = x·ln(x) - x + C
        let expr = Expression::function("ln", vec![Expression::symbol(x.clone())]);

        // This requires treating ln(x) as ln(x)·1 and using by parts
        let as_product = Expression::mul(vec![expr, Expression::integer(1)]);

        let result = IntegrationByParts::integrate(&as_product, x);
        assert!(result.is_some());
    }

    #[test]
    fn test_u_choice_priority() {
        let x = symbol!(x);

        // Logarithmic should be chosen
        let ln_expr = Expression::function("ln", vec![Expression::symbol(x.clone())]);
        assert!(IntegrationByParts::is_good_u_choice(&ln_expr, &x));

        // Inverse trig should be chosen
        let arcsin_expr = Expression::function("arcsin", vec![Expression::symbol(x.clone())]);
        assert!(IntegrationByParts::is_good_u_choice(&arcsin_expr, &x));

        // Algebraic should be chosen
        let poly_expr = Expression::symbol(x.clone());
        assert!(IntegrationByParts::is_good_u_choice(&poly_expr, &x));

        // Exponential should NOT be chosen as u
        let exp_expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
        assert!(!IntegrationByParts::is_good_u_choice(&exp_expr, &x));

        // Trigonometric should NOT be chosen as u
        let sin_expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        assert!(!IntegrationByParts::is_good_u_choice(&sin_expr, &x));
    }
}
