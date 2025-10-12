//! Smart display utilities for natural mathematical notation
//!
//! This module provides high-performance, memory-efficient utilities for
//! displaying mathematical expressions in natural notation (x - y instead of x + -1 * y).

use super::Expression;
use std::fmt;

/// Smart display utilities
///
/// Provides O(1) pattern detection and minimal memory allocation
/// for natural mathematical notation display.
pub struct SmartDisplayFormatter;

impl SmartDisplayFormatter {
    /// Smart formatting for addition that detects subtraction patterns
    ///
    /// Converts internal canonical form (x + -1 * y) to natural notation (x - y)
    /// with optimal performance and minimal allocations.
    #[inline]
    pub fn format_addition_smartly(
        f: &mut fmt::Formatter<'_>,
        terms: &[Expression],
    ) -> fmt::Result {
        if terms.is_empty() {
            return write!(f, "0");
        }

        for (i, term) in terms.iter().enumerate() {
            match term {
                // Detect -1 * expr pattern (subtraction) - O(1) check
                Expression::Mul(factors) if Self::is_negative_one_multiplication(factors) => {
                    let positive_part = Self::extract_positive_part(factors);
                    if i == 0 {
                        write!(f, "-{}", positive_part)?;
                    } else {
                        write!(f, " - {}", positive_part)?;
                    }
                }
                // Regular positive term
                _ => {
                    if i == 0 {
                        write!(f, "{}", term)?;
                    } else {
                        write!(f, " + {}", term)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Smart formatting for multiplication that detects division patterns
    ///
    /// Converts internal canonical form (x * y^-1) to natural notation (x / y)
    /// with optimal performance and minimal allocations.
    #[inline]
    pub fn format_multiplication_smartly(
        f: &mut fmt::Formatter<'_>,
        factors: &[Expression],
    ) -> fmt::Result {
        if factors.is_empty() {
            return write!(f, "1");
        }

        // O(1) division pattern detection: x * y^(-1) → x / y
        if let Some((dividend, divisor)) = Self::extract_division_parts(factors) {
            return write!(f, "{} / {}", dividend, divisor);
        }

        // Regular multiplication - pre-allocate for performance
        let factor_strs: Vec<String> = factors.iter().map(|factor| format!("{}", factor)).collect();
        write!(f, "{}", factor_strs.join(" * "))
    }

    /// O(1) check if factors represent -1 * expr pattern (negated expression)
    ///
    /// This detects the canonical form created by the Neg trait implementation:
    /// Expression::mul(vec![Expression::integer(-1), expr])
    #[inline(always)]
    fn is_negative_one_multiplication(factors: &[Expression]) -> bool {
        // Exact pattern match for Neg trait output: [-1, expr]
        factors.len() == 2 && Self::is_negative_one(&factors[0])
    }

    /// O(1) check if expression is -1
    #[inline(always)]
    fn is_negative_one(expr: &Expression) -> bool {
        matches!(expr, Expression::Number(num) if num.is_negative_one())
    }

    /// Extract positive part from -1 * expr with minimal allocations
    ///
    /// Optimized for the exact Neg trait pattern: [-1, expr]
    /// Uses efficient string formatting with minimal heap allocations.
    #[inline]
    fn extract_positive_part(factors: &[Expression]) -> String {
        // Optimized for Neg trait pattern: exactly 2 factors
        if factors.len() == 2 {
            format!("{}", factors[1])
        } else {
            // Fallback for complex cases (rare)
            let positive_factors: Vec<String> =
                factors[1..].iter().map(|f| format!("{}", f)).collect();
            positive_factors.join(" * ")
        }
    }

    /// Check if an expression is in negated form
    ///
    /// This is a high-level utility that checks if an expression
    /// represents a negated value using the canonical Neg trait pattern.
    #[inline]
    pub fn is_negated_expression(expr: &Expression) -> bool {
        match expr {
            Expression::Mul(factors) => Self::is_negative_one_multiplication(factors),
            _ => false,
        }
    }

    /// Extract the positive form of a negated expression
    ///
    /// If the expression is negated (-1 * expr), returns the positive part.
    /// Otherwise returns None.
    #[inline]
    pub fn extract_negated_expression(expr: &Expression) -> Option<&Expression> {
        match expr {
            Expression::Mul(factors) if Self::is_negative_one_multiplication(factors) => {
                Some(&factors[1])
            }
            _ => None,
        }
    }

    /// O(1) check if factors represent division pattern: x * y^(-1)
    ///
    /// Detects the canonical form created by division operations
    /// where division is represented as multiplication by negative power.
    #[inline(always)]
    pub fn is_division_pattern(factors: &[Expression]) -> bool {
        factors.len() == 2 && Self::is_negative_power(&factors[1])
    }

    /// O(1) check if expression is y^(-1) (reciprocal)
    #[inline(always)]
    fn is_negative_power(expr: &Expression) -> bool {
        match expr {
            Expression::Pow(_, exp) => Self::is_negative_one(exp),
            _ => false,
        }
    }

    /// Extract dividend and divisor from division pattern
    ///
    /// For x * y^(-1), returns (x, y) for formatting as x / y
    #[inline]
    pub fn extract_division_parts(factors: &[Expression]) -> Option<(&Expression, &Expression)> {
        if factors.len() == 2 {
            if let Expression::Pow(base, exp) = &factors[1] {
                if Self::is_negative_one(exp) {
                    return Some((&factors[0], base));
                }
            }
        }
        None
    }

    /// Check if an expression is in division form
    ///
    /// High-level utility to detect if multiplication represents division
    #[inline]
    pub fn is_division_expression(expr: &Expression) -> bool {
        match expr {
            Expression::Mul(factors) => Self::is_division_pattern(factors),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{symbol, Expression};

    #[test]
    fn test_negative_one_detection() {
        let neg_one = Expression::integer(-1);
        assert!(SmartDisplayFormatter::is_negative_one(&neg_one));

        let pos_one = Expression::integer(1);
        assert!(!SmartDisplayFormatter::is_negative_one(&pos_one));
    }

    #[test]
    fn test_subtraction_pattern_detection() {
        let factors = vec![Expression::integer(-1), Expression::symbol(symbol!(y))];
        assert!(SmartDisplayFormatter::is_negative_one_multiplication(
            &factors
        ));

        let factors = vec![Expression::integer(2), Expression::symbol(symbol!(y))];
        assert!(!SmartDisplayFormatter::is_negative_one_multiplication(
            &factors
        ));
    }

    #[test]
    fn test_positive_part_extraction() {
        let factors = vec![Expression::integer(-1), Expression::symbol(symbol!(y))];
        let result = SmartDisplayFormatter::extract_positive_part(&factors);
        assert_eq!(result, "y");

        let factors = vec![
            Expression::integer(-1),
            Expression::symbol(symbol!(x)),
            Expression::symbol(symbol!(y)),
        ];
        let result = SmartDisplayFormatter::extract_positive_part(&factors);
        assert_eq!(result, "x * y");
    }

    #[test]
    fn test_division_pattern_detection() {
        // Test x * y^(-1) pattern
        let factors = vec![
            Expression::symbol(symbol!(x)),
            Expression::pow(Expression::symbol(symbol!(y)), Expression::integer(-1)),
        ];
        assert!(SmartDisplayFormatter::is_division_pattern(&factors));

        // Test regular multiplication
        let factors = vec![
            Expression::symbol(symbol!(x)),
            Expression::symbol(symbol!(y)),
        ];
        assert!(!SmartDisplayFormatter::is_division_pattern(&factors));
    }

    #[test]
    fn test_division_parts_extraction() {
        let factors = vec![
            Expression::symbol(symbol!(x)),
            Expression::pow(Expression::symbol(symbol!(y)), Expression::integer(-1)),
        ];

        let result = SmartDisplayFormatter::extract_division_parts(&factors);
        assert!(result.is_some());

        if let Some((dividend, divisor)) = result {
            assert_eq!(dividend, &Expression::symbol(symbol!(x)));
            assert_eq!(divisor, &Expression::symbol(symbol!(y)));
        }
    }

    #[test]
    fn test_high_level_utilities() {
        // Test negated expression detection
        let negated = Expression::mul(vec![
            Expression::integer(-1),
            Expression::symbol(symbol!(x)),
        ]);
        assert!(SmartDisplayFormatter::is_negated_expression(&negated));

        // Test division expression detection
        let division = Expression::mul(vec![
            Expression::symbol(symbol!(x)),
            Expression::pow(Expression::symbol(symbol!(y)), Expression::integer(-1)),
        ]);
        assert!(SmartDisplayFormatter::is_division_expression(&division));
    }
}
