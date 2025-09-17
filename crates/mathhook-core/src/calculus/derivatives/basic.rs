//! Basic differentiation rules for constants, symbols, and sums

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;

/// Basic derivative operations
pub struct BasicDerivatives;

impl BasicDerivatives {
    /// Handle derivative of calculus expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::derivatives::Derivative;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::derivative(Expression::symbol(x.clone()), x.clone(), 1);
    /// let second = expr.derivative(x);
    /// ```
    pub fn handle_calculus(
        expr: &Expression,
        data: &crate::core::expression::CalculusData,
        variable: Symbol,
    ) -> Expression {
        match data {
            crate::core::expression::CalculusData::Derivative {
                variable: var,
                order,
                ..
            } => {
                if *var == variable {
                    Expression::derivative(expr.clone(), variable, order + 1)
                } else {
                    Expression::integer(0) // d/dx[f(y)] = 0
                }
            }
            _ => Expression::derivative(expr.clone(), variable, 1),
        }
    }

    /// Handle derivative of symbol expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::derivatives::Derivative;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let expr = Expression::symbol(x.clone());
    /// let dx = expr.derivative(x);
    /// let dy = expr.derivative(y);
    /// ```
    pub fn handle_symbol(sym: &Symbol, variable: &Symbol) -> Expression {
        if sym == variable {
            Expression::integer(1) // dx/dx = 1
        } else {
            Expression::integer(0) // dy/dx = 0
        }
    }

    /// Handle derivative of sum expressions using linearity
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::Derivative;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::add(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::integer(5)
    /// ]);
    /// let result = expr.derivative(x);
    /// ```
    pub fn handle_sum(terms: &[Expression], variable: &Symbol) -> Expression {
        let mut derivative_terms = Vec::with_capacity(terms.len());

        for term in terms {
            derivative_terms.push(term.derivative(variable.clone()));
        }

        Expression::add(derivative_terms).simplify() // d/dx[f + g] = f' + g'
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;
    use crate::MathConstant;

    #[test]
    fn test_basic_constant_derivatives() {
        let x = symbol!(x);
        let y = symbol!(y);

        assert_eq!(
            Expression::integer(2).derivative(x.clone()),
            Expression::integer(0) // d/dx[2] = 0
        );
        assert_eq!(
            Expression::symbol(x.clone()).derivative(x.clone()),
            Expression::integer(1) // dx/dx = 1
        );
        assert_eq!(
            Expression::symbol(x.clone()).derivative(y.clone()),
            Expression::integer(0) // dx/dy = 0
        );
        assert_eq!(
            Expression::integer(-1).derivative(x.clone()),
            Expression::integer(0) // d/dx[-1] = 0
        );
        assert_eq!(
            Expression::constant(MathConstant::Pi).derivative(x.clone()),
            Expression::integer(0) // d/dx[π] = 0
        );
    }

    #[test]
    fn test_sum_linearity() {
        let x = symbol!(x);

        // d/dx[x + 5] = 1 + 0 = 1
        let sum = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(5)]);
        let result = sum.derivative(x.clone()).simplify();
        assert_eq!(result, Expression::integer(1));

        // d/dx[2x + 3x] = 2 + 3 = 5
        let linear_combo = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        ]);
        let linear_result = linear_combo.derivative(x.clone()).simplify();
        assert_eq!(linear_result, Expression::integer(5));
    }

    #[test]
    fn test_multiple_variables() {
        let x = symbol!(x);
        let y = symbol!(y);

        // f(x,y) = xy + x² + y
        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(y.clone()),
        ]);

        // ∂f/∂x = y + 2x
        let dx = expr.derivative(x.clone()).simplify();
        let expected_dx = Expression::add(vec![
            Expression::symbol(y.clone()),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        ])
        .simplify();

        // ∂f/∂y = x + 1
        let dy = expr.derivative(y.clone()).simplify();
        let expected_dy =
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]).simplify();

        assert_eq!(dx, expected_dx);
        assert_eq!(dy, expected_dy);
    }

    #[test]
    fn test_special_constants() {
        let x = symbol!(x);

        // d/dx[π] = 0
        let pi_derivative = Expression::constant(MathConstant::Pi).derivative(x.clone());
        assert_eq!(pi_derivative, Expression::integer(0));

        // d/dx[e] = 0
        let e_derivative = Expression::constant(MathConstant::E).derivative(x.clone());
        assert_eq!(e_derivative, Expression::integer(0));

        // d/dx[i] = 0
        let i_derivative = Expression::constant(MathConstant::I).derivative(x.clone());
        assert_eq!(i_derivative, Expression::integer(0));
    }

    #[test]
    fn test_nested_sums() {
        let x = symbol!(x);

        // d/dx[x + (2x + 3)] = d/dx[3x + 3] = 3
        let nested = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::add(vec![
                Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
                Expression::integer(3),
            ]),
        ]);

        let result = nested.derivative(x.clone()).simplify();
        assert_eq!(result, Expression::integer(3));
    }

    #[test]
    fn test_zero_and_negative_constants() {
        let x = symbol!(x);

        // d/dx[0] = 0
        assert_eq!(
            Expression::integer(0).derivative(x.clone()),
            Expression::integer(0)
        );

        // d/dx[-42] = 0
        assert_eq!(
            Expression::integer(-42).derivative(x.clone()),
            Expression::integer(0)
        );

        // d/dx[3.14] = 0
        assert_eq!(
            Expression::float(std::f64::consts::PI).derivative(x.clone()),
            Expression::integer(0)
        );
    }
}
