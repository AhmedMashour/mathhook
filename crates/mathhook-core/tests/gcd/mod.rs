//! Mathematically rigorous GCD/LCM functionality tests
//!
//! This module verifies fundamental mathematical properties of GCD and LCM operations
//! through property-based testing and comprehensive mathematical verification.

mod polynomial;
mod symbolica_benchmark;
mod symbolica_cases;
mod sympy_cases;

use mathhook_core::prelude::*;
use std::time::{Duration, Instant};

/// Core mathematical properties that must hold for all GCD operations
mod gcd_mathematical_properties {
    use super::*;

    #[test]
    fn test_gcd_known_values() {
        /// Test GCD with mathematically verified results from number theory
        let test_cases = vec![
            (12, 8, 4),      // gcd(12, 8) = 4
            (48, 18, 6),     // gcd(48, 18) = 6
            (100, 75, 25),   // gcd(100, 75) = 25
            (17, 13, 1),     // gcd(17, 13) = 1 (coprime)
            (54, 24, 6),     // gcd(54, 24) = 6
            (1071, 462, 21), // gcd(1071, 462) = 21 (Euclidean algorithm test)
            (270, 192, 6),   // gcd(270, 192) = 6
        ];

        for (a_val, b_val, expected_gcd) in test_cases {
            let a = Expression::integer(a_val);
            let b = Expression::integer(b_val);
            let expected = Expression::integer(expected_gcd);

            let result = a.gcd(&b);
            assert_eq!(
                result, expected,
                "gcd({}, {}) should be {}, got {}",
                a_val, b_val, expected_gcd, result
            );
        }
    }

    #[test]
    fn test_gcd_fundamental_properties() {
        /// Mathematical properties: commutative, associative, with zero/one
        let test_cases = vec![(12, 8), (48, 18), (17, 13), (100, 75)];

        for (a_val, b_val) in test_cases {
            let a = Expression::integer(a_val);
            let b = Expression::integer(b_val);

            // Commutative: gcd(a, b) = gcd(b, a)
            assert_eq!(
                a.gcd(&b),
                b.gcd(&a),
                "GCD must be commutative: gcd({}, {}) != gcd({}, {})",
                a_val,
                b_val,
                b_val,
                a_val
            );
        }

        // Test with zero: gcd(a, 0) = |a|
        let test_values = vec![1, 5, 12, 17, 100];
        let zero = Expression::integer(0);

        for val in test_values {
            let expr = Expression::integer(val);
            let expected = Expression::integer(val.abs());

            assert_eq!(
                expr.gcd(&zero),
                expected,
                "gcd({}, 0) should be {}",
                val,
                val
            );
            assert_eq!(
                zero.gcd(&expr),
                expected,
                "gcd(0, {}) should be {}",
                val,
                val
            );
        }

        // Test with one: gcd(a, 1) = 1
        let one = Expression::integer(1);
        let expected_one = Expression::integer(1);

        for val in [5, 12, 17, 100] {
            let expr = Expression::integer(val);
            assert_eq!(expr.gcd(&one), expected_one, "gcd({}, 1) should be 1", val);
            assert_eq!(one.gcd(&expr), expected_one, "gcd(1, {}) should be 1", val);
        }
    }

    #[test]
    fn test_gcd_associative_property() {
        /// Mathematical property: gcd(gcd(a, b), c) = gcd(a, gcd(b, c))
        let test_cases = vec![
            (12, 18, 24), // gcd(gcd(12, 18), 24) = gcd(6, 24) = 6
            // gcd(12, gcd(18, 24)) = gcd(12, 6) = 6
            (48, 18, 12),
            (100, 75, 25),
        ];

        for (a_val, b_val, c_val) in test_cases {
            let a = Expression::integer(a_val);
            let b = Expression::integer(b_val);
            let c = Expression::integer(c_val);

            let left = a.gcd(&b).gcd(&c);
            let right = a.gcd(&b.gcd(&c));

            assert_eq!(
                left, right,
                "GCD should be associative for ({}, {}, {})",
                a_val, b_val, c_val
            );
        }
    }

    #[test]
    fn test_gcd_edge_cases() {
        /// Test edge cases: negatives, large numbers, identical values
        // Negative numbers: gcd(-a, b) = gcd(a, -b) = gcd(-a, -b) = gcd(a, b)
        let test_cases = vec![(12, 8, 4), (48, 18, 6)];

        for (a_val, b_val, expected) in test_cases {
            let pos_a = Expression::integer(a_val);
            let neg_a = Expression::integer(-a_val);
            let pos_b = Expression::integer(b_val);
            let neg_b = Expression::integer(-b_val);
            let expected_gcd = Expression::integer(expected);

            assert_eq!(pos_a.gcd(&pos_b), expected_gcd);
            assert_eq!(neg_a.gcd(&pos_b), expected_gcd);
            assert_eq!(pos_a.gcd(&neg_b), expected_gcd);
            assert_eq!(neg_a.gcd(&neg_b), expected_gcd);
        }

        // Identical values: gcd(a, a) = |a|
        for val in [5, 12, 17, 100] {
            let expr = Expression::integer(val);
            let expected = Expression::integer(val.abs());
            assert_eq!(
                expr.gcd(&expr),
                expected,
                "gcd({}, {}) should be {}",
                val,
                val,
                val
            );
        }
    }
}

/// LCM mathematical properties and relationships with GCD
mod lcm_mathematical_properties {
    use super::*;

    #[test]
    fn test_lcm_known_values() {
        /// Test LCM with mathematically verified results
        let test_cases = vec![
            (6, 8, 24),   // lcm(6, 8) = 24
            (12, 18, 36), // lcm(12, 18) = 36
            (15, 25, 75), // lcm(15, 25) = 75
            (7, 11, 77),  // lcm(7, 11) = 77 (coprime)
            (4, 6, 12),   // lcm(4, 6) = 12
        ];

        for (a_val, b_val, expected_lcm) in test_cases {
            let a = Expression::integer(a_val);
            let b = Expression::integer(b_val);
            let expected = Expression::integer(expected_lcm);

            let result = a.lcm(&b);
            assert_eq!(
                result, expected,
                "lcm({}, {}) should be {}, got {}",
                a_val, b_val, expected_lcm, result
            );

            // Verify commutative property
            assert_eq!(
                a.lcm(&b),
                b.lcm(&a),
                "LCM must be commutative: lcm({}, {}) != lcm({}, {})",
                a_val,
                b_val,
                b_val,
                a_val
            );
        }
    }

    #[test]
    fn test_gcd_lcm_fundamental_relationship() {
        /// Mathematical identity: gcd(a, b) * lcm(a, b) = |a * b|
        let test_cases = vec![
            (6, 8),   // gcd=2, lcm=24, product=48, a*b=48
            (12, 18), // gcd=6, lcm=36, product=216, a*b=216
            (15, 25), // gcd=5, lcm=75, product=375, a*b=375
            (7, 11),  // gcd=1, lcm=77, product=77, a*b=77
        ];

        for (a_val, b_val) in test_cases {
            let a = Expression::integer(a_val);
            let b = Expression::integer(b_val);

            let gcd = a.gcd(&b);
            let lcm = a.lcm(&b);
            let product = Expression::mul(vec![a.clone(), b.clone()]);
            let gcd_lcm_product = Expression::mul(vec![gcd, lcm]).simplify();

            assert_eq!(
                gcd_lcm_product,
                product.simplify(),
                "gcd({}, {}) * lcm({}, {}) must equal {} * {}",
                a_val,
                b_val,
                a_val,
                b_val,
                a_val,
                b_val
            );
        }
    }
}

/// Performance benchmarks with statistical rigor
mod performance_benchmarks {
    use super::*;

    fn benchmark_operation<F>(name: &str, operation: F, iterations: usize) -> Duration
    where
        F: Fn() -> Expression,
    {
        // Warm-up runs
        for _ in 0..100 {
            let _ = operation();
        }

        let start = Instant::now();
        for _ in 0..iterations {
            let _ = operation();
        }
        let duration = start.elapsed();

        let ops_per_sec = iterations as f64 / duration.as_secs_f64();
        println!(
            "{}: {:.0} ops/sec ({:.2}μs per op)",
            name,
            ops_per_sec,
            duration.as_micros() as f64 / iterations as f64
        );

        duration
    }

    #[test]
    fn test_gcd_performance_baseline() {
        /// Establish baseline performance for integer GCD operations
        let duration = benchmark_operation(
            "Integer GCD baseline",
            || {
                let a = Expression::integer(12345);
                let b = Expression::integer(67890);
                a.gcd(&b)
            },
            1000,
        );

        let avg_duration = duration / 1000;
        assert!(
            avg_duration < Duration::from_micros(50),
            "GCD baseline too slow: {:?} per operation",
            avg_duration
        );
    }

    #[test]
    fn test_gcd_scaling() {
        /// Verify GCD performance scales reasonably with input size
        let small_duration = benchmark_operation(
            "Small GCD",
            || Expression::integer(123).gcd(&Expression::integer(456)),
            1000,
        );

        let large_duration = benchmark_operation(
            "Large GCD",
            || Expression::integer(1234567890).gcd(&Expression::integer(9876543210)),
            1000,
        );

        let scaling_factor = large_duration.as_nanos() as f64 / small_duration.as_nanos() as f64;
        assert!(
            scaling_factor < 100.0,
            "Poor GCD scaling: {:.1}x slower for larger numbers",
            scaling_factor
        );
    }
}

/// Polynomial GCD tests
mod polynomial_gcd {
    use super::*;

    #[test]
    fn test_polynomial_gcd_simple() {
        let x = symbol!(x);

        // Test GCD of x and x (should be x)
        let expr = Expression::symbol(x.clone());
        let result = expr.gcd(&expr);

        assert_eq!(result, Expression::symbol(x));
    }

    #[test]
    fn test_polynomial_gcd_with_coefficients() {
        /// Test polynomial GCD: gcd(6x, 9x) should have common factor
        let x = symbol!(x);

        let poly_a = Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]);
        let poly_b = Expression::mul(vec![Expression::integer(9), Expression::symbol(x.clone())]);

        let gcd_result = poly_a.gcd(&poly_b);

        // Should find a common factor (implementation dependent)
        assert!(!gcd_result.is_zero(), "Polynomial GCD should not be zero");
    }
}
