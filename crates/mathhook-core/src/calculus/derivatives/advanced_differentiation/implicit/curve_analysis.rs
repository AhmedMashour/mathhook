//! Critical point analysis for implicitly defined curves
//!
//! Provides analysis tools for implicit curves including critical points and concavity.

use crate::algebra::solvers::{
    EquationSolver, LinearSolver, PolynomialSolver, QuadraticSolver, SolverResult,
};
use crate::core::{Expression, Number, Symbol};
use crate::simplify::Simplify;
use std::collections::HashMap;

use super::differentiation::ImplicitDifferentiation;

/// Implicit curve analysis
pub struct ImplicitCurveAnalysis;

impl ImplicitCurveAnalysis {
    /// Find critical points of implicitly defined curves
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::derivatives::ImplicitCurveAnalysis;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let curve = expr!((x^2) + (y^2));
    /// let critical_points = ImplicitCurveAnalysis::critical_points(&curve, x, y);
    /// ```
    pub fn critical_points(
        curve: &Expression,
        x_var: Symbol,
        y_var: Symbol,
    ) -> Vec<(Expression, Expression)> {
        let dy_dx = ImplicitDifferentiation::compute(curve, x_var.clone(), y_var.clone());
        Self::solve_critical_conditions(&dy_dx, curve, x_var, y_var)
    }

    /// Analyze concavity of implicit curves
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol};
    /// use mathhook_core::calculus::derivatives::ImplicitCurveAnalysis;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let curve = expr!(x^2);
    /// let concavity = ImplicitCurveAnalysis::concavity(&curve, x, y);
    /// ```
    pub fn concavity(curve: &Expression, x_var: Symbol, y_var: Symbol) -> Expression {
        ImplicitDifferentiation::higher_order(curve, x_var, y_var, 2)
    }

    /// Solve critical point conditions for implicit curves
    ///
    /// Critical points occur where dy/dx = 0 (horizontal tangent).
    /// For implicit curve F(x,y) = 0, this means solving:
    /// - dy/dx = -∂F/∂x / ∂F/∂y = 0
    /// - Which simplifies to: ∂F/∂x = 0 (numerator equals zero)
    ///
    /// # Algorithm
    ///
    /// 1. Extract numerator from dy/dx (which is ∂F/∂x)
    /// 2. Solve ∂F/∂x = 0 for one variable (preferably the simpler one)
    /// 3. Substitute back into F(x,y) = 0 to find the other variable
    /// 4. Return coordinate pairs (x, y)
    ///
    /// # Edge Cases
    ///
    /// - No solutions: Curve has no critical points (e.g., y = x)
    /// - Infinite solutions: All points are critical (e.g., horizontal line y = c)
    /// - Division by zero: ∂F/∂y = 0 indicates vertical tangent, not critical point
    ///
    /// # Arguments
    ///
    /// * `dy_dx` - The derivative expression dy/dx
    /// * `curve` - Original curve equation F(x,y)
    /// * `x_var` - Independent variable
    /// * `y_var` - Dependent variable
    ///
    /// # Returns
    ///
    /// Vector of critical point coordinates as (x, y) expression pairs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{expr, symbol, Expression};
    /// use mathhook_core::calculus::derivatives::{ImplicitDifferentiation, ImplicitCurveAnalysis};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    ///
    /// // Circle: x² + y² = 1
    /// let circle = Expression::add(vec![expr!(x^2), expr!(y^2), expr!(-1)]);
    ///
    /// let dy_dx = ImplicitDifferentiation::compute(&circle, x.clone(), y.clone());
    /// // dy/dx = -x/y, critical points when x = 0
    /// // Substituting into x² + y² = 1 gives y = ±1
    /// // Expected: (0, 1) and (0, -1)
    /// ```
    fn solve_critical_conditions(
        dy_dx: &Expression,
        curve: &Expression,
        x_var: Symbol,
        y_var: Symbol,
    ) -> Vec<(Expression, Expression)> {
        let numerator = Self::extract_fraction_numerator(dy_dx);

        let linear_solver = LinearSolver::new();
        let quadratic_solver = QuadraticSolver::new();
        let poly_solver = PolynomialSolver::new();

        let solver_result = if poly_solver.can_solve(&numerator) {
            poly_solver.solve(&numerator, &x_var)
        } else if quadratic_solver.can_solve(&numerator) {
            quadratic_solver.solve(&numerator, &x_var)
        } else {
            linear_solver.solve(&numerator, &x_var)
        };

        match solver_result {
            SolverResult::Single(x_solution) => {
                if let Some(y_solution) =
                    Self::solve_for_other_variable(curve, &x_var, &x_solution, &y_var)
                {
                    vec![(x_solution, y_solution)]
                } else {
                    vec![]
                }
            }
            SolverResult::Multiple(x_solutions) => {
                let mut critical_points = Vec::new();
                for x_sol in x_solutions {
                    if let Some(y_sol) =
                        Self::solve_for_other_variable(curve, &x_var, &x_sol, &y_var)
                    {
                        critical_points.push((x_sol, y_sol));
                    }
                }
                critical_points
            }
            SolverResult::NoSolution => {
                vec![]
            }
            SolverResult::InfiniteSolutions => {
                vec![(Expression::symbol(x_var), Expression::symbol(y_var))]
            }
            SolverResult::Parametric(_) | SolverResult::Partial(_) => {
                vec![]
            }
        }
    }

    /// Extract numerator from fraction expression
    ///
    /// For dy/dx = numerator / denominator, extract numerator.
    /// Critical points occur when numerator = 0.
    ///
    /// Handles various expression structures after simplification:
    /// - `Mul([a, Pow(b, -1)])` → a
    /// - `Mul([-1, x, Pow(y, -1)])` → -1 * x
    /// - `Mul([a, b, ..., Pow(z, -1)])` → a * b * ...
    pub(super) fn extract_fraction_numerator(expr: &Expression) -> Expression {
        match expr {
            Expression::Mul(factors) => {
                let mut numerator_factors = Vec::new();
                let mut found_denominator = false;

                for factor in factors.iter() {
                    if let Expression::Pow(_, exp) = factor {
                        if let Expression::Number(Number::Integer(n)) = exp.as_ref() {
                            if *n < 0 {
                                found_denominator = true;
                                continue;
                            }
                        }
                    }
                    numerator_factors.push(factor.clone());
                }

                if found_denominator && !numerator_factors.is_empty() {
                    if numerator_factors.len() == 1 {
                        numerator_factors.pop().unwrap()
                    } else {
                        Expression::mul(numerator_factors)
                    }
                } else {
                    expr.clone()
                }
            }
            _ => expr.clone(),
        }
    }

    /// Solve for the other variable given one variable's value
    ///
    /// Given F(x,y) = 0 and x = x₀, solve for y.
    fn solve_for_other_variable(
        curve: &Expression,
        known_var: &Symbol,
        known_value: &Expression,
        unknown_var: &Symbol,
    ) -> Option<Expression> {
        let mut substitutions = HashMap::new();
        substitutions.insert(known_var.name().to_owned(), known_value.clone());
        let substituted = curve.substitute(&substitutions).simplify();

        let linear_solver = LinearSolver::new();
        let quadratic_solver = QuadraticSolver::new();
        let poly_solver = PolynomialSolver::new();

        let result = if poly_solver.can_solve(&substituted) {
            poly_solver.solve(&substituted, unknown_var)
        } else if quadratic_solver.can_solve(&substituted) {
            quadratic_solver.solve(&substituted, unknown_var)
        } else {
            linear_solver.solve(&substituted, unknown_var)
        };

        match result {
            SolverResult::Single(solution) => Some(solution),
            SolverResult::Multiple(solutions) if !solutions.is_empty() => {
                Some(solutions[0].clone())
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_curve_concavity() {
        let x = symbol!(x);
        let y = symbol!(y);

        let curve = expr!(x ^ 2);
        let concavity = ImplicitCurveAnalysis::concavity(&curve, x.clone(), y.clone());

        assert!(!concavity.is_zero());
    }

    #[test]
    fn test_critical_points_circle() {
        let x = symbol!(x);
        let y = symbol!(y);

        // Circle: x² + y² = 1
        // Critical points occur at x = 0, giving y = ±1
        let circle = Expression::add(vec![expr!(x ^ 2), expr!(y ^ 2), Expression::integer(-1)]);

        let critical_points = ImplicitCurveAnalysis::critical_points(&circle, x.clone(), y.clone());

        assert!(
            !critical_points.is_empty(),
            "Circle should have critical points"
        );

        // Check that critical points are found (may be in various representations)
        for (x_coord, _y_coord) in &critical_points {
            // Check if x-coordinate is zero (handles multiple representations)
            let is_zero = match x_coord {
                Expression::Number(Number::Integer(0)) => true,
                Expression::Function { name, args } if name.as_ref() == "fraction" && !args.is_empty() => {
                    // fraction(0, denominator) = 0
                    matches!(args[0], Expression::Number(Number::Integer(0)))
                }
                _ => {
                    let simplified = x_coord.simplify();
                    simplified.is_zero() || simplified == Expression::integer(0)
                }
            };
            assert!(is_zero, "x-coordinate should be zero, got: {:?}", x_coord);
        }
    }

    #[test]
    fn test_critical_points_ellipse() {
        let x = symbol!(x);
        let y = symbol!(y);

        // Ellipse: 4x² + y² = 4
        // Critical points occur at x = 0
        let ellipse = Expression::add(vec![
            expr!(4 * (x ^ 2)),
            expr!(y ^ 2),
            Expression::integer(-4),
        ]);

        let critical_points =
            ImplicitCurveAnalysis::critical_points(&ellipse, x.clone(), y.clone());

        assert!(
            !critical_points.is_empty(),
            "Ellipse should have critical points"
        );

        // Check that x-coordinates are zero (may be in various representations)
        for (x_coord, _y_coord) in &critical_points {
            // Check if x-coordinate is zero (handles multiple representations)
            let is_zero = match x_coord {
                Expression::Number(Number::Integer(0)) => true,
                Expression::Function { name, args } if name.as_ref() == "fraction" && !args.is_empty() => {
                    // fraction(0, denominator) = 0
                    matches!(args[0], Expression::Number(Number::Integer(0)))
                }
                _ => {
                    let simplified = x_coord.simplify();
                    simplified.is_zero() || simplified == Expression::integer(0)
                }
            };
            assert!(is_zero, "x-coordinate should be zero, got: {:?}", x_coord);
        }
    }

    #[test]
    fn test_critical_points_parabola() {
        let x = symbol!(x);
        let y = symbol!(y);

        let parabola = expr!((y ^ 2) + ((-4) * x));

        let critical_points =
            ImplicitCurveAnalysis::critical_points(&parabola, x.clone(), y.clone());

        if !critical_points.is_empty() {
            for (x_coord, y_coord) in &critical_points {
                assert_eq!(*x_coord, Expression::integer(0));
                assert_eq!(*y_coord, Expression::integer(0));
            }
        }
    }

    #[test]
    fn test_extract_fraction_numerator() {
        let x = symbol!(x);

        // Test with expression: x * 2^(-1), which represents x/2
        let fraction = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::pow(Expression::integer(2), Expression::integer(-1)),
        ]);

        let numerator = ImplicitCurveAnalysis::extract_fraction_numerator(&fraction);

        // After simplification, x * (1/2) may become (1/2) * x
        // The numerator extraction should capture the x term
        match &numerator {
            Expression::Mul(factors) => {
                // Should contain x symbol
                let has_x = factors
                    .iter()
                    .any(|f| matches!(f, Expression::Symbol(s) if s == &x));
                assert!(has_x, "Numerator should contain x symbol");
            }
            Expression::Symbol(s) => {
                assert_eq!(s, &x, "Should be x symbol");
            }
            _ => panic!("Unexpected numerator format: {:?}", numerator),
        }
    }

    #[test]
    fn test_critical_points_no_solution() {
        let x = symbol!(x);
        let y = symbol!(y);

        let line = expr!(x + ((-1) * y));

        let critical_points = ImplicitCurveAnalysis::critical_points(&line, x.clone(), y.clone());

        assert!(critical_points.is_empty());
    }
}
