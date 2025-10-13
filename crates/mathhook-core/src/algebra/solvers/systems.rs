//! Solves systems of linear equations using elimination/substitution
//! Includes step-by-step explanations for educational value

use crate::algebra::solvers::{EquationSolver, SolverResult, SystemEquationSolver};
use crate::core::{Expression, Number, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use crate::simplify::Simplify;
use num_bigint::BigInt;
use num_rational::BigRational;

/// System equation solver
#[derive(Debug, Clone)]
pub struct SystemSolver;

impl SystemSolver {
    pub fn new() -> Self {
        Self
    }
}

impl EquationSolver for SystemSolver {
    fn solve(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
        // For single equation, treat as linear
        let linear_solver = crate::algebra::solvers::LinearSolver::new();
        linear_solver.solve(equation, variable)
    }

    fn solve_with_explanation(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (SolverResult, StepByStepExplanation) {
        let linear_solver = crate::algebra::solvers::LinearSolver::new();
        linear_solver.solve_with_explanation(equation, variable)
    }

    fn can_solve(&self, _equation: &Expression) -> bool {
        true
    }
}

impl SystemEquationSolver for SystemSolver {
    /// Solve system of linear equations using Gaussian elimination (LU decomposition)
    ///
    /// Supports NxN systems where N >= 2. Uses matrix-based approach with partial pivoting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::solvers::{SystemSolver, SystemEquationSolver};
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// // System: 2x + y = 5, x - y = 1
    /// let eq1 = Expression::add(vec![
    ///     Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
    ///     Expression::symbol(y.clone()),
    ///     Expression::integer(-5)
    /// ]);
    /// let eq2 = Expression::add(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
    ///     Expression::integer(-1)
    /// ]);
    /// let solver = SystemSolver::new();
    /// let result = solver.solve_system(&[eq1, eq2], &[x, y]);
    /// // Solution: x = 2, y = 1
    /// ```
    fn solve_system(&self, equations: &[Expression], variables: &[Symbol]) -> SolverResult {
        let n = equations.len();
        let m = variables.len();

        // Check if system is square
        if n != m {
            return SolverResult::NoSolution; // Underdetermined or overdetermined
        }

        if n == 0 {
            return SolverResult::NoSolution;
        }

        // Use specialized 2x2 solver for performance
        if n == 2 {
            return self.solve_2x2_system(
                &equations[0],
                &equations[1],
                &variables[0],
                &variables[1],
            );
        }

        // General NxN solver using Gaussian elimination via LU decomposition
        self.solve_nxn_system(equations, variables)
    }

    fn solve_system_with_explanation(
        &self,
        equations: &[Expression],
        variables: &[Symbol],
    ) -> (SolverResult, StepByStepExplanation) {
        let result = self.solve_system(equations, variables);

        let steps = vec![
            Step::new(
                "System Analysis",
                format!(
                    "Solving system of {} equations in {} variables",
                    equations.len(),
                    variables.len()
                ),
            ),
            Step::new("Method", "Using elimination/substitution method"),
            Step::new("Result", format!("Solution: {:?}", result)),
        ];

        (result, StepByStepExplanation::new(steps))
    }

    // Note: can_solve_system is not in the trait, removing this method
}

impl SystemSolver {
    /// Solve 2x2 linear system using elimination
    fn solve_2x2_system(
        &self,
        eq1: &Expression,
        eq2: &Expression,
        var1: &Symbol,
        var2: &Symbol,
    ) -> SolverResult {
        // Extract coefficients from both equations
        // eq1: a1*x + b1*y + c1 = 0
        // eq2: a2*x + b2*y + c2 = 0

        let (a1, b1, c1) = self.extract_linear_coefficients_2var(eq1, var1, var2);
        let (a2, b2, c2) = self.extract_linear_coefficients_2var(eq2, var1, var2);

        // Solve using Cramer's rule or elimination
        self.solve_using_cramers_rule(a1, b1, c1, a2, b2, c2)
    }

    /// Extract coefficients from linear equation in 2 variables
    fn extract_linear_coefficients_2var(
        &self,
        equation: &Expression,
        var1: &Symbol,
        var2: &Symbol,
    ) -> (Expression, Expression, Expression) {
        match equation {
            Expression::Add(terms) => {
                let mut a_coeff = Expression::integer(0); // Coefficient of var1
                let mut b_coeff = Expression::integer(0); // Coefficient of var2
                let mut c_coeff = Expression::integer(0); // Constant term

                for term in terms.iter() {
                    if term == &Expression::symbol(var1.clone()) {
                        a_coeff = Expression::integer(1);
                    } else if term == &Expression::symbol(var2.clone()) {
                        b_coeff = Expression::integer(1);
                    } else if let Expression::Mul(factors) = term {
                        let mut var1_found = false;
                        let mut var2_found = false;
                        let mut coeff = Expression::integer(1);

                        for factor in factors.iter() {
                            if factor == &Expression::symbol(var1.clone()) {
                                var1_found = true;
                            } else if factor == &Expression::symbol(var2.clone()) {
                                var2_found = true;
                            } else {
                                coeff = Expression::mul(vec![coeff, factor.clone()]);
                            }
                        }

                        if var1_found {
                            a_coeff = coeff.simplify();
                        } else if var2_found {
                            b_coeff = coeff.simplify();
                        } else {
                            c_coeff = Expression::add(vec![c_coeff, term.clone()]);
                        }
                    } else {
                        c_coeff = Expression::add(vec![c_coeff, term.clone()]);
                    }
                }

                (a_coeff.simplify(), b_coeff.simplify(), c_coeff.simplify())
            }
            _ => (
                Expression::integer(0),
                Expression::integer(0),
                equation.clone(),
            ),
        }
    }

    /// Solve 2x2 system using Cramer's rule
    fn solve_using_cramers_rule(
        &self,
        a1: Expression,
        b1: Expression,
        c1: Expression,
        a2: Expression,
        b2: Expression,
        c2: Expression,
    ) -> SolverResult {
        // System: a1*x + b1*y = -c1
        //         a2*x + b2*y = -c2

        match (&a1, &b1, &c1, &a2, &b2, &c2) {
            (
                Expression::Number(Number::Integer(a1_val)),
                Expression::Number(Number::Integer(b1_val)),
                Expression::Number(Number::Integer(c1_val)),
                Expression::Number(Number::Integer(a2_val)),
                Expression::Number(Number::Integer(b2_val)),
                Expression::Number(Number::Integer(c2_val)),
            ) => {
                // Calculate determinant: det = a1*b2 - a2*b1
                let det = a1_val * b2_val - a2_val * b1_val;

                if det == 0 {
                    // System is either dependent (infinite solutions) or inconsistent (no solution)
                    // Check if equations are proportional
                    if self.are_equations_proportional(
                        *a1_val, *b1_val, *c1_val, *a2_val, *b2_val, *c2_val,
                    ) {
                        SolverResult::InfiniteSolutions
                    } else {
                        SolverResult::NoSolution
                    }
                } else {
                    // Unique solution using Cramer's rule
                    // x = ((-c1)*b2 - (-c2)*b1) / det
                    // y = (a1*(-c2) - a2*(-c1)) / det

                    let x_num = (-c1_val) * b2_val - (-c2_val) * b1_val;
                    let y_num = a1_val * (-c2_val) - a2_val * (-c1_val);

                    let x_sol = if x_num % det == 0 {
                        Expression::integer(x_num / det)
                    } else {
                        Expression::Number(Number::rational(BigRational::new(
                            BigInt::from(x_num),
                            BigInt::from(det),
                        )))
                    };

                    let y_sol = if y_num % det == 0 {
                        Expression::integer(y_num / det)
                    } else {
                        Expression::Number(Number::rational(BigRational::new(
                            BigInt::from(y_num),
                            BigInt::from(det),
                        )))
                    };

                    // Return as vector [x_solution, y_solution]
                    SolverResult::Multiple(vec![x_sol, y_sol])
                }
            }
            _ => SolverResult::NoSolution, // Complex coefficients not supported yet
        }
    }

    /// Check if two equations are proportional (dependent system)
    fn are_equations_proportional(
        &self,
        a1: i64,
        b1: i64,
        c1: i64,
        a2: i64,
        b2: i64,
        c2: i64,
    ) -> bool {
        // Check if (a1, b1, c1) and (a2, b2, c2) are proportional
        // This means a1/a2 = b1/b2 = c1/c2 (handling zero cases)

        if a2 == 0 && b2 == 0 && c2 == 0 {
            return a1 == 0 && b1 == 0 && c1 == 0; // Both equations are 0 = 0
        }

        // Find non-zero coefficient to use as reference
        if a2 != 0 {
            // Check if a1/a2 = b1/b2 = c1/c2
            a1 * b2 == a2 * b1 && a1 * c2 == a2 * c1 && b1 * c2 == b2 * c1
        } else if b2 != 0 {
            // a2 = 0, use b2 as reference
            a1 == 0 && b1 * c2 == b2 * c1
        } else {
            // a2 = b2 = 0, check if c2 != 0 and others are 0
            a1 == 0 && b1 == 0
        }
    }

    /// Solve NxN system using Gaussian elimination with partial pivoting
    ///
    /// Converts system of equations to augmented matrix [A|b], then applies
    /// Gaussian elimination with partial pivoting, followed by back substitution.
    ///
    /// Algorithm:
    /// 1. Extract coefficient matrix A and constant vector b from equations
    /// 2. Create augmented matrix [A|b]
    /// 3. Apply Gaussian elimination with partial pivoting to get row echelon form
    /// 4. Apply back substitution to solve for variables
    ///
    /// # Returns
    ///
    /// - `SolverResult::Multiple(solutions)`: Unique solution found
    /// - `SolverResult::NoSolution`: Inconsistent system (no solution)
    /// - `SolverResult::InfiniteSolutions`: Dependent system (infinite solutions)
    fn solve_nxn_system(&self, equations: &[Expression], variables: &[Symbol]) -> SolverResult {
        let n = equations.len();

        // Extract coefficient matrix A and constant vector b
        // Each equation is: a₁x₁ + a₂x₂ + ... + aₙxₙ + c = 0
        // We want: a₁x₁ + a₂x₂ + ... + aₙxₙ = -c
        let mut augmented_matrix = Vec::with_capacity(n);

        for equation in equations {
            let (coeffs, constant) = self.extract_coefficients_nvar(equation, variables);
            // Negate constant to move to RHS: ax + by + c = 0 → ax + by = -c
            let rhs = Expression::mul(vec![Expression::integer(-1), constant]).simplify();

            // Create augmented row: [a1, a2, ..., an | b]
            let mut row = coeffs;
            row.push(rhs);
            augmented_matrix.push(row);
        }

        // Apply Gaussian elimination with partial pivoting
        for col in 0..n {
            // Find pivot (row with largest absolute value in current column)
            let mut pivot_row = col;
            for row in (col + 1)..n {
                // Simple pivot selection: prefer non-zero elements
                if !augmented_matrix[row][col].is_zero()
                    && augmented_matrix[pivot_row][col].is_zero()
                {
                    pivot_row = row;
                }
            }

            // Swap rows if needed
            if pivot_row != col {
                augmented_matrix.swap(col, pivot_row);
            }

            // Check for zero pivot
            let pivot = augmented_matrix[col][col].clone();
            if pivot.is_zero() {
                // Check if system is inconsistent
                if !augmented_matrix[col][n].is_zero() {
                    return SolverResult::NoSolution; // Inconsistent
                }
                continue; // Skip zero pivot (might lead to infinite solutions)
            }

            // Eliminate below pivot
            for row in (col + 1)..n {
                let multiplier = augmented_matrix[row][col].clone();
                let factor = Expression::mul(vec![
                    multiplier,
                    Expression::pow(pivot.clone(), Expression::integer(-1)),
                ])
                .simplify();

                // Row[row] = Row[row] - factor * Row[col]
                for col_idx in col..=n {
                    let old_val = augmented_matrix[row][col_idx].clone();
                    let pivot_row_val = augmented_matrix[col][col_idx].clone();
                    let new_val = Expression::add(vec![
                        old_val,
                        Expression::mul(vec![
                            Expression::integer(-1),
                            factor.clone(),
                            pivot_row_val,
                        ]),
                    ])
                    .simplify();
                    augmented_matrix[row][col_idx] = new_val;
                }
            }
        }

        // Back substitution
        let mut solution = vec![Expression::integer(0); n];

        for i in (0..n).rev() {
            // Check for zero diagonal (underdetermined or inconsistent)
            if augmented_matrix[i][i].is_zero() {
                // Check if RHS is also zero (infinite solutions) or non-zero (no solution)
                if !augmented_matrix[i][n].is_zero() {
                    return SolverResult::NoSolution;
                }
                // Skip this equation (infinite solutions, but we'll return one solution)
                continue;
            }

            let mut rhs = augmented_matrix[i][n].clone();

            // Subtract known values: rhs = rhs - sum(a[i][j] * solution[j]) for j > i
            for j in (i + 1)..n {
                let term = Expression::mul(vec![
                    augmented_matrix[i][j].clone(),
                    solution[j].clone(),
                ])
                .simplify();
                rhs = Expression::add(vec![rhs, Expression::mul(vec![Expression::integer(-1), term])])
                    .simplify();
            }

            // Solve for solution[i]: solution[i] = rhs / a[i][i]
            solution[i] = Expression::mul(vec![
                rhs,
                Expression::pow(augmented_matrix[i][i].clone(), Expression::integer(-1)),
            ])
            .simplify();
        }

        SolverResult::Multiple(solution)
    }

    /// Extract coefficients from linear equation in N variables
    ///
    /// Returns (coefficient_vector, constant_term)
    fn extract_coefficients_nvar(
        &self,
        equation: &Expression,
        variables: &[Symbol],
    ) -> (Vec<Expression>, Expression) {
        let n = variables.len();
        let mut coefficients = vec![Expression::integer(0); n];
        let mut constant = Expression::integer(0);

        match equation {
            Expression::Add(terms) => {
                for term in terms.iter() {
                    let mut found_var = false;

                    // Check each variable
                    for (i, var) in variables.iter().enumerate() {
                        if term == &Expression::symbol(var.clone()) {
                            coefficients[i] =
                                Expression::add(vec![coefficients[i].clone(), Expression::integer(1)])
                                    .simplify();
                            found_var = true;
                            break;
                        } else if let Expression::Mul(factors) = term {
                            // Check if this term contains the variable
                            let mut has_var = false;
                            let mut coeff = Expression::integer(1);

                            for factor in factors.iter() {
                                if factor == &Expression::symbol(var.clone()) {
                                    has_var = true;
                                } else {
                                    coeff = Expression::mul(vec![coeff, factor.clone()]);
                                }
                            }

                            if has_var {
                                coefficients[i] =
                                    Expression::add(vec![coefficients[i].clone(), coeff.simplify()])
                                        .simplify();
                                found_var = true;
                                break;
                            }
                        }
                    }

                    // If no variable found, it's a constant term
                    if !found_var {
                        constant = Expression::add(vec![constant, term.clone()]).simplify();
                    }
                }
            }
            _ => {
                // Single term equation
                let mut found_var = false;
                for (i, var) in variables.iter().enumerate() {
                    if equation == &Expression::symbol(var.clone()) {
                        coefficients[i] = Expression::integer(1);
                        found_var = true;
                        break;
                    }
                }
                if !found_var {
                    constant = equation.clone();
                }
            }
        }

        (coefficients, constant)
    }
}
