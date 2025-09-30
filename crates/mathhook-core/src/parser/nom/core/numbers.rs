/// Number parsing using nom combinators
///
/// Handles parsing of various number formats:
/// - Integers: 42, -17, 0
/// - Floats: 3.14, -2.5, 0.0
/// - Scientific notation: 1e10, 2.5e-3, -1.23E+5
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    combinator::{map_res, opt, recognize},
    sequence::pair,
    IResult, Parser,
};

use crate::core::Expression;

/// Parse a number (integer or float)
///
/// This is the main entry point for number parsing. It handles both
/// integers and floating-point numbers with optional scientific notation.
pub fn number(input: &str) -> IResult<&str, Expression> {
    alt((
        map_res(scientific_notation, |s: &str| {
            s.parse::<f64>().map(Expression::number)
        }),
        map_res(float, |s: &str| s.parse::<f64>().map(Expression::number)),
        map_res(integer, |s: &str| s.parse::<i64>().map(Expression::integer)),
    ))
    .parse(input)
}

/// Parse an integer
///
/// Handles positive and negative integers: 42, -17, 0
fn integer(input: &str) -> IResult<&str, &str> {
    recognize(pair(opt(tag("-")), digit1)).parse(input)
}

/// Parse a floating-point number
///
/// Handles numbers with decimal points: 3.14, -2.5, 0.0
fn float(input: &str) -> IResult<&str, &str> {
    recognize((opt(tag("-")), digit1, tag("."), digit1)).parse(input)
}

/// Parse scientific notation
///
/// Handles scientific notation: 1e10, 2.5e-3, -1.23E+5
fn scientific_notation(input: &str) -> IResult<&str, &str> {
    recognize((
        alt((float, integer)),
        one_of("eE"),
        opt(one_of("+-")),
        digit1,
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_parsing() {
        // Positive integer
        let result = number("42");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed integer 42: {:?}", expr);

        // Negative integer
        let result = number("-17");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed integer -17: {:?}", expr);

        // Zero
        let result = number("0");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed integer 0: {:?}", expr);
    }

    #[test]
    fn test_float_parsing() {
        // Positive float
        let result = number("3.14");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed float 3.14: {:?}", expr);

        // Negative float
        let result = number("-2.5");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed float -2.5: {:?}", expr);
    }

    #[test]
    fn test_scientific_notation() {
        // Basic scientific notation
        let result = number("1e10");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed scientific 1e10: {:?}", expr);

        // Scientific with decimal
        let result = number("2.5e-3");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed scientific 2.5e-3: {:?}", expr);

        // Scientific with positive exponent
        let result = number("-1.23E+5");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed scientific -1.23E+5: {:?}", expr);
    }

    #[test]
    fn test_whitespace_handling() {
        // Number with leading whitespace
        let result = number("  42");
        assert!(result.is_err()); // Should fail - we don't handle whitespace in number parser

        // Number with trailing whitespace
        let result = number("42  ");
        assert!(result.is_ok());
        let (remaining, _) = result.unwrap();
        assert_eq!(remaining, "  "); // Remaining whitespace should be left
    }
}
