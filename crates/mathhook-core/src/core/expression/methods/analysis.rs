//! Expression analysis methods
//!
//! This module provides methods for analyzing properties of expressions,
//! including commutativity analysis and variable occurrence counting.

use super::super::Expression;
use crate::core::commutativity::Commutativity;
use crate::core::Symbol;
use std::sync::Arc;

impl Expression {
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
    /// Basic scalar symbols (commutative):
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
    /// ```
    ///
    /// Matrix symbols (noncommutative):
    /// ```
    /// use mathhook_core::core::symbol::Symbol;
    /// use mathhook_core::core::expression::Expression;
    /// use mathhook_core::core::commutativity::Commutativity;
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
                let piece_comms = data
                    .pieces
                    .iter()
                    .flat_map(|(expr, cond)| [expr.commutativity(), cond.commutativity()]);
                let default_comm = data.default.as_ref().map(|e| e.commutativity()).into_iter();
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
                } => Commutativity::combine([expression.commutativity(), point.commutativity()]),
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

    /// Count occurrences of a variable in the expression
    ///
    /// Recursively counts how many times a specific variable symbol appears
    /// in the expression tree. This is useful for:
    /// - Determining if an expression is polynomial in a variable
    /// - Analyzing variable dependencies
    /// - Checking if a variable appears in an equation
    ///
    /// # Arguments
    ///
    /// * `variable` - The symbol to count occurrences of
    ///
    /// # Returns
    ///
    /// The number of times the variable appears in the expression
    ///
    /// # Examples
    ///
    /// Basic counting in simple expressions:
    /// ```
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::mul(vec![
    ///     Expression::integer(2),
    ///     Expression::symbol(x.clone()),
    /// ]);
    /// assert_eq!(expr.count_variable_occurrences(&x), 1);
    /// ```
    ///
    /// Counting multiple occurrences:
    /// ```
    /// use mathhook_core::{Expression, symbol};
    /// use std::sync::Arc;
    ///
    /// let x = symbol!(x);
    /// // x^2 + 2*x + 1 has 2 occurrences of x (in x^2 and in 2*x)
    /// let expr = Expression::Add(Arc::new(vec![
    ///     Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
    ///     Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
    ///     Expression::integer(1),
    /// ]));
    /// assert_eq!(expr.count_variable_occurrences(&x), 2);
    /// ```
    ///
    /// Counting in power expressions:
    /// ```
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// // x^x has 2 occurrences (base and exponent)
    /// let expr = Expression::pow(
    ///     Expression::symbol(x.clone()),
    ///     Expression::symbol(x.clone())
    /// );
    /// assert_eq!(expr.count_variable_occurrences(&x), 2);
    /// ```
    ///
    /// Counting in functions:
    /// ```
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// // sin(x)
    /// let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    /// assert_eq!(expr.count_variable_occurrences(&x), 1);
    ///
    /// // f(x, x, 2) has 2 occurrences
    /// let expr2 = Expression::function("f", vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::symbol(x.clone()),
    ///     Expression::integer(2),
    /// ]);
    /// assert_eq!(expr2.count_variable_occurrences(&x), 2);
    /// ```
    ///
    /// Zero occurrences when variable is not present:
    /// ```
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let expr = Expression::symbol(y.clone());
    /// assert_eq!(expr.count_variable_occurrences(&x), 0);
    /// ```
    pub fn count_variable_occurrences(&self, variable: &Symbol) -> usize {
        match self {
            Expression::Symbol(s) if s == variable => 1,
            Expression::Symbol(_) | Expression::Number(_) | Expression::Constant(_) => 0,

            Expression::Add(terms) | Expression::Mul(terms) | Expression::Set(terms) => terms
                .iter()
                .map(|t| t.count_variable_occurrences(variable))
                .sum(),

            Expression::Pow(base, exp) => {
                base.count_variable_occurrences(variable)
                    + exp.count_variable_occurrences(variable)
            }

            Expression::Function { args, .. } => args
                .iter()
                .map(|a| a.count_variable_occurrences(variable))
                .sum(),

            Expression::Complex(data) => {
                data.real.count_variable_occurrences(variable)
                    + data.imag.count_variable_occurrences(variable)
            }

            Expression::Matrix(matrix) => {
                let (rows, cols) = matrix.dimensions();
                let mut count = 0;
                for i in 0..rows {
                    for j in 0..cols {
                        count += matrix
                            .get_element(i, j)
                            .count_variable_occurrences(variable);
                    }
                }
                count
            }

            Expression::Relation(data) => {
                data.left.count_variable_occurrences(variable)
                    + data.right.count_variable_occurrences(variable)
            }

            Expression::Piecewise(data) => {
                let pieces_count: usize = data
                    .pieces
                    .iter()
                    .map(|(expr, cond)| {
                        expr.count_variable_occurrences(variable)
                            + cond.count_variable_occurrences(variable)
                    })
                    .sum();
                let default_count = data
                    .default
                    .as_ref()
                    .map_or(0, |e| e.count_variable_occurrences(variable));
                pieces_count + default_count
            }

            Expression::Interval(data) => {
                data.start.count_variable_occurrences(variable)
                    + data.end.count_variable_occurrences(variable)
            }

            Expression::Calculus(data) => match data.as_ref() {
                crate::core::expression::data_types::CalculusData::Derivative {
                    expression,
                    variable: v,
                    ..
                } => {
                    expression.count_variable_occurrences(variable)
                        + if v == variable { 1 } else { 0 }
                }
                crate::core::expression::data_types::CalculusData::Integral {
                    integrand,
                    variable: v,
                    bounds,
                } => {
                    let integrand_count = integrand.count_variable_occurrences(variable);
                    let var_count = if v == variable { 1 } else { 0 };
                    let bounds_count = bounds.as_ref().map_or(0, |(lower, upper)| {
                        lower.count_variable_occurrences(variable)
                            + upper.count_variable_occurrences(variable)
                    });
                    integrand_count + var_count + bounds_count
                }
                crate::core::expression::data_types::CalculusData::Limit {
                    expression,
                    variable: v,
                    point,
                    ..
                } => {
                    expression.count_variable_occurrences(variable)
                        + if v == variable { 1 } else { 0 }
                        + point.count_variable_occurrences(variable)
                }
                crate::core::expression::data_types::CalculusData::Sum {
                    expression,
                    variable: v,
                    start,
                    end,
                }
                | crate::core::expression::data_types::CalculusData::Product {
                    expression,
                    variable: v,
                    start,
                    end,
                } => {
                    expression.count_variable_occurrences(variable)
                        + if v == variable { 1 } else { 0 }
                        + start.count_variable_occurrences(variable)
                        + end.count_variable_occurrences(variable)
                }
            },

            Expression::MethodCall(data) => {
                data.object.count_variable_occurrences(variable)
                    + data
                        .args
                        .iter()
                        .map(|a| a.count_variable_occurrences(variable))
                        .sum::<usize>()
            }
        }
    }

    pub fn contains_variable(&self, symbol: &Symbol) -> bool {
        self.count_variable_occurrences(symbol) > 0
    }

    /// Check if expression is just the variable itself
    pub fn is_simple_variable(&self, var: &Symbol) -> bool {
        matches!(self, Expression::Symbol(s) if s == var)
    }

    /// Check if this expression is a specific symbol
    ///
    /// Convenience method for pattern matching against a specific symbol.
    /// More readable than inline matches! pattern in complex conditions.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to check against
    ///
    /// # Returns
    ///
    /// True if this expression is exactly the given symbol
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::{Expression, symbol};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let expr = Expression::symbol(x.clone());
    ///
    /// assert!(expr.is_symbol_matching(&x));
    /// assert!(!expr.is_symbol_matching(&y));
    /// ```
    pub fn is_symbol_matching(&self, symbol: &Symbol) -> bool {
        matches!(self, Expression::Symbol(s) if s == symbol)
    }

    /// Extract the base and exponent from a Pow expression
    ///
    /// Returns Some((base, exp)) if this is a Pow expression, None otherwise.
    /// This is a helper method for pattern matching with the Arc-based structure.
    #[inline]
    pub fn as_pow(&self) -> Option<(&Expression, &Expression)> {
        match self {
            Expression::Pow(base, exp) => Some((base.as_ref(), exp.as_ref())),
            _ => None,
        }
    }

    /// Extract the name and args from a Function expression
    ///
    /// Returns Some((name, args)) if this is a Function expression, None otherwise.
    /// This is a helper method for pattern matching with the Arc-based structure.
    #[inline]
    pub fn as_function(&self) -> Option<(&str, &[Expression])> {
        match self {
            Expression::Function { name, args } => Some((name.as_ref(), args.as_slice())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::expression::data_types::{
        CalculusData, ComplexData, PiecewiseData, RelationData, RelationType,
    };
    use crate::expr;
    use crate::matrices::unified::Matrix;
    use crate::symbol;

    #[test]
    fn test_commutativity_scalar_multiplication() {
        let x = Symbol::scalar("x");
        let y = Symbol::scalar("y");
        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);
        assert_eq!(expr.commutativity(), Commutativity::Commutative);
    }

    #[test]
    fn test_commutativity_matrix_multiplication() {
        let a = Symbol::matrix("A");
        let b = Symbol::matrix("B");
        let expr = Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
        ]);
        assert_eq!(expr.commutativity(), Commutativity::Noncommutative);
    }

    #[test]
    fn test_count_in_symbol() {
        let x = symbol!(x);
        let expr = Expression::symbol(x.clone());
        assert_eq!(expr.count_variable_occurrences(&x), 1);

        let y = symbol!(y);
        assert_eq!(expr.count_variable_occurrences(&y), 0);
    }

    #[test]
    fn test_count_in_add() {
        let x = symbol!(x);
        let y = symbol!(y);
        let raw_expr = Expression::Add(Arc::new(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]));
        assert_eq!(raw_expr.count_variable_occurrences(&x), 2);
        assert_eq!(raw_expr.count_variable_occurrences(&y), 1);
    }

    #[test]
    fn test_count_in_pow() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), expr!(2));
        assert_eq!(expr.count_variable_occurrences(&x), 1);

        let expr2 = Expression::pow(Expression::symbol(x.clone()), Expression::symbol(x.clone()));
        assert_eq!(expr2.count_variable_occurrences(&x), 2);
    }

    #[test]
    fn test_count_in_function() {
        let x = symbol!(x);
        let expr = Expression::function("sin", vec![Expression::symbol(x.clone())]);
        assert_eq!(expr.count_variable_occurrences(&x), 1);

        let expr2 = Expression::function(
            "f",
            vec![
                Expression::symbol(x.clone()),
                Expression::symbol(x.clone()),
                expr!(2),
            ],
        );
        assert_eq!(expr2.count_variable_occurrences(&x), 2);
    }

    #[test]
    fn test_count_in_matrix() {
        let x = symbol!(x);
        let y = symbol!(y);
        let matrix = Matrix::dense(vec![
            vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())],
            vec![Expression::symbol(x.clone()), Expression::integer(1)],
        ]);
        let expr = Expression::Matrix(Arc::new(matrix));
        assert_eq!(expr.count_variable_occurrences(&x), 2);
        assert_eq!(expr.count_variable_occurrences(&y), 1);
    }

    #[test]
    fn test_count_in_complex() {
        let x = symbol!(x);
        let expr = Expression::Complex(Arc::new(ComplexData {
            real: Expression::symbol(x.clone()),
            imag: Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        }));
        assert_eq!(expr.count_variable_occurrences(&x), 2);
    }

    #[test]
    fn test_count_in_relation() {
        let x = symbol!(x);
        let expr = Expression::Relation(Arc::new(RelationData {
            left: Expression::symbol(x.clone()),
            right: Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            relation_type: RelationType::Equal,
        }));
        assert_eq!(expr.count_variable_occurrences(&x), 2);
    }

    #[test]
    fn test_count_in_piecewise() {
        let x = symbol!(x);
        let expr = Expression::Piecewise(Arc::new(PiecewiseData {
            pieces: vec![
                (Expression::symbol(x.clone()), Expression::symbol(x.clone())),
                (Expression::integer(0), Expression::symbol(x.clone())),
            ],
            default: Some(Expression::symbol(x.clone())),
        }));
        assert_eq!(expr.count_variable_occurrences(&x), 4);
    }

    #[test]
    fn test_count_in_integral() {
        let x = symbol!(x);
        let expr = Expression::Calculus(Arc::new(CalculusData::Integral {
            integrand: Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            variable: x.clone(),
            bounds: Some((Expression::integer(0), Expression::symbol(x.clone()))),
        }));
        assert_eq!(expr.count_variable_occurrences(&x), 3);
    }

    #[test]
    fn test_is_symbol_matching() {
        let x = symbol!(x);
        let y = symbol!(y);
        let expr_x = Expression::symbol(x.clone());
        let expr_num = Expression::integer(42);

        assert!(expr_x.is_symbol_matching(&x));
        assert!(!expr_x.is_symbol_matching(&y));
        assert!(!expr_num.is_symbol_matching(&x));
    }

    #[test]
    fn test_as_pow() {
        let x = symbol!(x);
        let pow_expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

        let (base, exp) = pow_expr.as_pow().expect("should be a Pow");
        assert_eq!(*base, Expression::symbol(x.clone()));
        assert_eq!(*exp, Expression::integer(2));

        let not_pow = Expression::integer(42);
        assert!(not_pow.as_pow().is_none());
    }

    #[test]
    fn test_as_function() {
        let x = symbol!(x);
        let func = Expression::function("sin", vec![Expression::symbol(x.clone())]);

        let (name, args) = func.as_function().expect("should be a Function");
        assert_eq!(name, "sin");
        assert_eq!(args.len(), 1);
        assert_eq!(args[0], Expression::symbol(x.clone()));

        let not_func = Expression::integer(42);
        assert!(not_func.as_function().is_none());
    }
}
