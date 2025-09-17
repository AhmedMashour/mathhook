//! Matrix linear system solvers
//!
//! Provides methods for solving Ax = b using LU, Cholesky, and QR decompositions.

use crate::core::Expression;
use crate::error::MathError;
use crate::matrices::types::MatrixData;
use crate::matrices::unified::operations::CoreMatrixOps;
use crate::matrices::unified::Matrix;

impl Matrix {
    /// Solve Lx = b for lower triangular L using forward substitution
    ///
    /// # Arguments
    /// * `b` - Right-hand side vector
    ///
    /// # Returns
    /// Solution vector x
    ///
    /// # Errors
    /// * `DivisionByZero` if any diagonal element is zero
    /// * `DomainError` if dimensions don't match
    ///
    /// # Algorithm
    /// For i = 0 to n-1:
    ///   `x[i]` = (`b[i]` - Σ(`L[i][j]` * `x[j]`) for `j` < i) / `L[i][i]`
    pub fn forward_substitution(&self, b: &[Expression]) -> Result<Vec<Expression>, MathError> {
        let (rows, cols) = self.dimensions();

        if rows != cols {
            return Err(MathError::DomainError {
                operation: "forward_substitution".to_string(),
                value: Expression::function("matrix", vec![]),
                reason: format!(
                    "Forward substitution requires square matrix, got {}x{}",
                    rows, cols
                ),
            });
        }

        if b.len() != rows {
            return Err(MathError::DomainError {
                operation: "forward_substitution".to_string(),
                value: Expression::function("vector", vec![]),
                reason: format!(
                    "Dimension mismatch: matrix is {}x{} but b has {} elements",
                    rows,
                    cols,
                    b.len()
                ),
            });
        }

        let mut x = vec![Expression::integer(0); rows];

        for i in 0..rows {
            // Accumulate sum = Σ L[i][j] * x[j] for j < i
            let mut terms: Vec<Expression> = Vec::new();
            for (j, xj) in x.iter().enumerate().take(i) {
                let lij = self.get_element(i, j);
                // Use is_zero_fast() - avoids simplify() in hot loop
                if !lij.is_zero_fast() && !xj.is_zero_fast() {
                    terms.push(Expression::mul(vec![lij, xj.clone()]));
                }
            }

            let lii = self.get_element(i, i);
            // Use is_zero_fast() - pivot elements should already be simplified
            if lii.is_zero_fast() {
                return Err(MathError::DivisionByZero);
            }

            // x[i] = (b[i] - sum) / L[i][i]
            // Note: Expression::add() and operators already simplify, no need for .simplify()
            let numerator = if terms.is_empty() {
                b[i].clone()
            } else {
                let sum = Expression::add(terms);
                b[i].clone() - sum // Operator already simplifies
            };

            // Compute x[i] = numerator / L[i][i]
            // Directly compute integer/integer to produce clean results
            x[i] = if lii == Expression::integer(1) {
                numerator
            } else {
                // Try to compute integer division directly for clean results
                match (&numerator, &lii) {
                    (
                        Expression::Number(crate::core::Number::Integer(num)),
                        Expression::Number(crate::core::Number::Integer(den)),
                    ) => {
                        if *den != 0 && num % den == 0 {
                            Expression::integer(num / den)
                        } else if *den != 0 {
                            // Create rational for non-exact division
                            use num_bigint::BigInt;
                            use num_rational::BigRational;
                            Expression::Number(crate::core::Number::rational(BigRational::new(
                                BigInt::from(*num),
                                BigInt::from(*den),
                            )))
                        } else {
                            Expression::mul(vec![
                                numerator,
                                Expression::pow(lii, Expression::integer(-1)),
                            ])
                        }
                    }
                    _ => Expression::mul(vec![
                        numerator,
                        Expression::pow(lii, Expression::integer(-1)),
                    ]),
                }
            };
        }

        Ok(x)
    }

    /// Solve Ux = b for upper triangular U using backward substitution
    ///
    /// # Arguments
    /// * `b` - Right-hand side vector
    ///
    /// # Returns
    /// Solution vector x
    ///
    /// # Errors
    /// * `DivisionByZero` if any diagonal element is zero
    /// * `DomainError` if dimensions don't match
    ///
    /// For i = n-1 down to 0:
    ///   `x[i]` = (`b[i]` - Σ(`U[i][j]` * `x[j]`) for j > i) / `U[i][i]`
    pub fn backward_substitution(&self, b: &[Expression]) -> Result<Vec<Expression>, MathError> {
        let (rows, cols) = self.dimensions();

        if rows != cols {
            return Err(MathError::DomainError {
                operation: "backward_substitution".to_string(),
                value: Expression::function("matrix", vec![]),
                reason: format!(
                    "Backward substitution requires square matrix, got {}x{}",
                    rows, cols
                ),
            });
        }

        if b.len() != rows {
            return Err(MathError::DomainError {
                operation: "backward_substitution".to_string(),
                value: Expression::function("vector", vec![]),
                reason: format!(
                    "Dimension mismatch: matrix is {}x{} but b has {} elements",
                    rows,
                    cols,
                    b.len()
                ),
            });
        }
        let mut x = vec![Expression::integer(0); rows];

        for i in (0..rows).rev() {
            // Accumulate sum = Σ U[i][j] * x[j] for j > i
            let mut terms: Vec<Expression> = Vec::new();
            for (j, xj) in x.iter().enumerate().skip(i + 1) {
                let uij = self.get_element(i, j);
                // Use is_zero_fast() - avoids simplify() in hot loop
                if !uij.is_zero_fast() && !xj.is_zero_fast() {
                    terms.push(Expression::mul(vec![uij, xj.clone()]));
                }
            }

            let uii = self.get_element(i, i);
            // Use is_zero_fast() - pivot elements should already be simplified
            if uii.is_zero_fast() {
                return Err(MathError::DivisionByZero);
            }

            // x[i] = (b[i] - sum) / U[i][i]
            // Note: Expression::add() and operators already simplify, no need for .simplify()
            let numerator = if terms.is_empty() {
                b[i].clone()
            } else {
                let sum = Expression::add(terms);
                b[i].clone() - sum // Operator already simplifies
            };

            // Compute x[i] = numerator / U[i][i]
            // Directly compute integer/integer to produce clean results
            x[i] = if uii == Expression::integer(1) {
                numerator
            } else {
                // Try to compute integer division directly for clean results
                match (&numerator, &uii) {
                    (
                        Expression::Number(crate::core::Number::Integer(num)),
                        Expression::Number(crate::core::Number::Integer(den)),
                    ) => {
                        if *den != 0 && num % den == 0 {
                            Expression::integer(num / den)
                        } else if *den != 0 {
                            // Create rational for non-exact division
                            use num_bigint::BigInt;
                            use num_rational::BigRational;
                            Expression::Number(crate::core::Number::rational(BigRational::new(
                                BigInt::from(*num),
                                BigInt::from(*den),
                            )))
                        } else {
                            Expression::mul(vec![
                                numerator,
                                Expression::pow(uii, Expression::integer(-1)),
                            ])
                        }
                    }
                    _ => Expression::mul(vec![
                        numerator,
                        Expression::pow(uii, Expression::integer(-1)),
                    ]),
                }
            };
        }

        Ok(x)
    }

    /// Solve Ax = b using optimal decomposition
    ///
    /// # Arguments
    /// * `b` - Right-hand side vector
    ///
    /// # Returns
    /// Solution vector x
    ///
    /// # Errors
    /// * `DomainError` if matrix is not square or dimensions don't match
    /// * `DivisionByZero` if matrix is singular
    ///
    /// # Algorithm Selection
    /// - Symmetric positive definite matrices: Cholesky (LL^T), ~2x faster
    /// - General square matrices: LU decomposition with partial pivoting
    ///
    /// # Examples
    /// ```
    /// use mathhook_core::matrices::Matrix;
    /// use mathhook_core::expr;
    ///
    /// let a = Matrix::from_arrays([[2, 1], [1, 3]]);
    /// let b = vec![expr!(5), expr!(7)];
    /// let x = a.solve(&b).unwrap();
    /// ```
    pub fn solve(&self, b: &[Expression]) -> Result<Vec<Expression>, MathError> {
        let (rows, cols) = self.dimensions();

        if rows != cols {
            return Err(MathError::DomainError {
                operation: "solve".to_string(),
                value: Expression::function("matrix", vec![]),
                reason: format!("Solve requires square matrix, got {}x{}", rows, cols),
            });
        }

        if b.len() != rows {
            return Err(MathError::DomainError {
                operation: "solve".to_string(),
                value: Expression::function("vector", vec![]),
                reason: format!(
                    "Dimension mismatch: matrix is {}x{} but b has {} elements",
                    rows,
                    cols,
                    b.len()
                ),
            });
        }

        // Try Cholesky for symmetric matrices (2x faster for SPD)
        if self.is_symmetric() {
            if let Some(chol) = self.cholesky_decomposition() {
                // Solve LL^T x = b
                // Step 1: Ly = b (forward substitution)
                let y = chol.l.forward_substitution(b)?;
                // Step 2: L^T x = y (backward substitution on L transpose)
                let lt = chol.l.transpose();
                return lt.backward_substitution(&y);
            }
            // Fall through to LU if Cholesky fails (not positive definite)
        }

        // General case: LU decomposition with partial pivoting
        self.solve_via_lu(b)
    }

    /// Solve Ax = b using LU decomposition
    ///
    /// This is the fallback solver for non-SPD matrices.
    fn solve_via_lu(&self, b: &[Expression]) -> Result<Vec<Expression>, MathError> {
        let lu = self.lu_decomposition().ok_or(MathError::DivisionByZero)?;

        let pb = apply_permutation(&lu.p, b);

        let y = lu.l.forward_substitution(&pb)?;

        let x = lu.u.backward_substitution(&y)?;

        Ok(x)
    }

    /// Solve least squares problem: min ||Ax - b||₂ using QR decomposition
    ///
    /// # Arguments
    /// * `b` - Right-hand side vector
    ///
    /// # Returns
    /// Solution vector x that minimizes ||Ax - b||₂
    ///
    /// # Errors
    /// * `DomainError` if dimensions don't match or m < n
    /// * `DivisionByZero` if R has zero diagonal elements
    ///
    /// # Algorithm
    /// For m×n matrix A (m >= n):
    /// 1. Compute A = QR (Q is m×n, R is n×n upper triangular)
    /// 2. Compute c = Q^T * b
    /// 3. Solve Rx = c`[0:n]` using backward substitution
    ///
    /// # Examples
    /// ```
    /// use mathhook_core::matrices::Matrix;
    /// use mathhook_core::expr;
    ///
    /// // Overdetermined system: 3 equations, 2 unknowns
    /// let a = Matrix::from_arrays([[1, 0], [0, 1], [1, 1]]);
    /// let b = vec![expr!(1), expr!(2), expr!(2)];
    /// let x = a.solve_least_squares(&b).unwrap();
    /// ```
    pub fn solve_least_squares(&self, b: &[Expression]) -> Result<Vec<Expression>, MathError> {
        let (rows, cols) = self.dimensions();

        if rows < cols {
            return Err(MathError::DomainError {
                operation: "solve_least_squares".to_string(),
                value: Expression::function("matrix", vec![]),
                reason: format!(
                    "Least squares requires m >= n (overdetermined), got {}x{}",
                    rows, cols
                ),
            });
        }

        if b.len() != rows {
            return Err(MathError::DomainError {
                operation: "solve_least_squares".to_string(),
                value: Expression::function("vector", vec![]),
                reason: format!(
                    "Dimension mismatch: matrix is {}x{} but b has {} elements",
                    rows,
                    cols,
                    b.len()
                ),
            });
        }

        // For square matrices, use standard solve
        if rows == cols {
            return self.solve(b);
        }

        // QR decomposition: A = QR
        let qr = self.qr_decomposition().ok_or(MathError::DomainError {
            operation: "solve_least_squares".to_string(),
            value: Expression::function("matrix", vec![]),
            reason: "QR decomposition failed (linearly dependent columns)".to_string(),
        })?;

        // Compute c = Q^T * b
        let qt = qr.q.transpose();
        let c = matrix_vector_multiply(&qt, b);

        // Take first n elements for Rx = c[0:n]
        let c_truncated: Vec<Expression> = c.into_iter().take(cols).collect();

        // Solve Rx = c using backward substitution
        qr.r.backward_substitution(&c_truncated)
    }

    /// Compute inverse using LU decomposition: A^(-1) = solve(A, I) column by column
    ///
    /// For each column j of identity matrix I, solve A*x_j = e_j
    /// The solution vectors x_j form the columns of A^(-1)
    pub(crate) fn inverse_via_lu(&self) -> Option<Matrix> {
        let (n, _) = self.dimensions();
        if n == 0 {
            return None;
        }

        // Compute LU decomposition once
        let lu = self.lu_decomposition()?;

        // Solve for each column of the inverse
        let mut inv_columns: Vec<Vec<Expression>> = Vec::with_capacity(n);

        for j in 0..n {
            // Create unit vector e_j
            let e_j: Vec<Expression> = (0..n)
                .map(|i| {
                    if i == j {
                        Expression::integer(1)
                    } else {
                        Expression::integer(0)
                    }
                })
                .collect();

            // Solve A * x_j = e_j using precomputed LU
            let pb = apply_permutation(&lu.p, &e_j);
            let y = match lu.l.forward_substitution(&pb) {
                Ok(y) => y,
                Err(_) => return None,
            };
            let x_j = match lu.u.backward_substitution(&y) {
                Ok(x) => x,
                Err(_) => return None,
            };

            inv_columns.push(x_j);
        }

        // Transpose columns to rows for Matrix::Dense
        let mut result_rows: Vec<Vec<Expression>> = Vec::with_capacity(n);
        for i in 0..n {
            let row: Vec<Expression> = inv_columns.iter().map(|col| col[i].clone()).collect();
            result_rows.push(row);
        }

        Some(Matrix::Dense(MatrixData { rows: result_rows }).optimize())
    }
}

/// Multiply matrix M by vector v: result = M * v
fn matrix_vector_multiply(m: &Matrix, v: &[Expression]) -> Vec<Expression> {
    let (rows, cols) = m.dimensions();
    let mut result = Vec::with_capacity(rows);

    for i in 0..rows {
        let mut terms: Vec<Expression> = Vec::new();
        for (j, vj) in v.iter().enumerate().take(cols) {
            let mij = m.get_element(i, j);
            // Use is_zero_fast() - avoids simplify() in hot loop
            if !mij.is_zero_fast() && !vj.is_zero_fast() {
                terms.push(Expression::mul(vec![mij, vj.clone()]));
            }
        }
        // Note: Expression::add() already simplifies internally, no need for .simplify()
        let row_sum = if terms.is_empty() {
            Expression::integer(0)
        } else {
            Expression::add(terms)
        };
        result.push(row_sum);
    }

    result
}

/// Apply permutation matrix P to vector b: result = P * b
///
/// Optimized for permutation matrices: O(n) instead of O(n²)
/// since each row of P has exactly one non-zero element (which is 1).
pub(crate) fn apply_permutation(p: &Option<Matrix>, b: &[Expression]) -> Vec<Expression> {
    match p {
        None => b.to_vec(),
        Some(p_matrix) => {
            let n = b.len();
            let mut result = Vec::with_capacity(n);

            for i in 0..n {
                // Find the column j where P[i][j] = 1
                // For a permutation matrix, there's exactly one such j per row
                for (j, bj) in b.iter().enumerate() {
                    let pij = p_matrix.get_element(i, j);
                    // Use is_zero_fast() - permutation elements are 0 or 1 literals
                    if !pij.is_zero_fast() {
                        // P[i][j] = 1, so result[i] = b[j]
                        result.push(bj.clone());
                        break;
                    }
                }
            }

            result
        }
    }
}
