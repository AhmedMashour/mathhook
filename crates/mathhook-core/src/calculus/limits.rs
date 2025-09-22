//! Limit computation and analysis
//!
//! Implements symbolic limit computation including one-sided limits,
//! limits at infinity, and indeterminate form resolution.

use crate::algebra::simplify::Simplify;
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};

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
    /// use mathhook_core::{Expression, Symbol};
    /// use mathhook_core::calculus::Limits;
    ///
    /// let x = Symbol::new("x");
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
    /// use mathhook_core::{Expression, Symbol};
    /// use mathhook_core::calculus::{Limits, LimitDirection};
    ///
    /// let x = Symbol::new("x");
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
    /// use mathhook_core::{Expression, Symbol};
    /// use mathhook_core::calculus::Limits;
    ///
    /// let x = Symbol::new("x");
    /// let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));
    /// let result = expr.limit_at_infinity(&x);
    /// ```
    fn limit_at_infinity(&self, variable: &Symbol) -> Expression;

    /// Compute limit at negative infinity
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    /// use mathhook_core::calculus::Limits;
    ///
    /// let x = Symbol::new("x");
    /// let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    /// let result = expr.limit_at_negative_infinity(&x);
    /// ```
    fn limit_at_negative_infinity(&self, variable: &Symbol) -> Expression;
}

/// Limit computation methods and techniques
pub struct LimitMethods;

impl LimitMethods {
    /// Apply L'Hôpital's rule for indeterminate forms
    pub fn lhopital_rule(
        numerator: &Expression,
        denominator: &Expression,
        variable: &Symbol,
        point: &Expression,
    ) -> Expression {
        let num_derivative = numerator.derivative(variable.clone());
        let den_derivative = denominator.derivative(variable.clone());

        Expression::function(
            "limit",
            vec![
                Expression::mul(vec![
                    num_derivative,
                    Expression::pow(den_derivative, Expression::integer(-1)),
                ]),
                Expression::symbol(variable.clone()),
                point.clone(),
            ],
        )
    }

    /// Compute polynomial limit
    pub fn polynomial_limit(
        expr: &Expression,
        variable: &Symbol,
        point: &Expression,
    ) -> Expression {
        // For polynomials, limit is just substitution
        Self::substitute_and_evaluate(expr, variable, point)
    }

    /// Compute rational function limit
    pub fn rational_limit(
        numerator: &Expression,
        denominator: &Expression,
        variable: &Symbol,
        point: &Expression,
    ) -> Expression {
        let num_at_point = Self::substitute_and_evaluate(numerator, variable, point);
        let den_at_point = Self::substitute_and_evaluate(denominator, variable, point);

        match (&num_at_point, &den_at_point) {
            (_, den) if den.is_zero() => {
                // Check if numerator is also zero (0/0 form)
                if num_at_point.is_zero() {
                    // Apply L'Hôpital's rule
                    Self::lhopital_rule(numerator, denominator, variable, point)
                } else {
                    // ±∞ (depending on sign and direction)
                    Expression::infinity()
                }
            }
            (num, den) => {
                // Regular division
                Expression::mul(vec![
                    num.clone(),
                    Expression::pow(den.clone(), Expression::integer(-1)),
                ])
            }
        }
    }

    /// Compute trigonometric limits
    pub fn trigonometric_limit(
        expr: &Expression,
        variable: &Symbol,
        point: &Expression,
    ) -> Expression {
        // Handle special cases like lim(sin(x)/x, x->0) = 1
        if let Expression::Mul(factors) = expr {
            if factors.len() == 2 {
                if let (Expression::Function { name, args }, Expression::Pow(base, exp)) =
                    (&factors[0], &factors[1])
                {
                    if name == "sin"
                        && args.len() == 1
                        && base.as_ref() == &args[0]
                        && **exp == Expression::integer(-1)
                        && point.is_zero()
                    {
                        return Expression::integer(1);
                    }
                }
            }
        }

        Expression::function(
            "limit",
            vec![
                expr.clone(),
                Expression::symbol(variable.clone()),
                point.clone(),
            ],
        )
    }

    /// Substitute variable with point and evaluate
    pub fn substitute_and_evaluate(
        expr: &Expression,
        variable: &Symbol,
        point: &Expression,
    ) -> Expression {
        match expr {
            Expression::Symbol(sym) => {
                if sym == variable {
                    point.clone()
                } else {
                    expr.clone()
                }
            }
            Expression::Add(terms) => {
                let substituted: Vec<Expression> = terms
                    .iter()
                    .map(|term| Self::substitute_and_evaluate(term, variable, point))
                    .collect();
                Expression::add(substituted).simplify()
            }
            Expression::Mul(factors) => {
                let substituted: Vec<Expression> = factors
                    .iter()
                    .map(|factor| Self::substitute_and_evaluate(factor, variable, point))
                    .collect();
                Expression::mul(substituted).simplify()
            }
            Expression::Pow(base, exp) => {
                let sub_base = Self::substitute_and_evaluate(base, variable, point);
                let sub_exp = Self::substitute_and_evaluate(exp, variable, point);
                Expression::pow(sub_base, sub_exp).simplify()
            }
            Expression::Function { name, args } => {
                let substituted_args: Vec<Expression> = args
                    .iter()
                    .map(|arg| Self::substitute_and_evaluate(arg, variable, point))
                    .collect();
                Expression::function(name.clone(), substituted_args)
            }
            _ => expr.clone(),
        }
    }

    /// Check for indeterminate forms
    pub fn is_indeterminate_form(expr: &Expression, variable: &Symbol, point: &Expression) -> bool {
        let substituted = Self::substitute_and_evaluate(expr, variable, point);

        // Check for common indeterminate forms
        match &substituted {
            Expression::Function { name, args } if name == "undefined" => true,
            Expression::Mul(factors) if factors.len() == 2 => {
                // Check for 0 * ∞ form
                (factors[0].is_zero() && Self::is_infinite(&factors[1]))
                    || (factors[1].is_zero() && Self::is_infinite(&factors[0]))
            }
            _ => false,
        }
    }

    /// Check if expression represents infinity
    pub fn is_infinite(expr: &Expression) -> bool {
        matches!(
            expr,
            Expression::Constant(crate::core::MathConstant::Infinity)
        )
    }
}

impl Limits for Expression {
    fn limit(&self, variable: &Symbol, point: &Expression) -> Expression {
        // Try direct substitution first
        let substituted = LimitMethods::substitute_and_evaluate(self, variable, point);

        // Check if result is well-defined
        if !LimitMethods::is_indeterminate_form(&substituted, variable, point) {
            return substituted;
        }

        // Handle special cases based on expression structure
        match self {
            Expression::Mul(factors) if factors.len() == 2 => {
                // Check for rational functions
                if let Expression::Pow(denom, exp) = &factors[1] {
                    if **exp == Expression::integer(-1) {
                        return LimitMethods::rational_limit(&factors[0], denom, variable, point);
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
        Expression::function(
            "limit",
            vec![
                self.clone(),
                Expression::symbol(variable.clone()),
                Expression::infinity(),
            ],
        )
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

    #[test]
    fn test_polynomial_limit() {
        let x = Symbol::new("x");
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let point = Expression::integer(3);
        let result = expr.limit(&x, &point);

        assert_eq!(result, Expression::integer(9));
    }

    #[test]
    fn test_rational_limit_continuous() {
        let x = Symbol::new("x");
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
        let x = Symbol::new("x");
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
    fn test_limit_at_infinity() {
        let x = Symbol::new("x");
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));
        let result = expr.limit_at_infinity(&x);

        // Should be represented as a limit function call
        assert!(matches!(result, Expression::Function { name, .. } if name == "limit"));
    }
}
