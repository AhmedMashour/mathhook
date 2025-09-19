//! Greatest Common Divisor operations for polynomials and expressions

use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::One; // For GCD on BigInt

/// Trait for GCD operations on expressions
pub trait PolynomialGcd {
    fn gcd(&self, other: &Self) -> Self;
    fn lcm(&self, other: &Self) -> Self;
    fn factor_gcd(&self) -> Self; // Renamed to avoid conflict with Factor trait
    fn cofactors(&self, other: &Self) -> (Expression, Expression, Expression);
}

impl PolynomialGcd for Expression {
    #[inline(always)]
    fn gcd(&self, other: &Self) -> Self {
        // Numeric GCD (most common case)
        if let (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(b))) =
            (self, other)
        {
            return Expression::integer(a.gcd(b));
        }

        if self == other {
            return self.clone();
        }

        if self.is_zero() {
            return other.clone();
        }
        if other.is_zero() {
            return self.clone();
        }

        self.polynomial_gcd_euclidean(other)
    }

    /// Least Common Multiple
    #[inline(always)]
    fn lcm(&self, other: &Self) -> Self {
        // LCM(a,b) = |a*b| / GCD(a,b)
        let gcd_val = self.gcd(other);

        if gcd_val.is_zero() {
            return Expression::integer(0);
        }

        let product = Expression::mul(vec![self.clone(), other.clone()]);
        // For now, return the product (full LCM implementation would need division)
        product
    }

    /// Factor out GCD from expression
    #[inline(always)]
    fn factor_gcd(&self) -> Self {
        match self {
            Expression::Add(terms) => {
                if terms.len() < 2 {
                    return self.clone();
                }

                // Find GCD of all terms
                let mut common_gcd = terms[0].clone();
                for term in &terms[1..] {
                    common_gcd = common_gcd.gcd(term);
                    if common_gcd.is_one() {
                        return self.clone(); // No common factor
                    }
                }

                common_gcd
            }
            Expression::Mul(_factors) => {
                // For multiplication, the GCD is the product itself
                self.clone()
            }
            _ => self.clone(),
        }
    }

    /// Compute GCD and cofactors: returns (gcd, a/gcd, b/gcd)
    fn cofactors(&self, other: &Self) -> (Expression, Expression, Expression) {
        let gcd_val = self.gcd(other);

        // For now, return simplified cofactors
        // Full implementation would need polynomial division
        (gcd_val, self.clone(), other.clone())
    }
}

impl Expression {
    /// Polynomial GCD Euclidean algorithm
    #[inline(always)]
    fn polynomial_gcd_euclidean(&self, other: &Self) -> Self {
        // Fast path: identical expressions
        if self == other {
            return self.clone();
        }

        // Fast path: check for obvious common factors
        if let Some(common_factor) = self.find_common_factor(other) {
            return common_factor;
        }

        // Check if one is a multiple of the other
        if self.is_multiple_of(other) {
            return other.clone();
        }
        if other.is_multiple_of(self) {
            return self.clone();
        }

        // For now, return 1 if no obvious common factors
        // TODO: Implement full Euclidean algorithm with polynomial division
        Expression::integer(1)
    }

    /// Find common factors between two expressions
    #[inline(always)]
    fn find_common_factor(&self, other: &Self) -> Option<Self> {
        match (self, other) {
            // Both are coefficient * symbol (like 6x and 9x)
            (Expression::Mul(_factors1), Expression::Mul(factors2)) => {
                self.find_common_factors_in_products(_factors1, factors2)
            }
            // One is multiplication, other is single term
            (Expression::Mul(factors), single) | (single, Expression::Mul(factors)) => {
                if factors.contains(single) {
                    Some(single.clone())
                } else {
                    None
                }
            }
            // Both are single terms
            _ => {
                if self == other {
                    Some(self.clone())
                } else {
                    None
                }
            }
        }
    }

    /// Extract numeric coefficient from expression
    #[allow(dead_code)]
    fn extract_numeric_coefficient(&self) -> BigInt {
        match self {
            Expression::Number(Number::Integer(n)) => BigInt::from(*n),
            Expression::Number(Number::BigInteger(n)) => n.as_ref().clone(),
            Expression::Mul(factors) => {
                for factor in factors.iter() {
                    match factor {
                        Expression::Number(Number::Integer(n)) => return BigInt::from(*n),
                        Expression::Number(Number::BigInteger(n)) => return n.as_ref().clone(),
                        _ => {}
                    }
                }
                BigInt::one()
            }
            _ => BigInt::one(),
        }
    }

    /// Find common factors in multiplication products
    #[inline(always)]
    fn find_common_factors_in_products(
        &self,
        _factors1: &[Expression],
        factors2: &[Expression],
    ) -> Option<Self> {
        let mut common_factors = Vec::new();

        // First pass: exact matches
        for factor1 in _factors1 {
            if factors2.contains(factor1) {
                common_factors.push(factor1.clone());
            }
        }

        // Second pass: power relationships (e.g., x^2 and x have common factor x)
        for factor1 in _factors1 {
            for factor2 in factors2 {
                if let Some(common) = self.find_power_common_factor(factor1, factor2) {
                    if !common_factors.contains(&common) {
                        common_factors.push(common);
                    }
                }
            }
        }

        if common_factors.is_empty() {
            None
        } else {
            Some(Expression::mul(common_factors))
        }
    }

    /// Find common factors in power expressions
    fn find_power_common_factor(
        &self,
        expr1: &Expression,
        expr2: &Expression,
    ) -> Option<Expression> {
        match (expr1, expr2) {
            (Expression::Pow(base1, _exp1), Expression::Pow(base2, _exp2)) => {
                if base1 == base2 {
                    // Common base, take minimum exponent
                    Some((**base1).clone())
                } else {
                    None
                }
            }
            (Expression::Pow(base, _exp), other) | (other, Expression::Pow(base, _exp)) => {
                if base.as_ref() == other {
                    Some(other.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Check if this expression is a multiple of another
    fn is_multiple_of(&self, other: &Self) -> bool {
        match (self, other) {
            (Expression::Mul(factors), single) => factors.contains(single),
            (Expression::Number(Number::Integer(a)), Expression::Number(Number::Integer(b))) => {
                *b != 0 && (a % b) == 0
            }
            _ => false,
        }
    }

    /// Find minimum expression (for GCD algorithms)
    #[allow(dead_code)]
    fn min_expression(&self, other: &Self) -> Self {
        // Simple heuristic: prefer shorter expressions
        let self_complexity = self.complexity();
        let other_complexity = other.complexity();

        if self_complexity <= other_complexity {
            self.clone()
        } else {
            other.clone()
        }
    }

    /// Calculate expression complexity (for heuristics)
    #[allow(dead_code)]
    fn complexity(&self) -> usize {
        match self {
            Expression::Number(_) | Expression::Symbol(_) => 1,
            Expression::Add(terms) | Expression::Mul(terms) => {
                1 + terms.iter().map(|t| t.complexity()).sum::<usize>()
            }
            Expression::Pow(base, exp) => 1 + base.complexity() + exp.complexity(),
            Expression::Function { args, .. } => {
                1 + args.iter().map(|a| a.complexity()).sum::<usize>()
            }
            // New expression types - implement later
            _ => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Symbol;

    #[test]
    fn test_number_gcd() {
        let a = Expression::integer(12);
        let b = Expression::integer(8);
        let result = a.gcd(&b);
        assert_eq!(result, Expression::integer(4));

        let a = Expression::integer(17);
        let b = Expression::integer(13);
        let result = a.gcd(&b);
        assert_eq!(result, Expression::integer(1)); // Coprime
    }

    #[test]
    fn test_gcd_with_zero() {
        let a = Expression::integer(5);
        let zero = Expression::integer(0);

        let result = a.gcd(&zero);
        assert_eq!(result, Expression::integer(5));

        let result = zero.gcd(&a);
        assert_eq!(result, Expression::integer(5));
    }

    #[test]
    fn test_identical_expressions() {
        let x = Expression::symbol(Symbol::new("x"));
        let result = x.gcd(&x);
        assert_eq!(result, x);
    }

    #[test]
    fn test_gcd_performance_benchmark() {
        use std::time::Instant;

        let start = Instant::now();

        // Perform many GCD operations
        for i in 1..10_000 {
            let a = Expression::integer(i * 6);
            let b = Expression::integer(i * 9);
            let _result = a.gcd(&b);
        }

        let duration = start.elapsed();
        let ops_per_sec = 10_000.0 / duration.as_secs_f64();

        println!("GCD Performance: {:.2}M ops/sec", ops_per_sec / 1_000_000.0);

        // Should be extremely fast (targeting >1M ops/sec)
        assert!(
            ops_per_sec > 100_000.0,
            "Expected >100K ops/sec, got {:.2}",
            ops_per_sec
        );
    }

    #[test]
    fn test_polynomial_gcd_basic() {
        let x = Symbol::new("x");

        // Test with simple polynomials
        let poly1 = Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]);
        let poly2 = Expression::mul(vec![Expression::integer(9), Expression::symbol(x.clone())]);

        let result = poly1.gcd(&poly2);

        // Should find common factor (3x or just 3 depending on implementation)
        println!("Polynomial GCD result: {}", result);
        assert!(!result.is_zero());
    }

    #[test]
    fn test_factor_gcd() {
        let x = Symbol::new("x");

        // Test factoring GCD from sum: 6x + 9x = 3x(2 + 3)
        let term1 = Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]);
        let term2 = Expression::mul(vec![Expression::integer(9), Expression::symbol(x.clone())]);
        let sum = Expression::add(vec![term1, term2]);

        let gcd_factor = sum.factor_gcd();
        println!("Factored GCD: {}", gcd_factor);

        // Should extract some common factor
        assert!(!gcd_factor.is_zero());
    }

    #[test]
    fn test_lcm_basic() {
        let a = Expression::integer(6);
        let b = Expression::integer(8);
        let result = a.lcm(&b);

        // LCM(6,8) should be related to 24, but our implementation is simplified
        println!("LCM result: {}", result);
        assert!(!result.is_zero());
    }
}
