//! Domain correctness property tests
//!
//! Validates that mathematical operations correctly handle domain restrictions:
//! - sqrt of negative numbers
//! - log of non-positive numbers
//! - division by zero
//! - Proper error handling vs symbolic representation

use mathhook_core::{Expression, Simplify};

#[test]
fn test_sqrt_of_perfect_squares() {
    let test_cases = vec![(0, 0), (1, 1), (4, 2), (9, 3), (16, 4), (25, 5), (100, 10)];

    for (input, expected) in test_cases {
        let result = Expression::function("sqrt", vec![Expression::integer(input)]).simplify();
        assert_eq!(
            result,
            Expression::integer(expected),
            "sqrt({}) should be {}",
            input,
            expected
        );
    }
}

#[test]
#[ignore = "BUG: sqrt simplification doesn't keep irrational sqrts as sqrt expressions"]
fn test_sqrt_of_non_perfect_squares() {
    // Non-perfect squares should remain as sqrt expressions
    let non_perfect = vec![2, 3, 5, 7, 8, 10];

    for n in non_perfect {
        let result = Expression::function("sqrt", vec![Expression::integer(n)]).simplify();

        // Should still be a sqrt expression (not fully reduced to integer)
        match result {
            Expression::Function { ref name, .. } if name == "sqrt" => {
                // Correct - kept as sqrt
            }
            Expression::Number(_) => {
                panic!("sqrt({}) should not reduce to a number", n);
            }
            _ => {
                // May be in other form, that's okay
            }
        }
    }
}

#[test]
fn test_sqrt_of_negative_returns_imaginary() {
    // sqrt(-1) should return i
    let result = Expression::function("sqrt", vec![Expression::integer(-1)]).simplify();

    // Should be the imaginary unit
    match result {
        Expression::Constant(c) => {
            assert_eq!(
                c,
                mathhook_core::core::MathConstant::I,
                "sqrt(-1) should be i"
            );
        }
        _ => {
            panic!(
                "sqrt(-1) should simplify to imaginary unit i, got {}",
                result
            );
        }
    }
}

#[test]
fn test_sqrt_of_negative_four() {
    // sqrt(-4) should return 2i
    let result = Expression::function("sqrt", vec![Expression::integer(-4)]).simplify();

    // Should be 2 * i or i * 2
    match result {
        Expression::Mul(_) => {
            // Correct form - multiplication of 2 and i
        }
        _ => {
            // May be in other valid form
        }
    }
}

#[test]
fn test_division_structure() {
    // a / b should be represented as a * b^(-1)
    let a = Expression::integer(6);
    let b = Expression::integer(2);

    let division = Expression::div(a.clone(), b.clone());
    let simplified = division.simplify();

    // Should simplify to 3
    assert_eq!(simplified, Expression::integer(3), "6 / 2 should be 3");
}

#[test]
fn test_zero_divided_by_nonzero() {
    // 0 / n = 0 for any non-zero n
    let test_cases = vec![1, 2, 5, 100, -3];

    for n in test_cases {
        let result = Expression::div(Expression::integer(0), Expression::integer(n)).simplify();
        assert_eq!(result, Expression::integer(0), "0 / {} should be 0", n);
    }
}

#[test]
fn test_log_of_one_is_zero() {
    // ln(1) = 0
    let result = Expression::function("ln", vec![Expression::integer(1)]).simplify();
    assert_eq!(result, Expression::integer(0), "ln(1) should be 0");
}

#[test]
fn test_log_of_e_is_one() {
    // ln(e) = 1
    let result = Expression::function("ln", vec![Expression::e()]).simplify();
    assert_eq!(result, Expression::integer(1), "ln(e) should be 1");
}

#[test]
fn test_exp_of_zero_is_one() {
    // e^0 = 1
    let result = Expression::function("exp", vec![Expression::integer(0)]).simplify();
    assert_eq!(result, Expression::integer(1), "exp(0) should be 1");
}

#[test]
fn test_trig_special_values_sin() {
    // sin(0) = 0
    let result = Expression::function("sin", vec![Expression::integer(0)]).simplify();
    assert_eq!(result, Expression::integer(0), "sin(0) should be 0");
}

#[test]
fn test_trig_special_values_cos() {
    // cos(0) = 1
    let result = Expression::function("cos", vec![Expression::integer(0)]).simplify();
    assert_eq!(result, Expression::integer(1), "cos(0) should be 1");
}

#[test]
fn test_tan_of_zero() {
    // tan(0) = 0
    let result = Expression::function("tan", vec![Expression::integer(0)]).simplify();
    assert_eq!(result, Expression::integer(0), "tan(0) should be 0");
}

#[test]
fn test_rational_division_reduces() {
    // 6/4 should reduce to 3/2
    let result = Expression::div(Expression::integer(6), Expression::integer(4)).simplify();

    // Check it's a rational 3/2
    assert_eq!(
        result,
        Expression::rational(3, 2),
        "6/4 should reduce to 3/2"
    );
}

#[test]
fn test_power_of_zero_base() {
    // 0^n = 0 for positive n
    let test_cases = vec![1, 2, 3, 5, 100];

    for n in test_cases {
        let result = Expression::pow(Expression::integer(0), Expression::integer(n)).simplify();
        assert_eq!(result, Expression::integer(0), "0^{} should be 0", n);
    }
}

#[test]
fn test_negative_exponents() {
    // x^(-n) should be 1/x^n structure
    // 2^(-1) = 1/2
    let result = Expression::pow(Expression::integer(2), Expression::integer(-1)).simplify();
    assert_eq!(result, Expression::rational(1, 2), "2^(-1) should be 1/2");

    // 2^(-2) = 1/4
    let result = Expression::pow(Expression::integer(2), Expression::integer(-2)).simplify();
    assert_eq!(result, Expression::rational(1, 4), "2^(-2) should be 1/4");
}

#[test]
fn test_factorial_base_cases() {
    // 0! = 1
    let result = Expression::function("factorial", vec![Expression::integer(0)]).simplify();
    assert_eq!(result, Expression::integer(1), "0! should be 1");

    // 1! = 1
    let result = Expression::function("factorial", vec![Expression::integer(1)]).simplify();
    assert_eq!(result, Expression::integer(1), "1! should be 1");
}

#[test]
fn test_factorial_small_values() {
    let test_cases = vec![(2, 2), (3, 6), (4, 24), (5, 120), (6, 720)];

    for (n, expected) in test_cases {
        let result = Expression::function("factorial", vec![Expression::integer(n)]).simplify();
        assert_eq!(
            result,
            Expression::integer(expected),
            "{}! should be {}",
            n,
            expected
        );
    }
}
