//! Coefficient extraction operations for term collection

use crate::core::{Expression, Number, Symbol};
use crate::expr;
use num_bigint::BigInt;
use num_traits::{One, Zero};

impl Expression {
    /// Extract coefficient and power from a term with respect to a variable
    pub(super) fn extract_coefficient_and_power(
        &self,
        term: &Expression,
        var: &Symbol,
    ) -> (BigInt, Expression) {
        match term {
            Expression::Number(Number::Integer(n)) => (BigInt::from(*n), Expression::integer(0)),

            Expression::Symbol(s) if s == var => (BigInt::one(), Expression::integer(1)),

            Expression::Symbol(_) => (BigInt::zero(), Expression::integer(0)),

            Expression::Pow(base, exp) => {
                if let Expression::Symbol(s) = base.as_ref() {
                    if s == var {
                        return (BigInt::one(), exp.as_ref().clone());
                    }
                }
                (BigInt::zero(), Expression::integer(0))
            }

            Expression::Mul(factors) => {
                let mut coefficient = BigInt::one();
                let mut power = expr!(0);
                let mut has_var = false;

                for factor in factors.iter() {
                    match factor {
                        Expression::Number(Number::Integer(n)) => {
                            coefficient *= BigInt::from(*n);
                        }
                        Expression::Symbol(s) if s == var => {
                            power = expr!(1);
                            has_var = true;
                        }
                        Expression::Pow(base, exp) => {
                            if let Expression::Symbol(s) = base.as_ref() {
                                if s == var {
                                    power = exp.as_ref().clone();
                                    has_var = true;
                                }
                            }
                        }
                        _ => {}
                    }
                }

                if has_var {
                    (coefficient, power)
                } else {
                    (BigInt::zero(), expr!(0))
                }
            }

            _ => (BigInt::zero(), expr!(0)),
        }
    }

    /// Extract coefficient and base term from any expression
    pub(super) fn extract_coefficient_and_base(&self, expr: &Expression) -> (BigInt, Expression) {
        match expr {
            Expression::Number(Number::Integer(n)) => (BigInt::from(*n), expr!(1)),

            Expression::Symbol(_) => (BigInt::one(), expr.clone()),

            Expression::Mul(factors) => {
                let mut coefficient = BigInt::one();
                let mut non_numeric_factors = Vec::new();

                for factor in factors.iter() {
                    if let Expression::Number(Number::Integer(n)) = factor {
                        coefficient *= BigInt::from(*n);
                    } else {
                        non_numeric_factors.push(factor.clone());
                    }
                }

                let base = if non_numeric_factors.is_empty() {
                    expr!(1)
                } else if non_numeric_factors.len() == 1 {
                    non_numeric_factors[0].clone()
                } else {
                    Expression::mul(non_numeric_factors)
                };

                (coefficient, base)
            }

            _ => (BigInt::one(), expr.clone()),
        }
    }

    /// Check if an expression is constant (contains no variables)
    pub(super) fn is_constant(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Number(_) => true,
            Expression::Symbol(_) => false,
            Expression::Add(terms) | Expression::Mul(terms) => {
                terms.iter().all(|t| self.is_constant(t))
            }
            Expression::Pow(base, exp) => self.is_constant(base) && self.is_constant(exp),
            Expression::Function { args, .. } => args.iter().all(|a| self.is_constant(a)),
            Expression::Complex(_) => true,
            Expression::Matrix(_) => false,
            Expression::Constant(_) => true,
            Expression::Relation(_) => false,
            Expression::Piecewise(_) => false,
            Expression::Set(_) => false,
            Expression::Interval(_) => true,
            Expression::Calculus(_) => false,
            Expression::MethodCall(method_data) => {
                self.is_constant(&method_data.object)
                    && method_data.args.iter().all(|a| self.is_constant(a))
            }
        }
    }

    /// Check if two expressions have the same factor order
    ///
    /// For noncommutative terms, AB and BA are DIFFERENT and should NOT be combined
    pub(super) fn same_factor_order(&self, expr1: &Expression, expr2: &Expression) -> bool {
        match (expr1, expr2) {
            (Expression::Mul(factors1), Expression::Mul(factors2)) => {
                if factors1.len() != factors2.len() {
                    return false;
                }
                factors1.iter().zip(factors2.iter()).all(|(f1, f2)| f1 == f2)
            }
            _ => expr1 == expr2,
        }
    }
}
