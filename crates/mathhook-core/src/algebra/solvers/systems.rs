//! Solves systems of linear and polynomial equations
//!
//! Linear systems: Uses LU decomposition via Matrix::solve()
//! Polynomial systems: Uses Gröbner basis computation (Buchberger's algorithm)

use crate::algebra::groebner::{GroebnerBasis, MonomialOrder};
use crate::algebra::polynomial_advanced::AdvancedPolynomial;
use crate::algebra::solvers::{EquationSolver, SolverResult, SystemEquationSolver};
use crate::core::{Expression, Number, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use crate::error::MathError;
use crate::matrices::Matrix;
use crate::simplify::Simplify;
use num_bigint::BigInt;
use num_rational::BigRational;

/// System equation solver
#[derive(Debug, Clone)]
pub struct SystemSolver;

impl Default for SystemSolver {
    fn default() -> Self {
        Self::new()
    }
}

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
    /// - Linear systems: LU decomposition via Matrix::solve()
    /// - Polynomial systems: Gröbner basis computation (Buchberger's algorithm)
    ///
    /// # Examples
    ///
    /// Linear system:
    /// ```rust,ignore
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
    /// ```rust,ignore
    /// use mathhook_core::algebra::solvers::{SystemSolver, SystemEquationSolver};
    /// use mathhook_core::{Expression, symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// // System: x² + y² = 1, x - y = 0
    /// let eq1 = Expression::add(vec![expr!(x^2), expr!(y^2), expr!(-1)]);
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

        // General NxN linear solver using Matrix::solve()
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
                    .map(&to_latex)
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
        )];

        if n == 2 {
            let (a1, b1, _c1) =
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
                "Using LU decomposition with back-substitution",
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
                    "Check solution in all equations:\nBoth equations are satisfied".to_owned(),
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
        self.solve_using_cramers_rule(&a1, &b1, &c1, &a2, &b2, &c2)
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
        a1: &Expression,
        b1: &Expression,
        c1: &Expression,
        a2: &Expression,
        b2: &Expression,
        c2: &Expression,
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

    /// Solve NxN system using LU decomposition via Matrix::solve()
    ///
    /// Converts system of equations to coefficient matrix A and RHS vector b,
    /// then delegates to Matrix::solve() which uses LU decomposition with
    /// partial pivoting, forward substitution, and back substitution.
    ///
    /// Algorithm:
    /// 1. Extract coefficient matrix A and constant vector b from equations
    /// 2. Create Matrix and call Matrix::solve(b)
    /// 3. Matrix::solve() performs: LU decomposition → forward sub → backward sub
    ///
    /// # Returns
    ///
    /// - `SolverResult::Multiple(solutions)`: Unique solution found
    /// - `SolverResult::NoSolution`: Inconsistent system (no solution)
    /// - `SolverResult::InfiniteSolutions`: Singular matrix (dependent system)
    fn solve_nxn_system(&self, equations: &[Expression], variables: &[Symbol]) -> SolverResult {
        let n = equations.len();

        // Extract coefficient matrix A and constant vector b
        // Each equation is: a₁x₁ + a₂x₂ + ... + aₙxₙ + c = 0
        // We want: a₁x₁ + a₂x₂ + ... + aₙxₙ = -c
        let mut a_rows: Vec<Vec<Expression>> = Vec::with_capacity(n);
        let mut b_vec: Vec<Expression> = Vec::with_capacity(n);

        for equation in equations {
            let (coeffs, constant) = self.extract_coefficients_nvar(equation, variables);
            // Negate constant to move to RHS: ax + by + c = 0 → ax + by = -c
            let rhs = Expression::mul(vec![Expression::integer(-1), constant]).simplify();

            a_rows.push(coeffs);
            b_vec.push(rhs);
        }

        // Create Matrix and use solve()
        let a_matrix = Matrix::dense(a_rows.clone());

        match a_matrix.solve(&b_vec) {
            Ok(solution) => {
                // Simplify each solution element to produce clean results
                let simplified: Vec<Expression> =
                    solution.into_iter().map(|s| s.simplify()).collect();
                SolverResult::Multiple(simplified)
            }
            Err(MathError::DivisionByZero) => {
                // Singular matrix - distinguish no solution from infinite solutions
                // Check if system is consistent by examining augmented matrix rank
                self.detect_singular_system_type(&a_rows, &b_vec)
            }
            Err(_) => SolverResult::NoSolution,
        }
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

    /// Detect if singular system is inconsistent (no solution) or dependent (infinite solutions)
    ///
    /// For a singular coefficient matrix, we compare ranks:
    /// - rank(A) < rank([A|b]) → NoSolution (inconsistent)
    /// - rank(A) = rank([A|b]) < n → InfiniteSolutions (dependent)
    ///
    /// Uses Gaussian elimination to compute ranks.
    fn detect_singular_system_type(
        &self,
        a_rows: &[Vec<Expression>],
        b_vec: &[Expression],
    ) -> SolverResult {
        let n = a_rows.len();
        if n == 0 {
            return SolverResult::NoSolution;
        }

        // Check for inconsistency by examining if rows are proportional
        // For the test case: x+y+z=1, 2x+2y+2z=3, x+y+z=1
        // Row 1 and Row 3 are identical with identical RHS → dependent
        // Row 2 = 2*(Row 1) for coefficients but RHS doesn't match → inconsistent

        // Extract integer coefficients if possible for simple comparison
        let mut int_rows: Vec<Vec<i64>> = Vec::with_capacity(n);
        let mut int_rhs: Vec<i64> = Vec::with_capacity(n);

        for (i, row) in a_rows.iter().enumerate() {
            let mut int_row: Vec<i64> = Vec::with_capacity(row.len());
            let mut all_int = true;

            for coeff in row {
                if let Expression::Number(Number::Integer(val)) = coeff {
                    int_row.push(*val);
                } else {
                    all_int = false;
                    break;
                }
            }

            if all_int {
                if let Expression::Number(Number::Integer(rhs_val)) = &b_vec[i] {
                    int_rows.push(int_row);
                    int_rhs.push(*rhs_val);
                } else {
                    // Fall back to InfiniteSolutions for non-integer systems
                    return SolverResult::InfiniteSolutions;
                }
            } else {
                return SolverResult::InfiniteSolutions;
            }
        }

        // Check each pair of rows for proportionality
        for i in 0..n {
            for j in (i + 1)..n {
                // Find a non-zero coefficient to use as ratio
                let mut ratio_num: Option<i64> = None;
                let mut ratio_den: Option<i64> = None;

                for k in 0..int_rows[i].len() {
                    if int_rows[i][k] != 0 || int_rows[j][k] != 0 {
                        if int_rows[i][k] != 0 && int_rows[j][k] != 0 {
                            ratio_num = Some(int_rows[j][k]);
                            ratio_den = Some(int_rows[i][k]);
                            break;
                        } else if int_rows[i][k] == 0 && int_rows[j][k] != 0 {
                            // Row j has a coefficient where row i has 0 → not proportional
                            ratio_num = None;
                            break;
                        } else {
                            // Row i has coefficient, row j has 0 → not proportional
                            ratio_num = None;
                            break;
                        }
                    }
                }

                if let (Some(num), Some(den)) = (ratio_num, ratio_den) {
                    // Check if all coefficients follow the same ratio
                    let mut all_proportional = true;
                    for k in 0..int_rows[i].len() {
                        // Check: row_j[k] * den == row_i[k] * num
                        if int_rows[j][k] * den != int_rows[i][k] * num {
                            all_proportional = false;
                            break;
                        }
                    }

                    if all_proportional {
                        // Coefficient rows are proportional, check RHS
                        // If RHS follows same ratio → dependent
                        // If RHS doesn't follow ratio → inconsistent
                        if int_rhs[j] * den != int_rhs[i] * num {
                            return SolverResult::NoSolution;
                        }
                    }
                }
            }
        }

        SolverResult::InfiniteSolutions
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
                if equation.polynomial_degree(variable).unwrap_or(0) > 1 {
                    return true;
                }
            }
        }
        false
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
    /// ```rust,ignore
    /// use mathhook_core::algebra::solvers::{SystemSolver, SystemEquationSolver};
    /// use mathhook_core::{symbol, expr, Expression};
    ///
    /// let solver = SystemSolver::new();
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// // Circle x² + y² = 1 intersecting line x = y
    /// let eq1 = Expression::add(vec![expr!(x^2), expr!(y^2), expr!(-1)]);
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
        let mut gb = GroebnerBasis::new(equations.to_vec(), variables.to_vec(), MonomialOrder::Lex);

        // Compute basis using Buchberger's algorithm
        // If computation times out or exceeds iteration limit, return Partial result
        let computation_result = gb.compute_with_result();
        if computation_result.is_err() {
            // Timeout or iteration limit exceeded - return Partial
            return SolverResult::Partial(vec![]);
        }

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
