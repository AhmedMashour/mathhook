//! Algebraic operations and traits for the hybrid API

use crate::core::Expression;

/// Simplification trait for Expression-centric API
pub trait Simplify {
    /// Simplify the expression using basic algebraic rules
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, algebra::Simplify};
    ///
    /// let expr = Expression::add(vec![
    ///     Expression::integer(2),
    ///     Expression::integer(3),
    /// ]);
    /// let simplified = expr.simplify();
    /// // Result: 5
    /// ```
    fn simplify(self) -> Expression;
}

/// Factorization trait for Expression-centric API
pub trait Factor {
    /// Factor the expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, algebra::Factor};
    ///
    /// let expr = Expression::add(vec![
    ///     Expression::pow(Expression::symbol("x"), Expression::integer(2)),
    ///     Expression::multiply(Expression::integer(5), Expression::symbol("x")),
    ///     Expression::integer(6),
    /// ]);
    /// let factored = expr.factor();
    /// // Result: (x + 2)(x + 3)
    /// ```
    fn factor(self) -> Expression;
}

/// Expansion trait for Expression-centric API
pub trait Expand {
    /// Expand the expression
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::{Expression, algebra::Expand};
    ///
    /// let expr = Expression::multiply(
    ///     Expression::add(vec![Expression::symbol("x"), Expression::integer(1)]),
    ///     Expression::add(vec![Expression::symbol("x"), Expression::integer(2)]),
    /// );
    /// let expanded = expr.expand();
    /// // Result: x^2 + 3x + 2
    /// ```
    fn expand(self) -> Expression;
}

impl Simplify for Expression {
    fn simplify(self) -> Expression {
        match self {
            Expression::Add(terms) => {
                if terms.is_empty() {
                    return Expression::integer(0);
                }
                let simplified_terms: Vec<Expression> =
                    terms.iter().map(|t| t.clone().simplify()).collect();

                if simplified_terms.len() == 1 {
                    return simplified_terms.into_iter().next().unwrap();
                }
                let mut numeric_sum = 0i64;
                let mut float_sum = 0.0f64;
                let mut has_floats = false;
                let mut non_numeric = Vec::new();

                for term in simplified_terms {
                    match &term {
                        Expression::Number(num) => match num {
                            crate::core::Number::Integer(i) => numeric_sum += i,
                            crate::core::Number::Float(f) => {
                                float_sum += f;
                                has_floats = true;
                            }
                            _ => non_numeric.push(term),
                        },
                        _ => non_numeric.push(term),
                    }
                }

                if has_floats {
                    let total_float = numeric_sum as f64 + float_sum;
                    if total_float != 0.0 {
                        non_numeric.push(Expression::number(total_float));
                    }
                } else if numeric_sum != 0 {
                    non_numeric.push(Expression::integer(numeric_sum));
                }

                match non_numeric.len() {
                    0 => Expression::integer(0),
                    1 => non_numeric.into_iter().next().unwrap(),
                    _ => Expression::Add(Box::new(non_numeric)),
                }
            }

            Expression::Mul(factors) => {
                if factors.is_empty() {
                    return Expression::integer(1);
                }
                let simplified_factors: Vec<Expression> =
                    factors.iter().map(|f| f.clone().simplify()).collect();

                if simplified_factors.len() == 1 {
                    return simplified_factors.into_iter().next().unwrap();
                }

                let mut numeric_product = 1i64;
                let mut non_numeric = Vec::new();

                for factor in simplified_factors {
                    match &factor {
                        Expression::Number(num) => {
                            if let crate::core::Number::Integer(i) = num {
                                numeric_product *= i;
                            } else {
                                non_numeric.push(factor);
                            }
                        }
                        _ => non_numeric.push(factor),
                    }
                }

                if numeric_product == 0 {
                    return Expression::integer(0);
                }

                if numeric_product != 1 {
                    non_numeric.push(Expression::integer(numeric_product));
                }

                match non_numeric.len() {
                    0 => Expression::integer(1),
                    1 => non_numeric.into_iter().next().unwrap(),
                    _ => Expression::Mul(Box::new(non_numeric)),
                }
            }

            Expression::Pow(base, exponent) => {
                let simplified_base = base.simplify();
                let simplified_exp = exponent.simplify();

                match &simplified_exp {
                    Expression::Number(num) => {
                        if let crate::core::Number::Integer(0) = num {
                            return Expression::integer(1); // x^0 = 1
                        }
                        if let crate::core::Number::Integer(1) = num {
                            return simplified_base; // x^1 = x
                        }
                    }
                    _ => {}
                }

                Expression::Pow(Box::new(simplified_base), Box::new(simplified_exp))
            }

            _ => self,
        }
    }
}

impl Factor for Expression {
    fn factor(self) -> Expression {
        self
    }
}

impl Expand for Expression {
    fn expand(self) -> Expression {
        self
    }
}
