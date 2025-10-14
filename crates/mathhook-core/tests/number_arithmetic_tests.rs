//! Comprehensive tests for Number arithmetic with overflow handling

use mathhook_core::{MathError, Number};
use num_bigint::BigInt;
use num_rational::BigRational;

#[test]
fn test_number_type_size_is_16_bytes() {
    assert_eq!(
        std::mem::size_of::<Number>(),
        16,
        "Number type must be exactly 16 bytes"
    );
}

#[test]
fn test_integer_addition_basic() {
    let a = Number::integer(5);
    let b = Number::integer(3);
    let result = (a + b).unwrap();
    assert_eq!(result, Number::integer(8));
}

#[test]
fn test_integer_addition_overflow_promotes_to_bigint() {
    let a = Number::integer(i64::MAX);
    let b = Number::integer(1);
    let result = (a + b).unwrap();

    match result {
        Number::BigInteger(n) => {
            assert_eq!(*n, BigInt::from(i64::MAX) + BigInt::from(1));
        }
        _ => panic!("Expected BigInteger promotion on overflow"),
    }
}

#[test]
fn test_integer_subtraction_basic() {
    let a = Number::integer(10);
    let b = Number::integer(3);
    let result = (a - b).unwrap();
    assert_eq!(result, Number::integer(7));
}

#[test]
fn test_integer_subtraction_underflow_promotes_to_bigint() {
    let a = Number::integer(i64::MIN);
    let b = Number::integer(1);
    let result = (a - b).unwrap();

    match result {
        Number::BigInteger(n) => {
            assert_eq!(*n, BigInt::from(i64::MIN) - BigInt::from(1));
        }
        _ => panic!("Expected BigInteger promotion on underflow"),
    }
}

#[test]
fn test_integer_multiplication_basic() {
    let a = Number::integer(6);
    let b = Number::integer(7);
    let result = (a * b).unwrap();
    assert_eq!(result, Number::integer(42));
}

#[test]
fn test_integer_multiplication_overflow_promotes_to_bigint() {
    let a = Number::integer(i64::MAX / 2);
    let b = Number::integer(3);
    let result = (a * b).unwrap();

    match result {
        Number::BigInteger(_) => {}
        _ => panic!("Expected BigInteger promotion on multiplication overflow"),
    }
}

#[test]
fn test_division_exact() {
    let a = Number::integer(10);
    let b = Number::integer(2);
    let result = (a / b).unwrap();
    assert_eq!(result, Number::integer(5));
}

#[test]
fn test_division_creates_rational() {
    let a = Number::integer(6);
    let b = Number::integer(4);
    let result = (a / b).unwrap();

    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(3));
            assert_eq!(r.denom(), &BigInt::from(2));
        }
        _ => panic!("Expected rational 3/2"),
    }
}

#[test]
fn test_division_by_zero_returns_error() {
    let a = Number::integer(5);
    let b = Number::integer(0);
    let result = a / b;

    assert!(result.is_err());
    match result {
        Err(MathError::DivisionByZero) => {}
        _ => panic!("Expected DivisionByZero error"),
    }
}

#[test]
fn test_rational_addition() {
    let a = Number::Rational(Box::new(BigRational::new(BigInt::from(1), BigInt::from(2))));
    let b = Number::Rational(Box::new(BigRational::new(BigInt::from(1), BigInt::from(3))));
    let result = (a + b).unwrap();

    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(5));
            assert_eq!(r.denom(), &BigInt::from(6));
        }
        _ => panic!("Expected rational 5/6"),
    }
}

#[test]
fn test_rational_multiplication() {
    let a = Number::Rational(Box::new(BigRational::new(BigInt::from(2), BigInt::from(3))));
    let b = Number::Rational(Box::new(BigRational::new(BigInt::from(3), BigInt::from(4))));
    let result = (a * b).unwrap();

    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(1));
            assert_eq!(r.denom(), &BigInt::from(2));
        }
        _ => panic!("Expected rational 1/2"),
    }
}

#[test]
fn test_rational_division() {
    let a = Number::Rational(Box::new(BigRational::new(BigInt::from(1), BigInt::from(2))));
    let b = Number::Rational(Box::new(BigRational::new(BigInt::from(1), BigInt::from(4))));
    let result = (a / b).unwrap();

    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(2));
            assert_eq!(r.denom(), &BigInt::from(1));
        }
        _ => panic!("Expected rational 2/1"),
    }
}

#[test]
fn test_float_addition() {
    let a = Number::float(3.5);
    let b = Number::float(2.5);
    let result = (a + b).unwrap();

    match result {
        Number::Float(f) => {
            assert!((f - 6.0).abs() < 1e-10);
        }
        _ => panic!("Expected float"),
    }
}

#[test]
fn test_float_multiplication() {
    let a = Number::float(2.5);
    let b = Number::float(4.0);
    let result = (a * b).unwrap();

    match result {
        Number::Float(f) => {
            assert!((f - 10.0).abs() < 1e-10);
        }
        _ => panic!("Expected float"),
    }
}

#[test]
fn test_float_division() {
    let a = Number::float(10.0);
    let b = Number::float(4.0);
    let result = (a / b).unwrap();

    match result {
        Number::Float(f) => {
            assert!((f - 2.5).abs() < 1e-10);
        }
        _ => panic!("Expected float"),
    }
}

#[test]
fn test_mixed_integer_float_addition() {
    let a = Number::integer(5);
    let b = Number::float(3.5);
    let result = (a + b).unwrap();

    match result {
        Number::Float(f) => {
            assert!((f - 8.5).abs() < 1e-10);
        }
        _ => panic!("Expected float"),
    }
}

#[test]
fn test_mixed_integer_rational_addition() {
    let a = Number::integer(2);
    let b = Number::Rational(Box::new(BigRational::new(BigInt::from(1), BigInt::from(2))));
    let result = (a + b).unwrap();

    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(5));
            assert_eq!(r.denom(), &BigInt::from(2));
        }
        _ => panic!("Expected rational 5/2"),
    }
}

#[test]
fn test_mixed_bigint_integer_addition() {
    let a = Number::BigInteger(Box::new(BigInt::from(100)));
    let b = Number::integer(50);
    let result = (a + b).unwrap();

    match result {
        Number::BigInteger(n) => {
            assert_eq!(*n, BigInt::from(150));
        }
        _ => panic!("Expected BigInteger"),
    }
}

#[test]
fn test_bigint_addition() {
    let a = Number::BigInteger(Box::new(BigInt::from(1000)));
    let b = Number::BigInteger(Box::new(BigInt::from(2000)));
    let result = (a + b).unwrap();

    match result {
        Number::BigInteger(n) => {
            assert_eq!(*n, BigInt::from(3000));
        }
        _ => panic!("Expected BigInteger"),
    }
}

#[test]
fn test_bigint_multiplication() {
    let a = Number::BigInteger(Box::new(BigInt::from(1000)));
    let b = Number::BigInteger(Box::new(BigInt::from(2000)));
    let result = (a * b).unwrap();

    match result {
        Number::BigInteger(n) => {
            assert_eq!(*n, BigInt::from(2000000));
        }
        _ => panic!("Expected BigInteger"),
    }
}

#[test]
fn test_is_zero_integer() {
    let zero = Number::integer(0);
    assert!(zero.is_zero());

    let non_zero = Number::integer(5);
    assert!(!non_zero.is_zero());
}

#[test]
fn test_is_zero_float() {
    let zero = Number::float(0.0);
    assert!(zero.is_zero());

    let non_zero = Number::float(1.5);
    assert!(!non_zero.is_zero());
}

#[test]
fn test_is_zero_rational() {
    let zero = Number::Rational(Box::new(BigRational::new(BigInt::from(0), BigInt::from(1))));
    assert!(zero.is_zero());

    let non_zero = Number::Rational(Box::new(BigRational::new(BigInt::from(1), BigInt::from(2))));
    assert!(!non_zero.is_zero());
}

#[test]
fn test_is_zero_biginteger() {
    let zero = Number::BigInteger(Box::new(BigInt::from(0)));
    assert!(zero.is_zero());

    let non_zero = Number::BigInteger(Box::new(BigInt::from(42)));
    assert!(!non_zero.is_zero());
}

#[test]
fn test_commutative_addition() {
    let a = Number::integer(5);
    let b = Number::integer(3);

    let result1 = (a.clone() + b.clone()).unwrap();
    let result2 = (b + a).unwrap();

    assert_eq!(result1, result2);
}

#[test]
fn test_commutative_multiplication() {
    let a = Number::integer(6);
    let b = Number::integer(7);

    let result1 = (a.clone() * b.clone()).unwrap();
    let result2 = (b * a).unwrap();

    assert_eq!(result1, result2);
}

#[test]
fn test_distributive_property() {
    let a = Number::integer(2);
    let b = Number::integer(3);
    let c = Number::integer(4);

    let left = (a.clone() * (b.clone() + c.clone()).unwrap()).unwrap();

    let term1 = (a.clone() * b).unwrap();
    let term2 = (a * c).unwrap();
    let right = (term1 + term2).unwrap();

    assert_eq!(left, right);
}

#[test]
fn test_integer_max_overflow_addition() {
    let a = Number::integer(i64::MAX);
    let b = Number::integer(i64::MAX);
    let result = (a + b).unwrap();

    match result {
        Number::BigInteger(n) => {
            assert_eq!(*n, BigInt::from(i64::MAX) + BigInt::from(i64::MAX));
        }
        _ => panic!("Expected BigInteger promotion on large overflow"),
    }
}

#[test]
fn test_integer_min_underflow_subtraction() {
    let a = Number::integer(i64::MIN);
    let b = Number::integer(i64::MAX);
    let result = (a - b).unwrap();

    match result {
        Number::BigInteger(n) => {
            assert_eq!(*n, BigInt::from(i64::MIN) - BigInt::from(i64::MAX));
        }
        _ => panic!("Expected BigInteger promotion on large underflow"),
    }
}

#[test]
fn test_integer_max_squared_overflow() {
    let a = Number::integer(i64::MAX);
    let result = (a.clone() * a).unwrap();

    match result {
        Number::BigInteger(_) => {}
        _ => panic!("Expected BigInteger promotion on MAX squared"),
    }
}

#[test]
fn test_no_overflow_stays_integer() {
    let a = Number::integer(100);
    let b = Number::integer(200);
    let result = (a + b).unwrap();

    match result {
        Number::Integer(300) => {}
        _ => panic!("Should stay as Integer when no overflow"),
    }
}

#[test]
fn test_mixed_bigint_rational_multiplication() {
    let a = Number::BigInteger(Box::new(BigInt::from(12)));
    let b = Number::Rational(Box::new(BigRational::new(BigInt::from(2), BigInt::from(3))));
    let result = (a * b).unwrap();

    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(8));
            assert_eq!(r.denom(), &BigInt::from(1));
        }
        _ => panic!("Expected rational result"),
    }
}

#[test]
fn test_mixed_bigint_rational_division() {
    let a = Number::BigInteger(Box::new(BigInt::from(10)));
    let b = Number::Rational(Box::new(BigRational::new(BigInt::from(2), BigInt::from(5))));
    let result = (a / b).unwrap();

    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(25));
            assert_eq!(r.denom(), &BigInt::from(1));
        }
        _ => panic!("Expected rational result"),
    }
}

#[test]
fn test_mixed_float_rational_addition() {
    let a = Number::Float(2.5);
    let b = Number::Rational(Box::new(BigRational::new(BigInt::from(1), BigInt::from(2))));
    let result = (a + b).unwrap();

    match result {
        Number::Float(f) => {
            assert!((f - 3.0).abs() < 1e-10);
        }
        _ => panic!("Expected float result"),
    }
}

#[test]
fn test_mixed_bigint_float_multiplication() {
    let a = Number::BigInteger(Box::new(BigInt::from(5)));
    let b = Number::Float(2.5);
    let result = (a * b).unwrap();

    match result {
        Number::Float(f) => {
            assert!((f - 12.5).abs() < 1e-10);
        }
        _ => panic!("Expected float result"),
    }
}

#[test]
fn test_rational_subtraction() {
    let a = Number::Rational(Box::new(BigRational::new(BigInt::from(3), BigInt::from(4))));
    let b = Number::Rational(Box::new(BigRational::new(BigInt::from(1), BigInt::from(4))));
    let result = (a - b).unwrap();

    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(1));
            assert_eq!(r.denom(), &BigInt::from(2));
        }
        _ => panic!("Expected rational 1/2"),
    }
}

#[test]
fn test_mixed_integer_rational_subtraction() {
    let a = Number::integer(5);
    let b = Number::Rational(Box::new(BigRational::new(BigInt::from(3), BigInt::from(2))));
    let result = (a - b).unwrap();

    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(7));
            assert_eq!(r.denom(), &BigInt::from(2));
        }
        _ => panic!("Expected rational 7/2"),
    }
}

#[test]
fn test_mixed_integer_rational_multiplication() {
    let a = Number::integer(6);
    let b = Number::Rational(Box::new(BigRational::new(BigInt::from(2), BigInt::from(3))));
    let result = (a * b).unwrap();

    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(4));
            assert_eq!(r.denom(), &BigInt::from(1));
        }
        _ => panic!("Expected rational 4/1"),
    }
}

#[test]
fn test_mixed_integer_rational_division() {
    let a = Number::integer(10);
    let b = Number::Rational(Box::new(BigRational::new(BigInt::from(5), BigInt::from(2))));
    let result = (a / b).unwrap();

    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(4));
            assert_eq!(r.denom(), &BigInt::from(1));
        }
        _ => panic!("Expected rational 4/1"),
    }
}

#[test]
fn test_bigint_subtraction() {
    let a = Number::BigInteger(Box::new(BigInt::from(5000)));
    let b = Number::BigInteger(Box::new(BigInt::from(3000)));
    let result = (a - b).unwrap();

    match result {
        Number::BigInteger(n) => {
            assert_eq!(*n, BigInt::from(2000));
        }
        _ => panic!("Expected BigInteger"),
    }
}

#[test]
fn test_bigint_division_exact() {
    let a = Number::BigInteger(Box::new(BigInt::from(100)));
    let b = Number::BigInteger(Box::new(BigInt::from(10)));
    let result = (a / b).unwrap();

    match result {
        Number::BigInteger(n) => {
            assert_eq!(*n, BigInt::from(10));
        }
        _ => panic!("Expected BigInteger"),
    }
}

#[test]
fn test_bigint_division_creates_rational() {
    let a = Number::BigInteger(Box::new(BigInt::from(10)));
    let b = Number::BigInteger(Box::new(BigInt::from(3)));
    let result = (a / b).unwrap();

    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(10));
            assert_eq!(r.denom(), &BigInt::from(3));
        }
        _ => panic!("Expected rational 10/3"),
    }
}

#[test]
fn test_float_subtraction() {
    let a = Number::float(10.5);
    let b = Number::float(3.5);
    let result = (a - b).unwrap();

    match result {
        Number::Float(f) => {
            assert!((f - 7.0).abs() < 1e-10);
        }
        _ => panic!("Expected float"),
    }
}

#[test]
fn test_associative_addition() {
    let a = Number::integer(2);
    let b = Number::integer(3);
    let c = Number::integer(4);

    let left = ((a.clone() + b.clone()).unwrap() + c.clone()).unwrap();
    let right = (a + (b + c).unwrap()).unwrap();

    assert_eq!(left, right);
}

#[test]
fn test_associative_multiplication() {
    let a = Number::integer(2);
    let b = Number::integer(3);
    let c = Number::integer(4);

    let left = ((a.clone() * b.clone()).unwrap() * c.clone()).unwrap();
    let right = (a * (b * c).unwrap()).unwrap();

    assert_eq!(left, right);
}

#[test]
fn test_float_addition_overflow() {
    let a = Number::float(f64::MAX);
    let b = Number::float(f64::MAX);
    let result = a + b;

    assert!(result.is_err());
    match result {
        Err(MathError::NumericOverflow { .. }) => {}
        _ => panic!("Expected NumericOverflow error"),
    }
}

#[test]
fn test_float_multiplication_overflow() {
    let a = Number::float(f64::MAX);
    let b = Number::float(2.0);
    let result = a * b;

    assert!(result.is_err());
    match result {
        Err(MathError::NumericOverflow { .. }) => {}
        _ => panic!("Expected NumericOverflow error"),
    }
}

#[test]
fn test_float_division_by_zero_overflow() {
    let a = Number::float(1.0);
    let b = Number::float(0.0);
    let result = a / b;

    assert!(result.is_err());
    match result {
        Err(MathError::DivisionByZero) | Err(MathError::NumericOverflow { .. }) => {}
        _ => panic!("Expected DivisionByZero or NumericOverflow error"),
    }
}

#[test]
fn test_float_nan_detection() {
    let a = Number::float(0.0);
    let b = Number::float(0.0);
    let result = a / b;

    assert!(result.is_err());
    match result {
        Err(MathError::DivisionByZero) | Err(MathError::NumericOverflow { .. }) => {}
        _ => panic!("Expected DivisionByZero or NumericOverflow error for NaN"),
    }
}

#[test]
fn test_float_subtraction_no_overflow() {
    let a = Number::float(f64::MAX);
    let b = Number::float(f64::MAX);
    let result = (a - b).unwrap();

    match result {
        Number::Float(f) => {
            assert_eq!(f, 0.0);
        }
        _ => panic!("Expected float 0.0"),
    }
}

#[test]
fn test_bigint_to_float_conversion_overflow() {
    use num_bigint::BigInt;

    let huge_bigint = Number::BigInteger(Box::new(BigInt::parse_bytes(b"999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999", 10).unwrap()));
    let f = Number::float(1.0);
    let result = huge_bigint + f;

    assert!(result.is_err());
    match result {
        Err(MathError::NumericOverflow { .. }) => {}
        _ => panic!("Expected NumericOverflow for BigInt to float conversion"),
    }
}

#[test]
fn test_integer_float_mixed_no_overflow() {
    let a = Number::integer(100);
    let b = Number::float(2.5);
    let result = (a * b).unwrap();

    match result {
        Number::Float(f) => {
            assert!((f - 250.0).abs() < 1e-10);
        }
        _ => panic!("Expected float 250.0"),
    }
}

#[test]
fn test_rational_float_mixed_operations() {
    use num_bigint::BigInt;
    use num_rational::BigRational;

    let a = Number::Rational(Box::new(BigRational::new(BigInt::from(1), BigInt::from(2))));
    let b = Number::float(0.5);
    let result = (a + b).unwrap();

    match result {
        Number::Float(f) => {
            assert!((f - 1.0).abs() < 1e-10);
        }
        _ => panic!("Expected float 1.0"),
    }
}

#[test]
fn test_power_basic() {
    let base = Number::integer(2);
    let exp = Number::integer(3);
    let result = base.pow(&exp).unwrap();
    assert_eq!(result, Number::integer(8));
}

#[test]
fn test_power_overflow_promotes_to_bigint() {
    let base = Number::integer(2);
    let exp = Number::integer(63);
    let result = base.pow(&exp).unwrap();

    match result {
        Number::BigInteger(_) | Number::Integer(_) => {}
        _ => panic!("Expected Integer or BigInteger"),
    }
}

#[test]
fn test_power_zero_exponent() {
    let base = Number::integer(5);
    let exp = Number::integer(0);
    let result = base.pow(&exp).unwrap();
    assert_eq!(result, Number::integer(1));
}

#[test]
fn test_power_float() {
    let base = Number::float(2.0);
    let exp = Number::float(0.5);
    let result = base.pow(&exp).unwrap();

    match result {
        Number::Float(f) => {
            assert!((f - 1.414213562373095).abs() < 1e-10);
        }
        _ => panic!("Expected float"),
    }
}

#[test]
fn test_power_float_overflow() {
    let base = Number::float(f64::MAX);
    let exp = Number::float(2.0);
    let result = base.pow(&exp);

    assert!(result.is_err());
    match result {
        Err(MathError::NumericOverflow { .. }) => {}
        _ => panic!("Expected NumericOverflow"),
    }
}

#[test]
fn test_negation_integer() {
    let a = Number::integer(5);
    let result = (-a).unwrap();
    assert_eq!(result, Number::integer(-5));
}

#[test]
fn test_negation_integer_min_promotes_to_bigint() {
    let a = Number::integer(i64::MIN);
    let result = (-a).unwrap();

    match result {
        Number::BigInteger(n) => {
            assert_eq!(*n, -BigInt::from(i64::MIN));
        }
        _ => panic!("Expected BigInteger promotion on MIN negation"),
    }
}

#[test]
fn test_negation_float() {
    let a = Number::float(3.14);
    let result = (-a).unwrap();

    match result {
        Number::Float(f) => {
            assert_eq!(f, -3.14);
        }
        _ => panic!("Expected float"),
    }
}

#[test]
fn test_negation_rational() {
    use num_bigint::BigInt;
    use num_rational::BigRational;

    let a = Number::Rational(Box::new(BigRational::new(BigInt::from(3), BigInt::from(4))));
    let result = (-a).unwrap();

    match result {
        Number::Rational(r) => {
            assert_eq!(r.numer(), &BigInt::from(-3));
            assert_eq!(r.denom(), &BigInt::from(4));
        }
        _ => panic!("Expected rational -3/4"),
    }
}

#[test]
fn test_negation_bigint() {
    use num_bigint::BigInt;

    let a = Number::BigInteger(Box::new(BigInt::from(1000)));
    let result = (-a).unwrap();

    match result {
        Number::BigInteger(n) => {
            assert_eq!(*n, BigInt::from(-1000));
        }
        _ => panic!("Expected BigInteger -1000"),
    }
}
