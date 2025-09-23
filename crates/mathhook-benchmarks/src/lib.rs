//! Benchmark utilities and shared test data for MathHook performance testing
//!
//! This crate provides common utilities, test data, and helper functions
//! for benchmarking the MathHook ecosystem across all crates.
//!
//! Focus areas:
//! - Bulk numeric operations (SIMD candidates)
//! - Matrix operations (perfect for SIMD)
//! - Polynomial evaluation (Horner's method)
//! - Mixed symbolic-numeric workloads (realistic CAS usage)

use mathhook_core::{Expression, Number, Symbol};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::cast::ToPrimitive;

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
            Expression::mul(vec![Expression::integer(coeff), Expression::symbol(x)]),
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
            Expression::mul(vec![
                Expression::integer(a),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
            Expression::mul(vec![Expression::integer(b), Expression::symbol(x)]),
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
                Expression::mul(vec![
                    coeff,
                    Expression::pow(Expression::symbol(x.clone()), Expression::integer(i as i64)),
                ])
            };
            terms.push(term);
        }

        Expression::add(terms)
    }

    /// Create bulk numeric data for SIMD benchmarking
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_benchmarks::BenchmarkData;
    ///
    /// let integers = BenchmarkData::bulk_integers(100);
    /// let rationals = BenchmarkData::bulk_rationals(50);
    /// ```
    pub fn bulk_integers(count: usize) -> Vec<Expression> {
        (1..=count).map(|i| Expression::integer(i as i64)).collect()
    }

    /// Create bulk rational numbers for SIMD testing
    pub fn bulk_rationals(count: usize) -> Vec<Expression> {
        (1..=count)
            .map(|i| {
                Expression::Number(Number::rational(BigRational::new(
                    BigInt::from(i),
                    BigInt::from(i + 1),
                )))
            })
            .collect()
    }

    /// Create bulk float numbers for SIMD testing
    pub fn bulk_floats(count: usize) -> Vec<Expression> {
        (1..=count)
            .map(|i| Expression::Number(Number::Float(i as f64 * 0.1)))
            .collect()
    }

    /// Create a test matrix for SIMD matrix operations
    pub fn test_matrix(rows: usize, cols: usize) -> Expression {
        let matrix_rows: Vec<Vec<Expression>> = (0..rows)
            .map(|i| {
                (0..cols)
                    .map(|j| Expression::integer((i * cols + j + 1) as i64))
                    .collect()
            })
            .collect();
        Expression::matrix(matrix_rows)
    }

    /// Create a dense polynomial for Horner's method benchmarking
    pub fn dense_polynomial(var_name: &str, degree: usize) -> Expression {
        let x = Symbol::new(var_name);
        let terms: Vec<Expression> = (0..=degree)
            .map(|i| {
                let coeff = Expression::integer((i + 1) as i64);
                if i == 0 {
                    coeff
                } else {
                    Expression::mul(vec![
                        coeff,
                        Expression::pow(
                            Expression::symbol(x.clone()),
                            Expression::integer(i as i64),
                        ),
                    ])
                }
            })
            .collect();
        Expression::add(terms)
    }

    /// Create mixed symbolic-numeric expressions for realistic benchmarking
    pub fn mixed_expression(var_names: &[&str], num_terms: usize) -> Expression {
        let symbols: Vec<Symbol> = var_names.iter().map(|name| Symbol::new(name)).collect();
        let terms: Vec<Expression> = (1..=num_terms)
            .map(|i| match i % (symbols.len() + 1) {
                0 => Expression::integer(i as i64),
                j => Expression::mul(vec![
                    Expression::integer(i as i64),
                    Expression::symbol(symbols[j - 1].clone()),
                ]),
            })
            .collect();
        Expression::add(terms)
    }
}

/// SIMD benchmark utilities
pub struct SimdBenchmarkUtils;

impl SimdBenchmarkUtils {
    /// Extract numeric values from expressions for SIMD processing
    pub fn extract_numeric_values(expressions: &[Expression]) -> Vec<f64> {
        expressions
            .iter()
            .filter_map(|expr| match expr {
                Expression::Number(Number::Integer(i)) => Some(*i as f64),
                Expression::Number(Number::Float(f)) => Some(*f),
                Expression::Number(Number::Rational(r)) => r.to_f64(),
                _ => None,
            })
            .collect()
    }

    /// Check if expressions are suitable for SIMD processing
    pub fn is_simd_suitable(expressions: &[Expression], min_size: usize) -> bool {
        if expressions.len() < min_size {
            return false;
        }

        // Check if majority are numeric
        let numeric_count = expressions
            .iter()
            .filter(|expr| matches!(expr, Expression::Number(_)))
            .count();

        numeric_count as f64 / expressions.len() as f64 > 0.7 // 70% numeric threshold
    }

    /// Create SIMD-friendly test data
    pub fn simd_test_data(size: usize) -> (Vec<f64>, Vec<f64>) {
        let a: Vec<f64> = (0..size).map(|i| i as f64).collect();
        let b: Vec<f64> = (0..size).map(|i| (size - i) as f64).collect();
        (a, b)
    }
}
