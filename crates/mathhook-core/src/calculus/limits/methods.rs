use super::*;
use crate::calculus::derivatives::Derivative;
use crate::simplify::Simplify;

use crate::{Expression, Number, Symbol};

/// Limit computation methods and techniques
pub struct LimitMethods;

impl LimitMethods {
    /// Apply L'Hôpital's rule for indeterminate forms
    ///
    /// For indeterminate forms 0/0 or ∞/∞:
    /// lim[x→a] f(x)/g(x) = lim[x→a] f'(x)/g'(x)
    ///
    /// Order is preserved for noncommutative expressions.
    /// Applies L'Hôpital's rule recursively up to 5 times if needed.
    pub fn lhopital_rule(
        numerator: &Expression,
        denominator: &Expression,
        variable: &Symbol,
        point: &Expression,
    ) -> Expression {
        Self::lhopital_rule_recursive(numerator, denominator, variable, point, 0)
    }

    fn lhopital_rule_recursive(
        numerator: &Expression,
        denominator: &Expression,
        variable: &Symbol,
        point: &Expression,
        depth: usize,
    ) -> Expression {
        const MAX_DEPTH: usize = 5;

        if depth >= MAX_DEPTH {
            return Expression::function(
                "limit",
                vec![
                    Expression::mul(vec![
                        numerator.clone(),
                        Expression::pow(denominator.clone(), Expression::integer(-1)),
                    ]),
                    Expression::symbol(variable.clone()),
                    point.clone(),
                ],
            );
        }

        let num_derivative = numerator.derivative(variable.clone());
        let den_derivative = denominator.derivative(variable.clone());

        let num_at_point = Self::substitute_and_evaluate(&num_derivative, variable, point);
        let den_at_point = Self::substitute_and_evaluate(&den_derivative, variable, point);

        if num_at_point.is_zero() && den_at_point.is_zero() {
            return Self::lhopital_rule_recursive(
                &num_derivative,
                &den_derivative,
                variable,
                point,
                depth + 1,
            );
        }

        if den_at_point.is_zero() {
            return Expression::infinity();
        }

        Expression::mul(vec![
            num_at_point,
            Expression::pow(den_at_point, Expression::integer(-1)),
        ])
        .simplify()
    }

    /// Apply L'Hôpital's rule for limits at infinity
    ///
    /// For indeterminate forms ∞/∞ as x→∞:
    /// lim[x→∞] f(x)/g(x) = lim[x→∞] f'(x)/g'(x)
    pub fn lhopital_rule_at_infinity(
        numerator: &Expression,
        denominator: &Expression,
        variable: &Symbol,
    ) -> Expression {
        let num_derivative = numerator.derivative(variable.clone());
        let den_derivative = denominator.derivative(variable.clone());

        let derivative_ratio = Expression::mul(vec![
            num_derivative,
            Expression::pow(den_derivative, Expression::integer(-1)),
        ]);

        derivative_ratio.limit_at_infinity(variable)
    }

    /// Compute polynomial limit
    pub fn polynomial_limit(
        expr: &Expression,
        variable: &Symbol,
        point: &Expression,
    ) -> Expression {
        Self::substitute_and_evaluate(expr, variable, point)
    }

    /// Compute rational function limit at infinity
    ///
    /// Handles cases like ln(x)/x where degree-based analysis doesn't work
    pub fn rational_limit_at_infinity(
        numerator: &Expression,
        denominator: &Expression,
        variable: &Symbol,
    ) -> Expression {
        let num_limit = numerator.limit_at_infinity(variable);
        let den_limit = denominator.limit_at_infinity(variable);

        if Self::is_infinite(&num_limit) && Self::is_infinite(&den_limit) {
            return Self::lhopital_rule_at_infinity(numerator, denominator, variable);
        }

        if den_limit.is_zero() {
            return Expression::infinity();
        }

        Expression::mul(vec![
            num_limit,
            Expression::pow(den_limit, Expression::integer(-1)),
        ])
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
                if num_at_point.is_zero() {
                    Self::lhopital_rule(numerator, denominator, variable, point)
                } else {
                    Expression::infinity()
                }
            }
            (num, den) => Expression::mul(vec![
                num.clone(),
                Expression::pow(den.clone(), Expression::integer(-1)),
            ])
            .simplify(),
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

                let has_zero = substituted.iter().any(|f| f.is_zero());
                let has_division_by_zero = substituted.iter().any(|f| {
                    matches!(f, Expression::Pow(base, exp)
                        if base.as_ref().is_zero() && matches!(exp.as_ref(), Expression::Number(Number::Integer(n)) if *n < 0))
                });
                let has_undefined = substituted
                    .iter()
                    .any(|f| matches!(f, Expression::Function { name, .. } if name == "undefined"));

                if has_zero && (has_undefined || has_division_by_zero) {
                    Expression::mul(substituted)
                } else {
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

        match &substituted {
            Expression::Function { name, args: _ } if name == "undefined" => true,
            Expression::Mul(factors) if factors.len() == 2 => {
                (factors[0].is_zero() && Self::is_infinite(&factors[1]))
                    || (factors[1].is_zero() && Self::is_infinite(&factors[0]))
                    || (factors[0].is_zero()
                        && matches!(&factors[1], Expression::Pow(base, exp)
                            if base.as_ref().is_zero() && matches!(exp.as_ref(), Expression::Number(Number::Integer(n)) if *n < 0)))
                    || (factors[1].is_zero()
                        && matches!(&factors[0], Expression::Pow(base, exp)
                            if base.as_ref().is_zero() && matches!(exp.as_ref(), Expression::Number(Number::Integer(n)) if *n < 0)))
                    || (factors[0].is_zero()
                        && matches!(&factors[1], Expression::Function { name, .. } if name == "undefined"))
                    || (factors[1].is_zero()
                        && matches!(&factors[0], Expression::Function { name, .. } if name == "undefined"))
            }
            Expression::Pow(base, exp)
                if base.as_ref().is_zero()
                    && matches!(exp.as_ref(), Expression::Number(Number::Integer(n)) if *n < 0) =>
            {
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
