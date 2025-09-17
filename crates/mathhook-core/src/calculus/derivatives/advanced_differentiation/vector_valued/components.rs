//! Component-wise differentiation for vector-valued functions
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;
/// Component-wise vector differentiation operations
pub struct VectorComponents;
impl VectorComponents {
    /// Compute first derivative of vector components
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::calculus::derivatives::advanced_differentiation::vector_valued::components::VectorComponents;
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let t = symbol!(t);
    /// let components = vec![
    ///     Expression::function("cos", vec![Expression::symbol(t.clone())]),
    ///     Expression::function("sin", vec![Expression::symbol(t.clone())]),
    ///     Expression::symbol(t.clone())
    /// ];
    /// let velocity = VectorComponents::derivative(&components, t);
    /// ```
    pub fn derivative(components: &[Expression], parameter: &Symbol) -> Vec<Expression> {
        let mut derivatives = Vec::with_capacity(components.len());
        for component in components {
            derivatives.push(component.derivative(parameter.clone()).simplify());
        }
        derivatives
    }
    /// Compute second derivative of vector components
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::calculus::derivatives::advanced_differentiation::vector_valued::components::VectorComponents;
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let t = symbol!(t);
    /// let components = vec![
    ///     Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
    ///     Expression::symbol(t.clone()),
    ///     Expression::integer(1)
    /// ];
    /// let acceleration = VectorComponents::second_derivative(&components, t);
    /// ```
    pub fn second_derivative(components: &[Expression], parameter: Symbol) -> Vec<Expression> {
        let first = Self::derivative(components, &parameter);
        Self::derivative(&first, &parameter)
    }
    /// Compute nth derivative of vector components
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::calculus::derivatives::advanced_differentiation::vector_valued::components::VectorComponents;
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let t = symbol!(t);
    /// let components = vec![
    ///     Expression::pow(Expression::symbol(t.clone()), Expression::integer(4)),
    ///     Expression::pow(Expression::symbol(t.clone()), Expression::integer(3))
    /// ];
    /// let third_derivative = VectorComponents::nth_derivative(&components, t, 3);
    /// ```
    pub fn nth_derivative(
        components: &[Expression],
        parameter: Symbol,
        order: u32,
    ) -> Vec<Expression> {
        if order == 0 {
            return components.to_vec();
        }
        if order == 1 {
            return Self::derivative(components, &parameter);
        }
        let mut current = components.to_vec();
        for _ in 0..order {
            current = Self::derivative(&current, &parameter);
        }
        current
    }
    /// Compute magnitude of vector
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::calculus::derivatives::advanced_differentiation::vector_valued::components::VectorComponents;
    /// use mathhook_core::Expression;
    ///
    /// let components = vec![
    ///     Expression::integer(3),
    ///     Expression::integer(4),
    ///     Expression::integer(0)
    /// ];
    /// let magnitude = VectorComponents::magnitude(&components);
    /// ```
    pub fn magnitude(components: &[Expression]) -> Expression {
        let sum_of_squares: Vec<Expression> = components
            .iter()
            .map(|component| Expression::pow(component.clone(), Expression::integer(2)))
            .collect();
        Expression::function("sqrt", vec![Expression::add(sum_of_squares).simplify()])
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};
    #[test]
    fn test_linear_vector_derivative() {
        let t = symbol!(t);
        let components = vec![
            Expression::symbol(t.clone()),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(t.clone())]),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(t.clone())]),
        ];
        let velocity = VectorComponents::derivative(&components, &t);
        assert_eq!(velocity.len(), 3);
        assert_eq!(velocity[0].simplify(), Expression::integer(1));
        assert_eq!(velocity[1].simplify(), Expression::integer(2));
        assert_eq!(velocity[2].simplify(), Expression::integer(3));
    }
    #[test]
    fn test_polynomial_vector_derivative() {
        let t = symbol!(t);
        let components = vec![
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(3)),
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(4)),
        ];
        let velocity = VectorComponents::derivative(&components, &t);
        assert_eq!(velocity.len(), 3);
        assert!(!velocity[0].is_zero());
        assert!(!velocity[1].is_zero());
        assert!(!velocity[2].is_zero());
    }
    #[test]
    fn test_trigonometric_vector_derivative() {
        let t = symbol!(t);
        let components = vec![
            Expression::function("cos", vec![Expression::symbol(t.clone())]),
            Expression::function("sin", vec![Expression::symbol(t.clone())]),
            Expression::symbol(t.clone()),
        ];
        let velocity = VectorComponents::derivative(&components, &t);
        assert_eq!(velocity.len(), 3);
        assert!(!velocity[0].is_zero());
        assert!(!velocity[1].is_zero());
        assert_eq!(velocity[2].simplify(), Expression::integer(1));
    }
    #[test]
    fn test_exponential_vector_derivative() {
        let t = symbol!(t);
        let components = vec![
            Expression::function("exp", vec![Expression::symbol(t.clone())]),
            Expression::function("ln", vec![Expression::symbol(t.clone())]),
        ];
        let velocity = VectorComponents::derivative(&components, &t);
        assert_eq!(velocity.len(), 2);
        assert!(!velocity[0].is_zero());
        assert!(!velocity[1].is_zero());
    }
    #[test]
    fn test_second_derivative() {
        let t = symbol!(t);
        let components = vec![
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(3)),
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
            Expression::symbol(t.clone()),
        ];
        let acceleration = VectorComponents::second_derivative(&components, t.clone());
        assert_eq!(acceleration.len(), 3);
        assert!(!acceleration[0].is_zero());
        assert_eq!(acceleration[2].simplify(), expr!(0));
    }
    #[test]
    fn test_nth_derivative() {
        let t = symbol!(t);
        let components = vec![expr!(t ^ 4), expr!(t ^ 3)];
        let third = VectorComponents::nth_derivative(&components, t.clone(), 3);
        let fourth = VectorComponents::nth_derivative(&components, t.clone(), 4);
        assert_eq!(third.len(), 2);
        assert!(!third[0].is_zero());
        assert!(!third[1].is_zero());
        assert_eq!(fourth.len(), 2);
        assert!(!fourth[0].is_zero());
        assert_eq!(fourth[1].simplify(), Expression::integer(0));
    }
    #[test]
    fn test_zero_order_derivative() {
        let t = symbol!(t);
        let components = vec![expr!(t), expr!(5)];
        let zero_order = VectorComponents::nth_derivative(&components, t.clone(), 0);
        assert_eq!(zero_order.len(), 2);
        assert_eq!(zero_order[0], Expression::symbol(t));
        assert_eq!(zero_order[1], Expression::integer(5));
    }
    #[test]
    fn test_vector_magnitude() {
        let components = vec![expr!(3), expr!(4), expr!(0)];
        let magnitude = VectorComponents::magnitude(&components);
        assert!(!magnitude.is_zero());
    }
    #[test]
    fn test_constant_vector() {
        let t = symbol!(t);
        let components = vec![expr!(1), expr!(2), expr!(3)];
        let velocity = VectorComponents::derivative(&components, &t);
        assert_eq!(velocity.len(), 3);
        for component in velocity {
            assert_eq!(component.simplify(), Expression::integer(0));
        }
    }
    #[test]
    fn test_mixed_functions_vector() {
        let t = symbol!(t);
        let components = vec![
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
            Expression::function("sin", vec![Expression::symbol(t.clone())]),
            Expression::function("exp", vec![Expression::symbol(t.clone())]),
        ];
        let velocity = VectorComponents::derivative(&components, &t);
        assert_eq!(velocity.len(), 3);
        for component in velocity {
            assert!(!component.is_zero());
        }
    }
    #[test]
    fn test_single_component_vector() {
        let t = symbol!(t);
        let components = vec![Expression::pow(
            Expression::symbol(t.clone()),
            Expression::integer(3),
        )];
        let velocity = VectorComponents::derivative(&components, &t);
        assert_eq!(velocity.len(), 1);
        assert!(!velocity[0].is_zero());
    }
    #[test]
    fn test_empty_vector() {
        let t = symbol!(t);
        let components: Vec<Expression> = vec![];
        let velocity = VectorComponents::derivative(&components, &t);
        assert_eq!(velocity.len(), 0);
    }
}
