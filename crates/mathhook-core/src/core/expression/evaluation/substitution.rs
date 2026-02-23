//! Variable substitution for expressions
//!
//! Contains the `substitute()` method for replacing symbols with expressions.

use super::super::Expression;
use crate::simplify::Simplify;
use std::collections::HashMap;
use std::sync::Arc;

impl Expression {
    /// Substitute variables with expressions
    ///
    /// Recursively replaces all occurrences of symbols with provided expressions.
    ///
    /// # Arguments
    ///
    /// * `substitutions` - Map from symbol name to replacement expression
    ///
    /// # Returns
    ///
    /// New expression with substitutions applied
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use mathhook_core::{expr, symbol};
    /// use std::collections::HashMap;
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let e = expr!(x + y);
    ///
    /// let mut subs = HashMap::new();
    /// subs.insert("x".to_string(), expr!(3));
    /// subs.insert("y".to_string(), expr!(4));
    ///
    /// let result = e.substitute(&subs);
    /// assert_eq!(result, expr!(3 + 4));
    /// ```
    pub fn substitute(&self, substitutions: &HashMap<String, Expression>) -> Expression {
        match self {
            Expression::Number(_) | Expression::Constant(_) => self.clone(),

            Expression::Symbol(sym) => substitutions
                .get(sym.name())
                .cloned()
                .unwrap_or_else(|| self.clone()),

            Expression::Add(terms) => {
                let new_terms: Vec<Expression> =
                    terms.iter().map(|t| t.substitute(substitutions)).collect();
                Expression::add(new_terms)
            }

            Expression::Mul(factors) => {
                let new_factors: Vec<Expression> = factors
                    .iter()
                    .map(|f| f.substitute(substitutions))
                    .collect();
                Expression::mul(new_factors)
            }

            Expression::Pow(base, exp) => {
                let new_base = base.substitute(substitutions);
                let new_exp = exp.substitute(substitutions);
                Expression::pow(new_base, new_exp)
            }

            Expression::Function { name, args } => {
                let new_args: Vec<Expression> = args
                    .iter()
                    .map(|arg| arg.substitute(substitutions))
                    .collect();
                Expression::function(name.clone(), new_args)
            }

            Expression::Set(elements) => {
                let new_elements: Vec<Expression> = elements
                    .iter()
                    .map(|e| e.substitute(substitutions))
                    .collect();
                Expression::set(new_elements)
            }

            Expression::Complex(data) => {
                let new_real = data.real.substitute(substitutions);
                let new_imag = data.imag.substitute(substitutions);
                Expression::complex(new_real, new_imag)
            }

            Expression::Relation(data) => {
                let new_left = data.left.substitute(substitutions);
                let new_right = data.right.substitute(substitutions);
                Expression::relation(new_left, new_right, data.relation_type)
            }

            Expression::Piecewise(data) => {
                let new_pieces: Vec<(Expression, Expression)> = data
                    .pieces
                    .iter()
                    .map(|(expr, cond)| {
                        (
                            expr.substitute(substitutions),
                            cond.substitute(substitutions),
                        )
                    })
                    .collect();
                let new_default = data.default.as_ref().map(|d| d.substitute(substitutions));
                Expression::piecewise(new_pieces, new_default)
            }

            Expression::Interval(data) => {
                let new_start = data.start.substitute(substitutions);
                let new_end = data.end.substitute(substitutions);
                Expression::interval(new_start, new_end, data.start_inclusive, data.end_inclusive)
            }

            Expression::Calculus(data) => {
                use crate::core::expression::CalculusData;
                let new_data = match data.as_ref() {
                    CalculusData::Derivative {
                        expression,
                        variable,
                        order,
                    } => CalculusData::Derivative {
                        expression: expression.substitute(substitutions),
                        variable: variable.clone(),
                        order: *order,
                    },
                    CalculusData::Integral {
                        integrand,
                        variable,
                        bounds,
                    } => CalculusData::Integral {
                        integrand: integrand.substitute(substitutions),
                        variable: variable.clone(),
                        bounds: bounds.as_ref().map(|(lower, upper)| {
                            (
                                lower.substitute(substitutions),
                                upper.substitute(substitutions),
                            )
                        }),
                    },
                    CalculusData::Limit {
                        expression,
                        variable,
                        point,
                        direction,
                    } => CalculusData::Limit {
                        expression: expression.substitute(substitutions),
                        variable: variable.clone(),
                        point: point.substitute(substitutions),
                        direction: *direction,
                    },
                    CalculusData::Sum {
                        expression,
                        variable,
                        start,
                        end,
                    } => CalculusData::Sum {
                        expression: expression.substitute(substitutions),
                        variable: variable.clone(),
                        start: start.substitute(substitutions),
                        end: end.substitute(substitutions),
                    },
                    CalculusData::Product {
                        expression,
                        variable,
                        start,
                        end,
                    } => CalculusData::Product {
                        expression: expression.substitute(substitutions),
                        variable: variable.clone(),
                        start: start.substitute(substitutions),
                        end: end.substitute(substitutions),
                    },
                };
                Expression::Calculus(Arc::new(new_data))
            }

            Expression::MethodCall(data) => {
                let new_object = data.object.substitute(substitutions);
                let new_args: Vec<Expression> = data
                    .args
                    .iter()
                    .map(|arg| arg.substitute(substitutions))
                    .collect();
                Expression::method_call(new_object, data.method_name.clone(), new_args)
            }

            Expression::Matrix(m) => {
                if m.is_symmetric() {
                    let size = m.dimensions().0;
                    for i in 0..size {
                        for j in 0..i{
                            m.get_element(i, j).substitute(substitutions);
                        }
                    }
                    Expression::Matrix(m.clone())
                } else {
                    let (i_bound, j_bound) = m.dimensions();
                    for i in 0..i_bound {
                        for j in 0..j_bound {
                            m.get_element(i, j).substitute(substitutions);
                        }
                    }
                    Expression::Matrix(m.clone())
                }
            },
        }
    }

    /// Substitute and simplify in one step
    ///
    /// Convenience method that applies substitutions and then simplifies the result.
    ///
    /// # Arguments
    ///
    /// * `substitutions` - Map from symbol name to replacement expression
    ///
    /// # Returns
    ///
    /// New simplified expression with substitutions applied
    pub fn substitute_and_simplify(
        &self,
        substitutions: &HashMap<String, Expression>,
    ) -> Expression {
        self.substitute(substitutions).simplify()
    }
}
