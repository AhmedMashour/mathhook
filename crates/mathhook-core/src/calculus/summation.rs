//! Summation and product operations
//!
//! Implements symbolic summation including finite sums, infinite series,
//! products, and convergence analysis.
//!
//! Preserves order for noncommutative expressions (matrices, operators, quaternions).
//! When summing or multiplying noncommutative terms, order is maintained.
//!
//! # Mathematical Background
//!
//! **SymPy Validated: 2025-11-16** (validation script: `scripts/validate_summation.py`)
//!
//! All formulas cross-validated against SymPy for mathematical correctness.
//!
//! # Educational Features
//!
//! The summation module provides step-by-step explanations for series computation:
//!
//! ```rust,ignore
//! use mathhook_core::{expr, symbol};
//! use mathhook_core::calculus::summation::educational::SummationEducational;
//!
//! let i = symbol!(i);
//! let sum_expr = expr!(i);
//!
//! let explanation = sum_expr.explain_finite_sum(&i, &expr!(1), &expr!(10));
//!
//! for step in &explanation.steps {
//!     println!("{}: {}", step.title, step.description);
//! }
//! ```

use crate::core::{Expression, Number, Symbol};
use crate::expr;
use crate::simplify::Simplify;

pub mod educational;

/// Trait for summation and product operations
pub trait Summation {
    /// Compute finite sum
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::Summation;
    ///
    /// let i = symbol!(i);
    /// let start = expr!(1);
    /// let end = expr!(10);
    /// let result = i.clone().into().finite_sum(&i, &start, &end);
    /// ```
    fn finite_sum(&self, variable: &Symbol, start: &Expression, end: &Expression) -> Expression;

    /// Compute infinite sum
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::Summation;
    ///
    /// let n = symbol!(n);
    /// let expr = expr!(n ^ (-2));
    /// let start = expr!(1);
    /// let result = expr.infinite_sum(&n, &start);
    /// ```
    fn infinite_sum(&self, variable: &Symbol, start: &Expression) -> Expression;

    /// Compute finite product
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::Summation;
    ///
    /// let i = symbol!(i);
    /// let start = expr!(1);
    /// let end = expr!(5);
    /// let result = i.clone().into().finite_product(&i, &start, &end);
    /// ```
    fn finite_product(&self, variable: &Symbol, start: &Expression, end: &Expression)
        -> Expression;

    /// Compute infinite product
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::Summation;
    ///
    /// let n = symbol!(n);
    /// let expr = expr!(1 + (n ^ (-2)));
    /// let start = expr!(1);
    /// let result = expr.infinite_product(&n, &start);
    /// ```
    fn infinite_product(&self, variable: &Symbol, start: &Expression) -> Expression;
}

/// Summation methods and utilities
///
/// # Performance
///
/// All methods use closed-form formulas when applicable, providing O(1) time complexity
/// for arithmetic/geometric series and power sums (k ≤ 3). General sums remain symbolic.
pub struct SummationMethods;

/// Power sum formula registry (Faulhaber's formulas)
///
/// This registry-based approach avoids hardcoded matching on integer values.
/// Each formula is validated against SymPy for correctness.
struct PowerSumFormula {
    power: i64,
    compute: fn(&Expression) -> Expression,
}

impl PowerSumFormula {
    const FORMULAS: &'static [PowerSumFormula] = &[
        PowerSumFormula {
            power: 0,
            compute: |n| n.clone(),
        },
        PowerSumFormula {
            power: 1,
            compute: |n| {
                let n_plus_1 = Expression::add(vec![n.clone(), expr!(1)]).simplify();
                Expression::mul(vec![n.clone(), n_plus_1, expr!(1 / 2)]).simplify()
            },
        },
        PowerSumFormula {
            power: 2,
            compute: |n| {
                let n_plus_1 = Expression::add(vec![n.clone(), expr!(1)]).simplify();
                let two_n = Expression::mul(vec![expr!(2), n.clone()]).simplify();
                let two_n_plus_1 = Expression::add(vec![two_n, expr!(1)]).simplify();
                Expression::mul(vec![n.clone(), n_plus_1, two_n_plus_1, expr!(1 / 6)]).simplify()
            },
        },
        PowerSumFormula {
            power: 3,
            compute: |n| {
                let n_plus_1 = Expression::add(vec![n.clone(), expr!(1)]).simplify();
                let base = Expression::mul(vec![n.clone(), n_plus_1, expr!(1 / 2)]).simplify();
                Expression::pow(base, expr!(2)).simplify()
            },
        },
    ];

    fn lookup(power: i64) -> Option<fn(&Expression) -> Expression> {
        Self::FORMULAS
            .iter()
            .find(|formula| formula.power == power)
            .map(|formula| formula.compute)
    }
}

impl SummationMethods {
    /// Compute arithmetic series sum: Σ(a + (i-1)d) from i=1 to n
    ///
    /// # Formula
    /// Sum = n/2 * (2a + (n-1)d)
    /// # Performance
    /// - **Time Complexity:** O(1) - uses closed-form formula
    /// - **Space Complexity:** O(1) - constant expression construction
    pub fn arithmetic_series(
        first_term: &Expression,
        common_difference: &Expression,
        num_terms: &Expression,
    ) -> Expression {
        let n_over_2 = Expression::mul(vec![num_terms.clone(), expr!(1 / 2)]);
        let two_a = Expression::mul(vec![expr!(2), first_term.clone()]);
        let n_minus_1 = Expression::add(vec![num_terms.clone(), expr!(-1)]);
        let n_minus_1_times_d = Expression::mul(vec![n_minus_1, common_difference.clone()]);
        let inner_sum = Expression::add(vec![two_a, n_minus_1_times_d]);

        Expression::mul(vec![n_over_2, inner_sum]).simplify()
    }

    /// Compute geometric series sum: Σ(ar^(i-1)) from i=1 to n
    ///
    /// # Formula
    /// Sum = a * (1 - r^n) / (1 - r) for r ≠ 1
    ///
    /// # Performance
    /// - **Time Complexity:** O(1) - uses closed-form formula
    /// - **Space Complexity:** O(1) - constant expression construction
    pub fn geometric_series(
        first_term: &Expression,
        common_ratio: &Expression,
        num_terms: &Expression,
    ) -> Expression {
        let simplified_ratio = common_ratio.simplify();
        let ratio_power = Expression::pow(simplified_ratio.clone(), num_terms.clone()).simplify();
        let one_minus_ratio_power = Expression::add(vec![
            expr!(1),
            Expression::mul(vec![expr!(-1), ratio_power]),
        ])
        .simplify();

        let numerator = Expression::mul(vec![first_term.clone(), one_minus_ratio_power]).simplify();
        let denominator = Expression::add(vec![
            expr!(1),
            Expression::mul(vec![expr!(-1), simplified_ratio]),
        ])
        .simplify();

        Expression::mul(vec![numerator, Expression::pow(denominator, expr!(-1))]).simplify()
    }

    /// Compute infinite geometric series sum: Σ(ar^(i-1)) from i=1 to ∞
    ///
    /// # Formula
    /// Sum = a / (1 - r) for |r| < 1
    ///
    /// # Domain Restriction
    /// Convergence requires |r| < 1. For |r| ≥ 1, series diverges.
    ///
    /// # Performance
    /// - **Time Complexity:** O(1) - uses closed-form formula
    /// - **Space Complexity:** O(1) - constant expression construction
    pub fn infinite_geometric_series(
        first_term: &Expression,
        common_ratio: &Expression,
    ) -> Expression {
        let one_minus_r = Expression::add(vec![
            expr!(1),
            Expression::mul(vec![expr!(-1), common_ratio.clone()]),
        ])
        .simplify();

        Expression::mul(vec![
            first_term.clone(),
            Expression::pow(one_minus_r, expr!(-1)),
        ])
        .simplify()
    }

    /// Compute power sum: Σ(i^k) from i=1 to n using Faulhaber's formulas
    ///
    /// # Formulas (SymPy Validated)
    /// - k=0: Σ1 = n
    /// - k=1: Σi = n(n+1)/2
    /// - k=2: Σi² = n(n+1)(2n+1)/6
    /// - k=3: Σi³ = [n(n+1)/2]²
    ///
    /// # Performance
    /// - **Time Complexity:** O(1) for k ∈ {0,1,2,3}, symbolic for k > 3
    /// - **Space Complexity:** O(1) - constant expression construction
    pub fn power_sum(power: &Expression, upper_limit: &Expression) -> Expression {
        if let Expression::Number(Number::Integer(k_val)) = power {
            if let Some(compute_fn) = PowerSumFormula::lookup(*k_val) {
                return compute_fn(upper_limit);
            }
        }

        Expression::function("power_sum", vec![power.clone(), upper_limit.clone()])
    }

    /// Check convergence of infinite series
    ///
    /// Simplified p-series test: Σ(1/n^p) converges iff p > 1
    ///
    /// # Performance
    /// - **Time Complexity:** O(1) - pattern matching only
    /// - **Space Complexity:** O(1) - returns enum variant
    pub fn convergence_test(expr: &Expression, variable: &Symbol) -> ConvergenceResult {
        if let Expression::Pow(base, exp) = expr {
            if matches!(
                (base.as_ref(), exp.as_ref()),
                (Expression::Symbol(sym), Expression::Number(Number::Float(exp_val)))
                if sym == variable && *exp_val < -1.0
            ) {
                return ConvergenceResult::Convergent;
            }

            if matches!(
                (base.as_ref(), exp.as_ref()),
                (Expression::Symbol(sym), Expression::Number(Number::Float(exp_val)))
                if sym == variable && *exp_val >= -1.0
            ) {
                return ConvergenceResult::Divergent;
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
        if let Expression::Symbol(sym) = self {
            if sym == variable {
                let n = Expression::add(vec![
                    end.clone(),
                    Expression::mul(vec![expr!(-1), start.clone()]),
                    expr!(1),
                ]);

                let first = start.clone();
                let last = end.clone();

                return Expression::mul(vec![n, Expression::add(vec![first, last]), expr!(1 / 2)])
                    .simplify();
            }
        }

        if matches!(
            self,
            Expression::Pow(base, _) if matches!(base.as_ref(), Expression::Symbol(sym) if sym == variable)
        ) {
            if let Expression::Pow(_, exp) = self {
                return SummationMethods::power_sum(exp, end);
            }
        }

        Expression::function(
            "finite_sum",
            vec![
                self.clone(),
                variable.clone().into(),
                start.clone(),
                end.clone(),
            ],
        )
    }

    fn infinite_sum(&self, variable: &Symbol, start: &Expression) -> Expression {
        if let Expression::Pow(base, exp) = self {
            if matches!(
                (base.as_ref(), exp.as_ref()),
                (Expression::Number(Number::Float(r_val)), Expression::Symbol(sym))
                if sym == variable && r_val.abs() < 1.0
            ) {
                let one_minus_r = Expression::add(vec![
                    expr!(1),
                    Expression::mul(vec![expr!(-1), base.as_ref().clone()]),
                ]);

                return Expression::mul(vec![
                    Expression::pow(base.as_ref().clone(), start.clone()),
                    Expression::pow(one_minus_r, expr!(-1)),
                ])
                .simplify();
            }
        }

        Expression::function(
            "infinite_sum",
            vec![self.clone(), variable.clone().into(), start.clone()],
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
                variable.clone().into(),
                start.clone(),
                end.clone(),
            ],
        )
    }

    fn infinite_product(&self, variable: &Symbol, start: &Expression) -> Expression {
        Expression::function(
            "infinite_product",
            vec![self.clone(), variable.clone().into(), start.clone()],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_arithmetic_series() {
        let first = expr!(1);
        let diff = expr!(1);
        let n = expr!(10);

        let result = SummationMethods::arithmetic_series(&first, &diff, &n);
        assert_eq!(result, expr!(55));
    }

    #[test]
    fn test_geometric_series() {
        let first = expr!(1);
        let ratio = expr!(1 / 2);
        let n = expr!(3);

        let result = SummationMethods::geometric_series(&first, &ratio, &n);
        assert_eq!(result.simplify(), Expression::rational(7, 4));
    }

    #[test]
    fn test_power_sum_linear() {
        let power = expr!(1);
        let n = expr!(5);

        let result = SummationMethods::power_sum(&power, &n);
        assert_eq!(result.simplify(), expr!(15));
    }

    #[test]
    fn test_power_sum_quadratic() {
        let power = expr!(2);
        let n = expr!(3);

        let result = SummationMethods::power_sum(&power, &n);
        assert_eq!(result.simplify(), expr!(14));
    }

    #[test]
    fn test_finite_sum_linear() {
        let i = symbol!(i);
        let start = expr!(1);
        let end = expr!(4);

        let expr_i: Expression = i.clone().into();
        let result = expr_i.finite_sum(&i, &start, &end);
        assert_eq!(result.simplify(), expr!(10));
    }

    #[test]
    fn test_infinite_geometric_series() {
        let first = expr!(1);
        let ratio = Expression::rational(1, 3);

        let result = SummationMethods::infinite_geometric_series(&first, &ratio);
        assert_eq!(result, Expression::rational(3, 2));
    }
}
