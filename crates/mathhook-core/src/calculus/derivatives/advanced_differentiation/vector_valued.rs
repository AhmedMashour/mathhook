//! Vector-valued function differentiation
//!
//! Handles differentiation of vector-valued functions r(t) = [x(t), y(t), z(t)]

use crate::algebra::simplify::Simplify;
use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};

/// Vector-valued function differentiation
pub struct VectorValuedDifferentiation;

impl VectorValuedDifferentiation {
    /// Compute derivative
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let t = Symbol::new("t");
    /// let components = vec![
    ///     Expression::function("cos", vec![Expression::symbol(t.clone())]),
    ///     Expression::function("sin", vec![Expression::symbol(t.clone())]),
    ///     Expression::symbol(t.clone())
    /// ];
    /// let velocity = VectorValuedDifferentiation::derivative(&components, t);
    /// ```
    pub fn derivative(components: &[Expression], parameter: Symbol) -> Vec<Expression> {
        let mut derivatives = Vec::with_capacity(components.len());

        for component in components {
            derivatives.push(component.derivative(parameter.clone()).simplify());
        }

        derivatives
    }

    /// Compute second derivative
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let t = Symbol::new("t");
    /// let components = vec![
    ///     Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
    ///     Expression::symbol(t.clone()),
    ///     Expression::integer(1)
    /// ];
    /// let acceleration = VectorValuedDifferentiation::second_derivative(&components, t);
    /// ```
    pub fn second_derivative(components: &[Expression], parameter: Symbol) -> Vec<Expression> {
        let cache = Self::compute_derivative_cache(components, parameter.clone());
        Self::compute_full_derivative_cache(&cache.first_derivatives, parameter).second_derivatives
    }

    /// Compute nth derivative
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let t = Symbol::new("t");
    /// let components = vec![
    ///     Expression::pow(Expression::symbol(t.clone()), Expression::integer(4)),
    ///     Expression::pow(Expression::symbol(t.clone()), Expression::integer(3))
    /// ];
    /// let third_derivative = VectorValuedDifferentiation::nth_derivative(&components, t, 3);
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
            return Self::derivative(components, parameter);
        }

        let mut current = components.to_vec();
        for _ in 0..order {
            current = Self::derivative(&current, parameter.clone());
        }

        current
    }

    /// Compute magnitude of vector
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let components = vec![
    ///     Expression::integer(3),
    ///     Expression::integer(4),
    ///     Expression::integer(0)
    /// ];
    /// let magnitude = VectorValuedDifferentiation::magnitude(&components);
    /// ```
    pub fn magnitude(components: &[Expression]) -> Expression {
        let sum_of_squares: Vec<Expression> = components
            .iter()
            .map(|component| Expression::pow(component.clone(), Expression::integer(2)))
            .collect();

        Expression::function("sqrt", vec![Expression::add(sum_of_squares).simplify()])
    }

    /// Compute unit tangent vector T = r'(t) / |r'(t)|
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let t = Symbol::new("t");
    /// let components = vec![
    ///     Expression::function("cos", vec![Expression::symbol(t.clone())]),
    ///     Expression::function("sin", vec![Expression::symbol(t.clone())]),
    /// ];
    /// let unit_tangent = VectorValuedDifferentiation::unit_tangent(&components, t);
    /// ```
    pub fn unit_tangent(components: &[Expression], parameter: Symbol) -> Vec<Expression> {
        let velocity = Self::derivative(components, parameter);
        let speed = Self::magnitude(&velocity);

        velocity
            .into_iter()
            .map(|component| Self::scalar_divide_optimized(component, &speed))
            .collect()
    }

    /// Compute curvature κ = |r' × r''| / |r'|³
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let t = Symbol::new("t");
    /// let components = vec![
    ///     Expression::symbol(t.clone()),
    ///     Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
    /// ];
    /// let curvature = VectorValuedDifferentiation::curvature(&components, t);
    /// ```
    pub fn curvature(components: &[Expression], parameter: Symbol) -> Expression {
        let cache = Self::compute_full_derivative_cache(components, parameter);

        if components.len() == 2 {
            let cross_magnitude = Expression::function(
                "abs",
                vec![Expression::add(vec![
                    Expression::mul(vec![
                        cache.first_derivatives[0].clone(),
                        cache.second_derivatives[1].clone(),
                    ]),
                    Expression::mul(vec![
                        Expression::integer(-1),
                        cache.first_derivatives[1].clone(),
                        cache.second_derivatives[0].clone(),
                    ]),
                ])
                .simplify()],
            );

            let speed_cubed = Expression::pow(
                Self::magnitude(&cache.first_derivatives),
                Expression::integer(3),
            );

            Self::create_division_inline(&cross_magnitude, &speed_cubed)
        } else if components.len() == 3 {
            let cross_product = Self::cross_product_3d_optimized(
                &cache.first_derivatives,
                &cache.second_derivatives,
            );
            let cross_magnitude = Self::magnitude(&cross_product);
            let speed_cubed = Expression::pow(
                Self::magnitude(&cache.first_derivatives),
                Expression::integer(3),
            );

            Self::create_division_inline(&cross_magnitude, &speed_cubed)
        } else {
            Expression::integer(0)
        }
    }

    /// Compute torsion τ for 3D curves
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, Symbol};
    ///
    /// let t = Symbol::new("t");
    /// let components = vec![
    ///     Expression::function("cos", vec![Expression::symbol(t.clone())]),
    ///     Expression::function("sin", vec![Expression::symbol(t.clone())]),
    ///     Expression::symbol(t.clone())
    /// ];
    /// let torsion = VectorValuedDifferentiation::torsion(&components, t);
    /// ```
    pub fn torsion(components: &[Expression], parameter: Symbol) -> Expression {
        if components.len() != 3 {
            return Expression::integer(0);
        }

        let first = Self::derivative(components, parameter.clone());
        let second = Self::derivative(&first, parameter.clone());
        let third = Self::derivative(&second, parameter);

        let cross_first_second = Self::cross_product_3d_optimized(&first, &second);
        let numerator = Self::dot_product_inline(&cross_first_second, &third);

        let cross_magnitude_squared =
            Expression::pow(Self::magnitude(&cross_first_second), Expression::integer(2));

        Self::create_division_inline(&numerator, &cross_magnitude_squared)
    }

    /// Compute derivative cache
    fn compute_derivative_cache(components: &[Expression], parameter: Symbol) -> DerivativeCache {
        DerivativeCache {
            first_derivatives: Self::derivative(components, parameter),
        }
    }

    /// Compute full derivative cache
    fn compute_full_derivative_cache(
        components: &[Expression],
        parameter: Symbol,
    ) -> FullDerivativeCache {
        let first = Self::derivative(components, parameter.clone());
        let second = Self::derivative(&first, parameter);

        FullDerivativeCache {
            first_derivatives: first,
            second_derivatives: second,
        }
    }

    /// Create division expression
    fn create_division_inline(numerator: &Expression, denominator: &Expression) -> Expression {
        Expression::mul(vec![
            numerator.clone(),
            Expression::pow(denominator.clone(), Expression::integer(-1)),
        ])
    }

    /// Scalar division
    fn scalar_divide_optimized(numerator: Expression, denominator: &Expression) -> Expression {
        Expression::mul(vec![
            numerator,
            Expression::pow(denominator.clone(), Expression::integer(-1)),
        ])
    }

    /// Cross product magnitude for 2D
    fn cross_product_magnitude_optimized(v1: &[Expression], v2: &[Expression]) -> Expression {
        if v1.len() >= 2 && v2.len() >= 2 {
            Expression::add(vec![
                Expression::mul(vec![v1[0].clone(), v2[1].clone()]),
                Expression::mul(vec![Expression::integer(-1), v1[1].clone(), v2[0].clone()]),
            ])
            .simplify()
        } else {
            Expression::integer(0)
        }
    }

    /// 3D cross product
    fn cross_product_3d_optimized(v1: &[Expression], v2: &[Expression]) -> Vec<Expression> {
        if v1.len() >= 3 && v2.len() >= 3 {
            vec![
                Expression::add(vec![
                    Expression::mul(vec![v1[1].clone(), v2[2].clone()]),
                    Expression::mul(vec![Expression::integer(-1), v1[2].clone(), v2[1].clone()]),
                ])
                .simplify(),
                Expression::add(vec![
                    Expression::mul(vec![v1[2].clone(), v2[0].clone()]),
                    Expression::mul(vec![Expression::integer(-1), v1[0].clone(), v2[2].clone()]),
                ])
                .simplify(),
                Expression::add(vec![
                    Expression::mul(vec![v1[0].clone(), v2[1].clone()]),
                    Expression::mul(vec![Expression::integer(-1), v1[1].clone(), v2[0].clone()]),
                ])
                .simplify(),
            ]
        } else {
            vec![Expression::integer(0); 3]
        }
    }

    /// Dot product
    fn dot_product_inline(v1: &[Expression], v2: &[Expression]) -> Expression {
        let min_len = v1.len().min(v2.len());
        let mut terms = Vec::with_capacity(min_len);

        for i in 0..min_len {
            terms.push(Expression::mul(vec![v1[i].clone(), v2[i].clone()]));
        }

        Expression::add(terms).simplify()
    }
}

/// Cached first derivatives
struct DerivativeCache {
    first_derivatives: Vec<Expression>,
}

/// Cached full derivatives
struct FullDerivativeCache {
    first_derivatives: Vec<Expression>,
    second_derivatives: Vec<Expression>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_vector_derivative() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::symbol(t.clone()),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(t.clone())]),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(t.clone())]),
        ];

        let velocity = VectorValuedDifferentiation::derivative(&components, t.clone());

        assert_eq!(velocity.len(), 3);
        assert_eq!(velocity[0].simplify(), Expression::integer(1));
        assert_eq!(velocity[1].simplify(), Expression::integer(2));
        assert_eq!(velocity[2].simplify(), Expression::integer(3));
    }

    #[test]
    fn test_polynomial_vector_derivative() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(3)),
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(4)),
        ];

        let velocity = VectorValuedDifferentiation::derivative(&components, t.clone());

        assert_eq!(velocity.len(), 3);
        assert!(!velocity[0].is_zero());
        assert!(!velocity[1].is_zero());
        assert!(!velocity[2].is_zero());
    }

    #[test]
    fn test_trigonometric_vector_derivative() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::function("cos", vec![Expression::symbol(t.clone())]),
            Expression::function("sin", vec![Expression::symbol(t.clone())]),
            Expression::symbol(t.clone()),
        ];

        let velocity = VectorValuedDifferentiation::derivative(&components, t.clone());

        assert_eq!(velocity.len(), 3);
        assert!(!velocity[0].is_zero());
        assert!(!velocity[1].is_zero());
        assert_eq!(velocity[2].simplify(), Expression::integer(1));
    }

    #[test]
    fn test_exponential_vector_derivative() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::function("exp", vec![Expression::symbol(t.clone())]),
            Expression::function("ln", vec![Expression::symbol(t.clone())]),
        ];

        let velocity = VectorValuedDifferentiation::derivative(&components, t.clone());

        assert_eq!(velocity.len(), 2);
        assert!(!velocity[0].is_zero());
        assert!(!velocity[1].is_zero());
    }

    #[test]
    fn test_second_derivative() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(3)),
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
            Expression::symbol(t.clone()),
        ];

        let acceleration = VectorValuedDifferentiation::second_derivative(&components, t.clone());

        assert_eq!(acceleration.len(), 3);
        assert!(!acceleration[0].is_zero());
        assert!(!acceleration[1].is_zero());
        assert_eq!(acceleration[2].simplify(), Expression::integer(0));
    }

    #[test]
    fn test_nth_derivative() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(4)),
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(3)),
        ];

        let third = VectorValuedDifferentiation::nth_derivative(&components, t.clone(), 3);
        let fourth = VectorValuedDifferentiation::nth_derivative(&components, t.clone(), 4);

        assert_eq!(third.len(), 2);
        assert!(!third[0].is_zero());
        assert!(!third[1].is_zero());

        assert_eq!(fourth.len(), 2);
        assert!(!fourth[0].is_zero());
        assert_eq!(fourth[1].simplify(), Expression::integer(0));
    }

    #[test]
    fn test_zero_order_derivative() {
        let t = Symbol::new("t");

        let components = vec![Expression::symbol(t.clone()), Expression::integer(5)];

        let zero_order = VectorValuedDifferentiation::nth_derivative(&components, t.clone(), 0);

        assert_eq!(zero_order.len(), 2);
        assert_eq!(zero_order[0], Expression::symbol(t));
        assert_eq!(zero_order[1], Expression::integer(5));
    }

    #[test]
    fn test_vector_magnitude() {
        let components = vec![
            Expression::integer(3),
            Expression::integer(4),
            Expression::integer(0),
        ];

        let magnitude = VectorValuedDifferentiation::magnitude(&components);
        assert!(!magnitude.is_zero());
    }

    #[test]
    fn test_unit_tangent_vector() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::function("cos", vec![Expression::symbol(t.clone())]),
            Expression::function("sin", vec![Expression::symbol(t.clone())]),
        ];

        let unit_tangent = VectorValuedDifferentiation::unit_tangent(&components, t.clone());

        assert_eq!(unit_tangent.len(), 2);
        assert!(!unit_tangent[0].is_zero());
        assert!(!unit_tangent[1].is_zero());
    }

    #[test]
    fn test_curvature_2d() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::symbol(t.clone()),
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
        ];

        let curvature = VectorValuedDifferentiation::curvature(&components, t.clone());
        assert!(!curvature.is_zero());
    }

    #[test]
    fn test_curvature_3d() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::function("cos", vec![Expression::symbol(t.clone())]),
            Expression::function("sin", vec![Expression::symbol(t.clone())]),
            Expression::symbol(t.clone()),
        ];

        let curvature = VectorValuedDifferentiation::curvature(&components, t.clone());
        assert!(!curvature.is_zero());
    }

    #[test]
    fn test_torsion_3d() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::function("cos", vec![Expression::symbol(t.clone())]),
            Expression::function("sin", vec![Expression::symbol(t.clone())]),
            Expression::symbol(t.clone()),
        ];

        let torsion = VectorValuedDifferentiation::torsion(&components, t.clone());
        assert!(!torsion.is_zero());
    }

    #[test]
    fn test_torsion_2d_returns_zero() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::symbol(t.clone()),
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
        ];

        let torsion = VectorValuedDifferentiation::torsion(&components, t.clone());
        assert_eq!(torsion.simplify(), Expression::integer(0));
    }

    #[test]
    fn test_constant_vector() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::integer(1),
            Expression::integer(2),
            Expression::integer(3),
        ];

        let velocity = VectorValuedDifferentiation::derivative(&components, t.clone());

        assert_eq!(velocity.len(), 3);
        for component in velocity {
            assert_eq!(component.simplify(), Expression::integer(0));
        }
    }

    #[test]
    fn test_mixed_functions_vector() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
            Expression::function("sin", vec![Expression::symbol(t.clone())]),
            Expression::function("exp", vec![Expression::symbol(t.clone())]),
        ];

        let velocity = VectorValuedDifferentiation::derivative(&components, t.clone());

        assert_eq!(velocity.len(), 3);
        for component in velocity {
            assert!(!component.is_zero());
        }
    }

    #[test]
    fn test_helix_curve() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::function("cos", vec![Expression::symbol(t.clone())]),
            Expression::function("sin", vec![Expression::symbol(t.clone())]),
            Expression::symbol(t.clone()),
        ];

        let velocity = VectorValuedDifferentiation::derivative(&components, t.clone());
        let acceleration = VectorValuedDifferentiation::second_derivative(&components, t.clone());

        assert_eq!(velocity.len(), 3);
        assert_eq!(acceleration.len(), 3);

        for component in &velocity {
            assert!(!component.is_zero());
        }

        assert!(!acceleration[0].is_zero());
        assert!(!acceleration[1].is_zero());
        assert_eq!(acceleration[2].simplify(), Expression::integer(0));
    }

    #[test]
    fn test_parametric_surface_normal() {
        let t = Symbol::new("t");

        let components = vec![
            Expression::mul(vec![
                Expression::symbol(t.clone()),
                Expression::function("cos", vec![Expression::symbol(t.clone())]),
            ]),
            Expression::mul(vec![
                Expression::symbol(t.clone()),
                Expression::function("sin", vec![Expression::symbol(t.clone())]),
            ]),
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
        ];

        let velocity = VectorValuedDifferentiation::derivative(&components, t.clone());
        let magnitude = VectorValuedDifferentiation::magnitude(&velocity);

        assert_eq!(velocity.len(), 3);
        assert!(!magnitude.is_zero());
    }

    #[test]
    fn test_single_component_vector() {
        let t = Symbol::new("t");

        let components = vec![Expression::pow(
            Expression::symbol(t.clone()),
            Expression::integer(3),
        )];

        let velocity = VectorValuedDifferentiation::derivative(&components, t.clone());

        assert_eq!(velocity.len(), 1);
        assert!(!velocity[0].is_zero());
    }

    #[test]
    fn test_empty_vector() {
        let t = Symbol::new("t");

        let components: Vec<Expression> = vec![];

        let velocity = VectorValuedDifferentiation::derivative(&components, t.clone());

        assert_eq!(velocity.len(), 0);
    }
}
