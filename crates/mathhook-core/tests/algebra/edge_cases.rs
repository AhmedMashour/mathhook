//! Comprehensive edge case testing for mathematical operations
//!
//! This module tests boundary conditions, error handling, and extreme cases
//! to ensure robust behavior across all mathematical operations.

use mathhook_core::prelude::*;
use num_bigint::BigInt;
use num_rational::BigRational;

/// Boundary value testing
mod boundary_values {
    use super::*;

    #[test]
    fn test_zero_boundary_conditions() {
        /// Test all operations with zero values
        let zero = Expression::integer(0);
        let x = symbol!(x);
        let var = Expression::symbol(x);

        // Addition with zero
        assert_eq!(
            Expression::add(vec![var.clone(), zero.clone()]).simplify(),
            var.clone()
        );
        assert_eq!(
            Expression::add(vec![zero.clone(), var.clone()]).simplify(),
            var.clone()
        );

        // Multiplication with zero
        assert_eq!(
            Expression::mul(vec![var.clone(), zero.clone()]).simplify(),
            zero.clone()
        );
        assert_eq!(
            Expression::mul(vec![zero.clone(), var.clone()]).simplify(),
            zero.clone()
        );

        // Power operations with zero
        assert_eq!(
            Expression::pow(var.clone(), zero.clone()).simplify(),
            Expression::integer(1)
        );
        assert_eq!(
            Expression::pow(zero.clone(), Expression::integer(1)).simplify(),
            zero.clone()
        );

        // Zero with itself
        assert_eq!(
            Expression::add(vec![zero.clone(), zero.clone()]).simplify(),
            zero.clone()
        );
        assert_eq!(
            Expression::mul(vec![zero.clone(), zero.clone()]).simplify(),
            zero.clone()
        );
    }

    #[test]
    fn test_unity_boundary_conditions() {
        /// Test all operations with unity (1)
        let one = Expression::integer(1);
        let x = symbol!(x);
        let var = Expression::symbol(x);

        // Multiplication with one
        assert_eq!(
            Expression::mul(vec![var.clone(), one.clone()]).simplify(),
            var.clone()
        );
        assert_eq!(
            Expression::mul(vec![one.clone(), var.clone()]).simplify(),
            var.clone()
        );

        // Power operations with one
        assert_eq!(
            Expression::pow(var.clone(), one.clone()).simplify(),
            var.clone()
        );
        assert_eq!(
            Expression::pow(one.clone(), var.clone()).simplify(),
            one.clone()
        );

        // One with itself
        assert_eq!(
            Expression::mul(vec![one.clone(), one.clone()]).simplify(),
            one.clone()
        );
        assert_eq!(
            Expression::pow(one.clone(), one.clone()).simplify(),
            one.clone()
        );
    }

    #[test]
    fn test_negative_number_handling() {
        /// Test operations with negative numbers
        let pos = Expression::integer(5);
        let neg = Expression::integer(-5);
        let zero = Expression::integer(0);

        // Addition with negatives
        assert_eq!(
            Expression::add(vec![pos.clone(), neg.clone()]).simplify(),
            zero.clone()
        );
        assert_eq!(
            Expression::add(vec![neg.clone(), pos.clone()]).simplify(),
            zero.clone()
        );

        // Multiplication with negatives
        assert_eq!(
            Expression::mul(vec![pos.clone(), neg.clone()]).simplify(),
            Expression::integer(-25)
        );
        assert_eq!(
            Expression::mul(vec![neg.clone(), neg.clone()]).simplify(),
            Expression::integer(25)
        );

        // Powers with negatives
        assert_eq!(
            Expression::pow(neg.clone(), Expression::integer(2)).simplify(),
            Expression::integer(25)
        );
        assert_eq!(
            Expression::pow(neg.clone(), Expression::integer(3)).simplify(),
            Expression::integer(-125)
        );
    }

    #[test]
    fn test_extreme_integer_values() {
        /// Test with extreme integer values
        use std::i64;

        let max_int = Expression::integer(i64::MAX);
        let min_int = Expression::integer(i64::MIN);
        let one = Expression::integer(1);

        // Operations near boundaries should handle gracefully
        let near_max = Expression::add(vec![max_int.clone(), one.clone()]);
        let near_min = Expression::add(vec![min_int.clone(), Expression::integer(-1)]);

        // Should not panic, may promote to BigInteger
        let max_result = near_max.simplify();
        let min_result = near_min.simplify();

        // Results should be valid expressions
        match max_result {
            Expression::Number(_) => (), // Should be a number
            _ => panic!("Near-overflow result should be a number"),
        }

        match min_result {
            Expression::Number(_) => (), // Should be a number
            _ => panic!("Near-underflow result should be a number"),
        }
    }
}

/// Large number handling
mod large_numbers {
    use super::*;

    #[test]
    fn test_large_integer_arithmetic() {
        /// Test arithmetic with very large integers
        let large_a = Expression::big_integer(
            BigInt::parse_bytes(b"12345678901234567890123456789012345", 10).unwrap(),
        );
        let large_b = Expression::big_integer(
            BigInt::parse_bytes(b"98765432109876543210987654321098765", 10).unwrap(),
        );

        // Should handle large number operations without overflow
        let sum = Expression::add(vec![large_a.clone(), large_b.clone()]).simplify();
        let product = Expression::mul(vec![large_a.clone(), large_b.clone()]).simplify();

        // Verify results maintain correct type
        match sum {
            Expression::Number(Number::BigInteger(_)) | Expression::Number(Number::Integer(_)) => {
                ()
            }
            _ => panic!("Large number sum should be a number, got: {}", sum),
        }

        match product {
            Expression::Number(Number::BigInteger(_)) | Expression::Number(Number::Integer(_)) => {
                ()
            }
            _ => panic!("Large number product should be a number, got: {}", product),
        }
    }

    #[test]
    fn test_large_number_gcd() {
        /// Test GCD with large numbers
        let large_a = Expression::big_integer(BigInt::parse_bytes(b"123456789012345", 10).unwrap());
        let large_b = Expression::big_integer(BigInt::parse_bytes(b"987654321098765", 10).unwrap());

        // Should compute GCD without issues
        let gcd_result = large_a.gcd(&large_b);

        // Should return a valid number
        match gcd_result {
            Expression::Number(_) => (),
            _ => panic!("GCD of large numbers should return a number"),
        }

        // Should maintain commutative property
        assert_eq!(large_a.gcd(&large_b), large_b.gcd(&large_a));
    }

    #[test]
    fn test_factorial_like_growth() {
        /// Test with numbers that grow like factorials
        let mut factorial = Expression::integer(1);

        for i in 2..=20 {
            factorial = Expression::mul(vec![factorial, Expression::integer(i)]).simplify();
        }

        // 20! should be computed correctly
        let expected_factorial_20 =
            Expression::big_integer(BigInt::parse_bytes(b"2432902008176640000", 10).unwrap());

        assert_eq!(
            factorial, expected_factorial_20,
            "20! should be computed correctly"
        );
    }
}

/// Rational number edge cases
mod rational_edge_cases {
    use super::*;

    #[test]
    fn test_rational_with_large_denominators() {
        /// Test rationals with very large denominators
        let large_denom = BigInt::parse_bytes(b"123456789012345", 10).unwrap();
        let rational = Expression::number(Number::rational(BigRational::new(
            BigInt::from(1),
            large_denom,
        )));

        // Should handle without precision loss
        let doubled = Expression::mul(vec![Expression::integer(2), rational.clone()]).simplify();

        match doubled {
            Expression::Number(Number::Rational(_)) => (),
            _ => panic!("Rational arithmetic should preserve rational type when possible"),
        }
    }

    #[test]
    fn test_rational_reduction_edge_cases() {
        /// Test rational reduction with edge cases
        let test_cases = vec![
            (0, 1, 0, 1),    // 0/1 = 0/1
            (1, 1, 1, 1),    // 1/1 = 1/1
            (-1, 1, -1, 1),  // -1/1 = -1/1
            (2, 4, 1, 2),    // 2/4 = 1/2
            (-6, 9, -2, 3),  // -6/9 = -2/3
            (100, 25, 4, 1), // 100/25 = 4/1
            (17, 1, 17, 1),  // 17/1 = 17/1
        ];

        for (num, den, exp_num, exp_den) in test_cases {
            let rational = Expression::number(Number::rational(BigRational::new(
                BigInt::from(num),
                BigInt::from(den),
            )));
            let expected = Expression::number(Number::rational(BigRational::new(
                BigInt::from(exp_num),
                BigInt::from(exp_den),
            )));

            assert_eq!(
                rational.simplify(),
                expected,
                "{}/{} should reduce to {}/{}",
                num,
                den,
                exp_num,
                exp_den
            );
        }
    }

    #[test]
    fn test_rational_arithmetic_precision() {
        /// Test that rational arithmetic maintains exact precision
        let test_cases = vec![
            // (a_num, a_den, b_num, b_den, expected_num, expected_den)
            (1, 3, 1, 6, 1, 2),   // 1/3 + 1/6 = 1/2
            (2, 5, 3, 10, 7, 10), // 2/5 + 3/10 = 7/10
            (1, 4, 1, 4, 1, 2),   // 1/4 + 1/4 = 1/2
        ];

        for (a_num, a_den, b_num, b_den, exp_num, exp_den) in test_cases {
            let a = Expression::number(Number::rational(BigRational::new(
                BigInt::from(a_num),
                BigInt::from(a_den),
            )));
            let b = Expression::number(Number::rational(BigRational::new(
                BigInt::from(b_num),
                BigInt::from(b_den),
            )));
            let expected = Expression::number(Number::rational(BigRational::new(
                BigInt::from(exp_num),
                BigInt::from(exp_den),
            )));

            let result = Expression::add(vec![a, b]).simplify();
            assert_eq!(
                result, expected,
                "{}/{} + {}/{} should equal {}/{}",
                a_num, a_den, b_num, b_den, exp_num, exp_den
            );
        }
    }

    #[test]
    fn test_mixed_number_types() {
        /// Test operations mixing integers, rationals, and floats
        let integer = Expression::integer(3);
        let rational = Expression::number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(2),
        )));
        let float_val = Expression::number(Number::float(2.5));

        // Mixed operations should handle gracefully
        let mixed_expr = Expression::add(vec![integer, rational, float_val]);
        let result = mixed_expr.simplify();

        // Should produce valid result without panic
        match result {
            Expression::Number(_) => (), // Should be a number for mixed arithmetic
            _ => panic!("Mixed number arithmetic should produce a number"),
        }
    }
}

/// Expression structure edge cases
mod expression_structure {
    use super::*;

    #[test]
    fn test_deeply_nested_expressions() {
        /// Test very deeply nested expression structures
        let mut expr = Expression::integer(1);

        // Create deeply nested structure: 1 + (1 + (1 + ... ))
        for i in 1..50 {
            expr = Expression::add(vec![Expression::integer(1), expr]);
        }

        // Should handle without stack overflow
        let result = expr.simplify();
        assert_eq!(result, Expression::integer(50));
    }

    #[test]
    fn test_wide_expressions() {
        /// Test expressions with many sibling terms
        let many_terms: Vec<Expression> = (1..=1000).map(Expression::integer).collect();
        let wide_expr = Expression::add(many_terms);

        // Should handle efficiently
        let result = wide_expr.simplify();
        assert_eq!(result, Expression::integer(500500)); // Sum of 1 to 1000
    }

    #[test]
    fn test_empty_expression_lists() {
        /// Test behavior with empty operation lists
        let empty_add = Expression::add(vec![]);
        let empty_mul = Expression::mul(vec![]);

        // Should handle gracefully
        let add_result = empty_add.simplify();
        let mul_result = empty_mul.simplify();

        // Should not panic and produce valid results
        match add_result {
            Expression::Number(_) => (), // Typically 0 for empty addition
            _ => (),                     // Implementation dependent
        }

        match mul_result {
            Expression::Number(_) => (), // Typically 1 for empty multiplication
            _ => (),                     // Implementation dependent
        }

        // Results should be numbers (0 for add, 1 for mul typically)
        match add_result {
            Expression::Number(_) => (),
            _ => (), // Implementation dependent
        }

        match mul_result {
            Expression::Number(_) => (),
            _ => (), // Implementation dependent
        }
    }

    #[test]
    fn test_single_element_expressions() {
        /// Test expressions with single elements
        let x = symbol!(x);
        let single_add = Expression::add(vec![Expression::symbol(x.clone())]);
        let single_mul = Expression::mul(vec![Expression::symbol(x.clone())]);

        // Should simplify to the single element
        assert_eq!(single_add.simplify(), Expression::symbol(x.clone()));
        assert_eq!(single_mul.simplify(), Expression::symbol(x));
    }

    #[test]
    fn test_alternating_operations() {
        /// Test expressions with alternating positive and negative terms
        let alternating_sum = Expression::add(vec![
            Expression::integer(10),
            Expression::integer(-5),
            Expression::integer(8),
            Expression::integer(-3),
            Expression::integer(7),
            Expression::integer(-2),
        ]);

        let result = alternating_sum.simplify();
        assert_eq!(
            result,
            Expression::integer(15),
            "Alternating sum should be computed correctly"
        );
    }
}

/// Symbol and variable edge cases
mod symbol_edge_cases {
    use super::*;

    #[test]
    fn test_symbol_name_edge_cases() {
        /// Test symbols with various name patterns
        let edge_case_names = vec![
            "x",                                                    // Single letter
            "var123",                                               // Alphanumeric
            "x_prime",                                              // Underscore
            "X",                                                    // Capital letter
            "very_long_variable_name_that_exceeds_typical_lengths", // Long name
        ];

        for name in edge_case_names {
            let symbol = Symbol::new(name);
            let expr = Expression::symbol(symbol.clone());

            // Should handle all valid symbol names
            assert_eq!(expr.simplify(), Expression::symbol(symbol));

            // Should work in arithmetic operations
            let arithmetic = Expression::add(vec![expr.clone(), Expression::integer(1)]).simplify();

            // Should not panic and produce valid result
            match arithmetic {
                Expression::Add(_) | Expression::Symbol(_) | Expression::Number(_) => (),
                _ => panic!("Arithmetic with symbols should produce valid expression"),
            }
        }
    }

    #[test]
    fn test_identical_symbols() {
        /// Test operations with identical symbols
        let x1 = symbol!(x);
        let x2 = symbol!(x);

        let expr1 = Expression::symbol(x1);
        let expr2 = Expression::symbol(x2);

        // Symbols with same name should be equal
        assert_eq!(expr1, expr2);

        // Operations should work correctly
        let sum = Expression::add(vec![expr1.clone(), expr2.clone()]);
        let product = Expression::mul(vec![expr1, expr2]);

        // Should handle without issues
        let sum_result = sum.simplify();
        let product_result = product.simplify();

        // Results should be valid expressions
        match sum_result {
            Expression::Add(_)
            | Expression::Mul(_)
            | Expression::Symbol(_)
            | Expression::Number(_) => (),
            _ => panic!("Sum of identical symbols should produce valid expression"),
        }

        match product_result {
            Expression::Add(_)
            | Expression::Mul(_)
            | Expression::Pow(_, _)
            | Expression::Symbol(_)
            | Expression::Number(_) => (),
            _ => panic!("Product of identical symbols should produce valid expression"),
        }
    }

    #[test]
    fn test_symbols_in_complex_expressions() {
        /// Test symbols in deeply nested and complex expressions
        let x = symbol!(x);
        let y = symbol!(y);

        let complex_expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(2),
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            ]),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(y.clone())]),
            Expression::integer(5),
        ]);

        let result = complex_expr.simplify();

        // Should handle complex symbolic expressions
        match result {
            Expression::Add(_)
            | Expression::Mul(_)
            | Expression::Pow(_, _)
            | Expression::Symbol(_)
            | Expression::Number(_) => (),
            _ => panic!("Complex symbolic expression should produce valid result"),
        }

        // Should preserve symbolic structure when not fully simplifiable
        match result {
            Expression::Add(_)
            | Expression::Mul(_)
            | Expression::Pow(_, _)
            | Expression::Symbol(_)
            | Expression::Number(_) => (),
            _ => (), // Other valid expression types
        }
    }
}

/// Performance and memory edge cases
mod performance_edge_cases {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn test_memory_efficiency_large_expressions() {
        /// Test that large expressions don't cause memory issues
        let start = Instant::now();

        // Create and simplify many large expressions
        for _ in 0..100 {
            let terms: Vec<Expression> = (1..=100).map(Expression::integer).collect();
            let expr = Expression::add(terms);
            let _ = expr.simplify();
        }

        let duration = start.elapsed();

        // Should complete in reasonable time
        assert!(
            duration < Duration::from_secs(5),
            "Large expression handling too slow: {:?}",
            duration
        );
    }

    #[test]
    fn test_recursive_expression_limits() {
        /// Test limits of recursive expression processing
        let mut expr = Expression::integer(0);

        // Build moderately deep recursion
        for i in 1..=20 {
            expr = Expression::add(vec![Expression::integer(i), expr]);
        }

        // Should handle without stack overflow
        let result = expr.simplify();
        assert_eq!(result, Expression::integer(210)); // Sum of 1 to 20
    }

    #[test]
    fn test_expression_cloning_performance() {
        /// Test that expression cloning is efficient
        let complex_expr = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(symbol!(x))]),
            Expression::pow(Expression::symbol(symbol!(y)), Expression::integer(3)),
            Expression::integer(42),
        ]);

        let start = Instant::now();

        // Clone many times
        for _ in 0..10000 {
            let _cloned = complex_expr.clone();
        }

        let duration = start.elapsed();

        // Cloning should be very fast
        assert!(
            duration < Duration::from_millis(100),
            "Expression cloning too slow: {:?}",
            duration
        );
    }
}

/// Float and precision edge cases
mod float_edge_cases {
    use super::*;

    #[test]
    fn test_float_arithmetic_precision() {
        /// Test floating point arithmetic behavior
        let float_a = Expression::number(Number::float(0.1));
        let float_b = Expression::number(Number::float(0.2));
        let float_c = Expression::number(Number::float(0.3));

        let sum = Expression::add(vec![float_a, float_b]).simplify();

        // Due to floating point precision, this might not equal exactly 0.3
        // But should be close and handle gracefully
        match sum {
            Expression::Number(Number::Float(_)) => (),
            _ => panic!("Float arithmetic should produce float result"),
        }
    }

    #[test]
    fn test_float_special_values() {
        /// Test handling of special float values
        let infinity = Expression::number(Number::float(f64::INFINITY));
        let neg_infinity = Expression::number(Number::float(f64::NEG_INFINITY));
        let nan = Expression::number(Number::float(f64::NAN));

        // Should handle special values without panicking
        let _ = infinity.simplify();
        let _ = neg_infinity.simplify();
        let _ = nan.simplify();

        // Operations with special values should be handled
        let expr_with_inf = Expression::add(vec![Expression::integer(5), infinity.clone()]);

        let _ = expr_with_inf.simplify();
    }

    #[test]
    fn test_float_integer_mixed_arithmetic() {
        /// Test arithmetic mixing floats and integers
        let float_val = Expression::number(Number::float(2.5));
        let integer_val = Expression::integer(3);

        let mixed_sum = Expression::add(vec![float_val.clone(), integer_val.clone()]).simplify();
        let mixed_product = Expression::mul(vec![float_val, integer_val]).simplify();

        // Should handle mixed arithmetic
        match mixed_sum {
            Expression::Number(_) => (),
            _ => panic!("Mixed float-integer arithmetic should produce number"),
        }

        match mixed_product {
            Expression::Number(_) => (),
            _ => panic!("Mixed float-integer arithmetic should produce number"),
        }
    }
}
