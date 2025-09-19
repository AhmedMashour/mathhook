//! Benchmark utilities and shared test data for MathHook performance testing
//!
//! This crate provides common utilities, test data, and helper functions
//! for benchmarking the MathHook ecosystem across all crates.

use mathhook_core::{Expression, Symbol};

/// Common test expressions for benchmarking
pub struct BenchmarkData;

impl BenchmarkData {
    /// Create a simple linear expression for benchmarking
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_benchmarks::BenchmarkData;
    ///
    /// let expr = BenchmarkData::linear_expression("x", 2, 3);
    /// ```
    pub fn linear_expression(var_name: &str, coeff: i64, constant: i64) -> Expression {
        let x = Symbol::new(var_name);
        Expression::add(vec![
            Expression::multiply(Expression::integer(coeff), Expression::symbol(x)),
            Expression::integer(constant),
        ])
    }

    /// Create a quadratic expression for benchmarking
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_benchmarks::BenchmarkData;
    ///
    /// let expr = BenchmarkData::quadratic_expression("x", 1, -5, 6);
    /// ```
    pub fn quadratic_expression(var_name: &str, a: i64, b: i64, c: i64) -> Expression {
        let x = Symbol::new(var_name);
        Expression::add(vec![
            Expression::multiply(
                Expression::integer(a),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ),
            Expression::multiply(Expression::integer(b), Expression::symbol(x)),
            Expression::integer(c),
        ])
    }

    /// Create a complex polynomial for stress testing
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_benchmarks::BenchmarkData;
    ///
    /// let expr = BenchmarkData::complex_polynomial("x", 5);
    /// ```
    pub fn complex_polynomial(var_name: &str, degree: u32) -> Expression {
        let x = Symbol::new(var_name);
        let mut terms = Vec::new();

        for i in 0..=degree {
            let coeff = Expression::integer((degree - i + 1) as i64);
            let term = if i == 0 {
                coeff
            } else {
                Expression::multiply(
                    coeff,
                    Expression::pow(Expression::symbol(x.clone()), Expression::integer(i as i64)),
                )
            };
            terms.push(term);
        }

        Expression::add(terms)
    }
}
