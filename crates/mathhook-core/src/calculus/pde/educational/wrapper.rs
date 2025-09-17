//! Educational PDE solver wrapper
//!
//! Wraps PDE solvers to capture intermediate steps and provide
//! human-readable explanations of the solution process.

use crate::algebra::solvers::SolverResult;
use crate::calculus::pde::registry::PDESolverRegistry;
use crate::core::{Expression, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};

/// Educational PDE solver that provides step-by-step explanations
///
/// # Examples
///
/// ```
/// use mathhook_core::calculus::pde::educational::wrapper::EducationalPDESolver;
/// use mathhook_core::{symbol, expr};
///
/// let solver = EducationalPDESolver::new();
/// let u = symbol!(u);
/// let x = symbol!(x);
/// let t = symbol!(t);
///
/// let equation = expr!(u + x + t);
/// let (result, explanation) = solver.solve_pde(&equation, &u, &[x, t]);
///
/// assert!(!explanation.steps.is_empty());
/// ```
pub struct EducationalPDESolver {
    registry: PDESolverRegistry,
}

impl EducationalPDESolver {
    /// Create a new educational PDE solver
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::calculus::pde::educational::wrapper::EducationalPDESolver;
    ///
    /// let solver = EducationalPDESolver::new();
    /// ```
    pub fn new() -> Self {
        Self {
            registry: PDESolverRegistry::new(),
        }
    }

    /// Detect if equation contains partial derivatives (PDE indicator)
    fn is_pde(equation: &Expression) -> bool {
        equation.to_string().contains("∂") || has_multiple_variables(equation)
    }

    /// Solve PDE with detailed educational steps
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::calculus::pde::educational::wrapper::EducationalPDESolver;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let solver = EducationalPDESolver::new();
    /// let u = symbol!(u);
    /// let x = symbol!(x);
    /// let t = symbol!(t);
    ///
    /// let equation = expr!(u + x);
    /// let (result, explanation) = solver.solve_pde(&equation, &u, &[x, t]);
    ///
    /// // Educational explanation is always provided
    /// assert!(!explanation.steps.is_empty());
    /// ```
    pub fn solve_pde(
        &self,
        equation: &Expression,
        variable: &Symbol,
        independent_vars: &[Symbol],
    ) -> (SolverResult, StepByStepExplanation) {
        use crate::calculus::pde::types::Pde;

        let pde = Pde::new(
            equation.clone(),
            variable.clone(),
            independent_vars.to_vec(),
        );

        let mut steps = vec![Step::new(
            "PDE Classification",
            "Analyzing partial differential equation structure",
        )];

        if let Ok(pde_type) = crate::calculus::pde::classification::classify_pde(&pde) {
            steps.push(Step::new(
                "PDE Type Detected",
                format!("This is a {:?} PDE", pde_type),
            ));
        }

        match self.registry.solve(&pde) {
            Ok(solution) => {
                steps.push(Step::new("Solution Found", "PDE solved successfully"));
                steps.push(Step::new(
                    "General Solution",
                    format!("Solution: {}", solution.solution),
                ));

                (
                    SolverResult::Single(solution.solution),
                    StepByStepExplanation::new(steps),
                )
            }
            Err(err) => {
                steps.push(Step::new(
                    "Solver Status",
                    format!("PDE solving: {:?}", err),
                ));

                (SolverResult::NoSolution, StepByStepExplanation::new(steps))
            }
        }
    }
}

impl Default for EducationalPDESolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Implement the EquationSolver trait for integration with SmartEquationSolver
impl crate::algebra::solvers::EquationSolver for EducationalPDESolver {
    fn solve(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> crate::algebra::solvers::SolverResult {
        if !Self::is_pde(equation) {
            return SolverResult::NoSolution;
        }

        let independent_vars = extract_independent_vars(equation, variable);

        let (result, _) = self.solve_pde(equation, variable, &independent_vars);
        result
    }

    fn solve_with_explanation(
        &self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (
        crate::algebra::solvers::SolverResult,
        crate::educational::step_by_step::StepByStepExplanation,
    ) {
        if !Self::is_pde(equation) {
            let steps = vec![Step::new(
                "Equation Analysis",
                "This equation does not appear to be a PDE",
            )];
            return (SolverResult::NoSolution, StepByStepExplanation::new(steps));
        }

        let independent_vars = extract_independent_vars(equation, variable);
        self.solve_pde(equation, variable, &independent_vars)
    }

    fn can_solve(&self, equation: &Expression) -> bool {
        Self::is_pde(equation)
    }
}

/// Check if expression contains multiple variables (PDE indicator)
fn has_multiple_variables(expr: &Expression) -> bool {
    use crate::core::Expression;
    use std::collections::HashSet;

    fn count_vars(e: &Expression, vars: &mut HashSet<String>) {
        match e {
            Expression::Symbol(s) => {
                vars.insert(s.name().to_owned());
            }
            Expression::Add(terms) => {
                for term in terms.as_ref() {
                    count_vars(term, vars);
                }
            }
            Expression::Mul(factors) => {
                for factor in factors.as_ref() {
                    count_vars(factor, vars);
                }
            }
            Expression::Pow(base, exp) => {
                count_vars(base, vars);
                count_vars(exp, vars);
            }
            Expression::Function { args, .. } => {
                for arg in args.as_ref() {
                    count_vars(arg, vars);
                }
            }
            _ => {}
        }
    }

    let mut vars = HashSet::new();
    count_vars(expr, &mut vars);
    vars.len() >= 2
}

/// Extract independent variables from equation
///
/// Uses heuristic approach: collects all variables except the dependent variable.
/// Returns variables found in the equation structure.
fn extract_independent_vars(equation: &Expression, dependent: &Symbol) -> Vec<Symbol> {
    use crate::core::Expression;
    use std::collections::HashSet;

    fn collect_vars(e: &Expression, vars: &mut HashSet<Symbol>) {
        match e {
            Expression::Symbol(s) => {
                vars.insert(s.clone());
            }
            Expression::Add(terms) => {
                for term in terms.as_ref() {
                    collect_vars(term, vars);
                }
            }
            Expression::Mul(factors) => {
                for factor in factors.as_ref() {
                    collect_vars(factor, vars);
                }
            }
            Expression::Pow(base, exp) => {
                collect_vars(base, vars);
                collect_vars(exp, vars);
            }
            Expression::Function { args, .. } => {
                for arg in args.as_ref() {
                    collect_vars(arg, vars);
                }
            }
            _ => {}
        }
    }

    let mut vars = HashSet::new();
    collect_vars(equation, &mut vars);

    vars.remove(dependent);

    vars.into_iter().collect()
}

#[cfg(test)]
use crate::algebra::solvers::EquationSolver;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_educational_pde_solver_creation() {
        let solver = EducationalPDESolver::new();
        let _ = solver;
    }

    #[test]
    fn test_is_pde_true() {
        let _x = symbol!(x);
        let _y = symbol!(y);
        let eq = expr!(_x + _y);
        assert!(EducationalPDESolver::is_pde(&eq));
    }

    #[test]
    fn test_is_pde_false() {
        let _x = symbol!(x);
        let eq = expr!(_x);
        assert!(!EducationalPDESolver::is_pde(&eq));
    }

    #[test]
    fn test_has_multiple_variables() {
        let _x = symbol!(x);
        let _y = symbol!(y);
        let eq = expr!(_x + _y);
        assert!(has_multiple_variables(&eq));
    }

    #[test]
    fn test_has_multiple_variables_single() {
        let _x = symbol!(x);
        let eq = expr!(_x * (_x ^ 2));
        assert!(!has_multiple_variables(&eq));
    }

    #[test]
    fn test_extract_independent_vars() {
        let u = symbol!(u);
        let _x = symbol!(x);
        let _t = symbol!(t);
        let eq = expr!(u + _x + _t);

        let independent = extract_independent_vars(&eq, &u);
        assert_eq!(independent.len(), 2);
    }

    #[test]
    fn test_can_solve_pde() {
        let solver = EducationalPDESolver::new();
        let _x = symbol!(x);
        let _y = symbol!(y);
        let eq = expr!(_x + _y);

        assert!(solver.can_solve(&eq));
    }

    #[test]
    fn test_can_solve_non_pde() {
        let solver = EducationalPDESolver::new();
        let _x = symbol!(x);
        let eq = expr!(_x);

        assert!(!solver.can_solve(&eq));
    }

    #[test]
    fn test_solve_returns_no_solution_for_non_pde() {
        let solver = EducationalPDESolver::new();
        let _x = symbol!(x);
        let eq = expr!(_x);

        let result = solver.solve(&eq, &_x);
        assert!(matches!(result, SolverResult::NoSolution));
    }

    #[test]
    fn test_solve_pde_basic() {
        let solver = EducationalPDESolver::new();
        let u = symbol!(u);
        let _x = symbol!(x);
        let _t = symbol!(t);
        let eq = expr!(u + _x + _t);

        let (result, explanation) = solver.solve_pde(&eq, &u, &[_x, _t]);

        match result {
            SolverResult::Single(_) | SolverResult::NoSolution => {
                // Either is acceptable for this test
            }
            _ => panic!("Unexpected result type"),
        }

        assert!(!explanation.steps.is_empty());
    }

    #[test]
    fn test_solve_with_explanation_pde() {
        let solver = EducationalPDESolver::new();
        let u = symbol!(u);
        let _x = symbol!(x);
        let _y = symbol!(y);
        let eq = expr!(u + _x + _y);

        let (_, explanation) = solver.solve_with_explanation(&eq, &u);
        assert!(!explanation.steps.is_empty());
    }

    #[test]
    fn test_solve_with_explanation_non_pde() {
        let solver = EducationalPDESolver::new();
        let _x = symbol!(x);
        let eq = expr!(_x);

        let (result, explanation) = solver.solve_with_explanation(&eq, &_x);
        assert!(matches!(result, SolverResult::NoSolution));
        assert!(!explanation.steps.is_empty());
    }

    #[test]
    fn test_default_impl() {
        let solver = EducationalPDESolver::default();
        let _ = solver;
    }
}
