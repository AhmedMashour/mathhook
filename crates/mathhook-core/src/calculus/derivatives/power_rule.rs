//! Power rule implementation for derivatives

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;

/// Power rule implementation
pub struct PowerRule;

impl PowerRule {
    /// Apply power rule and chain rule for power expressions
    ///
    /// # Examples
    ///
    /// ```rust
/// use mathhook_core::simplify::Simplify;
/// use mathhook_core::calculus::derivatives::Derivative;
    /// use mathhook_core::{Expression, PowerRule};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let base = Expression::symbol(x.clone());
    /// let exponent = Expression::integer(2);
    /// let result = PowerRule::apply(&base, &exponent, x);
    /// ```
    pub fn apply(base: &Expression, exponent: &Expression, variable: Symbol) -> Expression {
        match (base, exponent) {
            (Expression::Symbol(sym), Expression::Number(Number::Integer(n)))
                if *sym == variable =>
            {
                Self::simple_power_rule(*n, variable) // d/dx[x^n] = nx^(n-1)
            }
            _ => Self::logarithmic_differentiation(base, exponent, variable),
        }
    }

    /// Handle simple power rule for symbol raised to integer power
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, PowerRule};
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// let result = PowerRule::simple_power_rule(3, x);
    /// ```
    pub fn simple_power_rule(n: i64, variable: Symbol) -> Expression {
        match n {
            0 => Expression::integer(0), // d/dx[x^0] = d/dx[1] = 0
            1 => Expression::integer(1), // d/dx[x^1] = d/dx[x] = 1
            _ => Expression::mul(vec![
                Expression::integer(n),
                Expression::pow(Expression::symbol(variable), Expression::integer(n - 1)),
            ]), // d/dx[x^n] = nx^(n-1)
        }
    }

    /// Handle general power using logarithmic differentiation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::derivatives::PowerRule;
    ///
    /// let x = symbol!(x);
    /// let base = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    /// let exponent = Expression::symbol(x.clone());
    /// let result = PowerRule::logarithmic_differentiation(&base, &exponent, x.clone());
    /// ```
    pub fn logarithmic_differentiation(
        base: &Expression,
        exponent: &Expression,
        variable: Symbol,
    ) -> Expression {
        let ln_base = Expression::function("ln", vec![base.clone()]);
        let exp_derivative = exponent.derivative(variable.clone());
        let base_derivative = base.derivative(variable);

        let original_expr = Expression::pow(base.clone(), exponent.clone());

        // d/dx[f^g] = f^g * (g'ln(f) + g*f'/f)
        Expression::mul(vec![
            original_expr,
            Expression::add(vec![
                Expression::mul(vec![exp_derivative, ln_base]),
                Expression::mul(vec![
                    exponent.clone(),
                    Self::div(base_derivative, base.clone()),
                ]),
            ]),
        ])
        .simplify()
    }

    /// Create division expression helper for derivatives
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::derivatives::PowerRule;
    ///
    /// let x = symbol!(x);
    /// let numerator = Expression::integer(1);
    /// let denominator = Expression::symbol(x);
    /// let division = PowerRule::div(numerator, denominator);
    /// ```
    pub fn div(numerator: Expression, denominator: Expression) -> Expression {
        Expression::mul(vec![
            numerator,
            Expression::pow(denominator, Expression::integer(-1)),
        ]) // a/b = a * b^(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_integer_powers() {
        let x = symbol!(x);

        // d/dx[x^0] = 0
        let x0 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(0));
        assert_eq!(x0.derivative(x.clone()).simplify(), Expression::integer(0));

        // d/dx[x^1] = 1
        let x1 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(1));
        assert_eq!(x1.derivative(x.clone()).simplify(), Expression::integer(1));

        // d/dx[x^2] = 2x
        let x2 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let expected_x2 =
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);
        assert_eq!(x2.derivative(x.clone()).simplify(), expected_x2.simplify());

        // d/dx[x^3] = 3x^2
        let x3 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
        let expected_x3 = Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]);
        assert_eq!(x3.derivative(x.clone()).simplify(), expected_x3.simplify());

        // d/dx[x^5] = 5x^4
        let x5 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(5));
        let expected_x5 = Expression::mul(vec![
            Expression::integer(5),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(4)),
        ]);
        assert_eq!(x5.derivative(x.clone()).simplify(), expected_x5.simplify());
    }

    #[test]
    fn test_negative_powers() {
        let x = symbol!(x);

        // d/dx[x^(-1)] = -x^(-2)
        let x_neg1 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));
        let expected_neg1 = Expression::mul(vec![
            Expression::integer(-1),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(-2)),
        ]);
        assert_eq!(
            x_neg1.derivative(x.clone()).simplify(),
            expected_neg1.simplify()
        );

        // d/dx[x^(-2)] = -2x^(-3)
        let x_neg2 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-2));
        let expected_neg2 = Expression::mul(vec![
            Expression::integer(-2),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(-3)),
        ]);
        assert_eq!(
            x_neg2.derivative(x.clone()).simplify(),
            expected_neg2.simplify()
        );

        // d/dx[x^(-3)] = -3x^(-4)
        let x_neg3 = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-3));
        let expected_neg3 = Expression::mul(vec![
            Expression::integer(-3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(-4)),
        ]);
        assert_eq!(
            x_neg3.derivative(x.clone()).simplify(),
            expected_neg3.simplify()
        );
    }

    #[test]
    fn test_fractional_powers() {
        let x = symbol!(x);

        // d/dx[x^(1/2)] = (1/2)x^(-1/2)
        let sqrt_x = Expression::pow(
            Expression::symbol(x.clone()),
            Expression::mul(vec![
                Expression::integer(1),
                Expression::pow(Expression::integer(2), Expression::integer(-1)),
            ]),
        );
        let result = sqrt_x.derivative(x.clone());
        assert!(!result.is_zero());

        // d/dx[x^(1/3)] = (1/3)x^(-2/3)
        let cbrt_x = Expression::pow(
            Expression::symbol(x.clone()),
            Expression::mul(vec![
                Expression::integer(1),
                Expression::pow(Expression::integer(3), Expression::integer(-1)),
            ]),
        );
        let result_cbrt = cbrt_x.derivative(x.clone());
        assert!(!result_cbrt.is_zero());
    }

    #[test]
    fn test_variable_exponents() {
        let x = symbol!(x);
        let y = symbol!(y);

        // d/dx[x^y] and d/dy[x^y]
        let x_to_y = Expression::pow(Expression::symbol(x.clone()), Expression::symbol(y.clone()));
        let dx_result = x_to_y.derivative(x.clone());
        let dy_result = x_to_y.derivative(y.clone());

        assert!(!dx_result.is_zero());
        assert!(!dy_result.is_zero());

        // d/dx[y^x] and d/dy[y^x]
        let y_to_x = Expression::pow(Expression::symbol(y.clone()), Expression::symbol(x.clone()));
        let dx_y_to_x = y_to_x.derivative(x.clone());
        let dy_y_to_x = y_to_x.derivative(y.clone());

        assert!(!dx_y_to_x.is_zero());
        assert!(!dy_y_to_x.is_zero());
    }

    #[test]
    fn test_special_cases() {
        let x = symbol!(x);

        // d/dx[x^x]
        let x_to_x = Expression::pow(Expression::symbol(x.clone()), Expression::symbol(x.clone()));
        let result = x_to_x.derivative(x.clone());
        assert!(!result.is_zero());

        // d/dx[2^x]
        let two_to_x = Expression::pow(Expression::integer(2), Expression::symbol(x.clone()));
        let result_2x = two_to_x.derivative(x.clone());
        assert!(!result_2x.is_zero());

        // d/dx[x^π]
        let x_to_pi = Expression::pow(
            Expression::symbol(x.clone()),
            Expression::constant(crate::MathConstant::Pi),
        );
        let result_x_pi = x_to_pi.derivative(x.clone());
        assert!(!result_x_pi.is_zero());
    }
}
