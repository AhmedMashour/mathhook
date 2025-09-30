/// LaTeX vector and differential operators
///
/// This module handles LaTeX commands for vectors, gradients, and differential operators
/// like \vec{}, \nabla, \partial, etc.
///
/// Architecture:
/// - Focused on vector calculus and differential geometry
/// - Performance: Efficient parsing with minimal allocations
/// - Memory: Zero-copy parsing where possible
/// - Readability: Clear mathematical semantics
use nom::{
    branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::map,
    sequence::delimited, IResult, Parser,
};

use crate::core::Expression;
use crate::parser::nom::core::expression;

/// Parse LaTeX vector and differential operators
///
/// Handles: \vec{F}, \nabla, \partial, \nabla \cdot, \nabla \times, etc.
pub fn latex_vector_operators(input: &str) -> IResult<&str, Expression> {
    alt((
        latex_vec,
        latex_nabla_operations,
        latex_partial,
        latex_nabla,
        latex_hat,
        latex_bar,
        latex_tilde,
        latex_dot,
        latex_ddot,
    ))
    .parse(input)
}

/// Parse \vec{F} - vector notation
///
/// Example: \vec{F} → Expression::function("vector", [F])
fn latex_vec(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\vec"), multispace0).parse(input)?;
    let (input, expr) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        expression,
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    Ok((input, Expression::function("vector", vec![expr])))
}

/// Parse \hat{x} - unit vector notation
///
/// Example: \hat{x} → Expression::function("unit_vector", [x])
fn latex_hat(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\hat"), multispace0).parse(input)?;
    let (input, expr) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        expression,
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    Ok((input, Expression::function("unit_vector", vec![expr])))
}

/// Parse \bar{x} - overline notation
///
/// Example: \bar{x} → Expression::function("overline", [x])
fn latex_bar(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\bar"), multispace0).parse(input)?;
    let (input, expr) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        expression,
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    Ok((input, Expression::function("overline", vec![expr])))
}

/// Parse \tilde{x} - tilde notation
///
/// Example: \tilde{x} → Expression::function("tilde", [x])
fn latex_tilde(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\tilde"), multispace0).parse(input)?;
    let (input, expr) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        expression,
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    Ok((input, Expression::function("tilde", vec![expr])))
}

/// Parse \dot{x} - single dot notation (time derivative)
///
/// Example: \dot{x} → Expression::function("time_derivative", [x])
fn latex_dot(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\dot"), multispace0).parse(input)?;
    let (input, expr) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        expression,
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    Ok((input, Expression::function("time_derivative", vec![expr])))
}

/// Parse \ddot{x} - double dot notation (second time derivative)
///
/// Example: \ddot{x} → Expression::function("second_time_derivative", [x])
fn latex_ddot(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\ddot"), multispace0).parse(input)?;
    let (input, expr) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        expression,
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    Ok((
        input,
        Expression::function("second_time_derivative", vec![expr]),
    ))
}

/// Parse \nabla operations - del operator and its combinations
///
/// Handles: \nabla \cdot (divergence), \nabla \times (curl), \nabla^2 (laplacian)
fn latex_nabla_operations(input: &str) -> IResult<&str, Expression> {
    alt((latex_laplacian, latex_divergence, latex_curl)).parse(input)
}

/// Parse \nabla^2 - Laplacian operator
///
/// Example: \nabla^2 → Expression::function("laplacian", [])
fn latex_laplacian(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\nabla^2"), multispace0).parse(input)?;
    Ok((input, Expression::function("laplacian", vec![])))
}

/// Parse \nabla \cdot - divergence operator
///
/// Example: \nabla \cdot → Expression::function("divergence", [])
fn latex_divergence(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\nabla"), multispace0).parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\cdot"), multispace0).parse(input)?;
    Ok((input, Expression::function("divergence", vec![])))
}

/// Parse \nabla \times - curl operator
///
/// Example: \nabla \times → Expression::function("curl", [])
fn latex_curl(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\nabla"), multispace0).parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\times"), multispace0).parse(input)?;
    Ok((input, Expression::function("curl", vec![])))
}

/// Parse \nabla - del/gradient operator
///
/// Example: \nabla → Expression::symbol("∇")
fn latex_nabla(input: &str) -> IResult<&str, Expression> {
    map(delimited(multispace0, tag("\\nabla"), multispace0), |_| {
        Expression::symbol("∇")
    })
    .parse(input)
}

/// Parse \partial - partial derivative symbol
///
/// Example: \partial → Expression::symbol("∂")
fn latex_partial(input: &str) -> IResult<&str, Expression> {
    map(
        delimited(multispace0, tag("\\partial"), multispace0),
        |_| Expression::symbol("∂"),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Expression;
    use nom::combinator::all_consuming;

    #[test]
    fn test_latex_vec() {
        // Test vector F
        let result = all_consuming(latex_vec).parse("\\vec{F}").unwrap().1;
        assert_eq!(
            result,
            Expression::function("vector", vec![Expression::symbol("F")])
        );

        // Test vector with expression
        let result = all_consuming(latex_vec).parse("\\vec{AB}").unwrap().1;
        assert_eq!(
            result,
            Expression::function("vector", vec![Expression::symbol("AB")])
        );
    }

    #[test]
    fn test_latex_hat() {
        // Test unit vector
        let result = all_consuming(latex_hat).parse("\\hat{x}").unwrap().1;
        assert_eq!(
            result,
            Expression::function("unit_vector", vec![Expression::symbol("x")])
        );
    }

    #[test]
    fn test_latex_nabla() {
        // Test nabla symbol
        let result = all_consuming(latex_nabla).parse("\\nabla").unwrap().1;
        assert_eq!(result, Expression::symbol("∇"));
    }

    #[test]
    fn test_latex_partial() {
        // Test partial symbol
        let result = all_consuming(latex_partial).parse("\\partial").unwrap().1;
        assert_eq!(result, Expression::symbol("∂"));
    }

    #[test]
    fn test_latex_laplacian() {
        // Test Laplacian operator
        let result = all_consuming(latex_laplacian).parse("\\nabla^2").unwrap().1;
        assert_eq!(result, Expression::function("laplacian", vec![]));
    }

    #[test]
    fn test_latex_divergence() {
        // Test divergence operator
        let result = all_consuming(latex_divergence)
            .parse("\\nabla \\cdot")
            .unwrap()
            .1;
        assert_eq!(result, Expression::function("divergence", vec![]));
    }

    #[test]
    fn test_latex_curl() {
        // Test curl operator
        let result = all_consuming(latex_curl)
            .parse("\\nabla \\times")
            .unwrap()
            .1;
        assert_eq!(result, Expression::function("curl", vec![]));
    }

    #[test]
    fn test_latex_dot() {
        // Test time derivative
        let result = all_consuming(latex_dot).parse("\\dot{x}").unwrap().1;
        assert_eq!(
            result,
            Expression::function("time_derivative", vec![Expression::symbol("x")])
        );
    }

    #[test]
    fn test_latex_ddot() {
        // Test second time derivative
        let result = all_consuming(latex_ddot).parse("\\ddot{x}").unwrap().1;
        assert_eq!(
            result,
            Expression::function("second_time_derivative", vec![Expression::symbol("x")])
        );
    }
}
