/// Whitespace handling utilities for nom parsing
///
/// Provides efficient whitespace parsing and management for mathematical expressions.
use nom::{
    character::complete::multispace0, combinator::map, sequence::delimited, IResult, Parser,
};

/// Parse optional whitespace
///
/// This is a fundamental utility used throughout the parser to handle
/// optional whitespace between tokens.
///
/// Note: For nom 8.0, we use multispace0 directly in the operators module
/// instead of this generic wrapper to avoid complex trait bounds.
pub fn ws<'a, O>(
    inner: impl Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>>,
) -> impl Parser<&'a str, Output = O, Error = nom::error::Error<&'a str>> {
    delimited(multispace0, inner, multispace0)
}

/// Parse mandatory whitespace
pub fn ws1(input: &str) -> IResult<&str, ()> {
    map(nom::character::complete::multispace1, |_| ()).parse(input)
}

/// Skip optional whitespace (returns nothing)
pub fn skip_ws(input: &str) -> IResult<&str, ()> {
    map(multispace0, |_| ()).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::tag;

    #[test]
    fn test_whitespace_handling() {
        // Test ws combinator with a simple tag
        let mut parser = ws(tag("hello"));

        // Should parse with no whitespace
        assert_eq!(parser.parse("hello"), Ok(("", "hello")));

        // Should parse with leading whitespace
        assert_eq!(parser.parse("  hello"), Ok(("", "hello")));

        // Should parse with trailing whitespace
        assert_eq!(parser.parse("hello  "), Ok(("", "hello")));

        // Should parse with both leading and trailing whitespace
        assert_eq!(parser.parse("  hello  "), Ok(("", "hello")));
    }
}
