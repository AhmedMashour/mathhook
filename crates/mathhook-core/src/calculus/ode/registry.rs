//! ODE Solver Registry
//!
//! Registry-based dispatch system for ODE solvers, eliminating hardcoded match patterns.

use super::classifier::ODEType;
use super::first_order::{
    HomogeneousODESolver, LinearFirstOrderSolver, ODEError, ODEResult, SeparableODESolver,
};
use crate::core::{Expression, Symbol};
use std::collections::HashMap;
use std::sync::Arc;

/// Check if an expression contains a given symbol
fn contains_symbol(expr: &Expression, sym: &Symbol) -> bool {
    match expr {
        Expression::Symbol(s) => s == sym,
        Expression::Add(terms) | Expression::Mul(terms) => {
            terms.iter().any(|t| contains_symbol(t, sym))
        }
        Expression::Pow(base, exp) => contains_symbol(base, sym) || contains_symbol(exp, sym),
        Expression::Function { args, .. } => args.iter().any(|a| contains_symbol(a, sym)),
        _ => false,
    }
}

/// Trait for first-order ODE solvers
pub trait FirstOrderSolver: Send + Sync {
    fn solve(&self, rhs: &Expression, dependent: &Symbol, independent: &Symbol) -> ODEResult;

    fn can_solve(&self, rhs: &Expression, dependent: &Symbol, independent: &Symbol) -> bool;

    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

struct SeparableSolverAdapter;

impl FirstOrderSolver for SeparableSolverAdapter {
    fn solve(&self, rhs: &Expression, dependent: &Symbol, independent: &Symbol) -> ODEResult {
        let solver = SeparableODESolver::new();
        solver.solve(rhs, dependent, independent, None)
    }

    #[inline]
    fn can_solve(&self, rhs: &Expression, dependent: &Symbol, independent: &Symbol) -> bool {
        SeparableODESolver::new().is_separable(rhs, dependent, independent)
    }

    #[inline]
    fn name(&self) -> &'static str {
        "Separable"
    }

    #[inline]
    fn description(&self) -> &'static str {
        "Solves separable ODEs of the form dy/dx = g(x)h(y)"
    }
}

struct LinearFirstOrderSolverAdapter;

impl FirstOrderSolver for LinearFirstOrderSolverAdapter {
    fn solve(&self, rhs: &Expression, dependent: &Symbol, independent: &Symbol) -> ODEResult {
        let (p, q) = extract_linear_coefficients(rhs, dependent, independent)?;
        let solver = LinearFirstOrderSolver;
        LinearFirstOrderSolver::solve(&solver, &p, &q, dependent, independent, None)
    }

    #[inline]
    fn can_solve(&self, rhs: &Expression, dependent: &Symbol, independent: &Symbol) -> bool {
        extract_linear_coefficients(rhs, dependent, independent).is_ok()
    }

    #[inline]
    fn name(&self) -> &'static str {
        "Linear First-Order"
    }

    #[inline]
    fn description(&self) -> &'static str {
        "Solves linear first-order ODEs using integrating factor method"
    }
}

struct HomogeneousSolverAdapter;

impl FirstOrderSolver for HomogeneousSolverAdapter {
    fn solve(&self, rhs: &Expression, dependent: &Symbol, independent: &Symbol) -> ODEResult {
        let solver = HomogeneousODESolver;
        solver.solve(rhs, dependent, independent)
    }

    #[inline]
    fn can_solve(&self, rhs: &Expression, dependent: &Symbol, independent: &Symbol) -> bool {
        HomogeneousODESolver.is_homogeneous(rhs, dependent, independent)
    }

    #[inline]
    fn name(&self) -> &'static str {
        "Homogeneous"
    }

    #[inline]
    fn description(&self) -> &'static str {
        "Solves homogeneous ODEs of the form dy/dx = f(y/x)"
    }
}

pub struct ODESolverRegistry {
    solvers: HashMap<ODEType, Arc<dyn FirstOrderSolver>>,
    priority_order: Vec<ODEType>,
}

impl ODESolverRegistry {
    pub fn new() -> Self {
        let mut solvers: HashMap<ODEType, Arc<dyn FirstOrderSolver>> = HashMap::new();

        solvers.insert(ODEType::Separable, Arc::new(SeparableSolverAdapter));
        solvers.insert(
            ODEType::LinearFirstOrder,
            Arc::new(LinearFirstOrderSolverAdapter),
        );
        solvers.insert(ODEType::Homogeneous, Arc::new(HomogeneousSolverAdapter));

        let priority_order = vec![
            ODEType::Separable,
            ODEType::LinearFirstOrder,
            ODEType::Homogeneous,
        ];

        Self {
            solvers,
            priority_order,
        }
    }

    #[inline]
    pub fn get_solver(&self, ode_type: &ODEType) -> Option<&Arc<dyn FirstOrderSolver>> {
        self.solvers.get(ode_type)
    }

    pub fn try_all_solvers(
        &self,
        rhs: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
    ) -> ODEResult {
        for ode_type in &self.priority_order {
            if let Some(solver) = self.solvers.get(ode_type) {
                if solver.can_solve(rhs, dependent, independent) {
                    return solver.solve(rhs, dependent, independent);
                }
            }
        }

        Err(ODEError::UnknownType {
            equation: rhs.clone(),
            reason: "No suitable solver found after trying all registered methods".to_owned(),
        })
    }
}

impl Default for ODESolverRegistry {
    fn default() -> Self {
        Self::new()
    }
}

fn extract_linear_coefficients(
    rhs: &Expression,
    dependent: &Symbol,
    _independent: &Symbol,
) -> Result<(Expression, Expression), ODEError> {
    use crate::expr;

    match rhs {
        Expression::Add(terms) => {
            let mut p_terms = Vec::new();
            let mut q_terms = Vec::new();

            for term in terms.iter() {
                if contains_symbol(term, dependent) {
                    if let Some(_coeff) = extract_y_coefficient(term, dependent) {
                        p_terms.push(expr!((-1) * _coeff));
                    } else {
                        return Err(ODEError::NotLinearForm {
                            reason: "Cannot extract coefficient from term containing y".to_owned(),
                        });
                    }
                } else {
                    q_terms.push(term.clone());
                }
            }

            let p = if p_terms.is_empty() {
                expr!(0)
            } else {
                Expression::add(p_terms)
            };

            let q = if q_terms.is_empty() {
                expr!(0)
            } else {
                Expression::add(q_terms)
            };

            Ok((p, q))
        }
        Expression::Mul(factors) => {
            let mut y_factor = None;
            let mut other_factors = Vec::new();

            for factor in factors.iter() {
                if contains_symbol(factor, dependent) {
                    if matches!(factor, Expression::Symbol(s) if s == dependent) {
                        y_factor = Some(expr!(1));
                    } else {
                        return Err(ODEError::NotLinearForm {
                            reason: "Complex y term in product".to_owned(),
                        });
                    }
                } else {
                    other_factors.push(factor.clone());
                }
            }

            if y_factor.is_some() {
                let _coeff = if other_factors.is_empty() {
                    expr!(1)
                } else {
                    Expression::mul(other_factors)
                };

                Ok((expr!((-1) * _coeff), expr!(0)))
            } else {
                Ok((expr!(0), rhs.clone()))
            }
        }
        _ => {
            if contains_symbol(rhs, dependent) {
                if matches!(rhs, Expression::Symbol(s) if s == dependent) {
                    Ok((expr!(-1), expr!(0)))
                } else {
                    Err(ODEError::NotLinearForm {
                        reason: "Cannot extract linear form".to_owned(),
                    })
                }
            } else {
                Ok((expr!(0), rhs.clone()))
            }
        }
    }
}

fn extract_y_coefficient(term: &Expression, y: &Symbol) -> Option<Expression> {
    use crate::expr;

    match term {
        Expression::Symbol(s) if s == y => Some(expr!(1)),
        Expression::Mul(factors) => {
            let mut coeff_factors = Vec::new();
            let mut found_y = false;

            for factor in factors.iter() {
                if matches!(factor, Expression::Symbol(s) if s == y) {
                    found_y = true;
                } else {
                    coeff_factors.push(factor.clone());
                }
            }

            if found_y {
                Some(if coeff_factors.is_empty() {
                    expr!(1)
                } else {
                    Expression::mul(coeff_factors)
                })
            } else {
                None
            }
        }
        _ => None,
    }
}
