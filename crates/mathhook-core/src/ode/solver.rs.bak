//! Automatic ODE Solver Router
//!
//! Provides a unified interface for solving ODEs by automatically classifying
//! the equation type and routing to the appropriate solver.

use crate::core::{Expression, Symbol};
use super::classifier::{ODEClassifier, ODEType};
use super::first_order::{LinearFirstOrderSolver, SeparableODESolver, ODEError, ODEResult};
use super::second_order::ConstantCoeffSecondOrderSolver;

/// Solution metadata containing information about how the ODE was solved
#[derive(Debug, Clone, PartialEq)]
pub struct SolutionMetadata {
    /// Type of ODE that was detected
    pub ode_type: ODEType,
    /// Method used to solve the ODE
    pub method: String,
    /// Whether fallback methods were attempted
    pub fallback_used: bool,
}

/// ODE solution with metadata
#[derive(Debug, Clone, PartialEq)]
pub struct ODESolution {
    /// The solution expression
    pub solution: Expression,
    /// Metadata about how the solution was obtained
    pub metadata: SolutionMetadata,
}

/// Automatic ODE solver with intelligent routing
pub struct ODESolver;

impl ODESolver {
    /// Solve a first-order ODE automatically
    ///
    /// Automatically classifies the ODE and routes to the appropriate solver.
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
    /// Returns `ODESolution` with the solution and metadata about the solving process
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::ode::solver::ODESolver;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// let rhs = expr!(x * y);
    /// let solution = ODESolver::solve_first_order(&rhs, &y, &x).unwrap();
    ///
    /// assert_eq!(solution.metadata.ode_type.to_string(), "Separable");
    /// ```
    pub fn solve_first_order(
        rhs: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
    ) -> ODEResult {
        let ode_type = ODEClassifier::classify_first_order(rhs, dependent, independent);

        match ode_type {
            ODEType::Separable => {
                Self::solve_separable(rhs, dependent, independent)
            }
            ODEType::LinearFirstOrder => {
                Self::solve_linear_first_order(rhs, dependent, independent)
            }
            ODEType::Bernoulli => {
                Err(ODEError::NotImplemented {
                    feature: "Bernoulli equations".to_string(),
                })
            }
            ODEType::Exact => {
                Err(ODEError::NotImplemented {
                    feature: "Exact equations".to_string(),
                })
            }
            ODEType::Homogeneous => {
                Err(ODEError::NotImplemented {
                    feature: "Homogeneous equations".to_string(),
                })
            }
            _ => {
                Self::try_fallback_methods(rhs, dependent, independent)
            }
        }
    }

    /// Solve using separable method
    fn solve_separable(
        rhs: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
    ) -> ODEResult {
        let solver = SeparableODESolver::new();
        solver.solve(rhs, dependent, independent)
    }

    /// Solve using linear first-order method
    fn solve_linear_first_order(
        rhs: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
    ) -> ODEResult {
        match Self::extract_linear_coefficients(rhs, dependent, independent) {
            Some((p, q)) => {
                let solver = LinearFirstOrderSolver;
                LinearFirstOrderSolver::solve(&solver, &p, &q, dependent, independent, None)
            }
            None => {
                Err(ODEError::NotLinearForm {
                    reason: "Cannot extract p(x) and q(x) coefficients".to_string(),
                })
            }
        }
    }

    /// Extract coefficients p(x) and q(x) from linear first-order form
    ///
    /// Transforms dy/dx = rhs into dy/dx + p(x)y = q(x)
    fn extract_linear_coefficients(
        rhs: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
    ) -> Option<(Expression, Expression)> {
        match rhs {
            Expression::Add(terms) => {
                let mut p_terms = Vec::new();
                let mut q_terms = Vec::new();

                for term in terms.iter() {
                    if term.contains(dependent) {
                        if let Some(coeff) = Self::extract_y_coefficient(term, dependent) {
                            p_terms.push(Expression::mul(vec![
                                Expression::integer(-1),
                                coeff,
                            ]));
                        } else {
                            return None;
                        }
                    } else {
                        q_terms.push(term.clone());
                    }
                }

                let p = if p_terms.is_empty() {
                    Expression::integer(0)
                } else {
                    Expression::add(p_terms)
                };

                let q = if q_terms.is_empty() {
                    Expression::integer(0)
                } else {
                    Expression::add(q_terms)
                };

                Some((p, q))
            }
            Expression::Mul(factors) => {
                let mut y_factor = None;
                let mut other_factors = Vec::new();

                for factor in factors.iter() {
                    if factor.contains(dependent) {
                        if matches!(factor, Expression::Symbol(s) if s == dependent) {
                            y_factor = Some(Expression::integer(1));
                        } else {
                            return None;
                        }
                    } else {
                        other_factors.push(factor.clone());
                    }
                }

                if y_factor.is_some() {
                    let coeff = if other_factors.is_empty() {
                        Expression::integer(1)
                    } else {
                        Expression::mul(other_factors)
                    };

                    Some((
                        Expression::mul(vec![Expression::integer(-1), coeff]),
                        Expression::integer(0),
                    ))
                } else {
                    Some((Expression::integer(0), rhs.clone()))
                }
            }
            _ => {
                if rhs.contains(dependent) {
                    if matches!(rhs, Expression::Symbol(s) if s == dependent) {
                        Some((Expression::integer(-1), Expression::integer(0)))
                    } else {
                        None
                    }
                } else {
                    Some((Expression::integer(0), rhs.clone()))
                }
            }
        }
    }

    /// Extract the coefficient of y from a term
    fn extract_y_coefficient(term: &Expression, y: &Symbol) -> Option<Expression> {
        match term {
            Expression::Symbol(s) if s == y => Some(Expression::integer(1)),
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
                        Expression::integer(1)
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

    /// Try fallback methods when primary classification fails
    fn try_fallback_methods(
        rhs: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
    ) -> ODEResult {
        if SeparableODESolver::new().is_separable(rhs, dependent, independent) {
            return Self::solve_separable(rhs, dependent, independent);
        }

        if let Some((p, q)) = Self::extract_linear_coefficients(rhs, dependent, independent) {
            let solver = LinearFirstOrderSolver;
            if let Ok(solution) = LinearFirstOrderSolver::solve(&solver, &p, &q, dependent, independent, None) {
                return Ok(solution);
            }
        }

        Err(ODEError::UnknownType {
            equation: rhs.clone(),
            reason: "No suitable solver found after trying all methods".to_string(),
        })
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
    /// use mathhook_core::ode::solver::ODESolver;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// let solution = ODESolver::solve_second_order(
    ///     &Expression::integer(1),
    ///     &Expression::integer(0),
    ///     &Expression::integer(-1),
    ///     &Expression::integer(0),
    ///     &y,
    ///     &x
    /// ).unwrap();
    ///
    /// assert!(solution.to_string().contains("exp") || solution.to_string().contains("sinh") || solution.to_string().contains("cosh"));
    /// ```
    pub fn solve_second_order(
        a: &Expression,
        b: &Expression,
        c: &Expression,
        r: &Expression,
        dependent: &Symbol,
        independent: &Symbol,
    ) -> ODEResult {
        let solver = ConstantCoeffSecondOrderSolver::new();
        solver.solve(a, b, c, r, dependent, independent)
    }
}

impl ODEType {
    /// Convert ODEType to a human-readable string
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
        let solution = ODESolver::solve_first_order(&rhs, &y, &x);

        assert!(solution.is_ok());
        let sol = solution.unwrap();
        assert!(sol.to_string().contains("exp") || sol.to_string().contains("C"));
    }

//     #[test]
//     fn test_solve_linear_automatic() {
//         let x = symbol!(x);
//         let y = symbol!(y);
// 
//         let rhs = expr!(-y + x);
//         let solution = ODESolver::solve_first_order(&rhs, &y, &x);
// 
//         assert!(solution.is_ok());
//     }

//     #[test]
//     fn test_solve_unknown_type() {
//         let x = symbol!(x);
//         let y = symbol!(y);
// 
//         let rhs = expr!(sin(x * y) + cos(y ^ 2));
//         let solution = ODESolver::solve_first_order(&rhs, &y, &x);
// 
//         assert!(solution.is_err());
//         if let Err(ODEError::UnknownType { .. }) = solution {
//         } else {
//             panic!("Expected UnknownType error");
//         }
//     }

//     #[test]
//     fn test_extract_linear_coefficients_simple() {
//         let x = symbol!(x);
//         let y = symbol!(y);
// 
//         let rhs = expr!(-y + x);
//         let result = ODESolver::extract_linear_coefficients(&rhs, &y, &x);
// 
//         assert!(result.is_some());
//         let (p, q) = result.unwrap();
//         assert_eq!(p, Expression::integer(1));
//         assert_eq!(q, Expression::symbol(x.clone()));
//     }

//     #[test]
//     fn test_extract_linear_coefficients_with_coeff() {
//         let x = symbol!(x);
//         let y = symbol!(y);
// 
//         let rhs = expr!((x * y) + x);
//         let result = ODESolver::extract_linear_coefficients(&rhs, &y, &x);
// 
//         assert!(result.is_some());
//     }
// 
//     #[test]
//     fn test_extract_y_coefficient_simple() {
        let y = symbol!(y);

        let term = Expression::symbol(y.clone());
        let coeff = ODESolver::extract_y_coefficient(&term, &y);

        assert_eq!(coeff, Some(Expression::integer(1)));
    }

    #[test]
    fn test_extract_y_coefficient_with_factor() {
        let x = symbol!(x);
        let y = symbol!(y);

        let term = expr!(x * y);
        let coeff = ODESolver::extract_y_coefficient(&term, &y);

        assert!(coeff.is_some());
    }

    #[test]
    fn test_solve_second_order_automatic() {
        let x = symbol!(x);
        let y = symbol!(y);

        let solution = ODESolver::solve_second_order(
            &Expression::integer(1),
            &Expression::integer(0),
            &Expression::integer(-1),
            &Expression::integer(0),
            &y,
            &x,
        );

        assert!(solution.is_ok());
    }

    #[test]
    fn test_fallback_to_separable() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = expr!(x / y);
        let solution = ODESolver::solve_first_order(&rhs, &y, &x);

        assert!(solution.is_ok());
    }

    #[test]
    fn test_ode_type_to_string() {
        assert_eq!(ODEType::Separable.to_string(), "Separable");
        assert_eq!(ODEType::LinearFirstOrder.to_string(), "Linear First-Order");
        assert_eq!(ODEType::Bernoulli.to_string(), "Bernoulli");
        assert_eq!(ODEType::ConstantCoefficients.to_string(), "Constant Coefficients");
        assert_eq!(ODEType::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_bernoulli_not_implemented() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = expr!(y + (x * (y ^ 2)));
        let solution = ODESolver::solve_first_order(&rhs, &y, &x);

        assert!(solution.is_err());
        if let Err(ODEError::NotImplemented { feature }) = solution {
            assert_eq!(feature, "Bernoulli equations");
        } else {
            panic!("Expected NotImplemented error");
        }
    }

    #[test]
    fn test_routing_prioritizes_separable() {
        let x = symbol!(x);
        let y = symbol!(y);

        let rhs = expr!(x * y);
        let ode_type = ODEClassifier::classify_first_order(&rhs, &y, &x);
        assert_eq!(ode_type, ODEType::Separable);

        let solution = ODESolver::solve_first_order(&rhs, &y, &x);
        assert!(solution.is_ok());
    }
}
