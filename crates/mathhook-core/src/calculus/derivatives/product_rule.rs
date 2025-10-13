//! Product rule implementation for derivatives

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;

/// Product rule implementation for two factors
pub struct ProductRule;

impl ProductRule {
    /// Handle derivative of product expressions using product rule
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, ProductRule};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::Derivative;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::mul(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::function("sin", vec![Expression::symbol(x.clone())])
    /// ]);
    /// let result = expr.derivative(x);
    /// ```
    pub fn handle_product(factors: &[Expression], variable: Symbol) -> Expression {
        match factors.len() {
            0 => Expression::integer(0),
            1 => factors[0].derivative(variable),
            2 => Self::apply(&factors[0], &factors[1], variable),
            _ => GeneralProductRule::apply(factors, variable),
        }
    }

    /// Apply product rule for two expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, ProductRule};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let u = Expression::symbol(x.clone());
    /// let v = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    /// let result = ProductRule::apply(&u, &v, x);
    /// ```
    pub fn apply(u: &Expression, v: &Expression, variable: Symbol) -> Expression {
        let du = u.derivative(variable.clone());
        let dv = v.derivative(variable);

        Expression::add(vec![
            Expression::mul(vec![du, v.clone()]),
            Expression::mul(vec![u.clone(), dv]),
        ])
        .simplify()
    }
}

/// General product rule for multiple factors
pub struct GeneralProductRule;

impl GeneralProductRule {
    /// Apply product rule for multiple factors
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::derivatives::GeneralProductRule;
    ///
    /// let x = symbol!(x);
    /// let factors = vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::function("sin", vec![Expression::symbol(x.clone())]),
    ///     Expression::function("cos", vec![Expression::symbol(x.clone())])
    /// ];
    /// let result = GeneralProductRule::apply(&factors, x.clone());
    /// ```
    pub fn apply(factors: &[Expression], variable: Symbol) -> Expression {
        let derivative_terms: Vec<Expression> = (0..factors.len())
            .map(|i| {
                let term_factors: Vec<Expression> = factors
                    .iter()
                    .enumerate()
                    .map(|(j, factor)| {
                        if i == j {
                            factor.derivative(variable.clone())
                        } else {
                            factor.clone()
                        }
                    })
                    .collect();
                Expression::mul(term_factors)
            })
            .collect();

        Expression::add(derivative_terms).simplify()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_basic_product_rule() {
        let x = symbol!(x);

        let u_v = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]);
        let expected = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(1),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            ]),
        ]);
        assert_eq!(u_v.derivative(x.clone()).simplify(), expected.simplify());

        let x_sin_x = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
        ]);
        let result = x_sin_x.derivative(x.clone());
        assert!(!result.is_zero());
    }

    #[test]
    fn test_constant_products() {
        let x = symbol!(x);

        let const_x = Expression::mul(vec![Expression::integer(5), Expression::symbol(x.clone())]);
        assert_eq!(
            const_x.derivative(x.clone()).simplify(),
            Expression::integer(5)
        );

        let const_x2 = Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]);
        let expected = Expression::mul(vec![
            Expression::integer(3),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        ]);
        assert_eq!(
            const_x2.derivative(x.clone()).simplify(),
            expected.simplify()
        );

        let pi_x = Expression::mul(vec![
            Expression::constant(crate::MathConstant::Pi),
            Expression::symbol(x.clone()),
        ]);
        assert_eq!(
            pi_x.derivative(x.clone()).simplify(),
            Expression::constant(crate::MathConstant::Pi)
        );
    }

    #[test]
    fn test_polynomial_products() {
        let x = symbol!(x);

        let poly1 = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(x.clone()),
            Expression::integer(1),
        ]);

        let poly2 = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(2)]);

        let product = Expression::mul(vec![poly1, poly2]);
        let result = product.derivative(x.clone());
        assert!(!result.is_zero());
    }

    #[test]
    fn test_function_products() {
        let x = symbol!(x);

        let sin_cos = Expression::mul(vec![
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::function("cos", vec![Expression::symbol(x.clone())]),
        ]);
        let result = sin_cos.derivative(x.clone());
        assert!(!result.is_zero());

        let exp_ln = Expression::mul(vec![
            Expression::function("exp", vec![Expression::symbol(x.clone())]),
            Expression::function("ln", vec![Expression::symbol(x.clone())]),
        ]);
        let result_exp_ln = exp_ln.derivative(x.clone());
        assert!(!result_exp_ln.is_zero());
    }

    #[test]
    fn test_three_factor_products() {
        let x = symbol!(x);

        let xyz = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
        ]);
        let result = xyz.derivative(x.clone());
        assert!(!result.is_zero());

        let const_multi = Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
            Expression::function("exp", vec![Expression::symbol(x.clone())]),
        ]);
        let result_const = const_multi.derivative(x.clone());
        assert!(!result_const.is_zero());
    }

    #[test]
    fn test_zero_products() {
        let x = symbol!(x);

        let zero_x = Expression::mul(vec![Expression::integer(0), Expression::symbol(x.clone())]);
        assert_eq!(
            zero_x.derivative(x.clone()).simplify(),
            Expression::integer(0)
        );

        let x_zero = Expression::mul(vec![Expression::symbol(x.clone()), Expression::integer(0)]);
        assert_eq!(
            x_zero.derivative(x.clone()).simplify(),
            Expression::integer(0)
        );

        let zero_sin = Expression::mul(vec![
            Expression::integer(0),
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
        ]);
        assert_eq!(
            zero_sin.derivative(x.clone()).simplify(),
            Expression::integer(0)
        );
    }

    #[test]
    fn test_multivariate_products() {
        let x = symbol!(x);
        let y = symbol!(y);

        let xy = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);
        assert_eq!(
            xy.derivative(x.clone()).simplify(),
            Expression::symbol(y.clone())
        );
        assert_eq!(
            xy.derivative(y.clone()).simplify(),
            Expression::symbol(x.clone())
        );

        let x2y = Expression::mul(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(y.clone()),
        ]);
        let expected_dx = Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);
        assert_eq!(x2y.derivative(x.clone()).simplify(), expected_dx.simplify());
        assert_eq!(
            x2y.derivative(y.clone()).simplify(),
            Expression::pow(Expression::symbol(x), Expression::integer(2))
        );
    }

    #[test]
    fn test_nested_products() {
        let x = symbol!(x);

        let nested = Expression::mul(vec![
            Expression::mul(vec![Expression::symbol(x.clone()), Expression::integer(2)]),
            Expression::mul(vec![Expression::symbol(x.clone()), Expression::integer(3)]),
        ]);
        let result = nested.derivative(x.clone());
        assert!(!result.is_zero());

        let deep_nested = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::mul(vec![Expression::symbol(x.clone()), Expression::integer(4)]),
            ]),
        ]);
        let result_deep = deep_nested.derivative(x.clone());
        assert!(!result_deep.is_zero());
    }
}
