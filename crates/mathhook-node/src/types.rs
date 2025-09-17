//! Types module for MathHook Node.js bindings
//!
//! This module was automatically extracted from lib.rs using syn-based refactoring.

use crate::JsExpression;
use mathhook_core::calculus::pde::standard::heat::HeatEquationSolver;
use mathhook_core::calculus::pde::standard::laplace::LaplaceEquationSolver;
use mathhook_core::calculus::pde::standard::wave::WaveEquationSolver;
use mathhook_core::calculus::pde::types::{InitialCondition, Pde};
use mathhook_core::{expr, symbol, Expression, MathSolver, Symbol};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[doc = " Step in explanation"]
#[napi(object)]
pub struct JsStep {
    pub title: String,
    pub description: String,
    pub before: String,
    pub after: String,
}

#[doc = " Step-by-step explanation"]
#[napi(object)]
pub struct JsStepByStepExplanation {
    pub steps: Vec<JsStep>,
}

#[doc = " LU Decomposition result"]
#[napi]
pub struct LUDecompositionResult {
    pub(crate) l: JsExpression,
    pub(crate) u: JsExpression,
    pub(crate) p: Option<JsExpression>,
}

#[doc = " QR Decomposition result"]
#[napi]
pub struct QRDecompositionResult {
    pub(crate) q: JsExpression,
    pub(crate) r: JsExpression,
}

#[doc = " SVD Decomposition result"]
#[napi]
pub struct SVDDecompositionResult {
    pub(crate) u: JsExpression,
    pub(crate) sigma: JsExpression,
    pub(crate) vt: JsExpression,
}

#[doc = " Solver result wrapper for JavaScript"]
#[napi(object)]
pub struct JsSolverResult {
    #[doc = " Type of result: \"single\", \"multiple\", \"no_solution\", \"infinite_solutions\""]
    pub result_type: String,
    #[doc = " Solution expressions as strings (empty for no_solution/infinite_solutions)"]
    pub solutions: Vec<String>,
    #[doc = " Number of solutions found"]
    pub count: u32,
    #[doc = " Optional metadata about the solution"]
    pub metadata: Option<String>,
}

#[doc = " JavaScript wrapper for MathSolver"]
#[napi]
pub struct JsMathSolver {
    inner: MathSolver,
}

#[doc = " PDE Solution result"]
#[napi(object)]
pub struct PDESolution {
    #[doc = " Solution expression"]
    pub solution: String,
    #[doc = " Method used for solving"]
    pub method: String,
    #[doc = " Eigenvalues (for separation of variables)"]
    pub eigenvalues: Option<Vec<String>>,
    #[doc = " Fourier coefficients"]
    pub coefficients: Option<Vec<String>>,
}

#[doc = " PDE Solver for Partial Differential Equations"]
#[napi]
pub struct JsPDESolver {
    heat_solver: HeatEquationSolver,
    wave_solver: WaveEquationSolver,
    laplace_solver: LaplaceEquationSolver,
}

#[napi]
impl LUDecompositionResult {
    #[doc = " Get lower triangular matrix L"]
    #[napi(getter)]
    pub fn get_l(&self) -> JsExpression {
        JsExpression {
            inner: self.l.inner.clone(),
        }
    }
    #[doc = " Get upper triangular matrix U"]
    #[napi(getter)]
    pub fn get_u(&self) -> JsExpression {
        JsExpression {
            inner: self.u.inner.clone(),
        }
    }
    #[doc = " Get permutation matrix P (if pivoting was needed)"]
    #[napi(getter)]
    pub fn get_p(&self) -> Option<JsExpression> {
        self.p.as_ref().map(|p| JsExpression {
            inner: p.inner.clone(),
        })
    }
}

#[napi]
impl QRDecompositionResult {
    #[doc = " Get orthogonal matrix Q"]
    #[napi(getter)]
    pub fn get_q(&self) -> JsExpression {
        JsExpression {
            inner: self.q.inner.clone(),
        }
    }
    #[doc = " Get upper triangular matrix R"]
    #[napi(getter)]
    pub fn get_r(&self) -> JsExpression {
        JsExpression {
            inner: self.r.inner.clone(),
        }
    }
}

#[napi]
impl SVDDecompositionResult {
    #[doc = " Get left singular vectors U"]
    #[napi(getter)]
    pub fn get_u(&self) -> JsExpression {
        JsExpression {
            inner: self.u.inner.clone(),
        }
    }
    #[doc = " Get singular values (diagonal matrix Σ)"]
    #[napi(getter)]
    pub fn get_sigma(&self) -> JsExpression {
        JsExpression {
            inner: self.sigma.inner.clone(),
        }
    }
    #[doc = " Get right singular vectors V^T"]
    #[napi(getter)]
    pub fn get_vt(&self) -> JsExpression {
        JsExpression {
            inner: self.vt.inner.clone(),
        }
    }
}

#[napi]
impl JsMathSolver {
    #[doc = " Create a new solver"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```javascript"]
    #[doc = " const solver = new JsMathSolver();"]
    #[doc = " ```"]
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: MathSolver::new(),
        }
    }
    #[doc = " Solve an equation and return structured result"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```javascript"]
    #[doc = " const solver = new JsMathSolver();"]
    #[doc = " const x = JsExpression.symbol(\"x\");"]
    #[doc = " const five = JsExpression.integer(5);"]
    #[doc = " const equation = JsExpression.equation(x, five);"]
    #[doc = " const result = solver.solve(equation, \"x\");"]
    #[doc = " // result = { resultType: \"single\", solutions: [\"5\"], count: 1, metadata: \"Single solution found\" }"]
    #[doc = " ```"]
    #[napi]
    pub fn solve(&mut self, equation: &JsExpression, variable: String) -> JsSolverResult {
        use mathhook_core::SolverResult;
        let symbol = Symbol::new(variable);
        let result = self.inner.solve(&equation.inner, &symbol);
        match result {
            SolverResult::Single(expr) => JsSolverResult {
                result_type: "single".to_string(),
                solutions: vec![format!("{}", expr)],
                count: 1,
                metadata: Some("Single solution found".to_string()),
            },
            SolverResult::Multiple(exprs) => {
                let count = exprs.len() as u32;
                JsSolverResult {
                    result_type: "multiple".to_string(),
                    solutions: exprs.iter().map(|e| format!("{}", e)).collect(),
                    count,
                    metadata: Some(format!("{} solutions found", count)),
                }
            }
            SolverResult::NoSolution => JsSolverResult {
                result_type: "no_solution".to_string(),
                solutions: vec![],
                count: 0,
                metadata: Some("No solution exists for this equation".to_string()),
            },
            SolverResult::InfiniteSolutions => JsSolverResult {
                result_type: "infinite_solutions".to_string(),
                solutions: vec![],
                count: 0,
                metadata: Some("Infinite solutions exist (identity equation)".to_string()),
            },
        }
    }
}

impl Default for JsMathSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[napi]
impl JsPDESolver {
    #[doc = " Create a new PDE solver"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```javascript"]
    #[doc = " const solver = new JsPDESolver();"]
    #[doc = " ```"]
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            heat_solver: HeatEquationSolver::new(),
            wave_solver: WaveEquationSolver::new(),
            laplace_solver: LaplaceEquationSolver::new(),
        }
    }
    #[doc = " Solve the heat equation ∂u/∂t = α∇²u"]
    #[doc = ""]
    #[doc = " For 1D heat equation with Dirichlet boundary conditions and initial temperature distribution."]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = ""]
    #[doc = " * `dependent_var` - Dependent variable (e.g., \"u\")"]
    #[doc = " * `spatial_var` - Spatial variable (e.g., \"x\")"]
    #[doc = " * `temporal_var` - Time variable (e.g., \"t\")"]
    #[doc = " * `alpha` - Thermal diffusivity coefficient"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```javascript"]
    #[doc = " const solver = new JsPDESolver();"]
    #[doc = " const alpha = JsExpression.integer(1);"]
    #[doc = " const solution = solver.solveHeatEquation(\"u\", \"x\", \"t\", alpha);"]
    #[doc = " // Returns heat equation solution with Fourier series"]
    #[doc = " ```"]
    #[napi]
    #[allow(deprecated)]
    pub fn solve_heat_equation(
        &self,
        _dependent_var: String,
        _spatial_var: String,
        _temporal_var: String,
        alpha: &JsExpression,
    ) -> Result<PDESolution> {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = Expression::symbol(u.clone());
        let pde = Pde::new(equation, u, vec![x, t]);
        let ic = InitialCondition::value(expr!(1));
        match self
            .heat_solver
            .solve_heat_equation_1d(&pde, &alpha.inner, &[], &ic)
        {
            Ok(result) => Ok(PDESolution {
                solution: format!("{}", result.solution),
                method: "Separation of Variables (Heat Equation)".to_string(),
                eigenvalues: Some(
                    result
                        .eigenvalues
                        .iter()
                        .map(|e| format!("{}", e))
                        .collect(),
                ),
                coefficients: Some(
                    result
                        .coefficients
                        .iter()
                        .map(|c| format!("{}", c))
                        .collect(),
                ),
            }),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Failed to solve heat equation: {:?}", e),
            )),
        }
    }
    #[doc = " Solve the wave equation ∂²u/∂t² = c²∇²u"]
    #[doc = ""]
    #[doc = " For 1D wave equation with Dirichlet boundary conditions and initial displacement/velocity."]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = ""]
    #[doc = " * `dependent_var` - Dependent variable (e.g., \"u\")"]
    #[doc = " * `spatial_var` - Spatial variable (e.g., \"x\")"]
    #[doc = " * `temporal_var` - Time variable (e.g., \"t\")"]
    #[doc = " * `wave_speed` - Wave propagation speed c"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```javascript"]
    #[doc = " const solver = new JsPDESolver();"]
    #[doc = " const c = JsExpression.integer(1);"]
    #[doc = " const solution = solver.solveWaveEquation(\"u\", \"x\", \"t\", c);"]
    #[doc = " // Returns wave equation solution with Fourier series"]
    #[doc = " ```"]
    #[napi]
    pub fn solve_wave_equation(
        &self,
        _dependent_var: String,
        _spatial_var: String,
        _temporal_var: String,
        wave_speed: &JsExpression,
    ) -> Result<PDESolution> {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = Expression::symbol(u.clone());
        let pde = Pde::new(equation, u, vec![x, t]);
        let ic_pos = InitialCondition::value(expr!(1));
        let ic_vel = InitialCondition::derivative(expr!(0));
        match self.wave_solver.solve_wave_equation_1d(
            &pde,
            &wave_speed.inner,
            &[],
            &ic_pos,
            &ic_vel,
        ) {
            Ok(result) => {
                let mut all_coeffs: Vec<String> = result
                    .position_coefficients
                    .iter()
                    .map(|c| format!("{}", c))
                    .collect();
                all_coeffs.extend(
                    result
                        .velocity_coefficients
                        .iter()
                        .map(|c| format!("{}", c)),
                );
                Ok(PDESolution {
                    solution: format!("{}", result.solution),
                    method: "Separation of Variables (Wave Equation)".to_string(),
                    eigenvalues: Some(
                        result
                            .eigenvalues
                            .iter()
                            .map(|e| format!("{}", e))
                            .collect(),
                    ),
                    coefficients: Some(all_coeffs),
                })
            }
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Failed to solve wave equation: {:?}", e),
            )),
        }
    }
    #[doc = " Solve the Laplace equation ∇²u = 0"]
    #[doc = ""]
    #[doc = " For 2D Laplace equation on rectangular domain with Dirichlet boundary conditions."]
    #[doc = ""]
    #[doc = " # Arguments"]
    #[doc = ""]
    #[doc = " * `dependent_var` - Dependent variable (e.g., \"u\")"]
    #[doc = " * `x_var` - First spatial variable (e.g., \"x\")"]
    #[doc = " * `y_var` - Second spatial variable (e.g., \"y\")"]
    #[doc = ""]
    #[doc = " # Examples"]
    #[doc = ""]
    #[doc = " ```javascript"]
    #[doc = " const solver = new JsPDESolver();"]
    #[doc = " const solution = solver.solveLaplaceEquation(\"u\", \"x\", \"y\");"]
    #[doc = " // Returns Laplace equation solution with Fourier series"]
    #[doc = " ```"]
    #[napi]
    pub fn solve_laplace_equation(
        &self,
        _dependent_var: String,
        _x_var: String,
        _y_var: String,
    ) -> Result<PDESolution> {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = Expression::symbol(u.clone());
        let pde = Pde::new(equation, u, vec![x, y]);
        match self.laplace_solver.solve_laplace_equation_2d(&pde, &[]) {
            Ok(result) => Ok(PDESolution {
                solution: format!("{}", result.solution),
                method: "Separation of Variables (Laplace Equation)".to_string(),
                eigenvalues: Some(
                    result
                        .x_eigenvalues
                        .iter()
                        .map(|e| format!("{}", e))
                        .collect(),
                ),
                coefficients: Some(
                    result
                        .coefficients
                        .iter()
                        .map(|c| format!("{}", c))
                        .collect(),
                ),
            }),
            Err(e) => Err(Error::new(
                Status::GenericFailure,
                format!("Failed to solve Laplace equation: {:?}", e),
            )),
        }
    }
}

impl Default for JsPDESolver {
    fn default() -> Self {
        Self::new()
    }
}
