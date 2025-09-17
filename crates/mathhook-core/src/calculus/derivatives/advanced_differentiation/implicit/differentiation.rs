//! Implicit differentiation operations for implicitly defined functions
//!
//! Handles differentiation where y is defined implicitly through F(x,y) = 0.

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;

/// Implicit differentiation operations
pub struct ImplicitDifferentiation;

impl ImplicitDifferentiation {
    /// Compute dy/dx for implicitly defined function F(x,y) = 0
    ///
    /// Uses the formula: dy/dx = -∂F/∂x / ∂F/∂y
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::derivatives::ImplicitDifferentiation;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// // For x² + y² = 1 (circle)
    /// let equation = expr!((x^2) + (y^2));
    /// let dy_dx = ImplicitDifferentiation::compute(&equation, x, y);
    /// ```
    pub fn compute(
        equation: &Expression,
        independent_var: Symbol,
        dependent_var: Symbol,
    ) -> Expression {
        let partial_x = equation.derivative(independent_var);
        let partial_y = equation.derivative(dependent_var);

        let numerator = Expression::mul(vec![Expression::integer(-1), partial_x]);
        Self::create_division(&numerator, &partial_y).simplify()
    }

    /// Compute higher-order implicit derivatives
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::derivatives::ImplicitDifferentiation;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let equation = expr!((x^2) + (y^2));
    /// let d2y_dx2 = ImplicitDifferentiation::higher_order(&equation, x, y, 2);
    /// ```
    pub fn higher_order(
        equation: &Expression,
        independent_var: Symbol,
        dependent_var: Symbol,
        order: u32,
    ) -> Expression {
        if order == 0 {
            return Expression::symbol(dependent_var);
        }

        if order == 1 {
            return Self::compute(equation, independent_var, dependent_var);
        }

        let first_derivative = Self::compute(equation, independent_var.clone(), dependent_var);

        Self::differentiate_implicit_result(&first_derivative, independent_var, order - 1)
    }

    /// Handle multiple implicit variables
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::derivatives::Derivative;
    /// use mathhook_core::{expr, symbol, Expression};
    /// use mathhook_core::calculus::derivatives::ImplicitDifferentiation;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let z = symbol!(z);
    /// // For F(x,y,z) = 0, compute ∂z/∂x and ∂z/∂y
    /// let equation = Expression::add(vec![expr!(x), expr!(y), expr!(z)]);
    /// let partials = ImplicitDifferentiation::multiple_variables(
    ///     &equation,
    ///     vec![x, y],
    ///     &z
    /// );
    /// ```
    pub fn multiple_variables(
        equation: &Expression,
        independent_vars: Vec<Symbol>,
        dependent_var: &Symbol,
    ) -> Vec<Expression> {
        let mut results = Vec::with_capacity(independent_vars.len());

        for var in independent_vars {
            let partial = Self::compute(equation, var, dependent_var.clone());
            results.push(partial);
        }

        results
    }

    /// Create division expression (helper for fractions)
    pub(super) fn create_division(numerator: &Expression, denominator: &Expression) -> Expression {
        Expression::mul(vec![
            numerator.clone(),
            Expression::pow(denominator.clone(), Expression::integer(-1)),
        ])
    }

    /// Differentiate implicit result for higher orders
    fn differentiate_implicit_result(
        expr: &Expression,
        independent_var: Symbol,
        remaining_order: u32,
    ) -> Expression {
        if remaining_order == 0 {
            return expr.clone();
        }

        let derivative = expr.derivative(independent_var.clone());

        Self::differentiate_implicit_result(&derivative, independent_var, remaining_order - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_circle_implicit_derivative() {
        let x = symbol!(x);
        let y = symbol!(y);

        let circle = expr!((x ^ 2) + (y ^ 2));

        let dy_dx = ImplicitDifferentiation::compute(&circle, x.clone(), y.clone());
        assert!(!dy_dx.is_zero());
    }

    #[test]
    fn test_ellipse_implicit_derivative() {
        let x = symbol!(x);
        let y = symbol!(y);

        let ellipse = expr!((4 * (x ^ 2)) + (y ^ 2));

        let dy_dx = ImplicitDifferentiation::compute(&ellipse, x.clone(), y.clone());
        assert!(!dy_dx.is_zero());
    }

    #[test]
    fn test_hyperbola_implicit_derivative() {
        let x = symbol!(x);
        let y = symbol!(y);

        let hyperbola = expr!((x ^ 2) + ((-1) * (y ^ 2)));

        let dy_dx = ImplicitDifferentiation::compute(&hyperbola, x.clone(), y.clone());
        assert!(!dy_dx.is_zero());
    }

    #[test]
    fn test_parabola_implicit_derivative() {
        let x = symbol!(x);
        let y = symbol!(y);

        let parabola = expr!((y ^ 2) + ((-4) * x));

        let dy_dx = ImplicitDifferentiation::compute(&parabola, x.clone(), y.clone());
        assert!(!dy_dx.is_zero());
    }

    #[test]
    fn test_cubic_curve_implicit_derivative() {
        let x = symbol!(x);
        let y = symbol!(y);

        let cubic = Expression::add(vec![
            expr!(x ^ 3),
            expr!(y ^ 3),
            Expression::mul(vec![
                Expression::integer(-3),
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
        ]);

        let dy_dx = ImplicitDifferentiation::compute(&cubic, x.clone(), y.clone());
        assert!(!dy_dx.is_zero());
    }

    #[test]
    fn test_linear_implicit_derivative() {
        let x = symbol!(x);
        let y = symbol!(y);

        let linear = Expression::add(vec![expr!(2 * x), expr!(3 * y), Expression::integer(-6)]);

        let dy_dx = ImplicitDifferentiation::compute(&linear, x.clone(), y.clone());
        assert!(!dy_dx.is_zero());
    }

    #[test]
    fn test_higher_order_derivatives() {
        let x = symbol!(x);
        let y = symbol!(y);

        let circle = expr!((x ^ 2) + (y ^ 2));

        let first = ImplicitDifferentiation::higher_order(&circle, x.clone(), y.clone(), 1);
        let second = ImplicitDifferentiation::higher_order(&circle, x.clone(), y.clone(), 2);
        let _third = ImplicitDifferentiation::higher_order(&circle, x.clone(), y.clone(), 3);

        assert!(!first.is_zero());
        assert!(!second.is_zero());
    }

    #[test]
    fn test_zero_order_derivative() {
        let x = symbol!(x);
        let y = symbol!(y);

        let equation = expr!(x ^ 2);
        let zero_order = ImplicitDifferentiation::higher_order(&equation, x.clone(), y.clone(), 0);

        assert_eq!(zero_order, Expression::symbol(y));
    }

    #[test]
    fn test_multiple_variables_derivatives() {
        let x = symbol!(x);
        let y = symbol!(y);
        let z = symbol!(z);

        let equation = expr!((x ^ 2) + (y ^ 2) + (z ^ 2));

        let partials =
            ImplicitDifferentiation::multiple_variables(&equation, vec![x.clone(), y.clone()], &z);

        assert_eq!(partials.len(), 2);
        assert!(!partials[0].is_zero());
        assert!(!partials[1].is_zero());
    }

    #[test]
    fn test_trigonometric_implicit_curves() {
        let x = symbol!(x);
        let y = symbol!(y);

        let trig_curve = Expression::add(vec![expr!(sin(x)), expr!(cos(y))]);

        let dy_dx = ImplicitDifferentiation::compute(&trig_curve, x.clone(), y.clone());
        assert!(!dy_dx.is_zero());
    }

    #[test]
    fn test_exponential_implicit_curves() {
        let x = symbol!(x);
        let y = symbol!(y);

        let exp_curve = Expression::add(vec![expr!(exp(x)), expr!(ln(y))]);

        let dy_dx = ImplicitDifferentiation::compute(&exp_curve, x.clone(), y.clone());
        assert!(!dy_dx.is_zero());
    }

    #[test]
    fn test_mixed_polynomial_trigonometric() {
        let x = symbol!(x);
        let y = symbol!(y);

        let mixed = Expression::add(vec![expr!(x ^ 2), expr!(sin(y))]);

        let dy_dx = ImplicitDifferentiation::compute(&mixed, x.clone(), y.clone());
        assert!(!dy_dx.is_zero());
    }

    #[test]
    fn test_rational_function_implicit() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rational = expr!((x * y) + (-1));

        let dy_dx = ImplicitDifferentiation::compute(&rational, x.clone(), y.clone());
        assert!(!dy_dx.is_zero());
    }

    #[test]
    fn test_parametric_like_implicit() {
        let x = symbol!(x);
        let y = symbol!(y);

        let parametric_form = expr!(((x + (-1)) ^ 2) + ((y + (-2)) ^ 2));

        let dy_dx = ImplicitDifferentiation::compute(&parametric_form, x.clone(), y.clone());
        assert!(!dy_dx.is_zero());
    }

    #[test]
    fn test_implicit_differentiation_chain_rule() {
        let x = symbol!(x);
        let y = symbol!(y);

        let chain_curve = Expression::function(
            "sin",
            vec![Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ])],
        );

        let dy_dx = ImplicitDifferentiation::compute(&chain_curve, x.clone(), y.clone());
        assert!(!dy_dx.is_zero());
    }

    #[test]
    fn test_multiple_independent_variables() {
        let x = symbol!(x);
        let y = symbol!(y);
        let z = symbol!(z);
        let w = symbol!(w);

        let multi_var = expr!(x + y + z + w);

        let partials = ImplicitDifferentiation::multiple_variables(
            &multi_var,
            vec![x.clone(), y.clone(), z.clone()],
            &w,
        );

        assert_eq!(partials.len(), 3);
        for partial in partials {
            assert!(!partial.is_zero());
        }
    }

    #[test]
    fn test_constant_equation_implicit() {
        let x = symbol!(x);
        let y = symbol!(y);

        let constant_eq = Expression::integer(5);
        let dy_dx = ImplicitDifferentiation::compute(&constant_eq, x.clone(), y.clone());

        assert!(!dy_dx.is_zero());
    }
}
