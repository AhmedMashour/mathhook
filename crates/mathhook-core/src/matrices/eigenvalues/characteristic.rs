use crate::core::expression::Expression;
use crate::core::symbol::Symbol;
use crate::simplify::Simplify;

/// Characteristic polynomial of a matrix
///
/// Represents the polynomial det(A - λI) where A is a matrix and λ is a variable.
/// The roots of this polynomial are the eigenvalues of the matrix.
///
/// # Mathematical Definition
///
/// For an n×n matrix A, the characteristic polynomial is:
/// $$p(\lambda) = \det(A - \lambda I)$$
///
/// This expands to a polynomial of degree n:
/// $$p(\lambda) = c_0 + c_1\lambda + c_2\lambda^2 + \cdots + c_n\lambda^n$$
///
/// # Properties
///
/// - Degree equals matrix dimension
/// - Coefficients are polynomial expressions in matrix entries
/// - Roots (eigenvalues) may be real or complex
/// - Leading coefficient is (-1)^n
/// - Constant term is det(A)
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{expr, symbol, Expression};
/// use mathhook_core::matrices::eigenvalues::characteristic::CharacteristicPolynomial;
///
/// let lambda = symbol!(lambda);
///
/// // 2×2 matrix characteristic polynomial: λ² - trace·λ + det
/// let poly = CharacteristicPolynomial::new(
///     vec![
///         Expression::add(vec![
///            expr!(a * d),
///          Expression::mul(vec![expr!(-1), expr!(b * c)]),
///         ]), // det(A)
///         Expression::mul(vec![expr!(-1), expr!(a + d)]),            // -trace(A)
///         expr!(1),                   // leading coefficient
///     ],
///     lambda.clone()
/// );
/// ```
#[derive(Debug, Clone)]
pub struct CharacteristicPolynomial {
    /// Coefficients of the polynomial [c₀, c₁, c₂, ..., cₙ]
    /// where p(λ) = c₀ + c₁λ + c₂λ² + ... + cₙλⁿ
    pub coefficients: Vec<Expression>,
    /// Variable symbol (typically λ or lambda)
    pub variable: Symbol,
}

impl CharacteristicPolynomial {
    /// Creates new characteristic polynomial
    ///
    /// # Arguments
    ///
    /// * `coefficients` - Polynomial coefficients [c₀, c₁, ..., cₙ]
    /// * `variable` - Variable symbol (typically λ)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::matrices::eigenvalues::characteristic::CharacteristicPolynomial;
    ///
    /// let lambda = symbol!(lambda);
    /// let poly = CharacteristicPolynomial::new(
    ///     vec![expr!(6), expr!(-5), expr!(1)],  // λ² - 5λ + 6
    ///     lambda
    /// );
    /// ```
    pub fn new(coefficients: Vec<Expression>, variable: Symbol) -> Self {
        Self {
            coefficients,
            variable,
        }
    }

    /// Returns the degree of the polynomial
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::matrices::eigenvalues::characteristic::CharacteristicPolynomial;
    ///
    /// let lambda = symbol!(lambda);
    /// let poly = CharacteristicPolynomial::new(
    ///     vec![expr!(1), expr!(2), expr!(3)],
    ///     lambda
    /// );
    /// assert_eq!(poly.degree(), 2);
    /// ```
    pub fn degree(&self) -> usize {
        if self.coefficients.is_empty() {
            0
        } else {
            self.coefficients.len() - 1
        }
    }

    /// Converts polynomial to expression form
    ///
    /// Returns: c₀ + c₁λ + c₂λ² + ... + cₙλⁿ
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::matrices::eigenvalues::characteristic::CharacteristicPolynomial;
    ///
    /// let lambda = symbol!(lambda);
    /// let poly = CharacteristicPolynomial::new(
    ///     vec![expr!(6), expr!(-5), expr!(1)],
    ///     lambda.clone()
    /// );
    ///
    /// let expr = poly.to_expression();
    /// // Represents: 6 - 5λ + λ²
    /// ```
    pub fn to_expression(&self) -> Expression {
        if self.coefficients.is_empty() {
            return Expression::integer(0);
        }

        let mut terms = Vec::new();

        for (power, coeff) in self.coefficients.iter().enumerate() {
            if coeff.is_zero() {
                continue;
            }

            let term = if power == 0 {
                coeff.clone()
            } else if power == 1 {
                Expression::mul(vec![
                    coeff.clone(),
                    Expression::symbol(self.variable.clone()),
                ])
            } else {
                Expression::mul(vec![
                    coeff.clone(),
                    Expression::pow(
                        Expression::symbol(self.variable.clone()),
                        Expression::integer(power as i64),
                    ),
                ])
            };

            terms.push(term);
        }

        if terms.is_empty() {
            Expression::integer(0)
        } else if terms.len() == 1 {
            terms[0].clone()
        } else {
            Expression::add(terms).simplify()
        }
    }

    /// Evaluates polynomial at given value
    ///
    /// Uses Horner's method for efficient evaluation.
    ///
    /// # Arguments
    ///
    /// * `value` - Value to substitute for variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::matrices::eigenvalues::characteristic::CharacteristicPolynomial;
    ///
    /// let lambda = symbol!(lambda);
    /// let poly = CharacteristicPolynomial::new(
    ///     vec![expr!(6), expr!(-5), expr!(1)],  // λ² - 5λ + 6
    ///     lambda
    /// );
    ///
    /// let result = poly.evaluate(&expr!(2));  // 2² - 5(2) + 6 = 0
    /// assert_eq!(result, expr!(0));
    /// ```
    pub fn evaluate(&self, value: &Expression) -> Expression {
        if self.coefficients.is_empty() {
            return Expression::integer(0);
        }

        // Horner's method: p(x) = c₀ + x(c₁ + x(c₂ + ... + x·cₙ))
        let mut result = self.coefficients.last().unwrap().clone();

        for coeff in self.coefficients.iter().rev().skip(1) {
            result = Expression::add(vec![
                coeff.clone(),
                Expression::mul(vec![value.clone(), result]),
            ])
            .simplify();
        }

        result
    }

    /// Adds two characteristic polynomials
    ///
    /// Note: This is polynomial addition, not matrix addition.
    /// Both polynomials must use the same variable.
    ///
    /// # Arguments
    ///
    /// * `poly1` - First polynomial
    /// * `poly2` - Second polynomial
    ///
    /// # Returns
    ///
    /// Sum of the two polynomials
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::matrices::eigenvalues::characteristic::{CharacteristicPolynomial, CharacteristicPolynomialBuilder};
    ///
    /// let lambda = symbol!(lambda);
    /// let builder = CharacteristicPolynomialBuilder;
    ///
    /// let poly1 = CharacteristicPolynomial::new(
    ///     vec![expr!(1), expr!(2)],  // 1 + 2λ
    ///     lambda.clone()
    /// );
    /// let poly2 = CharacteristicPolynomial::new(
    ///     vec![expr!(3), expr!(4)],  // 3 + 4λ
    ///     lambda.clone()
    /// );
    ///
    /// let sum = builder.add(&poly1, &poly2);  // 4 + 6λ
    /// assert_eq!(sum.coefficients.len(), 2);
    /// ```
    pub fn add(&self, other: &CharacteristicPolynomial) -> CharacteristicPolynomial {
        let max_len = self.coefficients.len().max(other.coefficients.len());
        let mut coefficients = vec![Expression::integer(0); max_len];

        for (i, coeff) in coefficients.iter_mut().enumerate().take(max_len) {
            let coeff1 = if i < self.coefficients.len() {
                self.coefficients[i].clone()
            } else {
                Expression::integer(0)
            };

            let coeff2 = if i < other.coefficients.len() {
                other.coefficients[i].clone()
            } else {
                Expression::integer(0)
            };

            *coeff = Expression::add(vec![coeff1, coeff2]).simplify();
        }

        CharacteristicPolynomial {
            coefficients,
            variable: self.variable.clone(),
        }
    }

    /// Multiplies two characteristic polynomials
    ///
    /// # Arguments
    ///
    /// * `poly1` - First polynomial
    /// * `poly2` - Second polynomial
    ///
    /// # Returns
    ///
    /// Product of the two polynomials
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::matrices::eigenvalues::characteristic::CharacteristicPolynomial;
    ///
    /// let lambda = symbol!(lambda);
    ///
    /// let poly1 = CharacteristicPolynomial::new(
    ///     vec![expr!(1), expr!(1)],  // 1 + λ
    ///     lambda.clone()
    /// );
    /// let poly2 = CharacteristicPolynomial::new(
    ///     vec![expr!(2), expr!(1)],  // 2 + λ
    ///     lambda.clone()
    /// );
    ///
    /// let product = poly1.multiply(&poly2);  // 2 + 3λ + λ²
    /// assert_eq!(product.degree(), 2);
    /// ```
    pub fn multiply(&self, other: &CharacteristicPolynomial) -> CharacteristicPolynomial {
        if self.coefficients.is_empty() || other.coefficients.is_empty() {
            return CharacteristicPolynomial::new(vec![], self.variable.clone());
        }

        let result_len = self.coefficients.len() + other.coefficients.len() - 1;
        let mut result_coeffs = vec![Expression::integer(0); result_len];

        for (i, c1) in self.coefficients.iter().enumerate() {
            for (j, c2) in other.coefficients.iter().enumerate() {
                let product = Expression::mul(vec![c1.clone(), c2.clone()]).simplify();
                result_coeffs[i + j] =
                    Expression::add(vec![result_coeffs[i + j].clone(), product]).simplify();
            }
        }

        CharacteristicPolynomial::new(result_coeffs, self.variable.clone())
    }

    /// Formats polynomial as human-readable string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::matrices::eigenvalues::characteristic::CharacteristicPolynomial;
    ///
    /// let lambda = symbol!(lambda);
    /// let poly = CharacteristicPolynomial::new(
    ///     vec![expr!(6), expr!(-5), expr!(1)],
    ///     lambda
    /// );
    ///
    /// let formatted = poly.format();
    /// // Output: "6 + (-5)·λ + λ²"
    /// ```
    pub fn format(&self) -> String {
        if self.coefficients.is_empty() {
            return "0".to_owned();
        }

        let mut parts = Vec::new();

        for (power, coeff) in self.coefficients.iter().enumerate() {
            if coeff.is_zero() {
                continue;
            }

            let mut part = String::new();

            if !coeff.is_one() || power == 0 {
                part.push_str(&coeff.to_string());
            }

            if power > 0 {
                if !coeff.is_one() {
                    part.push('·');
                }
                part.push_str(self.variable.name.as_ref());

                if power > 1 {
                    part.push('^');
                    part.push_str(&power.to_string());
                }
            }

            parts.push(part);
        }

        if parts.is_empty() {
            "0".to_owned()
        } else {
            parts.join(" + ")
        }
    }
}

/// Builder for characteristic polynomials
///
/// Provides methods for constructing characteristic polynomials from matrices.
pub struct CharacteristicPolynomialBuilder;

impl CharacteristicPolynomialBuilder {
    /// Adds two characteristic polynomials
    ///
    /// # Arguments
    ///
    /// * `poly1` - First polynomial
    /// * `poly2` - Second polynomial
    ///
    /// # Returns
    ///
    /// Sum of the two polynomials (same as `CharacteristicPolynomial::add`)
    pub fn add(
        &self,
        poly1: &CharacteristicPolynomial,
        poly2: &CharacteristicPolynomial,
    ) -> CharacteristicPolynomial {
        let max_len = poly1.coefficients.len().max(poly2.coefficients.len());
        let mut coefficients = vec![Expression::integer(0); max_len];

        for (i, coeff) in coefficients.iter_mut().enumerate().take(max_len) {
            let coeff1 = if i < poly1.coefficients.len() {
                poly1.coefficients[i].clone()
            } else {
                Expression::integer(0)
            };

            let coeff2 = if i < poly2.coefficients.len() {
                poly2.coefficients[i].clone()
            } else {
                Expression::integer(0)
            };

            *coeff = Expression::add(vec![coeff1, coeff2]).simplify();
        }

        CharacteristicPolynomial {
            coefficients,
            variable: poly1.variable.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_new_polynomial() {
        let lambda = symbol!(lambda);
        let poly =
            CharacteristicPolynomial::new(vec![expr!(1), expr!(2), expr!(3)], lambda.clone());

        assert_eq!(poly.coefficients.len(), 3);
        assert_eq!(poly.degree(), 2);
    }

    #[test]
    fn test_polynomial_degree() {
        let lambda = symbol!(lambda);

        let poly1 = CharacteristicPolynomial::new(vec![expr!(1)], lambda.clone());
        assert_eq!(poly1.degree(), 0);

        let poly2 = CharacteristicPolynomial::new(vec![expr!(1), expr!(2)], lambda.clone());
        assert_eq!(poly2.degree(), 1);

        let poly3 =
            CharacteristicPolynomial::new(vec![expr!(1), expr!(2), expr!(3)], lambda.clone());
        assert_eq!(poly3.degree(), 2);

        let poly_empty = CharacteristicPolynomial::new(vec![], lambda);
        assert_eq!(poly_empty.degree(), 0);
    }

    #[test]
    fn test_polynomial_to_expression() {
        let lambda = symbol!(lambda);
        let poly = CharacteristicPolynomial::new(vec![expr!(6), expr!(-5), expr!(1)], lambda);

        let expr = poly.to_expression();
        // Should represent: 6 - 5λ + λ²
        assert!(!expr.is_zero());
    }

    #[test]
    fn test_polynomial_evaluate() {
        let lambda = symbol!(lambda);
        let poly = CharacteristicPolynomial::new(vec![expr!(6), expr!(-5), expr!(1)], lambda);

        // Test at λ = 2: 6 - 5(2) + 2² = 6 - 10 + 4 = 0
        let result = poly.evaluate(&expr!(2));
        assert_eq!(result.simplify(), expr!(0));

        // Test at λ = 3: 6 - 5(3) + 3² = 6 - 15 + 9 = 0
        let result = poly.evaluate(&expr!(3));
        assert_eq!(result.simplify(), expr!(0));
    }

    #[test]
    fn test_polynomial_addition() {
        let lambda = symbol!(lambda);
        let poly1 = CharacteristicPolynomial::new(vec![expr!(1), expr!(2)], lambda.clone());
        let poly2 = CharacteristicPolynomial::new(vec![expr!(3), expr!(4)], lambda.clone());

        let sum = poly1.add(&poly2);

        assert_eq!(sum.coefficients.len(), 2);
        assert_eq!(sum.coefficients[0].simplify(), expr!(4));
        assert_eq!(sum.coefficients[1].simplify(), expr!(6));
    }

    #[test]
    fn test_polynomial_addition_different_lengths() {
        let lambda = symbol!(lambda);
        let poly1 =
            CharacteristicPolynomial::new(vec![expr!(1), expr!(2), expr!(3)], lambda.clone());
        let poly2 = CharacteristicPolynomial::new(vec![expr!(4), expr!(5)], lambda.clone());

        let sum = poly1.add(&poly2);

        assert_eq!(sum.coefficients.len(), 3);
        assert_eq!(sum.coefficients[0].simplify(), expr!(5));
        assert_eq!(sum.coefficients[1].simplify(), expr!(7));
        assert_eq!(sum.coefficients[2].simplify(), expr!(3));
    }

    #[test]
    fn test_polynomial_multiplication() {
        let lambda = symbol!(lambda);
        let poly1 = CharacteristicPolynomial::new(vec![expr!(1), expr!(1)], lambda.clone());
        let poly2 = CharacteristicPolynomial::new(vec![expr!(2), expr!(1)], lambda.clone());

        let product = poly1.multiply(&poly2);

        // (1 + λ)(2 + λ) = 2 + 3λ + λ²
        assert_eq!(product.degree(), 2);
        assert_eq!(product.coefficients[0].simplify(), expr!(2));
        assert_eq!(product.coefficients[1].simplify(), expr!(3));
        assert_eq!(product.coefficients[2].simplify(), expr!(1));
    }

    #[test]
    fn test_polynomial_format() {
        let lambda = symbol!(lambda);
        let poly = CharacteristicPolynomial::new(vec![expr!(6), expr!(-5), expr!(1)], lambda);

        let formatted = poly.format();
        assert!(formatted.contains(poly.variable.name.as_ref()));
    }

    #[test]
    fn test_builder_add() {
        let lambda = symbol!(lambda);
        let builder = CharacteristicPolynomialBuilder;

        let poly1 = CharacteristicPolynomial::new(vec![expr!(1), expr!(2)], lambda.clone());
        let poly2 = CharacteristicPolynomial::new(vec![expr!(3), expr!(4)], lambda.clone());

        let sum = builder.add(&poly1, &poly2);

        assert_eq!(sum.coefficients.len(), 2);
        assert_eq!(sum.coefficients[0].simplify(), expr!(4));
        assert_eq!(sum.coefficients[1].simplify(), expr!(6));
    }
}
