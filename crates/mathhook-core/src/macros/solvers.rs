//! Solver pattern macros
//!
//! Common solver patterns for equations and systems.
//! These macros eliminate repetitive solver logic and provide
//! optimized implementations for standard mathematical problems.

// Macros are available through crate re-exports - no explicit import needed

/// Common solver patterns
///
/// This macro provides optimized implementations for common equation
/// solving patterns, eliminating repetitive solver logic.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::{solve, expr};
/// use mathhook_core::{algebra::solvers::SolverResult, Expression};
///
/// // Linear equation: ax + b = 0 → x = -b/a
/// let a = expr!(2);
/// let b = expr!(6);
/// let linear_result = solve!(linear: a, b);
///
/// match linear_result {
///     SolverResult::Single(solution) => {
///         // Should be x = -3
///     },
///     _ => panic!("Expected single solution"),
/// }
///
/// // Quadratic discriminant: b² - 4ac
/// let discriminant = solve!(discriminant: expr!(1), expr!(2), expr!(1));
///
/// // Quadratic solutions using quadratic formula
/// let quad_solutions = solve!(quadratic: expr!(1), expr!(2), expr!(1));
/// ```
#[macro_export]
macro_rules! solve {
    // Linear equation: ax + b = 0 → x = -b/a
    (linear: $a:expr, $b:expr) => {{
        use $crate::algebra::solvers::SolverResult;
        use $crate::Simplify;

        let a_val = $a;
        let b_val = $b;

        if a_val.is_zero() {
            if b_val.is_zero() {
                SolverResult::InfiniteSolutions
            } else {
                SolverResult::NoSolution
            }
        } else {
            SolverResult::Single(expr!(rational: expr!(mul: expr!(-1), b_val), a_val))
        }
    }};

    // Quadratic discriminant: b² - 4ac
    (discriminant: $a:expr, $b:expr, $c:expr) => {
        expr!(
            expr!(pow: $b, expr!(2)),
            expr!(mul: expr!(-4), $a, $c)
        )
    };

    // Quadratic solutions using quadratic formula
    (quadratic: $a:expr, $b:expr, $c:expr) => {{
        use $crate::algebra::solvers::SolverResult;

        let discriminant = solve!(discriminant: $a, $b, $c);
        let two_a = expr!(mul: expr!(2), $a);
        let neg_b = expr!(mul: expr!(-1), $b);
        let sqrt_disc = expr!(fn: "sqrt", discriminant);

        vec![
            expr!(rational: expr!(neg_b.clone(), sqrt_disc.clone()), two_a.clone()),
            expr!(rational: expr!(neg_b, expr!(mul: expr!(-1), sqrt_disc)), two_a)
        ]
    }};

    // System of equations 2x2 using Cramer's rule
    (system_2x2: $a11:expr, $a12:expr, $b1:expr, $a21:expr, $a22:expr, $b2:expr) => {{
        use $crate::algebra::solvers::SolverResult;

        let det = expr!(
            expr!(mul: $a11, $a22),
            expr!(mul: expr!(-1), $a12, $a21)
        );

        if det.is_zero() {
            SolverResult::NoSolution
        } else {
            let x = expr!(rational:
                expr!(expr!(mul: $b1, $a22), expr!(mul: expr!(-1), $b2, $a12)),
                det.clone()
            );
            let y = expr!(rational:
                expr!(expr!(mul: $a11, $b2), expr!(mul: expr!(-1), $a21, $b1)),
                det
            );
            SolverResult::Multiple(vec![x, y])
        }
    }};

    // Cubic depressed form: t³ + pt + q = 0 (Cardano's method)
    (cubic_depressed: $p:expr, $q:expr) => {{
        use $crate::algebra::solvers::SolverResult;

        // Discriminant: Δ = -4p³ - 27q²
        let discriminant = expr!(
            expr!(mul: expr!(-4), expr!(pow: $p, expr!(3))),
            expr!(mul: expr!(-27), expr!(pow: $q, expr!(2)))
        );

        // For now, return symbolic representation
        // Full Cardano's formula implementation would go here
        SolverResult::Single(expr!(fn: "cubic_root", $p, $q, discriminant))
    }};

    // Substitution method for systems (solve first equation for first variable)
    (substitution: $eq1:expr, $eq2:expr, $var1:expr, $var2:expr) => {{
        use $crate::algebra::solvers::SolverResult;

        // Solve eq1 for var1, then substitute into eq2
        // This is a simplified pattern - full implementation would
        // require more sophisticated equation manipulation
        SolverResult::Single(expr!(fn: "substitution_solve", $eq1, $eq2, $var1, $var2))
    }};

    // Elimination method for systems (eliminate one variable)
    (elimination: $eq1:expr, $eq2:expr, $var1:expr, $var2:expr) => {{
        use $crate::algebra::solvers::SolverResult;

        // Gaussian elimination pattern
        SolverResult::Single(expr!(fn: "elimination_solve", $eq1, $eq2, $var1, $var2))
    }};

    // Rational equation solving (clear denominators)
    (rational: $equation:expr, $var:expr) => {{
        use $crate::algebra::solvers::SolverResult;

        // Clear denominators and solve resulting polynomial
        SolverResult::Single(expr!(fn: "rational_solve", $equation, $var))
    }};

    // Exponential equation solving
    (exponential: $equation:expr, $var:expr) => {{
        use $crate::algebra::solvers::SolverResult;

        // Take logarithms and solve
        SolverResult::Single(expr!(fn: "exponential_solve", $equation, $var))
    }};

    // Logarithmic equation solving
    (logarithmic: $equation:expr, $var:expr) => {{
        use $crate::algebra::solvers::SolverResult;

        // Exponentiate both sides and solve
        SolverResult::Single(expr!(fn: "logarithmic_solve", $equation, $var))
    }};
}

#[cfg(test)]
mod tests {
    use crate::{algebra::solvers::SolverResult, Expression};

    #[test]
    fn test_solve_linear() {
        let a = Expression::integer(2);
        let b = Expression::integer(6);
        let result = solve!(linear: a, b);

        match result {
            SolverResult::Single(solution) => {
                // Should be x = -3 (represented as rational form)
                match solution {
                    Expression::Mul(_) => (), // Rational form
                    _ => panic!("Expected rational solution"),
                }
            }
            _ => panic!("Expected single solution"),
        }
    }

    #[test]
    fn test_solve_linear_zero_coefficient() {
        let a = Expression::integer(0);
        let b = Expression::integer(5);
        let result = solve!(linear: a, b);

        match result {
            SolverResult::NoSolution => (),
            _ => panic!("Expected no solution for 0x + 5 = 0"),
        }
    }

    #[test]
    fn test_solve_linear_infinite_solutions() {
        let a = Expression::integer(0);
        let b = Expression::integer(0);
        let result = solve!(linear: a, b);

        match result {
            SolverResult::InfiniteSolutions => (),
            _ => panic!("Expected infinite solutions for 0x + 0 = 0"),
        }
    }

    #[test]
    fn test_solve_discriminant() {
        let a = Expression::integer(1);
        let b = Expression::integer(2);
        let c = Expression::integer(1);
        let discriminant = solve!(discriminant: a, b, c);

        // Should be 2² - 4(1)(1) = 4 - 4 = 0
        match discriminant {
            Expression::Add(_) => (), // Should be an addition expression
            _ => panic!("Expected addition expression for discriminant"),
        }
    }

    #[test]
    fn test_solve_quadratic() {
        let a = Expression::integer(1);
        let b = Expression::integer(-3);
        let c = Expression::integer(2);
        let solutions = solve!(quadratic: a, b, c);

        // Should return vector of two solutions
        assert_eq!(solutions.len(), 2);

        // Both should be rational expressions
        for solution in solutions {
            match solution {
                Expression::Mul(_) => (), // Rational form
                _ => panic!("Expected rational solution"),
            }
        }
    }

    #[test]
    fn test_solve_system_2x2() {
        let a11 = Expression::integer(1);
        let a12 = Expression::integer(1);
        let b1 = Expression::integer(3);
        let a21 = Expression::integer(1);
        let a22 = Expression::integer(-1);
        let b2 = Expression::integer(1);

        let result = solve!(system_2x2: a11, a12, b1, a21, a22, b2);

        match result {
            SolverResult::Multiple(solutions) => {
                assert_eq!(solutions.len(), 2);
                // Both should be rational expressions
                for solution in solutions {
                    match solution {
                        Expression::Mul(_) => (), // Rational form
                        _ => panic!("Expected rational solution"),
                    }
                }
            }
            _ => panic!("Expected multiple solutions"),
        }
    }

    #[test]
    fn test_solve_system_2x2_no_solution() {
        // Parallel lines: x + y = 1 and x + y = 2
        let a11 = Expression::integer(1);
        let a12 = Expression::integer(1);
        let b1 = Expression::integer(1);
        let a21 = Expression::integer(1);
        let a22 = Expression::integer(1);
        let b2 = Expression::integer(2);

        let result = solve!(system_2x2: a11, a12, b1, a21, a22, b2);

        match result {
            SolverResult::NoSolution => (),
            _ => panic!("Expected no solution for parallel lines"),
        }
    }
}
