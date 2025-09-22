//! Complex expression simplification
//!
//! Handles simplification of complex number expressions, ensuring that
//! real and imaginary parts are properly simplified.

use super::Simplify;
use crate::core::Expression;

/// Simplify complex expressions
#[inline(always)]
pub fn simplify_complex(complex_expr: &Expression) -> Expression {
    match complex_expr {
        Expression::Complex(complex) => {
            // Process real and imaginary parts directly for performance
            let simplified_real = complex.real.clone();
            let simplified_imag = complex.imag.clone();

            // Check for special cases
            match (&simplified_real, &simplified_imag) {
                // If imaginary part is zero, return just the real part
                (real, Expression::Number(crate::core::Number::Integer(0))) => real.clone(),
                (real, Expression::Number(crate::core::Number::Float(f))) if *f == 0.0 => {
                    real.clone()
                }

                // If real part is zero and imaginary part is not zero, return just imaginary part with i
                (Expression::Number(crate::core::Number::Integer(0)), imag) => {
                    Expression::mul(vec![
                        imag.clone(),
                        Expression::constant(crate::core::MathConstant::I),
                    ])
                }
                (Expression::Number(crate::core::Number::Float(f)), imag) if *f == 0.0 => {
                    Expression::mul(vec![
                        imag.clone(),
                        Expression::constant(crate::core::MathConstant::I),
                    ])
                }

                // Both parts are non-zero, return simplified complex
                _ => Expression::complex(simplified_real, simplified_imag),
            }
        }
        _ => complex_expr.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::MathConstant;

    #[test]
    fn test_complex_simplification() {
        // Complex with zero imaginary part should become real
        let complex_real = Expression::complex(Expression::integer(3), Expression::integer(0));
        let simplified = simplify_complex(&complex_real);
        assert_eq!(simplified, Expression::integer(3));

        // Complex with zero real part should become imaginary
        let complex_imag = Expression::complex(Expression::integer(0), Expression::integer(2));
        let simplified = simplify_complex(&complex_imag);
        assert_eq!(
            simplified,
            Expression::mul(vec![
                Expression::integer(2),
                Expression::constant(MathConstant::I)
            ])
        );
    }
}
