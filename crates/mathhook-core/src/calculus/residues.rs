//! Residue calculus and complex analysis
//!
//! Implements residue computation, contour integration,
//! and complex analysis operations for symbolic computation.
//!
//! This module is organized into focused sub-modules:
//! - Core residue computation and integration (in mod.rs)
//! - Singularity classification (singularities.rs)
//! - Pole finding for various function types (pole_finding.rs)
//! - Helper utilities (helpers.rs)

mod helpers;
mod pole_finding;
mod singularities;

use crate::calculus::derivatives::Derivative;
use crate::core::{Expression, Symbol};
use crate::simplify::Simplify;

use pole_finding::{find_rational_poles, find_transcendental_poles};

// Re-export public API
pub use singularities::{classify_singularity, ComplexAnalysis, SingularityType};

/// Trait for residue calculus operations
pub trait ResidueCalculus {
    /// Compute residue at a pole
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::ResidueCalculus;
    ///
    /// let z = symbol!(z);
    /// let expr = Expression::pow(
    ///     Expression::add(vec![
    ///         Expression::symbol(z.clone()),
    ///         Expression::integer(-1)
    ///     ]),
    ///     Expression::integer(-1)
    /// );
    /// let pole = Expression::integer(1);
    /// let result = expr.residue(&z, &pole);
    /// ```
    fn residue(&self, variable: &Symbol, pole: &Expression) -> Expression;

    /// Find all poles of the function
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::ResidueCalculus;
    ///
    /// let z = symbol!(z);
    /// let expr = Expression::pow(
    ///     Expression::mul(vec![
    ///         Expression::add(vec![Expression::symbol(z.clone()), Expression::integer(-1)]),
    ///         Expression::add(vec![Expression::symbol(z.clone()), Expression::integer(-2)])
    ///     ]),
    ///     Expression::integer(-1)
    /// );
    /// let poles = expr.find_poles(&z);
    /// ```
    fn find_poles(&self, variable: &Symbol) -> Vec<Expression>;

    /// Compute contour integral using residue theorem
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    /// use mathhook_core::calculus::ResidueCalculus;
    ///
    /// let z = symbol!(z);
    /// let expr = Expression::pow(
    ///     Expression::add(vec![Expression::symbol(z.clone()), Expression::integer(-1)]),
    ///     Expression::integer(-1)
    /// );
    /// let result = expr.contour_integral(&z);
    /// ```
    fn contour_integral(&self, variable: &Symbol) -> Expression;
}

/// Residue computation methods
pub struct ResidueMethods;

impl ResidueMethods {
    /// Compute residue for simple pole using limit formula
    ///
    /// # Arguments
    ///
    /// * `numerator` - Numerator of the rational function
    /// * `denominator` - Denominator of the rational function
    /// * `variable` - The variable symbol
    /// * `pole` - Location of the pole
    ///
    /// # Returns
    ///
    /// Expression representing the residue computation
    pub fn simple_pole_residue(
        numerator: &Expression,
        denominator: &Expression,
        variable: &Symbol,
        pole: &Expression,
    ) -> Expression {
        // Res(f, a) = lim_{z→a} (z-a)f(z) for simple pole
        let z_minus_a = Expression::add(vec![
            Expression::symbol(variable.clone()),
            Expression::mul(vec![Expression::integer(-1), pole.clone()]),
        ]);

        let limit_expr = Expression::mul(vec![
            z_minus_a,
            Expression::mul(vec![
                numerator.clone(),
                Expression::pow(denominator.clone(), Expression::integer(-1)),
            ]),
        ]);

        Expression::function(
            "limit",
            vec![
                limit_expr,
                Expression::symbol(variable.clone()),
                pole.clone(),
            ],
        )
    }

    /// Compute residue for pole of order m using derivative formula
    ///
    /// # Arguments
    ///
    /// * `numerator` - Numerator of the rational function
    /// * `denominator` - Denominator of the rational function
    /// * `variable` - The variable symbol
    /// * `pole` - Location of the pole
    /// * `order` - Order of the pole
    ///
    /// # Returns
    ///
    /// Expression representing the residue
    pub fn higher_order_pole_residue(
        numerator: &Expression,
        denominator: &Expression,
        variable: &Symbol,
        pole: &Expression,
        order: u32,
    ) -> Expression {
        if order == 1 {
            return Self::simple_pole_residue(numerator, denominator, variable, pole);
        }

        // Res(f, a) = (1/(m-1)!) * lim_{z→a} d^(m-1)/dz^(m-1) [(z-a)^m * f(z)]
        let z_minus_a = Expression::add(vec![
            Expression::symbol(variable.clone()),
            Expression::mul(vec![Expression::integer(-1), pole.clone()]),
        ]);

        let multiplied_expr = Expression::mul(vec![
            Expression::pow(z_minus_a, Expression::integer(order as i64)),
            Expression::mul(vec![
                numerator.clone(),
                Expression::pow(denominator.clone(), Expression::integer(-1)),
            ]),
        ]);

        let derivative_expr = multiplied_expr.nth_derivative(variable.clone(), order - 1);

        let factorial = Self::factorial(order - 1);
        let limit_result = Expression::function(
            "limit",
            vec![
                derivative_expr,
                Expression::symbol(variable.clone()),
                pole.clone(),
            ],
        );

        Expression::mul(vec![
            Expression::pow(factorial, Expression::integer(-1)),
            limit_result,
        ])
        .simplify()
    }

    /// Compute factorial
    ///
    /// # Arguments
    ///
    /// * `n` - The number to compute factorial of
    ///
    /// # Returns
    ///
    /// Expression representing n!
    pub fn factorial(n: u32) -> Expression {
        let result = match n {
            0 | 1 => 1,
            _ => (2..=n as i64).product(),
        };
        Expression::integer(result)
    }
}

impl ResidueCalculus for Expression {
    fn residue(&self, variable: &Symbol, pole: &Expression) -> Expression {
        // Check if this is a rational function
        if let Expression::Mul(factors) = self {
            if factors.len() == 2 {
                if let Expression::Pow(denom, exp) = &factors[1] {
                    if **exp == Expression::integer(-1) {
                        // This is numerator/denominator form
                        let numerator = &factors[0];
                        let denominator = denom;

                        // Determine pole order
                        let order = self.pole_order(variable, pole);
                        return ResidueMethods::higher_order_pole_residue(
                            numerator,
                            denominator,
                            variable,
                            pole,
                            order,
                        );
                    }
                }
            }
        }

        // General case - use Laurent series
        Expression::function(
            "residue",
            vec![
                self.clone(),
                Expression::symbol(variable.clone()),
                pole.clone(),
            ],
        )
    }

    fn find_poles(&self, variable: &Symbol) -> Vec<Expression> {
        // Find poles of the expression
        match self {
            // Rational functions: poles where denominator = 0
            Expression::Mul(factors) if factors.len() == 2 => {
                if let Expression::Pow(denom, exp) = &factors[1] {
                    if **exp == Expression::integer(-1) {
                        let numerator = &factors[0];
                        return find_rational_poles(numerator, denom, variable);
                    }
                }
                vec![]
            }
            // Direct reciprocal: 1/f(x)
            Expression::Pow(base, exp) => {
                if let Expression::Number(crate::core::Number::Integer(n)) = exp.as_ref() {
                    if *n == -1 {
                        return find_rational_poles(&Expression::integer(1), base, variable);
                    }
                }
                vec![]
            }
            // Transcendental functions with known poles
            Expression::Function { name, args } => find_transcendental_poles(name, args, variable),
            _ => vec![],
        }
    }

    fn contour_integral(&self, variable: &Symbol) -> Expression {
        let poles = self.find_poles(variable);
        if poles.is_empty() {
            return Expression::integer(0);
        }

        // Residue theorem: ∮f(z)dz = 2πi * Σ Res(f, poles inside contour)
        let residue_sum = if poles.len() == 1 {
            self.residue(variable, &poles[0])
        } else {
            let residues: Vec<Expression> = poles
                .iter()
                .map(|pole| self.residue(variable, pole))
                .collect();
            Expression::add(residues)
        };

        Expression::mul(vec![
            Expression::integer(2),
            Expression::pi(),
            Expression::i(),
            residue_sum,
        ])
        .simplify()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;
    use crate::symbol;

    #[test]
    fn test_simple_pole_residue() {
        let z = symbol!(z);
        let numerator = Expression::integer(1);
        let denominator =
            Expression::add(vec![Expression::symbol(z.clone()), Expression::integer(-1)]);
        let pole = Expression::integer(1);

        let result = ResidueMethods::simple_pole_residue(&numerator, &denominator, &z, &pole);

        // Residue of 1/(z-1) at z=1 should be 1
        assert!(matches!(result, Expression::Function { name, .. } if name.as_ref() == "limit"));
    }

    #[test]
    fn test_factorial() {
        assert_eq!(ResidueMethods::factorial(0), Expression::integer(1));
        assert_eq!(ResidueMethods::factorial(1), Expression::integer(1));
        assert_eq!(ResidueMethods::factorial(4), Expression::integer(24));
        assert_eq!(ResidueMethods::factorial(5), Expression::integer(120));
    }

    #[test]
    fn test_pole_order() {
        let z = symbol!(z);
        let expr = Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(
                Expression::pow(
                    Expression::add(vec![Expression::symbol(z.clone()), Expression::integer(-1)]),
                    Expression::integer(2),
                ),
                Expression::integer(-1),
            ),
        ]);
        let pole = Expression::integer(1);

        let order = expr.pole_order(&z, &pole);
        assert!(order >= 1, "Pole order should be at least 1, got {}", order);
    }

    #[test]
    fn test_is_analytic() {
        let z = symbol!(z);
        let polynomial = Expression::pow(Expression::symbol(z.clone()), Expression::integer(2));
        let point = Expression::integer(1);

        assert!(polynomial.is_analytic_at(&z, &point));
    }

    #[test]
    fn test_pole_order_simple_pole() {
        let z = symbol!(z);
        // 1/(z-2) has a simple pole (order 1) at z=2
        let expr = Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(
                Expression::add(vec![Expression::symbol(z.clone()), Expression::integer(-2)]),
                Expression::integer(-1),
            ),
        ]);
        let pole = Expression::integer(2);

        let order = expr.pole_order(&z, &pole);
        assert_eq!(order, 1, "1/(z-2) should have pole of order 1 at z=2");
    }

    #[test]
    fn test_pole_order_double_pole() {
        let z = symbol!(z);
        // 1/(z-3)^2 has a double pole (order 2) at z=3
        let expr = Expression::pow(
            Expression::add(vec![Expression::symbol(z.clone()), Expression::integer(-3)]),
            Expression::integer(-2),
        );
        let pole = Expression::integer(3);

        let order = expr.pole_order(&z, &pole);
        assert_eq!(order, 2, "1/(z-3)^2 should have pole of order 2 at z=3");
    }

    #[test]
    fn test_pole_order_triple_pole() {
        let z = symbol!(z);
        // 1/(z-1)^3 has a triple pole (order 3) at z=1
        let expr = Expression::pow(
            Expression::add(vec![Expression::symbol(z.clone()), Expression::integer(-1)]),
            Expression::integer(-3),
        );
        let pole = Expression::integer(1);

        let order = expr.pole_order(&z, &pole);
        assert_eq!(order, 3, "1/(z-1)^3 should have pole of order 3 at z=1");
    }

    #[test]
    fn test_pole_order_no_pole() {
        let z = symbol!(z);
        // 1/(z-2) evaluated at z=5 should have no pole (order 0)
        let expr = Expression::mul(vec![
            Expression::integer(1),
            Expression::pow(
                Expression::add(vec![Expression::symbol(z.clone()), Expression::integer(-2)]),
                Expression::integer(-1),
            ),
        ]);
        let point = Expression::integer(5);

        let order = expr.pole_order(&z, &point);
        assert_eq!(
            order, 0,
            "1/(z-2) should have no pole at z=5 (denominator is non-zero)"
        );
    }

    #[test]
    fn test_pole_order_polynomial_no_pole() {
        let z = symbol!(z);
        // z^2 is a polynomial, no poles anywhere
        let expr = Expression::pow(Expression::symbol(z.clone()), Expression::integer(2));
        let point = Expression::integer(0);

        let order = expr.pole_order(&z, &point);
        assert_eq!(order, 0, "Polynomial z^2 should have no poles");
    }

    #[test]
    fn test_pole_order_at_origin() {
        let z = symbol!(z);
        // 1/z has a simple pole at z=0
        let expr = Expression::pow(Expression::symbol(z.clone()), Expression::integer(-1));
        let pole = Expression::integer(0);

        let order = expr.pole_order(&z, &pole);
        assert_eq!(order, 1, "1/z should have pole of order 1 at z=0");
    }

    #[test]
    fn test_pole_order_at_origin_higher() {
        let z = symbol!(z);
        // 1/z^4 has a pole of order 4 at z=0
        let expr = Expression::pow(Expression::symbol(z.clone()), Expression::integer(-4));
        let pole = Expression::integer(0);

        let order = expr.pole_order(&z, &pole);
        assert_eq!(order, 4, "1/z^4 should have pole of order 4 at z=0");
    }

    #[test]
    fn test_classify_singularity_pole() {
        let z = symbol!(z);
        // 1/(z-1)^2 has a pole of order 2 at z=1
        let expr = expr!((z - 1) ^ (-2));
        let point = expr!(1);

        let classification = classify_singularity(&expr, &z, &point);
        assert_eq!(classification, SingularityType::Pole(2));
    }

    #[test]
    fn test_find_poles_transcendental_tan() {
        let x = symbol!(x);
        let expr = Expression::function("tan", vec![Expression::symbol(x.clone())]);

        let poles = expr.find_poles(&x);
        assert!(!poles.is_empty(), "tan(x) should have poles");
    }

    #[test]
    fn test_find_poles_transcendental_cot() {
        let x = symbol!(x);
        let expr = Expression::function("cot", vec![Expression::symbol(x.clone())]);

        let poles = expr.find_poles(&x);
        assert!(!poles.is_empty(), "cot(x) should have poles");
        assert_eq!(poles[0], expr!(0));
    }
}
