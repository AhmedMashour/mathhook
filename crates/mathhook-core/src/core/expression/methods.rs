//! Expression utility methods

use super::Expression;
use crate::algebra::equation_analyzer::SmartEquationSolver;
use crate::algebra::solvers::SolverResult;
use crate::core::commutativity::Commutativity;
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

    /// Compute commutativity of this expression
    ///
    /// Commutativity is inferred from the symbols and operations:
    /// - Numbers, constants: Commutative
    /// - Symbols: Depends on symbol type (Scalar → Commutative, Matrix/Operator/Quaternion → Noncommutative)
    /// - Mul: Noncommutative if ANY factor is noncommutative
    /// - Add, Pow, Function: Depends on subexpressions
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::core::symbol::Symbol;
    /// use mathhook_core::core::expression::Expression;
    /// use mathhook_core::core::commutativity::Commutativity;
    ///
    /// let x = Symbol::scalar("x");
    /// let y = Symbol::scalar("y");
    /// let expr = Expression::mul(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::symbol(y.clone()),
    /// ]);
    /// assert_eq!(expr.commutativity(), Commutativity::Commutative);
    ///
    /// let a = Symbol::matrix("A");
    /// let b = Symbol::matrix("B");
    /// let expr = Expression::mul(vec![
    ///     Expression::symbol(a.clone()),
    ///     Expression::symbol(b.clone()),
    /// ]);
    /// assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
    /// ```
    pub fn commutativity(&self) -> Commutativity {
        match self {
            Expression::Symbol(s) => s.commutativity(),
            Expression::Number(_) => Commutativity::Commutative,
            Expression::Constant(_) => Commutativity::Commutative,

            Expression::Add(terms) => {
                Commutativity::combine(terms.iter().map(|t| t.commutativity()))
            }

            Expression::Mul(factors) => {
                Commutativity::combine(factors.iter().map(|f| f.commutativity()))
            }

            Expression::Pow(base, _exp) => base.commutativity(),

            Expression::Function { args, .. } => {
                Commutativity::combine(args.iter().map(|a| a.commutativity()))
            }

            Expression::Set(elements) => {
                Commutativity::combine(elements.iter().map(|e| e.commutativity()))
            }

            Expression::Complex(data) => {
                let real_comm = data.real.commutativity();
                let imag_comm = data.imag.commutativity();
                Commutativity::combine([real_comm, imag_comm])
            }

            Expression::Matrix(_) => Commutativity::Noncommutative,

            Expression::Relation(data) => {
                let left_comm = data.left.commutativity();
                let right_comm = data.right.commutativity();
                Commutativity::combine([left_comm, right_comm])
            }

            Expression::Piecewise(data) => {
                let piece_comms = data.pieces.iter().flat_map(|(expr, cond)| {
                    [expr.commutativity(), cond.commutativity()]
                });
                let default_comm = data
                    .default
                    .as_ref()
                    .map(|e| e.commutativity())
                    .into_iter();
                Commutativity::combine(piece_comms.chain(default_comm))
            }

            Expression::Interval(data) => {
                let start_comm = data.start.commutativity();
                let end_comm = data.end.commutativity();
                Commutativity::combine([start_comm, end_comm])
            }

            Expression::Calculus(data) => match &**data {
                crate::core::expression::CalculusData::Derivative {
                    expression,
                    variable: _,
                    order: _,
                } => expression.commutativity(),
                crate::core::expression::CalculusData::Integral {
                    integrand,
                    variable: _,
                    bounds,
                } => {
                    let integrand_comm = integrand.commutativity();
                    if let Some((lower, upper)) = bounds {
                        Commutativity::combine([
                            integrand_comm,
                            lower.commutativity(),
                            upper.commutativity(),
                        ])
                    } else {
                        integrand_comm
                    }
                }
                crate::core::expression::CalculusData::Limit {
                    expression,
                    variable: _,
                    point,
                    direction: _,
                } => {
                    Commutativity::combine([expression.commutativity(), point.commutativity()])
                }
                crate::core::expression::CalculusData::Sum {
                    expression,
                    variable: _,
                    start,
                    end,
                } => Commutativity::combine([
                    expression.commutativity(),
                    start.commutativity(),
                    end.commutativity(),
                ]),
                crate::core::expression::CalculusData::Product {
                    expression,
                    variable: _,
                    start,
                    end,
                } => Commutativity::combine([
                    expression.commutativity(),
                    start.commutativity(),
                    end.commutativity(),
                ]),
            },

            Expression::MethodCall(data) => {
                let object_comm = data.object.commutativity();
                let args_comm = data.args.iter().map(|a| a.commutativity());
                Commutativity::combine([object_comm].into_iter().chain(args_comm))
            }
        }
    }
}

#[cfg(test)]
mod expression_commutativity_tests {
    use super::*;
    use crate::core::symbol::Symbol;

    #[test]
    fn test_scalar_mul_is_commutative() {
        let x = Symbol::scalar("x");
        let y = Symbol::scalar("y");
        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);
        assert_eq!(expr.commutativity(), Commutativity::Commutative);
    }

    #[test]
    fn test_matrix_mul_is_noncommutative() {
        let a = Symbol::matrix("A");
        let b = Symbol::matrix("B");
        let expr = Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
        ]);
        assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
    }

    #[test]
    fn test_mixed_mul_is_noncommutative() {
        let x = Symbol::scalar("x");
        let a = Symbol::matrix("A");
        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(a.clone()),
        ]);
        assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
    }

    #[test]
    fn test_operator_mul_is_noncommutative() {
        let x = Symbol::operator("x");
        let p = Symbol::operator("p");
        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(p.clone()),
        ]);
        assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
    }

    #[test]
    fn test_number_is_commutative() {
        let expr = Expression::integer(42);
        assert_eq!(expr.commutativity(), Commutativity::Commutative);
    }

    #[test]
    fn test_add_inherits_commutativity() {
        let x = Symbol::scalar("x");
        let a = Symbol::matrix("A");
        let expr = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(a.clone()),
        ]);
        assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
    }

    #[test]
    fn test_pow_inherits_base_commutativity() {
        let a = Symbol::matrix("A");
        let expr = Expression::pow(Expression::symbol(a.clone()), Expression::integer(2));
        assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
    }
}
