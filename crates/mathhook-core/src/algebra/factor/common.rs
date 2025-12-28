//! Common factor extraction and GCD operations

use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};

impl Expression {
    /// Find common factor in a list of terms
    pub(super) fn find_common_factor_in_terms(&self, terms: &[Expression]) -> Expression {
        if terms.is_empty() {
            return Expression::integer(1);
        }

        let mut common = self.extract_factors(&terms[0]);

        for term in &terms[1..] {
            let term_factors = self.extract_factors(term);
            common = self.intersect_factors(&common, &term_factors);

            if common.is_empty() {
                return Expression::integer(1);
            }
        }

        if common.is_empty() {
            Expression::integer(1)
        } else {
            Expression::mul(common)
        }
    }

    /// Extract factors from an expression
    pub(super) fn extract_factors(&self, expr: &Expression) -> Vec<Expression> {
        match expr {
            Expression::Number(Number::Integer(n)) => {
                if !n.is_zero() && !n.is_one() {
                    vec![expr.clone()]
                } else {
                    vec![]
                }
            }
            Expression::Symbol(_) => vec![expr.clone()],
            Expression::Mul(factors) => (**factors).clone(),
            Expression::Pow(base, _exp) => vec![(**base).clone()],
            _ => vec![expr.clone()],
        }
    }

    /// Find intersection of two factor lists
    pub(super) fn intersect_factors(
        &self,
        factors1: &[Expression],
        factors2: &[Expression],
    ) -> Vec<Expression> {
        let mut common = Vec::new();

        for factor1 in factors1 {
            if factors2.contains(factor1) {
                common.push(factor1.clone());
            }
        }

        let num1 = self.extract_numeric_factor(factors1);
        let num2 = self.extract_numeric_factor(factors2);

        if let (Some(n1), Some(n2)) = (num1, num2) {
            let gcd_num = n1.gcd(&n2);
            if !gcd_num.is_one() {
                common.push(Expression::big_integer(gcd_num));
            }
        }

        common
    }

    /// Extract numeric factor from factor list
    pub(super) fn extract_numeric_factor(&self, factors: &[Expression]) -> Option<BigInt> {
        for factor in factors {
            if let Expression::Number(Number::Integer(n)) = factor {
                return Some(BigInt::from(*n));
            }
        }
        None
    }

    /// Divide expression by a factor (simplified division)
    pub(super) fn divide_by_factor(&self, expr: &Expression, factor: &Expression) -> Expression {
        match (expr, factor) {
            (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(b))) => {
                if !b.is_zero() && (a % b).is_zero() {
                    Expression::integer(a / b)
                } else {
                    expr.clone()
                }
            }

            (Expression::Symbol(s1), Expression::Symbol(s2)) if s1 == s2 => Expression::integer(1),

            (Expression::Mul(factors), _) => {
                let mut remaining_factors = factors.as_ref().clone();
                if let Some(pos) = remaining_factors.iter().position(|f| f == factor) {
                    remaining_factors.remove(pos);
                    if remaining_factors.is_empty() {
                        Expression::integer(1)
                    } else if remaining_factors.len() == 1 {
                        remaining_factors[0].clone()
                    } else {
                        Expression::mul(remaining_factors)
                    }
                } else {
                    expr.clone()
                }
            }

            _ => expr.clone(),
        }
    }

    /// Factor out numeric coefficients
    pub fn factor_numeric_coefficient(&self) -> (BigInt, Expression) {
        match self {
            Expression::Number(Number::Integer(n)) => (BigInt::from(*n), Expression::integer(1)),
            Expression::Number(Number::BigInteger(n)) => {
                (n.as_ref().clone(), Expression::integer(1))
            }
            Expression::Mul(factors) => {
                let mut coefficient = BigInt::one();
                let mut non_numeric_factors = Vec::new();

                for factor in factors.iter() {
                    match factor {
                        Expression::Number(Number::Integer(n)) => {
                            coefficient *= BigInt::from(*n);
                        }
                        Expression::Number(Number::BigInteger(n)) => {
                            coefficient *= n.as_ref();
                        }
                        _ => {
                            non_numeric_factors.push(factor.clone());
                        }
                    }
                }

                let remaining = if non_numeric_factors.is_empty() {
                    Expression::integer(1)
                } else if non_numeric_factors.len() == 1 {
                    non_numeric_factors[0].clone()
                } else {
                    Expression::mul(non_numeric_factors)
                };

                (coefficient, remaining)
            }
            _ => (BigInt::one(), self.clone()),
        }
    }
}
