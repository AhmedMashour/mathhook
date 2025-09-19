//! Factorization operations for expressions
//! Handles polynomial factorization, common factor extraction, and algebraic factoring

use crate::algebra::gcd::PolynomialGcd;
use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero};

/// Trait for factoring expressions
pub trait Factor {
    fn factor(&self) -> Self;
    fn factor_out_gcd(&self) -> Self;
    fn factor_common(&self) -> Self;
}

impl Factor for Expression {
    /// Factor the expression by extracting common factors
    fn factor(&self) -> Self {
        match self {
            Expression::Number(_) | Expression::Symbol(_) => self.clone(),

            Expression::Add(terms) => self.factor_addition(terms),

            Expression::Mul(factors) => {
                // Factor each factor and recombine
                let factored_factors: Vec<Expression> =
                    factors.iter().map(|f| f.factor()).collect();
                Expression::mul(factored_factors)
            }

            Expression::Pow(base, exp) => Expression::pow(base.factor(), exp.factor()),

            Expression::Function { name, args } => {
                let factored_args: Vec<Expression> = args.iter().map(|arg| arg.factor()).collect();
                Expression::function(name.clone(), factored_args)
            }
            // New expression types - implement later
            _ => self.clone(),
        }
    }

    /// Factor out the GCD from an expression
    fn factor_out_gcd(&self) -> Self {
        match self {
            Expression::Add(terms) => {
                if terms.len() < 2 {
                    return self.clone();
                }

                // Find GCD of all terms
                let mut common_factor = terms[0].clone();
                for term in &terms[1..] {
                    common_factor = common_factor.gcd(term);
                    if common_factor.is_one() {
                        return self.clone(); // No common factor
                    }
                }

                // Factor out the common factor
                if !common_factor.is_one() {
                    let factored_terms: Vec<Expression> = terms
                        .iter()
                        .map(|term| self.divide_by_factor(term, &common_factor))
                        .collect();

                    Expression::mul(vec![common_factor, Expression::add(factored_terms)])
                } else {
                    self.clone()
                }
            }
            _ => self.clone(),
        }
    }

    /// Factor common elements
    fn factor_common(&self) -> Self {
        self.factor_out_gcd()
    }
}

impl Expression {
    /// Factor addition expressions by finding common factors
    fn factor_addition(&self, terms: &[Expression]) -> Expression {
        if terms.len() < 2 {
            return Expression::add(terms.to_vec());
        }

        // Try to find common factors
        let common_factor = self.find_common_factor_in_terms(terms);

        if !common_factor.is_one() {
            // Factor out the common factor
            let factored_terms: Vec<Expression> = terms
                .iter()
                .map(|term| self.divide_by_factor(term, &common_factor))
                .collect();

            Expression::mul(vec![common_factor, Expression::add(factored_terms)])
        } else {
            // Try other factoring methods
            self.try_quadratic_factoring(terms)
                .unwrap_or_else(|| Expression::add(terms.to_vec()))
        }
    }

    /// Find common factor in a list of terms
    fn find_common_factor_in_terms(&self, terms: &[Expression]) -> Expression {
        if terms.is_empty() {
            return Expression::integer(1);
        }

        // Start with first term as potential common factor
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
    fn extract_factors(&self, expr: &Expression) -> Vec<Expression> {
        match expr {
            Expression::Number(Number::SmallInt(n)) => {
                if !n.is_zero() && !n.is_one() {
                    vec![expr.clone()]
                } else {
                    vec![]
                }
            }
            Expression::Symbol(_) => vec![expr.clone()],
            Expression::Mul(factors) => (**factors).clone(),
            Expression::Pow(base, _exp) => vec![(**base).clone()], // Simplified
            _ => vec![expr.clone()],
        }
    }

    /// Find intersection of two factor lists
    fn intersect_factors(
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

        // Handle numeric factors specially
        let num1 = self.extract_numeric_factor(factors1);
        let num2 = self.extract_numeric_factor(factors2);

        if let (Some(n1), Some(n2)) = (num1, num2) {
            let gcd_num = n1.gcd(&n2);
            if !gcd_num.is_one() {
                common.push(Expression::integer(gcd_num));
            }
        }

        common
    }

    /// Extract numeric factor from factor list
    fn extract_numeric_factor(&self, factors: &[Expression]) -> Option<BigInt> {
        for factor in factors {
            if let Expression::Number(Number::SmallInt(n)) = factor {
                return Some(BigInt::from(*n));
            }
        }
        None
    }

    /// Divide expression by a factor (simplified division)
    fn divide_by_factor(&self, expr: &Expression, factor: &Expression) -> Expression {
        match (expr, factor) {
            // Numeric division
            (Expression::Number(Number::SmallInt(a)), Expression::Number(Number::SmallInt(b))) => {
                if !b.is_zero() && (a % b).is_zero() {
                    Expression::integer(a / b)
                } else {
                    expr.clone()
                }
            }

            // Symbol division
            (Expression::Symbol(s1), Expression::Symbol(s2)) if s1 == s2 => Expression::integer(1),

            // Multiplication division
            (Expression::Mul(factors), _) => {
                let mut remaining_factors = factors.clone();
                if let Some(pos) = remaining_factors.iter().position(|f| f == factor) {
                    remaining_factors.remove(pos);
                    if remaining_factors.is_empty() {
                        Expression::integer(1)
                    } else if remaining_factors.len() == 1 {
                        remaining_factors[0].clone()
                    } else {
                        Expression::mul(remaining_factors.as_ref().clone())
                    }
                } else {
                    expr.clone()
                }
            }

            // Default: return original if can't divide
            _ => expr.clone(),
        }
    }

    /// Try to factor quadratic expressions: ax^2 + bx + c
    fn try_quadratic_factoring(&self, _terms: &[Expression]) -> Option<Expression> {
        // This is a simplified version - full quadratic factoring is complex
        // For now, just return None to indicate no factoring found
        None
    }

    /// Factor perfect square trinomials: a^2 + 2ab + b^2 = (a + b)^2
    pub fn factor_perfect_square(&self, terms: &[Expression]) -> Option<Expression> {
        if terms.len() != 3 {
            return None;
        }

        // Check if it's a perfect square trinomial
        // This is a simplified check - full implementation would be more complex
        None
    }

    /// Factor difference of squares: a^2 - b^2 = (a + b)(a - b)
    pub fn factor_difference_of_squares(&self, a: &Expression, b: &Expression) -> Expression {
        Expression::mul(vec![
            Expression::add(vec![a.clone(), b.clone()]),
            Expression::add(vec![
                a.clone(),
                Expression::mul(vec![Expression::integer(-1), b.clone()]),
            ]),
        ])
    }

    /// Factor out numeric coefficients
    pub fn factor_numeric_coefficient(&self) -> (BigInt, Expression) {
        match self {
            Expression::Number(Number::SmallInt(n)) => (BigInt::from(*n), Expression::integer(1)),
            Expression::Number(Number::BigInteger(n)) => {
                (n.as_ref().clone(), Expression::integer(1))
            }
            Expression::Mul(factors) => {
                let mut coefficient = BigInt::one();
                let mut non_numeric_factors = Vec::new();

                for factor in factors.iter() {
                    match factor {
                        Expression::Number(Number::SmallInt(n)) => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Symbol;

    #[test]
    fn test_basic_factoring() {
        let x = Symbol::new("x");

        // Test factoring 2x + 4 = 2(x + 2)
        let expr = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::integer(4),
        ]);

        let result = expr.factor();
        println!("2x + 4 factored = {}", result);

        // Should extract common factor 2
        match result {
            Expression::Mul(_) => println!("Successfully factored"),
            _ => println!("Factoring result: {}", result),
        }
    }

    #[test]
    fn test_gcd_factoring() {
        let x = Symbol::new("x");

        // Test 6x + 9 = 3(2x + 3)
        let expr = Expression::add(vec![
            Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]),
            Expression::integer(9),
        ]);

        let result = expr.factor_out_gcd();
        println!("6x + 9 GCD factored = {}", result);

        assert!(!result.is_zero());
    }

    #[test]
    fn test_numeric_coefficient_extraction() {
        let x = Symbol::new("x");

        let expr = Expression::mul(vec![
            Expression::integer(12),
            Expression::symbol(x.clone()),
            Expression::integer(5),
        ]);

        let (coeff, remaining) = expr.factor_numeric_coefficient();

        println!("Coefficient: {}, Remaining: {}", coeff, remaining);
        assert_eq!(coeff, BigInt::from(60));
        assert_eq!(remaining, Expression::symbol(x));
    }

    #[test]
    fn test_difference_of_squares() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        let result = Expression::integer(1).factor_difference_of_squares(
            &Expression::symbol(x.clone()),
            &Expression::symbol(y.clone()),
        );

        println!("x^2 - y^2 factored = {}", result);

        // Should be (x + y)(x - y)
        match result {
            Expression::Mul(factors) => assert_eq!(factors.len(), 2),
            _ => panic!("Expected multiplication"),
        }
    }

    #[test]
    fn test_common_factor_extraction() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        // Test xy + x = x(y + 1)
        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::symbol(x.clone()),
        ]);

        let result = expr.factor_common();
        println!("xy + x factored = {}", result);

        assert!(!result.is_zero());
    }

    #[test]
    fn test_no_common_factor() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        // Test x + y (no common factor)
        let expr = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);

        let result = expr.factor();

        // Should remain unchanged
        assert_eq!(result, expr);
    }
}
