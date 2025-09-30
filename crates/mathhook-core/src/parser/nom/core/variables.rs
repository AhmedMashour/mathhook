/// Variable and identifier parsing using nom combinators
///
/// Handles parsing of mathematical variables and identifiers:
/// - Simple variables: x, y, z, a, b
/// - Multi-character identifiers: var, alpha, beta
/// - Mathematical constants: pi, e, i, infinity
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0},
    combinator::{map, recognize},
    sequence::pair,
    IResult, Parser,
};

use crate::core::Expression;
use crate::parser::nom::shared::constants::parse_simple_constants;

/// Parse a variable or mathematical constant
///
/// This handles both simple variables (x, y) and mathematical constants (pi, e, i).
pub fn variable(input: &str) -> IResult<&str, Expression> {
    alt((parse_simple_constants, identifier)).parse(input)
}

/// Parse a general identifier
///
/// Handles variable names that start with a letter and can contain letters and numbers.
fn identifier(input: &str) -> IResult<&str, Expression> {
    map(recognize(pair(alpha1, alphanumeric0)), |s: &str| {
        Expression::symbol(s)
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_variables() {
        // Single letter variables
        let result = variable("x");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed variable x: {:?}", expr);

        let result = variable("y");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed variable y: {:?}", expr);
    }

    #[test]
    fn test_multi_character_identifiers() {
        // Multi-character identifiers
        let result = variable("alpha");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed identifier alpha: {:?}", expr);

        let result = variable("var1");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed identifier var1: {:?}", expr);
    }

    #[test]
    fn test_mathematical_constants() {
        // Pi constant
        let result = variable("pi");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed constant pi: {:?}", expr);

        // Euler's number
        let result = variable("e");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed constant e: {:?}", expr);

        // Imaginary unit
        let result = variable("i");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed constant i: {:?}", expr);

        // Infinity
        let result = variable("infinity");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed constant infinity: {:?}", expr);
    }

    #[test]
    fn test_constant_precedence() {
        // Ensure constants are recognized before general identifiers
        // "pi" should be parsed as the constant π, not as a variable named "pi"
        let result = variable("pi");
        assert!(result.is_ok());
        let (_, expr) = result.unwrap();

        // This should be a Pi constant, not a Symbol("pi")
        match expr {
            Expression::Constant(_) => println!("✓ Correctly parsed as constant"),
            Expression::Symbol(_) => panic!("❌ Incorrectly parsed as symbol"),
            _ => panic!("❌ Unexpected expression type"),
        }
    }
}
