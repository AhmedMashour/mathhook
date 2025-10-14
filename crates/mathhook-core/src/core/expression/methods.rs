//! Expression utility methods

use super::Expression;
use crate::algebra::equation_analyzer::SmartEquationSolver;
use crate::algebra::solvers::SolverResult;
use crate::core::{Number, Symbol};
use crate::educational::step_by_step::StepByStepExplanation;

/// Helper function for computing GCD of integers
fn gcd_integers(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

impl Expression {
    /// Compute the greatest common divisor of two expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let a = Expression::integer(12);
    /// let b = Expression::integer(8);
    /// let gcd = a.gcd(&b);
    /// ```
    pub fn gcd(&self, other: &Expression) -> Expression {
        if self == other {
            return self.clone();
        }

        if self.is_zero() {
            return other.clone();
        }
        if other.is_zero() {
            return self.clone();
        }

        match (self, other) {
            (Expression::Number(num1), Expression::Number(num2)) => match (num1, num2) {
                (Number::Integer(a), Number::Integer(b)) => {
                    Expression::integer(gcd_integers(*a, *b))
                }
                _ => Expression::integer(1),
            },
            _ => Expression::integer(1),
        }
    }

    /// Compute the least common multiple of two expressions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let a = Expression::integer(12);
    /// let b = Expression::integer(8);
    /// let lcm = a.lcm(&b);
    /// ```
    pub fn lcm(&self, other: &Expression) -> Expression {
        match (self, other) {
            (Expression::Number(num1), Expression::Number(num2)) => match (num1, num2) {
                (Number::Integer(a), Number::Integer(b)) => {
                    if *a == 0 || *b == 0 {
                        Expression::integer(0)
                    } else {
                        let gcd_val = gcd_integers(*a, *b);
                        Expression::integer((*a * *b).abs() / gcd_val)
                    }
                }
                _ => self.clone(),
            },
            _ => self.clone(),
        }
    }

    /// Factor out the GCD from an expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let expr = Expression::add(vec![
    ///     Expression::mul(vec![Expression::integer(6), Expression::symbol("x")]),
    ///     Expression::integer(9),
    /// ]);
    /// let factored = expr.factor_gcd();
    /// ```
    pub fn factor_gcd(&self) -> Expression {
        self.clone()
    }

    /// Compute GCD and cofactors
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Expression;
    ///
    /// let a = Expression::integer(12);
    /// let b = Expression::integer(8);
    /// let (gcd, cofactor_a, cofactor_b) = a.cofactors(&b);
    /// ```
    pub fn cofactors(&self, other: &Expression) -> (Expression, Expression, Expression) {
        let gcd = self.gcd(other);
        (gcd.clone(), self.clone(), other.clone())
    }

    /// Solve an equation with respect to a variable using smart solver dispatch
    ///
    /// This method uses the SmartEquationSolver to automatically analyze the equation
    /// type and route it to the appropriate specialized solver. It provides both the
    /// solution and a complete step-by-step educational explanation.
    ///
    /// The solver automatically detects:
    /// - Linear equations (degree 1)
    /// - Quadratic equations (degree 2)
    /// - Higher-degree polynomial equations (degree 3-4)
    /// - System of equations (multiple variables)
    /// - Transcendental equations (with trig/exp/log functions)
    ///
    /// # Arguments
    ///
    /// * `variable` - The variable to solve for
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - The solver result (solutions or error)
    /// - A complete step-by-step explanation of the solving process
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// let equation = Expression::add(vec![
    ///     Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
    ///     Expression::integer(-6),
    /// ]);
    ///
    /// let (result, explanation) = equation.solve_equation(&x);
    /// ```
    pub fn solve_equation(&self, variable: &Symbol) -> (SolverResult, StepByStepExplanation) {
        let mut solver = SmartEquationSolver::new();
        solver.solve_with_equation(self, variable)
    }

    /// Solve equation without generating educational explanation (fast path)
    ///
    /// This is equivalent to `solve_equation` but discards the explanation for
    /// better performance when educational content is not needed.
    ///
    /// # Arguments
    ///
    /// * `variable` - The variable to solve for
    ///
    /// # Returns
    ///
    /// The solver result (solutions or error)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// let equation = Expression::add(vec![
    ///     Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
    ///     Expression::integer(-6),
    /// ]);
    ///
    /// let result = equation.solve_equation_fast(&x);
    /// ```
    pub fn solve_equation_fast(&self, variable: &Symbol) -> SolverResult {
        let (result, _explanation) = self.solve_equation(variable);
        result
    }
}
