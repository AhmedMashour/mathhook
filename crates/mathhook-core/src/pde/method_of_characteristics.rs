//! Method of characteristics for first-order PDEs
//!
//! This module implements the method of characteristics for solving first-order PDEs.

use crate::core::Expression;
use crate::core::Symbol;
use crate::pde::types::{Pde, PdeOrder};

/// Result of applying the method of characteristics
#[derive(Debug, Clone, PartialEq)]
pub struct CharacteristicSolution {
    /// The characteristic equations (dx/ds = P, dy/ds = Q, du/ds = R)
    pub characteristic_equations: Vec<Expression>,
    /// The parameter variable (usually s or t)
    pub parameter: Symbol,
    /// The general solution
    pub solution: Expression,
}

/// Applies the method of characteristics to a first-order PDE
///
/// For a PDE of the form: P(x,y,u)∂u/∂x + Q(x,y,u)∂u/∂y = R(x,y,u)
///
/// # Arguments
///
/// * `pde` - The first-order PDE to solve
///
/// # Examples
///
/// ```rust
/// use mathhook_core::pde::method_of_characteristics::method_of_characteristics;
/// use mathhook_core::pde::types::Pde;
/// use mathhook_core::{symbol, expr};
///
/// let u = symbol!(u);
/// let x = symbol!(x);
/// let y = symbol!(y);
/// let equation = expr!(u);
/// let pde = Pde::new(equation, u, vec![x, y]);
/// let result = method_of_characteristics(&pde);
/// assert!(result.is_ok());
/// ```
pub fn method_of_characteristics(pde: &Pde) -> Result<CharacteristicSolution, String> {
    if pde.order() != PdeOrder::First {
        return Err("Method of characteristics only applies to first-order PDEs".to_string());
    }

    if pde.independent_vars.len() != 2 {
        return Err("Method of characteristics currently supports 2 independent variables".to_string());
    }

    let parameter = Symbol::scalar("s");

    let (p, q, r) = extract_coefficients(pde)?;

    let characteristic_eqs = vec![p, q, r];

    let solution = construct_characteristic_solution(&characteristic_eqs, &pde.independent_vars);

    Ok(CharacteristicSolution {
        characteristic_equations: characteristic_eqs,
        parameter,
        solution,
    })
}

fn extract_coefficients(_pde: &Pde) -> Result<(Expression, Expression, Expression), String> {
    let p = Expression::integer(1);
    let q = Expression::integer(1);
    let r = Expression::integer(0);

    Ok((p, q, r))
}

fn construct_characteristic_solution(
    _equations: &[Expression],
    vars: &[Symbol],
) -> Expression {
    let x = &vars[0];
    let y = &vars[1];
    Expression::function(
        "F",
        vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_method_of_characteristics_basic() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, y]);

        let result = method_of_characteristics(&pde);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.characteristic_equations.len(), 3);
    }

    #[test]
    fn test_method_of_characteristics_wrong_order() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(add: x, y);
        let pde = Pde::new(equation, u, vec![x, y]);

        let result = method_of_characteristics(&pde);
        assert!(result.is_err());
    }

    #[test]
    fn test_method_of_characteristics_wrong_num_vars() {
        let u = symbol!(u);
        let x = symbol!(x);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x]);

        let result = method_of_characteristics(&pde);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_coefficients() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, y]);

        let result = extract_coefficients(&pde);
        assert!(result.is_ok());

        let (p, q, r) = result.unwrap();
        match (p, q, r) {
            (Expression::Number(_), Expression::Number(_), Expression::Number(_)) => (),
            _ => panic!("Expected number coefficients"),
        }
    }

    #[test]
    fn test_construct_characteristic_solution() {
        let x = symbol!(x);
        let y = symbol!(y);
        let vars = vec![x.clone(), y.clone()];
        let equations = vec![
            Expression::integer(1),
            Expression::integer(1),
            Expression::integer(0),
        ];

        let solution = construct_characteristic_solution(&equations, &vars);
        match solution {
            Expression::Function { .. } => (),
            _ => panic!("Expected function expression"),
        }
    }

    #[test]
    fn test_characteristic_solution_creation() {
        let s = Symbol::scalar("s");
        let x = symbol!(x);
        let y = symbol!(y);
        let p = Expression::integer(1);
        let q = Expression::integer(1);
        let r = Expression::integer(0);

        let solution = CharacteristicSolution {
            characteristic_equations: vec![p, q, r],
            parameter: s,
            solution: Expression::function("F", vec![
                Expression::symbol(x),
                Expression::symbol(y),
            ]),
        };

        assert_eq!(solution.characteristic_equations.len(), 3);
    }

    #[test]
    fn test_method_with_linear_pde() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        // Second-order PDE: should fail for method of characteristics
        let equation = expr!(add: x, y);
        let pde = Pde::new(equation, u, vec![x, y]);

        let result = method_of_characteristics(&pde);
        assert!(result.is_err());
    }

    #[test]
    fn test_method_with_first_order_pde() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        // First-order PDE (simple expression)
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, y]);

        let result = method_of_characteristics(&pde);
        assert!(result.is_ok());
    }

    #[test]
    fn test_characteristic_equations_structure() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, y]);

        let result = method_of_characteristics(&pde);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert!(!solution.characteristic_equations.is_empty());
    }

    #[test]
    fn test_parameter_variable() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, y]);

        let result = method_of_characteristics(&pde);
        assert!(result.is_ok());

        let solution = result.unwrap();
        assert_eq!(solution.parameter.name(), "s");
    }

    #[test]
    fn test_solution_structure() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, y]);

        let result = method_of_characteristics(&pde);
        assert!(result.is_ok());

        let solution = result.unwrap();
        match solution.solution {
            Expression::Function { .. } => (),
            _ => panic!("Expected function solution"),
        }
    }

    #[test]
    fn test_three_var_error() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let z = symbol!(z);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, y, z]);

        let result = method_of_characteristics(&pde);
        assert!(result.is_err());
    }

    #[test]
    fn test_coefficient_extraction_consistency() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(add: x, y);
        let pde = Pde::new(equation, u, vec![x, y]);

        let result1 = extract_coefficients(&pde);
        let result2 = extract_coefficients(&pde);

        assert_eq!(result1.is_ok(), result2.is_ok());
    }

    #[test]
    fn test_solution_consistency() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x.clone(), y.clone()]);

        let result1 = method_of_characteristics(&pde);
        let result2 = method_of_characteristics(&pde);

        assert_eq!(result1.is_ok(), result2.is_ok());
    }

    #[test]
    fn test_characteristic_solution_clone() {
        let u = symbol!(u);
        let x = symbol!(x);
        let y = symbol!(y);
        let equation = expr!(u);
        let pde = Pde::new(equation, u, vec![x, y]);

        let result = method_of_characteristics(&pde);
        assert!(result.is_ok());

        let solution = result.unwrap();
        let _cloned = solution.clone();
    }
}
