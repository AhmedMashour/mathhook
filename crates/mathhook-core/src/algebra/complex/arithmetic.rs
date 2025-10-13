//! Complex number arithmetic methods for Expression
//!
//! Provides convenience methods for working with complex numbers, including
//! extraction of real and imaginary parts, polar form conversions, and
//! simplification operations.

use crate::core::Expression;
use crate::simplify::Simplify;
use crate::expr;
use super::operations::ComplexOperations;

impl Expression {
    /// Extract the real part of a complex number
    ///
    /// Returns the real component of a complex expression. For non-complex
    /// expressions, returns the expression itself.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, expr};
    ///
    /// let z = Expression::complex(expr!(3), expr!(4));
    /// let real_part = z.real();
    /// assert_eq!(real_part, expr!(3));
    /// ```
    pub fn real(&self) -> Expression {
        match self {
            Expression::Complex(data) => data.real.clone(),
            _ => self.clone(),
        }
    }

    /// Extract the imaginary part of a complex number
    ///
    /// Returns the imaginary component of a complex expression. For non-complex
    /// expressions, returns zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, expr};
    ///
    /// let z = Expression::complex(expr!(3), expr!(4));
    /// let imag_part = z.imag();
    /// assert_eq!(imag_part, expr!(4));
    /// ```
    pub fn imag(&self) -> Expression {
        match self {
            Expression::Complex(data) => data.imag.clone(),
            _ => Expression::integer(0),
        }
    }

    /// Compute the complex conjugate
    ///
    /// Returns the complex conjugate (a + bi → a - bi). For non-complex
    /// expressions, returns the expression itself.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, expr};
    ///
    /// let z = Expression::complex(expr!(3), expr!(4));
    /// let conjugate = z.conjugate();
    /// if let Expression::Complex(data) = conjugate {
    ///     assert_eq!(data.real, expr!(3));
    ///     assert_eq!(data.imag, expr!(-4));
    /// }
    /// ```
    pub fn conjugate(&self) -> Expression {
        self.complex_conjugate()
    }

    /// Compute the absolute value (modulus) of a complex number
    ///
    /// Returns |z| = √(re² + im²). For complex numbers, this is the magnitude.
    /// For real numbers, this is the absolute value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, expr};
    ///
    /// let z = Expression::complex(expr!(3), expr!(4));
    /// let magnitude = z.abs();
    /// ```
    pub fn abs(&self) -> Expression {
        self.complex_modulus()
    }

    /// Compute the argument (phase angle) of a complex number
    ///
    /// Returns the angle θ = atan2(im, re) in radians, in the range (-π, π].
    /// This is the principal value of the argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, expr};
    ///
    /// let z = Expression::complex(expr!(1), expr!(1));
    /// let angle = z.arg();
    /// ```
    pub fn arg(&self) -> Expression {
        self.complex_argument()
    }

    /// Convert to polar form (magnitude, angle)
    ///
    /// Returns (r, θ) where z = r·e^(iθ). The angle is in radians,
    /// in the range (-π, π].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, expr};
    ///
    /// let z = Expression::complex(expr!(3), expr!(4));
    /// let (magnitude, angle) = z.to_polar();
    /// ```
    pub fn to_polar(&self) -> (Expression, Expression) {
        self.to_polar_form()
    }

    /// Create a complex number from polar form
    ///
    /// Converts polar coordinates (magnitude, angle) to rectangular form (a + bi).
    /// The angle should be in radians.
    ///
    /// # Arguments
    ///
    /// * `magnitude` - The magnitude (r) of the complex number
    /// * `angle` - The angle (θ) in radians
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, expr};
    ///
    /// let magnitude = expr!(5);
    /// let angle = Expression::pi();
    /// let z = Expression::from_polar(magnitude, angle);
    /// ```
    pub fn from_polar(magnitude: Expression, angle: Expression) -> Expression {
        Self::from_polar_form(magnitude, angle)
    }

    /// Create a complex number from polar form
    ///
    /// Converts polar coordinates (magnitude, angle) to rectangular form (a + bi).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, expr};
    ///
    /// let magnitude = expr!(5);
    /// let angle = Expression::pi();
    /// let z = Expression::from_polar_form(magnitude, angle);
    /// ```
    pub fn from_polar_form(magnitude: Expression, angle: Expression) -> Expression {
        Expression::complex(
            Expression::mul(vec![
                magnitude.clone(),
                Expression::function("cos", vec![angle.clone()]),
            ])
            .simplify(),
            Expression::mul(vec![magnitude, Expression::function("sin", vec![angle])]).simplify(),
        )
    }

    /// Simplify complex expressions by removing zero parts
    ///
    /// Converts complex numbers to their simplest form by removing zero
    /// real or imaginary components.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, expr};
    /// use mathhook_core::simplify::Simplify;
    ///
    /// let z = Expression::complex(expr!(3), expr!(0));
    /// let simplified = z.simplify();
    /// ```
    pub fn simplify_complex(expr: &Expression) -> Expression {
        match expr {
            Expression::Complex(data) => {
                let real_simplified = data.real.simplify();
                let imag_simplified = data.imag.simplify();

                if imag_simplified.is_zero() {
                    return real_simplified;
                }

                if real_simplified.is_zero() {
                    return Expression::mul(vec![imag_simplified, Expression::i()]).simplify();
                }

                Expression::complex(real_simplified, imag_simplified)
            }
            _ => expr.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_complex_addition() {
        let z1 = Expression::complex(expr!(3), expr!(4));
        let z2 = Expression::complex(expr!(1), expr!(2));
        let result = z1.complex_add(&z2);

        if let Expression::Complex(data) = result {
            assert_eq!(data.real, expr!(4));
            assert_eq!(data.imag, expr!(6));
        } else {
            panic!("Expected complex result");
        }
    }

    #[test]
    fn test_complex_subtraction() {
        let z1 = Expression::complex(expr!(5), expr!(7));
        let z2 = Expression::complex(expr!(2), expr!(3));
        let result = z1.complex_subtract(&z2);

        if let Expression::Complex(data) = result {
            assert_eq!(data.real, expr!(3));
            assert_eq!(data.imag, expr!(4));
        } else {
            panic!("Expected complex result");
        }
    }

    #[test]
    fn test_complex_multiplication() {
        let z1 = Expression::complex(expr!(3), expr!(4));
        let z2 = Expression::complex(expr!(1), expr!(2));
        let result = z1.complex_multiply(&z2);

        if let Expression::Complex(data) = result {
            assert_eq!(data.real, expr!(-5));
            assert_eq!(data.imag, expr!(10));
        } else {
            panic!("Expected complex result");
        }
    }

    #[test]
    fn test_complex_division() {
        let z1 = Expression::complex(expr!(2), expr!(3));
        let z2 = Expression::complex(expr!(1), expr!(-1));
        let result = z1.complex_divide(&z2);

        if let Expression::Complex(_) = result {
        } else {
            panic!("Expected complex result");
        }
    }

    #[test]
    fn test_complex_conjugate() {
        let z = Expression::complex(expr!(3), expr!(4));
        let result = z.complex_conjugate();

        if let Expression::Complex(data) = result {
            assert_eq!(data.real, expr!(3));
            assert_eq!(data.imag, expr!(-4));
        } else {
            panic!("Expected complex result");
        }
    }

    #[test]
    fn test_real_method() {
        let z = Expression::complex(expr!(3), expr!(4));
        let real_part = z.real();
        assert_eq!(real_part, expr!(3));

        let real_num = expr!(5);
        let real_part = real_num.real();
        assert_eq!(real_part, expr!(5));
    }

    #[test]
    fn test_imag_method() {
        let z = Expression::complex(expr!(3), expr!(4));
        let imag_part = z.imag();
        assert_eq!(imag_part, expr!(4));

        let real_num = expr!(5);
        let imag_part = real_num.imag();
        assert_eq!(imag_part, expr!(0));
    }

    #[test]
    fn test_conjugate_method() {
        let z = Expression::complex(expr!(3), expr!(4));
        let conjugate = z.conjugate();

        if let Expression::Complex(data) = conjugate {
            assert_eq!(data.real, expr!(3));
            assert_eq!(data.imag, expr!(-4));
        } else {
            panic!("Expected complex result");
        }
    }

    #[test]
    fn test_abs_method() {
        let z = Expression::complex(expr!(3), expr!(4));
        let magnitude = z.abs();

        match magnitude {
            Expression::Function { .. } => {}
            _ => panic!("Expected function expression for abs"),
        }
    }

    #[test]
    fn test_arg_method() {
        let z = Expression::complex(expr!(1), expr!(1));
        let angle = z.arg();

        match angle {
            Expression::Function { .. } => {}
            _ => panic!("Expected function expression for arg"),
        }
    }

    #[test]
    fn test_to_polar_method() {
        let z = Expression::complex(expr!(3), expr!(4));
        let (_magnitude, _angle) = z.to_polar();
    }

    #[test]
    fn test_from_polar_method() {
        let magnitude = expr!(5);
        let angle = expr!(0);
        let _z = Expression::from_polar(magnitude, angle);
    }

    #[test]
    fn test_complex_with_symbols() {
        let x = Expression::symbol(symbol!(x));
        let y = Expression::symbol(symbol!(y));
        let a = Expression::symbol(symbol!(a));
        let b = Expression::symbol(symbol!(b));

        let z1 = Expression::complex(x.clone(), y.clone());
        let z2 = Expression::complex(a.clone(), b.clone());
        let result = z1.complex_add(&z2);

        if let Expression::Complex(data) = result {
            match (&data.real, &data.imag) {
                (Expression::Add(real_terms), Expression::Add(imag_terms)) => {
                    assert_eq!(real_terms.len(), 2);
                    assert_eq!(imag_terms.len(), 2);
                }
                _ => panic!("Expected addition expressions for real and imaginary parts"),
            }
        } else {
            panic!("Expected complex result");
        }
    }

    #[test]
    fn test_simplify_complex() {
        let z = Expression::complex(expr!(3), expr!(0));
        let result = Expression::simplify_complex(&z);
        assert_eq!(result, expr!(3));

        let z = Expression::complex(expr!(0), expr!(4));
        let result = Expression::simplify_complex(&z);
        assert_eq!(
            result,
            Expression::mul(vec![expr!(4), Expression::i()])
        );
    }

    #[test]
    fn test_complex_zero() {
        let z = Expression::complex(expr!(0), expr!(0));
        let real_part = z.real();
        let imag_part = z.imag();
        assert_eq!(real_part, expr!(0));
        assert_eq!(imag_part, expr!(0));
    }

    #[test]
    fn test_complex_pure_real() {
        let z = Expression::complex(expr!(5), expr!(0));
        assert!(z.is_real());
        assert!(!z.is_pure_imaginary());
    }

    #[test]
    fn test_complex_pure_imaginary() {
        let z = Expression::complex(expr!(0), expr!(5));
        assert!(!z.is_real());
        assert!(z.is_pure_imaginary());
    }

    #[test]
    fn test_complex_general() {
        let z = Expression::complex(expr!(3), expr!(4));
        assert!(!z.is_real());
        assert!(z.is_imaginary());
        assert!(!z.is_pure_imaginary());
    }

    #[test]
    fn test_complex_multiplication_zero() {
        let z1 = Expression::complex(expr!(3), expr!(4));
        let z2 = Expression::complex(expr!(0), expr!(0));
        let result = z1.complex_multiply(&z2);

        if let Expression::Complex(data) = result {
            assert_eq!(data.real, expr!(0));
            assert_eq!(data.imag, expr!(0));
        } else {
            panic!("Expected complex result");
        }
    }

    #[test]
    fn test_complex_addition_negative() {
        let z1 = Expression::complex(expr!(-2), expr!(-3));
        let z2 = Expression::complex(expr!(5), expr!(7));
        let result = z1.complex_add(&z2);

        if let Expression::Complex(data) = result {
            assert_eq!(data.real, expr!(3));
            assert_eq!(data.imag, expr!(4));
        } else {
            panic!("Expected complex result");
        }
    }

    #[test]
    fn test_conjugate_twice() {
        let z = Expression::complex(expr!(3), expr!(4));
        let conjugate = z.conjugate();
        let double_conjugate = conjugate.conjugate();

        if let Expression::Complex(data) = double_conjugate {
            assert_eq!(data.real, expr!(3));
            assert_eq!(data.imag, expr!(4));
        } else {
            panic!("Expected complex result");
        }
    }

    #[test]
    fn test_complex_multiply_i() {
        let z = Expression::complex(expr!(3), expr!(4));
        let i = Expression::complex(expr!(0), expr!(1));
        let result = z.complex_multiply(&i);

        if let Expression::Complex(data) = result {
            assert_eq!(data.real, expr!(-4));
            assert_eq!(data.imag, expr!(3));
        } else {
            panic!("Expected complex result");
        }
    }

    #[test]
    fn test_from_polar_zero_angle() {
        let magnitude = expr!(5);
        let angle = expr!(0);
        let z = Expression::from_polar(magnitude, angle);

        if let Expression::Complex(_) = z {
        } else {
            panic!("Expected complex result from polar conversion");
        }
    }

    #[test]
    fn test_complex_real_extraction() {
        let real = expr!(7);
        let imag = expr!(-3);
        let z = Expression::complex(real.clone(), imag.clone());

        assert_eq!(z.real(), real);
        assert_eq!(z.imag(), imag);
    }

    #[test]
    fn test_complex_subtraction_result_zero() {
        let z1 = Expression::complex(expr!(3), expr!(4));
        let z2 = Expression::complex(expr!(3), expr!(4));
        let result = z1.complex_subtract(&z2);

        if let Expression::Complex(data) = result {
            assert_eq!(data.real, expr!(0));
            assert_eq!(data.imag, expr!(0));
        } else {
            panic!("Expected complex result");
        }
    }
}
