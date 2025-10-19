//! Core substitution trait and single-expression substitution implementation

use crate::core::Expression;
use crate::simplify::Simplify;

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
        if self == old {
            return new.clone();
        }

        let result = match self {
            Expression::Number(_) | Expression::Constant(_) => self.clone(),

            Expression::Symbol(_) => self.clone(),

            Expression::Add(terms) => {
                let new_terms: Vec<Expression> = terms.iter().map(|t| t.subs(old, new)).collect();
                Expression::Add(Box::new(new_terms))
            }

            Expression::Mul(factors) => {
                let new_factors: Vec<Expression> =
                    factors.iter().map(|f| f.subs(old, new)).collect();
                Expression::mul(new_factors)
            }

            Expression::Pow(base, exp) => {
                let new_base = base.subs(old, new);
                let new_exp = exp.subs(old, new);
                Expression::Pow(Box::new(new_base), Box::new(new_exp))
            }

            Expression::Function { name, args } => {
                let new_args: Vec<Expression> = args.iter().map(|a| a.subs(old, new)).collect();
                Expression::Function {
                    name: name.clone(),
                    args: Box::new(new_args),
                }
            }

            Expression::Set(elements) => {
                let new_elements: Vec<Expression> =
                    elements.iter().map(|e| e.subs(old, new)).collect();
                Expression::Set(Box::new(new_elements))
            }

            Expression::Complex(data) => {
                let new_real = data.real.subs(old, new);
                let new_imag = data.imag.subs(old, new);
                Expression::Complex(Box::new(crate::core::expression::ComplexData {
                    real: new_real,
                    imag: new_imag,
                }))
            }

            Expression::Matrix(matrix) => {
                let (rows, cols) = matrix.dimensions();
                let mut new_data: Vec<Vec<Expression>> = Vec::with_capacity(rows);

                for i in 0..rows {
                    let mut row: Vec<Expression> = Vec::with_capacity(cols);
                    for j in 0..cols {
                        let elem = matrix.get_element(i, j);
                        row.push(elem.subs(old, new));
                    }
                    new_data.push(row);
                }

                Expression::Matrix(Box::new(crate::matrix::unified::Matrix::dense(new_data)))
            }

            Expression::Relation(data) => {
                let new_left = data.left.subs(old, new);
                let new_right = data.right.subs(old, new);
                Expression::Relation(Box::new(crate::core::expression::RelationData {
                    left: new_left,
                    right: new_right,
                    relation_type: data.relation_type,
                }))
            }

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

        result.simplify()
    }

    fn subs_multiple(&self, substitutions: &[(Expression, Expression)]) -> Expression {
        super::rewrite::subs_multiple_impl(self, substitutions)
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

        assert_eq!(result, Expression::integer(6));
    }

    #[test]
    fn test_substitution_in_multiplication() {
        let x = symbol!(x);
        let expr = Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);

        let result = expr.subs(&Expression::symbol(x.clone()), &Expression::integer(3));

        assert_eq!(result, Expression::integer(6));
    }

    #[test]
    fn test_substitution_in_power() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

        let result = expr.subs(&Expression::symbol(x.clone()), &Expression::integer(3));

        assert_eq!(result, Expression::integer(9));
    }

    #[test]
    fn test_substitution_in_function() {
        let x = symbol!(x);
        let expr = Expression::function("sin".to_string(), vec![Expression::symbol(x.clone())]);

        let result = expr.subs(&Expression::symbol(x.clone()), &Expression::integer(0));

        assert_eq!(result, Expression::integer(0));
    }

    #[test]
    fn test_nested_substitution() {
        let x = symbol!(x);
        let expr = Expression::mul(vec![
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::mul(vec![Expression::integer(-1), Expression::integer(1)]),
            ]),
        ]);

        let result = expr.subs(&Expression::symbol(x.clone()), &Expression::integer(2));

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
    fn test_substitution_doesnt_recurse_into_replacement() {
        let x = symbol!(x);
        let y = symbol!(y);
        let expr = Expression::symbol(x.clone());

        let result = expr.subs(
            &Expression::symbol(x.clone()),
            &Expression::symbol(y.clone()),
        );

        assert_eq!(result, Expression::symbol(y.clone()));

        let result2 = result.subs(&Expression::symbol(y.clone()), &Expression::integer(5));

        assert_eq!(result2, Expression::integer(5));
    }

    #[test]
    fn test_substitution_preserves_position_matrices() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);
        let c = symbol!(C; matrix);

        let expr = Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
            Expression::symbol(a.clone()),
        ]);

        let result = expr.subs(&Expression::symbol(a.clone()), &Expression::symbol(c.clone()));

        let expected = Expression::mul(vec![
            Expression::symbol(c.clone()),
            Expression::symbol(b.clone()),
            Expression::symbol(c.clone()),
        ]);

        assert_eq!(
            result, expected,
            "Substitution A->C in ABA must preserve positions to get CBC"
        );
    }

    #[test]
    fn test_substitution_preserves_position_operators() {
        let p = symbol!(p; operator);
        let x = symbol!(x; operator);
        let h = symbol!(H; operator);

        let expr = Expression::mul(vec![
            Expression::symbol(p.clone()),
            Expression::symbol(x.clone()),
            Expression::symbol(p.clone()),
        ]);

        let result = expr.subs(&Expression::symbol(p.clone()), &Expression::symbol(h.clone()));

        let expected = Expression::mul(vec![
            Expression::symbol(h.clone()),
            Expression::symbol(x.clone()),
            Expression::symbol(h.clone()),
        ]);

        assert_eq!(
            result, expected,
            "Substitution p->H in pxp must preserve positions to get HxH"
        );
    }

    #[test]
    fn test_substitution_multiple_occurrences_different_positions() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);
        let c = symbol!(C; matrix);
        let d = symbol!(D; matrix);

        let expr = Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(b.clone()),
            Expression::symbol(c.clone()),
            Expression::symbol(a.clone()),
        ]).simplify();

        let result = expr.subs(&Expression::symbol(a.clone()), &Expression::symbol(d.clone()));

        let expected = Expression::mul(vec![
            Expression::symbol(d.clone()),
            Expression::symbol(b.clone()),
            Expression::symbol(c.clone()),
            Expression::symbol(d.clone()),
        ]).simplify();

        assert_eq!(
            result, expected,
            "Substitution A->D in ABCA must preserve all positions to get DBCD"
        );
    }

    #[test]
    fn test_substitution_quaternions_position_matters() {
        let i = symbol!(i; quaternion);
        let j = symbol!(j; quaternion);
        let k = symbol!(k; quaternion);

        let expr = Expression::mul(vec![
            Expression::symbol(i.clone()),
            Expression::symbol(j.clone()),
            Expression::symbol(i.clone()),
        ]);

        let result = expr.subs(&Expression::symbol(i.clone()), &Expression::symbol(k.clone()));

        let expected = Expression::mul(vec![
            Expression::symbol(k.clone()),
            Expression::symbol(j.clone()),
            Expression::symbol(k.clone()),
        ]);

        assert_eq!(
            result, expected,
            "Substitution i->k in iji must preserve positions to get kjk"
        );
    }

    #[test]
    fn test_substitution_scalars_commutative_still_preserves_structure() {
        let x = symbol!(x);
        let y = symbol!(y);
        let z = symbol!(z);

        let expr = Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
            Expression::symbol(x.clone()),
        ]);

        let result = expr.subs(&Expression::symbol(x.clone()), &Expression::symbol(z.clone()));

        let expected = Expression::mul(vec![
            Expression::symbol(z.clone()),
            Expression::symbol(y.clone()),
            Expression::symbol(z.clone()),
        ]);

        assert_eq!(
            result, expected,
            "Substitution x->z in xyx preserves structure even for commutative scalars"
        );
    }
}
