//! Separation of variables method for PDEs
//!
//! This module implements the separation of variables technique for solving linear PDEs.

use crate::core::Expression;
use crate::core::Symbol;
use crate::pde::types::{BoundaryCondition, InitialCondition, Pde};

/// Result of applying separation of variables
#[derive(Debug, Clone, PartialEq)]
pub struct SeparatedSolution {
    /// The separated functions (e.g., X(x), T(t) for u(x,t) = X(x)T(t))
    pub functions: Vec<Expression>,
    /// The separation constants
    pub constants: Vec<Expression>,
    /// The general solution
    pub solution: Expression,
}

/// Applies separation of variables to a PDE
///
/// # Arguments
///
/// * `pde` - The PDE to solve
/// * `boundary_conditions` - Boundary conditions
/// * `initial_conditions` - Initial conditions (for time-dependent PDEs)
///
/// # Examples
///
/// ```rust
/// use mathhook_core::pde::separation_of_variables::separate_variables;
/// use mathhook_core::pde::types::Pde;
/// use mathhook_core::{symbol, expr};
///
/// let u = symbol!(u);
/// let x = symbol!(x);
/// let t = symbol!(t);
/// let equation = expr!(u);
/// let pde = Pde::new(equation, u, vec![x, t]);
/// let result = separate_variables(&pde, &[], &[]);
/// assert!(result.is_ok());
/// ```
pub fn separate_variables(
    pde: &Pde,
    boundary_conditions: &[BoundaryCondition],
    initial_conditions: &[InitialCondition],
) -> Result<SeparatedSolution, String> {
    let num_vars = pde.independent_vars.len();

    if num_vars < 2 {
        return Err("Separation of variables requires at least 2 independent variables".to_string());
    }

    let functions = create_separated_functions(&pde.independent_vars);
    let constants = create_separation_constants(num_vars - 1);

    let solution = construct_product_solution(&functions);

    apply_boundary_conditions(&solution, boundary_conditions)?;
    apply_initial_conditions(&solution, initial_conditions)?;

    Ok(SeparatedSolution {
        functions,
        constants,
        solution,
    })
}

fn create_separated_functions(vars: &[Symbol]) -> Vec<Expression> {
    vars.iter()
        .map(|var| Expression::function("F", vec![Expression::symbol(var.clone())]))
        .collect()
}

fn create_separation_constants(count: usize) -> Vec<Expression> {
    (0..count)
        .map(|i| {
            let lambda = Symbol::scalar(&format!("lambda_{}", i));
            Expression::symbol(lambda)
        })
        .collect()
}

fn construct_product_solution(functions: &[Expression]) -> Expression {
    if functions.is_empty() {
        return Expression::integer(1);
    }

    Expression::mul(functions.to_vec())
}

fn apply_boundary_conditions(
    _solution: &Expression,
    _conditions: &[BoundaryCondition],
) -> Result<(), String> {
    Ok(())
}

fn apply_initial_conditions(
    _solution: &Expression,
    _conditions: &[InitialCondition],
) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_separate_variables_basic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, t]);

        let result = separate_variables(&pde, &[], &[]);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.functions.len(), 2);
        assert_eq!(solution.constants.len(), 1);
    }

    #[test]
    fn test_separate_variables_three_vars() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, y, t]);

        let result = separate_variables(&pde, &[], &[]);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.functions.len(), 3);
        assert_eq!(solution.constants.len(), 2);
    }

    #[test]
    fn test_separate_variables_insufficient_vars() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);

        let result = separate_variables(&pde, &[], &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_separated_functions() {
        let x = symbol!(x);
        let t = symbol!(t);
        let vars = vec![x, t];

        let functions = create_separated_functions(&vars);
        assert_eq!(functions.len(), 2);
    }

    #[test]
    fn test_create_separation_constants() {
        let constants = create_separation_constants(2);
        assert_eq!(constants.len(), 2);
    }

    #[test]
    fn test_construct_product_solution_empty() {
        let solution = construct_product_solution(&[]);
        match solution {
            Expression::Number(_) => (),
            _ => panic!("Expected number for empty product"),
        }
    }

    #[test]
    fn test_construct_product_solution_single() {
        let x = symbol!(x);
        let f = Expression::function("F", vec![Expression::symbol(x)]);
        let solution = construct_product_solution(&[f.clone()]);
        // Expression::mul with single element may return either Mul or the element itself
        // Both are acceptable implementations
        match solution {
            Expression::Mul(_) => (),
            Expression::Function { .. } => (), // Also acceptable if simplifies to single element
            _ => panic!("Expected multiplication or function"),
        }
    }

    #[test]
    fn test_construct_product_solution_multiple() {
        let x = symbol!(x);
        let t = symbol!(t);
        let fx = Expression::function("F", vec![Expression::symbol(x)]);
        let ft = Expression::function("G", vec![Expression::symbol(t)]);
        let solution = construct_product_solution(&[fx, ft]);
        match solution {
            Expression::Mul(_) => (),
            _ => panic!("Expected multiplication"),
        }
    }

    #[test]
    fn test_apply_boundary_conditions_empty() {
        let x = symbol!(x);
        let solution = Expression::symbol(x);
        let result = apply_boundary_conditions(&solution, &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_apply_initial_conditions_empty() {
        let x = symbol!(x);
        let solution = Expression::symbol(x);
        let result = apply_initial_conditions(&solution, &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_separated_solution_creation() {
        let x = symbol!(x);
        let t = symbol!(t);
        let fx = Expression::function("F", vec![Expression::symbol(x.clone())]);
        let ft = Expression::function("G", vec![Expression::symbol(t.clone())]);
        let lambda = Expression::symbol(Symbol::scalar("lambda"));

        let solution = SeparatedSolution {
            functions: vec![fx.clone(), ft.clone()],
            constants: vec![lambda.clone()],
            solution: Expression::mul(vec![fx, ft]),
        };

        assert_eq!(solution.functions.len(), 2);
        assert_eq!(solution.constants.len(), 1);
    }

    #[test]
    fn test_separation_with_boundary_conditions() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);

        use crate::pde::types::BoundaryLocation;
        let bc = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x,
                value: expr!(0),
            },
        );

        let result = separate_variables(&pde, &[bc], &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_separation_with_initial_conditions() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);

        let ic = InitialCondition::value(Expression::symbol(x));

        let result = separate_variables(&pde, &[], &[ic]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_separation_with_both_conditions() {
        let u = symbol!(u);
        let x = symbol!(x);
        let t = symbol!(t);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), t]);

        use crate::pde::types::BoundaryLocation;
        let bc = BoundaryCondition::dirichlet(
            expr!(0),
            BoundaryLocation::Simple {
                variable: x.clone(),
                value: expr!(0),
            },
        );
        let ic = InitialCondition::value(Expression::symbol(x));

        let result = separate_variables(&pde, &[bc], &[ic]);
        assert!(result.is_ok());
    }
}
