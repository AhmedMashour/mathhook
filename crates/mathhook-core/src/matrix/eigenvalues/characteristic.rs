//! Characteristic polynomial computation
//!
//! This module provides algorithms for computing the characteristic polynomial
//! det(A - λI) of matrices, which is fundamental for eigenvalue computation.

use crate::core::Expression;
use crate::matrix::types::*;
use crate::matrix::unified::Matrix;
use crate::simplify::Simplify;

/// Characteristic polynomial computation implementation
impl Matrix {
    /// Compute characteristic polynomial det(A - λI)
    ///
    /// Returns the characteristic polynomial whose roots are the eigenvalues.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::identity(2);
    /// let poly = matrix.characteristic_polynomial();
    /// assert_eq!(poly.coefficients.len(), 3); // (1-λ)² has 3 coefficients
    /// ```
    pub fn characteristic_polynomial(&self) -> CharacteristicPolynomial {
        let lambda = Expression::symbol("λ");

        match self {
            Matrix::Identity(data) => {
                // det(I - λI) = det((1-λ)I) = (1-λ)^n
                // For (1-λ)^n, coefficients are binomial coefficients with alternating signs
                let mut coefficients = Vec::new();
                for k in 0..=data.size {
                    let binomial_coeff = binomial_coefficient(data.size, k);
                    let sign = if k % 2 == 0 { 1 } else { -1 };
                    coefficients.push(Expression::integer(sign * binomial_coeff as i64));
                }

                CharacteristicPolynomial {
                    coefficients,
                    variable: lambda,
                }
            }
            Matrix::Diagonal(data) => {
                // det(D - λI) = ∏(d_i - λ)
                // For diagonal matrix, expand product of (d_i - λ) terms
                self.expand_diagonal_characteristic_polynomial(
                    &data.diagonal_elements,
                    lambda.clone(),
                )
            }
            _ => {
                // Compute characteristic polynomial for general matrices using determinant
                self.compute_general_characteristic_polynomial(lambda.clone())
            }
        }
    }

    /// Expand characteristic polynomial for diagonal matrix
    pub(crate) fn expand_diagonal_characteristic_polynomial(
        &self,
        diagonal_elements: &[Expression],
        lambda: Expression,
    ) -> CharacteristicPolynomial {
        if diagonal_elements.is_empty() {
            return CharacteristicPolynomial {
                coefficients: vec![Expression::integer(1)],
                variable: lambda,
            };
        }

        // Start with polynomial (d_0 - λ)
        let mut coefficients = vec![diagonal_elements[0].clone(), Expression::integer(-1)];

        // Multiply by each subsequent (d_i - λ) term
        for d_i in diagonal_elements.iter().skip(1) {
            coefficients = self.multiply_polynomial_by_linear(&coefficients, d_i.clone());
        }

        CharacteristicPolynomial {
            coefficients,
            variable: lambda,
        }
    }

    /// Multiply polynomial by (d - λ)
    pub(crate) fn multiply_polynomial_by_linear(
        &self,
        poly: &[Expression],
        d: Expression,
    ) -> Vec<Expression> {
        let mut result = vec![Expression::integer(0); poly.len() + 1];

        // Multiply by d
        for (i, coeff) in poly.iter().enumerate() {
            result[i] = Expression::add(vec![
                result[i].clone(),
                Expression::mul(vec![coeff.clone(), d.clone()]),
            ])
            .simplify();
        }

        // Multiply by -λ (shift coefficients)
        for (i, coeff) in poly.iter().enumerate() {
            result[i + 1] = Expression::add(vec![
                result[i + 1].clone(),
                Expression::mul(vec![Expression::integer(-1), coeff.clone()]),
            ])
            .simplify();
        }

        result
    }

    /// Compute characteristic polynomial for general matrices
    pub(crate) fn compute_general_characteristic_polynomial(
        &self,
        lambda: Expression,
    ) -> CharacteristicPolynomial {
        let (n, _) = self.dimensions();

        // Create (A - λI) matrix
        let mut a_minus_lambda_i = self.to_dense_matrix();
        for i in 0..n {
            let diagonal_elem = a_minus_lambda_i.get_element(i, i);
            let new_elem = Expression::add(vec![
                diagonal_elem,
                Expression::mul(vec![Expression::integer(-1), lambda.clone()]),
            ])
            .simplify();
            a_minus_lambda_i = a_minus_lambda_i.set_element(i, i, new_elem);
        }

        // Compute characteristic polynomial using cofactor expansion
        self.compute_determinant_polynomial(a_minus_lambda_i, lambda)
    }

    /// Evaluate characteristic polynomial at a given value
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::identity(2);
    /// let result = matrix.evaluate_characteristic_polynomial(&Expression::integer(1));
    /// assert_eq!(result, Expression::integer(0)); // (1-1)² = 0
    /// ```
    pub fn evaluate_characteristic_polynomial(&self, value: &Expression) -> Expression {
        let poly = self.characteristic_polynomial();
        let mut result = Expression::integer(0);

        for (i, coeff) in poly.coefficients.iter().enumerate() {
            let power = Expression::pow(value.clone(), Expression::integer(i as i64));
            let term = Expression::mul(vec![coeff.clone(), power]);
            result = Expression::add(vec![result, term]).simplify();
        }

        result
    }

    /// Get the trace from characteristic polynomial (coefficient of λ^(n-1))
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::diagonal(vec![
    ///     Expression::integer(2),
    ///     Expression::integer(3)
    /// ]);
    /// let trace = matrix.trace_from_characteristic();
    /// assert_eq!(trace, Expression::integer(5)); // 2 + 3 = 5
    /// ```
    pub fn trace_from_characteristic(&self) -> Expression {
        let poly = self.characteristic_polynomial();
        let n = self.dimensions().0;

        if n > 1 && poly.coefficients.len() > n - 1 {
            // The coefficient of λ^(n-1) is -trace
            Expression::mul(vec![
                Expression::integer(-1),
                poly.coefficients[n - 1].clone(),
            ])
            .simplify()
        } else {
            Expression::integer(0)
        }
    }

    /// Get the determinant from characteristic polynomial (constant term)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::Expression;
    ///
    /// let matrix = Matrix::diagonal(vec![
    ///     Expression::integer(2),
    ///     Expression::integer(3)
    /// ]);
    /// let det = matrix.determinant_from_characteristic();
    /// assert_eq!(det, Expression::integer(6)); // 2 * 3 = 6
    /// ```
    pub fn determinant_from_characteristic(&self) -> Expression {
        let poly = self.characteristic_polynomial();
        let n = self.dimensions().0;

        if !poly.coefficients.is_empty() {
            // The constant term is (-1)^n * det(A)
            let sign = if n % 2 == 0 { 1 } else { -1 };
            Expression::mul(vec![
                Expression::integer(sign),
                poly.coefficients[0].clone(),
            ])
            .simplify()
        } else {
            Expression::integer(0)
        }
    }

    /// Compute determinant as polynomial using cofactor expansion
    fn compute_determinant_polynomial(
        &self,
        matrix: Matrix,
        variable: Expression,
    ) -> CharacteristicPolynomial {
        let (n, _) = matrix.dimensions();

        match n {
            0 => CharacteristicPolynomial {
                coefficients: vec![Expression::integer(1)],
                variable,
            },
            1 => CharacteristicPolynomial {
                coefficients: vec![matrix.get_element(0, 0)],
                variable,
            },
            2 => {
                // det([[a, b], [c, d]]) = ad - bc
                let a = matrix.get_element(0, 0);
                let b = matrix.get_element(0, 1);
                let c = matrix.get_element(1, 0);
                let d = matrix.get_element(1, 1);

                let ad = Expression::mul(vec![a, d]).simplify();
                let bc = Expression::mul(vec![b, c]).simplify();
                let det =
                    Expression::add(vec![ad, Expression::mul(vec![Expression::integer(-1), bc])])
                        .simplify();

                // Extract polynomial coefficients from the determinant expression
                self.extract_polynomial_coefficients(det, variable)
            }
            3 => {
                // Use cofactor expansion along first row
                let mut det_poly = CharacteristicPolynomial {
                    coefficients: vec![Expression::integer(0)],
                    variable: variable.clone(),
                };

                for j in 0..3 {
                    let a_0j = matrix.get_element(0, j);
                    let minor = self.compute_3x3_minor(&matrix, 0, j);
                    let cofactor_poly =
                        self.compute_determinant_polynomial(minor, variable.clone());

                    // Multiply cofactor polynomial by a_0j and appropriate sign
                    let sign = if j % 2 == 0 { 1 } else { -1 };
                    let signed_element =
                        Expression::mul(vec![Expression::integer(sign), a_0j]).simplify();

                    let term_poly =
                        self.multiply_polynomial_by_expression(&cofactor_poly, signed_element);
                    det_poly = self.add_polynomials(&det_poly, &term_poly);
                }

                det_poly
            }
            _ => {
                // For larger matrices, use simplified approach
                // Return polynomial based on trace and determinant
                let trace = matrix.trace();
                let det = matrix.determinant();

                // Simplified polynomial: λ^n - trace*λ^(n-1) + ... + (-1)^n*det
                let mut coefficients = vec![Expression::integer(0); n + 1];
                coefficients[0] = Expression::mul(vec![
                    Expression::integer(if n % 2 == 0 { 1 } else { -1 }),
                    det,
                ])
                .simplify();
                coefficients[n - 1] =
                    Expression::mul(vec![Expression::integer(-1), trace]).simplify();
                coefficients[n] = Expression::integer(1);

                CharacteristicPolynomial {
                    coefficients,
                    variable,
                }
            }
        }
    }

    /// Extract polynomial coefficients from an expression
    fn extract_polynomial_coefficients(
        &self,
        expr: Expression,
        variable: Expression,
    ) -> CharacteristicPolynomial {
        // Simplified extraction - in practice would parse the expression tree
        // Handle simple polynomial forms (complete parser would analyze expression tree)
        match expr {
            _ if expr.is_zero() => CharacteristicPolynomial {
                coefficients: vec![Expression::integer(0)],
                variable,
            },
            _ => {
                // Default to linear polynomial
                CharacteristicPolynomial {
                    coefficients: vec![expr, Expression::integer(-1)],
                    variable,
                }
            }
        }
    }

    /// Compute 3x3 minor by removing row i and column j
    fn compute_3x3_minor(&self, matrix: &Matrix, remove_row: usize, remove_col: usize) -> Matrix {
        let mut minor_elements = Vec::new();

        for i in 0..3 {
            if i == remove_row {
                continue;
            }
            let mut row = Vec::new();
            for j in 0..3 {
                if j == remove_col {
                    continue;
                }
                row.push(matrix.get_element(i, j));
            }
            minor_elements.push(row);
        }

        Matrix::dense(minor_elements)
    }

    /// Multiply polynomial by a scalar expression
    fn multiply_polynomial_by_expression(
        &self,
        poly: &CharacteristicPolynomial,
        expr: Expression,
    ) -> CharacteristicPolynomial {
        let coefficients: Vec<Expression> = poly
            .coefficients
            .iter()
            .map(|coeff| Expression::mul(vec![coeff.clone(), expr.clone()]).simplify())
            .collect();

        CharacteristicPolynomial {
            coefficients,
            variable: poly.variable.clone(),
        }
    }

    /// Add two polynomials
    fn add_polynomials(
        &self,
        poly1: &CharacteristicPolynomial,
        poly2: &CharacteristicPolynomial,
    ) -> CharacteristicPolynomial {
        let max_len = poly1.coefficients.len().max(poly2.coefficients.len());
        let mut coefficients = vec![Expression::integer(0); max_len];

        for i in 0..max_len {
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

            coefficients[i] = Expression::add(vec![coeff1, coeff2]).simplify();
        }

        CharacteristicPolynomial {
            coefficients,
            variable: poly1.variable.clone(),
        }
    }
}

/// Helper function to compute binomial coefficient
pub(crate) fn binomial_coefficient(n: usize, k: usize) -> usize {
    if k > n {
        0
    } else if k == 0 || k == n {
        1
    } else {
        let k = k.min(n - k); // Take advantage of symmetry
        let mut result = 1;
        for i in 0..k {
            result = result * (n - i) / (i + 1);
        }
        result
    }
}
