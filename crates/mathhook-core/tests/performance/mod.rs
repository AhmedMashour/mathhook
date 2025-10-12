//! Performance benchmarking and regression testing
//!
//! This module provides comprehensive performance testing to ensure
//! mathematical operations maintain acceptable performance characteristics.

use mathhook_core::prelude::*;
use std::time::{Duration, Instant};

/// Benchmark utilities for consistent measurement
mod benchmark_utils {
    use super::*;

    pub fn benchmark_operation<F, R>(
        name: &str,
        operation: F,
        iterations: usize,
        warmup_iterations: usize,
    ) -> (Duration, Vec<R>)
    where
        F: Fn() -> R,
    {
        // Warmup runs to stabilize performance
        for _ in 0..warmup_iterations {
            let _ = operation();
        }

        // Collect results and timing
        let mut results = Vec::with_capacity(iterations);
        let start = Instant::now();

        for _ in 0..iterations {
            results.push(operation());
        }

        let duration = start.elapsed();
        let ops_per_sec = iterations as f64 / duration.as_secs_f64();
        let avg_time = duration.as_micros() as f64 / iterations as f64;

        println!(
            "{}: {:.0} ops/sec ({:.2}μs per op)",
            name, ops_per_sec, avg_time
        );

        (duration, results)
    }

    pub fn assert_performance_threshold(
        duration: Duration,
        iterations: usize,
        min_ops_per_sec: f64,
        operation_name: &str,
    ) {
        let actual_ops_per_sec = iterations as f64 / duration.as_secs_f64();
        assert!(
            actual_ops_per_sec >= min_ops_per_sec,
            "{} performance below threshold: {:.0} ops/sec < {:.0} ops/sec",
            operation_name,
            actual_ops_per_sec,
            min_ops_per_sec
        );
    }

    pub fn measure_memory_usage<F, R>(operation: F) -> R
    where
        F: FnOnce() -> R,
    {
        // Simple memory usage measurement
        // In production, this would use more sophisticated profiling
        operation()
    }
}

/// Core operation performance benchmarks
mod core_operations {
    use super::*;
    use benchmark_utils::*;

    #[test]
    fn test_integer_arithmetic_performance() {
        // Addition benchmark
        let (duration, _) = benchmark_operation(
            "Integer addition",
            || Expression::add(vec![Expression::integer(123), Expression::integer(456)]).simplify(),
            10000,
            1000,
        );
        assert_performance_threshold(duration, 10000, 50_000.0, "Integer addition");

        // Multiplication benchmark
        let (duration, _) = benchmark_operation(
            "Integer multiplication",
            || Expression::mul(vec![Expression::integer(123), Expression::integer(456)]).simplify(),
            10000,
            1000,
        );
        assert_performance_threshold(duration, 10000, 50_000.0, "Integer multiplication");

        // Power benchmark
        let (duration, _) = benchmark_operation(
            "Integer power",
            || Expression::pow(Expression::integer(12), Expression::integer(3)).simplify(),
            10000,
            1000,
        );
        assert_performance_threshold(duration, 10000, 25_000.0, "Integer power");
    }

    #[test]
    fn test_rational_arithmetic_performance() {
        use num_bigint::BigInt;
        /// Benchmark rational number arithmetic
        use num_rational::BigRational;

        let rational_a = Expression::number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(3),
        )));
        let rational_b = Expression::number(Number::rational(BigRational::new(
            BigInt::from(2),
            BigInt::from(5),
        )));

        let (duration, _) = benchmark_operation(
            "Rational addition",
            || Expression::add(vec![rational_a.clone(), rational_b.clone()]).simplify(),
            5000,
            500,
        );
        assert_performance_threshold(duration, 5000, 5_000.0, "Rational addition");
    }

    #[test]
    fn test_symbolic_operations_performance() {
        let x = symbol!(x);
        let y = symbol!(y);
        let var_x = Expression::symbol(x.clone());
        let var_y = Expression::symbol(y.clone());

        let (duration, _) = benchmark_operation(
            "Symbolic addition",
            || Expression::add(vec![var_x.clone(), var_y.clone()]).simplify(),
            10000,
            1000,
        );
        assert_performance_threshold(duration, 10000, 25_000.0, "Symbolic addition");

        // Symbolic multiplication
        let (duration, _) = benchmark_operation(
            "Symbolic multiplication",
            || Expression::mul(vec![var_x.clone(), var_y.clone()]).simplify(),
            10000,
            1000,
        );
        assert_performance_threshold(duration, 10000, 25_000.0, "Symbolic multiplication");
    }

    #[test]
    fn test_expression_construction_performance() {
        let (duration, _) = benchmark_operation(
            "Expression construction",
            || {
                Expression::add(vec![
                    Expression::integer(1),
                    Expression::integer(2),
                    Expression::integer(3),
                ])
            },
            50000,
            5000,
        );
        assert_performance_threshold(duration, 50000, 100_000.0, "Expression construction");
    }
}

/// GCD/LCM performance benchmarks
mod gcd_performance {
    use super::*;
    use benchmark_utils::*;

    #[test]
    fn test_gcd_performance_scaling() {
        let test_cases = vec![
            (123, 456, "Small integers"),
            (12345, 67890, "Medium integers"),
            (1234567890, 9876543210, "Large integers"),
        ];

        let mut previous_duration = Duration::from_nanos(1);

        for (a, b, description) in test_cases {
            let (duration, _) = benchmark_operation(
                &format!("GCD {}", description),
                || Expression::integer(a).gcd(&Expression::integer(b)),
                1000,
                100,
            );

            // Each size class should not be more than 50x slower than previous
            let scaling_factor = duration.as_nanos() as f64 / previous_duration.as_nanos() as f64;
            assert!(
                scaling_factor < 50.0 || previous_duration.as_nanos() == 1,
                "Poor GCD scaling: {:.1}x slower for {}",
                scaling_factor,
                description
            );

            previous_duration = duration;
        }
    }

    #[test]
    fn test_lcm_performance() {
        let (duration, _) = benchmark_operation(
            "LCM computation",
            || Expression::integer(12).lcm(&Expression::integer(18)),
            5000,
            500,
        );
        assert_performance_threshold(duration, 5000, 10_000.0, "LCM computation");
    }

    #[test]
    fn test_gcd_with_coprime_numbers() {
        let (duration, _) = benchmark_operation(
            "GCD coprime numbers",
            || Expression::integer(17).gcd(&Expression::integer(13)),
            10000,
            1000,
        );
        assert_performance_threshold(duration, 10000, 20_000.0, "GCD coprime numbers");
    }
}

/// Expression complexity performance
mod complexity_performance {
    use super::*;
    use benchmark_utils::*;

    #[test]
    fn test_large_expression_performance() {
        let sizes = vec![10, 50, 100, 200];

        for size in sizes {
            let terms: Vec<Expression> = (1..=size).map(Expression::integer).collect();

            let (duration, _) = benchmark_operation(
                &format!("Large addition ({})", size),
                || Expression::add(terms.clone()).simplify(),
                1000,
                100,
            );

            // Performance should scale reasonably (not exponentially)
            let ops_per_sec = 1000.0 / duration.as_secs_f64();
            let min_threshold = match size {
                10 => 20_000.0,
                50 => 5_000.0,
                100 => 2_000.0,
                200 => 500.0,
                _ => 100.0,
            };

            assert!(
                ops_per_sec >= min_threshold,
                "Large expression ({} terms) too slow: {:.0} ops/sec",
                size,
                ops_per_sec
            );
        }
    }

    #[test]
    fn test_nested_expression_performance() {
        let depths = vec![5, 10, 20];

        for depth in depths {
            let (duration, _) = benchmark_operation(
                &format!("Nested expression (depth {})", depth),
                || {
                    let mut expr = Expression::integer(1);
                    for i in 1..=depth {
                        expr = Expression::add(vec![Expression::integer(i), expr]);
                    }
                    expr.simplify()
                },
                1000,
                100,
            );

            let ops_per_sec = 1000.0 / duration.as_secs_f64();
            assert!(
                ops_per_sec >= 5_000.0,
                "Nested expression (depth {}) too slow: {:.0} ops/sec",
                depth,
                ops_per_sec
            );
        }
    }

    #[test]
    fn test_mixed_operation_performance() {
        let (duration, _) = benchmark_operation(
            "Mixed operations",
            || {
                let x = Expression::symbol(symbol!(x));
                Expression::add(vec![
                    Expression::mul(vec![Expression::integer(2), x.clone()]),
                    Expression::pow(x.clone(), Expression::integer(2)),
                    Expression::integer(5),
                ])
                .simplify()
            },
            5000,
            500,
        );
        assert_performance_threshold(duration, 5000, 10_000.0, "Mixed operations");
    }
}

/// Memory usage performance
mod memory_performance {
    use super::*;
    use benchmark_utils::*;

    #[test]
    fn test_memory_efficiency() {
        let iterations = 1000;
        let start = Instant::now();

        for i in 0..iterations {
            let expr = Expression::add(vec![
                Expression::integer(i),
                Expression::integer(i + 1),
                Expression::integer(i + 2),
            ]);
            let _ = expr.simplify();
        }

        let duration = start.elapsed();

        // Should complete quickly without excessive allocations
        assert!(
            duration < Duration::from_millis(200),
            "Memory-intensive operations taking too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_expression_cloning_performance() {
        let complex_expr = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(symbol!(x))]),
            Expression::pow(Expression::symbol(symbol!(y)), Expression::integer(3)),
            Expression::integer(42),
        ]);

        let (duration, _) =
            benchmark_operation("Expression cloning", || complex_expr.clone(), 50000, 5000);
        assert_performance_threshold(duration, 50000, 200_000.0, "Expression cloning");
    }

    #[test]
    fn test_large_number_memory_usage() {
        use num_bigint::BigInt;

        let large_numbers: Vec<Expression> = (0..100)
            .map(|i| {
                let big_int = BigInt::parse_bytes(
                    format!("123456789012345678901234567890{:02}", i).as_bytes(),
                    10,
                )
                .unwrap();
                Expression::big_integer(big_int)
            })
            .collect();

        let start = Instant::now();

        for expr in large_numbers {
            let _ = expr.simplify();
        }

        let duration = start.elapsed();

        // Should handle large numbers efficiently
        assert!(
            duration < Duration::from_millis(500),
            "Large number processing too slow: {:?}",
            duration
        );
    }
}

/// Regression testing
mod regression_tests {
    use super::*;
    use benchmark_utils::*;

    #[test]
    fn test_performance_regression_baseline() {
        // Test simple addition
        let (duration, _) = benchmark_operation(
            "Regression test: simple addition",
            || Expression::add(vec![Expression::integer(42), Expression::integer(58)]).simplify(),
            10000,
            1000,
        );
        assert_performance_threshold(duration, 10000, 50_000.0, "Simple addition regression");

        // Test GCD
        let (duration, _) = benchmark_operation(
            "Regression test: GCD",
            || Expression::integer(12345).gcd(&Expression::integer(67890)),
            5000,
            500,
        );
        assert_performance_threshold(duration, 5000, 25_000.0, "GCD regression");

        // Test symbolic operations
        let x = Expression::symbol(symbol!(x));
        let (duration, _) = benchmark_operation(
            "Regression test: symbolic",
            || Expression::add(vec![x.clone(), Expression::integer(1)]).simplify(),
            10000,
            1000,
        );
        assert_performance_threshold(duration, 10000, 25_000.0, "Symbolic regression");

        println!("All performance regression tests passed!");
    }

    #[test]
    fn test_complexity_regression() {
        let small_size = 50;
        let large_size = 200;

        // Benchmark small expression
        let small_terms: Vec<Expression> = (1..=small_size).map(Expression::integer).collect();
        let (small_duration, _) = benchmark_operation(
            "Complexity regression: small",
            || Expression::add(small_terms.clone()).simplify(),
            1000,
            100,
        );

        // Benchmark large expression
        let large_terms: Vec<Expression> = (1..=large_size).map(Expression::integer).collect();
        let (large_duration, _) = benchmark_operation(
            "Complexity regression: large",
            || Expression::add(large_terms.clone()).simplify(),
            1000,
            100,
        );

        // Large should not be more than 16x slower (4x size increase -> max 4² complexity)
        let complexity_ratio = large_duration.as_nanos() as f64 / small_duration.as_nanos() as f64;
        assert!(
            complexity_ratio < 16.0,
            "Algorithmic complexity regression: {:.1}x slower for 4x larger input",
            complexity_ratio
        );
    }
}

/// Comparative performance tests
mod comparative_performance {
    use super::*;
    use benchmark_utils::*;

    #[test]
    fn test_operation_relative_performance() {
        let iterations = 10000;

        // Addition
        let (add_duration, _) = benchmark_operation(
            "Relative: Addition",
            || Expression::add(vec![Expression::integer(123), Expression::integer(456)]).simplify(),
            iterations,
            1000,
        );

        // Multiplication
        let (mul_duration, _) = benchmark_operation(
            "Relative: Multiplication",
            || Expression::mul(vec![Expression::integer(123), Expression::integer(456)]).simplify(),
            iterations,
            1000,
        );

        // Power
        let (pow_duration, _) = benchmark_operation(
            "Relative: Power",
            || Expression::pow(Expression::integer(12), Expression::integer(3)).simplify(),
            iterations,
            1000,
        );

        // Verify reasonable relative performance
        let mul_add_ratio = mul_duration.as_nanos() as f64 / add_duration.as_nanos() as f64;
        let pow_add_ratio = pow_duration.as_nanos() as f64 / add_duration.as_nanos() as f64;

        assert!(
            mul_add_ratio < 5.0,
            "Multiplication should not be more than 5x slower than addition, got {:.1}x",
            mul_add_ratio
        );
        assert!(
            pow_add_ratio < 20.0,
            "Power should not be more than 20x slower than addition, got {:.1}x",
            pow_add_ratio
        );
    }

    #[test]
    fn test_integer_vs_rational_performance() {
        use num_bigint::BigInt;
        /// Compare performance between integer and rational arithmetic
        use num_rational::BigRational;

        let iterations = 5000;

        // Integer arithmetic
        let (int_duration, _) = benchmark_operation(
            "Integer vs Rational: Integer",
            || Expression::add(vec![Expression::integer(123), Expression::integer(456)]).simplify(),
            iterations,
            500,
        );

        // Rational arithmetic
        let rational_a = Expression::number(Number::rational(BigRational::new(
            BigInt::from(123),
            BigInt::from(1000),
        )));
        let rational_b = Expression::number(Number::rational(BigRational::new(
            BigInt::from(456),
            BigInt::from(1000),
        )));

        let (rat_duration, _) = benchmark_operation(
            "Integer vs Rational: Rational",
            || Expression::add(vec![rational_a.clone(), rational_b.clone()]).simplify(),
            iterations,
            500,
        );

        // Rational should be slower but not excessively so
        let ratio = rat_duration.as_nanos() as f64 / int_duration.as_nanos() as f64;
        assert!(
            ratio < 50.0,
            "Rational arithmetic should not be more than 50x slower than integer, got {:.1}x",
            ratio
        );
    }
}
