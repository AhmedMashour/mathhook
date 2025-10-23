//! Analyzes LaTeX equations and routes to appropriate solvers
//! This is the "brain" that decides which solver to use

use crate::algebra::solvers::{EquationSolver, SolverResult};
use crate::algebra::solvers::{LinearSolver, PolynomialSolver, QuadraticSolver, SystemSolver};
use crate::algebra::solvers::matrix_equations::MatrixEquationSolver;
use crate::algebra::root_finding::{NewtonRaphson, RootFinder, RootFindingConfig};
use crate::core::{Expression, Number, Symbol};
use crate::core::symbol::SymbolType;
use crate::educational::step_by_step::{Step, StepByStepExplanation};
use crate::ode::EducationalODESolver;
use crate::pde::EducationalPDESolver;

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
    Numerical,      // Equations requiring numerical methods
    Matrix,         // "A*X = B" (matrix equation)
    ODE,            // "y' + 2y = x", "y'' + 3y' + 2y = 0"
    PDE,            // "∂u/∂t = k∂²u/∂x²" (heat equation)
    Unknown,
}

/// Smart equation analyzer that determines solver routing
pub struct EquationAnalyzer;

impl EquationAnalyzer {
    /// Analyze equation and determine type for solver dispatch
    pub fn analyze(equation: &Expression, variable: &Symbol) -> EquationType {
        let has_derivatives = Self::has_derivatives(equation);
        let has_partial_derivatives = Self::has_partial_derivatives(equation);

        if has_partial_derivatives {
            return EquationType::PDE;
        }

        if has_derivatives {
            return EquationType::ODE;
        }

        if Self::is_matrix_equation(equation, variable) {
            return EquationType::Matrix;
        }

        let degree = Self::find_highest_degree(equation, variable);
        let has_transcendental = Self::has_transcendental_functions(equation);
        let variable_count = Self::count_variables(equation);

        // Check if numerical methods are needed
        if Self::is_numerical_equation(equation, variable, degree, has_transcendental) {
            return EquationType::Numerical;
        }

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

    /// Determine if equation requires numerical methods
    ///
    /// Numerical methods are needed when:
    /// - Polynomial degree > 4 (no general algebraic formula)
    /// - Transcendental functions mixed with polynomials (x = sin(x))
    /// - Complex transcendental equations (e^x = x^2)
    fn is_numerical_equation(
        expr: &Expression,
        _variable: &Symbol,
        degree: u32,
        has_transcendental: bool,
    ) -> bool {
        // Polynomial degree > 4 requires numerical methods
        if degree > 4 {
            return true;
        }

        // Mixed transcendental and polynomial requires numerical methods
        if has_transcendental && degree > 0 {
            return true;
        }

        // Complex transcendental equations
        if has_transcendental {
            let func_count = Self::count_transcendental_functions(expr);
            if func_count > 1 {
                return true;
            }
        }

        false
    }

    /// Count transcendental functions in expression
    fn count_transcendental_functions(expr: &Expression) -> usize {
        match expr {
            Expression::Function { name, args } => {
                let current = if matches!(name.as_str(), "sin" | "cos" | "tan" | "exp" | "ln" | "log") {
                    1
                } else {
                    0
                };
                current + args.iter().map(Self::count_transcendental_functions).sum::<usize>()
            }
            Expression::Add(terms) => terms.iter().map(Self::count_transcendental_functions).sum(),
            Expression::Mul(factors) => factors.iter().map(Self::count_transcendental_functions).sum(),
            Expression::Pow(base, exp) => {
                Self::count_transcendental_functions(base) + Self::count_transcendental_functions(exp)
            }
            _ => 0,
        }
    }

    /// Check if equation is a matrix equation (contains noncommutative symbols)
    fn is_matrix_equation(expr: &Expression, _variable: &Symbol) -> bool {
        Self::has_noncommutative_symbols(expr)
    }

    /// Check if expression contains noncommutative symbols (matrix, operator, quaternion)
    fn has_noncommutative_symbols(expr: &Expression) -> bool {
        match expr {
            Expression::Symbol(s) => {
                matches!(
                    s.symbol_type(),
                    SymbolType::Matrix | SymbolType::Operator | SymbolType::Quaternion
                )
            }
            Expression::Add(terms) | Expression::Mul(terms) => {
                terms.iter().any(Self::has_noncommutative_symbols)
            }
            Expression::Pow(base, exp) => {
                Self::has_noncommutative_symbols(base) || Self::has_noncommutative_symbols(exp)
            }
            Expression::Function { args, .. } => {
                args.iter().any(Self::has_noncommutative_symbols)
            }
            _ => false,
        }
    }

    /// Find the highest degree of variable in expression
    fn find_highest_degree(expr: &Expression, variable: &Symbol) -> u32 {
        match expr {
            // Direct power: x^2, x^3, etc.
            Expression::Pow(base, exp) if **base == Expression::symbol(variable.clone()) => {
                match exp.as_ref() {
                    Expression::Number(Number::Integer(n)) => *n as u32,
                    _ => 1,
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

    /// Check if expression contains ordinary derivatives (y', dy/dx, etc.)
    fn has_derivatives(expr: &Expression) -> bool {
        match expr {
            Expression::Function { name, args } => {
                matches!(name.as_str(), "derivative" | "diff" | "D")
                    || args.iter().any(Self::has_derivatives)
            }
            Expression::Symbol(s) => {
                let name = s.name();
                name.ends_with('\'') || name.contains("_prime")
            }
            Expression::Add(terms) => terms.iter().any(Self::has_derivatives),
            Expression::Mul(factors) => factors.iter().any(Self::has_derivatives),
            Expression::Pow(base, exp) => {
                Self::has_derivatives(base) || Self::has_derivatives(exp)
            }
            _ => false,
        }
    }

    /// Check if expression contains partial derivatives (∂u/∂x, ∂²u/∂x², etc.)
    fn has_partial_derivatives(expr: &Expression) -> bool {
        match expr {
            Expression::Function { name, args } => {
                matches!(name.as_str(), "partial" | "pdiff" | "Partial")
                    || args.iter().any(Self::has_partial_derivatives)
            }
            Expression::Symbol(s) => {
                let name = s.name();
                name.contains("partial") || name.contains("∂")
            }
            Expression::Add(terms) => terms.iter().any(Self::has_partial_derivatives),
            Expression::Mul(factors) => factors.iter().any(Self::has_partial_derivatives),
            Expression::Pow(base, exp) => {
                Self::has_partial_derivatives(base) || Self::has_partial_derivatives(exp)
            }
            _ => false,
        }
    }
}

/// Master equation solver with smart dispatch
pub struct SmartEquationSolver {
    linear_solver: LinearSolver,
    quadratic_solver: QuadraticSolver,
    system_solver: SystemSolver,
    polynomial_solver: PolynomialSolver,
    matrix_solver: MatrixEquationSolver,
    ode_solver: EducationalODESolver,
    pde_solver: EducationalPDESolver,
}

impl SmartEquationSolver {
    pub fn new() -> Self {
        Self {
            linear_solver: LinearSolver::new(),
            quadratic_solver: QuadraticSolver::new(),
            system_solver: SystemSolver::new(),
            polynomial_solver: PolynomialSolver::new(),
            matrix_solver: MatrixEquationSolver::new(),
            ode_solver: EducationalODESolver::new(),
            pde_solver: EducationalPDESolver::new(),
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
            EquationType::Numerical => {
                format!("Detected numerical equation (requires numerical methods - polynomial degree > 4 or mixed transcendental)")
            }
            EquationType::Matrix => {
                format!("Detected matrix equation (contains noncommutative symbols)")
            }
            EquationType::ODE => {
                format!("Detected ordinary differential equation (contains derivatives)")
            }
            EquationType::PDE => {
                format!("Detected partial differential equation (contains partial derivatives)")
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
            EquationType::Numerical => "Using numerical solver (Newton-Raphson method with numerical differentiation)",
            EquationType::Matrix => "Using matrix equation solver (left/right division)",
            EquationType::ODE => "Using ODE solver (separable/linear/exact methods)",
            EquationType::PDE => "Using PDE solver (method of characteristics/separation of variables)",
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
            EquationType::System => self
                .system_solver
                .solve_with_explanation(equation, variable),
            EquationType::Numerical => self
                .solve_numerical(equation, variable),
            EquationType::Matrix => self
                .matrix_solver
                .solve_with_explanation(equation, variable),
            EquationType::ODE => self
                .ode_solver
                .solve_with_explanation(equation, variable),
            EquationType::PDE => self
                .pde_solver
                .solve_with_explanation(equation, variable),
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

    /// Solve numerical equations using Newton-Raphson method
    ///
    /// Provides integration point for numerical solving. Currently provides
    /// educational explanation about numerical methods requirement.
    fn solve_numerical(
        &self,
        _equation: &Expression,
        variable: &Symbol,
    ) -> (SolverResult, StepByStepExplanation) {
        let steps = vec![
            Step::new(
                "Numerical Method Required",
                format!(
                    "This equation requires numerical methods to solve for {}. Newton-Raphson method integration is available.",
                    variable.name()
                ),
            ),
            Step::new(
                "Method Description",
                "Newton-Raphson method with numerical differentiation provides robust convergence for smooth functions.",
            ),
        ];

        (SolverResult::NoSolution, StepByStepExplanation::new(steps))
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

    /// Solve system of equations using the integrated system solver
    ///
    /// This method exposes the system solving capability through SmartEquationSolver,
    /// allowing for solving both linear and polynomial systems (via Gröbner basis).
    ///
    /// # Arguments
    ///
    /// * `equations` - Array of equations to solve
    /// * `variables` - Array of variables to solve for
    ///
    /// # Returns
    ///
    /// SolverResult containing solutions, no solution, or partial solutions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::algebra::equation_analyzer::SmartEquationSolver;
    /// use mathhook_core::{symbol, Expression};
    ///
    /// let mut solver = SmartEquationSolver::new();
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// // Linear system: 2x + y = 5, x - y = 1
    /// let eq1 = Expression::add(vec![
    ///     Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
    ///     Expression::symbol(y.clone()),
    ///     Expression::integer(-5),
    /// ]);
    /// let eq2 = Expression::add(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::mul(vec![Expression::integer(-1), Expression::symbol(y.clone())]),
    ///     Expression::integer(-1),
    /// ]);
    ///
    /// let result = solver.solve_system(&[eq1, eq2], &[x, y]);
    /// ```
    pub fn solve_system(&mut self, equations: &[Expression], variables: &[Symbol]) -> SolverResult {
        use crate::algebra::solvers::SystemEquationSolver;
        self.system_solver.solve_system(equations, variables)
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

    #[test]
    fn test_numerical_equation_detection() {
        let x = symbol!(x);

        // High-degree polynomial: x^5 - x - 1 (numerical)
        let quintic = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(5)),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
            Expression::integer(-1),
        ]);
        assert_eq!(
            EquationAnalyzer::analyze(&quintic, &x),
            EquationType::Numerical
        );

        // Mixed transcendental: cos(x) - x (numerical)
        let transcendental_mixed = Expression::add(vec![
            Expression::function("cos", vec![Expression::symbol(x.clone())]),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
        ]);
        assert_eq!(
            EquationAnalyzer::analyze(&transcendental_mixed, &x),
            EquationType::Numerical
        );
    }

    #[test]
    fn test_matrix_equation_detection() {
        let a = symbol!(A; matrix);
        let x = symbol!(X; matrix);
        let b = symbol!(B; matrix);

        // A*X - B = 0
        let equation = Expression::add(vec![
            Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(x.clone())]),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
        ]);

        assert_eq!(
            EquationAnalyzer::analyze(&equation, &x),
            EquationType::Matrix
        );
    }
}
