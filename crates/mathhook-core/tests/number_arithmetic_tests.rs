//! Comprehensive tests for Number arithmetic with overflow handling

use mathhook_core::{MathError, Number};
use num_bigint::BigInt;
use num_rational::BigRational;

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
