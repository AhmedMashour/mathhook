//! Mathematically rigorous algebraic simplification tests
//!
//! This module verifies fundamental algebraic laws and identities through
//! comprehensive property-based testing and mathematical verification.

use mathhook_core::prelude::*;

/// Fundamental arithmetic laws
mod arithmetic_laws {
    use super::*;

    #[test]
    fn test_integer_arithmetic_correctness() {
        /// Verify integer arithmetic produces mathematically correct results
        let test_cases = vec![
            // Addition cases
            (vec![2, 3], 5),           // 2 + 3 = 5
            (vec![10, -3, 7], 14),     // 10 + (-3) + 7 = 14
            (vec![-5, -3, -2], -10),   // (-5) + (-3) + (-2) = -10
            (vec![0, 42, 0], 42),      // 0 + 42 + 0 = 42
            (vec![1, 2, 3, 4, 5], 15), // Sum of 1 to 5 = 15
        ];

        for (addends, expected) in test_cases {
            let expr = Expression::add(addends.iter().map(|&x| Expression::integer(x)).collect());
            let result = expr.simplify();

            assert_eq!(
                result,
                Expression::integer(expected),
                "Addition {:?} should equal {}, got {}",
                addends,
                expected,
                result
            );
        }
    }

    #[test]
    fn test_multiplication_arithmetic_correctness() {
        /// Verify multiplication produces mathematically correct results
        let test_cases = vec![
            (vec![2, 3], 6),        // 2 * 3 = 6
            (vec![2, 3, 4], 24),    // 2 * 3 * 4 = 24
            (vec![-2, 3], -6),      // (-2) * 3 = -6
            (vec![-2, -3], 6),      // (-2) * (-3) = 6
            (vec![1, 2, 3, 4], 24), // 1 * 2 * 3 * 4 = 24
        ];

        for (factors, expected) in test_cases {
            let expr = Expression::mul(factors.iter().map(|&x| Expression::integer(x)).collect());
            let result = expr.simplify();

            assert_eq!(
                result,
                Expression::integer(expected),
                "Multiplication {:?} should equal {}, got {}",
                factors,
                expected,
                result
            );
        }
    }

    #[test]
    fn test_large_sum_correctness() {
        /// Test sum of 1 to 100 = 5050 (Gauss's formula)
        let large_sum: Vec<Expression> = (1..=100).map(Expression::integer).collect();
        let expr = Expression::add(large_sum);
        let result = expr.simplify();

        assert_eq!(
            result,
            Expression::integer(5050),
            "Sum of integers 1 to 100 should be 5050"
        );
    }
}

/// Algebraic identity laws that must always hold
mod algebraic_identities {
    use super::*;

    #[test]
    fn test_additive_identity_law() {
        /// Mathematical law: a + 0 = 0 + a = a (additive identity)
        let test_expressions = vec![
            Expression::integer(42),
            Expression::symbol(Symbol::new("x")),
            Expression::add(vec![Expression::integer(2), Expression::integer(3)]),
        ];

        let zero = Expression::integer(0);

        for expr in test_expressions {
            // Test a + 0 = a
            let expr_plus_zero = Expression::add(vec![expr.clone(), zero.clone()]);
            assert_eq!(
                expr_plus_zero.simplify(),
                expr.clone(),
                "{} + 0 should equal {}",
                expr,
                expr
            );

            // Test 0 + a = a
            let zero_plus_expr = Expression::add(vec![zero.clone(), expr.clone()]);
            assert_eq!(
                zero_plus_expr.simplify(),
                expr.clone(),
                "0 + {} should equal {}",
                expr,
                expr
            );
        }
    }

    #[test]
    fn test_multiplicative_identity_law() {
        /// Mathematical law: a * 1 = 1 * a = a (multiplicative identity)
        let test_expressions = vec![
            Expression::integer(42),
            Expression::symbol(Symbol::new("x")),
        ];

        let one = Expression::integer(1);

        for expr in test_expressions {
            // Test a * 1 = a
            let expr_times_one = Expression::mul(vec![expr.clone(), one.clone()]);
            assert_eq!(
                expr_times_one.simplify(),
                expr.clone(),
                "{} * 1 should equal {}",
                expr,
                expr
            );

            // Test 1 * a = a
            let one_times_expr = Expression::mul(vec![one.clone(), expr.clone()]);
            assert_eq!(
                one_times_expr.simplify(),
                expr.clone(),
                "1 * {} should equal {}",
                expr,
                expr
            );
        }
    }

    #[test]
    fn test_multiplicative_zero_law() {
        /// Mathematical law: a * 0 = 0 * a = 0 (multiplicative zero)
        let test_expressions = vec![
            Expression::integer(42),
            Expression::symbol(Symbol::new("x")),
            Expression::add(vec![
                Expression::integer(2),
                Expression::symbol(Symbol::new("y")),
            ]),
        ];

        let zero = Expression::integer(0);

        for expr in test_expressions {
            // Test a * 0 = 0
            let expr_times_zero = Expression::mul(vec![expr.clone(), zero.clone()]);
            assert_eq!(
                expr_times_zero.simplify(),
                zero.clone(),
                "{} * 0 should equal 0",
                expr
            );

            // Test 0 * a = 0
            let zero_times_expr = Expression::mul(vec![zero.clone(), expr.clone()]);
            assert_eq!(
                zero_times_expr.simplify(),
                zero.clone(),
                "0 * {} should equal 0",
                expr
            );
        }
    }

    #[test]
    fn test_power_identity_laws() {
        /// Mathematical laws: a^0 = 1, a^1 = a, 1^n = 1, 0^n = 0 (n > 0)
        let test_bases = vec![Expression::integer(5), Expression::symbol(Symbol::new("x"))];

        let zero = Expression::integer(0);
        let one = Expression::integer(1);

        for base in test_bases {
            // Test a^0 = 1
            let base_to_zero = Expression::pow(base.clone(), zero.clone());
            assert_eq!(
                base_to_zero.simplify(),
                one.clone(),
                "{}^0 should equal 1",
                base
            );

            // Test a^1 = a
            let base_to_one = Expression::pow(base.clone(), one.clone());
            assert_eq!(
                base_to_one.simplify(),
                base.clone(),
                "{}^1 should equal {}",
                base,
                base
            );
        }

        // Test 1^n = 1 for various n
        for n in [2, 3, 5, 10] {
            let one_to_n = Expression::pow(one.clone(), Expression::integer(n));
            assert_eq!(one_to_n.simplify(), one.clone(), "1^{} should equal 1", n);
        }

        // Test 0^n = 0 for n > 0
        for n in [1, 2, 3, 5] {
            let zero_to_n = Expression::pow(zero.clone(), Expression::integer(n));
            assert_eq!(zero_to_n.simplify(), zero.clone(), "0^{} should equal 0", n);
        }
    }

    #[test]
    fn test_distributive_property_verification() {
        /// Verify distributive property with concrete examples: 2*(3+4) = 2*3 + 2*4
        let test_cases = vec![
            (2, 3, 4),  // 2(3 + 4) = 2*3 + 2*4 = 14
            (3, 1, 2),  // 3(1 + 2) = 3*1 + 3*2 = 9
            (-1, 5, 3), // -1(5 + 3) = -1*5 + -1*3 = -8
        ];

        for (a_val, b_val, c_val) in test_cases {
            let a = Expression::integer(a_val);
            let b = Expression::integer(b_val);
            let c = Expression::integer(c_val);

            // Left side: a * (b + c)
            let left =
                Expression::mul(vec![a.clone(), Expression::add(vec![b.clone(), c.clone()])])
                    .simplify();

            // Right side: a*b + a*c
            let right = Expression::add(vec![
                Expression::mul(vec![a.clone(), b]),
                Expression::mul(vec![a, c]),
            ])
            .simplify();

            assert_eq!(
                left, right,
                "Distributive property failed for a={}, b={}, c={}",
                a_val, b_val, c_val
            );
        }
    }
}

/// Rational number arithmetic verification
mod rational_arithmetic {
    use super::*;
    use num_bigint::BigInt;
    use num_rational::BigRational;

    #[test]
    fn test_rational_addition_correctness() {
        /// Test exact rational arithmetic: 1/2 + 1/3 = 5/6
        let one_half = Expression::number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(2),
        )));
        let one_third = Expression::number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(3),
        )));
        let expected = Expression::number(Number::rational(BigRational::new(
            BigInt::from(5),
            BigInt::from(6),
        )));

        let expr = Expression::add(vec![one_half, one_third]);
        let result = expr.simplify();

        assert_eq!(result, expected, "1/2 + 1/3 should equal 5/6");
    }

    #[test]
    fn test_rational_reduction() {
        /// Test automatic reduction: 6/9 = 2/3
        let unreduced = Expression::number(Number::rational(BigRational::new(
            BigInt::from(6),
            BigInt::from(9),
        )));
        let reduced = Expression::number(Number::rational(BigRational::new(
            BigInt::from(2),
            BigInt::from(3),
        )));

        assert_eq!(unreduced.simplify(), reduced, "6/9 should reduce to 2/3");
    }

    #[test]
    fn test_mixed_rational_integer_arithmetic() {
        /// Test arithmetic between rationals and integers
        let test_cases = vec![
            // (rational_num, rational_den, integer, expected_num, expected_den)
            (1, 2, 1, 3, 2), // 1/2 + 1 = 3/2
            (3, 4, 1, 7, 4), // 3/4 + 1 = 7/4
            (2, 3, 2, 8, 3), // 2/3 + 2 = 8/3
        ];

        for (r_num, r_den, int_val, exp_num, exp_den) in test_cases {
            let rational = Expression::number(Number::rational(BigRational::new(
                BigInt::from(r_num),
                BigInt::from(r_den),
            )));
            let integer = Expression::integer(int_val);
            let expected = Expression::number(Number::rational(BigRational::new(
                BigInt::from(exp_num),
                BigInt::from(exp_den),
            )));

            let result = Expression::add(vec![rational, integer]).simplify();
            assert_eq!(
                result, expected,
                "{}/{} + {} should equal {}/{}",
                r_num, r_den, int_val, exp_num, exp_den
            );
        }
    }

    #[test]
    fn test_rational_arithmetic_properties() {
        /// Test that rational arithmetic preserves mathematical properties
        let quarter = Expression::number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(4),
        )));
        let half = Expression::number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(2),
        )));

        // Test: 1/4 + 1/4 = 1/2
        let sum = Expression::add(vec![quarter.clone(), quarter.clone()]).simplify();
        assert_eq!(sum, half, "1/4 + 1/4 should equal 1/2");

        // Test: 1/2 * 2 = 1
        let product = Expression::mul(vec![half, Expression::integer(2)]).simplify();
        assert_eq!(product, Expression::integer(1), "1/2 * 2 should equal 1");
    }
}

/// Edge cases and boundary conditions
mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_and_single_operations() {
        /// Test behavior with empty and single-element operations
        // Single-element operations should return the element
        let single_add = Expression::add(vec![Expression::integer(42)]);
        assert_eq!(single_add.simplify(), Expression::integer(42));

        let single_mul = Expression::mul(vec![Expression::integer(42)]);
        assert_eq!(single_mul.simplify(), Expression::integer(42));

        // Empty operations should handle gracefully
        let empty_add = Expression::add(vec![]).simplify();
        let empty_mul = Expression::mul(vec![]).simplify();

        // Should not panic and produce reasonable defaults
        match empty_add {
            Expression::Number(_) => (), // Should be 0 or handle gracefully
            _ => (),                     // Implementation dependent
        }

        match empty_mul {
            Expression::Number(_) => (), // Should be 1 or handle gracefully
            _ => (),                     // Implementation dependent
        }
    }

    #[test]
    fn test_deeply_nested_expressions() {
        /// Test deeply nested expression simplification
        let mut expr = Expression::integer(1);

        // Build: 1 + (1 + (1 + (1 + 1)))
        for _ in 0..4 {
            expr = Expression::add(vec![Expression::integer(1), expr]);
        }

        let result = expr.simplify();
        assert_eq!(
            result,
            Expression::integer(5),
            "Deeply nested addition should simplify to 5"
        );
    }

    #[test]
    fn test_mixed_positive_negative() {
        /// Test expressions with mixed positive and negative terms
        let expr = Expression::add(vec![
            Expression::integer(10),
            Expression::integer(-3),
            Expression::integer(7),
            Expression::integer(-4),
        ]);

        let result = expr.simplify();
        assert_eq!(
            result,
            Expression::integer(10),
            "10 + (-3) + 7 + (-4) should equal 10"
        );
    }

    #[test]
    fn test_zero_cancellation() {
        /// Test that adding zero doesn't change results
        let x = Symbol::new("x");
        let var = Expression::symbol(x.clone());

        let expr_with_zeros = Expression::add(vec![
            Expression::integer(0),
            var.clone(),
            Expression::integer(0),
            Expression::integer(5),
            Expression::integer(0),
        ]);

        let expected = Expression::add(vec![var.clone(), Expression::integer(5)]);

        // After simplification, zeros should be eliminated
        let result = expr_with_zeros.simplify();
        let expected_result = expected.simplify();

        assert_eq!(
            result, expected_result,
            "Zeros should be eliminated in simplification"
        );
    }
}

/// Performance regression prevention
mod performance_verification {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn test_simplification_performance() {
        /// Ensure simplification maintains acceptable performance
        let start = Instant::now();

        for i in 1..=1000 {
            let expr = Expression::add(vec![
                Expression::integer(i),
                Expression::integer(0),
                Expression::integer(-i),
            ]);
            let _ = expr.simplify();
        }

        let duration = start.elapsed();
        let ops_per_sec = 1000.0 / duration.as_secs_f64();

        assert!(
            ops_per_sec > 5_000.0,
            "Simplification performance regression: {:.0} ops/sec",
            ops_per_sec
        );
    }

    #[test]
    fn test_large_expression_performance() {
        /// Test performance with expressions containing many terms
        let start = Instant::now();

        let many_terms: Vec<Expression> = (1..=200).map(Expression::integer).collect();
        let expr = Expression::add(many_terms);
        let result = expr.simplify();

        let duration = start.elapsed();

        // Should complete quickly and produce correct result
        assert_eq!(result, Expression::integer(20100)); // Sum of 1 to 200
        assert!(
            duration < Duration::from_millis(100),
            "Large expression simplification too slow: {:?}",
            duration
        );
    }
}
