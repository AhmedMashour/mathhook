//! Solver module for MathHook Python bindings
//!
//! This module was automatically extracted from lib.rs using syn-based refactoring.

use crate::PyExpression;

use crate::types::PySolverResult;
use mathhook_core::{MathSolver, Symbol};
use pyo3::prelude::*;

#[doc = " Python wrapper for MathSolver"]
#[pyclass]
pub struct PyMathSolver {
    inner: MathSolver,
}

#[pymethods]
impl PyMathSolver {
    #[doc = " Create a new solver"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " solver = PyMathSolver()"]
    #[doc = " ```"]
    #[new]
    pub fn new() -> Self {
        Self {
            inner: MathSolver::new(),
        }
    }
    #[doc = " Solve an equation and return structured results"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```python"]
    #[doc = " solver = PyMathSolver()"]
    #[doc = " x = PyExpression.symbol(\"x\")"]
    #[doc = " five = PyExpression.integer(5)"]
    #[doc = " equation = PyExpression.equation(x, five)"]
    #[doc = " result = solver.solve(equation, \"x\")"]
    #[doc = " for solution in result.solutions:"]
    #[doc = "     print(solution.to_simple())"]
    #[doc = " ```"]
    pub fn solve(&mut self, equation: &PyExpression, variable: &str) -> PySolverResult {
        use mathhook_core::solvers::SolverResult;
        let symbol = Symbol::new(variable);
        let result = self.inner.solve(&equation.inner, &symbol);
        let (solutions, metadata) = match result {
            SolverResult::Single(expr) => (
                vec![PyExpression { inner: expr }],
                Some(format!("Single solution for variable: {}", variable)),
            ),
            SolverResult::Multiple(exprs) => {
                let count = exprs.len();
                let solutions = exprs
                    .into_iter()
                    .map(|expr| PyExpression { inner: expr })
                    .collect();
                (
                    solutions,
                    Some(format!(
                        "Found {} solutions for variable: {}",
                        count, variable
                    )),
                )
            }
            SolverResult::NoSolution => (vec![], Some("No solution found".to_string())),
            SolverResult::InfiniteSolutions => {
                (vec![], Some("Infinite solutions exist".to_string()))
            }
        };
        PySolverResult {
            solutions,
            metadata,
        }
    }
}

impl Default for PyMathSolver {
    fn default() -> Self {
        Self::new()
    }
}
