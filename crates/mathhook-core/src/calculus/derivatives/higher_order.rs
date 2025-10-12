//! Higher-order derivative utilities

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;
use crate::symbol;

/// Higher-order derivative operations
pub struct HigherOrderDerivatives;

impl HigherOrderDerivatives {
    /// Compute nth-order derivative
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    /// use mathhook_core::calculus::derivatives::HigherOrderDerivatives;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(4));
    /// let second_derivative = HigherOrderDerivatives::compute(&expr, x, 2);
    /// ```
    pub fn compute(expr: &Expression, variable: Symbol, order: u32) -> Expression {
        match order {
            0 => expr.clone(),
            1 => expr.derivative(variable).simplify(),
            n => {
                let mut result = expr.clone();
                for _ in 0..n {
                    result = result.derivative(variable.clone()).simplify();
                }
                result
            }
        }
    }

    /// Compute mixed partial derivatives
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let expr = Expression::mul(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::symbol(y.clone())
    /// ]);
    /// let mixed_partial = HigherOrderDerivatives::mixed_partial(&expr, vec![(x, 1), (y, 1)]);
    /// ```
    pub fn mixed_partial(expr: &Expression, derivatives: Vec<(Symbol, u32)>) -> Expression {
        let mut result = expr.clone();

        for (variable, order) in derivatives {
            if order > 0 {
                result = Self::compute(&result, variable, order);
            }
        }

        result.simplify()
    }

    /// Check if higher-order derivative exists
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    /// let exists = HigherOrderDerivatives::exists(&expr, x, 5);
    /// ```
    pub fn exists(expr: &Expression, variable: Symbol, order: u32) -> bool {
        if order == 0 {
            return true;
        }

        if !expr.is_differentiable(variable.clone()) {
            return false;
        }

        if order == 1 {
            return true;
        }

        let first_derivative = expr.derivative(variable.clone());
        Self::exists(&first_derivative, variable, order - 1)
    }

    /// Compute derivative table up to specified order
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    /// let table = HigherOrderDerivatives::derivative_table(&expr, x, 4);
    /// ```
    pub fn derivative_table(
        expr: &Expression,
        variable: Symbol,
        max_order: u32,
    ) -> Vec<Expression> {
        let mut table = Vec::with_capacity((max_order + 1) as usize);
        let mut current = expr.clone();

        table.push(current.clone());

        for _ in 1..=max_order {
            current = current.derivative(variable.clone()).simplify();
            table.push(current.clone());
        }

        table
    }

    /// Find pattern in higher-order derivatives
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    /// let pattern = HigherOrderDerivatives::find_pattern(&expr, x, 8);
    /// ```
    pub fn find_pattern(expr: &Expression, variable: Symbol, check_orders: u32) -> Option<u32> {
        let derivatives = Self::derivative_table(expr, variable, check_orders);

        for period in 1..=(check_orders / 2) {
            if Self::has_period(&derivatives, period as usize) {
                return Some(period);
            }
        }

        None
    }

    /// Check if derivative sequence has a given period
    fn has_period(derivatives: &[Expression], period: usize) -> bool {
        if derivatives.len() < 2 * period {
            return false;
        }

        for i in 0..period {
            if derivatives[i] != derivatives[i + period] {
                return false;
            }
        }

        true
    }

    /// Compute Leibniz rule for nth derivative of product
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let u = Expression::symbol(x.clone());
    /// let v = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    /// let nth_product = HigherOrderDerivatives::leibniz_rule(&u, &v, x, 3);
    /// ```
    pub fn leibniz_rule(u: &Expression, v: &Expression, variable: Symbol, n: u32) -> Expression {
        let mut terms = Vec::with_capacity((n + 1) as usize);

        for k in 0..=n {
            let binomial_coeff = Self::binomial_coefficient(n, k);
            let u_derivative = Self::compute(u, variable.clone(), k);
            let v_derivative = Self::compute(v, variable.clone(), n - k);

            let term = Expression::mul(vec![
                Expression::integer(binomial_coeff),
                u_derivative,
                v_derivative,
            ]);

            terms.push(term);
        }

        Expression::add(terms).simplify()
    }

    /// Compute binomial coefficient C(n,k)
    fn binomial_coefficient(n: u32, k: u32) -> i64 {
        if k > n {
            return 0;
        }

        if k == 0 || k == n {
            return 1;
        }

        let k = k.min(n - k);
        let mut result = 1i64;

        for i in 0..k {
            result = result * (n - i) as i64 / (i + 1) as i64;
        }

        result
    }

    /// Compute Faà di Bruno's formula for chain rule higher derivatives
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let inner = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
    /// let outer_name = "sin";
    /// let chain_derivative = HigherOrderDerivatives::faa_di_bruno(outer_name, &inner, x, 2);
    /// ```
    pub fn faa_di_bruno(
        outer_function: &str,
        inner_function: &Expression,
        variable: Symbol,
        n: u32,
    ) -> Expression {
        if n == 0 {
            return Expression::function(outer_function, vec![inner_function.clone()]);
        }

        if n == 1 {
            let outer_derivative = Self::get_function_derivative(outer_function, inner_function);
            let inner_derivative = inner_function.derivative(variable);
            return Expression::mul(vec![outer_derivative, inner_derivative]).simplify();
        }

        Expression::derivative(
            Expression::function(outer_function, vec![inner_function.clone()]),
            variable,
            n,
        )
    }

    /// Get derivative of standard function
    fn get_function_derivative(function_name: &str, arg: &Expression) -> Expression {
        match function_name {
            "sin" => Expression::function("cos", vec![arg.clone()]),
            "cos" => Expression::mul(vec![
                Expression::integer(-1),
                Expression::function("sin", vec![arg.clone()]),
            ]),
            "exp" => Expression::function("exp", vec![arg.clone()]),
            "ln" => Expression::mul(vec![
                Expression::integer(1),
                Expression::pow(arg.clone(), Expression::integer(-1)),
            ]),
            _ => Expression::derivative(
                Expression::function(function_name, vec![arg.clone()]),
                symbol!(x),
                1,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_polynomial_higher_derivatives() {
        let x = symbol!(x);

        let x4 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(4));

        let first = HigherOrderDerivatives::compute(&x4, x.clone(), 1);
        assert!(!first.is_zero());

        let second = HigherOrderDerivatives::compute(&x4, x.clone(), 2);
        assert!(!second.is_zero());

        let third = HigherOrderDerivatives::compute(&x4, x.clone(), 3);
        assert!(!third.is_zero());

        let fourth = HigherOrderDerivatives::compute(&x4, x.clone(), 4);
        assert!(!fourth.is_zero());

        let fifth = HigherOrderDerivatives::compute(&x4, x.clone(), 5);
        assert_eq!(fifth.simplify(), Expression::integer(0));
    }

    #[test]
    fn test_exponential_derivatives() {
        let x = symbol!(x);

        let exp_x = Expression::function("exp", vec![Expression::symbol(x.clone())]);

        for order in 1..=5 {
            let derivative = HigherOrderDerivatives::compute(&exp_x, x.clone(), order);
            assert!(!derivative.is_zero());
        }
    }

    #[test]
    fn test_trigonometric_patterns() {
        let x = symbol!(x);

        let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        let cos_x = Expression::function("cos", vec![Expression::symbol(x.clone())]);

        let sin_table = HigherOrderDerivatives::derivative_table(&sin_x, x.clone(), 8);
        assert_eq!(sin_table.len(), 9);

        let cos_table = HigherOrderDerivatives::derivative_table(&cos_x, x.clone(), 8);
        assert_eq!(cos_table.len(), 9);

        let sin_pattern = HigherOrderDerivatives::find_pattern(&sin_x, x.clone(), 8);
        assert!(sin_pattern.is_some());

        let cos_pattern = HigherOrderDerivatives::find_pattern(&cos_x, x.clone(), 8);
        assert!(cos_pattern.is_some());
    }

    #[test]
    fn test_mixed_partial_derivatives() {
        let x = symbol!(x);
        let y = symbol!(y);

        let xy = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);

        let mixed_xy =
            HigherOrderDerivatives::mixed_partial(&xy, vec![(x.clone(), 1), (y.clone(), 1)]);
        assert!(!mixed_xy.is_zero());

        let x2y = Expression::mul(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(y.clone()),
        ]);

        let mixed_x2y =
            HigherOrderDerivatives::mixed_partial(&x2y, vec![(x.clone(), 2), (y.clone(), 1)]);
        assert!(!mixed_x2y.is_zero());
    }

    #[test]
    fn test_derivative_existence() {
        let x = symbol!(x);

        let smooth_expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        assert!(HigherOrderDerivatives::exists(&smooth_expr, x.clone(), 0));
        assert!(HigherOrderDerivatives::exists(&smooth_expr, x.clone(), 1));
        assert!(HigherOrderDerivatives::exists(&smooth_expr, x.clone(), 10));

        let polynomial = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
        assert!(HigherOrderDerivatives::exists(&polynomial, x.clone(), 3));
        assert!(HigherOrderDerivatives::exists(&polynomial, x.clone(), 4));
    }

    #[test]
    fn test_leibniz_rule() {
        let x = symbol!(x);

        let u = Expression::symbol(x.clone());
        let v = Expression::function("sin", vec![Expression::symbol(x.clone())]);

        let leibniz_2 = HigherOrderDerivatives::leibniz_rule(&u, &v, x.clone(), 2);
        assert!(!leibniz_2.is_zero());

        let leibniz_3 = HigherOrderDerivatives::leibniz_rule(&u, &v, x.clone(), 3);
        assert!(!leibniz_3.is_zero());

        let u_poly = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let v_exp = Expression::function("exp", vec![Expression::symbol(x.clone())]);

        let leibniz_poly = HigherOrderDerivatives::leibniz_rule(&u_poly, &v_exp, x.clone(), 2);
        assert!(!leibniz_poly.is_zero());
    }

    #[test]
    fn test_binomial_coefficients() {
        assert_eq!(HigherOrderDerivatives::binomial_coefficient(0, 0), 1);
        assert_eq!(HigherOrderDerivatives::binomial_coefficient(1, 0), 1);
        assert_eq!(HigherOrderDerivatives::binomial_coefficient(1, 1), 1);
        assert_eq!(HigherOrderDerivatives::binomial_coefficient(2, 1), 2);
        assert_eq!(HigherOrderDerivatives::binomial_coefficient(3, 2), 3);
        assert_eq!(HigherOrderDerivatives::binomial_coefficient(4, 2), 6);
        assert_eq!(HigherOrderDerivatives::binomial_coefficient(5, 3), 10);
        assert_eq!(HigherOrderDerivatives::binomial_coefficient(10, 5), 252);

        assert_eq!(HigherOrderDerivatives::binomial_coefficient(5, 6), 0);
    }

    #[test]
    fn test_faa_di_bruno_formula() {
        let x = symbol!(x);

        let inner = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

        let chain_1 = HigherOrderDerivatives::faa_di_bruno("sin", &inner, x.clone(), 1);
        assert!(!chain_1.is_zero());

        let chain_2 = HigherOrderDerivatives::faa_di_bruno("sin", &inner, x.clone(), 2);
        assert!(!chain_2.is_zero());

        let exp_inner = Expression::function("exp", vec![Expression::symbol(x.clone())]);
        let exp_chain = HigherOrderDerivatives::faa_di_bruno("ln", &exp_inner, x.clone(), 1);
        assert!(!exp_chain.is_zero());
    }

    #[test]
    fn test_zero_order_derivative() {
        let x = symbol!(x);

        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::integer(5),
        ]);

        let zero_order = HigherOrderDerivatives::compute(&expr, x.clone(), 0);
        assert_eq!(zero_order, expr);
    }

    #[test]
    fn test_multivariate_higher_derivatives() {
        let x = symbol!(x);
        let y = symbol!(y);

        let multivar = Expression::add(vec![
            Expression::mul(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::pow(Expression::symbol(y.clone()), Expression::integer(3)),
            ]),
            Expression::function(
                "sin",
                vec![Expression::mul(vec![
                    Expression::symbol(x.clone()),
                    Expression::symbol(y.clone()),
                ])],
            ),
        ]);

        let dx2 = HigherOrderDerivatives::compute(&multivar, x.clone(), 2);
        assert!(!dx2.is_zero());

        let dy3 = HigherOrderDerivatives::compute(&multivar, y.clone(), 3);
        assert!(!dy3.is_zero());

        let mixed =
            HigherOrderDerivatives::mixed_partial(&multivar, vec![(x.clone(), 1), (y.clone(), 2)]);
        assert!(!mixed.is_zero());
    }

    #[test]
    fn test_edge_cases() {
        let x = symbol!(x);

        let constant = Expression::integer(42);
        let const_deriv = HigherOrderDerivatives::compute(&constant, x.clone(), 5);
        assert_eq!(const_deriv.simplify(), Expression::integer(0));

        let linear = Expression::symbol(x.clone());
        let linear_second = HigherOrderDerivatives::compute(&linear, x.clone(), 2);
        assert_eq!(linear_second.simplify(), Expression::integer(0));

        let empty_mixed = HigherOrderDerivatives::mixed_partial(&linear, vec![]);
        assert_eq!(empty_mixed, linear);
    }
}
