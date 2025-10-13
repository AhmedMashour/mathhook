//! ComplexOperations trait for complex arithmetic operations
//!
//! Provides trait methods for performing arithmetic operations on complex numbers
//! represented as expressions with symbolic real and imaginary parts.

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
