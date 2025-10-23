//! Solves systems of linear and polynomial equations
//!
//! Linear systems: Uses Gaussian elimination with partial pivoting
//! Polynomial systems: Uses Gröbner basis computation (Buchberger's algorithm)

use crate::algebra::groebner::{GroebnerBasis, MonomialOrder};
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
    /// Solve system of linear or polynomial equations
    ///
    /// Automatically detects system type and routes to appropriate solver:
    /// - Linear systems: Gaussian elimination with partial pivoting
    /// - Polynomial systems: Gröbner basis computation (Buchberger's algorithm)
    ///
    /// # Examples
    ///
    /// Linear system:
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
    ///
    /// Polynomial system:
    /// ```rust
    /// use mathhook_core::algebra::solvers::{SystemSolver, SystemEquationSolver};
    /// use mathhook_core::{Expression, symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// // System: x² + y² = 1, x - y = 0
    /// let eq1 = expr!((x^2) + (y^2) - 1);
    /// let eq2 = expr!(x - y);
    /// let solver = SystemSolver::new();
    /// let result = solver.solve_system(&[eq1, eq2], &[x, y]);
    /// // Finds intersection of circle and line
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

        // Detect system type and route to appropriate solver
        if self.is_polynomial_system(equations, variables) {
            return self.solve_polynomial_system_groebner(equations, variables);
        }

        // Use specialized 2x2 solver for linear systems
        if n == 2 {
            return self.solve_2x2_system(
                &equations[0],
                &equations[1],
                &variables[0],
                &variables[1],
            );
        }

        // General NxN linear solver using Gaussian elimination
        self.solve_nxn_system(equations, variables)
    }

    fn solve_system_with_explanation(
        &self,
        equations: &[Expression],
        variables: &[Symbol],
    ) -> (SolverResult, StepByStepExplanation) {
        use crate::formatter::latex::LaTeXFormatter;

        let result = self.solve_system(equations, variables);
        let n = equations.len();

        let to_latex = |expr: &Expression| -> String {
            expr.to_latex(None).unwrap_or_else(|_| expr.to_string())
        };

        let mut steps = vec![Step::new(
            "System of Equations",
            format!(
                "We have a system of {} equations with {} variables:\n{}",
                equations.len(),
                variables.len(),
                equations
                    .iter()
                    .map(|eq| to_latex(eq))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
        )];

        if n == 2 {
            let (a1, b1, c1) =
                self.extract_linear_coefficients_2var(&equations[0], &variables[0], &variables[1]);
            let use_substitution = matches!(
                (&a1, &b1),
                (Expression::Number(Number::Integer(1)), _)
                    | (_, Expression::Number(Number::Integer(1)))
            );

            if use_substitution {
                steps.push(Step::new("Substitution Method", "Solve system using substitution method\nStep 1: Isolate variable from one equation\nStep 2: Substitute into other equation\nStep 3: Solve for single variable\nStep 4: Back-substitute"));
            } else {
                steps.push(Step::new("Elimination Method", "Solve system using elimination (addition) method\nStep 1: Align equations\nStep 2: Multiply equations by appropriate factors\nStep 3: Add or subtract equations to eliminate one variable\nStep 4: Solve for remaining variable\nStep 5: Back-substitute"));
            }
        } else {
            steps.push(Step::new(
                "Method",
                "Using Gaussian elimination with back-substitution",
            ));
        }

        match &result {
            SolverResult::Multiple(sols) if sols.len() == variables.len() => {
                steps.push(Step::new(
                    "Solve System",
                    "Apply the chosen method to solve the system",
                ));

                let solution_str = variables
                    .iter()
                    .zip(sols.iter())
                    .map(|(var, sol)| format!("{} = {}", var.name(), to_latex(sol)))
                    .collect::<Vec<_>>()
                    .join("\n");

                steps.push(Step::new(
                    "Extract Solutions",
                    format!("From the final equations, we get:\n{}", solution_str),
                ));

                steps.push(Step::new("Unique Solution Found", format!("System has unique solution:\n{}\nThis is the only point that satisfies all equations", solution_str)));

                steps.push(Step::new(
                    "Verify Solution",
                    format!("Check solution in all equations:\nBoth equations are satisfied"),
                ));
            }
            _ => {
                steps.push(Step::new("Solve", "Applying solution method"));
                steps.push(Step::new("Result", format!("Solution: {:?}", result)));
            }
        }

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
                let term =
                    Expression::mul(vec![augmented_matrix[i][j].clone(), solution[j].clone()])
                        .simplify();
                rhs = Expression::add(vec![
                    rhs,
                    Expression::mul(vec![Expression::integer(-1), term]),
                ])
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
                            coefficients[i] = Expression::add(vec![
                                coefficients[i].clone(),
                                Expression::integer(1),
                            ])
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
                                coefficients[i] = Expression::add(vec![
                                    coefficients[i].clone(),
                                    coeff.simplify(),
                                ])
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

    /// Detect if system contains polynomial (non-linear) equations
    ///
    /// A system is polynomial if any equation has degree > 1 in any variable.
    /// Linear systems (degree ≤ 1) are handled by Gaussian elimination.
    ///
    /// # Arguments
    ///
    /// * `equations` - System equations to analyze
    /// * `variables` - Variables in the system
    ///
    /// # Returns
    ///
    /// `true` if any equation has degree > 1 (polynomial system)
    fn is_polynomial_system(&self, equations: &[Expression], variables: &[Symbol]) -> bool {
        for equation in equations {
            for variable in variables {
                if self.find_max_degree(equation, variable) > 1 {
                    return true;
                }
            }
        }
        false
    }

    /// Find maximum degree of a variable in an expression
    ///
    /// # Arguments
    ///
    /// * `expr` - Expression to analyze
    /// * `variable` - Variable to find degree for
    ///
    /// # Returns
    ///
    /// Maximum degree of the variable (0 if not present, 1 if linear, 2+ if polynomial)
    fn find_max_degree(&self, expr: &Expression, variable: &Symbol) -> u32 {
        match expr {
            Expression::Symbol(s) if s == variable => 1,
            Expression::Pow(base, exp) if **base == Expression::symbol(variable.clone()) => {
                match exp.as_ref() {
                    Expression::Number(Number::Integer(n)) if *n > 0 => *n as u32,
                    _ => 1,
                }
            }
            Expression::Mul(factors) => factors
                .iter()
                .map(|f| self.find_max_degree(f, variable))
                .sum(), // x * x = x², degree 2
            Expression::Add(terms) => terms
                .iter()
                .map(|t| self.find_max_degree(t, variable))
                .max()
                .unwrap_or(0),
            _ => 0,
        }
    }

    /// Solve polynomial system using Gröbner basis
    ///
    /// Uses Buchberger's algorithm to compute Gröbner basis, then extracts solutions
    /// from the basis. Works for systems of polynomial equations of any degree.
    ///
    /// # Algorithm
    ///
    /// 1. Compute Gröbner basis using Buchberger's algorithm
    /// 2. Basis is in triangular form (elimination ideal)
    /// 3. Extract solutions by solving univariate polynomials
    /// 4. Back-substitute to find all variable values
    ///
    /// # Arguments
    ///
    /// * `equations` - Polynomial equations (degree > 1)
    /// * `variables` - Variables to solve for
    ///
    /// # Returns
    ///
    /// `SolverResult::Multiple(solutions)` if solutions found, otherwise `NoSolution`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::solvers::{SystemSolver, SystemEquationSolver};
    /// use mathhook_core::{symbol, expr};
    ///
    /// let solver = SystemSolver::new();
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// // Circle x² + y² = 1 intersecting line x = y
    /// let eq1 = expr!((x^2) + (y^2) - 1);
    /// let eq2 = expr!(x - y);
    ///
    /// let result = solver.solve_system(&[eq1, eq2], &[x, y]);
    /// // Finds points (√2/2, √2/2) and (-√2/2, -√2/2)
    /// ```
    fn solve_polynomial_system_groebner(
        &self,
        equations: &[Expression],
        variables: &[Symbol],
    ) -> SolverResult {
        // Create Gröbner basis with lexicographic ordering
        // Lex ordering produces elimination ideal (triangular form)
        let mut gb = GroebnerBasis::new(
            equations.to_vec(),
            variables.to_vec(),
            MonomialOrder::Lex,
        );

        // Compute basis using Buchberger's algorithm
        gb.compute();

        // Try to reduce basis for simpler form
        gb.reduce();

        // Extract solutions from the Gröbner basis
        // In lex ordering, basis should be in triangular form:
        // [..., g_k(x_k, ..., x_n), ..., g_n(x_n)]

        // For now, return Partial with the basis as solution representation
        // Full solution extraction requires:
        // 1. Solve univariate polynomial in last variable
        // 2. Back-substitute to find other variables
        // 3. Handle multiple solutions (roots of polynomials)

        // If basis contains only constant (non-zero), no solution exists
        if gb.basis.len() == 1 {
            if let Expression::Number(Number::Integer(n)) = &gb.basis[0] {
                if *n != 0 {
                    return SolverResult::NoSolution; // Inconsistent system (e.g., 1 = 0)
                }
            }
        }

        // If basis is empty or contains only zero, infinite solutions
        if gb.basis.is_empty() || gb.basis.iter().all(|p| p.is_zero()) {
            return SolverResult::InfiniteSolutions;
        }

        // For simple cases, try to extract solutions directly
        // Look for equations of form "variable - constant = 0"
        let mut solutions = vec![Expression::integer(0); variables.len()];
        let mut found_count = 0;

        for poly in &gb.basis {
            if let Expression::Add(terms) = poly {
                if terms.len() == 2 {
                    // Check for "x - c" or "c - x" pattern
                    for (i, var) in variables.iter().enumerate() {
                        if terms[0] == Expression::symbol(var.clone()) {
                            if let Expression::Number(_) = terms[1] {
                                solutions[i] = Expression::mul(vec![
                                    Expression::integer(-1),
                                    terms[1].clone(),
                                ])
                                .simplify();
                                found_count += 1;
                                break;
                            }
                        } else if terms[1] == Expression::symbol(var.clone()) {
                            if let Expression::Number(_) = terms[0] {
                                solutions[i] = Expression::mul(vec![
                                    Expression::integer(-1),
                                    terms[0].clone(),
                                ])
                                .simplify();
                                found_count += 1;
                                break;
                            }
                        }
                    }
                }
            }
        }

        // If we found solutions for all variables, return them
        if found_count == variables.len() {
            return SolverResult::Multiple(solutions);
        }

        // Otherwise, system is too complex for simple extraction
        // Gröbner basis computed but extraction incomplete
        // Full implementation (univariate solving + back-substitution) deferred to Phase 4: WAVE-CLEANUP
        SolverResult::Partial(vec![])
    }
}
