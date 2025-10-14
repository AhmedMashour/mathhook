//! Geometric operations for vector-valued functions

use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;

use super::components::VectorComponents;

/// Geometric properties of vector-valued functions
pub struct VectorGeometry;

impl VectorGeometry {
    /// Compute unit tangent vector T = r'(t) / |r'(t)|
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::derivatives::advanced_differentiation::vector_valued::geometry::VectorGeometry;
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let t = symbol!(t);
    /// let components = vec![
    ///     Expression::function("cos", vec![Expression::symbol(t.clone())]),
    ///     Expression::function("sin", vec![Expression::symbol(t.clone())]),
    /// ];
    /// let unit_tangent = VectorGeometry::unit_tangent(&components, t);
    /// ```
    pub fn unit_tangent(components: &[Expression], parameter: Symbol) -> Vec<Expression> {
        let velocity = VectorComponents::derivative(components, parameter);
        let speed = VectorComponents::magnitude(&velocity);

        velocity
            .into_iter()
            .map(|component| scalar_divide(component, &speed))
            .collect()
    }

    /// Compute curvature κ = |r' × r''| / |r'|³
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::derivatives::advanced_differentiation::vector_valued::geometry::VectorGeometry;
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let t = symbol!(t);
    /// let components = vec![
    ///     Expression::symbol(t.clone()),
    ///     Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
    /// ];
    /// let curvature = VectorGeometry::curvature(&components, t);
    /// ```
    pub fn curvature(components: &[Expression], parameter: Symbol) -> Expression {
        let first = VectorComponents::derivative(components, parameter.clone());
        let second = VectorComponents::derivative(&first, parameter);

        if components.len() == 2 {
            let cross_magnitude = Expression::function(
                "abs",
                vec![Expression::add(vec![
                    Expression::mul(vec![first[0].clone(), second[1].clone()]),
                    Expression::mul(vec![
                        Expression::integer(-1),
                        first[1].clone(),
                        second[0].clone(),
                    ]),
                ])
                .simplify()],
            );

            let speed_cubed =
                Expression::pow(VectorComponents::magnitude(&first), Expression::integer(3));

            create_division(&cross_magnitude, &speed_cubed)
        } else if components.len() == 3 {
            let cross_product = cross_product_3d(&first, &second);
            let cross_magnitude = VectorComponents::magnitude(&cross_product);
            let speed_cubed =
                Expression::pow(VectorComponents::magnitude(&first), Expression::integer(3));

            create_division(&cross_magnitude, &speed_cubed)
        } else {
            Expression::integer(0)
        }
    }

    /// Compute torsion τ for 3D curves
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::calculus::derivatives::advanced_differentiation::vector_valued::geometry::VectorGeometry;
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let t = symbol!(t);
    /// let components = vec![
    ///     Expression::function("cos", vec![Expression::symbol(t.clone())]),
    ///     Expression::function("sin", vec![Expression::symbol(t.clone())]),
    ///     Expression::symbol(t.clone())
    /// ];
    /// let torsion = VectorGeometry::torsion(&components, t);
    /// ```
    pub fn torsion(components: &[Expression], parameter: Symbol) -> Expression {
        if components.len() != 3 {
            return Expression::integer(0);
        }

        let first = VectorComponents::derivative(components, parameter.clone());
        let second = VectorComponents::derivative(&first, parameter.clone());
        let third = VectorComponents::derivative(&second, parameter);

        let cross_first_second = cross_product_3d(&first, &second);
        let numerator = dot_product(&cross_first_second, &third);

        let cross_magnitude_squared = Expression::pow(
            VectorComponents::magnitude(&cross_first_second),
            Expression::integer(2),
        );

        create_division(&numerator, &cross_magnitude_squared)
    }
}

fn scalar_divide(numerator: Expression, denominator: &Expression) -> Expression {
    Expression::mul(vec![
        numerator,
        Expression::pow(denominator.clone(), Expression::integer(-1)),
    ])
}

fn create_division(numerator: &Expression, denominator: &Expression) -> Expression {
    Expression::mul(vec![
        numerator.clone(),
        Expression::pow(denominator.clone(), Expression::integer(-1)),
    ])
}

fn cross_product_3d(v1: &[Expression], v2: &[Expression]) -> Vec<Expression> {
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

fn dot_product(v1: &[Expression], v2: &[Expression]) -> Expression {
    let min_len = v1.len().min(v2.len());
    let mut terms = Vec::with_capacity(min_len);

    for i in 0..min_len {
        terms.push(Expression::mul(vec![v1[i].clone(), v2[i].clone()]));
    }

    Expression::add(terms).simplify()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_unit_tangent_vector() {
        let t = symbol!(t);

        let components = vec![
            Expression::function("cos", vec![Expression::symbol(t.clone())]),
            Expression::function("sin", vec![Expression::symbol(t.clone())]),
        ];

        let unit_tangent = VectorGeometry::unit_tangent(&components, t.clone());

        assert_eq!(unit_tangent.len(), 2);
        assert!(!unit_tangent[0].is_zero());
        assert!(!unit_tangent[1].is_zero());
    }

    #[test]
    fn test_curvature_2d() {
        let t = symbol!(t);

        let components = vec![
            Expression::symbol(t.clone()),
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
        ];

        let curvature = VectorGeometry::curvature(&components, t.clone());
        assert!(!curvature.is_zero());
    }

    #[test]
    fn test_curvature_3d() {
        let t = symbol!(t);

        let components = vec![
            Expression::function("cos", vec![Expression::symbol(t.clone())]),
            Expression::function("sin", vec![Expression::symbol(t.clone())]),
            Expression::symbol(t.clone()),
        ];

        let curvature = VectorGeometry::curvature(&components, t.clone());
        assert!(!curvature.is_zero());
    }

    #[test]
    fn test_torsion_3d() {
        let t = symbol!(t);

        let components = vec![
            Expression::function("cos", vec![Expression::symbol(t.clone())]),
            Expression::function("sin", vec![Expression::symbol(t.clone())]),
            Expression::symbol(t.clone()),
        ];

        let torsion = VectorGeometry::torsion(&components, t.clone());
        assert!(!torsion.is_zero());
    }

    #[test]
    fn test_torsion_2d_returns_zero() {
        let t = symbol!(t);

        let components = vec![
            Expression::symbol(t.clone()),
            Expression::pow(Expression::symbol(t.clone()), Expression::integer(2)),
        ];

        let torsion = VectorGeometry::torsion(&components, t.clone());
        assert_eq!(torsion.simplify(), Expression::integer(0));
    }

    #[test]
    fn test_helix_curve() {
        let t = symbol!(t);

        let components = vec![
            Expression::function("cos", vec![Expression::symbol(t.clone())]),
            Expression::function("sin", vec![Expression::symbol(t.clone())]),
            Expression::symbol(t.clone()),
        ];

        let velocity = VectorComponents::derivative(&components, t.clone());
        let acceleration = VectorComponents::second_derivative(&components, t.clone());

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
        let t = symbol!(t);

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

        let velocity = VectorComponents::derivative(&components, t.clone());
        let magnitude = VectorComponents::magnitude(&velocity);

        assert_eq!(velocity.len(), 3);
        assert!(!magnitude.is_zero());
    }
}
