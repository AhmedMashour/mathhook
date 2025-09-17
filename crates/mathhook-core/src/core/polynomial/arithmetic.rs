//! Polynomial Arithmetic Operations
//!
//! Provides polynomial division operations including long division,
//! quotient extraction, and remainder computation.

use super::error::PolynomialError;
use super::properties::PolynomialProperties;
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;

/// Trait for polynomial arithmetic operations
///
/// Provides methods for polynomial division operations.
/// These operations require the expressions to be polynomials
/// in the specified variable.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::polynomial::PolynomialArithmetic;
/// use mathhook_core::core::Expression;
/// use mathhook_core::symbol;
///
/// let x = symbol!(x);
/// // x^2 - 1
/// let dividend = Expression::add(vec![
///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
///     Expression::integer(-1),
/// ]);
/// // x - 1
/// let divisor = Expression::add(vec![
///     Expression::symbol(x.clone()),
///     Expression::integer(-1),
/// ]);
///
/// let (quotient, remainder) = dividend.poly_div(&divisor, &x).unwrap();
/// // quotient = x + 1, remainder = 0
/// ```
pub trait PolynomialArithmetic: PolynomialProperties {
    /// Perform polynomial long division
    ///
    /// Divides `self` by `divisor` with respect to the variable `var`.
    /// Returns (quotient, remainder) such that `self = quotient * divisor + remainder`.
    ///
    /// # Arguments
    ///
    /// * `divisor` - The polynomial to divide by
    /// * `var` - The variable to treat as the polynomial indeterminate
    ///
    /// # Returns
    ///
    /// `Ok((quotient, remainder))` if both expressions are valid polynomials,
    /// `Err(PolynomialError)` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::polynomial::PolynomialArithmetic;
    /// use mathhook_core::core::Expression;
    /// use mathhook_core::symbol;
    ///
    /// let x = symbol!(x);
    /// // x^2 - 1
    /// let dividend = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::integer(-1),
    /// ]);
    /// // x - 1
    /// let divisor = Expression::add(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::integer(-1),
    /// ]);
    ///
    /// let (q, r) = dividend.poly_div(&divisor, &x).unwrap();
    /// // q = x + 1, r = 0
    /// ```
    fn poly_div(
        &self,
        divisor: &Expression,
        var: &Symbol,
    ) -> Result<(Expression, Expression), PolynomialError>;

    /// Get quotient of polynomial division
    ///
    /// # Arguments
    ///
    /// * `divisor` - The polynomial to divide by
    /// * `var` - The variable to treat as the polynomial indeterminate
    ///
    /// # Returns
    ///
    /// The quotient of the division
    fn poly_quo(&self, divisor: &Expression, var: &Symbol) -> Result<Expression, PolynomialError>;

    /// Get remainder of polynomial division
    ///
    /// # Arguments
    ///
    /// * `divisor` - The polynomial to divide by
    /// * `var` - The variable to treat as the polynomial indeterminate
    ///
    /// # Returns
    ///
    /// The remainder of the division
    fn poly_rem(&self, divisor: &Expression, var: &Symbol) -> Result<Expression, PolynomialError>;

    /// Check if one polynomial divides another exactly
    ///
    /// # Arguments
    ///
    /// * `divisor` - The potential divisor
    /// * `var` - The variable
    ///
    /// # Returns
    ///
    /// True if `divisor` divides `self` with zero remainder
    fn is_divisible_by(&self, divisor: &Expression, var: &Symbol) -> bool;
}

impl PolynomialArithmetic for Expression {
    fn poly_div(
        &self,
        divisor: &Expression,
        var: &Symbol,
    ) -> Result<(Expression, Expression), PolynomialError> {
        polynomial_long_division(self, divisor, var)
    }

    fn poly_quo(&self, divisor: &Expression, var: &Symbol) -> Result<Expression, PolynomialError> {
        let (quotient, _) = self.poly_div(divisor, var)?;
        Ok(quotient)
    }

    fn poly_rem(&self, divisor: &Expression, var: &Symbol) -> Result<Expression, PolynomialError> {
        let (_, remainder) = self.poly_div(divisor, var)?;
        Ok(remainder)
    }

    fn is_divisible_by(&self, divisor: &Expression, var: &Symbol) -> bool {
        match self.poly_rem(divisor, var) {
            Ok(rem) => rem.is_zero(),
            Err(_) => false,
        }
    }
}

/// Perform polynomial long division
///
/// Implements the standard polynomial long division algorithm.
fn polynomial_long_division(
    dividend: &Expression,
    divisor: &Expression,
    var: &Symbol,
) -> Result<(Expression, Expression), PolynomialError> {
    use super::classification::PolynomialClassification;

    // Check that both are polynomials in the variable
    if !dividend.is_polynomial_in(std::slice::from_ref(var)) {
        return Err(PolynomialError::NotPolynomial {
            expression: dividend.clone(),
            reason: "dividend is not a polynomial in the given variable".to_owned(),
        });
    }
    if !divisor.is_polynomial_in(std::slice::from_ref(var)) {
        return Err(PolynomialError::NotPolynomial {
            expression: divisor.clone(),
            reason: "divisor is not a polynomial in the given variable".to_owned(),
        });
    }

    // Get degrees
    let dividend_deg = dividend.degree(var).unwrap_or(0);
    let divisor_deg = divisor.degree(var).unwrap_or(0);

    // Check for division by zero
    if divisor.is_zero() {
        return Err(PolynomialError::DivisionByZero);
    }

    // If divisor degree > dividend degree, quotient is 0, remainder is dividend
    if divisor_deg > dividend_deg {
        return Ok((Expression::integer(0), dividend.clone()));
    }

    // Get leading coefficient of divisor
    let divisor_lc = divisor.leading_coefficient(var);

    // Initialize quotient and remainder
    let mut quotient_terms: Vec<Expression> = Vec::new();
    let mut remainder = dividend.simplify();

    // Max iterations to prevent infinite loops (degree + 1 iterations should suffice)
    let max_iterations = (dividend_deg + 2) as usize;
    let mut iterations = 0;

    // Perform division
    while !remainder.is_zero() && iterations < max_iterations {
        iterations += 1;

        let rem_deg = remainder.degree(var).unwrap_or(-1);
        if rem_deg < divisor_deg {
            break;
        }

        // Calculate the term to add to quotient
        let rem_lc = remainder.leading_coefficient(var);
        let term_coef = divide_expressions(&rem_lc, &divisor_lc);
        let term_deg = rem_deg - divisor_deg;

        let term = if term_deg == 0 {
            term_coef.clone()
        } else {
            Expression::mul(vec![
                term_coef.clone(),
                Expression::pow(
                    Expression::symbol(var.clone()),
                    Expression::integer(term_deg),
                ),
            ])
        };

        quotient_terms.push(term.clone());

        // Subtract term * divisor from remainder
        let subtrahend = multiply_poly(&term, divisor);
        remainder = subtract_poly(&remainder, &subtrahend);

        // Simplify remainder using the actual simplify module
        remainder = remainder.simplify();
    }

    let quotient = if quotient_terms.is_empty() {
        Expression::integer(0)
    } else if quotient_terms.len() == 1 {
        quotient_terms.into_iter().next().unwrap()
    } else {
        Expression::add(quotient_terms)
    };

    Ok((quotient.simplify(), remainder))
}

/// Divide two expressions (for coefficients)
fn divide_expressions(num: &Expression, den: &Expression) -> Expression {
    match (num, den) {
        (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(b)))
            if *b != 0 =>
        {
            if a % b == 0 {
                Expression::integer(a / b)
            } else {
                Expression::mul(vec![
                    num.clone(),
                    Expression::pow(den.clone(), Expression::integer(-1)),
                ])
            }
        }
        _ => Expression::mul(vec![
            num.clone(),
            Expression::pow(den.clone(), Expression::integer(-1)),
        ]),
    }
}

/// Multiply two polynomials
fn multiply_poly(a: &Expression, b: &Expression) -> Expression {
    Expression::mul(vec![a.clone(), b.clone()])
}

/// Subtract two polynomials
fn subtract_poly(a: &Expression, b: &Expression) -> Expression {
    Expression::add(vec![
        a.clone(),
        Expression::mul(vec![Expression::integer(-1), b.clone()]),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_poly_div_simple() {
        let x = symbol!(x);

        // (x^2 - 1) / (x - 1) = (x + 1), remainder 0
        let dividend = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::integer(-1),
        ]);
        let divisor = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);

        let result = dividend.poly_div(&divisor, &x);
        assert!(result.is_ok());
    }

    #[test]
    fn test_poly_div_with_remainder() {
        let x = symbol!(x);

        // x^2 / (x - 1) has non-zero remainder
        let dividend = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let divisor = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(-1)]);

        let result = dividend.poly_div(&divisor, &x);
        assert!(result.is_ok());
    }

    #[test]
    fn test_poly_quo() {
        let x = symbol!(x);

        let dividend = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let divisor = Expression::symbol(x.clone());

        let result = dividend.poly_quo(&divisor, &x);
        assert!(result.is_ok());
    }

    #[test]
    fn test_poly_rem() {
        let x = symbol!(x);

        let dividend = Expression::symbol(x.clone());
        let divisor = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);

        let result = dividend.poly_rem(&divisor, &x);
        assert!(result.is_ok());
    }

    #[test]
    fn test_division_by_zero() {
        let x = symbol!(x);

        let dividend = Expression::symbol(x.clone());
        let divisor = Expression::integer(0);

        let result = dividend.poly_div(&divisor, &x);
        assert!(matches!(result, Err(PolynomialError::DivisionByZero)));
    }

    #[test]
    fn test_is_divisible_by() {
        let x = symbol!(x);

        // x^2 is divisible by x
        let dividend = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        let divisor = Expression::symbol(x.clone());

        assert!(dividend.is_divisible_by(&divisor, &x));
    }
}
