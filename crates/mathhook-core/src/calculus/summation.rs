//! Summation and product operations
//!
//! Implements symbolic summation including finite sums, infinite series,
//! products, and convergence analysis.
//!
//! Preserves order for noncommutative expressions (matrices, operators, quaternions).
//! When summing or multiplying noncommutative terms, order is maintained.

use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;

/// Trait for summation and product operations
pub trait Summation {
    /// Compute finite sum
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::Summation;
    ///
    /// let i = symbol!(i);
    /// let expr = Expression::symbol(i.clone());
    /// let start = Expression::integer(1);
    /// let end = Expression::integer(10);
    /// let result = expr.finite_sum(&i, &start, &end);
    /// ```
    fn finite_sum(&self, variable: &Symbol, start: &Expression, end: &Expression) -> Expression;

    /// Compute infinite sum
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::Summation;
    ///
    /// let n = symbol!(n);
    /// let expr = Expression::pow(Expression::symbol(n.clone()), Expression::integer(-2));
    /// let start = Expression::integer(1);
    /// let result = expr.infinite_sum(&n, &start);
    /// ```
    fn infinite_sum(&self, variable: &Symbol, start: &Expression) -> Expression;

    /// Compute finite product
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::Summation;
    ///
    /// let i = symbol!(i);
    /// let expr = Expression::symbol(i.clone());
    /// let start = Expression::integer(1);
    /// let end = Expression::integer(5);
    /// let result = expr.finite_product(&i, &start, &end);
    /// ```
    fn finite_product(&self, variable: &Symbol, start: &Expression, end: &Expression)
        -> Expression;

    /// Compute infinite product
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::Summation;
    ///
    /// let n = symbol!(n);
    /// let expr = Expression::add(vec![
    ///     Expression::integer(1),
    ///     Expression::pow(Expression::symbol(n.clone()), Expression::integer(-2))
    /// ]);
    /// let start = Expression::integer(1);
    /// let result = expr.infinite_product(&n, &start);
    /// ```
    fn infinite_product(&self, variable: &Symbol, start: &Expression) -> Expression;
}

/// Summation methods and utilities
pub struct SummationMethods;

impl SummationMethods {
    /// Compute arithmetic series sum: Σ(a + (i-1)d) from i=1 to n
    pub fn arithmetic_series(
        first_term: &Expression,
        common_difference: &Expression,
        num_terms: &Expression,
    ) -> Expression {
        // Sum = n/2 * (2a + (n-1)d)
        Expression::mul(vec![
            Expression::mul(vec![
                num_terms.clone(),
                Expression::mul(vec![
                    Expression::integer(1),
                    Expression::pow(Expression::integer(2), Expression::integer(-1)),
                ]),
            ]),
            Expression::add(vec![
                Expression::mul(vec![Expression::integer(2), first_term.clone()]),
                Expression::mul(vec![
                    Expression::add(vec![num_terms.clone(), Expression::integer(-1)]),
                    common_difference.clone(),
                ]),
            ]),
        ])
        .simplify()
    }

    /// Compute geometric series sum: Σ(ar^(i-1)) from i=1 to n
    pub fn geometric_series(
        first_term: &Expression,
        common_ratio: &Expression,
        num_terms: &Expression,
    ) -> Expression {
        // Sum = a * (1 - r^n) / (1 - r) for r ≠ 1
        let simplified_ratio = common_ratio.simplify();
        let ratio_power = Expression::pow(simplified_ratio.clone(), num_terms.clone()).simplify();
        let one_minus_ratio_power = Expression::add(vec![
            Expression::integer(1),
            Expression::mul(vec![Expression::integer(-1), ratio_power]),
        ])
        .simplify();

        let numerator = Expression::mul(vec![first_term.clone(), one_minus_ratio_power]).simplify();

        let denominator = Expression::add(vec![
            Expression::integer(1),
            Expression::mul(vec![Expression::integer(-1), simplified_ratio]),
        ])
        .simplify();

        Expression::mul(vec![
            numerator,
            Expression::pow(denominator, Expression::integer(-1)),
        ])
        .simplify()
    }

    /// Compute infinite geometric series sum: Σ(ar^(i-1)) from i=1 to ∞
    pub fn infinite_geometric_series(
        first_term: &Expression,
        common_ratio: &Expression,
    ) -> Expression {
        // Sum = a / (1 - r) for |r| < 1
        let one_minus_r = Expression::add(vec![
            Expression::integer(1),
            Expression::mul(vec![Expression::integer(-1), common_ratio.clone()]).simplify(),
        ])
        .simplify();

        Expression::mul(vec![
            first_term.clone(),
            Expression::pow(one_minus_r, Expression::integer(-1)),
        ])
        .simplify()
    }

    /// Compute power sum: Σ(i^k) from i=1 to n
    pub fn power_sum(power: &Expression, upper_limit: &Expression) -> Expression {
        if let Expression::Number(k) = power {
            if let crate::core::Number::Integer(k_val) = k {
                match k_val {
                    0 => upper_limit.clone(), // Σ1 = n
                    1 => Expression::mul(vec![
                        // Σi = n(n+1)/2
                        upper_limit.clone(),
                        Expression::add(vec![upper_limit.clone(), Expression::integer(1)])
                            .simplify(),
                        Expression::rational(1, 2),
                    ])
                    .simplify(),
                    2 => Expression::mul(vec![
                        // Σi² = n(n+1)(2n+1)/6
                        upper_limit.clone(),
                        Expression::add(vec![upper_limit.clone(), Expression::integer(1)])
                            .simplify(),
                        Expression::add(vec![
                            Expression::mul(vec![Expression::integer(2), upper_limit.clone()])
                                .simplify(),
                            Expression::integer(1),
                        ])
                        .simplify(),
                        Expression::rational(1, 6),
                    ])
                    .simplify(),
                    3 => Expression::pow(
                        // Σi³ = [n(n+1)/2]²
                        Expression::mul(vec![
                            upper_limit.clone(),
                            Expression::add(vec![upper_limit.clone(), Expression::integer(1)])
                                .simplify(),
                            Expression::mul(vec![
                                Expression::integer(1),
                                Expression::pow(Expression::integer(2), Expression::integer(-1)),
                            ])
                            .simplify(),
                        ])
                        .simplify(),
                        Expression::integer(2),
                    )
                    .simplify(),
                    _ => {
                        Expression::function("power_sum", vec![power.clone(), upper_limit.clone()])
                    }
                }
            } else {
                Expression::function("power_sum", vec![power.clone(), upper_limit.clone()])
            }
        } else {
            Expression::function("power_sum", vec![power.clone(), upper_limit.clone()])
        }
    }

    /// Check convergence of infinite series
    pub fn convergence_test(expr: &Expression, variable: &Symbol) -> ConvergenceResult {
        // Simplified convergence testing
        if let Expression::Pow(base, exp) = expr {
            if let (Expression::Symbol(sym), Expression::Number(n)) = (base.as_ref(), exp.as_ref())
            {
                if sym == variable {
                    if let crate::core::Number::Float(exp_val) = n {
                        if *exp_val < -1.0 {
                            return ConvergenceResult::Convergent;
                        } else if *exp_val >= -1.0 {
                            return ConvergenceResult::Divergent;
                        }
                    }
                }
            }
        }
        ConvergenceResult::Unknown
    }
}

/// Result of convergence analysis
#[derive(Debug, Clone, PartialEq)]
pub enum ConvergenceResult {
    Convergent,
    Divergent,
    ConditionallyConvergent,
    Unknown,
}

impl Summation for Expression {
    fn finite_sum(&self, variable: &Symbol, start: &Expression, end: &Expression) -> Expression {
        // Check for special cases
        if let Expression::Symbol(sym) = self {
            if sym == variable {
                // Σi from start to end
                let n = Expression::add(vec![
                    end.clone(),
                    Expression::mul(vec![Expression::integer(-1), start.clone()]),
                    Expression::integer(1),
                ]);
                let first = start.clone();
                let last = end.clone();
                return Expression::mul(vec![
                    n,
                    Expression::add(vec![first, last]),
                    Expression::mul(vec![
                        Expression::integer(1),
                        Expression::pow(Expression::integer(2), Expression::integer(-1)),
                    ]),
                ])
                .simplify();
            }
        }

        if let Expression::Pow(base, exp) = self {
            if let Expression::Symbol(sym) = base.as_ref() {
                if sym == variable {
                    return SummationMethods::power_sum(exp, end);
                }
            }
        }

        // General case
        Expression::function(
            "finite_sum",
            vec![
                self.clone(),
                Expression::symbol(variable.clone()),
                start.clone(),
                end.clone(),
            ],
        )
    }

    fn infinite_sum(&self, variable: &Symbol, start: &Expression) -> Expression {
        // Check for geometric series
        if let Expression::Pow(base, exp) = self {
            if let (Expression::Number(r), Expression::Symbol(sym)) = (base.as_ref(), exp.as_ref())
            {
                if sym == variable {
                    if let crate::core::Number::Float(r_val) = r {
                        if r_val.abs() < 1.0 {
                            // Convergent geometric series
                            return Expression::mul(vec![
                                Expression::pow(*base.clone(), start.clone()),
                                Expression::pow(
                                    Expression::add(vec![
                                        Expression::integer(1),
                                        Expression::mul(vec![
                                            Expression::integer(-1),
                                            *base.clone(),
                                        ]),
                                    ]),
                                    Expression::integer(-1),
                                ),
                            ])
                            .simplify();
                        }
                    }
                }
            }
        }

        Expression::function(
            "infinite_sum",
            vec![
                self.clone(),
                Expression::symbol(variable.clone()),
                start.clone(),
            ],
        )
    }

    fn finite_product(
        &self,
        variable: &Symbol,
        start: &Expression,
        end: &Expression,
    ) -> Expression {
        Expression::function(
            "finite_product",
            vec![
                self.clone(),
                Expression::symbol(variable.clone()),
                start.clone(),
                end.clone(),
            ],
        )
    }

    fn infinite_product(&self, variable: &Symbol, start: &Expression) -> Expression {
        Expression::function(
            "infinite_product",
            vec![
                self.clone(),
                Expression::symbol(variable.clone()),
                start.clone(),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_arithmetic_series() {
        let first = Expression::integer(1);
        let diff = Expression::integer(1);
        let n = Expression::integer(10);

        let result = SummationMethods::arithmetic_series(&first, &diff, &n);
        // 1+2+...+10 = 10*11/2 = 55
        assert_eq!(result, Expression::integer(55));
    }

    #[test]
    fn test_geometric_series() {
        let first = Expression::integer(1);
        let ratio = Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(Expression::integer(2), Expression::integer(-1)),
        ]);
        let n = Expression::integer(3);

        let result = SummationMethods::geometric_series(&first, &ratio, &n);
        // 1 + 1/2 + 1/4 = 7/4
        assert_eq!(result.simplify(), Expression::rational(7, 4));
    }

    #[test]
    fn test_power_sum_linear() {
        let power = Expression::integer(1);
        let n = Expression::integer(5);

        let result = SummationMethods::power_sum(&power, &n);
        // 1+2+3+4+5 = 15
        assert_eq!(result.simplify(), Expression::integer(15));
    }

    #[test]
    fn test_power_sum_quadratic() {
        let power = Expression::integer(2);
        let n = Expression::integer(3);

        let result = SummationMethods::power_sum(&power, &n);
        // 1²+2²+3² = 1+4+9 = 14
        assert_eq!(result.simplify(), Expression::integer(14));
    }

    #[test]
    fn test_finite_sum_linear() {
        let i = symbol!(i);
        let expr = Expression::symbol(i.clone());
        let start = Expression::integer(1);
        let end = Expression::integer(4);

        let result = expr.finite_sum(&i, &start, &end);
        // 1+2+3+4 = 10
        assert_eq!(result.simplify(), Expression::integer(10));
    }

    #[test]
    fn test_infinite_geometric_series() {
        let first = Expression::integer(1);
        let ratio = Expression::rational(1, 3);

        let result = SummationMethods::infinite_geometric_series(&first, &ratio);
        // 1/(1-1/3) = 3/2
        assert_eq!(result, Expression::rational(3, 2));
    }
}
