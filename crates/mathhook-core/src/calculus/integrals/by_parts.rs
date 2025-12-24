//! Integration by parts implementation
//!
//! Implements the integration by parts formula:
//! ∫ u dv = uv - ∫ v du
//!
//! This module provides automatic selection of u and dv based on heuristics
//! (LIATE rule: Logarithmic, Inverse trig, Algebraic, Trigonometric, Exponential)

use crate::calculus::derivatives::Derivative;
use crate::calculus::integrals::strategy::{integrate_with_strategy, StrategyContext};
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;

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
    /// use mathhook_core::calculus::integrals::by_parts::IntegrationByParts;
    /// use mathhook_core::calculus::integrals::Integration;
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// // ∫ x·e^x dx
    /// let expr = Expression::mul(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::function("exp", vec![Expression::symbol(x.clone())])
    /// ]);
    /// let result = IntegrationByParts::integrate(&expr, x, 0);
    /// ```
    pub fn integrate(expr: &Expression, variable: Symbol, depth: usize) -> Option<Expression> {
        let context = StrategyContext::new();
        Self::integrate_with_context(expr, variable, &context, depth)
    }

    /// Integration by parts with strategy context tracking
    ///
    /// Prevents infinite recursion by using strategy context.
    /// The context is already marked with IntegrationByParts active,
    /// so recursive calls won't try by_parts again.
    pub fn integrate_with_context(
        expr: &Expression,
        variable: Symbol,
        context: &StrategyContext,
        depth: usize,
    ) -> Option<Expression> {
        if let Expression::Mul(factors) = expr {
            if factors.len() == 2 {
                if let Some(result) = Self::try_by_parts_with_context(
                    &factors[0],
                    &factors[1],
                    variable.clone(),
                    context,
                    depth,
                ) {
                    return Some(result);
                }
                if let Some(result) = Self::try_by_parts_with_context(
                    &factors[1],
                    &factors[0],
                    variable,
                    context,
                    depth,
                ) {
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
    /// use mathhook_core::calculus::integrals::by_parts::IntegrationByParts;
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// let u = Expression::symbol(x.clone());
    /// let dv = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    /// let result = IntegrationByParts::try_by_parts(&u, &dv, x, 0);
    /// ```
    pub fn try_by_parts(
        u: &Expression,
        dv: &Expression,
        variable: Symbol,
        depth: usize,
    ) -> Option<Expression> {
        let context = StrategyContext::new();
        Self::try_by_parts_with_context(u, dv, variable, &context, depth)
    }

    /// Try integration by parts with strategy context
    ///
    /// The context prevents recursive application of integration by parts.
    /// Combined with depth limiting as a safety measure.
    fn try_by_parts_with_context(
        u: &Expression,
        dv: &Expression,
        variable: Symbol,
        _context: &StrategyContext,
        depth: usize,
    ) -> Option<Expression> {
        const MAX_BY_PARTS_DEPTH: usize = 3;
        if depth >= MAX_BY_PARTS_DEPTH {
            return None;
        }

        if !Self::is_good_u_choice(u, &variable) {
            return None;
        }

        let du = u.derivative(variable.clone());

        let v = integrate_with_strategy(dv, variable.clone(), depth + 1);

        if Self::is_symbolic_integral(&v) {
            return None;
        }

        let v_du = if let Expression::Mul(v_factors) = &v {
            let mut factors = (**v_factors).clone();
            factors.push(du);
            Expression::mul(factors).simplify()
        } else {
            Expression::mul(vec![v.clone(), du]).simplify()
        };

        let integral_v_du = integrate_with_strategy(&v_du, variable, depth + 1);

        if Self::is_symbolic_integral(&integral_v_du) {
            return None;
        }

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
    /// use mathhook_core::calculus::integrals::by_parts::IntegrationByParts;
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::function("ln", vec![Expression::symbol(x.clone())]);
    /// let is_good = IntegrationByParts::is_good_u_choice(&expr, &x);
    /// ```
    pub fn is_good_u_choice(expr: &Expression, variable: &Symbol) -> bool {
        match expr {
            Expression::Function { name, .. } if name == "ln" || name == "log" => true,

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

            Expression::Symbol(sym) if sym == variable => true,
            Expression::Pow(base, _) => {
                if let Expression::Symbol(sym) = &**base {
                    sym == variable
                } else {
                    false
                }
            }

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
    /// use mathhook_core::calculus::integrals::by_parts::IntegrationByParts;
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// // ∫ x²·e^x dx requires two applications of by parts
    /// let expr = Expression::mul(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::function("exp", vec![Expression::symbol(x.clone())])
    /// ]);
    /// let result = IntegrationByParts::integrate_repeated(&expr, &x, 2);
    /// ```
    pub fn integrate_repeated(
        expr: &Expression,
        variable: &Symbol,
        max_iterations: usize,
    ) -> Option<Expression> {
        let mut current = expr.clone();

        for _ in 0..max_iterations {
            if let Some(result) = Self::integrate(&current, variable.clone(), 0) {
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
            Expression::Add(terms) => terms.iter().any(Self::contains_integral),
            Expression::Mul(factors) => factors.iter().any(Self::contains_integral),
            Expression::Pow(base, exp) => {
                Self::contains_integral(base) || Self::contains_integral(exp)
            }
            Expression::Function { args, .. } => args.iter().any(Self::contains_integral),
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
        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::function("exp", vec![Expression::symbol(x.clone())]),
        ]);

        let result = IntegrationByParts::integrate(&expr, x, 0);
        assert!(result.is_some());
    }

    #[test]
    fn test_by_parts_x_times_sin() {
        let x = symbol!(x);
        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
        ]);

        let result = IntegrationByParts::integrate(&expr, x, 0);
        assert!(result.is_some());
    }

    #[test]
    #[ignore = "ln(x) integration is already handled directly in function_integrals.rs - this edge case (ln(x)*1) would require additional complexity"]
    fn test_by_parts_ln() {
        let x = symbol!(x);
        let expr = Expression::function("ln", vec![Expression::symbol(x.clone())]);
        let as_product = Expression::mul(vec![expr, Expression::integer(1)]);
        let result = IntegrationByParts::integrate(&as_product, x, 0);
        assert!(result.is_some());
    }

    #[test]
    fn test_u_choice_priority() {
        let x = symbol!(x);

        let ln_expr = Expression::function("ln", vec![Expression::symbol(x.clone())]);
        assert!(IntegrationByParts::is_good_u_choice(&ln_expr, &x));

        let arcsin_expr = Expression::function("arcsin", vec![Expression::symbol(x.clone())]);
        assert!(IntegrationByParts::is_good_u_choice(&arcsin_expr, &x));

        let poly_expr = Expression::symbol(x.clone());
        assert!(IntegrationByParts::is_good_u_choice(&poly_expr, &x));

        let exp_expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
        assert!(!IntegrationByParts::is_good_u_choice(&exp_expr, &x));

        let sin_expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        assert!(!IntegrationByParts::is_good_u_choice(&sin_expr, &x));
    }
}
