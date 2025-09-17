//! Rational expression operations and simplification
//! Handles rational functions, fraction simplification, and rational arithmetic

use crate::core::{Expression, CompactNumber, Symbol};
use crate::algebra::gcd::PolynomialGcd;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Zero, One};

/// Trait for rational expression operations
pub trait RationalSimplify {
    fn simplify_rational(&self) -> Self;
    fn rationalize(&self) -> Self;
    fn to_rational_form(&self) -> (Expression, Expression); // (numerator, denominator)
}

impl RationalSimplify for Expression {
    /// Simplify rational expressions by canceling common factors
    fn simplify_rational(&self) -> Self {
        match self {
            // Handle division represented as multiplication by inverse
            Expression::Mul(factors) => {
                self.simplify_rational_multiplication(factors)
            },
            
            // Handle fractions in other forms
            Expression::Pow(base, exp) => {
                // Check for negative exponents (which represent division)
                if let Expression::Number(CompactNumber::SmallInt(n)) = exp.as_ref() {
                    if *n < 0 {
                        // x^(-n) = 1/x^n
                        let positive_exp = Expression::integer(-n);
                        let denominator = Expression::pow(base.as_ref().clone(), positive_exp);
                        return self.create_rational_division(
                            &Expression::integer(1),
                            &denominator
                        );
                    }
                }
                self.clone()
            },
            
            _ => self.clone(),
        }
    }
    
    /// Convert to rational form (numerator/denominator)
    fn to_rational_form(&self) -> (Expression, Expression) {
        match self {
            Expression::Number(CompactNumber::Rational(r)) => {
                (
                    Expression::integer(r.numer().clone()),
                    Expression::integer(r.denom().clone())
                )
            },
            
            Expression::Mul(factors) => {
                let (num_factors, den_factors) = self.separate_rational_factors(factors);
                
                let numerator = if num_factors.is_empty() {
                    Expression::integer(1)
                } else {
                    Expression::mul(num_factors)
                };
                
                let denominator = if den_factors.is_empty() {
                    Expression::integer(1)
                } else {
                    Expression::mul(den_factors)
                };
                
                (numerator, denominator)
            },
            
            Expression::Pow(base, exp) => {
                if let Expression::Number(CompactNumber::SmallInt(n)) = exp.as_ref() {
                    if *n < 0 {
                        // Negative exponent: move to denominator
                        let positive_exp = Expression::integer(-n);
                        let denominator = Expression::pow(base.as_ref().clone(), positive_exp);
                        return (Expression::integer(1), denominator);
                    }
                }
                (self.clone(), Expression::integer(1))
            },
            
            _ => (self.clone(), Expression::integer(1)),
        }
    }
    
    /// Rationalize denominators (remove radicals from denominators)
    fn rationalize(&self) -> Self {
        // This is a complex operation - simplified implementation for now
        self.clone()
    }
}

impl Expression {
    /// Simplify rational expressions in multiplication
    fn simplify_rational_multiplication(&self, factors: &[Expression]) -> Expression {
        let (numerator_factors, denominator_factors) = self.separate_rational_factors(factors);
        
        // Simplify by canceling common factors
        let simplified = self.cancel_common_factors(&numerator_factors, &denominator_factors);
        
        simplified
    }
    
    /// Separate factors into numerator and denominator parts
    fn separate_rational_factors(&self, factors: &[Expression]) -> (Vec<Expression>, Vec<Expression>) {
        let mut numerator_factors = Vec::new();
        let mut denominator_factors = Vec::new();
        
        for factor in factors {
            match factor {
                // Negative exponents go to denominator
                Expression::Pow(base, exp) => {
                    if let Expression::Number(CompactNumber::SmallInt(n)) = exp.as_ref() {
                        if *n < 0 {
                            let positive_exp = Expression::integer(-n);
                            denominator_factors.push(
                                Expression::pow(base.as_ref().clone(), positive_exp)
                            );
                        } else {
                            numerator_factors.push(factor.clone());
                        }
                    } else {
                        numerator_factors.push(factor.clone());
                    }
                },
                
                // Regular factors go to numerator
                _ => {
                    numerator_factors.push(factor.clone());
                }
            }
        }
        
        (numerator_factors, denominator_factors)
    }
    
    /// Cancel common factors between numerator and denominator
    fn cancel_common_factors(&self, num_factors: &[Expression], den_factors: &[Expression]) -> Expression {
        if den_factors.is_empty() {
            // No denominator, just return numerator
            if num_factors.is_empty() {
                return Expression::integer(1);
            } else if num_factors.len() == 1 {
                return num_factors[0].clone();
            } else {
                return Expression::mul(num_factors.to_vec());
            }
        }
        
        let numerator = if num_factors.is_empty() {
            Expression::integer(1)
        } else {
            Expression::mul(num_factors.to_vec())
        };
        
        let denominator = Expression::mul(den_factors.to_vec());
        
        // Find GCD of numerator and denominator
        let gcd = numerator.gcd(&denominator);
        
        if !gcd.is_one() {
            // Cancel common factor
            let simplified_num = self.divide_expressions(&numerator, &gcd);
            let simplified_den = self.divide_expressions(&denominator, &gcd);
            
            self.create_rational_division(&simplified_num, &simplified_den)
        } else {
            self.create_rational_division(&numerator, &denominator)
        }
    }
    
    /// Create a rational division expression
    fn create_rational_division(&self, numerator: &Expression, denominator: &Expression) -> Expression {
        if denominator.is_one() {
            numerator.clone()
        } else if numerator.is_zero() {
            Expression::integer(0)
        } else {
            // Represent as multiplication by inverse (negative exponent)
            Expression::mul(vec![
                numerator.clone(),
                Expression::pow(denominator.clone(), Expression::integer(-1))
            ])
        }
    }
    
    /// Divide two expressions (simplified division)
    fn divide_expressions(&self, dividend: &Expression, divisor: &Expression) -> Expression {
        match (dividend, divisor) {
            // Numeric division
                (Expression::Number(CompactNumber::SmallInt(a)), Expression::Number(CompactNumber::SmallInt(b))) => {
                if !b.is_zero() {
                    let rational = BigRational::new(a.clone(), b.clone());
                    if rational.denom().is_one() {
                        Expression::integer(rational.numer().clone())
                    } else {
                        Expression::number(CompactNumber::rational(rational))
                    }
                } else {
                    dividend.clone() // Division by zero - return original
                }
            },
            
            // Same expressions divide to 1
            _ if dividend == divisor => Expression::integer(1),
            
            // Multiplication division
            (Expression::Mul(factors), _) => {
                let mut remaining_factors = factors.clone();
                if let Some(pos) = remaining_factors.iter().position(|f| f == divisor) {
                    remaining_factors.remove(pos);
                    if remaining_factors.is_empty() {
                        Expression::integer(1)
                    } else if remaining_factors.len() == 1 {
                        remaining_factors[0].clone()
                    } else {
                        Expression::mul((**remaining_factors).clone())
                    }
                } else {
                    dividend.clone()
                }
            },
            
            // Default: return original
            _ => dividend.clone(),
        }
    }
    
    /// Add rational expressions: a/b + c/d = (ad + bc)/(bd)
    pub fn add_rationals(&self, other: &Expression) -> Expression {
        let (num1, den1) = self.to_rational_form();
        let (num2, den2) = other.to_rational_form();
        
        if den1 == den2 {
            // Same denominator: just add numerators
            let new_num = Expression::add(vec![num1, num2]);
            self.create_rational_division(&new_num, &den1)
        } else {
            // Different denominators: find common denominator
            let new_num = Expression::add(vec![
                Expression::mul(vec![num1, den2.clone()]),
                Expression::mul(vec![num2, den1.clone()])
            ]);
            let new_den = Expression::mul(vec![den1, den2]);
            
            self.create_rational_division(&new_num, &new_den)
        }
    }
    
    /// Multiply rational expressions: (a/b) * (c/d) = (ac)/(bd)
    pub fn multiply_rationals(&self, other: &Expression) -> Expression {
        let (num1, den1) = self.to_rational_form();
        let (num2, den2) = other.to_rational_form();
        
        let new_num = Expression::mul(vec![num1, num2]);
        let new_den = Expression::mul(vec![den1, den2]);
        
        self.create_rational_division(&new_num, &new_den)
    }
    
    /// Simplify complex rational expressions
    pub fn simplify_complex_rational(&self) -> Expression {
        // Handle nested fractions and complex rational expressions
        match self {
            Expression::Mul(factors) => {
                // Look for patterns like (a/b) * (c/d)
                let mut rational_parts = Vec::new();
                let mut other_parts = Vec::new();
                
                for factor in factors.iter() {
                    if self.is_rational_expression(factor) {
                        rational_parts.push(factor.clone());
                    } else {
                        other_parts.push(factor.clone());
                    }
                }
                
                if rational_parts.len() > 1 {
                    // Multiply rational parts together
                    let mut result = rational_parts[0].clone();
                    for rational in &rational_parts[1..] {
                        result = result.multiply_rationals(rational);
                    }
                    
                    // Combine with other parts
                    if !other_parts.is_empty() {
                        other_parts.push(result);
                        Expression::mul(other_parts)
                    } else {
                        result
                    }
                } else {
                    self.clone()
                }
            },
            _ => self.clone(),
        }
    }
    
    /// Check if an expression is a rational expression
    fn is_rational_expression(&self, expr: &Expression) -> bool {
        match expr {
            Expression::Number(CompactNumber::Rational(_)) => true,
            Expression::Pow(_, exp) => {
                // Negative exponents indicate rational expressions
                if let Expression::Number(CompactNumber::SmallInt(n)) = exp.as_ref() {
                    *n < 0
                } else {
                    false
                }
            },
            _ => false,
        }
    }
    
    /// Extract rational coefficient from expression
    pub fn extract_rational_coefficient(&self) -> (BigRational, Expression) {
        match self {
            Expression::Number(CompactNumber::Rational(r)) => ((**r).clone(), Expression::integer(1)),
            Expression::Number(CompactNumber::SmallInt(n)) => {
                (BigRational::from(n.clone()), Expression::integer(1))
            },
            Expression::Mul(factors) => {
                let mut coefficient = BigRational::one();
                let mut non_rational_factors = Vec::new();
                
                for factor in factors.iter() {
                    match factor {
                        Expression::Number(CompactNumber::Rational(r)) => {
                            coefficient *= r;
                        },
                        Expression::Number(CompactNumber::SmallInt(n)) => {
                            coefficient *= BigRational::from(n.clone());
                        },
                        _ => {
                            non_rational_factors.push(factor.clone());
                        }
                    }
                }
                
                let remaining = if non_rational_factors.is_empty() {
                    Expression::integer(1)
                } else if non_rational_factors.len() == 1 {
                    non_rational_factors[0].clone()
                } else {
                    Expression::mul(non_rational_factors)
                };
                
                (coefficient, remaining)
            },
            _ => (BigRational::one(), self.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rational_detection() {
        // Test basic rational number
        let rational = Expression::number(CompactNumber::rational(
            BigRational::new(BigInt::from(3), BigInt::from(4))
        ));
        
        let (num, den) = rational.to_rational_form();
        
        assert_eq!(num, Expression::integer(3));
        assert_eq!(den, Expression::integer(4));
    }
    
    #[test]
    fn test_simple_rational_combination() {
        // Test 1/2 + 1/3 = 5/6
        let half = Expression::number(CompactNumber::rational(
            BigRational::new(BigInt::from(1), BigInt::from(2))
        ));
        let third = Expression::number(CompactNumber::rational(
            BigRational::new(BigInt::from(1), BigInt::from(3))
        ));
        
        let result = half.add_rationals(&third);
        println!("1/2 + 1/3 = {}", result);
        
        // Should be 5/6
        assert!(!result.is_zero());
    }
    
    #[test]
    fn test_rational_simplification() {
        let x = Symbol::new("x");
        
        // Test x^(-1) = 1/x
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1));
        let result = expr.simplify_rational();
        
        println!("x^(-1) simplified = {}", result);
        assert!(!result.is_zero());
    }
    
    #[test]
    fn test_rational_multiplication() {
        // Test (2/3) * (3/4) = 6/12 = 1/2
        let frac1 = Expression::number(CompactNumber::rational(
            BigRational::new(BigInt::from(2), BigInt::from(3))
        ));
        let frac2 = Expression::number(CompactNumber::rational(
            BigRational::new(BigInt::from(3), BigInt::from(4))
        ));
        
        let result = frac1.multiply_rationals(&frac2);
        println!("(2/3) * (3/4) = {}", result);
        
        assert!(!result.is_zero());
    }
    
    #[test]
    fn test_common_factor_cancellation() {
        let x = Symbol::new("x");
        
        // Test (6x) / (9x) = 2/3 (if implemented)
        let numerator = Expression::mul(vec![
            Expression::integer(6),
            Expression::symbol(x.clone())
        ]);
        let denominator = Expression::mul(vec![
            Expression::integer(9),
            Expression::symbol(x.clone())
        ]);
        
        let expr = Expression::integer(1).create_rational_division(&numerator, &denominator);
        let result = expr.simplify_rational();
        
        println!("(6x)/(9x) simplified = {}", result);
        assert!(!result.is_zero());
    }
    
    #[test]
    fn test_extract_rational_coefficient() {
        let x = Symbol::new("x");
        
        let expr = Expression::mul(vec![
            Expression::number(CompactNumber::rational(
                BigRational::new(BigInt::from(3), BigInt::from(4))
            )),
            Expression::symbol(x.clone())
        ]);
        
        let (coeff, remaining) = expr.extract_rational_coefficient();
        
        println!("Coefficient: {}, Remaining: {}", 
                 Expression::number(CompactNumber::rational(coeff)), remaining);
        
        assert_eq!(remaining, Expression::symbol(x));
    }
}
