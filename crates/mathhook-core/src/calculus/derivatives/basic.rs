//! Basic differentiation rules for constants, symbols, and sums

use crate::algebra::simplify::Simplify;
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};

/// Basic derivative operations
pub struct BasicDerivatives;

impl BasicDerivatives {
    /// Handle derivative of calculus expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let expr = Expression::derivative(Expression::symbol(x.clone()), x, 1);
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
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
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
    /// use mathhook_core::{Expression, Symbol};
    /// use mathhook_core::calculus::derivatives::Derivative;
    ///
    /// let x = Symbol::new("x");
    /// let expr = Expression::add(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::integer(5)
    /// ]);
    /// let result = expr.derivative(x);
    /// ```
    pub fn handle_sum(terms: &[Expression], variable: Symbol) -> Expression {
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
    use crate::MathConstant;

    #[test]
    fn test_basic_constant_derivatives() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

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
        let x = Symbol::new("x");

        // d/dx[x + 1] = 1
        let expr1 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
        assert_eq!(
            expr1.derivative(x.clone()).simplify(),
            Expression::integer(1)
        );

        // d/dx[2x + 3] = 2
        let expr2 = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::integer(3),
        ]);
        assert_eq!(
            expr2.derivative(x.clone()).simplify(),
            Expression::integer(2)
        );

        // d/dx[x + x] = 2
        let expr3 = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(x.clone()),
        ]);
        assert_eq!(
            expr3.derivative(x.clone()).simplify(),
            Expression::integer(2)
        );
    }

    #[test]
    fn test_multivariate_expressions() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

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
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x)]),
        ])
        .simplify();

        // ∂f/∂y = x + 1
        let dy = expr.derivative(y.clone()).simplify();
        let expected_dy =
            Expression::add(vec![Expression::symbol(x), Expression::integer(1)]).simplify();

        assert_eq!(dx, expected_dx);
        assert_eq!(dy, expected_dy);
    }

    #[test]
    fn test_special_constants() {
        let x = Symbol::new("x");

        // d/dx[πx] = π
        let pi_expr = Expression::mul(vec![
            Expression::constant(MathConstant::Pi),
            Expression::symbol(x.clone()),
        ]);
        assert_eq!(
            pi_expr.derivative(x.clone()).simplify(),
            Expression::constant(MathConstant::Pi)
        );

        // d/dx[e + x] = 1
        let e_expr = Expression::add(vec![
            Expression::constant(MathConstant::E),
            Expression::symbol(x.clone()),
        ]);
        assert_eq!(
            e_expr.derivative(x.clone()).simplify(),
            Expression::integer(1)
        );
    }
}
