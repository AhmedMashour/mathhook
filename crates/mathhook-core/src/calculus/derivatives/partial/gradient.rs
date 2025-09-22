//! Gradient and directional derivative operations

use crate::algebra::simplify::Simplify;
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};

/// Gradient vector operations
pub struct GradientOperations;

impl GradientOperations {
    /// Compute gradient vector
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let expr = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
    /// ]);
    /// let gradient = GradientOperations::compute(&expr, vec![x, y]);
    /// ```
    pub fn compute(expr: &Expression, variables: Vec<Symbol>) -> Vec<Expression> {
        let n = variables.len();
        let mut gradient = Vec::with_capacity(n);

        for var in variables {
            let partial = expr.derivative(var).simplify();
            gradient.push(partial);
        }

        gradient
    }

    /// Compute gradient with caching for repeated computations
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    /// use std::collections::HashMap;
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let expr = Expression::mul(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::symbol(y.clone())
    /// ]);
    /// let mut cache = HashMap::new();
    /// let gradient = GradientOperations::compute_cached(&expr, &[x, y], &mut cache);
    /// ```
    pub fn compute_cached(
        expr: &Expression,
        variables: &[Symbol],
        cache: &mut std::collections::HashMap<Symbol, Expression>,
    ) -> Vec<Expression> {
        let mut gradient = Vec::with_capacity(variables.len());

        for var in variables {
            let partial = cache
                .entry(var.clone())
                .or_insert_with(|| expr.derivative(var.clone()).simplify())
                .clone();
            gradient.push(partial);
        }

        gradient
    }
}

/// Directional derivative operations
pub struct DirectionalDerivatives;

impl DirectionalDerivatives {
    /// Compute directional derivative
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let expr = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
    /// ]);
    /// let direction = vec![Expression::integer(1), Expression::integer(1)];
    /// let dir_deriv = DirectionalDerivatives::compute(&expr, vec![x, y], direction);
    /// ```
    pub fn compute(
        expr: &Expression,
        variables: Vec<Symbol>,
        direction: Vec<Expression>,
    ) -> Expression {
        if variables.len() != direction.len() {
            panic!(
                "Dimension mismatch: {} variables vs {} direction components",
                variables.len(),
                direction.len()
            );
        }

        let gradient = GradientOperations::compute(expr, variables);
        Self::dot_product(gradient, direction)
    }

    /// Compute dot product of gradient and direction
    fn dot_product(gradient: Vec<Expression>, direction: Vec<Expression>) -> Expression {
        let n = gradient.len();
        let mut dot_terms = Vec::with_capacity(n);

        for (grad_component, dir_component) in gradient.into_iter().zip(direction) {
            dot_terms.push(Expression::mul(vec![grad_component, dir_component]));
        }

        Expression::add(dot_terms).simplify()
    }

    /// Compute unit directional derivative (normalizes direction vector)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let x = Symbol::new("x");
    /// let y = Symbol::new("y");
    /// let expr = Expression::add(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::pow(Expression::symbol(y.clone()), Expression::integer(2))
    /// ]);
    /// let direction = vec![Expression::integer(3), Expression::integer(4)];
    /// let unit_dir_deriv = DirectionalDerivatives::unit_directional(&expr, vec![x, y], direction);
    /// ```
    pub fn unit_directional(
        expr: &Expression,
        variables: Vec<Symbol>,
        direction: Vec<Expression>,
    ) -> Expression {
        let magnitude_squared: Vec<Expression> = direction
            .iter()
            .map(|component| Expression::pow(component.clone(), Expression::integer(2)))
            .collect();

        let magnitude =
            Expression::function("sqrt", vec![Expression::add(magnitude_squared).simplify()]);

        let unit_direction: Vec<Expression> = direction
            .into_iter()
            .map(|component| {
                Expression::mul(vec![
                    component,
                    Expression::pow(magnitude.clone(), Expression::integer(-1)),
                ])
            })
            .collect();

        Self::compute(expr, variables, unit_direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_basic_gradient_computation() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        let quadratic = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]);

        let gradient = GradientOperations::compute(&quadratic, vec![x.clone(), y.clone()]);
        assert_eq!(gradient.len(), 2);
        assert!(!gradient[0].is_zero());
        assert!(!gradient[1].is_zero());
    }

    #[test]
    fn test_linear_function_gradient() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        let linear = Expression::add(vec![
            Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
            Expression::mul(vec![Expression::integer(4), Expression::symbol(y.clone())]),
            Expression::integer(5),
        ]);

        let gradient = GradientOperations::compute(&linear, vec![x.clone(), y.clone()]);
        assert_eq!(gradient.len(), 2);
        assert_eq!(gradient[0].simplify(), Expression::integer(3));
        assert_eq!(gradient[1].simplify(), Expression::integer(4));
    }

    #[test]
    fn test_multivariate_polynomial_gradient() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        let poly = Expression::add(vec![
            Expression::mul(vec![
                Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
                Expression::symbol(y.clone()),
            ]),
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
            ]),
        ]);

        let gradient = GradientOperations::compute(&poly, vec![x.clone(), y.clone()]);
        assert_eq!(gradient.len(), 2);
        assert!(!gradient[0].is_zero());
        assert!(!gradient[1].is_zero());
    }

    #[test]
    fn test_gradient_caching() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        let expr = Expression::function(
            "sin",
            vec![Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ])],
        );

        let mut cache = HashMap::new();
        let gradient1 =
            GradientOperations::compute_cached(&expr, &[x.clone(), y.clone()], &mut cache);
        let gradient2 =
            GradientOperations::compute_cached(&expr, &[x.clone(), y.clone()], &mut cache);

        assert_eq!(gradient1.len(), 2);
        assert_eq!(gradient2.len(), 2);
        assert_eq!(gradient1[0], gradient2[0]);
        assert_eq!(gradient1[1], gradient2[1]);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_three_variable_gradient() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");
        let z = Symbol::new("z");

        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
            Expression::symbol(z.clone()),
        ]);

        let gradient = GradientOperations::compute(&expr, vec![x.clone(), y.clone(), z.clone()]);
        assert_eq!(gradient.len(), 3);
        assert!(!gradient[0].is_zero());
        assert!(!gradient[1].is_zero());
        assert_eq!(gradient[2].simplify(), Expression::integer(1));
    }

    #[test]
    fn test_directional_derivative_basic() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]);

        let direction = vec![Expression::integer(1), Expression::integer(0)];
        let dir_deriv =
            DirectionalDerivatives::compute(&expr, vec![x.clone(), y.clone()], direction);
        assert!(!dir_deriv.is_zero());
    }

    #[test]
    fn test_directional_derivative_diagonal() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);

        let direction = vec![Expression::integer(1), Expression::integer(1)];
        let dir_deriv =
            DirectionalDerivatives::compute(&expr, vec![x.clone(), y.clone()], direction);
        assert!(!dir_deriv.is_zero());
    }

    #[test]
    fn test_unit_directional_derivative() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]);

        let direction = vec![Expression::integer(3), Expression::integer(4)];
        let unit_dir_deriv =
            DirectionalDerivatives::unit_directional(&expr, vec![x.clone(), y.clone()], direction);
        assert!(!unit_dir_deriv.is_zero());
    }

    #[test]
    fn test_constant_function_gradient() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        let constant = Expression::integer(42);
        let gradient = GradientOperations::compute(&constant, vec![x.clone(), y.clone()]);

        assert_eq!(gradient.len(), 2);
        assert_eq!(gradient[0].simplify(), Expression::integer(0));
        assert_eq!(gradient[1].simplify(), Expression::integer(0));
    }

    #[test]
    fn test_single_variable_gradient() {
        let x = Symbol::new("x");

        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(3));
        let gradient = GradientOperations::compute(&expr, vec![x.clone()]);

        assert_eq!(gradient.len(), 1);
        assert!(!gradient[0].is_zero());
    }

    #[test]
    #[should_panic(expected = "Dimension mismatch")]
    fn test_directional_derivative_dimension_mismatch() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        let expr = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);

        let wrong_direction = vec![Expression::integer(1)];
        DirectionalDerivatives::compute(&expr, vec![x, y], wrong_direction);
    }

    #[test]
    fn test_trigonometric_function_gradient() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        let trig_expr = Expression::add(vec![
            Expression::function("sin", vec![Expression::symbol(x.clone())]),
            Expression::function("cos", vec![Expression::symbol(y.clone())]),
        ]);

        let gradient = GradientOperations::compute(&trig_expr, vec![x.clone(), y.clone()]);
        assert_eq!(gradient.len(), 2);
        assert!(!gradient[0].is_zero());
        assert!(!gradient[1].is_zero());
    }

    #[test]
    fn test_zero_direction_vector() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        let expr = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);

        let zero_direction = vec![Expression::integer(0), Expression::integer(0)];
        let dir_deriv =
            DirectionalDerivatives::compute(&expr, vec![x.clone(), y.clone()], zero_direction);
        assert_eq!(dir_deriv.simplify(), Expression::integer(0));
    }
}
