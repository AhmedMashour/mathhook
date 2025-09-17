//! Matrix equation solver for noncommutative algebra
//!
//! Handles equations involving matrices, operators, and quaternions where
//! multiplication order matters. Distinguishes between left and right division.
//!
//! # Mathematical Background: Why Order Matters in Matrix Equations
//!
//! In commutative algebra (scalars), multiplication order doesn't matter:
//! - `a * b = b * a`
//! - `a * x = b` can be solved as `x = b / a = b * (1/a) = (1/a) * b`
//!
//! But in noncommutative algebra (matrices, operators, quaternions), order is critical:
//! - `A * B ≠ B * A` (in general)
//! - Division must distinguish LEFT from RIGHT
//!
//! ## Left Division: A*X = B
//!
//! To solve `A*X = B` for X, we multiply both sides by A^(-1) on the LEFT:
//!
//! ```text
//! A*X = B
//! A^(-1) * (A*X) = A^(-1) * B    // Multiply left by A^(-1)
//! (A^(-1) * A) * X = A^(-1) * B  // Associativity
//! I * X = A^(-1) * B             // A^(-1)*A = I
//! X = A^(-1) * B                 // Solution
//! ```
//!
//! ## Right Division: X*A = B
//!
//! To solve `X*A = B` for X, we multiply both sides by A^(-1) on the RIGHT:
//!
//! ```text
//! X*A = B
//! (X*A) * A^(-1) = B * A^(-1)    // Multiply right by A^(-1)
//! X * (A*A^(-1)) = B * A^(-1)    // Associativity
//! X * I = B * A^(-1)             // A*A^(-1) = I
//! X = B * A^(-1)                 // Solution
//! ```
//!
//! ## Why We Can't Swap Order
//!
//! In general, `A^(-1) * B ≠ B * A^(-1)`, so:
//! - Solution to `A*X = B` is `X = A^(-1)*B` (NOT `B*A^(-1)`)
//! - Solution to `X*A = B` is `X = B*A^(-1)` (NOT `A^(-1)*B`)
//!
//! ## Real-World Examples
//!
//! **Linear Algebra**: Solving `A*x = b` for vector x
//! - `A` is coefficient matrix
//! - `x` is unknown vector
//! - `b` is result vector
//! - Solution: `x = A^(-1)*b` (left multiplication)
//!
//! **Quantum Mechanics**: Eigenvalue equations `H*ψ = E*ψ`
//! - `H` is Hamiltonian operator
//! - `ψ` is wavefunction (eigenstate)
//! - `E` is energy (eigenvalue, commutative)
//!
//! **Quaternions**: 3D rotations `q*v*conj(q)`
//! - `q` is rotation quaternion
//! - `v` is vector (as quaternion)
//! - Order matters: `q*v ≠ v*q`

use crate::algebra::solvers::{EquationSolver, SolverError, SolverResult};
use crate::core::commutativity::Commutativity;
use crate::core::{Expression, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use crate::simplify::Simplify;

/// Matrix equation solver specialized for noncommutative types
///
/// Handles equations of the form:
/// - Left multiplication: A*X = B (solution: X = A^(-1)*B)
/// - Right multiplication: X*A = B (solution: X = B*A^(-1))
///
/// # Examples
///
/// ```rust,ignore
/// use mathhook_core::{symbol, expr};
/// use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
/// use mathhook_core::algebra::solvers::EquationSolver;
///
/// let solver = MatrixEquationSolver::new();
/// let A = symbol!(A; matrix);
/// let B = symbol!(B; matrix);
/// let X = symbol!(X; matrix);
///
/// // Solve A*X = B for X
/// let equation = expr!((A*X) - B);
/// let result = solver.solve(&equation, &X);
/// ```
#[derive(Debug, Clone)]
pub struct MatrixEquationSolver {
    pub show_steps: bool,
}

impl MatrixEquationSolver {
    /// Create a new matrix equation solver
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
    ///
    /// let solver = MatrixEquationSolver::new();
    /// ```
    pub fn new() -> Self {
        Self { show_steps: true }
    }

    /// Create solver without step-by-step explanations (for performance)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
    ///
    /// let solver = MatrixEquationSolver::new_fast();
    /// ```
    pub fn new_fast() -> Self {
        Self { show_steps: false }
    }

    /// Detect if equation is left division (A*X = B)
    ///
    /// Returns Some((A, B)) if equation is A*X = B, None otherwise
    fn detect_left_division(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> Option<(Expression, Expression)> {
        let simplified = equation.simplify();

        match &simplified {
            // Pattern: A*X - B = 0
            Expression::Add(terms) if terms.len() == 2 => {
                // Look for pattern: Mul(A, X) and -B
                match (&terms[0], &terms[1]) {
                    (Expression::Mul(factors), b) if factors.len() == 2 => {
                        if let [a, Expression::Symbol(x)] = &factors[..] {
                            if x == variable && !a.contains_variable(variable) {
                                // Found A*X - B pattern
                                let neg_b =
                                    Expression::mul(vec![Expression::integer(-1), b.clone()]);
                                return Some((a.clone(), neg_b.simplify()));
                            }
                        }
                        None
                    }
                    _ => None,
                }
            }
            // Pattern: A*X = 0 (already simplified, b=0 implicit)
            Expression::Mul(factors) if factors.len() == 2 => {
                if let [a, Expression::Symbol(x)] = &factors[..] {
                    if x == variable && !a.contains_variable(variable) {
                        return Some((a.clone(), Expression::integer(0)));
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Detect if equation is right division (X*A = B)
    ///
    /// Returns Some((A, B)) if equation is X*A = B, None otherwise
    fn detect_right_division(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> Option<(Expression, Expression)> {
        let simplified = equation.simplify();

        match &simplified {
            // Pattern: X*A - B = 0
            Expression::Add(terms) if terms.len() == 2 => {
                // Look for pattern: Mul(X, A) and -B
                match (&terms[0], &terms[1]) {
                    (Expression::Mul(factors), b) if factors.len() == 2 => {
                        if let [Expression::Symbol(x), a] = &factors[..] {
                            if x == variable && !a.contains_variable(variable) {
                                // Found X*A - B pattern
                                let neg_b =
                                    Expression::mul(vec![Expression::integer(-1), b.clone()]);
                                return Some((a.clone(), neg_b.simplify()));
                            }
                        }
                        None
                    }
                    _ => None,
                }
            }
            // Pattern: X*A = 0 (already simplified, b=0 implicit)
            Expression::Mul(factors) if factors.len() == 2 => {
                if let [Expression::Symbol(x), a] = &factors[..] {
                    if x == variable && !a.contains_variable(variable) {
                        return Some((a.clone(), Expression::integer(0)));
                    }
                }
                None
            }
            _ => None,
        }
    }

    /// Solve left division: A*X = B → X = A^(-1)*B
    ///
    /// # Arguments
    ///
    /// * `A` - The left coefficient matrix/operator
    /// * `B` - The right-hand side
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::{symbol, expr};
    /// use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
    ///
    /// let solver = MatrixEquationSolver::new();
    /// let A = symbol!(A; matrix);
    /// let B = symbol!(B; matrix);
    ///
    /// let solution = solver.solve_left_division(&A, &B);
    /// // solution should be A^(-1)*B
    /// ```
    pub fn solve_left_division(
        &self,
        a: &Expression,
        b: &Expression,
    ) -> Result<Expression, SolverError> {
        // Check if A is potentially singular (for matrices)
        if self.is_zero_matrix(a) {
            return Err(SolverError::InvalidEquation(
                "Cannot invert zero matrix".to_owned(),
            ));
        }

        // X = A^(-1) * B (left multiplication)
        let a_inv = Expression::pow(a.clone(), Expression::integer(-1));
        let solution = Expression::mul(vec![a_inv, b.clone()]);

        Ok(solution.simplify())
    }

    /// Solve right division: X*A = B → X = B*A^(-1)
    ///
    /// # Arguments
    ///
    /// * `A` - The right coefficient matrix/operator
    /// * `B` - The right-hand side
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::{symbol, expr};
    /// use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
    ///
    /// let solver = MatrixEquationSolver::new();
    /// let A = symbol!(A; matrix);
    /// let B = symbol!(B; matrix);
    ///
    /// let solution = solver.solve_right_division(&A, &B);
    /// // solution should be B*A^(-1)
    /// ```
    pub fn solve_right_division(
        &self,
        a: &Expression,
        b: &Expression,
    ) -> Result<Expression, SolverError> {
        // Check if A is potentially singular (for matrices)
        if self.is_zero_matrix(a) {
            return Err(SolverError::InvalidEquation(
                "Cannot invert zero matrix".to_owned(),
            ));
        }

        // X = B * A^(-1) (right multiplication)
        let a_inv = Expression::pow(a.clone(), Expression::integer(-1));
        let solution = Expression::mul(vec![b.clone(), a_inv]);

        Ok(solution.simplify())
    }

    /// Check if expression represents a zero matrix
    fn is_zero_matrix(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Number(n) if n.is_zero() => true,
            Expression::Matrix(m) => {
                let (rows, cols) = m.dimensions();
                for i in 0..rows {
                    for j in 0..cols {
                        let elem = m.get_element(i, j);
                        if !elem.is_zero() {
                            return false;
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }

    /// Detect if variable appears in multiple positions (error case)
    fn variable_appears_multiple_times(&self, expr: &Expression, variable: &Symbol) -> bool {
        let count = expr.count_variable_occurrences(variable);
        count > 1
    }
}

impl Default for MatrixEquationSolver {
    fn default() -> Self {
        Self::new()
    }
}

impl EquationSolver for MatrixEquationSolver {
    fn solve(&self, equation: &Expression, variable: &Symbol) -> SolverResult {
        // Check if variable appears multiple times (error case for noncommutative)
        if self.variable_appears_multiple_times(equation, variable) {
            return SolverResult::NoSolution;
        }

        // Try left division first
        if let Some((a, b)) = self.detect_left_division(equation, variable) {
            match self.solve_left_division(&a, &b) {
                Ok(solution) => return SolverResult::Single(solution),
                Err(_) => return SolverResult::NoSolution,
            }
        }

        // Try right division
        if let Some((a, b)) = self.detect_right_division(equation, variable) {
            match self.solve_right_division(&a, &b) {
                Ok(solution) => return SolverResult::Single(solution),
                Err(_) => return SolverResult::NoSolution,
            }
        }

        SolverResult::NoSolution
    }

    fn solve_with_explanation(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (SolverResult, StepByStepExplanation) {
        let mut steps = vec![Step::new(
            "Given Equation",
            format!("Solve {} = 0 for {}", equation, variable.name),
        )];

        // Check commutativity
        if equation.commutativity() == Commutativity::Commutative {
            steps.push(Step::new(
                "Analysis",
                "All symbols are commutative - use standard linear solver instead",
            ));
            return (SolverResult::NoSolution, StepByStepExplanation::new(steps));
        }

        steps.push(Step::new(
            "Analysis",
            "Detected noncommutative symbols (matrix/operator/quaternion)",
        ));

        // Try left division
        if let Some((a, b)) = self.detect_left_division(equation, variable) {
            steps.push(Step::new(
                "Pattern",
                format!(
                    "Identified left division: {} * {} = {}",
                    a, variable.name, b
                ),
            ));
            steps.push(Step::new(
                "Solution Method",
                format!(
                    "{} = {}^(-1) * {} (inverse applied on LEFT)",
                    variable.name, a, b
                ),
            ));

            match self.solve_left_division(&a, &b) {
                Ok(solution) => {
                    steps.push(Step::new(
                        "Result",
                        format!("{} = {}", variable.name, solution),
                    ));
                    return (
                        SolverResult::Single(solution),
                        StepByStepExplanation::new(steps),
                    );
                }
                Err(e) => {
                    steps.push(Step::new("Error", format!("{:?}", e)));
                    return (SolverResult::NoSolution, StepByStepExplanation::new(steps));
                }
            }
        }

        // Try right division
        if let Some((a, b)) = self.detect_right_division(equation, variable) {
            steps.push(Step::new(
                "Pattern",
                format!(
                    "Identified right division: {} * {} = {}",
                    variable.name, a, b
                ),
            ));
            steps.push(Step::new(
                "Solution Method",
                format!(
                    "{} = {} * {}^(-1) (inverse applied on RIGHT)",
                    variable.name, b, a
                ),
            ));

            match self.solve_right_division(&a, &b) {
                Ok(solution) => {
                    steps.push(Step::new(
                        "Result",
                        format!("{} = {}", variable.name, solution),
                    ));
                    return (
                        SolverResult::Single(solution),
                        StepByStepExplanation::new(steps),
                    );
                }
                Err(e) => {
                    steps.push(Step::new("Error", format!("{:?}", e)));
                    return (SolverResult::NoSolution, StepByStepExplanation::new(steps));
                }
            }
        }

        steps.push(Step::new(
            "Result",
            "Could not identify left or right division pattern",
        ));
        (SolverResult::NoSolution, StepByStepExplanation::new(steps))
    }

    fn can_solve(&self, equation: &Expression) -> bool {
        equation.commutativity() != Commutativity::Commutative
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_left_division_detection() {
        let solver = MatrixEquationSolver::new();
        let a = symbol!(A; matrix);
        let x = symbol!(X; matrix);
        let b = symbol!(B; matrix);

        // A*X - B = 0
        let equation = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(a.clone()),
                Expression::symbol(x.clone()),
            ]),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
        ]);

        let result = solver.detect_left_division(&equation, &x);
        assert!(result.is_some());
    }

    #[test]
    fn test_right_division_detection() {
        let solver = MatrixEquationSolver::new();
        let a = symbol!(A; matrix);
        let x = symbol!(X; matrix);
        let b = symbol!(B; matrix);

        // X*A - B = 0
        let equation = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(a.clone()),
            ]),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
        ]);

        let result = solver.detect_right_division(&equation, &x);
        assert!(result.is_some());
    }
}
