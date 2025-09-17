//! Limit computation and analysis
//!
//! Implements symbolic limit computation including one-sided limits,
//! limits at infinity, and indeterminate form resolution with complete
//! step-by-step educational explanations.
//!
//! Preserves order for noncommutative expressions (matrices, operators, quaternions).
pub mod educational;
pub mod methods;

use crate::core::polynomial::PolynomialProperties;
use crate::core::{Expression, Number, Symbol};
use methods::LimitMethods;

/// Direction for limit computation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LimitDirection {
    /// Two-sided limit
    Both,
    /// Left-sided limit (approaching from below)
    Left,
    /// Right-sided limit (approaching from above)
    Right,
}

/// Trait for limit operations
pub trait Limits {
    /// Compute two-sided limit
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::Limits;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::mul(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::function("sin", vec![Expression::symbol(x.clone())])
    /// ]);
    /// let point = Expression::integer(0);
    /// let result = expr.limit(&x, &point);
    /// ```
    fn limit(&self, variable: &Symbol, point: &Expression) -> Expression;

    /// Compute one-sided limit
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::{Limits, LimitDirection};
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));
    /// let point = Expression::integer(0);
    /// let result = expr.limit_directed(&x, &point, LimitDirection::Right);
    /// ```
    fn limit_directed(
        &self,
        variable: &Symbol,
        point: &Expression,
        direction: LimitDirection,
    ) -> Expression;

    /// Compute limit at infinity
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::Limits;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));
    /// let result = expr.limit_at_infinity(&x);
    /// ```
    fn limit_at_infinity(&self, variable: &Symbol) -> Expression;

    /// Compute limit at negative infinity
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::Limits;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    /// let result = expr.limit_at_negative_infinity(&x);
    /// ```
    fn limit_at_negative_infinity(&self, variable: &Symbol) -> Expression;
}

impl Limits for Expression {
    fn limit(&self, variable: &Symbol, point: &Expression) -> Expression {
        // Try direct substitution first
        let substituted = LimitMethods::substitute_and_evaluate(self, variable, point);

        // Check if result is well-defined
        if !LimitMethods::is_indeterminate_form(&substituted, variable, point) {
            return substituted;
        }

        match self {
            Expression::Mul(factors) if factors.len() == 2 => {
                if let Expression::Pow(denom, exp) = &factors[1] {
                    if **exp == Expression::integer(-1) {
                        return LimitMethods::rational_limit(&factors[0], denom, variable, point);
                    }
                }
                if let Expression::Pow(denom, exp) = &factors[0] {
                    if **exp == Expression::integer(-1) {
                        return LimitMethods::rational_limit(&factors[1], denom, variable, point);
                    }
                }

                // Check for trigonometric limits
                LimitMethods::trigonometric_limit(self, variable, point)
            }
            Expression::Function { name: _, args: _ } => {
                LimitMethods::trigonometric_limit(self, variable, point)
            }
            _ => Expression::function(
                "limit",
                vec![
                    self.clone(),
                    Expression::symbol(variable.clone()),
                    point.clone(),
                ],
            ),
        }
    }

    fn limit_directed(
        &self,
        variable: &Symbol,
        point: &Expression,
        direction: LimitDirection,
    ) -> Expression {
        let direction_expr = match direction {
            LimitDirection::Both => Expression::symbol("both"),
            LimitDirection::Left => Expression::symbol("left"),
            LimitDirection::Right => Expression::symbol("right"),
        };

        Expression::function(
            "limit_directed",
            vec![
                self.clone(),
                Expression::symbol(variable.clone()),
                point.clone(),
                direction_expr,
            ],
        )
    }

    fn limit_at_infinity(&self, variable: &Symbol) -> Expression {
        match self {
            Expression::Number(_) | Expression::Constant(_) => self.clone(),

            Expression::Symbol(s) if s == variable => Expression::infinity(),

            Expression::Pow(base, exp) => {
                if let Expression::Symbol(s) = base.as_ref() {
                    if s == variable {
                        match exp.as_ref() {
                            Expression::Number(Number::Integer(n)) if *n > 0 => {
                                return Expression::infinity();
                            }
                            Expression::Number(Number::Integer(n)) if *n < 0 => {
                                return Expression::integer(0);
                            }
                            _ => {}
                        }
                    }
                }

                Expression::function(
                    "limit",
                    vec![
                        self.clone(),
                        Expression::symbol(variable.clone()),
                        Expression::infinity(),
                    ],
                )
            }

            Expression::Add(_terms) => {
                if let Some(degree) = self.degree(variable) {
                    let lc = self.leading_coefficient(variable);
                    if degree > 0 {
                        return Expression::infinity();
                    } else if degree == 0 {
                        return lc;
                    }
                }

                Expression::function(
                    "limit",
                    vec![
                        self.clone(),
                        Expression::symbol(variable.clone()),
                        Expression::infinity(),
                    ],
                )
            }

            Expression::Mul(factors) => {
                let mut numer_factors = Vec::new();
                let mut denom_factors = Vec::new();

                for factor in factors.as_ref() {
                    match factor {
                        Expression::Pow(base, exp) if *exp.as_ref() == Expression::integer(-1) => {
                            denom_factors.push(base.as_ref().clone());
                        }
                        _ => {
                            numer_factors.push(factor.clone());
                        }
                    }
                }

                if !denom_factors.is_empty() {
                    let numerator = if numer_factors.is_empty() {
                        Expression::integer(1)
                    } else if numer_factors.len() == 1 {
                        numer_factors[0].clone()
                    } else {
                        Expression::mul(numer_factors)
                    };

                    let denominator = if denom_factors.len() == 1 {
                        denom_factors[0].clone()
                    } else {
                        Expression::mul(denom_factors)
                    };

                    let num_degree = numerator.degree(variable);
                    let den_degree = denominator.degree(variable);

                    match (num_degree, den_degree) {
                        (Some(nd), Some(dd)) if nd == dd => {
                            let num_lc = numerator.leading_coefficient(variable);
                            let den_lc = denominator.leading_coefficient(variable);
                            return Expression::mul(vec![
                                num_lc,
                                Expression::pow(den_lc, Expression::integer(-1)),
                            ]);
                        }
                        (Some(nd), Some(dd)) if nd > dd => {
                            return Expression::infinity();
                        }
                        (Some(nd), Some(dd)) if nd < dd => {
                            return Expression::integer(0);
                        }
                        _ => {}
                    }
                }

                if let Some(degree) = self.degree(variable) {
                    if degree > 0 {
                        return Expression::infinity();
                    } else if degree == 0 {
                        return self.clone();
                    }
                }

                Expression::function(
                    "limit",
                    vec![
                        self.clone(),
                        Expression::symbol(variable.clone()),
                        Expression::infinity(),
                    ],
                )
            }

            _ => Expression::function(
                "limit",
                vec![
                    self.clone(),
                    Expression::symbol(variable.clone()),
                    Expression::infinity(),
                ],
            ),
        }
    }

    fn limit_at_negative_infinity(&self, variable: &Symbol) -> Expression {
        Expression::function(
            "limit",
            vec![
                self.clone(),
                Expression::symbol(variable.clone()),
                Expression::mul(vec![Expression::integer(-1), Expression::infinity()]),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_polynomial_limit() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let point = Expression::integer(3);
        let result = expr.limit(&x, &point);

        assert_eq!(result, Expression::integer(9));
    }

    #[test]
    fn test_rational_limit_continuous() {
        let x = symbol!(x);
        let numerator =
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
        let denominator =
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(2)]);
        let expr = Expression::mul(vec![
            numerator,
            Expression::pow(denominator, Expression::integer(-1)),
        ]);
        let point = Expression::integer(1);
        let result = expr.limit(&x, &point);

        assert_eq!(result, Expression::rational(2, 3));
    }

    #[test]
    fn test_trigonometric_limit() {
        let x = symbol!(x);
        let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        let expr = Expression::mul(vec![
            sin_x,
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)),
        ]);
        let point = Expression::integer(0);
        let result = expr.limit(&x, &point);

        assert_eq!(result, Expression::integer(1));
    }

    #[test]
    fn test_limit_at_infinity_constant() {
        let x = symbol!(x);
        let expr = Expression::integer(5);
        let result = expr.limit_at_infinity(&x);

        assert_eq!(result, Expression::integer(5));
    }

    #[test]
    fn test_limit_at_infinity_variable() {
        let x = symbol!(x);
        let expr = Expression::symbol(x.clone());
        let result = expr.limit_at_infinity(&x);

        assert_eq!(result, Expression::infinity());
    }

    #[test]
    fn test_limit_at_infinity_inverse() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));
        let result = expr.limit_at_infinity(&x);

        assert_eq!(result, Expression::integer(0));
    }

    #[test]
    fn test_limit_at_infinity_polynomial() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
        let result = expr.limit_at_infinity(&x);

        assert_eq!(result, Expression::infinity());
    }

    #[test]
    fn test_limit_at_infinity_rational_same_degree() {
        let x = symbol!(x);
        let numerator = Expression::mul(vec![
            Expression::integer(3),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]);
        let denominator = Expression::mul(vec![
            Expression::integer(2),
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        ]);
        let expr = Expression::mul(vec![
            numerator,
            Expression::pow(denominator, Expression::integer(-1)),
        ]);
        let result = expr.limit_at_infinity(&x);

        assert_eq!(result, Expression::rational(3, 2));
    }

    #[test]
    fn test_limit_at_infinity_rational_num_greater() {
        let x = symbol!(x);
        let numerator = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
        let denominator = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let expr = Expression::mul(vec![
            numerator,
            Expression::pow(denominator, Expression::integer(-1)),
        ]);
        let result = expr.limit_at_infinity(&x);

        assert_eq!(result, Expression::infinity());
    }

    #[test]
    fn test_limit_at_infinity_rational_den_greater() {
        let x = symbol!(x);
        let numerator = Expression::pow(Expression::symbol(x.clone()), Expression::integer(1));
        let denominator = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
        let expr = Expression::mul(vec![
            numerator,
            Expression::pow(denominator, Expression::integer(-1)),
        ]);
        let result = expr.limit_at_infinity(&x);

        assert_eq!(result, Expression::integer(0));
    }
}
