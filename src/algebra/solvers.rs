//! 🎯 EQUATION SOLVERS MODULE - MODERN RUST STRUCTURE
//! Comprehensive equation solving with step-by-step explanations
//! Following modern Rust 2021+ conventions and TDD approach

use crate::core::{Expression, Symbol};
use crate::educational::step_by_step::{StepByStepExplanation, Step};
use serde::{Deserialize, Serialize};

// Modern Rust: Individual solver modules
pub mod linear;
pub mod quadratic; 
pub mod systems;
pub mod polynomial;

// Re-exports for easy access
pub use linear::LinearSolver;
pub use quadratic::QuadraticSolver;
pub use systems::SystemSolver;
pub use polynomial::PolynomialSolver;

/// 🎯 SOLVER RESULT - UNIFIED RESULT TYPE
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SolverResult {
    /// Single solution found
    Single(Expression),
    /// Multiple solutions found
    Multiple(Vec<Expression>),
    /// No solution exists
    NoSolution,
    /// Infinite solutions exist
    InfiniteSolutions,
    /// Parametric solutions (for systems)
    Parametric(Vec<Expression>),
}

/// 🎯 SOLVER ERROR - UNIFIED ERROR HANDLING
#[derive(Debug, Clone, PartialEq)]
pub enum SolverError {
    /// Malformed equation
    InvalidEquation(String),
    /// Unsupported equation type
    UnsupportedType(String),
    /// Numerical instability
    NumericalInstability(String),
    /// Too complex to solve
    ComplexityLimit(String),
}

/// 🎯 EQUATION SOLVER TRAIT - COMMON INTERFACE
pub trait EquationSolver {
    /// Solve equation for given variable
    fn solve(&self, equation: &Expression, variable: &Symbol) -> SolverResult;
    
    /// Solve with step-by-step explanation (CRITICAL USER REQUIREMENT)
    fn solve_with_explanation(&self, equation: &Expression, variable: &Symbol) -> (SolverResult, StepByStepExplanation);
    
    /// Check if solver can handle this equation type
    fn can_solve(&self, equation: &Expression) -> bool;
}

/// 🎯 SYSTEM SOLVER TRAIT - FOR SYSTEM OF EQUATIONS
pub trait SystemEquationSolver {
    /// Solve system of equations
    fn solve_system(&self, equations: &[Expression], variables: &[Symbol]) -> SolverResult;
    
    /// Solve system with step-by-step explanation
    fn solve_system_with_explanation(&self, equations: &[Expression], variables: &[Symbol]) -> (SolverResult, StepByStepExplanation);
}

// ============================================================================
// 🧪 SOLVER RESULT UTILITY METHODS
// ============================================================================

impl SolverResult {
    /// Check if result represents a valid solution
    pub fn is_valid_solution(&self) -> bool {
        match self {
            SolverResult::NoSolution => true, // Valid to have no solution
            SolverResult::InfiniteSolutions => true, // Valid to have infinite solutions
            SolverResult::Single(expr) => expr.is_valid_expression(),
            SolverResult::Multiple(exprs) => exprs.iter().all(|e| e.is_valid_expression()),
            SolverResult::Parametric(exprs) => exprs.iter().all(|e| e.is_valid_expression()),
        }
    }
    
    /// Get number of solutions
    pub fn solution_count(&self) -> Option<usize> {
        match self {
            SolverResult::Single(_) => Some(1),
            SolverResult::Multiple(exprs) => Some(exprs.len()),
            SolverResult::Parametric(exprs) => Some(exprs.len()),
            SolverResult::NoSolution => Some(0),
            SolverResult::InfiniteSolutions => None, // Infinite
        }
    }
}

// ============================================================================
// 🎓 STEP-BY-STEP INTEGRATION (CRITICAL USER REQUIREMENT)
// ============================================================================

/// Extension trait for Expression to add solver step-by-step support
pub trait SolverStepByStep {
    /// Solve with complete step-by-step explanation
    fn solve_with_steps(&self, variable: &Symbol) -> (SolverResult, StepByStepExplanation);
    
    /// Generate step-by-step explanation for solving process
    fn explain_solving_steps(&self, variable: &Symbol) -> StepByStepExplanation;
}

impl SolverStepByStep for Expression {
    fn solve_with_steps(&self, variable: &Symbol) -> (SolverResult, StepByStepExplanation) {
        // This will be implemented by individual solvers
        // For now, return placeholder
        let explanation = StepByStepExplanation::new(vec![
            Step::new("Analysis", format!("Analyzing equation: {}", self)),
            Step::new("Method", "Determining appropriate solving method"),
            Step::new("Implementation", "Solver implementation in progress..."),
        ]);
        
        (SolverResult::NoSolution, explanation)
    }
    
    fn explain_solving_steps(&self, variable: &Symbol) -> StepByStepExplanation {
        StepByStepExplanation::new(vec![
            Step::new("Equation", format!("Given: {} = 0", self)),
            Step::new("Variable", format!("Solve for: {}", variable.name)),
            Step::new("Method", "Applying appropriate solving algorithm"),
        ])
    }
}

// ============================================================================
// 🔧 UTILITY FUNCTIONS
// ============================================================================

impl Expression {
    /// Check if expression is a valid mathematical expression
    pub fn is_valid_expression(&self) -> bool {
        // Basic validation - can be expanded
        match self {
            Expression::Number(_) | Expression::Symbol(_) => true,
            Expression::Add(terms) | Expression::Mul(terms) => {
                !terms.is_empty() && terms.iter().all(|t| t.is_valid_expression())
            },
            Expression::Pow(base, exp) => {
                base.is_valid_expression() && exp.is_valid_expression()
            },
            Expression::Function { args, .. } => {
                args.iter().all(|a| a.is_valid_expression())
            },
        }
    }
    
    /// Convert to LaTeX representation for solvers (avoid conflict)
    pub fn solver_to_latex(&self) -> String {
        match self {
            Expression::Number(n) => format!("{}", n),
            Expression::Symbol(s) => s.name.clone(),
            Expression::Add(terms) => {
                let term_strs: Vec<String> = terms.iter().map(|t| format!("{}", t)).collect();
                term_strs.join(" + ")
            },
            Expression::Mul(factors) => {
                let factor_strs: Vec<String> = factors.iter().map(|f| format!("{}", f)).collect();
                factor_strs.join(" \\cdot ")
            },
            Expression::Pow(base, exp) => {
                format!("{}^{{{}}}", base, exp)
            },
            Expression::Function { name, args } => {
                let arg_strs: Vec<String> = args.iter().map(|a| format!("{}", a)).collect();
                format!("\\{}({})", name, arg_strs.join(", "))
            },
        }
    }
}
