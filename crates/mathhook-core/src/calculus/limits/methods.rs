use super::*;
use crate::calculus::derivatives::Derivative;
use crate::simplify::Simplify;

use crate::{Expression, Symbol};

/// Limit computation methods and techniques
pub struct LimitMethods;

impl LimitMethods {
    /// Apply L'Hôpital's rule for indeterminate forms
    ///
    /// For indeterminate forms 0/0 or ∞/∞:
    /// lim[x→a] f(x)/g(x) = lim[x→a] f'(x)/g'(x)
    ///
    /// Order is preserved for noncommutative expressions.
    pub fn lhopital_rule(
        numerator: &Expression,
        denominator: &Expression,
        variable: &Symbol,
        point: &Expression,
    ) -> Expression {
        let num_derivative = numerator.derivative(variable.clone());
        let den_derivative = denominator.derivative(variable.clone());

        // Create the derivative ratio: f'(x) * (g'(x))^(-1)
        // Order preserved: f'(x) comes before 1/g'(x)
        let derivative_ratio = Expression::mul(vec![
            num_derivative,
            Expression::pow(den_derivative, Expression::integer(-1)),
        ]);

        // Recursively call limit to evaluate the derivative ratio
        derivative_ratio.limit(variable, point)
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
        if let Expression::Mul(factors) = expr {
            if factors.len() == 2 {
                let check_sin_over_x = |(func_expr, pow_expr): (&Expression, &Expression)| -> bool {
                    if let (Expression::Function { name, args }, Expression::Pow(base, exp)) =
                        (func_expr, pow_expr)
                    {
                        name == "sin"
                            && args.len() == 1
                            && base.as_ref() == &args[0]
                            && **exp == Expression::integer(-1)
                            && point.is_zero()
                    } else {
                        false
                    }
                };

                if check_sin_over_x((&factors[0], &factors[1]))
                    || check_sin_over_x((&factors[1], &factors[0]))
                {
                    return Expression::integer(1);
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

                // Check if we have a potential indeterminate form (0 with undefined)
                let has_zero = substituted.iter().any(|f| f.is_zero());
                let has_undefined = substituted
                    .iter()
                    .any(|f| matches!(f, Expression::Function { name, .. } if name == "undefined"));

                if has_zero && has_undefined {
                    // Don't simplify to preserve indeterminate form detection
                    Expression::mul(substituted)
                } else {
                    // Safe to simplify normal cases
                    Expression::mul(substituted).simplify()
                }
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
                Expression::function(name.clone(), substituted_args).simplify()
            }
            _ => expr.clone(),
        }
    }

    /// Check for indeterminate forms
    pub fn is_indeterminate_form(expr: &Expression, variable: &Symbol, point: &Expression) -> bool {
        let substituted = Self::substitute_and_evaluate(expr, variable, point);

        // Check for common indeterminate forms
        match &substituted {
            Expression::Function { name, args: _ } if name == "undefined" => true,
            Expression::Mul(factors) if factors.len() == 2 => {
                // Check for 0 * ∞ form
                (factors[0].is_zero() && Self::is_infinite(&factors[1]))
                    || (factors[1].is_zero() && Self::is_infinite(&factors[0]))
                    // Check for 0 * 0^(-1) form (which is 0/0)
                    || (factors[0].is_zero() && matches!(&factors[1], Expression::Pow(base, exp) if base.is_zero() && **exp == Expression::integer(-1)))
                    // Check for 0 * undefined form (which is 0/0)
                    || (factors[0].is_zero() && matches!(&factors[1], Expression::Function { name, .. } if name == "undefined"))
                    || (factors[1].is_zero() && matches!(&factors[0], Expression::Function { name, .. } if name == "undefined"))
            }
            // Check for 0^(-1) form directly
            Expression::Pow(base, exp) if base.is_zero() && **exp == Expression::integer(-1) => {
                true
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
