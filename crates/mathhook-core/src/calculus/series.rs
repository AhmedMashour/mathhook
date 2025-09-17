//! Series expansions and analysis
//!
//! Implements Taylor series, Laurent series, Maclaurin series,
//! and other infinite series expansions for symbolic computation.
//!
//! For noncommutative expressions (matrices, operators, quaternions):
//! - (A+B)^n expansion preserves order: A^2 + AB + BA + B^2 (NOT A^2 + 2AB + B^2)
//! - Taylor series terms maintain factor order
//! - Power series coefficients respect noncommutativity

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;

/// Types of series expansions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeriesType {
    /// Taylor series expansion
    Taylor,
    /// Laurent series (includes negative powers)
    Laurent,
    /// Maclaurin series (Taylor around 0)
    Maclaurin,
    /// Fourier series
    Fourier,
    /// Power series
    Power,
}

/// Trait for series expansion operations
pub trait SeriesExpansion {
    /// Compute Taylor series expansion
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::SeriesExpansion;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    /// let point = Expression::integer(0);
    /// let result = expr.taylor_series(&x, &point, 5);
    /// ```
    fn taylor_series(&self, variable: &Symbol, point: &Expression, order: u32) -> Expression;

    /// Compute Laurent series expansion
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::SeriesExpansion;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::pow(
    ///     Expression::function("sin", vec![Expression::symbol(x.clone())]),
    ///     Expression::integer(-1)
    /// );
    /// let point = Expression::integer(0);
    /// let result = expr.laurent_series(&x, &point, 5);
    /// ```
    fn laurent_series(&self, variable: &Symbol, point: &Expression, order: u32) -> Expression;

    /// Compute Maclaurin series (Taylor around 0)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::SeriesExpansion;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::function("cos", vec![Expression::symbol(x.clone())]);
    /// let result = expr.maclaurin_series(&x, 6);
    /// ```
    fn maclaurin_series(&self, variable: &Symbol, order: u32) -> Expression;

    /// Compute power series coefficients
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::SeriesExpansion;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
    /// let point = Expression::integer(0);
    /// let result = expr.power_series_coefficients(&x, &point, 5);
    /// ```
    fn power_series_coefficients(
        &self,
        variable: &Symbol,
        point: &Expression,
        order: u32,
    ) -> Vec<Expression>;
}

/// Series expansion methods and utilities
pub struct SeriesMethods;

impl SeriesMethods {
    /// Compute factorial
    pub fn factorial(n: u32) -> Expression {
        if n == 0 || n == 1 {
            Expression::integer(1)
        } else {
            let mut result = 1i64;
            for i in 2..=n {
                result *= i as i64;
            }
            Expression::integer(result)
        }
    }

    /// Compute binomial coefficient
    pub fn binomial_coefficient(n: u32, k: u32) -> Expression {
        if k > n {
            Expression::integer(0)
        } else if k == 0 || k == n {
            Expression::integer(1)
        } else {
            let numerator = Self::factorial(n);
            let denominator = Expression::mul(vec![Self::factorial(k), Self::factorial(n - k)]);
            Expression::mul(vec![
                numerator,
                Expression::pow(denominator, Expression::integer(-1)),
            ])
            .simplify()
        }
    }

    /// Get known series expansion for common functions
    pub fn known_series(
        function_name: &str,
        variable: &Symbol,
        point: &Expression,
        order: u32,
    ) -> Option<Expression> {
        match function_name {
            "exp" if point.is_zero() => {
                // e^x = 1 + x + x²/2! + x³/3! + ...
                let mut terms = vec![Expression::integer(1)];
                for n in 1..=order {
                    let term = Expression::mul(vec![
                        Expression::pow(
                            Expression::symbol(variable.clone()),
                            Expression::integer(n as i64),
                        ),
                        Expression::pow(Self::factorial(n), Expression::integer(-1)),
                    ])
                    .simplify();
                    terms.push(term);
                }
                Some(Expression::add(terms))
            }
            "sin" if point.is_zero() => {
                // sin(x) = x - x³/3! + x⁵/5! - x⁷/7! + ...
                let mut terms = Vec::new();
                for n in 0..=order {
                    let power = 2 * n + 1;
                    if power > order {
                        break;
                    }

                    let sign = if n % 2 == 0 { 1 } else { -1 };
                    let term = Expression::mul(vec![
                        Expression::integer(sign),
                        Expression::pow(
                            Expression::symbol(variable.clone()),
                            Expression::integer(power as i64),
                        ),
                        Expression::pow(Self::factorial(power), Expression::integer(-1)),
                    ])
                    .simplify();
                    terms.push(term);
                }
                Some(Expression::add(terms))
            }
            "cos" if point.is_zero() => {
                // cos(x) = 1 - x²/2! + x⁴/4! - x⁶/6! + ...
                let mut terms = Vec::new();
                for n in 0..=order {
                    let power = 2 * n;
                    if power > order {
                        break;
                    }

                    let sign = if n % 2 == 0 { 1 } else { -1 };
                    let term = if power == 0 {
                        Expression::integer(sign)
                    } else {
                        Expression::mul(vec![
                            Expression::integer(sign),
                            Expression::pow(
                                Expression::symbol(variable.clone()),
                                Expression::integer(power as i64),
                            ),
                            Expression::pow(Self::factorial(power), Expression::integer(-1)),
                        ])
                        .simplify()
                    };
                    terms.push(term);
                }
                Some(Expression::add(terms))
            }
            "ln" if *point == Expression::integer(1) => {
                // ln(1+x) = x - x²/2 + x³/3 - x⁴/4 + ...
                let mut terms = Vec::new();
                for n in 1..=order {
                    let sign = if n % 2 == 1 { 1 } else { -1 };
                    let term = Expression::mul(vec![
                        Expression::integer(sign),
                        Expression::pow(
                            Expression::symbol(variable.clone()),
                            Expression::integer(n as i64),
                        ),
                        Expression::pow(Expression::integer(n as i64), Expression::integer(-1)),
                    ]);
                    terms.push(term);
                }
                Some(Expression::add(terms))
            }
            _ => None,
        }
    }

    /// Compute general Taylor series using derivatives
    ///
    /// Taylor series: f(x) = Σ [f^(n)(a) / n!] * (x-a)^n
    ///
    /// For noncommutative expressions, order is preserved:
    /// - Derivative f^(n)(a) comes first
    /// - Power (x-a)^n comes second
    /// - Division by n! comes last
    pub fn general_taylor_series(
        expr: &Expression,
        variable: &Symbol,
        point: &Expression,
        order: u32,
    ) -> Expression {
        let mut terms = Vec::new();

        for n in 0..=order {
            let nth_derivative = expr.nth_derivative(variable.clone(), n);
            let derivative_at_point = Self::evaluate_at_point(&nth_derivative, variable, point);

            let x_minus_a = if point.is_zero() {
                Expression::symbol(variable.clone())
            } else {
                Expression::add(vec![
                    Expression::symbol(variable.clone()),
                    Expression::mul(vec![Expression::integer(-1), point.clone()]),
                ])
            };

            let term = if n == 0 {
                derivative_at_point
            } else {
                // Order: f^(n)(a) * (x-a)^n * (1/n!)
                // This preserves order for noncommutative derivatives
                Expression::mul(vec![
                    derivative_at_point,
                    Expression::pow(x_minus_a, Expression::integer(n as i64)),
                    Expression::pow(Self::factorial(n), Expression::integer(-1)),
                ])
            };

            terms.push(term);
        }

        Expression::add(terms).simplify()
    }

    /// Evaluate expression at a specific point
    pub fn evaluate_at_point(
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
                let evaluated: Vec<Expression> = terms
                    .iter()
                    .map(|term| Self::evaluate_at_point(term, variable, point))
                    .collect();
                Expression::add(evaluated).simplify()
            }
            Expression::Mul(factors) => {
                let evaluated: Vec<Expression> = factors
                    .iter()
                    .map(|factor| Self::evaluate_at_point(factor, variable, point))
                    .collect();
                Expression::mul(evaluated).simplify()
            }
            Expression::Pow(base, exp) => {
                let eval_base = Self::evaluate_at_point(base, variable, point);
                let eval_exp = Self::evaluate_at_point(exp, variable, point);
                Expression::pow(eval_base, eval_exp).simplify()
            }
            Expression::Function { name, args } => {
                let evaluated_args: Vec<Expression> = args
                    .iter()
                    .map(|arg| Self::evaluate_at_point(arg, variable, point))
                    .collect();
                Expression::function(name.clone(), evaluated_args)
            }
            _ => expr.clone(),
        }
    }
}

impl SeriesExpansion for Expression {
    fn taylor_series(&self, variable: &Symbol, point: &Expression, order: u32) -> Expression {
        // Try known series first
        if let Expression::Function { name, args } = self {
            if args.len() == 1 {
                if let Expression::Symbol(sym) = &args[0] {
                    if sym == variable {
                        if let Some(known) =
                            SeriesMethods::known_series(name, variable, point, order)
                        {
                            return known;
                        }
                    }
                }
            }
        }

        // Fall back to general Taylor series computation
        SeriesMethods::general_taylor_series(self, variable, point, order)
    }

    fn laurent_series(&self, variable: &Symbol, point: &Expression, order: u32) -> Expression {
        // Laurent series includes negative powers for functions with poles
        // For now, delegate to function call
        Expression::function(
            "laurent_series",
            vec![
                self.clone(),
                Expression::symbol(variable.clone()),
                point.clone(),
                Expression::integer(order as i64),
            ],
        )
    }

    fn maclaurin_series(&self, variable: &Symbol, order: u32) -> Expression {
        self.taylor_series(variable, &Expression::integer(0), order)
    }

    fn power_series_coefficients(
        &self,
        variable: &Symbol,
        point: &Expression,
        order: u32,
    ) -> Vec<Expression> {
        let mut coefficients = Vec::new();

        for n in 0..=order {
            let nth_derivative = self.nth_derivative(variable.clone(), n);
            let derivative_at_point =
                SeriesMethods::evaluate_at_point(&nth_derivative, variable, point);
            let coefficient = Expression::mul(vec![
                derivative_at_point,
                Expression::pow(SeriesMethods::factorial(n), Expression::integer(-1)),
            ])
            .simplify();
            coefficients.push(coefficient);
        }

        coefficients
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_exponential_maclaurin() {
        let x = symbol!(x);
        let expr = Expression::function("exp", vec![Expression::symbol(x.clone())]);
        let result = expr.maclaurin_series(&x, 3);

        // e^x ≈ 1 + x + x²/2 + x³/6
        let expected = Expression::add(vec![
            Expression::integer(1),
            Expression::symbol(x.clone()),
            Expression::mul(vec![
                Expression::rational(1, 2),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
            Expression::mul(vec![
                Expression::rational(1, 6),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
            ]),
        ]);

        assert_eq!(result.simplify(), expected.simplify());
    }

    #[test]
    fn test_sine_maclaurin() {
        let x = symbol!(x);
        let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        let result = expr.maclaurin_series(&x, 3);

        // sin(x) ≈ x - x³/6
        let expected = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::mul(vec![
                Expression::rational(-1, 6),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
            ]),
        ]);

        assert_eq!(result.simplify(), expected.simplify());
    }

    #[test]
    fn test_cosine_maclaurin() {
        let x = symbol!(x);
        let expr = Expression::function("cos", vec![Expression::symbol(x.clone())]);
        let result = expr.maclaurin_series(&x, 2);

        // cos(x) ≈ 1 - x²/2
        let expected = Expression::add(vec![
            Expression::integer(1),
            Expression::mul(vec![
                Expression::rational(-1, 2),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
        ]);

        assert_eq!(result.simplify(), expected.simplify());
    }

    #[test]
    fn test_factorial() {
        assert_eq!(SeriesMethods::factorial(0), Expression::integer(1));
        assert_eq!(SeriesMethods::factorial(1), Expression::integer(1));
        assert_eq!(SeriesMethods::factorial(4), Expression::integer(24));
        assert_eq!(SeriesMethods::factorial(5), Expression::integer(120));
    }
}
