//! Basic arithmetic operations with overflow checking
//!
//! Implements Add, Sub, Mul, Div, and Neg traits for Number type.
//! Uses checked arithmetic to detect overflow and promotes to BigInt or Rational when needed.
//! All float operations check for infinity and NaN.

use super::types::Number;
use crate::error::MathError;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::ToPrimitive;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Addition with overflow checking and promotion to BigInt
///
/// # Examples
///
/// ```rust
/// use mathhook_core::Number;
///
/// let a = Number::integer(5);
/// let b = Number::integer(3);
/// let result = (a + b).unwrap();
/// assert_eq!(result, Number::integer(8));
/// ```
impl Add for Number {
    type Output = Result<Number, MathError>;

    fn add(self, other: Number) -> Result<Number, MathError> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => match a.checked_add(b) {
                Some(result) => Ok(Number::Integer(result)),
                None => Ok(Number::BigInteger(Box::new(
                    BigInt::from(a) + BigInt::from(b),
                ))),
            },

            (Number::BigInteger(a), Number::BigInteger(b)) => {
                Ok(Number::BigInteger(Box::new(*a + *b)))
            }

            (Number::Integer(i), Number::BigInteger(bi))
            | (Number::BigInteger(bi), Number::Integer(i)) => {
                Ok(Number::BigInteger(Box::new(*bi + BigInt::from(i))))
            }

            (Number::Rational(a), Number::Rational(b)) => Ok(Number::Rational(Box::new(*a + *b))),

            (Number::Integer(i), Number::Rational(r))
            | (Number::Rational(r), Number::Integer(i)) => {
                let i_rational = BigRational::from(BigInt::from(i));
                Ok(Number::Rational(Box::new(i_rational + *r)))
            }

            (Number::BigInteger(bi), Number::Rational(r))
            | (Number::Rational(r), Number::BigInteger(bi)) => {
                let bi_rational = BigRational::from(*bi);
                Ok(Number::Rational(Box::new(bi_rational + *r)))
            }

            (Number::Float(a), Number::Float(b)) => {
                let result = a + b;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float addition".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Integer(i), Number::Float(f)) | (Number::Float(f), Number::Integer(i)) => {
                let result = i as f64 + f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "integer-float addition".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::BigInteger(bi), Number::Float(f))
            | (Number::Float(f), Number::BigInteger(bi)) => {
                let bi_float = bi.to_f64().ok_or_else(|| MathError::NumericOverflow {
                    operation: "BigInteger to float conversion".to_owned(),
                })?;
                let result = bi_float + f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "BigInteger-float addition".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Rational(r), Number::Float(f)) | (Number::Float(f), Number::Rational(r)) => {
                let numer_float = r
                    .numer()
                    .to_f64()
                    .ok_or_else(|| MathError::NumericOverflow {
                        operation: "Rational numerator to float conversion".to_owned(),
                    })?;
                let denom_float = r
                    .denom()
                    .to_f64()
                    .ok_or_else(|| MathError::NumericOverflow {
                        operation: "Rational denominator to float conversion".to_owned(),
                    })?;
                let r_float = numer_float / denom_float;
                let result = r_float + f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "Rational-float addition".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }
        }
    }
}

/// Subtraction with overflow checking and promotion to BigInt
///
/// # Examples
///
/// ```rust
/// use mathhook_core::Number;
///
/// let a = Number::integer(10);
/// let b = Number::integer(3);
/// let result = (a - b).unwrap();
/// assert_eq!(result, Number::integer(7));
/// ```
impl Sub for Number {
    type Output = Result<Number, MathError>;

    fn sub(self, other: Number) -> Result<Number, MathError> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => match a.checked_sub(b) {
                Some(result) => Ok(Number::Integer(result)),
                None => Ok(Number::BigInteger(Box::new(
                    BigInt::from(a) - BigInt::from(b),
                ))),
            },

            (Number::BigInteger(a), Number::BigInteger(b)) => {
                Ok(Number::BigInteger(Box::new(*a - *b)))
            }

            (Number::Integer(i), Number::BigInteger(bi)) => {
                Ok(Number::BigInteger(Box::new(BigInt::from(i) - *bi)))
            }

            (Number::BigInteger(bi), Number::Integer(i)) => {
                Ok(Number::BigInteger(Box::new(*bi - BigInt::from(i))))
            }

            (Number::Rational(a), Number::Rational(b)) => Ok(Number::Rational(Box::new(*a - *b))),

            (Number::Integer(i), Number::Rational(r)) => {
                let i_rational = BigRational::from(BigInt::from(i));
                Ok(Number::Rational(Box::new(i_rational - *r)))
            }

            (Number::Rational(r), Number::Integer(i)) => {
                let i_rational = BigRational::from(BigInt::from(i));
                Ok(Number::Rational(Box::new(*r - i_rational)))
            }

            (Number::BigInteger(bi), Number::Rational(r)) => {
                let bi_rational = BigRational::from(*bi);
                Ok(Number::Rational(Box::new(bi_rational - *r)))
            }

            (Number::Rational(r), Number::BigInteger(bi)) => {
                let bi_rational = BigRational::from(*bi);
                Ok(Number::Rational(Box::new(*r - bi_rational)))
            }

            (Number::Float(a), Number::Float(b)) => {
                let result = a - b;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float subtraction".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Integer(i), Number::Float(f)) => {
                let result = i as f64 - f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "integer-float subtraction".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Float(f), Number::Integer(i)) => {
                let result = f - i as f64;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float-integer subtraction".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::BigInteger(bi), Number::Float(f)) => {
                let bi_float = bi.to_f64().ok_or_else(|| MathError::NumericOverflow {
                    operation: "BigInteger to float conversion".to_owned(),
                })?;
                let result = bi_float - f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "BigInteger-float subtraction".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Float(f), Number::BigInteger(bi)) => {
                let bi_float = bi.to_f64().ok_or_else(|| MathError::NumericOverflow {
                    operation: "BigInteger to float conversion".to_owned(),
                })?;
                let result = f - bi_float;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float-BigInteger subtraction".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Rational(r), Number::Float(f)) => {
                let numer_float = r
                    .numer()
                    .to_f64()
                    .ok_or_else(|| MathError::NumericOverflow {
                        operation: "Rational numerator to float conversion".to_owned(),
                    })?;
                let denom_float = r
                    .denom()
                    .to_f64()
                    .ok_or_else(|| MathError::NumericOverflow {
                        operation: "Rational denominator to float conversion".to_owned(),
                    })?;
                let r_float = numer_float / denom_float;
                let result = r_float - f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "Rational-float subtraction".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Float(f), Number::Rational(r)) => {
                let numer_float = r
                    .numer()
                    .to_f64()
                    .ok_or_else(|| MathError::NumericOverflow {
                        operation: "Rational numerator to float conversion".to_owned(),
                    })?;
                let denom_float = r
                    .denom()
                    .to_f64()
                    .ok_or_else(|| MathError::NumericOverflow {
                        operation: "Rational denominator to float conversion".to_owned(),
                    })?;
                let r_float = numer_float / denom_float;
                let result = f - r_float;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float-Rational subtraction".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }
        }
    }
}

/// Multiplication with overflow checking and promotion to BigInt
///
/// # Examples
///
/// ```rust
/// use mathhook_core::Number;
///
/// let a = Number::integer(6);
/// let b = Number::integer(7);
/// let result = (a * b).unwrap();
/// assert_eq!(result, Number::integer(42));
/// ```
impl Mul for Number {
    type Output = Result<Number, MathError>;

    fn mul(self, other: Number) -> Result<Number, MathError> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => match a.checked_mul(b) {
                Some(result) => Ok(Number::Integer(result)),
                None => Ok(Number::BigInteger(Box::new(
                    BigInt::from(a) * BigInt::from(b),
                ))),
            },

            (Number::BigInteger(a), Number::BigInteger(b)) => {
                Ok(Number::BigInteger(Box::new(*a * *b)))
            }

            (Number::Integer(i), Number::BigInteger(bi))
            | (Number::BigInteger(bi), Number::Integer(i)) => {
                Ok(Number::BigInteger(Box::new(*bi * BigInt::from(i))))
            }

            (Number::Rational(a), Number::Rational(b)) => Ok(Number::Rational(Box::new(*a * *b))),

            (Number::Integer(i), Number::Rational(r))
            | (Number::Rational(r), Number::Integer(i)) => {
                let i_rational = BigRational::from(BigInt::from(i));
                Ok(Number::Rational(Box::new(i_rational * *r)))
            }

            (Number::BigInteger(bi), Number::Rational(r))
            | (Number::Rational(r), Number::BigInteger(bi)) => {
                let bi_rational = BigRational::from(*bi);
                Ok(Number::Rational(Box::new(bi_rational * *r)))
            }

            (Number::Float(a), Number::Float(b)) => {
                let result = a * b;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float multiplication".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Integer(i), Number::Float(f)) | (Number::Float(f), Number::Integer(i)) => {
                let result = i as f64 * f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "integer-float multiplication".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::BigInteger(bi), Number::Float(f))
            | (Number::Float(f), Number::BigInteger(bi)) => {
                let bi_float = bi.to_f64().ok_or_else(|| MathError::NumericOverflow {
                    operation: "BigInteger to float conversion".to_owned(),
                })?;
                let result = bi_float * f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "BigInteger-float multiplication".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Rational(r), Number::Float(f)) | (Number::Float(f), Number::Rational(r)) => {
                let numer_float = r
                    .numer()
                    .to_f64()
                    .ok_or_else(|| MathError::NumericOverflow {
                        operation: "Rational numerator to float conversion".to_owned(),
                    })?;
                let denom_float = r
                    .denom()
                    .to_f64()
                    .ok_or_else(|| MathError::NumericOverflow {
                        operation: "Rational denominator to float conversion".to_owned(),
                    })?;
                let r_float = numer_float / denom_float;
                let result = r_float * f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "Rational-float multiplication".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }
        }
    }
}

/// Division with division by zero check and automatic promotion to rational
///
/// # Examples
///
/// ```rust
/// use mathhook_core::Number;
///
/// let a = Number::integer(10);
/// let b = Number::integer(2);
/// let result = (a / b).unwrap();
/// assert_eq!(result, Number::integer(5));
/// ```
impl Div for Number {
    type Output = Result<Number, MathError>;

    fn div(self, other: Number) -> Result<Number, MathError> {
        if other.is_zero() {
            return Err(MathError::DivisionByZero);
        }

        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => {
                if a % b == 0 {
                    Ok(Number::Integer(a / b))
                } else {
                    Ok(Number::Rational(Box::new(BigRational::new(
                        BigInt::from(a),
                        BigInt::from(b),
                    ))))
                }
            }

            (Number::BigInteger(a), Number::BigInteger(b)) => {
                if (*a).clone() % (*b).clone() == BigInt::from(0) {
                    Ok(Number::BigInteger(Box::new(*a / *b)))
                } else {
                    Ok(Number::Rational(Box::new(BigRational::new(*a, *b))))
                }
            }

            (Number::Integer(i), Number::BigInteger(bi)) => Ok(Number::Rational(Box::new(
                BigRational::new(BigInt::from(i), *bi),
            ))),

            (Number::BigInteger(bi), Number::Integer(i)) => Ok(Number::Rational(Box::new(
                BigRational::new(*bi, BigInt::from(i)),
            ))),

            (Number::Rational(a), Number::Rational(b)) => Ok(Number::Rational(Box::new(*a / *b))),

            (Number::Integer(i), Number::Rational(r)) => {
                let i_rational = BigRational::from(BigInt::from(i));
                Ok(Number::Rational(Box::new(i_rational / *r)))
            }

            (Number::Rational(r), Number::Integer(i)) => {
                let i_rational = BigRational::from(BigInt::from(i));
                Ok(Number::Rational(Box::new(*r / i_rational)))
            }

            (Number::BigInteger(bi), Number::Rational(r)) => {
                let bi_rational = BigRational::from(*bi);
                Ok(Number::Rational(Box::new(bi_rational / *r)))
            }

            (Number::Rational(r), Number::BigInteger(bi)) => {
                let bi_rational = BigRational::from(*bi);
                Ok(Number::Rational(Box::new(*r / bi_rational)))
            }

            (Number::Float(a), Number::Float(b)) => {
                let result = a / b;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float division".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Integer(i), Number::Float(f)) => {
                let result = i as f64 / f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "integer-float division".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Float(f), Number::Integer(i)) => {
                let result = f / i as f64;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float-integer division".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::BigInteger(bi), Number::Float(f)) => {
                let bi_float = bi.to_f64().ok_or_else(|| MathError::NumericOverflow {
                    operation: "BigInteger to float conversion".to_owned(),
                })?;
                let result = bi_float / f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "BigInteger-float division".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Float(f), Number::BigInteger(bi)) => {
                let bi_float = bi.to_f64().ok_or_else(|| MathError::NumericOverflow {
                    operation: "BigInteger to float conversion".to_owned(),
                })?;
                let result = f / bi_float;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float-BigInteger division".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Rational(r), Number::Float(f)) => {
                let numer_float = r
                    .numer()
                    .to_f64()
                    .ok_or_else(|| MathError::NumericOverflow {
                        operation: "Rational numerator to float conversion".to_owned(),
                    })?;
                let denom_float = r
                    .denom()
                    .to_f64()
                    .ok_or_else(|| MathError::NumericOverflow {
                        operation: "Rational denominator to float conversion".to_owned(),
                    })?;
                let r_float = numer_float / denom_float;
                let result = r_float / f;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "Rational-float division".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }

            (Number::Float(f), Number::Rational(r)) => {
                let numer_float = r
                    .numer()
                    .to_f64()
                    .ok_or_else(|| MathError::NumericOverflow {
                        operation: "Rational numerator to float conversion".to_owned(),
                    })?;
                let denom_float = r
                    .denom()
                    .to_f64()
                    .ok_or_else(|| MathError::NumericOverflow {
                        operation: "Rational denominator to float conversion".to_owned(),
                    })?;
                let r_float = numer_float / denom_float;
                let result = f / r_float;
                if result.is_infinite() || result.is_nan() {
                    Err(MathError::NumericOverflow {
                        operation: "float-Rational division".to_owned(),
                    })
                } else {
                    Ok(Number::Float(result))
                }
            }
        }
    }
}

/// Negation with overflow checking
///
/// # Examples
///
/// ```rust
/// use mathhook_core::Number;
///
/// let a = Number::integer(5);
/// let result = (-a).unwrap();
/// assert_eq!(result, Number::integer(-5));
/// ```
impl Neg for Number {
    type Output = Result<Number, MathError>;

    fn neg(self) -> Result<Number, MathError> {
        match self {
            Number::Integer(i) => match i.checked_neg() {
                Some(result) => Ok(Number::Integer(result)),
                None => Ok(Number::BigInteger(Box::new(-BigInt::from(i)))),
            },

            Number::BigInteger(bi) => Ok(Number::BigInteger(Box::new(-*bi))),

            Number::Float(f) => Ok(Number::Float(-f)),

            Number::Rational(r) => Ok(Number::Rational(Box::new(-*r))),
        }
    }
}
