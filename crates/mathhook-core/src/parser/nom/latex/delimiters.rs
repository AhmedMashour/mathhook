/// LaTeX delimiters and brackets
///
/// This module handles LaTeX delimiter commands like \left, \right, \langle, \rangle,
/// and various bracket types for mathematical expressions.
///
/// Architecture:
/// - Focused on delimiter parsing and matching
/// - Performance: Efficient parsing with minimal allocations
/// - Memory: Zero-copy parsing where possible
/// - Readability: Clear delimiter semantics
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, sequence::delimited,
    IResult, Parser,
};

use crate::core::Expression;
use crate::parser::nom::core::expression;

/// Parse LaTeX delimiters and brackets
///
/// Handles: \left(...\right), \langle...\rangle, \left|...\right|, etc.
pub fn latex_delimiters(input: &str) -> IResult<&str, Expression> {
    alt((
        latex_left_right_parentheses,
        latex_left_right_brackets,
        latex_left_right_braces,
        latex_left_right_absolute,
        latex_angle_brackets,
        latex_floor_ceiling,
        latex_norm_brackets,
    ))
    .parse(input)
}

/// Parse \left(...\right) - scalable parentheses
///
/// Example: \left(x + y\right) → Expression with proper grouping
fn latex_left_right_parentheses(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\left("), multispace0).parse(input)?;
    let (input, expr) = expression.parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\right)"), multispace0).parse(input)?;

    // Return the expression directly as parentheses are just for grouping
    Ok((input, expr))
}

/// Parse \left[...\right] - scalable square brackets
///
/// Example: \left[x + y\right] → Expression with bracket notation
fn latex_left_right_brackets(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\left["), multispace0).parse(input)?;
    let (input, expr) = expression.parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\right]"), multispace0).parse(input)?;

    // Use function to represent bracket notation
    Ok((input, Expression::function("brackets", vec![expr])))
}

/// Parse \left\{...\right\} - scalable curly braces (sets)
///
/// Example: \left\{x | x > 0\right\} → Set notation
fn latex_left_right_braces(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\left\\{"), multispace0).parse(input)?;
    let (input, expr) = expression.parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\right\\}"), multispace0).parse(input)?;

    // Use function to represent set notation
    Ok((input, Expression::function("set", vec![expr])))
}

/// Parse \left|...\right| - scalable absolute value bars
///
/// Example: \left|x\right| → |x| (absolute value)
fn latex_left_right_absolute(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\left|"), multispace0).parse(input)?;
    let (input, expr) = expression.parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\right|"), multispace0).parse(input)?;

    // Use abs function for absolute value
    Ok((input, Expression::function("abs", vec![expr])))
}

/// Parse \langle...\rangle - angle brackets (inner product)
///
/// Example: \langle u, v \rangle → Inner product notation
fn latex_angle_brackets(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\langle"), multispace0).parse(input)?;
    let (input, expr) = expression.parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\rangle"), multispace0).parse(input)?;

    // Use function to represent inner product/angle brackets
    Ok((input, Expression::function("inner_product", vec![expr])))
}

/// Parse floor and ceiling functions
///
/// Handles: \lfloor...\rfloor, \lceil...\rceil
fn latex_floor_ceiling(input: &str) -> IResult<&str, Expression> {
    alt((latex_floor, latex_ceiling)).parse(input)
}

/// Parse \lfloor...\rfloor - floor function
///
/// Example: \lfloor x \rfloor → floor(x)
fn latex_floor(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\lfloor"), multispace0).parse(input)?;
    let (input, expr) = expression.parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\rfloor"), multispace0).parse(input)?;

    Ok((input, Expression::function("floor", vec![expr])))
}

/// Parse \lceil...\rceil - ceiling function
///
/// Example: \lceil x \rceil → ceil(x)
fn latex_ceiling(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\lceil"), multispace0).parse(input)?;
    let (input, expr) = expression.parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\rceil"), multispace0).parse(input)?;

    Ok((input, Expression::function("ceil", vec![expr])))
}

/// Parse norm brackets
///
/// Handles: \|...\| (double bars for norms)
fn latex_norm_brackets(input: &str) -> IResult<&str, Expression> {
    alt((latex_double_bars, latex_single_bars)).parse(input)
}

/// Parse \|...\| - double bars (norm)
///
/// Example: \|v\| → norm(v)
fn latex_double_bars(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\|"), multispace0).parse(input)?;
    let (input, expr) = expression.parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\|"), multispace0).parse(input)?;

    Ok((input, Expression::function("norm", vec![expr])))
}

/// Parse |...| - single bars (absolute value, simpler version)
///
/// Example: |x| → abs(x)
fn latex_single_bars(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("|"), multispace0).parse(input)?;
    let (input, expr) = expression.parse(input)?;
    let (input, _) = delimited(multispace0, tag("|"), multispace0).parse(input)?;

    Ok((input, Expression::function("abs", vec![expr])))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Expression;
    use nom::combinator::all_consuming;

    #[test]
    fn test_latex_left_right_parentheses() {
        // Test scalable parentheses
        let result = all_consuming(latex_left_right_parentheses)
            .parse("\\left(x + y\\right)")
            .unwrap()
            .1;
        // Should return the inner expression (parentheses are just for grouping)
        assert!(matches!(result, Expression::Add(_)));
    }

    #[test]
    fn test_latex_left_right_brackets() {
        // Test scalable brackets
        let result = all_consuming(latex_left_right_brackets)
            .parse("\\left[x\\right]")
            .unwrap()
            .1;
        assert_eq!(
            result,
            Expression::function("brackets", vec![Expression::symbol("x")])
        );
    }

    #[test]
    fn test_latex_left_right_braces() {
        // Test scalable braces (set notation)
        let result = all_consuming(latex_left_right_braces)
            .parse("\\left\\{x\\right\\}")
            .unwrap()
            .1;
        assert_eq!(
            result,
            Expression::function("set", vec![Expression::symbol("x")])
        );
    }

    #[test]
    fn test_latex_left_right_absolute() {
        // Test absolute value
        let result = all_consuming(latex_left_right_absolute)
            .parse("\\left|x\\right|")
            .unwrap()
            .1;
        assert_eq!(
            result,
            Expression::function("abs", vec![Expression::symbol("x")])
        );
    }

    #[test]
    fn test_latex_angle_brackets() {
        // Test angle brackets (inner product)
        let result = all_consuming(latex_angle_brackets)
            .parse("\\langle u \\rangle")
            .unwrap()
            .1;
        assert_eq!(
            result,
            Expression::function("inner_product", vec![Expression::symbol("u")])
        );
    }

    #[test]
    fn test_latex_floor() {
        // Test floor function
        let result = all_consuming(latex_floor)
            .parse("\\lfloor x \\rfloor")
            .unwrap()
            .1;
        assert_eq!(
            result,
            Expression::function("floor", vec![Expression::symbol("x")])
        );
    }

    #[test]
    fn test_latex_ceiling() {
        // Test ceiling function
        let result = all_consuming(latex_ceiling)
            .parse("\\lceil x \\rceil")
            .unwrap()
            .1;
        assert_eq!(
            result,
            Expression::function("ceil", vec![Expression::symbol("x")])
        );
    }

    #[test]
    fn test_latex_double_bars() {
        // Test norm (double bars)
        let result = all_consuming(latex_double_bars).parse("\\|v\\|").unwrap().1;
        assert_eq!(
            result,
            Expression::function("norm", vec![Expression::symbol("v")])
        );
    }

    #[test]
    fn test_latex_single_bars() {
        // Test absolute value (single bars)
        let result = all_consuming(latex_single_bars).parse("|x|").unwrap().1;
        assert_eq!(
            result,
            Expression::function("abs", vec![Expression::symbol("x")])
        );
    }
}
