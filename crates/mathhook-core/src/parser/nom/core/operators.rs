/// Operator precedence parsing using nom combinators
///
/// Implements mathematical operator precedence using nom's fold_many0 combinator
/// for left-associative operations. This eliminates the grammar conflicts that
/// plagued the LALRPOP implementation.
///
/// Precedence hierarchy (highest to lowest):
/// 1. Parentheses: (expr)
/// 2. Subscripts/Superscripts: expr_sub, expr^sup, expr_sub^sup
/// 3. Factorial: expr!
/// 4. Power: expr^expr (right-associative)
/// 5. Unary: -expr, +expr
/// 6. Multiplication/Division: expr*expr, expr/expr
/// 7. Addition/Subtraction: expr+expr, expr-expr
/// 8. Relations: expr=expr, expr<expr, etc.
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, multispace0},
    combinator::{map, opt, recognize},
    multi::{fold_many0, separated_list0},
    sequence::{delimited, pair, preceded},
    IResult, Parser,
};
use std::ops::Neg;

use crate::core::Expression;
use crate::parser::nom::core::{numbers, variables};

/// Parse relations (lowest precedence)
///
/// Handles: =, !=, <, <=, >, >=
pub fn relation(input: &str) -> IResult<&str, Expression> {
    let (input, first) = addition(input)?;
    fold_many0(
        pair(
            delimited(
                multispace0,
                alt((
                    tag("=="),
                    tag("!="),
                    tag("<="),
                    tag(">="),
                    tag("<"),
                    tag(">"),
                    tag("="),
                )),
                multispace0,
            ),
            addition,
        ),
        move || first.clone(),
        |acc, (op, expr)| {
            use crate::core::expression::RelationType;
            match op {
                "=" | "==" => Expression::relation(acc, expr, RelationType::Equal),
                "!=" => Expression::relation(acc, expr, RelationType::NotEqual),
                "<" => Expression::relation(acc, expr, RelationType::Less),
                "<=" => Expression::relation(acc, expr, RelationType::LessEqual),
                ">" => Expression::relation(acc, expr, RelationType::Greater),
                ">=" => Expression::relation(acc, expr, RelationType::GreaterEqual),
                _ => unreachable!("Invalid relation operator: {}", op),
            }
        },
    )
    .parse(input)
}

/// Parse addition and subtraction (left-associative)
///
/// Handles: expr + expr, expr - expr
pub fn addition(input: &str) -> IResult<&str, Expression> {
    let (input, first) = multiplication(input)?;
    fold_many0(
        pair(
            delimited(multispace0, alt((tag("+"), tag("-"))), multispace0),
            multiplication,
        ),
        move || first.clone(),
        |acc, (op, expr)| match op {
            "+" => Expression::add(vec![acc, expr]),
            "-" => Expression::add(vec![acc, expr.neg()]),
            _ => unreachable!("Invalid addition operator: {}", op),
        },
    )
    .parse(input)
}

/// Parse multiplication and division (left-associative)
///
/// Handles: expr * expr, expr / expr, expr \cdot expr, expr \times expr, expr \div expr, 2x, 3sin(x)
pub fn multiplication(input: &str) -> IResult<&str, Expression> {
    // First try implicit multiplication, then fall back to normal parsing
    let (input, first) =
        alt((crate::parser::nom::core::implicit_multiplication, power)).parse(input)?;
    fold_many0(
        pair(
            delimited(
                multispace0,
                alt((
                    tag("\\cdot"),  // LaTeX dot multiplication
                    tag("\\times"), // LaTeX times multiplication
                    tag("\\div"),   // LaTeX division
                    tag("*"),       // Standard multiplication
                    tag("/"),       // Standard division
                )),
                multispace0,
            ),
            power,
        ),
        move || first.clone(),
        |acc, (op, expr)| match op {
            "*" | "\\cdot" | "\\times" => Expression::mul(vec![acc, expr]),
            "/" | "\\div" => {
                Expression::mul(vec![acc, Expression::pow(expr, Expression::integer(-1))])
            }
            _ => unreachable!("Invalid multiplication operator: {}", op),
        },
    )
    .parse(input)
}

/// Parse exponentiation (right-associative)
///
/// Handles: expr ^ expr
/// Note: Right-associativity means a^b^c = a^(b^c)
pub fn power(input: &str) -> IResult<&str, Expression> {
    let (input, base) = subscript_superscript(input)?;
    let (input, exponent) = opt(preceded(
        delimited(multispace0, tag("^"), multispace0),
        power,
    ))
    .parse(input)?;

    match exponent {
        Some(exp) => Ok((input, Expression::pow(base, exp))),
        None => Ok((input, base)),
    }
}

/// Parse subscripts and superscripts
///
/// Handles: expr_sub, expr^sup, expr_sub^sup
/// Examples: x_1, x^2, x_1^2, H_n^{(1)}(x), P_l^m(x)
pub fn subscript_superscript(input: &str) -> IResult<&str, Expression> {
    let (input, base) = factorial(input)?;

    // Parse optional subscript
    let (input, subscript) = opt(preceded(
        tag("_"),
        alt((
            // Braced subscript: _{expr}
            delimited(tag("{"), relation, tag("}")),
            // Simple subscript: _x or _1
            alt((numbers::number, variables::variable)),
        )),
    ))
    .parse(input)?;

    // Parse optional superscript
    let (input, superscript) = opt(preceded(
        tag("^"),
        alt((
            // Braced superscript: ^{expr}
            delimited(tag("{"), relation, tag("}")),
            // Simple superscript: ^x or ^2
            alt((numbers::number, variables::variable)),
        )),
    ))
    .parse(input)?;

    // Build the expression based on what we found
    let result = match (subscript, superscript) {
        (None, None) => base,
        (Some(sub), None) => Expression::function("subscript", vec![base, sub]),
        (None, Some(sup)) => Expression::pow(base, sup),
        (Some(sub), Some(sup)) => {
            Expression::function("subscript_superscript", vec![base, sub, sup])
        }
    };

    Ok((input, result))
}

/// Parse factorial (postfix)
///
/// Handles: expr!
pub fn factorial(input: &str) -> IResult<&str, Expression> {
    let (input, expr) = unary(input)?;
    let (input, has_factorial) = opt(delimited(multispace0, tag("!"), multispace0)).parse(input)?;

    match has_factorial {
        Some(_) => Ok((input, Expression::function("factorial", vec![expr]))),
        None => Ok((input, expr)),
    }
}

/// Parse unary operators (prefix)
///
/// Handles: -expr, +expr
pub fn unary(input: &str) -> IResult<&str, Expression> {
    alt((
        map(
            preceded(delimited(multispace0, tag("-"), multispace0), unary),
            |expr| expr.neg(),
        ),
        map(
            preceded(delimited(multispace0, tag("+"), multispace0), unary),
            |expr| expr,
        ), // Unary plus is identity
        atom,
    ))
    .parse(input)
}

/// Parse atomic expressions (highest precedence)
///
/// Handles: numbers, variables, parenthesized expressions, function calls
pub fn atom(input: &str) -> IResult<&str, Expression> {
    delimited(
        multispace0,
        alt((
            parenthesized,
            function_call,
            numbers::number,
            variables::variable,
        )),
        multispace0,
    )
    .parse(input)
}

/// Parse parenthesized expressions
///
/// Handles: (expr)
fn parenthesized(input: &str) -> IResult<&str, Expression> {
    delimited(
        delimited(multispace0, tag("("), multispace0),
        relation, // Full expression inside parentheses
        delimited(multispace0, tag(")"), multispace0),
    )
    .parse(input)
}

/// Parse function calls
///
/// Handles: func(arg1, arg2, ...)
fn function_call(input: &str) -> IResult<&str, Expression> {
    let (input, func_name) = delimited(
        multispace0,
        recognize(pair(alpha1, alphanumeric0)),
        multispace0,
    )
    .parse(input)?;
    let (input, args) = delimited(
        delimited(multispace0, tag("("), multispace0),
        separated_list0(delimited(multispace0, tag(","), multispace0), relation),
        delimited(multispace0, tag(")"), multispace0),
    )
    .parse(input)?;

    Ok((input, Expression::function(func_name, args)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        // Addition
        let result = relation("1 + 2");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '1 + 2': {:?}", expr);

        // Subtraction
        let result = relation("5 - 3");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '5 - 3': {:?}", expr);

        // Multiplication
        let result = relation("2 * 3");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '2 * 3': {:?}", expr);

        // Division
        let result = relation("6 / 2");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '6 / 2': {:?}", expr);
    }

    #[test]
    fn test_operator_precedence() {
        // Multiplication before addition: 2 + 3 * 4 = 2 + (3 * 4) = 14
        let result = relation("2 + 3 * 4");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '2 + 3 * 4': {:?}", expr);

        // Power before multiplication: 2 * 3 ^ 2 = 2 * (3 ^ 2) = 18
        let result = relation("2 * 3 ^ 2");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '2 * 3 ^ 2': {:?}", expr);
    }

    #[test]
    fn test_associativity() {
        // Left-associative addition: 1 + 2 + 3 = ((1 + 2) + 3)
        let result = relation("1 + 2 + 3");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '1 + 2 + 3': {:?}", expr);

        // Right-associative power: 2 ^ 3 ^ 2 = 2 ^ (3 ^ 2) = 512
        let result = relation("2 ^ 3 ^ 2");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '2 ^ 3 ^ 2': {:?}", expr);
    }

    #[test]
    fn test_parentheses() {
        // Parentheses override precedence: (2 + 3) * 4 = 20
        let result = relation("(2 + 3) * 4");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '(2 + 3) * 4': {:?}", expr);
    }

    #[test]
    fn test_unary_operators() {
        // Unary minus
        let result = relation("-5");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '-5': {:?}", expr);

        // Unary plus
        let result = relation("+5");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '+5': {:?}", expr);

        // Complex unary: -(2 + 3)
        let result = relation("-(2 + 3)");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '-(2 + 3)': {:?}", expr);
    }

    #[test]
    fn test_subscript_superscript() {
        // Simple subscript
        let result = relation("x_1");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed 'x_1': {:?}", expr);

        // Simple superscript (should be power)
        let result = relation("x^2");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed 'x^2': {:?}", expr);

        // Both subscript and superscript
        let result = relation("x_1^2");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed 'x_1^2': {:?}", expr);

        // Braced subscript
        let result = relation("H_{n+1}");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed 'H_{{n+1}}': {:?}", expr);

        // Complex pattern
        let result = relation("P_l^m");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed 'P_l^m': {:?}", expr);
    }

    #[test]
    fn test_factorial() {
        // Simple factorial
        let result = relation("5!");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '5!': {:?}", expr);

        // Factorial with parentheses
        let result = relation("(3 + 2)!");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '(3 + 2)!': {:?}", expr);
    }

    #[test]
    fn test_function_calls() {
        // Simple function call
        let result = relation("sin(x)");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed 'sin(x)': {:?}", expr);

        // Function with multiple arguments
        let result = relation("max(1, 2, 3)");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed 'max(1, 2, 3)': {:?}", expr);
    }

    #[test]
    fn test_relations() {
        // Equality
        let result = relation("x = 5");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed 'x = 5': {:?}", expr);

        // Inequality
        let result = relation("x < y");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed 'x < y': {:?}", expr);
    }

    #[test]
    fn test_complex_expressions() {
        // Complex expression with multiple operators
        let result = relation("2 * x + 3 * y^2 - 1");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed '2 * x + 3 * y^2 - 1': {:?}", expr);

        // Expression with function and operators
        let result = relation("sin(x + 1) * cos(y)");
        assert!(result.is_ok());
        let (remaining, expr) = result.unwrap();
        assert_eq!(remaining, "");
        println!("Parsed 'sin(x + 1) * cos(y)': {:?}", expr);
    }
}
