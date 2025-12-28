//! Multiple-expression substitution and rewriting logic

use crate::core::Expression;
use crate::simplify::Simplify;
use std::sync::Arc;

/// Implementation for multiple simultaneous substitutions
pub(super) fn subs_multiple_impl(
    expr: &Expression,
    substitutions: &[(Expression, Expression)],
) -> Expression {
    if substitutions.is_empty() {
        return expr.clone();
    }

    for (old, new) in substitutions {
        if expr == old {
            return new.clone();
        }
    }

    match expr {
        Expression::Number(_) | Expression::Constant(_) | Expression::Symbol(_) => expr.clone(),

        Expression::Add(terms) => {
            let new_terms: Vec<Expression> = terms
                .iter()
                .map(|t| subs_multiple_impl(t, substitutions))
                .collect();
            Expression::Add(Arc::new(new_terms))
        }

        Expression::Mul(factors) => {
            let new_factors: Vec<Expression> = factors
                .iter()
                .map(|f| subs_multiple_impl(f, substitutions))
                .collect();
            Expression::Mul(Arc::new(new_factors))
        }

        Expression::Pow(base, exp) => {
            let new_base = subs_multiple_impl(base, substitutions);
            let new_exp = subs_multiple_impl(exp, substitutions);
            Expression::Pow(Arc::new(new_base), Arc::new(new_exp))
        }

        Expression::Function { name, args } => {
            let new_args: Vec<Expression> = args
                .iter()
                .map(|a| subs_multiple_impl(a, substitutions))
                .collect();
            Expression::Function {
                name: name.clone(),
                args: Arc::new(new_args),
            }
        }

        Expression::Set(elements) => {
            let new_elements: Vec<Expression> = elements
                .iter()
                .map(|e| subs_multiple_impl(e, substitutions))
                .collect();
            Expression::Set(Arc::new(new_elements))
        }

        Expression::Complex(data) => {
            let new_real = subs_multiple_impl(&data.real, substitutions);
            let new_imag = subs_multiple_impl(&data.imag, substitutions);
            Expression::Complex(Arc::new(crate::core::expression::ComplexData {
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
                    row.push(subs_multiple_impl(&elem, substitutions));
                }
                new_data.push(row);
            }

            Expression::Matrix(Arc::new(crate::matrices::unified::Matrix::dense(new_data)))
        }

        Expression::Relation(data) => {
            let new_left = subs_multiple_impl(&data.left, substitutions);
            let new_right = subs_multiple_impl(&data.right, substitutions);
            Expression::Relation(Arc::new(crate::core::expression::RelationData {
                left: new_left,
                right: new_right,
                relation_type: data.relation_type,
            }))
        }

        Expression::Piecewise(data) => {
            let new_pieces: Vec<(Expression, Expression)> = data
                .pieces
                .iter()
                .map(|(expr, cond)| {
                    (
                        subs_multiple_impl(expr, substitutions),
                        subs_multiple_impl(cond, substitutions),
                    )
                })
                .collect();

            let new_default = data
                .default
                .as_ref()
                .map(|d| subs_multiple_impl(d, substitutions));

            Expression::Piecewise(Arc::new(crate::core::expression::PiecewiseData {
                pieces: new_pieces,
                default: new_default,
            }))
        }

        Expression::Interval(data) => {
            let new_start = subs_multiple_impl(&data.start, substitutions);
            let new_end = subs_multiple_impl(&data.end, substitutions);
            Expression::Interval(Arc::new(crate::core::expression::IntervalData {
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
                    expression: subs_multiple_impl(expression, substitutions),
                    variable: variable.clone(),
                    order: *order,
                },

                CalculusData::Integral {
                    integrand,
                    variable,
                    bounds,
                } => CalculusData::Integral {
                    integrand: subs_multiple_impl(integrand, substitutions),
                    variable: variable.clone(),
                    bounds: bounds.as_ref().map(|(a, b)| {
                        (
                            subs_multiple_impl(a, substitutions),
                            subs_multiple_impl(b, substitutions),
                        )
                    }),
                },

                CalculusData::Limit {
                    expression,
                    variable,
                    point,
                    direction,
                } => CalculusData::Limit {
                    expression: subs_multiple_impl(expression, substitutions),
                    variable: variable.clone(),
                    point: subs_multiple_impl(point, substitutions),
                    direction: *direction,
                },

                CalculusData::Sum {
                    expression,
                    variable,
                    start,
                    end,
                } => CalculusData::Sum {
                    expression: subs_multiple_impl(expression, substitutions),
                    variable: variable.clone(),
                    start: subs_multiple_impl(start, substitutions),
                    end: subs_multiple_impl(end, substitutions),
                },

                CalculusData::Product {
                    expression,
                    variable,
                    start,
                    end,
                } => CalculusData::Product {
                    expression: subs_multiple_impl(expression, substitutions),
                    variable: variable.clone(),
                    start: subs_multiple_impl(start, substitutions),
                    end: subs_multiple_impl(end, substitutions),
                },
            };

            Expression::Calculus(Arc::new(new_data))
        }

        Expression::MethodCall(data) => {
            let new_object = subs_multiple_impl(&data.object, substitutions);
            let new_args: Vec<Expression> = data
                .args
                .iter()
                .map(|a| subs_multiple_impl(a, substitutions))
                .collect();

            Expression::MethodCall(Arc::new(crate::core::expression::MethodCallData {
                object: new_object,
                method_name: data.method_name.clone(),
                args: new_args,
            }))
        }
    }
    .simplify()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::Substitutable;
    use crate::prelude::*;

    #[test]
    fn test_multiple_substitution_both_variables() {
        let x = symbol!(x);
        let y = symbol!(y);
        let expr = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);

        let result = expr.subs_multiple(&[
            (Expression::symbol(x.clone()), Expression::integer(1)),
            (Expression::symbol(y.clone()), Expression::integer(2)),
        ]);

        assert_eq!(result, Expression::integer(3));
    }

    #[test]
    fn test_multiple_substitution_in_complex_expr() {
        let x = symbol!(x);
        let y = symbol!(y);
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

        assert_eq!(result, Expression::integer(49));
    }
}
