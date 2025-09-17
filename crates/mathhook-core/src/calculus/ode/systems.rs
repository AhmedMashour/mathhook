//! Linear system of ODEs solver
//!
//! Solves systems of first-order linear ODEs with constant coefficients:
//! dx/dt = Ax where A is a constant matrix
//!
//! Uses eigenvalue-eigenvector method for diagonalizable systems:
//! x(t) = c₁e^(λ₁t)v₁ + c₂e^(λ₂t)v₂ + ... + cₙe^(λₙt)vₙ

use crate::algebra::solvers::{linear::LinearSolver, EquationSolver, SolverResult};
use crate::calculus::ode::first_order::ODEError;
use crate::core::{Expression, Symbol};
use crate::matrices::Matrix;
use crate::simplify::Simplify;
use std::collections::HashMap;

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
    /// # Complexity
    ///
    /// * **Time:** O(n³) for eigenvalue decomposition of n×n matrix
    /// * **Space:** O(n²) for storing eigenvectors and intermediate results
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
    /// use mathhook_core::calculus::ode::systems::LinearSystemSolver;
    /// use mathhook_core::matrices::Matrix;
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

        if !coefficient_matrix.is_diagonalizable() {
            return Err(ODEError::NotImplemented {
                feature: "Non-diagonalizable systems (requires Jordan normal form)".to_owned(),
            });
        }

        let eigen_decomp =
            coefficient_matrix
                .eigen_decomposition()
                .ok_or_else(|| ODEError::NotImplemented {
                    feature: "Eigendecomposition failed".to_owned(),
                })?;

        let eigenvalues = &eigen_decomp.eigenvalues;
        let eigenvectors = &eigen_decomp.eigenvectors;

        let solution_components: Vec<Vec<Expression>> = eigenvalues
            .iter()
            .enumerate()
            .map(|(i, lambda)| {
                let eigenvector_col: Vec<Expression> = (0..n)
                    .map(|row_idx| eigenvectors.get_element(row_idx, i))
                    .collect();

                let exponent = Expression::mul(vec![
                    lambda.clone(),
                    Expression::symbol(independent_var.clone()),
                ]);
                let exp_term = Expression::function("exp", vec![exponent]);

                let c_symbol = Symbol::new(format!("C{}", i + 1));
                let c = Expression::symbol(c_symbol);

                let scaled_exp = Expression::mul(vec![c, exp_term]);

                eigenvector_col
                    .into_iter()
                    .map(|component| Expression::mul(vec![scaled_exp.clone(), component]))
                    .collect()
            })
            .collect();

        let final_solution: Vec<Expression> = (0..n)
            .map(|i| {
                let sum_terms: Vec<Expression> = solution_components
                    .iter()
                    .map(|comp| comp[i].clone())
                    .collect();
                Expression::add(sum_terms).simplify()
            })
            .collect();

        if let Some(ic) = initial_conditions {
            return self.apply_initial_conditions(&final_solution, &ic, n, eigenvectors);
        }

        Ok(final_solution)
    }

    /// Apply initial conditions to solve for integration constants
    ///
    /// Solves the linear system V*c = y₀ where:
    /// - V is the eigenvector matrix
    /// - c is the vector of constants [C1, C2, ..., Cn]
    /// - y₀ is the initial condition vector
    fn apply_initial_conditions(
        &self,
        general_solution: &[Expression],
        initial_conditions: &[Expression],
        n: usize,
        eigenvectors: &Matrix,
    ) -> Result<Vec<Expression>, ODEError> {
        if initial_conditions.len() != n {
            return Err(ODEError::NotLinearForm {
                reason: format!(
                    "Initial conditions length {} does not match system size {}",
                    initial_conditions.len(),
                    n
                ),
            });
        }

        let linear_solver = LinearSolver::new_fast();
        let mut constant_values: HashMap<String, Expression> = HashMap::new();

        for i in 0..n {
            let constant_name = format!("C{}", i + 1);
            let equation = self.build_constant_equation(i, n, eigenvectors, initial_conditions);

            let substituted_equation = if i == 0 {
                equation
            } else {
                equation.substitute(&constant_values).simplify()
            };

            let constant_symbol = Symbol::new(&constant_name);
            let value = self.solve_for_constant(
                &linear_solver,
                &substituted_equation,
                &constant_symbol,
                &constant_name,
            )?;

            constant_values.insert(constant_name, value);
        }

        let particular_solution: Vec<Expression> = general_solution
            .iter()
            .map(|expr| expr.substitute(&constant_values).simplify())
            .collect();

        Ok(particular_solution)
    }

    /// Build equation for a single integration constant
    ///
    /// Constructs: Σⱼ vᵢⱼ*Cⱼ - y₀ᵢ = 0
    fn build_constant_equation(
        &self,
        row_index: usize,
        n: usize,
        eigenvectors: &Matrix,
        initial_conditions: &[Expression],
    ) -> Expression {
        let mut equation_terms = Vec::new();

        for j in 0..n {
            let eigenvector_component = eigenvectors.get_element(row_index, j);
            let c_symbol = Symbol::new(format!("C{}", j + 1));
            let term = Expression::mul(vec![eigenvector_component, Expression::symbol(c_symbol)]);
            equation_terms.push(term);
        }

        equation_terms.push(Expression::mul(vec![
            Expression::integer(-1),
            initial_conditions[row_index].clone(),
        ]));

        Expression::add(equation_terms)
    }

    /// Solve for a single integration constant
    ///
    /// Handles all possible solver result cases with appropriate error messages
    fn solve_for_constant(
        &self,
        solver: &LinearSolver,
        equation: &Expression,
        variable: &Symbol,
        constant_name: &str,
    ) -> Result<Expression, ODEError> {
        match solver.solve(equation, variable) {
            SolverResult::Single(value) => Ok(value),
            SolverResult::NoSolution => Err(ODEError::NotLinearForm {
                reason: format!(
                    "No solution for integration constant {} (inconsistent system)",
                    constant_name
                ),
            }),
            SolverResult::InfiniteSolutions => Err(ODEError::NotLinearForm {
                reason: format!(
                    "Infinite solutions for integration constant {} (underdetermined)",
                    constant_name
                ),
            }),
            SolverResult::Multiple(_) => Err(ODEError::NotLinearForm {
                reason: format!(
                    "Multiple solutions for integration constant {}",
                    constant_name
                ),
            }),
            SolverResult::Parametric(_) => Err(ODEError::NotLinearForm {
                reason: format!(
                    "Parametric solutions not supported for integration constant {}",
                    constant_name
                ),
            }),
            SolverResult::Partial(_) => Err(ODEError::NotLinearForm {
                reason: format!(
                    "Partial solutions not supported for integration constant {}",
                    constant_name
                ),
            }),
        }
    }

    /// Solve 2×2 linear system dx/dt = Ax
    ///
    /// Specialized solver for 2×2 systems with explicit formulas.
    ///
    /// # Complexity
    ///
    /// * **Time:** O(1) for 2×2 eigenvalue computation (quadratic formula)
    /// * **Space:** O(1) for storing solution components
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
    /// use mathhook_core::calculus::ode::systems::LinearSystemSolver;
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
    /// # Complexity
    ///
    /// * **Time:** O(1) for 3×3 eigenvalue computation (cubic formula)
    /// * **Space:** O(1) for storing solution components
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
    /// use mathhook_core::calculus::ode::systems::LinearSystemSolver;
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
        let matrix = Matrix::diagonal(vec![expr!(1), expr!(2)]);
        let solver = LinearSystemSolver;
        let solution = solver.solve(&matrix, &t, None);

        assert!(solution.is_ok(), "Should solve diagonal system");
        let sol = solution.unwrap();
        assert_eq!(sol.len(), 2, "Should have 2 solution components");
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
    }

    #[test]
    fn test_2x2_system_with_initial_conditions() {
        let t = symbol!(t);
        let matrix = Matrix::diagonal(vec![expr!(1), expr!(2)]);
        let initial_conditions = vec![expr!(3), expr!(4)];

        let solver = LinearSystemSolver;
        let solution = solver.solve(&matrix, &t, Some(initial_conditions));

        assert!(
            solution.is_ok(),
            "Should solve system with initial conditions: {:?}",
            solution.err()
        );

        let sol = solution.unwrap();
        assert_eq!(sol.len(), 2, "Should have 2 solution components");

        let mut t_subs = HashMap::new();
        t_subs.insert(t.name().to_string(), expr!(0));

        let sol_at_zero: Vec<Expression> = sol
            .iter()
            .map(|expr| expr.substitute(&t_subs).simplify())
            .collect();

        assert_eq!(
            sol_at_zero[0].simplify(),
            expr!(3),
            "First component at t=0 should be 3"
        );
        assert_eq!(
            sol_at_zero[1].simplify(),
            expr!(4),
            "Second component at t=0 should be 4"
        );
    }

    #[test]
    fn test_2x2_system_zero_initial_conditions() {
        let t = symbol!(t);
        let matrix = Matrix::diagonal(vec![expr!(1), expr!(2)]);
        let initial_conditions = vec![expr!(0), expr!(0)];

        let solver = LinearSystemSolver;
        let solution = solver.solve(&matrix, &t, Some(initial_conditions));

        assert!(
            solution.is_ok(),
            "Should solve with zero initial conditions"
        );

        let sol = solution.unwrap();
        let mut t_subs = HashMap::new();
        t_subs.insert(t.name().to_string(), expr!(0));

        let sol_at_zero: Vec<Expression> = sol
            .iter()
            .map(|expr| expr.substitute(&t_subs).simplify())
            .collect();

        assert_eq!(
            sol_at_zero[0].simplify(),
            expr!(0),
            "First component at t=0 should be 0"
        );
        assert_eq!(
            sol_at_zero[1].simplify(),
            expr!(0),
            "Second component at t=0 should be 0"
        );
    }

    #[test]
    fn test_wrong_size_initial_conditions() {
        let t = symbol!(t);
        let matrix = Matrix::diagonal(vec![expr!(1), expr!(2)]);
        let initial_conditions = vec![expr!(1), expr!(2), expr!(3)];

        let solver = LinearSystemSolver;
        let result = solver.solve(&matrix, &t, Some(initial_conditions));

        assert!(
            result.is_err(),
            "Should reject mismatched initial condition size"
        );
    }
}
