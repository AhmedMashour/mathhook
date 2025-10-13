//! Substitution system for replacing expressions
//!
//! Provides recursive tree-walking substitution for Expression types.

use crate::core::{Expression, MathConstant, Number, Symbol};
use crate::pattern::matching::Pattern;
use crate::simplify::Simplify;
use std::collections::HashMap;

/// Trait for types that support substitution operations
pub trait Substitutable {
    /// Substitute a single expression with another
    ///
    /// Recursively walks the expression tree and replaces all occurrences
    /// of `old` with `new`. The replacement is structural - it compares
    /// expressions using PartialEq.
    ///
    /// # Arguments
    ///
    /// * `old` - The expression to replace
    /// * `new` - The expression to substitute in
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::prelude::*;
    /// use mathhook_core::pattern::Substitutable;
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::add(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::integer(1)
    /// ]);
    ///
    /// let result = expr.subs(&Expression::symbol(x.clone()), &Expression::integer(5));
    /// let expected = Expression::add(vec![
    ///     Expression::integer(5),
    ///     Expression::integer(1)
    /// ]);
    ///
    /// assert_eq!(result, expected);
    /// ```
    fn subs(&self, old: &Expression, new: &Expression) -> Expression;

    /// Apply multiple substitutions simultaneously
    ///
    /// This is more efficient than chaining multiple `subs()` calls because
    /// it performs all substitutions in a single tree traversal.
    ///
    /// # Arguments
    ///
    /// * `substitutions` - Slice of (old, new) expression pairs
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::prelude::*;
    /// use mathhook_core::pattern::Substitutable;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let expr = Expression::add(vec![
    ///     Expression::symbol(x.clone()),
    ///     Expression::symbol(y.clone())
    /// ]);
    ///
    /// let result = expr.subs_multiple(&[
    ///     (Expression::symbol(x.clone()), Expression::integer(1)),
    ///     (Expression::symbol(y.clone()), Expression::integer(2)),
    /// ]);
    ///
    /// let expected = Expression::add(vec![
    ///     Expression::integer(1),
    ///     Expression::integer(2)
    /// ]);
    ///
    /// assert_eq!(result, expected);
    /// ```
    fn subs_multiple(&self, substitutions: &[(Expression, Expression)]) -> Expression;
}

impl Substitutable for Expression {
    fn subs(&self, old: &Expression, new: &Expression) -> Expression {
        // If this expression matches the old expression, replace it
        if self == old {
            return new.clone();
        }

        // Otherwise, recursively substitute in subexpressions
        let result = match self {
            // Atomic types - no substitution needed
            Expression::Number(_) | Expression::Constant(_) => self.clone(),

            // Symbol - already checked equality above
            Expression::Symbol(_) => self.clone(),

            // Add - substitute in each term
            Expression::Add(terms) => {
                let new_terms: Vec<Expression> = terms.iter().map(|t| t.subs(old, new)).collect();
                Expression::Add(Box::new(new_terms))
            }

            // Multiply - substitute in each factor
            Expression::Mul(factors) => {
                let new_factors: Vec<Expression> =
                    factors.iter().map(|f| f.subs(old, new)).collect();
                Expression::Mul(Box::new(new_factors))
            }

            // Power - substitute in base and exponent
            Expression::Pow(base, exp) => {
                let new_base = base.subs(old, new);
                let new_exp = exp.subs(old, new);
                Expression::Pow(Box::new(new_base), Box::new(new_exp))
            }

            // Function - substitute in arguments
            Expression::Function { name, args } => {
                let new_args: Vec<Expression> = args.iter().map(|a| a.subs(old, new)).collect();
                Expression::Function {
                    name: name.clone(),
                    args: Box::new(new_args),
                }
            }

            // Set - substitute in elements
            Expression::Set(elements) => {
                let new_elements: Vec<Expression> =
                    elements.iter().map(|e| e.subs(old, new)).collect();
                Expression::Set(Box::new(new_elements))
            }

            // Complex - substitute in real and imaginary parts
            Expression::Complex(data) => {
                let new_real = data.real.subs(old, new);
                let new_imag = data.imag.subs(old, new);
                Expression::Complex(Box::new(crate::core::expression::ComplexData {
                    real: new_real,
                    imag: new_imag,
                }))
            }

            // Matrix - substitute in each element
            Expression::Matrix(_matrix) => {
                // TODO: Implement Matrix substitution when Matrix API is finalized
                self.clone()
            }

            // Relation - substitute in both sides
            Expression::Relation(data) => {
                let new_left = data.left.subs(old, new);
                let new_right = data.right.subs(old, new);
                Expression::Relation(Box::new(crate::core::expression::RelationData {
                    left: new_left,
                    right: new_right,
                    relation_type: data.relation_type,
                }))
            }

            // Piecewise - substitute in pieces and default
            Expression::Piecewise(data) => {
                let new_pieces: Vec<(Expression, Expression)> = data
                    .pieces
                    .iter()
                    .map(|(expr, cond)| (expr.subs(old, new), cond.subs(old, new)))
                    .collect();

                let new_default = data.default.as_ref().map(|d| d.subs(old, new));

                Expression::Piecewise(Box::new(crate::core::expression::PiecewiseData {
                    pieces: new_pieces,
                    default: new_default,
                }))
            }

            // Interval - substitute in bounds
            Expression::Interval(data) => {
                let new_start = data.start.subs(old, new);
                let new_end = data.end.subs(old, new);
                Expression::Interval(Box::new(crate::core::expression::IntervalData {
                    start: new_start,
                    end: new_end,
                    start_inclusive: data.start_inclusive,
                    end_inclusive: data.end_inclusive,
                }))
            }

            // Calculus - substitute in calculus expressions
            Expression::Calculus(data) => {
                use crate::core::expression::CalculusData;

                let new_data = match data.as_ref() {
                    CalculusData::Derivative {
                        expression,
                        variable,
                        order,
                    } => CalculusData::Derivative {
                        expression: expression.subs(old, new),
                        variable: variable.clone(),
                        order: *order,
                    },

                    CalculusData::Integral {
                        integrand,
                        variable,
                        bounds,
                    } => CalculusData::Integral {
                        integrand: integrand.subs(old, new),
                        variable: variable.clone(),
                        bounds: bounds
                            .as_ref()
                            .map(|(a, b)| (a.subs(old, new), b.subs(old, new))),
                    },

                    CalculusData::Limit {
                        expression,
                        variable,
                        point,
                        direction,
                    } => CalculusData::Limit {
                        expression: expression.subs(old, new),
                        variable: variable.clone(),
                        point: point.subs(old, new),
                        direction: *direction,
                    },

                    CalculusData::Sum {
                        expression,
                        variable,
                        start,
                        end,
                    } => CalculusData::Sum {
                        expression: expression.subs(old, new),
                        variable: variable.clone(),
                        start: start.subs(old, new),
                        end: end.subs(old, new),
                    },

                    CalculusData::Product {
                        expression,
                        variable,
                        start,
                        end,
                    } => CalculusData::Product {
                        expression: expression.subs(old, new),
                        variable: variable.clone(),
                        start: start.subs(old, new),
                        end: end.subs(old, new),
                    },
                };

                Expression::Calculus(Box::new(new_data))
            }

            // MethodCall - substitute in object and arguments
            Expression::MethodCall(data) => {
                let new_object = data.object.subs(old, new);
                let new_args: Vec<Expression> =
                    data.args.iter().map(|a| a.subs(old, new)).collect();

                Expression::MethodCall(Box::new(crate::core::expression::MethodCallData {
                    object: new_object,
                    method_name: data.method_name.clone(),
                    args: new_args,
                }))
            }
        };

        // Auto-simplify to match SymPy behavior
        result.simplify()
    }

    fn subs_multiple(&self, substitutions: &[(Expression, Expression)]) -> Expression {
        // If no substitutions, return self
        if substitutions.is_empty() {
            return self.clone();
        }

        // Check if this expression matches any of the old expressions
        for (old, new) in substitutions {
            if self == old {
                return new.clone();
            }
        }

        // Otherwise, recursively substitute in subexpressions
        match self {
            // Atomic types - no substitution needed
            Expression::Number(_) | Expression::Constant(_) | Expression::Symbol(_) => {
                self.clone()
            }

            // Add - substitute in each term
            Expression::Add(terms) => {
                let new_terms: Vec<Expression> = terms
                    .iter()
                    .map(|t| t.subs_multiple(substitutions))
                    .collect();
                Expression::Add(Box::new(new_terms))
            }

            // Multiply - substitute in each factor
            Expression::Mul(factors) => {
                let new_factors: Vec<Expression> = factors
                    .iter()
                    .map(|f| f.subs_multiple(substitutions))
                    .collect();
                Expression::Mul(Box::new(new_factors))
            }

            // Power - substitute in base and exponent
            Expression::Pow(base, exp) => {
                let new_base = base.subs_multiple(substitutions);
                let new_exp = exp.subs_multiple(substitutions);
                Expression::Pow(Box::new(new_base), Box::new(new_exp))
            }

            // Function - substitute in arguments
            Expression::Function { name, args } => {
                let new_args: Vec<Expression> = args
                    .iter()
                    .map(|a| a.subs_multiple(substitutions))
                    .collect();
                Expression::Function {
                    name: name.clone(),
                    args: Box::new(new_args),
                }
            }

            // Set - substitute in elements
            Expression::Set(elements) => {
                let new_elements: Vec<Expression> = elements
                    .iter()
                    .map(|e| e.subs_multiple(substitutions))
                    .collect();
                Expression::Set(Box::new(new_elements))
            }

            // Complex - substitute in real and imaginary parts
            Expression::Complex(data) => {
                let new_real = data.real.subs_multiple(substitutions);
                let new_imag = data.imag.subs_multiple(substitutions);
                Expression::Complex(Box::new(crate::core::expression::ComplexData {
                    real: new_real,
                    imag: new_imag,
                }))
            }

            // Matrix - substitute in each element
            Expression::Matrix(matrix) => {
                let (rows, cols) = matrix.dimensions();
                let mut new_data: Vec<Vec<Expression>> = Vec::with_capacity(rows);

                for i in 0..rows {
                    let mut row: Vec<Expression> = Vec::with_capacity(cols);
                    for j in 0..cols {
                        let elem = matrix.get_element(i, j);
                        row.push(elem.subs_multiple(substitutions));
                    }
                    new_data.push(row);
                }

                Expression::Matrix(Box::new(crate::matrix::unified::Matrix::dense(new_data)))
            }

            // Relation - substitute in both sides
            Expression::Relation(data) => {
                let new_left = data.left.subs_multiple(substitutions);
                let new_right = data.right.subs_multiple(substitutions);
                Expression::Relation(Box::new(crate::core::expression::RelationData {
                    left: new_left,
                    right: new_right,
                    relation_type: data.relation_type,
                }))
            }

            // Piecewise - substitute in pieces and default
            Expression::Piecewise(data) => {
                let new_pieces: Vec<(Expression, Expression)> = data
                    .pieces
                    .iter()
                    .map(|(expr, cond)| {
                        (
                            expr.subs_multiple(substitutions),
                            cond.subs_multiple(substitutions),
                        )
                    })
                    .collect();

                let new_default = data
                    .default
                    .as_ref()
                    .map(|d| d.subs_multiple(substitutions));

                Expression::Piecewise(Box::new(crate::core::expression::PiecewiseData {
                    pieces: new_pieces,
                    default: new_default,
                }))
            }

            // Interval - substitute in bounds
            Expression::Interval(data) => {
                let new_start = data.start.subs_multiple(substitutions);
                let new_end = data.end.subs_multiple(substitutions);
                Expression::Interval(Box::new(crate::core::expression::IntervalData {
                    start: new_start,
                    end: new_end,
                    start_inclusive: data.start_inclusive,
                    end_inclusive: data.end_inclusive,
                }))
            }

            // Calculus - substitute in calculus expressions
            Expression::Calculus(data) => {
                use crate::core::expression::CalculusData;

                let new_data = match data.as_ref() {
                    CalculusData::Derivative {
                        expression,
                        variable,
                        order,
                    } => CalculusData::Derivative {
                        expression: expression.subs_multiple(substitutions),
                        variable: variable.clone(),
                        order: *order,
                    },

                    CalculusData::Integral {
                        integrand,
                        variable,
                        bounds,
                    } => CalculusData::Integral {
                        integrand: integrand.subs_multiple(substitutions),
                        variable: variable.clone(),
                        bounds: bounds.as_ref().map(|(a, b)| {
                            (
                                a.subs_multiple(substitutions),
                                b.subs_multiple(substitutions),
                            )
                        }),
                    },

                    CalculusData::Limit {
                        expression,
                        variable,
                        point,
                        direction,
                    } => CalculusData::Limit {
                        expression: expression.subs_multiple(substitutions),
                        variable: variable.clone(),
                        point: point.subs_multiple(substitutions),
                        direction: *direction,
                    },

                    CalculusData::Sum {
                        expression,
                        variable,
                        start,
                        end,
                    } => CalculusData::Sum {
                        expression: expression.subs_multiple(substitutions),
                        variable: variable.clone(),
                        start: start.subs_multiple(substitutions),
                        end: end.subs_multiple(substitutions),
                    },

                    CalculusData::Product {
                        expression,
                        variable,
                        start,
                        end,
                    } => CalculusData::Product {
                        expression: expression.subs_multiple(substitutions),
                        variable: variable.clone(),
                        start: start.subs_multiple(substitutions),
                        end: end.subs_multiple(substitutions),
                    },
                };

                Expression::Calculus(Box::new(new_data))
            }

            // MethodCall - substitute in object and arguments
            Expression::MethodCall(data) => {
                let new_object = data.object.subs_multiple(substitutions);
                let new_args: Vec<Expression> = data
                    .args
                    .iter()
                    .map(|a| a.subs_multiple(substitutions))
                    .collect();

                Expression::MethodCall(Box::new(crate::core::expression::MethodCallData {
                    object: new_object,
                    method_name: data.method_name.clone(),
                    args: new_args,
                }))
            }
        }
        .simplify() // Auto-simplify to match SymPy behavior
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_basic_symbol_substitution() {
        let x = symbol!(x);
        let expr = Expression::symbol(x.clone());

        let result = expr.subs(&Expression::symbol(x.clone()), &Expression::integer(5));

        assert_eq!(result, Expression::integer(5));
    }

    #[test]
    fn test_substitution_in_addition() {
        let x = symbol!(x);
        let expr = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);

        let result = expr.subs(&Expression::symbol(x.clone()), &Expression::integer(5));

        // With auto-simplification, substitution now returns simplified result
        assert_eq!(result, Expression::integer(6));
    }

    #[test]
    fn test_substitution_in_multiplication() {
        let x = symbol!(x);
        let expr = Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);

        let result = expr.subs(&Expression::symbol(x.clone()), &Expression::integer(3));

        // With auto-simplification, substitution now returns simplified result
        assert_eq!(result, Expression::integer(6));
    }

    #[test]
    fn test_substitution_in_power() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

        let result = expr.subs(&Expression::symbol(x.clone()), &Expression::integer(3));

        // With auto-simplification, substitution now returns simplified result
        assert_eq!(result, Expression::integer(9));
    }

    #[test]
    fn test_substitution_in_function() {
        let x = symbol!(x);
        let expr = Expression::function("sin".to_string(), vec![Expression::symbol(x.clone())]);

        let result = expr.subs(&Expression::symbol(x.clone()), &Expression::integer(0));

        // With auto-simplification, sin(0) correctly evaluates to 0
        assert_eq!(result, Expression::integer(0));
    }

    #[test]
    fn test_nested_substitution() {
        let x = symbol!(x);
        // (x + 1) * (x - 1)
        let expr = Expression::mul(vec![
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::mul(vec![Expression::integer(-1), Expression::integer(1)]),
            ]),
        ]);

        let result = expr.subs(&Expression::symbol(x.clone()), &Expression::integer(2));

        // (2 + 1) * (2 - 1) = 3 * 1 = 3
        // With auto-simplification, substitution now returns simplified result
        assert_eq!(result, Expression::integer(3));
    }

    #[test]
    fn test_no_substitution_when_not_present() {
        let x = symbol!(x);
        let y = symbol!(y);
        let expr = Expression::symbol(y.clone());

        let result = expr.subs(&Expression::symbol(x.clone()), &Expression::integer(5));

        assert_eq!(result, Expression::symbol(y.clone()));
    }

    #[test]
    fn test_multiple_substitution_both_variables() {
        let x = symbol!(x);
        let y = symbol!(y);
        let expr = Expression::add(vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())]);

        let result = expr.subs_multiple(&[
            (Expression::symbol(x.clone()), Expression::integer(1)),
            (Expression::symbol(y.clone()), Expression::integer(2)),
        ]);

        // With auto-simplification, substitution now returns simplified result
        assert_eq!(result, Expression::integer(3));
    }

    #[test]
    fn test_multiple_substitution_in_complex_expr() {
        let x = symbol!(x);
        let y = symbol!(y);
        // x^2 + 2*x*y + y^2
        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::pow(Expression::symbol(y.clone()), Expression::integer(2)),
        ]);

        let result = expr.subs_multiple(&[
            (Expression::symbol(x.clone()), Expression::integer(3)),
            (Expression::symbol(y.clone()), Expression::integer(4)),
        ]);

        // 3^2 + 2*3*4 + 4^2 = 9 + 24 + 16 = 49
        // With auto-simplification, substitution now returns simplified result
        assert_eq!(result, Expression::integer(49));
    }

    #[test]
    fn test_substitution_doesnt_recurse_into_replacement() {
        let x = symbol!(x);
        let y = symbol!(y);
        let expr = Expression::symbol(x.clone());

        // Substitute x with y
        let result = expr.subs(&Expression::symbol(x.clone()), &Expression::symbol(y.clone()));

        assert_eq!(result, Expression::symbol(y.clone()));

        // Now substitute y - should not affect the result since we don't re-substitute
        let result2 = result.subs(&Expression::symbol(y.clone()), &Expression::integer(5));

        assert_eq!(result2, Expression::integer(5));
    }
}
