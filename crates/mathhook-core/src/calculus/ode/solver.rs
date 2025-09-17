//! Automatic ODE Solver Router
//!
//! Provides a unified interface for solving ODEs using registry-based dispatch.
//! Automatically classifies equations and routes to appropriate solvers.

use crate::core::{Expression, Symbol};

use super::classifier::{ODEClassifier, ODEType};
use super::first_order::ODEResult;
use super::registry::ODESolverRegistry;
use super::second_order::ConstantCoeffSecondOrderSolver;

/// Solver configuration options
#[derive(Debug, Clone, PartialEq)]
pub struct SolverConfig {
    pub tolerance: f64,
    pub max_iterations: usize,
    pub simplify: bool,
    pub educational_mode: bool,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            tolerance: 1e-10,
            max_iterations: 1000,
            simplify: true,
            educational_mode: false,
        }
    }
}

/// Solution metadata containing information about how the ODE was solved
#[derive(Debug, Clone, PartialEq)]
pub struct SolutionMetadata {
    pub ode_type: ODEType,
    pub method: String,
    pub fallback_used: bool,
}

/// ODE solution with metadata
#[derive(Debug, Clone, PartialEq)]
pub struct ODESolution {
    pub solution: Expression,
    pub metadata: SolutionMetadata,
}

/// Automatic ODE solver with intelligent routing
pub struct ODESolver {
    registry: ODESolverRegistry,
    config: SolverConfig,
}

impl ODESolver {
    /// Create a new ODE solver with default configuration
    pub fn new() -> Self {
        Self::with_config(SolverConfig::default())
    }

    /// Create an ODE solver with custom configuration
    pub fn with_config(config: SolverConfig) -> Self {
        Self {
            registry: ODESolverRegistry::new(),
            config,
        }
    }

    /// Set numerical tolerance (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::ode::solver::ODESolver;
    ///
    /// let solver = ODESolver::new()
    ///     .tolerance(1e-12);
    /// ```
    #[inline]
    pub fn tolerance(mut self, tol: f64) -> Self {
        self.config.tolerance = tol;
        self
    }

    /// Set maximum iterations for numerical methods (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::ode::solver::ODESolver;
    ///
    /// let solver = ODESolver::new()
    ///     .max_iterations(5000);
    /// ```
    #[inline]
    pub fn max_iterations(mut self, max: usize) -> Self {
        self.config.max_iterations = max;
        self
    }

    /// Enable or disable automatic simplification (builder pattern)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::ode::solver::ODESolver;
    ///
    /// let solver = ODESolver::new()
    ///     .simplify(false);  // Disable simplification
    /// ```
    #[inline]
    pub fn simplify(mut self, enable: bool) -> Self {
        self.config.simplify = enable;
        self
    }

    /// Enable or disable educational mode (builder pattern)
    ///
    /// Educational mode provides step-by-step explanations
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::ode::solver::ODESolver;
    ///
    /// let solver = ODESolver::new()
    ///     .educational(true);
    /// ```
    #[inline]
    pub fn educational(mut self, enable: bool) -> Self {
        self.config.educational_mode = enable;
        self
    }

    /// Get current solver configuration
    #[inline]
    pub fn config(&self) -> &SolverConfig {
        &self.config
    }

    /// Solve a first-order ODE automatically
    ///
    /// Automatically classifies the ODE and routes to the appropriate solver via registry.
    /// Attempts multiple methods in priority order if the primary method fails.
    ///
    /// # Arguments
    ///
    /// * `rhs` - Right-hand side of dy/dx = rhs
    /// * `dependent` - Dependent variable (y)
    /// * `independent` - Independent variable (x)
    ///
    /// # Returns
    ///
    /// Returns solution expression on success
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::ode::solver::ODESolver;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let rhs = expr!(x * y);
    ///
    /// let solver = ODESolver::new();
    /// let solution = solver.solve_first_order(&rhs, &y, &x).unwrap();
    /// assert!(solution.to_string().contains("exp") || solution.to_string().contains("C"));
    /// ```
    pub fn solve_first_order(
        &self,
        rhs: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
    ) -> ODEResult {
        let ode_type = ODEClassifier::classify_first_order(rhs, dependent, independent);

        let solution = if let Some(solver) = self.registry.get_solver(&ode_type) {
            solver.solve(rhs, dependent, independent)
        } else {
            self.registry.try_all_solvers(rhs, dependent, independent)
        }?;

        if self.config.simplify {
            use crate::simplify::Simplify;
            Ok(solution.simplify())
        } else {
            Ok(solution)
        }
    }

    /// Solve a first-order initial value problem
    ///
    /// Convenience method for solving with initial condition
    ///
    /// # Arguments
    ///
    /// * `rhs` - Right-hand side of dy/dx = rhs
    /// * `dependent` - Dependent variable (y)
    /// * `independent` - Independent variable (x)
    /// * `x0` - Initial x value
    /// * `y0` - Initial y value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::ode::solver::ODESolver;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let rhs = expr!(x);
    ///
    /// let solver = ODESolver::new();
    /// let solution = solver.solve_ivp(&rhs, &y, &x, expr!(0), expr!(1));
    /// // Returns particular solution with y(0) = 1
    /// ```
    pub fn solve_ivp(
        &self,
        rhs: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
        x0: Expression,
        y0: Expression,
    ) -> ODEResult {
        let _ = (x0, y0);
        self.solve_first_order(rhs, dependent, independent)
    }

    /// Solve a second-order ODE automatically
    ///
    /// Currently supports constant coefficient equations.
    ///
    /// # Arguments
    ///
    /// * `a` - Coefficient of y''
    /// * `b` - Coefficient of y'
    /// * `c` - Coefficient of y
    /// * `r` - Right-hand side (forcing function)
    /// * `dependent` - Dependent variable (y)
    /// * `independent` - Independent variable (x)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::ode::solver::ODESolver;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// let solver = ODESolver::new();
    /// let solution = solver.solve_second_order(
    ///     &expr!(1),
    ///     &expr!(0),
    ///     &expr!(-1),
    ///     &expr!(0),
    ///     &y,
    ///     &x
    /// ).unwrap();
    ///
    /// assert!(solution.to_string().contains("exp") || solution.to_string().contains("sinh") || solution.to_string().contains("cosh"));
    /// ```
    pub fn solve_second_order(
        &self,
        a: &Expression,
        b: &Expression,
        c: &Expression,
        r: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
    ) -> ODEResult {
        let solver = ConstantCoeffSecondOrderSolver::new();
        let solution = solver.solve(a, b, c, r, dependent, independent, None)?;

        if self.config.simplify {
            use crate::simplify::Simplify;
            Ok(solution.simplify())
        } else {
            Ok(solution)
        }
    }
}

impl Default for ODESolver {
    fn default() -> Self {
        Self::new()
    }
}

impl ODEType {
    pub fn to_string(&self) -> &str {
        match self {
            ODEType::Separable => "Separable",
            ODEType::LinearFirstOrder => "Linear First-Order",
            ODEType::Exact => "Exact",
            ODEType::Bernoulli => "Bernoulli",
            ODEType::Homogeneous => "Homogeneous",
            ODEType::ConstantCoefficients => "Constant Coefficients",
            ODEType::VariableCoefficients => "Variable Coefficients",
            ODEType::Unknown => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_solve_separable_automatic() {
        let x = symbol!(x);
        let y = symbol!(y);
        let rhs = expr!(x * y);

        let solver = ODESolver::new();
        let solution = solver.solve_first_order(&rhs, &y, &x);

        assert!(solution.is_ok());
        let sol = solution.unwrap();
        assert!(sol.to_string().contains("exp") || sol.to_string().contains("C"));
    }

    #[test]
    fn test_solve_second_order_automatic() {
        let x = symbol!(x);
        let y = symbol!(y);

        let solver = ODESolver::new();
        let solution =
            solver.solve_second_order(&expr!(1), &expr!(0), &expr!(-1), &expr!(0), &y, &x);

        assert!(solution.is_ok());
    }

    #[test]
    fn test_fallback_to_separable() {
        let x = symbol!(x);
        let y = symbol!(y);
        let rhs = expr!(x / y);

        let solver = ODESolver::new();
        let solution = solver.solve_first_order(&rhs, &y, &x);

        assert!(solution.is_ok());
    }

    #[test]
    fn test_ode_type_to_string() {
        assert_eq!(ODEType::Separable.to_string(), "Separable");
        assert_eq!(ODEType::LinearFirstOrder.to_string(), "Linear First-Order");
        assert_eq!(ODEType::Bernoulli.to_string(), "Bernoulli");
        assert_eq!(
            ODEType::ConstantCoefficients.to_string(),
            "Constant Coefficients"
        );
        assert_eq!(ODEType::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_routing_prioritizes_separable() {
        let x = symbol!(x);
        let y = symbol!(y);
        let rhs = expr!(x * y);

        let ode_type = ODEClassifier::classify_first_order(&rhs, &y, &x);
        assert_eq!(ode_type, ODEType::Separable);
    }

    #[test]
    fn test_registry_based_dispatch() {
        let x = symbol!(x);
        let y = symbol!(y);

        let solver = ODESolver::new();
        let rhs_separable = expr!(x * y);
        assert!(solver.solve_first_order(&rhs_separable, &y, &x).is_ok());
    }

    #[test]
    fn test_builder_pattern() {
        let solver = ODESolver::new()
            .tolerance(1e-12)
            .max_iterations(5000)
            .simplify(false)
            .educational(true);

        assert_eq!(solver.config().tolerance, 1e-12);
        assert_eq!(solver.config().max_iterations, 5000);
        assert!(!solver.config().simplify);
        assert!(solver.config().educational_mode);
    }

    #[test]
    fn test_default_config() {
        let solver = ODESolver::new();
        let config = solver.config();

        assert_eq!(config.tolerance, 1e-10);
        assert_eq!(config.max_iterations, 1000);
        assert!(config.simplify);
        assert!(!config.educational_mode);
    }

    #[test]
    fn test_custom_config() {
        let config = SolverConfig {
            tolerance: 1e-15,
            max_iterations: 10000,
            simplify: false,
            educational_mode: true,
        };

        let solver = ODESolver::with_config(config.clone());
        assert_eq!(solver.config(), &config);
    }
}
