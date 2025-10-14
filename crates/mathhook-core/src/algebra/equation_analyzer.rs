//! Analyzes LaTeX equations and routes to appropriate solvers
//! This is the "brain" that decides which solver to use

use crate::algebra::solvers::{EquationSolver, SolverResult};
use crate::algebra::solvers::{LinearSolver, PolynomialSolver, QuadraticSolver, SystemSolver};
use crate::core::{Expression, Number, Symbol};
use crate::educational::step_by_step::{Step, StepByStepExplanation};

/// Types of equations our system can handle
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EquationType {
    Constant,       // "5 = 0"
    Linear,         // "2x + 3 = 0"
    Quadratic,      // "x² + 3x + 2 = 0"
    Cubic,          // "x³ + 2x² + x + 1 = 0"
    Quartic,        // "x⁴ + x³ + x² + x + 1 = 0"
    System,         // "2x + 3y = 5, x - y = 1"
    Transcendental, // "sin(x) = 0", "e^x = 5"
    Unknown,
}

/// Smart equation analyzer that determines solver routing
pub struct EquationAnalyzer;

impl EquationAnalyzer {
    /// Analyze equation and determine type for solver dispatch
    pub fn analyze(equation: &Expression, variable: &Symbol) -> EquationType {
        let degree = Self::find_highest_degree(equation, variable);
        let has_transcendental = Self::has_transcendental_functions(equation);
        let variable_count = Self::count_variables(equation);

        match (degree, has_transcendental, variable_count) {
            (0, false, _) => EquationType::Constant,
            (1, false, 1) => EquationType::Linear,
            (2, false, 1) => EquationType::Quadratic,
            (3, false, 1) => EquationType::Cubic,
            (4, false, 1) => EquationType::Quartic,
            (_, false, 2..) => EquationType::System,
            (_, true, _) => EquationType::Transcendental,
            _ => EquationType::Unknown,
        }
    }

    /// Find the highest degree of variable in expression
    fn find_highest_degree(expr: &Expression, variable: &Symbol) -> u32 {
        match expr {
            // Direct power: x^2, x^3, etc.
            Expression::Pow(base, exp) if **base == Expression::symbol(variable.clone()) => {
                match exp.as_ref() {
                    Expression::Number(Number::Integer(n)) => *n as u32,
                    _ => 1, // Non-constant exponent
                }
            }
            // Multiplication: 3x^2, coefficient * x^power
            Expression::Mul(factors) => factors
                .iter()
                .map(|f| Self::find_highest_degree(f, variable))
                .max()
                .unwrap_or(0),
            // Addition: x^2 + 3x + 2
            Expression::Add(terms) => terms
                .iter()
                .map(|t| Self::find_highest_degree(t, variable))
                .max()
                .unwrap_or(0),
            // Simple variable: x (degree 1)
            _ if *expr == Expression::symbol(variable.clone()) => 1,
            // Constant or other variable
            _ => 0,
        }
    }

    /// Check for transcendental functions
    fn has_transcendental_functions(expr: &Expression) -> bool {
        match expr {
            Expression::Function { name, args } => {
                matches!(name.as_str(), "sin" | "cos" | "tan" | "exp" | "ln" | "log")
                    || args.iter().any(Self::has_transcendental_functions)
            }
            Expression::Add(terms) => terms.iter().any(Self::has_transcendental_functions),
            Expression::Mul(factors) => factors.iter().any(Self::has_transcendental_functions),
            Expression::Pow(base, exp) => {
                Self::has_transcendental_functions(base) || Self::has_transcendental_functions(exp)
            }
            _ => false,
        }
    }

    /// Count unique variables in expression
    fn count_variables(expr: &Expression) -> usize {
        let mut variables = std::collections::HashSet::new();
        Self::collect_variables(expr, &mut variables);
        variables.len()
    }

    /// Recursively collect all variables
    pub fn collect_variables(expr: &Expression, variables: &mut std::collections::HashSet<String>) {
        match expr {
            Expression::Symbol(s) => {
                variables.insert(s.name().to_string());
            }
            Expression::Add(terms) => {
                for term in terms.iter() {
                    Self::collect_variables(term, variables);
                }
            }
            Expression::Mul(factors) => {
                for factor in factors.iter() {
                    Self::collect_variables(factor, variables);
                }
            }
            Expression::Pow(base, exp) => {
                Self::collect_variables(base, variables);
                Self::collect_variables(exp, variables);
            }
            Expression::Function { args, .. } => {
                for arg in args.iter() {
                    Self::collect_variables(arg, variables);
                }
            }
            _ => {}
        }
    }
}

/// Master equation solver with smart dispatch
pub struct SmartEquationSolver {
    linear_solver: LinearSolver,
    quadratic_solver: QuadraticSolver,
    system_solver: SystemSolver,
    polynomial_solver: PolynomialSolver,
}

impl SmartEquationSolver {
    pub fn new() -> Self {
        Self {
            linear_solver: LinearSolver::new(),
            quadratic_solver: QuadraticSolver::new(),
            system_solver: SystemSolver::new(),
            polynomial_solver: PolynomialSolver::new(),
        }
    }

    /// Solve equation with educational explanation, including equation analysis
    ///
    /// This is the primary entry point for solving equations with full educational
    /// integration. It automatically:
    /// 1. Analyzes the equation type
    /// 2. Explains the equation structure
    /// 3. Selects the appropriate solver
    /// 4. Provides step-by-step solution with explanations
    ///
    /// # Arguments
    ///
    /// * `equation` - The equation expression to solve
    /// * `variable` - The variable to solve for
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - The solver result (solutions or error)
    /// - Complete step-by-step explanation starting with equation analysis
    pub fn solve_with_equation(
        &mut self,
        equation: &Expression,
        variable: &Symbol,
    ) -> (SolverResult, StepByStepExplanation) {
        let mut all_steps = Vec::new();

        let degree = EquationAnalyzer::find_highest_degree(equation, variable);
        let eq_type = EquationAnalyzer::analyze(equation, variable);

        let analysis_description = match eq_type {
            EquationType::Constant => {
                format!("Detected constant equation (no variables)")
            }
            EquationType::Linear => {
                format!("Detected linear equation (highest degree: {})", degree)
            }
            EquationType::Quadratic => {
                format!("Detected quadratic equation (highest degree: {})", degree)
            }
            EquationType::Cubic => {
                format!("Detected cubic equation (highest degree: {})", degree)
            }
            EquationType::Quartic => {
                format!("Detected quartic equation (highest degree: {})", degree)
            }
            EquationType::System => {
                format!("Detected system of equations (multiple variables)")
            }
            EquationType::Transcendental => {
                format!("Detected transcendental equation (contains trig/exp/log functions)")
            }
            EquationType::Unknown => {
                format!("Unknown equation type")
            }
        };

        all_steps.push(Step::new("Equation Analysis", analysis_description));

        let solver_description = match eq_type {
            EquationType::Linear => "Using linear equation solver (isolation method)",
            EquationType::Quadratic => "Using quadratic equation solver (quadratic formula)",
            EquationType::Cubic | EquationType::Quartic => "Using polynomial solver",
            EquationType::System => "Using system equation solver",
            _ => "No specialized solver available for this equation type",
        };

        all_steps.push(Step::new("Solver Selection", solver_description));

        let (result, mut solver_steps) = match eq_type {
            EquationType::Linear => self
                .linear_solver
                .solve_with_explanation(equation, variable),
            EquationType::Quadratic => self
                .quadratic_solver
                .solve_with_explanation(equation, variable),
            EquationType::Cubic | EquationType::Quartic => self
                .polynomial_solver
                .solve_with_explanation(equation, variable),
            EquationType::System => {
                self.linear_solver
                    .solve_with_explanation(equation, variable)
            }
            _ => {
                all_steps.push(Step::new(
                    "Status",
                    "This equation type is not yet fully implemented",
                ));
                (SolverResult::NoSolution, StepByStepExplanation::new(vec![]))
            }
        };

        all_steps.extend(solver_steps.steps);

        (result, StepByStepExplanation::new(all_steps))
    }

    /// Legacy solve method (deprecated, use solve_with_equation instead)
    pub fn solve(&mut self) -> (SolverResult, StepByStepExplanation) {
        let equation = Expression::integer(0);
        let variables = self.extract_variables(&equation);
        if variables.is_empty() {
            return (SolverResult::NoSolution, StepByStepExplanation::new(vec![]));
        }

        let primary_var = &variables[0];
        self.solve_with_equation(&equation, primary_var)
    }

    /// Extract variables from equation
    fn extract_variables(&self, equation: &Expression) -> Vec<Symbol> {
        let mut variables = std::collections::HashSet::new();
        EquationAnalyzer::collect_variables(equation, &mut variables);

        variables
            .into_iter()
            .map(|name| Symbol::new(&name))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_equation_type_detection() {
        let x = symbol!(x);

        // Linear: 2x + 3
        let linear = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::integer(3),
        ]);
        assert_eq!(EquationAnalyzer::analyze(&linear, &x), EquationType::Linear);

        // Quadratic: x^2 + 3x + 2
        let quadratic = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
            Expression::integer(2),
        ]);
        assert_eq!(
            EquationAnalyzer::analyze(&quadratic, &x),
            EquationType::Quadratic
        );
    }
}
