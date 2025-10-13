//! Complex number operations and arithmetic
//!
//! Handles symbolic complex numbers with Expression-based real and imaginary parts.
//! Provides comprehensive complex arithmetic including addition, multiplication,
//! division, conjugation, and polar form conversions.

use crate::core::Expression;
use crate::simplify::Simplify;
use crate::expr;

/// Trait for complex number operations
///
/// Provides methods for performing arithmetic and other operations on complex numbers
/// represented as expressions with symbolic real and imaginary parts.
pub trait ComplexOperations {
    /// Add two complex expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, ComplexOperations, expr};
    ///
    /// let z1 = Expression::complex(expr!(3), expr!(4));
    /// let z2 = Expression::complex(expr!(1), expr!(2));
    /// let result = z1.complex_add(&z2);
    /// ```
    fn complex_add(&self, other: &Expression) -> Expression;

    /// Subtract two complex expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, ComplexOperations, expr};
    ///
    /// let z1 = Expression::complex(expr!(5), expr!(3));
    /// let z2 = Expression::complex(expr!(2), expr!(1));
    /// let result = z1.complex_subtract(&z2);
    /// ```
    fn complex_subtract(&self, other: &Expression) -> Expression;

    /// Multiply two complex expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, ComplexOperations, expr};
    ///
    /// let z1 = Expression::complex(expr!(3), expr!(4));
    /// let z2 = Expression::complex(expr!(1), expr!(2));
    /// let result = z1.complex_multiply(&z2);
    /// ```
    fn complex_multiply(&self, other: &Expression) -> Expression;

    /// Divide two complex expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, ComplexOperations, expr};
    ///
    /// let z1 = Expression::complex(expr!(6), expr!(8));
    /// let z2 = Expression::complex(expr!(3), expr!(4));
    /// let result = z1.complex_divide(&z2);
    /// ```
    fn complex_divide(&self, other: &Expression) -> Expression;

    /// Get the complex conjugate
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, ComplexOperations, expr};
    ///
    /// let z = Expression::complex(expr!(3), expr!(4));
    /// let conjugate = z.complex_conjugate();
    /// ```
    fn complex_conjugate(&self) -> Expression;

    /// Get the modulus (absolute value)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, ComplexOperations, expr};
    ///
    /// let z = Expression::complex(expr!(3), expr!(4));
    /// let modulus = z.complex_modulus();
    /// ```
    fn complex_modulus(&self) -> Expression;

    /// Get the argument (angle in radians)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, ComplexOperations, expr};
    ///
    /// let z = Expression::complex(expr!(1), expr!(1));
    /// let argument = z.complex_argument();
    /// ```
    fn complex_argument(&self) -> Expression;

    /// Convert to polar form (magnitude, angle)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, ComplexOperations, expr};
    ///
    /// let z = Expression::complex(expr!(3), expr!(4));
    /// let (magnitude, angle) = z.to_polar_form();
    /// ```
    fn to_polar_form(&self) -> (Expression, Expression);

    /// Check if the expression is real
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, ComplexOperations, expr};
    ///
    /// let z = Expression::complex(expr!(5), expr!(0));
    /// assert!(z.is_real());
    /// ```
    fn is_real(&self) -> bool;

    /// Check if the expression has an imaginary component
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, ComplexOperations, expr};
    ///
    /// let z = Expression::complex(expr!(3), expr!(4));
    /// assert!(z.is_imaginary());
    /// ```
    fn is_imaginary(&self) -> bool;

    /// Check if the expression is pure imaginary
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, ComplexOperations, expr};
    ///
    /// let z = Expression::complex(expr!(0), expr!(5));
    /// assert!(z.is_pure_imaginary());
    /// ```
    fn is_pure_imaginary(&self) -> bool;
}

impl ComplexOperations for Expression {
    fn complex_add(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Complex(a), Expression::Complex(b)) => Expression::complex(
                Expression::add(vec![a.real.clone(), b.real.clone()]).simplify(),
                Expression::add(vec![a.imag.clone(), b.imag.clone()]).simplify(),
            ),
            _ => Expression::function("undefined", vec![]),
        }
    }

    fn complex_subtract(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Complex(a), Expression::Complex(b)) => Expression::complex(
                Expression::add(vec![
                    a.real.clone(),
                    Expression::mul(vec![expr!(-1), b.real.clone()]),
                ])
                .simplify(),
                Expression::add(vec![
                    a.imag.clone(),
                    Expression::mul(vec![expr!(-1), b.imag.clone()]),
                ])
                .simplify(),
            ),

            (Expression::Complex(a), real_expr) => Expression::complex(
                Expression::add(vec![
                    a.real.clone(),
                    Expression::mul(vec![expr!(-1), real_expr.clone()]),
                ])
                .simplify(),
                a.imag.clone(),
            ),

            (real_expr, Expression::Complex(b)) => Expression::complex(
                Expression::add(vec![
                    real_expr.clone(),
                    Expression::mul(vec![expr!(-1), b.real.clone()]),
                ])
                .simplify(),
                Expression::mul(vec![expr!(-1), b.imag.clone()]).simplify(),
            ),

            _ => Expression::add(vec![
                self.clone(),
                Expression::mul(vec![expr!(-1), other.clone()]),
            ]),
        }
    }

    fn complex_multiply(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Complex(a), Expression::Complex(b)) => {
                let ac = Expression::mul(vec![a.real.clone(), b.real.clone()]).simplify();
                let bd = Expression::mul(vec![a.imag.clone(), b.imag.clone()]).simplify();
                let ad = Expression::mul(vec![a.real.clone(), b.imag.clone()]).simplify();
                let bc = Expression::mul(vec![a.imag.clone(), b.real.clone()]).simplify();

                Expression::complex(
                    Expression::add(vec![
                        ac,
                        Expression::mul(vec![expr!(-1), bd]).simplify(),
                    ])
                    .simplify(),
                    Expression::add(vec![ad, bc]).simplify(),
                )
            }

            (Expression::Complex(a), real_expr) => Expression::complex(
                Expression::mul(vec![a.real.clone(), real_expr.clone()]).simplify(),
                Expression::mul(vec![a.imag.clone(), real_expr.clone()]).simplify(),
            ),

            (real_expr, Expression::Complex(b)) => Expression::complex(
                Expression::mul(vec![real_expr.clone(), b.real.clone()]).simplify(),
                Expression::mul(vec![real_expr.clone(), b.imag.clone()]).simplify(),
            ),

            _ => Expression::mul(vec![self.clone(), other.clone()]),
        }
    }

    fn complex_divide(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Complex(_a), Expression::Complex(b)) => {
                let conjugate = Expression::complex(
                    b.real.clone(),
                    Expression::mul(vec![expr!(-1), b.imag.clone()]),
                );

                let numerator = self.complex_multiply(&conjugate);
                let denominator = Expression::add(vec![
                    Expression::pow(b.real.clone(), expr!(2)),
                    Expression::pow(b.imag.clone(), expr!(2)),
                ])
                .simplify();

                match numerator {
                    Expression::Complex(num_data) => Expression::complex(
                        Expression::mul(vec![
                            num_data.real,
                            Expression::pow(denominator.clone(), expr!(-1)),
                        ])
                        .simplify(),
                        Expression::mul(vec![
                            num_data.imag,
                            Expression::pow(denominator, expr!(-1)),
                        ])
                        .simplify(),
                    ),
                    _ => Expression::mul(vec![
                        numerator,
                        Expression::pow(denominator, expr!(-1)),
                    ]),
                }
            }

            (Expression::Complex(a), real_expr) => Expression::complex(
                Expression::mul(vec![
                    a.real.clone(),
                    Expression::pow(real_expr.clone(), expr!(-1)),
                ])
                .simplify(),
                Expression::mul(vec![
                    a.imag.clone(),
                    Expression::pow(real_expr.clone(), expr!(-1)),
                ])
                .simplify(),
            ),

            (real_expr, Expression::Complex(b)) => {
                let conjugate = Expression::complex(
                    b.real.clone(),
                    Expression::mul(vec![expr!(-1), b.imag.clone()]),
                );
                let numerator = real_expr.complex_multiply(&conjugate);
                let denominator = Expression::add(vec![
                    Expression::pow(b.real.clone(), expr!(2)),
                    Expression::pow(b.imag.clone(), expr!(2)),
                ])
                .simplify();

                match numerator {
                    Expression::Complex(num_data) => Expression::complex(
                        Expression::mul(vec![
                            num_data.real,
                            Expression::pow(denominator.clone(), expr!(-1)),
                        ])
                        .simplify(),
                        Expression::mul(vec![
                            num_data.imag,
                            Expression::pow(denominator, expr!(-1)),
                        ])
                        .simplify(),
                    ),
                    _ => Expression::mul(vec![
                        numerator,
                        Expression::pow(denominator, expr!(-1)),
                    ]),
                }
            }

            _ => Expression::mul(vec![
                self.clone(),
                Expression::pow(other.clone(), expr!(-1)),
            ]),
        }
    }

    fn complex_conjugate(&self) -> Expression {
        match self {
            Expression::Complex(data) => Expression::complex(
                data.real.clone(),
                Expression::mul(vec![expr!(-1), data.imag.clone()]).simplify(),
            ),
            _ => self.clone(),
        }
    }

    fn complex_modulus(&self) -> Expression {
        match self {
            Expression::Complex(data) => Expression::function(
                "sqrt",
                vec![Expression::add(vec![
                    Expression::pow(data.real.clone(), expr!(2)),
                    Expression::pow(data.imag.clone(), expr!(2)),
                ])
                .simplify()],
            ),
            _ => Expression::function("abs", vec![self.clone()]),
        }
    }

    fn complex_argument(&self) -> Expression {
        match self {
            Expression::Complex(data) => {
                Expression::function("atan2", vec![data.imag.clone(), data.real.clone()])
            }
            _ => expr!(0),
        }
    }

    fn to_polar_form(&self) -> (Expression, Expression) {
        (self.complex_modulus(), self.complex_argument())
    }

    fn is_real(&self) -> bool {
        match self {
            Expression::Complex(data) => data.imag.is_zero(),
            _ => true,
        }
    }

    fn is_imaginary(&self) -> bool {
        match self {
            Expression::Complex(data) => !data.imag.is_zero(),
            _ => false,
        }
    }

    fn is_pure_imaginary(&self) -> bool {
        match self {
            Expression::Complex(data) => data.real.is_zero() && !data.imag.is_zero(),
            _ => false,
        }
    }
}

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
