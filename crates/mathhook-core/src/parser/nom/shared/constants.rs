//! Shared mathematical constants for LaTeX and Wolfram parsers
//!
//! This module provides a unified set of mathematical constants that can be used
//! by both LaTeX and Wolfram parsers, ensuring consistency and avoiding duplication.

use crate::core::Expression;
use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult, Parser};

/// Mathematical constants that are shared between LaTeX and Wolfram
#[derive(Debug, Clone, PartialEq)]
pub enum MathConstant {
    Pi,
    E,
    I,
    Infinity,
    NegativeInfinity,
    Undefined,
    GoldenRatio,
    EulerGamma,
    TribonacciConstant,
}

impl MathConstant {
    /// Convert to Expression
    pub fn to_expression(&self) -> Expression {
        match self {
            MathConstant::Pi => Expression::pi(),
            MathConstant::E => Expression::e(),
            MathConstant::I => Expression::i(),
            MathConstant::Infinity => Expression::infinity(),
            MathConstant::NegativeInfinity => {
                Expression::mul(vec![Expression::integer(-1), Expression::infinity()])
            }
            MathConstant::Undefined => Expression::undefined(),
            MathConstant::GoldenRatio => Expression::symbol("φ"),
            MathConstant::EulerGamma => Expression::symbol("γ"),
            MathConstant::TribonacciConstant => Expression::symbol("T"),
        }
    }

    /// Get LaTeX representation
    pub fn latex_name(&self) -> &'static str {
        match self {
            MathConstant::Pi => "\\pi",
            MathConstant::E => "\\e",
            MathConstant::I => "\\i",
            MathConstant::Infinity => "\\infty",
            MathConstant::NegativeInfinity => "-\\infty",
            MathConstant::Undefined => "\\text{undefined}",
            MathConstant::GoldenRatio => "\\phi",
            MathConstant::EulerGamma => "\\gamma",
            MathConstant::TribonacciConstant => "T",
        }
    }

    /// Get Wolfram representation
    pub fn wolfram_name(&self) -> &'static str {
        match self {
            MathConstant::Pi => "Pi",
            MathConstant::E => "E",
            MathConstant::I => "I",
            MathConstant::Infinity => "Infinity",
            MathConstant::NegativeInfinity => "-Infinity",
            MathConstant::Undefined => "Undefined",
            MathConstant::GoldenRatio => "GoldenRatio",
            MathConstant::EulerGamma => "EulerGamma",
            MathConstant::TribonacciConstant => "TribonacciConstant",
        }
    }

    /// Get simple/core representation
    pub fn simple_name(&self) -> &'static str {
        match self {
            MathConstant::Pi => "pi",
            MathConstant::E => "e",
            MathConstant::I => "i",
            MathConstant::Infinity => "infinity",
            MathConstant::NegativeInfinity => "-infinity",
            MathConstant::Undefined => "undefined",
            MathConstant::GoldenRatio => "GoldenRatio",
            MathConstant::EulerGamma => "EulerGamma",
            MathConstant::TribonacciConstant => "TribonacciConstant",
        }
    }
}

/// Parse LaTeX mathematical constants
pub fn parse_latex_constants(input: &str) -> IResult<&str, Expression> {
    alt((
        // Order matters: longer patterns first to avoid partial matches
        map(tag("\\eulergamma"), |_| {
            MathConstant::EulerGamma.to_expression()
        }),
        map(tag("\\varphi"), |_| {
            MathConstant::GoldenRatio.to_expression()
        }),
        map(tag("\\infty"), |_| MathConstant::Infinity.to_expression()),
        map(tag("\\gamma"), |_| MathConstant::EulerGamma.to_expression()),
        map(tag("\\phi"), |_| MathConstant::GoldenRatio.to_expression()),
        map(tag("\\pi"), |_| MathConstant::Pi.to_expression()),
        map(tag("\\e"), |_| MathConstant::E.to_expression()),
        map(tag("\\i"), |_| MathConstant::I.to_expression()),
    ))
    .parse(input)
}

/// Parse Wolfram mathematical constants
pub fn parse_wolfram_constants(input: &str) -> IResult<&str, Expression> {
    alt((
        // Order matters: longer patterns first to avoid partial matches
        map(tag("TribonacciConstant"), |_| {
            MathConstant::TribonacciConstant.to_expression()
        }),
        map(tag("GoldenRatio"), |_| {
            MathConstant::GoldenRatio.to_expression()
        }),
        map(tag("EulerGamma"), |_| {
            MathConstant::EulerGamma.to_expression()
        }),
        map(tag("Infinity"), |_| MathConstant::Infinity.to_expression()),
        map(tag("Undefined"), |_| {
            MathConstant::Undefined.to_expression()
        }),
        map(tag("Pi"), |_| MathConstant::Pi.to_expression()),
        map(tag("E"), |_| MathConstant::E.to_expression()),
        map(tag("I"), |_| MathConstant::I.to_expression()),
    ))
    .parse(input)
}

/// Parse simple/core mathematical constants
pub fn parse_simple_constants(input: &str) -> IResult<&str, Expression> {
    alt((
        // Order matters: longer patterns first to avoid partial matches
        map(tag("TribonacciConstant"), |_| {
            MathConstant::TribonacciConstant.to_expression()
        }),
        map(tag("GoldenRatio"), |_| {
            MathConstant::GoldenRatio.to_expression()
        }),
        map(tag("EulerGamma"), |_| {
            MathConstant::EulerGamma.to_expression()
        }),
        map(tag("infinity"), |_| MathConstant::Infinity.to_expression()),
        map(tag("undefined"), |_| {
            MathConstant::Undefined.to_expression()
        }),
        map(tag("pi"), |_| MathConstant::Pi.to_expression()),
        map(tag("e"), |_| MathConstant::E.to_expression()),
        map(tag("i"), |_| MathConstant::I.to_expression()),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latex_constants() {
        // Test basic constants
        let result = parse_latex_constants("\\pi").unwrap().1;
        assert_eq!(result, Expression::pi());

        let result = parse_latex_constants("\\phi").unwrap().1;
        assert_eq!(result, Expression::symbol("φ"));

        let result = parse_latex_constants("\\gamma").unwrap().1;
        assert_eq!(result, Expression::symbol("γ"));
    }

    #[test]
    fn test_wolfram_constants() {
        // Test Wolfram constants
        let result = parse_wolfram_constants("Pi").unwrap().1;
        assert_eq!(result, Expression::pi());

        let result = parse_wolfram_constants("GoldenRatio").unwrap().1;
        assert_eq!(result, Expression::symbol("φ"));

        let result = parse_wolfram_constants("EulerGamma").unwrap().1;
        assert_eq!(result, Expression::symbol("γ"));
    }

    #[test]
    fn test_simple_constants() {
        // Test simple constants
        let result = parse_simple_constants("pi").unwrap().1;
        assert_eq!(result, Expression::pi());

        let result = parse_simple_constants("GoldenRatio").unwrap().1;
        assert_eq!(result, Expression::symbol("φ"));
    }

    #[test]
    fn test_constant_conversions() {
        let golden_ratio = MathConstant::GoldenRatio;

        assert_eq!(golden_ratio.latex_name(), "\\phi");
        assert_eq!(golden_ratio.wolfram_name(), "GoldenRatio");
        assert_eq!(golden_ratio.simple_name(), "GoldenRatio");
        assert_eq!(golden_ratio.to_expression(), Expression::symbol("φ"));
    }
}
