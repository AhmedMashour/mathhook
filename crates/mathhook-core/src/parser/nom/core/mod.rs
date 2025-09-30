/// Core parsing primitives using nom combinators
///
/// This module contains the fundamental building blocks for mathematical expression parsing:
/// - Numbers (integers, floats, scientific notation)
/// - Variables (identifiers, symbols)
/// - Operators (with proper precedence handling)
/// - Parenthesized expressions
pub mod numbers;
pub mod operators;
pub mod variables;

use crate::core::Expression;
use nom::{IResult, Parser};

/// Main expression parser - entry point for all mathematical expressions
///
/// Handles operator precedence using nom's fold_many0 combinator for left-associative operations.
/// This eliminates the grammar conflicts that plagued the LALRPOP implementation.
pub fn expression(input: &str) -> IResult<&str, Expression> {
    use nom::branch::alt;

    // Try implicit multiplication first, then fall back to regular parsing
    alt((implicit_multiplication, operators::relation)).parse(input)
}

/// Parse implicit multiplication: 2x, 3sin(x), 4i, etc.
///
/// This handles cases where multiplication is implied without an explicit operator.
/// Enhanced to handle constants like 'i' in complex numbers (e.g., 4i = 4 * i).
pub fn implicit_multiplication(input: &str) -> IResult<&str, Expression> {
    use nom::character::complete::alpha1;
    use nom::combinator::{not, peek};

    // Look for number followed immediately by a letter (no space)
    // Use 'not' to ensure we don't consume whitespace between number and variable
    let (input, num) = numbers::number.parse(input)?;
    let (input, _) = peek(alpha1).parse(input)?; // Peek to ensure there's a letter next
    let (input, _) = not(nom::character::complete::multispace1).parse(input)?; // Ensure no space
    let (input, var) = variables::variable.parse(input)?;

    Ok((input, Expression::mul(vec![num, var])))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_expression_parsing() {
        // Test number parsing
        let result = expression("42");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed number: {:?}", expr);

        // Test variable parsing
        let result = expression("x");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed variable: {:?}", expr);
    }
}
