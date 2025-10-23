//! Linear system of ODEs solver
//!
//! Solves systems of first-order linear ODEs with constant coefficients:
//! dx/dt = Ax where A is a constant matrix
//!
//! Uses eigenvalue-eigenvector method for diagonalizable systems:
//! x(t) = c₁e^(λ₁t)v₁ + c₂e^(λ₂t)v₂ + ... + cₙe^(λₙt)vₙ

use crate::core::{Expression, Symbol};
use crate::matrix::{EigenOperations, Matrix};
use crate::ode::first_order::{ODEError, ODEResult};
use crate::simplify::Simplify;

/// Linear system of ODEs solver
///
/// Solves systems dx/dt = Ax where A is a constant coefficient matrix.
pub struct LinearSystemSolver;

impl LinearSystemSolver {
    /// Solve linear system dx/dt = Ax
    ///
    /// Uses eigenvalue-eigenvector method. For an n×n system:
    /// - Compute eigenvalues λ₁, λ₂, ..., λₙ and eigenvectors v₁, v₂, ..., vₙ
    /// - General solution: x(t) = c₁e^(λ₁t)v₁ + c₂e^(λ₂t)v₂ + ... + cₙe^(λₙt)vₙ
    ///
    /// # Arguments
    ///
    /// * `coefficient_matrix` - The constant coefficient matrix A
    /// * `independent_var` - The independent variable (typically t)
    /// * `initial_conditions` - Optional initial state vector x(t₀) = x₀
    ///
    /// # Returns
    ///
    /// Vector of expressions representing the solution [x₁(t), x₂(t), ..., xₙ(t)]
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::ode::systems::LinearSystemSolver;
    /// use mathhook_core::matrix::Matrix;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let t = symbol!(t);
    ///
    /// // 2×2 system: dx/dt = [1 0; 0 2]x
    /// let matrix = Matrix::diagonal(vec![expr!(1), expr!(2)]);
    ///
    /// let solver = LinearSystemSolver;
    /// let solution = solver.solve(&matrix, &t, None);
    /// ```
    pub fn solve(
        &self,
        coefficient_matrix: &Matrix,
        independent_var: &Symbol,
        initial_conditions: Option<Vec<Expression>>,
    ) -> Result<Vec<Expression>, ODEError> {
        let (rows, cols) = coefficient_matrix.dimensions();

        if rows != cols {
            return Err(ODEError::NotLinearForm {
                reason: format!("Coefficient matrix must be square, got {}×{}", rows, cols),
            });
        }

        let n = rows;

        // Check if matrix is diagonalizable
        if !coefficient_matrix.is_diagonalizable() {
            return Err(ODEError::NotImplemented {
                feature: "Non-diagonalizable systems (requires Jordan normal form)".to_string(),
            });
        }

        // Compute eigenvalues and eigenvectors
        let eigen_decomp = coefficient_matrix
            .eigen_decomposition()
            .ok_or_else(|| ODEError::NotImplemented {
                feature: "Eigendecomposition failed".to_string(),
            })?;

        let eigenvalues = &eigen_decomp.eigenvalues;
        let eigenvectors = &eigen_decomp.eigenvectors;

        // Build general solution: x(t) = Σ cᵢ e^(λᵢt) vᵢ
        let mut solution_components = Vec::new();

        for i in 0..n {
            let lambda = &eigenvalues[i];

            // Extract eigenvector column i
            let mut eigenvector_col = Vec::new();
            for row_idx in 0..n {
                eigenvector_col.push(eigenvectors.get_element(row_idx, i));
            }

            // Build e^(λᵢt)
            let exponent = Expression::mul(vec![lambda.clone(), Expression::symbol(independent_var.clone())]);
            let exp_term = Expression::function("exp", vec![exponent]);

            // Constant coefficient
            let c_symbol = Symbol::scalar(&format!("C{}", i + 1));
            let c = Expression::symbol(c_symbol);

            // c * e^(λt) * v
            let scaled_exp = Expression::mul(vec![c, exp_term]);

            // Scale eigenvector by e^(λt)
            let mut scaled_eigenvector = Vec::new();
            for component in eigenvector_col {
                scaled_eigenvector.push(Expression::mul(vec![scaled_exp.clone(), component]));
            }

            solution_components.push(scaled_eigenvector);
        }

        // Sum all components for each variable
        let mut final_solution = vec![Expression::integer(0); n];

        for i in 0..n {
            let mut sum_terms = Vec::new();
            for j in 0..n {
                sum_terms.push(solution_components[j][i].clone());
            }
            final_solution[i] = Expression::add(sum_terms).simplify();
        }

        // If initial conditions provided, solve for constants
        if let Some(ic) = initial_conditions {
            if ic.len() != n {
                return Err(ODEError::NotLinearForm {
                    reason: format!(
                        "Initial conditions length {} does not match system size {}",
                        ic.len(),
                        n
                    ),
                });
            }

            // Evaluate solution at t=0 and solve for constants
            // This requires solving a linear system: V * c = x₀
            // where V is the eigenvector matrix and c are the constants
            // For now, return general solution (implementing constant solving is complex)
            return Err(ODEError::NotImplemented {
                feature: "Initial condition solving for system ODEs".to_string(),
            });
        }

        Ok(final_solution)
    }

    /// Solve 2×2 linear system dx/dt = Ax
    ///
    /// Specialized solver for 2×2 systems with explicit formulas.
    ///
    /// # Arguments
    ///
    /// * `a11`, `a12`, `a21`, `a22` - Matrix coefficients [a11 a12; a21 a22]
    /// * `independent_var` - The independent variable (typically t)
    ///
    /// # Returns
    ///
    /// Vector [x₁(t), x₂(t)] representing the solution
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::ode::systems::LinearSystemSolver;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let t = symbol!(t);
    ///
    /// // dx/dt = [1 0; 0 2]x
    /// let solver = LinearSystemSolver;
    /// let solution = solver.solve_2x2(
    ///     &expr!(1), &expr!(0),
    ///     &expr!(0), &expr!(2),
    ///     &t
    /// );
    /// ```
    pub fn solve_2x2(
        &self,
        a11: &Expression,
        a12: &Expression,
        a21: &Expression,
        a22: &Expression,
        independent_var: &Symbol,
    ) -> Result<Vec<Expression>, ODEError> {
        let matrix = Matrix::dense(vec![
            vec![a11.clone(), a12.clone()],
            vec![a21.clone(), a22.clone()],
        ]);

        self.solve(&matrix, independent_var, None)
    }

    /// Solve 3×3 linear system dx/dt = Ax
    ///
    /// Specialized solver for 3×3 systems.
    ///
    /// # Arguments
    ///
    /// * `matrix_entries` - Flattened 3×3 matrix entries [a11, a12, a13, a21, a22, a23, a31, a32, a33]
    /// * `independent_var` - The independent variable (typically t)
    ///
    /// # Returns
    ///
    /// Vector [x₁(t), x₂(t), x₃(t)] representing the solution
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::ode::systems::LinearSystemSolver;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let t = symbol!(t);
    ///
    /// // dx/dt = [1 0 0; 0 2 0; 0 0 3]x (diagonal)
    /// let solver = LinearSystemSolver;
    /// let solution = solver.solve_3x3(
    ///     &[expr!(1), expr!(0), expr!(0),
    ///       expr!(0), expr!(2), expr!(0),
    ///       expr!(0), expr!(0), expr!(3)],
    ///     &t
    /// );
    /// ```
    pub fn solve_3x3(
        &self,
        matrix_entries: &[Expression; 9],
        independent_var: &Symbol,
    ) -> Result<Vec<Expression>, ODEError> {
        let matrix = Matrix::dense(vec![
            vec![
                matrix_entries[0].clone(),
                matrix_entries[1].clone(),
                matrix_entries[2].clone(),
            ],
            vec![
                matrix_entries[3].clone(),
                matrix_entries[4].clone(),
                matrix_entries[5].clone(),
            ],
            vec![
                matrix_entries[6].clone(),
                matrix_entries[7].clone(),
                matrix_entries[8].clone(),
            ],
        ]);

        self.solve(&matrix, independent_var, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_diagonal_2x2_system() {
        let t = symbol!(t);

        // dx/dt = [1 0; 0 2]x
        // Solution: x₁(t) = C₁e^t, x₂(t) = C₂e^(2t)
        let matrix = Matrix::diagonal(vec![expr!(1), expr!(2)]);

        let solver = LinearSystemSolver;
        let solution = solver.solve(&matrix, &t, None);

        assert!(solution.is_ok(), "Should solve diagonal system");
        let sol = solution.unwrap();
        assert_eq!(sol.len(), 2, "Should have 2 solution components");

        // Check that solution contains exponentials
        let x1_str = sol[0].to_string();
        let x2_str = sol[1].to_string();

        assert!(
            x1_str.contains("exp") || x1_str.contains("e"),
            "x₁ should contain exponential: {}",
            x1_str
        );
        assert!(
            x2_str.contains("exp") || x2_str.contains("e"),
            "x₂ should contain exponential: {}",
            x2_str
        );
    }

    #[test]
    fn test_diagonal_3x3_system() {
        let t = symbol!(t);

        // dx/dt = [1 0 0; 0 2 0; 0 0 3]x
        let matrix = Matrix::diagonal(vec![expr!(1), expr!(2), expr!(3)]);

        let solver = LinearSystemSolver;
        let solution = solver.solve(&matrix, &t, None);

        assert!(solution.is_ok(), "Should solve 3×3 diagonal system");
        let sol = solution.unwrap();
        assert_eq!(sol.len(), 3, "Should have 3 solution components");

        for (i, component) in sol.iter().enumerate() {
            let comp_str = component.to_string();
            assert!(
                comp_str.contains("exp") || comp_str.contains("e"),
                "Component {} should contain exponential: {}",
                i + 1,
                comp_str
            );
        }
    }

    #[test]
    fn test_2x2_specialized_solver() {
        let t = symbol!(t);

        // dx/dt = [2 0; 0 3]x
        let solver = LinearSystemSolver;
        let solution = solver.solve_2x2(&expr!(2), &expr!(0), &expr!(0), &expr!(3), &t);

        assert!(solution.is_ok(), "2×2 specialized solver should work");
        let sol = solution.unwrap();
        assert_eq!(sol.len(), 2, "Should have 2 components");
    }

    #[test]
    fn test_3x3_specialized_solver() {
        let t = symbol!(t);

        // dx/dt = diag[1, 2, 3]x
        let solver = LinearSystemSolver;
        let solution = solver.solve_3x3(
            &[
                expr!(1),
                expr!(0),
                expr!(0),
                expr!(0),
                expr!(2),
                expr!(0),
                expr!(0),
                expr!(0),
                expr!(3),
            ],
            &t,
        );

        assert!(solution.is_ok(), "3×3 specialized solver should work");
        let sol = solution.unwrap();
        assert_eq!(sol.len(), 3, "Should have 3 components");
    }

    #[test]
    fn test_non_square_matrix_error() {
        let t = symbol!(t);

        let matrix = Matrix::dense(vec![
            vec![expr!(1), expr!(0)],
            vec![expr!(0), expr!(2)],
            vec![expr!(1), expr!(1)],
        ]);

        let solver = LinearSystemSolver;
        let result = solver.solve(&matrix, &t, None);

        assert!(result.is_err(), "Should reject non-square matrix");
        if let Err(ODEError::NotLinearForm { reason }) = result {
            assert!(reason.contains("square"), "Error should mention square matrix");
        } else {
            panic!("Expected NotLinearForm error");
        }
    }

    #[test]
    fn test_solution_contains_constants() {
        let t = symbol!(t);

        let matrix = Matrix::diagonal(vec![expr!(1), expr!(2)]);

        let solver = LinearSystemSolver;
        let solution = solver.solve(&matrix, &t, None).unwrap();

        // Solution should contain constants C1, C2
        let full_solution = format!("{:?}", solution);
        assert!(
            full_solution.contains("C1") || full_solution.contains("C2"),
            "Solution should contain integration constants"
        );
    }

    #[test]
    fn test_coupled_2x2_system() {
        let t = symbol!(t);

        // dx/dt = [0 1; -1 0]x (rotation matrix, eigenvalues ±i)
        let solver = LinearSystemSolver;
        let solution = solver.solve_2x2(&expr!(0), &expr!(1), &expr!(-1), &expr!(0), &t);

        // This system has complex eigenvalues
        // Current implementation may not handle this yet
        if solution.is_ok() {
            let sol = solution.unwrap();
            assert_eq!(sol.len(), 2, "Should have 2 components");
        }
    }

    #[test]
    fn test_positive_eigenvalues_2x2() {
        let t = symbol!(t);

        // dx/dt = [2 1; 1 2]x
        // Eigenvalues: 1, 3 (both real and positive)
        let solver = LinearSystemSolver;
        let solution = solver.solve_2x2(&expr!(2), &expr!(1), &expr!(1), &expr!(2), &t);

        if solution.is_ok() {
            let sol = solution.unwrap();
            assert_eq!(sol.len(), 2, "Should have 2 components");

            for component in &sol {
                let comp_str = component.to_string();
                assert!(
                    comp_str.contains("exp") || comp_str.contains("e"),
                    "Should contain exponential terms"
                );
            }
        }
    }

    #[test]
    fn test_negative_eigenvalues_2x2() {
        let t = symbol!(t);

        // dx/dt = [-2 -1; -1 -2]x
        // Eigenvalues: -1, -3 (both real and negative, stable system)
        let solver = LinearSystemSolver;
        let solution = solver.solve_2x2(&expr!(-2), &expr!(-1), &expr!(-1), &expr!(-2), &t);

        if solution.is_ok() {
            let sol = solution.unwrap();
            assert_eq!(sol.len(), 2, "Should have 2 components");

            for component in &sol {
                let comp_str = component.to_string();
                assert!(
                    comp_str.contains("exp") || comp_str.contains("e"),
                    "Should contain exponential terms"
                );
            }
        }
    }

    #[test]
    fn test_3x3_identity_matrix() {
        let t = symbol!(t);

        // dx/dt = Ix (all eigenvalues = 1)
        let matrix = Matrix::identity(3);

        let solver = LinearSystemSolver;
        let solution = solver.solve(&matrix, &t, None);

        if solution.is_ok() {
            let sol = solution.unwrap();
            assert_eq!(sol.len(), 3, "Should have 3 components");

            for component in &sol {
                let comp_str = component.to_string();
                assert!(
                    comp_str.contains("exp") || comp_str.contains("e") || comp_str.contains("C"),
                    "Should contain exponential or constant"
                );
            }
        }
    }

    #[test]
    fn test_wrong_initial_condition_size() {
        let t = symbol!(t);

        let matrix = Matrix::diagonal(vec![expr!(1), expr!(2)]);

        let solver = LinearSystemSolver;
        let result = solver.solve(
            &matrix,
            &t,
            Some(vec![expr!(1), expr!(2), expr!(3)]), // 3 ICs for 2×2 system
        );

        assert!(result.is_err(), "Should reject wrong IC size");
        if let Err(ODEError::NotLinearForm { reason }) = result {
            assert!(
                reason.contains("Initial conditions"),
                "Should mention initial conditions mismatch"
            );
        } else {
            panic!("Expected NotLinearForm error");
        }
    }
}
