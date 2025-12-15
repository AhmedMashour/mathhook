//! Types module for MathHook Python bindings
//!
//! This module was automatically extracted from lib.rs using syn-based refactoring.

use crate::PyExpression;

use mathhook_core::algebra::groebner::{GroebnerBasis, MonomialOrder};
use mathhook_core::calculus::pde::standard::laplace::LaplaceEquationSolver;
use mathhook_core::calculus::pde::standard::wave::WaveEquationSolver;
use mathhook_core::calculus::pde::types::InitialCondition;
use mathhook_core::calculus::pde::{self, Pde};
use mathhook_core::prelude::*;
use mathhook_core::{Expression, Symbol};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct PyStep {
    #[pyo3(get)]
    pub title: String,
    #[pyo3(get)]
    pub description: String,
    #[pyo3(get)]
    pub before: String,
    #[pyo3(get)]
    pub after: String,
    #[pyo3(get)]
    pub expression: Option<PyExpression>,
}

#[pyclass]
#[derive(Clone)]
pub struct PyStepByStepExplanation {
    #[pyo3(get)]
    pub steps: Vec<PyStep>,
}

#[pyclass]
#[derive(Clone)]
pub struct PyPattern {
    pub(crate) inner: Pattern,
}

#[pyclass]
#[derive(Clone)]
pub struct PySolverResult {
    /// List of solution expressions
    #[pyo3(get)]
    pub solutions: Vec<PyExpression>,
    /// Optional metadata about the solving process
    #[pyo3(get)]
    pub metadata: Option<String>,
}

/// Result from solve_with_steps containing both solutions and educational steps
#[pyclass]
#[derive(Clone)]
pub struct PySolveWithStepsResult {
    /// List of solution expressions
    #[pyo3(get)]
    pub solutions: Vec<PyExpression>,
    /// Step-by-step explanation of the solving process
    #[pyo3(get)]
    pub steps: Vec<PyStep>,
    /// Result type: "single", "multiple", "no_solution", "infinite"
    #[pyo3(get)]
    pub result_type: String,
}

#[pyclass]
#[derive(Clone, Default)]
pub struct PyODESolver {}

#[pyclass]
#[derive(Default)]
pub struct PyPDESolver {}

#[pyclass]
pub struct PyGroebnerBasis {
    inner: GroebnerBasis,
}

#[pymethods]
impl PyPattern {
    #[doc = " Create a wildcard pattern that matches any expression"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " pattern = PyPattern.wildcard(\"x\")"]
    #[doc = " ```"]
    #[staticmethod]
    pub fn wildcard(name: &str) -> Self {
        Self {
            inner: Pattern::wildcard(name),
        }
    }
    #[doc = " Create an exact pattern that matches a specific expression"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " expr = PyExpression.symbol(\"x\")"]
    #[doc = " pattern = PyPattern.exact(expr)"]
    #[doc = " ```"]
    #[staticmethod]
    pub fn exact(expr: &PyExpression) -> Self {
        Self {
            inner: Pattern::Exact(expr.inner.clone()),
        }
    }
}

#[pymethods]
impl PySolverResult {
    #[doc = " String representation"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " result = solver.solve(equation, \"x\")"]
    #[doc = " print(str(result))"]
    #[doc = " ```"]
    pub fn __str__(&self) -> String {
        if self.solutions.is_empty() {
            "No solutions found".to_string()
        } else {
            let solutions_str = self
                .solutions
                .iter()
                .map(|s| s.to_simple())
                .collect::<Vec<_>>()
                .join(", ");
            format!("Solutions: [{}]", solutions_str)
        }
    }
    #[doc = " Better string representation for debugging"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " result = solver.solve(equation, \"x\")"]
    #[doc = " print(repr(result))"]
    #[doc = " ```"]
    pub fn __repr__(&self) -> String {
        format!(
            "PySolverResult(solutions={}, metadata={:?})",
            self.solutions.len(),
            self.metadata
        )
    }
    #[doc = " Check if any solutions were found"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " result = solver.solve(equation, \"x\")"]
    #[doc = " if result.has_solutions():"]
    #[doc = "     print(\"Found solutions\")"]
    #[doc = " ```"]
    pub fn has_solutions(&self) -> bool {
        !self.solutions.is_empty()
    }
    #[doc = " Get number of solutions"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " result = solver.solve(equation, \"x\")"]
    #[doc = " count = result.count()"]
    #[doc = " ```"]
    pub fn count(&self) -> usize {
        self.solutions.len()
    }
}

#[pymethods]
impl PyODESolver {
    #[doc = " Create a new ODE solver"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " solver = PyODESolver()"]
    #[doc = " ```"]
    #[new]
    pub fn new() -> Self {
        Self {}
    }
    #[doc = " Solve ODE using Euler method"]
    #[doc = ""]
    #[doc = " Solves dy/dx = f(x, y) using forward Euler method."]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = ""]
    #[doc = " * `ode` - The ODE expression representing dy/dx (function of x and y)"]
    #[doc = " * `x_var` - Name of the independent variable (usually \"x\")"]
    #[doc = " * `y_var` - Name of the dependent variable (usually \"y\")"]
    #[doc = " * `x0` - Initial x value"]
    #[doc = " * `y0` - Initial y value"]
    #[doc = " * `x_end` - Final x value"]
    #[doc = " * `step` - Step size"]
    #[doc = ""]
    #[doc = " # Returns"]
    #[doc = ""]
    #[doc = " List of tuples (x, y) representing solution points"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " from mathhook_python import PyODESolver, PyExpression"]
    #[doc = ""]
    #[doc = " solver = PyODESolver()"]
    #[doc = " x = PyExpression.symbol(\"x\")"]
    #[doc = " y = PyExpression.symbol(\"y\")"]
    #[doc = " ode = x  # dy/dx = x"]
    #[doc = " solution = solver.euler(ode, \"x\", \"y\", 0.0, 0.0, 1.0, 0.1)"]
    #[doc = " ```"]
    #[allow(clippy::too_many_arguments)]
    pub fn euler(
        &self,
        ode: &PyExpression,
        x_var: String,
        y_var: String,
        x0: f64,
        y0: f64,
        x_end: f64,
        step: f64,
    ) -> PyResult<Vec<(f64, f64)>> {
        use mathhook_core::calculus::ode::numerical::euler_method;
        use std::collections::HashMap;
        let f = |x: f64, y: f64| -> f64 {
            let mut vars = HashMap::new();
            vars.insert(x_var.clone(), Expression::float(x));
            vars.insert(y_var.clone(), Expression::float(y));
            let substituted = ode.inner.substitute(&vars);
            match substituted.evaluate_to_f64() {
                Ok(val) => val,
                Err(_) => f64::NAN,
            }
        };
        Ok(euler_method(f, x0, y0, x_end, step))
    }
    #[doc = " Solve ODE using Runge-Kutta 4th order method"]
    #[doc = ""]
    #[doc = " Solves dy/dx = f(x, y) using classical RK4 method."]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = ""]
    #[doc = " * `ode` - The ODE expression representing dy/dx (function of x and y)"]
    #[doc = " * `x_var` - Name of the independent variable"]
    #[doc = " * `y_var` - Name of the dependent variable"]
    #[doc = " * `x0` - Initial x value"]
    #[doc = " * `y0` - Initial y value"]
    #[doc = " * `x_end` - Final x value"]
    #[doc = " * `step` - Step size"]
    #[doc = ""]
    #[doc = " # Returns"]
    #[doc = ""]
    #[doc = " List of tuples (x, y) representing solution points"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " from mathhook_python import PyODESolver, PyExpression"]
    #[doc = ""]
    #[doc = " solver = PyODESolver()"]
    #[doc = " x = PyExpression.symbol(\"x\")"]
    #[doc = " y = PyExpression.symbol(\"y\")"]
    #[doc = " ode = y  # dy/dx = y (exponential growth)"]
    #[doc = " solution = solver.runge_kutta_4(ode, \"x\", \"y\", 0.0, 1.0, 1.0, 0.1)"]
    #[doc = " ```"]
    #[allow(clippy::too_many_arguments)]
    pub fn runge_kutta_4(
        &self,
        ode: &PyExpression,
        x_var: String,
        y_var: String,
        x0: f64,
        y0: f64,
        x_end: f64,
        step: f64,
    ) -> PyResult<Vec<(f64, f64)>> {
        use mathhook_core::calculus::ode::numerical::rk4_method;
        use std::collections::HashMap;
        let f = |x: f64, y: f64| -> f64 {
            let mut vars = HashMap::new();
            vars.insert(x_var.clone(), Expression::float(x));
            vars.insert(y_var.clone(), Expression::float(y));
            let substituted = ode.inner.substitute(&vars);
            match substituted.evaluate_to_f64() {
                Ok(val) => val,
                Err(_) => f64::NAN,
            }
        };
        Ok(rk4_method(f, x0, y0, x_end, step))
    }
    #[doc = " Solve ODE using adaptive Runge-Kutta-Fehlberg method (RKF45)"]
    #[doc = ""]
    #[doc = " Solves dy/dx = f(x, y) with automatic step size control."]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = ""]
    #[doc = " * `ode` - The ODE expression representing dy/dx"]
    #[doc = " * `x_var` - Name of the independent variable"]
    #[doc = " * `y_var` - Name of the dependent variable"]
    #[doc = " * `x0` - Initial x value"]
    #[doc = " * `y0` - Initial y value"]
    #[doc = " * `x_end` - Final x value"]
    #[doc = " * `tolerance` - Error tolerance (optional, default: 1e-6)"]
    #[doc = " * `initial_step` - Initial step size (optional, default: 0.1)"]
    #[doc = ""]
    #[doc = " # Returns"]
    #[doc = ""]
    #[doc = " List of tuples (x, y) representing solution points"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " from mathhook_python import PyODESolver, PyExpression"]
    #[doc = ""]
    #[doc = " solver = PyODESolver()"]
    #[doc = " x = PyExpression.symbol(\"x\")"]
    #[doc = " y = PyExpression.symbol(\"y\")"]
    #[doc = " ode = x.multiply(y)  # dy/dx = x*y"]
    #[doc = " solution = solver.runge_kutta_45(ode, \"x\", \"y\", 0.0, 1.0, 1.0, 1e-6, 0.1)"]
    #[doc = " ```"]
    #[allow(clippy::too_many_arguments)]
    pub fn runge_kutta_45(
        &self,
        ode: &PyExpression,
        x_var: String,
        y_var: String,
        x0: f64,
        y0: f64,
        x_end: f64,
        tolerance: Option<f64>,
        initial_step: Option<f64>,
    ) -> PyResult<Vec<(f64, f64)>> {
        use mathhook_core::calculus::ode::numerical::{rkf45_method, AdaptiveConfig};
        use std::collections::HashMap;
        let config = AdaptiveConfig {
            tolerance: tolerance.unwrap_or(1e-6),
            min_step: 1e-10,
            max_step: 1.0,
            safety_factor: 0.9,
        };
        let f = |x: f64, y: f64| -> f64 {
            let mut vars = HashMap::new();
            vars.insert(x_var.clone(), Expression::float(x));
            vars.insert(y_var.clone(), Expression::float(y));
            let substituted = ode.inner.substitute(&vars);
            match substituted.evaluate_to_f64() {
                Ok(val) => val,
                Err(_) => f64::NAN,
            }
        };
        Ok(rkf45_method(
            f,
            x0,
            y0,
            x_end,
            initial_step.unwrap_or(0.1),
            &config,
        ))
    }
}

#[pymethods]
impl PyPDESolver {
    #[doc = " Create a new PDE solver"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " from mathhook_python import PyPDESolver"]
    #[doc = ""]
    #[doc = " solver = PyPDESolver()"]
    #[doc = " ```"]
    #[new]
    pub fn new() -> Self {
        Self {}
    }
    #[doc = " Solve a PDE using automatic solver selection"]
    #[doc = ""]
    #[doc = " Classifies the PDE and dispatches to the appropriate solver based on PDE type"]
    #[doc = " (parabolic, hyperbolic, elliptic)."]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = ""]
    #[doc = " * `equation` - The PDE equation expression (should equal zero)"]
    #[doc = " * `dependent_var` - The dependent variable (e.g., \"u\" in u(x,t))"]
    #[doc = " * `independent_vars` - List of independent variable names (e.g., [\"x\", \"t\"])"]
    #[doc = ""]
    #[doc = " # Returns"]
    #[doc = ""]
    #[doc = " Expression representing the PDE solution"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " from mathhook_python import PyPDESolver, PyExpression"]
    #[doc = ""]
    #[doc = " solver = PyPDESolver()"]
    #[doc = " u = PyExpression.symbol(\"u\")"]
    #[doc = " x = PyExpression.symbol(\"x\")"]
    #[doc = " t = PyExpression.symbol(\"t\")"]
    #[doc = " equation = u  # Simplified PDE equation"]
    #[doc = " solution = solver.solve(equation, \"u\", [\"x\", \"t\"])"]
    #[doc = " ```"]
    pub fn solve(
        &self,
        equation: &PyExpression,
        dependent_var: String,
        independent_vars: Vec<String>,
    ) -> PyResult<PyExpression> {
        let dep_symbol = Symbol::new(&dependent_var);
        let indep_symbols: Vec<Symbol> = independent_vars.iter().map(Symbol::new).collect();
        let pde = Pde::new(equation.inner.clone(), dep_symbol, indep_symbols);
        match pde::solve(&pde) {
            Ok(solution) => Ok(PyExpression {
                inner: solution.solution,
            }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "PDE solving failed: {:?}",
                e
            ))),
        }
    }
    #[doc = " Solve heat equation using separation of variables"]
    #[doc = ""]
    #[doc = " Solves the heat equation: ∂u/∂t = α∇²u"]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = ""]
    #[doc = " * `alpha` - Thermal diffusivity constant"]
    #[doc = " * `spatial_var` - Spatial variable name (e.g., \"x\")"]
    #[doc = " * `time_var` - Time variable name (e.g., \"t\")"]
    #[doc = " * `domain_length` - Length of the spatial domain (for boundary conditions)"]
    #[doc = ""]
    #[doc = " # Returns"]
    #[doc = ""]
    #[doc = " Expression representing the heat equation solution"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " from mathhook_python import PyPDESolver, PyExpression"]
    #[doc = ""]
    #[doc = " solver = PyPDESolver()"]
    #[doc = " alpha = PyExpression.float(0.01)"]
    #[doc = " solution = solver.solve_heat_equation(alpha, \"x\", \"t\", PyExpression.integer(10))"]
    #[doc = " ```"]
    #[allow(deprecated)]
    pub fn solve_heat_equation(
        &self,
        alpha: &PyExpression,
        spatial_var: String,
        time_var: String,
        _domain_length: &PyExpression,
    ) -> PyResult<PyExpression> {
        let x_sym = Symbol::new(&spatial_var);
        let t_sym = Symbol::new(&time_var);
        let pde = Pde::new(alpha.inner.clone(), x_sym.clone(), vec![x_sym, t_sym]);
        use mathhook_core::calculus::pde::standard::heat::HeatEquationSolver;
        let solver = HeatEquationSolver::new();
        match solver.solve_heat_equation_1d(
            &pde,
            &alpha.inner,
            &[],
            &InitialCondition::Value {
                function: alpha.inner.clone(),
            },
        ) {
            Ok(solution) => Ok(PyExpression {
                inner: solution.solution,
            }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Heat equation solving failed: {:?}",
                e
            ))),
        }
    }
    #[doc = " Solve wave equation using separation of variables"]
    #[doc = ""]
    #[doc = " Solves the wave equation: ∂²u/∂t² = c²∇²u"]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = ""]
    #[doc = " * `wave_speed` - Wave propagation speed (c)"]
    #[doc = " * `spatial_var` - Spatial variable name (e.g., \"x\")"]
    #[doc = " * `time_var` - Time variable name (e.g., \"t\")"]
    #[doc = " * `domain_length` - Length of the spatial domain"]
    #[doc = ""]
    #[doc = " # Returns"]
    #[doc = ""]
    #[doc = " Expression representing the wave equation solution"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " from mathhook_python import PyPDESolver, PyExpression"]
    #[doc = ""]
    #[doc = " solver = PyPDESolver()"]
    #[doc = " c = PyExpression.integer(1)"]
    #[doc = " solution = solver.solve_wave_equation(c, \"x\", \"t\", PyExpression.integer(10))"]
    #[doc = " ```"]
    pub fn solve_wave_equation(
        &self,
        wave_speed: &PyExpression,
        spatial_var: String,
        time_var: String,
        _domain_length: &PyExpression,
    ) -> PyResult<PyExpression> {
        let x_sym = Symbol::new(&spatial_var);
        let t_sym = Symbol::new(&time_var);
        let pde = Pde::new(wave_speed.inner.clone(), x_sym.clone(), vec![x_sym, t_sym]);
        let solver = WaveEquationSolver::new();
        match solver.solve_wave_equation_1d(
            &pde,
            &wave_speed.inner,
            &[],
            &InitialCondition::Value {
                function: wave_speed.inner.clone(),
            },
            &InitialCondition::Value {
                function: wave_speed.inner.clone(),
            },
        ) {
            Ok(solution) => Ok(PyExpression {
                inner: solution.solution,
            }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Wave equation solving failed: {:?}",
                e
            ))),
        }
    }
    #[doc = " Solve Laplace equation"]
    #[doc = ""]
    #[doc = " Solves the Laplace equation: ∇²u = 0"]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = ""]
    #[doc = " * `variables` - List of spatial variable names (e.g., [\"x\", \"y\"])"]
    #[doc = ""]
    #[doc = " # Returns"]
    #[doc = ""]
    #[doc = " Expression representing the Laplace equation solution"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " from mathhook_python import PyPDESolver"]
    #[doc = ""]
    #[doc = " solver = PyPDESolver()"]
    #[doc = " solution = solver.solve_laplace_equation([\"x\", \"y\"])"]
    #[doc = " ```"]
    pub fn solve_laplace_equation(&self, variables: Vec<String>) -> PyResult<PyExpression> {
        let symbols: Vec<Symbol> = variables.iter().map(Symbol::new).collect();
        if symbols.len() != 2 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Laplace equation requires exactly 2 independent variables",
            ));
        }
        let dep_var = Symbol::new("u");
        let indep_vars = symbols;
        let pde = Pde::new(Expression::integer(0), dep_var, indep_vars);
        let solver = LaplaceEquationSolver::new();
        match solver.solve_laplace_equation_2d(&pde, &[]) {
            Ok(solution) => Ok(PyExpression {
                inner: solution.solution,
            }),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Laplace equation solving failed: {:?}",
                e
            ))),
        }
    }
}

#[pymethods]
impl PyGroebnerBasis {
    #[doc = " Create a new Gröbner basis from polynomials"]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = ""]
    #[doc = " * `polynomials` - List of polynomial expressions generating the ideal"]
    #[doc = " * `variables` - List of variable names in the polynomial ring"]
    #[doc = " * `ordering` - Monomial ordering (\"lex\", \"grlex\", or \"grevlex\")"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " from mathhook_python import PyGroebnerBasis, PyExpression"]
    #[doc = ""]
    #[doc = " x = PyExpression.symbol(\"x\")"]
    #[doc = " y = PyExpression.symbol(\"y\")"]
    #[doc = " f1 = PyExpression.parse(\"x^2 + y^2 - 1\")"]
    #[doc = " f2 = PyExpression.parse(\"x - y\")"]
    #[doc = " gb = PyGroebnerBasis([f1, f2], [\"x\", \"y\"], \"lex\")"]
    #[doc = " ```"]
    #[new]
    pub fn new(
        polynomials: Vec<PyExpression>,
        variables: Vec<String>,
        ordering: String,
    ) -> PyResult<Self> {
        let poly_exprs: Vec<Expression> = polynomials.iter().map(|p| p.inner.clone()).collect();
        let symbols: Vec<Symbol> = variables.iter().map(Symbol::new).collect();
        let order = match ordering.as_str() {
            "lex" => MonomialOrder::Lex,
            "grlex" => MonomialOrder::Grlex,
            "grevlex" => MonomialOrder::Grevlex,
            _ => {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Unknown monomial ordering: {}. Use 'lex', 'grlex', or 'grevlex'",
                    ordering
                )))
            }
        };
        Ok(Self {
            inner: GroebnerBasis::new(poly_exprs, symbols, order),
        })
    }
    #[doc = " Compute the Gröbner basis using Buchberger's algorithm"]
    #[doc = ""]
    #[doc = " Transforms the initial generators into a Gröbner basis by computing"]
    #[doc = " S-polynomials and adding non-zero remainders to the basis."]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " from mathhook_python import PyGroebnerBasis, PyExpression"]
    #[doc = ""]
    #[doc = " x = PyExpression.symbol(\"x\")"]
    #[doc = " y = PyExpression.symbol(\"y\")"]
    #[doc = " f1 = PyExpression.parse(\"x^2 + y^2 - 1\")"]
    #[doc = " f2 = PyExpression.parse(\"x - y\")"]
    #[doc = " gb = PyGroebnerBasis([f1, f2], [\"x\", \"y\"], \"lex\")"]
    #[doc = " gb.compute()"]
    #[doc = " basis_polys = gb.get_basis()"]
    #[doc = " ```"]
    pub fn compute(&mut self) -> PyResult<()> {
        match self.inner.compute_with_result() {
            Ok(()) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Gröbner basis computation failed: {:?}",
                e
            ))),
        }
    }
    #[doc = " Get the basis polynomials"]
    #[doc = ""]
    #[doc = " # Returns"]
    #[doc = ""]
    #[doc = " List of polynomial expressions in the Gröbner basis"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " from mathhook_python import PyGroebnerBasis"]
    #[doc = ""]
    #[doc = " gb = PyGroebnerBasis([...], [\"x\", \"y\"], \"lex\")"]
    #[doc = " gb.compute()"]
    #[doc = " basis = gb.get_basis()"]
    #[doc = " for poly in basis:"]
    #[doc = "     print(poly.to_simple())"]
    #[doc = " ```"]
    pub fn get_basis(&self) -> Vec<PyExpression> {
        self.inner
            .basis
            .iter()
            .map(|expr| PyExpression {
                inner: expr.clone(),
            })
            .collect()
    }
    #[doc = " Test if a polynomial is in the ideal generated by this basis"]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = ""]
    #[doc = " * `poly` - Polynomial to test for membership"]
    #[doc = ""]
    #[doc = " # Returns"]
    #[doc = ""]
    #[doc = " True if the polynomial reduces to zero modulo the basis"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " from mathhook_python import PyGroebnerBasis, PyExpression"]
    #[doc = ""]
    #[doc = " x = PyExpression.symbol(\"x\")"]
    #[doc = " y = PyExpression.symbol(\"y\")"]
    #[doc = " f1 = PyExpression.parse(\"x - y\")"]
    #[doc = " f2 = PyExpression.parse(\"y^2 - 1\")"]
    #[doc = " gb = PyGroebnerBasis([f1, f2], [\"x\", \"y\"], \"lex\")"]
    #[doc = " gb.compute()"]
    #[doc = ""]
    #[doc = " test = PyExpression.parse(\"x^2 - 1\")"]
    #[doc = " assert gb.contains(test)  # x^2 - 1 is in the ideal"]
    #[doc = " ```"]
    pub fn contains(&self, poly: &PyExpression) -> bool {
        self.inner.contains(&poly.inner)
    }
    #[doc = " Reduce the Gröbner basis to minimal form"]
    #[doc = ""]
    #[doc = " A reduced Gröbner basis has:"]
    #[doc = " 1. Leading coefficients are 1 (monic)"]
    #[doc = " 2. No monomial of any basis element is divisible by the leading"]
    #[doc = "    term of another basis element"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " from mathhook_python import PyGroebnerBasis"]
    #[doc = ""]
    #[doc = " gb = PyGroebnerBasis([...], [\"x\", \"y\"], \"lex\")"]
    #[doc = " gb.compute()"]
    #[doc = " gb.reduce()"]
    #[doc = " ```"]
    pub fn reduce(&mut self) {
        self.inner.reduce();
    }
    #[doc = " Check if the basis is reduced"]
    #[doc = ""]
    #[doc = " # Returns"]
    #[doc = ""]
    #[doc = " True if the basis is in reduced form"]
    pub fn is_reduced(&self) -> bool {
        self.inner.is_reduced
    }
}
