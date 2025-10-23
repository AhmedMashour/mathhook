//! Solves second-order linear ODEs with constant coefficients
//!
//! Solves equations of the form: ay'' + by' + cy = r(x)
//! where a, b, c are constants.
//!
//! Uses the characteristic equation method for homogeneous equations.

use crate::core::{Expression, Number, Symbol};
use crate::ode::first_order::{ODEError, ODEResult};
use crate::simplify::Simplify;
use num_bigint::Sign;

/// Constant coefficient second-order ODE solver
///
/// Solves ODEs of the form: ay'' + by' + cy = r(x)
/// where a, b, c are constant coefficients.
#[derive(Debug, Clone)]
pub struct ConstantCoeffSecondOrderSolver;

/// Root types for the characteristic equation
#[derive(Debug, Clone, PartialEq)]
pub enum RootType {
    /// Two distinct real roots r1, r2
    DistinctReal { r1: Expression, r2: Expression },
    /// One repeated real root r
    RepeatedReal { r: Expression },
    /// Complex conjugate roots: alpha ± beta*i
    ComplexConjugate { alpha: Expression, beta: Expression },
}

impl ConstantCoeffSecondOrderSolver {
    /// Create a new constant coefficient solver
    pub fn new() -> Self {
        Self
    }

    /// Solve ay'' + by' + cy = r(x)
    ///
    /// # Arguments
    ///
    /// * `a`, `b`, `c` - Constant coefficients
    /// * `r` - Right-hand side function r(x) (use 0 for homogeneous)
    /// * `dependent` - The dependent variable (y)
    /// * `independent` - The independent variable (x)
    /// * `initial_conditions` - Optional (y(x0), y'(x0))
    ///
    /// # Returns
    ///
    /// General solution or particular solution if initial conditions provided
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::ode::second_order::ConstantCoeffSecondOrderSolver;
    /// use mathhook_core::{symbol, expr};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// // y'' + 4y = 0 (simple harmonic oscillator)
    /// let solver = ConstantCoeffSecondOrderSolver::new();
    /// let solution = solver.solve(
    ///     &expr!(1), &expr!(0), &expr!(4),
    ///     &expr!(0),  // r(x) = 0 (homogeneous)
    ///     &y, &x, None
    /// ).unwrap();
    /// // Expected: y = C1*cos(2x) + C2*sin(2x)
    /// ```
    pub fn solve(
        &self,
        a: &Expression,
        b: &Expression,
        c: &Expression,
        r: &Expression,
        _dependent: &Symbol,
        independent: &Symbol,
        _initial_conditions: Option<(Expression, Expression)>,
    ) -> ODEResult {
        // Handle degenerate case: a = 0 (actually first-order)
        if a.is_zero() {
            return Err(ODEError::NotLinearForm {
                reason: "Coefficient 'a' is zero - this is a first-order ODE, not second-order".to_string(),
            });
        }

        // Solve characteristic equation: ar² + br + c = 0
        let roots = self.solve_characteristic(a, b, c)?;

        // Form homogeneous solution based on root type
        let y_h = self.homogeneous_solution(&roots, independent)?;

        // Handle non-homogeneous case
        let _solution = if r.is_zero() {
            // Homogeneous: y = y_h
            y_h
        } else {
            // Non-homogeneous: y = y_h + y_p
            // For Wave 1C MVP, only handle homogeneous case
            return Err(ODEError::NotImplemented {
                feature: "non-homogeneous constant coefficient ODEs".to_string(),
            });
        };

        Ok(_solution)
    }

    /// Solve characteristic equation ar² + br + c = 0
    ///
    /// Returns the type of roots (distinct real, repeated real, or complex conjugate)
    fn solve_characteristic(
        &self,
        a: &Expression,
        b: &Expression,
        c: &Expression,
    ) -> Result<RootType, ODEError> {
        // Compute discriminant: Δ = b² - 4ac
        let discriminant = Expression::add(vec![
            Expression::pow(b.clone(), Expression::integer(2)),
            Expression::mul(vec![
                Expression::integer(-4),
                a.clone(),
                c.clone(),
            ]),
        ]).simplify();

        self.classify_by_discriminant(a, b, &discriminant)
    }

    /// Classify roots by discriminant value
    fn classify_by_discriminant(
        &self,
        a: &Expression,
        b: &Expression,
        discriminant: &Expression,
    ) -> Result<RootType, ODEError> {
        // For MVP, handle numeric discriminant values
        if let Expression::Number(num) = discriminant {
            let two_a = Expression::mul(vec![Expression::integer(2), a.clone()]);

            // Check if discriminant is negative
            let is_negative = match num {
                Number::Integer(i) => *i < 0,
                Number::Float(f) => *f < 0.0,
                Number::BigInteger(bi) => bi.sign() == Sign::Minus,
                Number::Rational(r) => r.numer().sign() == Sign::Minus,
            };

            if is_negative {
                // Complex conjugate roots: α ± βi
                // α = -b/(2a), β = √(-Δ)/(2a)
                let alpha = Expression::mul(vec![
                    Expression::integer(-1),
                    b.clone(),
                    Expression::pow(two_a.clone(), Expression::integer(-1)),
                ]).simplify();

                let beta = Expression::mul(vec![
                    Expression::function("sqrt", vec![Expression::mul(vec![
                        Expression::integer(-1),
                        discriminant.clone(),
                    ])]),
                    Expression::pow(two_a, Expression::integer(-1)),
                ]).simplify();

                Ok(RootType::ComplexConjugate { alpha, beta })
            } else if num.is_zero() {
                // Repeated real root: r = -b/(2a)
                let r = Expression::mul(vec![
                    Expression::integer(-1),
                    b.clone(),
                    Expression::pow(two_a, Expression::integer(-1)),
                ]).simplify();

                Ok(RootType::RepeatedReal { r })
            } else {
                // Distinct real roots: r = (-b ± √Δ)/(2a)
                let sqrt_disc = Expression::function("sqrt", vec![discriminant.clone()]);

                let r1 = Expression::mul(vec![
                    Expression::add(vec![
                        Expression::mul(vec![Expression::integer(-1), b.clone()]),
                        sqrt_disc.clone(),
                    ]),
                    Expression::pow(two_a.clone(), Expression::integer(-1)),
                ]).simplify();

                let r2 = Expression::mul(vec![
                    Expression::add(vec![
                        Expression::mul(vec![Expression::integer(-1), b.clone()]),
                        Expression::mul(vec![Expression::integer(-1), sqrt_disc]),
                    ]),
                    Expression::pow(two_a, Expression::integer(-1)),
                ]).simplify();

                Ok(RootType::DistinctReal { r1, r2 })
            }
        } else {
            // Symbolic discriminant - cannot determine root type
            Err(ODEError::NotImplemented {
                feature: "symbolic discriminant classification".to_string(),
            })
        }
    }

    /// Form homogeneous solution based on root type
    fn homogeneous_solution(
        &self,
        roots: &RootType,
        independent: &Symbol,
    ) -> ODEResult {
        let x = Expression::symbol(independent.clone());
        let c1 = Expression::symbol(Symbol::scalar("C1"));
        let c2 = Expression::symbol(Symbol::scalar("C2"));

        match roots {
            RootType::DistinctReal { r1, r2 } => {
                // y = C1*e^(r1*x) + C2*e^(r2*x)
                let term1 = Expression::mul(vec![
                    c1,
                    Expression::function("exp", vec![Expression::mul(vec![r1.clone(), x.clone()])]),
                ]);
                let term2 = Expression::mul(vec![
                    c2,
                    Expression::function("exp", vec![Expression::mul(vec![r2.clone(), x])]),
                ]);
                Ok(Expression::add(vec![term1, term2]))
            }
            RootType::RepeatedReal { r } => {
                // y = (C1 + C2*x)*e^(r*x)
                let factor = Expression::add(vec![c1, Expression::mul(vec![c2, x.clone()])]);
                Ok(Expression::mul(vec![
                    factor,
                    Expression::function("exp", vec![Expression::mul(vec![r.clone(), x])]),
                ]))
            }
            RootType::ComplexConjugate { alpha, beta } => {
                // y = e^(α*x) * [C1*cos(β*x) + C2*sin(β*x)]
                let exp_term = Expression::function("exp", vec![Expression::mul(vec![alpha.clone(), x.clone()])]);
                let cos_term = Expression::mul(vec![c1, Expression::function("cos", vec![Expression::mul(vec![beta.clone(), x.clone()])])]);
                let sin_term = Expression::mul(vec![c2, Expression::function("sin", vec![Expression::mul(vec![beta.clone(), x])])]);

                Ok(Expression::mul(vec![exp_term, Expression::add(vec![cos_term, sin_term])]))
            }
        }
    }
}

impl Default for ConstantCoeffSecondOrderSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_simple_harmonic_oscillator() {
        let x = symbol!(x);
        let y = symbol!(y);

        // y'' + 4y = 0 → r² + 4 = 0 → r = ±2i
        // Solution: y = C1*cos(2x) + C2*sin(2x)
        let solver = ConstantCoeffSecondOrderSolver::new();
        let solution = solver.solve(
            &expr!(1),
            &expr!(0),
            &expr!(4),
            &expr!(0),
            &y,
            &x,
            None,
        );

        assert!(solution.is_ok(), "Simple harmonic oscillator should solve");
        let sol = solution.unwrap();
        let sol_str = sol.to_string();
        assert!(sol_str.contains("cos") || sol_str.contains("sin"),
                "Solution should contain trigonometric functions, got: {}", sol_str);
    }

    #[test]
    fn test_distinct_real_roots() {
        let x = symbol!(x);
        let y = symbol!(y);

        // y'' - 3y' + 2y = 0 → r² - 3r + 2 = 0 → r = 1, 2
        // Solution: y = C1*e^x + C2*e^(2x)
        let solver = ConstantCoeffSecondOrderSolver::new();
        let solution = solver.solve(
            &expr!(1),
            &expr!(-3),
            &expr!(2),
            &expr!(0),
            &y,
            &x,
            None,
        );

        assert!(solution.is_ok(), "Distinct real roots case should solve");
        let sol = solution.unwrap();
        let sol_str = sol.to_string();
        assert!(sol_str.contains("exp"), "Solution should contain exp, got: {}", sol_str);
    }

    #[test]
    fn test_repeated_real_root() {
        let x = symbol!(x);
        let y = symbol!(y);

        // y'' - 2y' + y = 0 → r² - 2r + 1 = 0 → r = 1 (repeated)
        // Solution: y = (C1 + C2*x)*e^x
        let solver = ConstantCoeffSecondOrderSolver::new();
        let solution = solver.solve(
            &expr!(1),
            &expr!(-2),
            &expr!(1),
            &expr!(0),
            &y,
            &x,
            None,
        );

        assert!(solution.is_ok(), "Repeated root case should solve");
        let sol = solution.unwrap();
        let sol_str = sol.to_string();
        assert!(sol_str.contains("exp"), "Solution should contain exp, got: {}", sol_str);
    }

    #[test]
    fn test_complex_with_damping() {
        let x = symbol!(x);
        let y = symbol!(y);

        // y'' + 2y' + 5y = 0 → r² + 2r + 5 = 0 → r = -1 ± 2i
        // Solution: y = e^(-x) * [C1*cos(2x) + C2*sin(2x)]
        let solver = ConstantCoeffSecondOrderSolver::new();
        let solution = solver.solve(
            &expr!(1),
            &expr!(2),
            &expr!(5),
            &expr!(0),
            &y,
            &x,
            None,
        );

        assert!(solution.is_ok(), "Complex conjugate roots should solve");
        let sol = solution.unwrap();
        let sol_str = sol.to_string();
        assert!(sol_str.contains("exp"), "Solution should contain exp");
        assert!(sol_str.contains("cos") || sol_str.contains("sin"),
                "Solution should contain trig functions");
    }

    #[test]
    fn test_zero_coefficient_a() {
        let x = symbol!(x);
        let y = symbol!(y);

        // 0*y'' + y' + y = 0 (actually first-order)
        let solver = ConstantCoeffSecondOrderSolver::new();
        let solution = solver.solve(
            &expr!(0),
            &expr!(1),
            &expr!(1),
            &expr!(0),
            &y,
            &x,
            None,
        );

        assert!(solution.is_err(), "Should reject zero leading coefficient");
    }
}
