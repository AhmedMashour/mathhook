/// LaTeX mathematical notation parsing using nom combinators
///
/// This module implements LaTeX-specific parsing for mathematical expressions,
/// including commands, functions, and special notation.
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

use crate::core::Expression;
use crate::parser::nom::core::expression;
use crate::parser::nom::shared::constants::parse_latex_constants;

// Import LaTeX submodules
mod delimiters;
mod fonts;
mod vectors;

use delimiters::latex_delimiters;
use fonts::latex_font_commands;
use vectors::latex_vector_operators;

/// Parse LaTeX mathematical expressions
///
/// This is the main entry point for LaTeX parsing, handling both LaTeX commands
/// and regular mathematical expressions.
pub fn latex_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        latex_command,
        expression, // Fall back to regular expression parsing
    ))
    .parse(input)
}

/// Parse LaTeX commands
///
/// Handles LaTeX commands like \frac{}{}, \sin(), \sqrt{}, etc.
fn latex_command(input: &str) -> IResult<&str, Expression> {
    alt((
        latex_calculus, // PRIORITY: Parse calculus commands first (\int, \sum, \lim, etc.)
        latex_subscript_superscript, // Add subscript/superscript support
        latex_font_commands, // Mathematical fonts (\mathcal, \mathbf, \text, etc.)
        latex_vector_operators, // Vector operations (\vec, \nabla, \partial, etc.)
        latex_delimiters, // Delimiters (\left, \right, \langle, \rangle, etc.)
        latex_fraction,
        latex_sqrt,
        latex_matrices,
        latex_sets,
        latex_trig_functions,
        latex_log_functions,
        parse_latex_constants, // Parse constants AFTER calculus to avoid \i matching \int
        latex_operators,
    ))
    .parse(input)
}

/// Parse LaTeX fractions: \frac{numerator}{denominator}
fn latex_fraction(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\frac"), multispace0).parse(input)?;
    let (input, numerator) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        latex_expression,
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;
    let (input, denominator) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        latex_expression,
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    Ok((
        input,
        Expression::mul(vec![
            numerator,
            Expression::pow(denominator, Expression::integer(-1)),
        ]),
    ))
}

/// Parse LaTeX square root: \sqrt{expression}
fn latex_sqrt(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\sqrt"), multispace0).parse(input)?;
    let (input, expr) = delimited(
        delimited(multispace0, tag("{"), multispace0),
        latex_expression,
        delimited(multispace0, tag("}"), multispace0),
    )
    .parse(input)?;

    Ok((input, Expression::pow(expr, Expression::rational(1, 2))))
}

/// Parse LaTeX trigonometric functions: \sin(x), \cos(x), \tan(x)
fn latex_trig_functions(input: &str) -> IResult<&str, Expression> {
    let (input, func_name) = delimited(
        multispace0,
        alt((
            tag("\\sin"),
            tag("\\cos"),
            tag("\\tan"),
            tag("\\sec"),
            tag("\\csc"),
            tag("\\cot"),
            tag("\\arcsin"),
            tag("\\arccos"),
            tag("\\arctan"),
            tag("\\sinh"),
            tag("\\cosh"),
            tag("\\tanh"),
        )),
        multispace0,
    )
    .parse(input)?;

    // Handle both \sin(x) and \sin x formats
    let (input, arg) = alt((
        // \sin(expression)
        delimited(
            delimited(multispace0, tag("("), multispace0),
            latex_expression,
            delimited(multispace0, tag(")"), multispace0),
        ),
        // \sin expression (without parentheses)
        preceded(multispace0, latex_expression),
    ))
    .parse(input)?;

    // Remove the backslash from function name
    let clean_name = &func_name[1..];
    Ok((input, Expression::function(clean_name, vec![arg])))
}

/// Parse LaTeX logarithmic functions: \ln(x), \log(x)
fn latex_log_functions(input: &str) -> IResult<&str, Expression> {
    let (input, func_name) =
        delimited(multispace0, alt((tag("\\ln"), tag("\\log"))), multispace0).parse(input)?;

    // Handle both \ln(x) and \ln x formats
    let (input, arg) = alt((
        // \ln(expression)
        delimited(
            delimited(multispace0, tag("("), multispace0),
            latex_expression,
            delimited(multispace0, tag(")"), multispace0),
        ),
        // \ln expression (without parentheses)
        preceded(multispace0, latex_expression),
    ))
    .parse(input)?;

    // Remove the backslash from function name
    let clean_name = &func_name[1..];
    Ok((input, Expression::function(clean_name, vec![arg])))
}

/// Parse LaTeX mathematical constants and Greek letters
fn latex_constants(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_latex_constants, // Shared mathematical constants
        latex_greek_lowercase,
        latex_greek_uppercase,
    ))
    .parse(input)
}

/// Parse lowercase Greek letters (most commonly used)
fn latex_greek_lowercase(input: &str) -> IResult<&str, Expression> {
    alt((
        // Variants first (longer patterns)
        map(tag("\\varepsilon"), |_| Expression::symbol("ε")),
        map(tag("\\vartheta"), |_| Expression::symbol("ϑ")),
        map(tag("\\varphi"), |_| Expression::symbol("ϕ")),
        // Common Greek letters
        map(tag("\\alpha"), |_| Expression::symbol("α")),
        map(tag("\\beta"), |_| Expression::symbol("β")),
        map(tag("\\gamma"), |_| Expression::symbol("γ")),
        map(tag("\\delta"), |_| Expression::symbol("δ")),
        map(tag("\\epsilon"), |_| Expression::symbol("ε")),
        map(tag("\\theta"), |_| Expression::symbol("θ")),
        map(tag("\\lambda"), |_| Expression::symbol("λ")),
        map(tag("\\mu"), |_| Expression::symbol("μ")),
        map(tag("\\nu"), |_| Expression::symbol("ν")),
        map(tag("\\rho"), |_| Expression::symbol("ρ")),
        map(tag("\\sigma"), |_| Expression::symbol("σ")),
        map(tag("\\tau"), |_| Expression::symbol("τ")),
        map(tag("\\phi"), |_| Expression::symbol("φ")),
        map(tag("\\chi"), |_| Expression::symbol("χ")),
        map(tag("\\psi"), |_| Expression::symbol("ψ")),
        map(tag("\\omega"), |_| Expression::symbol("ω")),
    ))
    .parse(input)
}

/// Parse uppercase Greek letters (most commonly used)
fn latex_greek_uppercase(input: &str) -> IResult<&str, Expression> {
    alt((
        map(tag("\\Gamma"), |_| Expression::symbol("Γ")),
        map(tag("\\Delta"), |_| Expression::symbol("Δ")),
        map(tag("\\Theta"), |_| Expression::symbol("Θ")),
        map(tag("\\Lambda"), |_| Expression::symbol("Λ")),
        map(tag("\\Xi"), |_| Expression::symbol("Ξ")),
        map(tag("\\Pi"), |_| Expression::symbol("Π")),
        map(tag("\\Sigma"), |_| Expression::symbol("Σ")),
        map(tag("\\Phi"), |_| Expression::symbol("Φ")),
        map(tag("\\Psi"), |_| Expression::symbol("Ψ")),
        map(tag("\\Omega"), |_| Expression::symbol("Ω")),
    ))
    .parse(input)
}

/// Parse LaTeX mathematical operators and symbols
fn latex_operators(input: &str) -> IResult<&str, Expression> {
    alt((
        latex_arithmetic_operators,
        latex_comparison_operators,
        latex_set_operators,
        latex_logic_operators,
        latex_arrow_operators,
        latex_misc_symbols,
    ))
    .parse(input)
}

/// Parse arithmetic operators (symbols only, not operators)
fn latex_arithmetic_operators(input: &str) -> IResult<&str, Expression> {
    alt((
        // Note: \cdot, \times, \div are now handled in core multiplication parser
        map(tag("\\pm"), |_| Expression::symbol("±")),
        map(tag("\\mp"), |_| Expression::symbol("∓")),
    ))
    .parse(input)
}

/// Parse comparison operators
fn latex_comparison_operators(input: &str) -> IResult<&str, Expression> {
    alt((
        map(tag("\\leq"), |_| Expression::symbol("≤")),
        map(tag("\\geq"), |_| Expression::symbol("≥")),
        map(tag("\\neq"), |_| Expression::symbol("≠")),
        map(tag("\\approx"), |_| Expression::symbol("≈")),
        map(tag("\\equiv"), |_| Expression::symbol("≡")),
        map(tag("\\sim"), |_| Expression::symbol("∼")),
        map(tag("\\simeq"), |_| Expression::symbol("≃")),
        map(tag("\\cong"), |_| Expression::symbol("≅")),
        map(tag("\\propto"), |_| Expression::symbol("∝")),
    ))
    .parse(input)
}

/// Parse set theory operators
fn latex_set_operators(input: &str) -> IResult<&str, Expression> {
    alt((
        map(tag("\\subseteq"), |_| Expression::symbol("⊆")), // Longer first
        map(tag("\\supseteq"), |_| Expression::symbol("⊇")),
        map(tag("\\emptyset"), |_| Expression::symbol("∅")),
        map(tag("\\notin"), |_| Expression::symbol("∉")),
        map(tag("\\subset"), |_| Expression::symbol("⊂")),
        map(tag("\\supset"), |_| Expression::symbol("⊃")),
        map(tag("\\cup"), |_| Expression::symbol("∪")),
        map(tag("\\cap"), |_| Expression::symbol("∩")),
        map(tag("\\in"), |_| Expression::symbol("∈")),
    ))
    .parse(input)
}

/// Parse logic operators
fn latex_logic_operators(input: &str) -> IResult<&str, Expression> {
    alt((
        map(tag("\\implies"), |_| Expression::symbol("⟹")),
        map(tag("\\forall"), |_| Expression::symbol("∀")),
        map(tag("\\exists"), |_| Expression::symbol("∃")),
        map(tag("\\land"), |_| Expression::symbol("∧")),
        map(tag("\\lnot"), |_| Expression::symbol("¬")),
        map(tag("\\lor"), |_| Expression::symbol("∨")),
        map(tag("\\neg"), |_| Expression::symbol("¬")),
        map(tag("\\iff"), |_| Expression::symbol("⟺")),
    ))
    .parse(input)
}

/// Parse arrow operators
fn latex_arrow_operators(input: &str) -> IResult<&str, Expression> {
    alt((
        map(tag("\\leftrightarrow"), |_| Expression::symbol("↔")), // Longer first
        map(tag("\\Leftrightarrow"), |_| Expression::symbol("⇔")),
        map(tag("\\rightarrow"), |_| Expression::symbol("→")),
        map(tag("\\Rightarrow"), |_| Expression::symbol("⇒")),
        map(tag("\\leftarrow"), |_| Expression::symbol("←")),
        map(tag("\\Leftarrow"), |_| Expression::symbol("⇐")),
    ))
    .parse(input)
}

/// Parse miscellaneous symbols
fn latex_misc_symbols(input: &str) -> IResult<&str, Expression> {
    alt((
        map(tag("\\partial"), |_| Expression::symbol("∂")),
        map(tag("\\nabla"), |_| Expression::symbol("∇")),
        map(tag("\\hbar"), |_| Expression::symbol("ℏ")),
        map(tag("\\ell"), |_| Expression::symbol("ℓ")),
        map(tag("\\Re"), |_| Expression::symbol("ℜ")),
        map(tag("\\Im"), |_| Expression::symbol("ℑ")),
        map(tag("\\aleph"), |_| Expression::symbol("ℵ")),
    ))
    .parse(input)
}

/// Parse LaTeX calculus expressions: \int, \sum, \prod, \lim, \partial
fn latex_calculus(input: &str) -> IResult<&str, Expression> {
    alt((
        latex_multiple_integrals, // Multiple integrals: \iint, \iiint, \oint
        latex_integral,
        latex_sum,
        latex_product,
        latex_limit,
        latex_derivative,
        latex_partial,
        latex_binomial,  // Binomial coefficients: \binom{n}{k}
        latex_factorial, // Factorials: n!, n!!
    ))
    .parse(input)
}

/// Parse LaTeX integrals: \int, \int_a^b
fn latex_integral(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\int"), multispace0).parse(input)?;

    // Try to parse bounds: \int_a^b, \int_{a}^{b}, \int^b_a, or \int^{b}_{a}
    let (input, bounds) = opt(alt((
        // \int_a^b format (with or without braces)
        pair(
            preceded(
                tag("_"),
                alt((
                    delimited(tag("{"), expression, tag("}")),
                    expression, // Allow bounds without braces
                )),
            ),
            preceded(
                tag("^"),
                alt((
                    delimited(tag("{"), expression, tag("}")),
                    expression, // Allow bounds without braces
                )),
            ),
        ),
        // \int^b_a format (with or without braces)
        pair(
            preceded(
                tag("^"),
                alt((
                    delimited(tag("{"), expression, tag("}")),
                    expression, // Allow bounds without braces
                )),
            ),
            preceded(
                tag("_"),
                alt((
                    delimited(tag("{"), expression, tag("}")),
                    expression, // Allow bounds without braces
                )),
            ),
        ),
    )))
    .parse(input)?;

    let (lower_bound, upper_bound) =
        bounds.unwrap_or((Expression::symbol("a"), Expression::symbol("b")));

    let (input, integrand) = delimited(multispace0, expression, multispace0).parse(input)?;
    let (input, _) = opt(delimited(multispace0, tag("d"), multispace0)).parse(input)?;
    let (input, variable) = opt(delimited(
        multispace0,
        crate::parser::nom::core::variables::variable,
        multispace0,
    ))
    .map(|opt| opt.unwrap_or(Expression::symbol("x")))
    .parse(input)?;

    Ok((
        input,
        Expression::function(
            "integrate",
            vec![integrand, variable, lower_bound, upper_bound],
        ),
    ))
}

/// Parse LaTeX sums: \sum, \sum_{i=1}^n
fn latex_sum(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\sum"), multispace0).parse(input)?;

    // Try to parse bounds: \sum_{i=1}^n
    let (input, (index, start, end)) = opt((
        preceded(
            tag("_{"),
            delimited(
                multispace0,
                crate::parser::nom::core::variables::variable,
                multispace0,
            ),
        ),
        preceded(tag("="), delimited(multispace0, expression, multispace0)),
        preceded(
            tag("}^"),
            alt((
                delimited(tag("{"), expression, tag("}")),
                expression, // Allow upper bound without braces
            )),
        ),
    ))
    .map(|opt| {
        opt.unwrap_or((
            Expression::symbol("i"),
            Expression::integer(1),
            Expression::symbol("n"),
        ))
    })
    .parse(input)?;

    let (input, expr) = delimited(multispace0, expression, multispace0).parse(input)?;

    Ok((
        input,
        Expression::function("sum", vec![expr, index, start, end]),
    ))
}

/// Parse LaTeX products: \prod, \prod_{i=1}^n
fn latex_product(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\prod"), multispace0).parse(input)?;

    // Similar to sum but for products
    let (input, (index, start, end)) = opt((
        preceded(
            tag("_{"),
            delimited(
                multispace0,
                crate::parser::nom::core::variables::variable,
                multispace0,
            ),
        ),
        preceded(
            tag("="),
            delimited(multispace0, latex_expression, multispace0),
        ),
        preceded(
            tag("}^{"),
            delimited(multispace0, latex_expression, tag("}")),
        ),
    ))
    .map(|opt| {
        opt.unwrap_or((
            Expression::symbol("i"),
            Expression::integer(1),
            Expression::symbol("n"),
        ))
    })
    .parse(input)?;

    let (input, expr) = delimited(multispace0, latex_expression, multispace0).parse(input)?;

    Ok((
        input,
        Expression::function("product", vec![expr, index, start, end]),
    ))
}

/// Parse LaTeX limits: \lim_{x \to a}
fn latex_limit(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\lim"), multispace0).parse(input)?;

    // Parse \lim_{x \to a}
    let (input, limit_spec) = opt(pair(
        preceded(
            tag("_{"),
            delimited(
                multispace0,
                crate::parser::nom::core::variables::variable,
                multispace0,
            ),
        ),
        preceded(
            tag("\\to"),
            delimited(multispace0, latex_expression, tag("}")),
        ),
    ))
    .parse(input)?;

    let (variable, approach) =
        limit_spec.unwrap_or((Expression::symbol("x"), Expression::integer(0)));

    let (input, expr) = delimited(multispace0, latex_expression, multispace0).parse(input)?;

    Ok((
        input,
        Expression::function("limit", vec![expr, variable, approach]),
    ))
}

/// Parse LaTeX derivatives: \frac{d}{dx}
fn latex_derivative(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\frac{d}{d"), multispace0).parse(input)?;
    let (input, variable) = delimited(
        multispace0,
        crate::parser::nom::core::variables::variable,
        multispace0,
    )
    .parse(input)?;
    let (input, _) = delimited(multispace0, tag("}"), multispace0).parse(input)?;
    let (input, expr) = delimited(multispace0, latex_expression, multispace0).parse(input)?;

    Ok((
        input,
        Expression::function("derivative", vec![expr, variable]),
    ))
}

/// Parse LaTeX partial derivatives: \partial
fn latex_partial(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\partial"), multispace0).parse(input)?;
    let (input, expr) = delimited(multispace0, latex_expression, multispace0).parse(input)?;

    Ok((input, Expression::function("partial", vec![expr])))
}

/// Parse LaTeX matrices: \begin{pmatrix}...\end{pmatrix}
fn latex_matrices(input: &str) -> IResult<&str, Expression> {
    alt((latex_pmatrix, latex_bmatrix, latex_vmatrix)).parse(input)
}

/// Parse parenthesis matrices: \begin{pmatrix}...\end{pmatrix}
fn latex_pmatrix(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\begin{pmatrix}"), multispace0).parse(input)?;
    let (input, rows) = separated_list0(
        delimited(multispace0, tag("\\\\"), multispace0),
        separated_list0(
            delimited(multispace0, tag("&"), multispace0),
            latex_expression,
        ),
    )
    .parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\end{pmatrix}"), multispace0).parse(input)?;

    let matrix_elements: Vec<Expression> = rows.into_iter().flatten().collect();
    Ok((input, Expression::function("matrix", matrix_elements)))
}

/// Parse bracket matrices: \begin{bmatrix}...\end{bmatrix}
fn latex_bmatrix(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\begin{bmatrix}"), multispace0).parse(input)?;
    let (input, rows) = separated_list0(
        delimited(multispace0, tag("\\\\"), multispace0),
        separated_list0(
            delimited(multispace0, tag("&"), multispace0),
            latex_expression,
        ),
    )
    .parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\end{bmatrix}"), multispace0).parse(input)?;

    let matrix_elements: Vec<Expression> = rows.into_iter().flatten().collect();
    Ok((input, Expression::function("matrix", matrix_elements)))
}

/// Parse determinant matrices: \begin{vmatrix}...\end{vmatrix}
fn latex_vmatrix(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\begin{vmatrix}"), multispace0).parse(input)?;
    let (input, rows) = separated_list0(
        delimited(multispace0, tag("\\\\"), multispace0),
        separated_list0(
            delimited(multispace0, tag("&"), multispace0),
            latex_expression,
        ),
    )
    .parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\end{vmatrix}"), multispace0).parse(input)?;

    let matrix_elements: Vec<Expression> = rows.into_iter().flatten().collect();
    Ok((
        input,
        Expression::function(
            "determinant",
            vec![Expression::function("matrix", matrix_elements)],
        ),
    ))
}

/// Parse LaTeX sets: \{...\}
fn latex_sets(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("\\{"), multispace0).parse(input)?;
    let (input, elements) = separated_list0(
        delimited(multispace0, tag(","), multispace0),
        latex_expression,
    )
    .parse(input)?;
    let (input, _) = delimited(multispace0, tag("\\}"), multispace0).parse(input)?;

    Ok((input, Expression::function("set", elements)))
}

/// Parse LaTeX subscripts and superscripts
///
/// Handles: \alpha_1, \beta^{(n)}, \gamma_{i,j}^{(k)}, P_l^m(x)
/// Examples: \alpha_1, \beta^2, \gamma_{i+j}, P_l^m(\theta, \phi)
fn latex_subscript_superscript(input: &str) -> IResult<&str, Expression> {
    // Parse the base (LaTeX constant or regular expression)
    let (input, base) = alt((
        latex_constants,
        crate::parser::nom::core::variables::variable,
    ))
    .parse(input)?;

    // Parse optional subscript
    let (input, subscript) = opt(preceded(
        tag("_"),
        alt((
            // Braced subscript: _{expr}
            delimited(tag("{"), latex_expression, tag("}")),
            // Simple subscript: _x or _1
            alt((
                crate::parser::nom::core::numbers::number,
                crate::parser::nom::core::variables::variable,
                latex_constants,
            )),
        )),
    ))
    .parse(input)?;

    // Parse optional superscript
    let (input, superscript) = opt(preceded(
        tag("^"),
        alt((
            // Braced superscript: ^{expr}
            delimited(tag("{"), latex_expression, tag("}")),
            // Simple superscript: ^x or ^2
            alt((
                crate::parser::nom::core::numbers::number,
                crate::parser::nom::core::variables::variable,
                latex_constants,
            )),
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

/// Parse implicit multiplication: 2x, 3sin(x), etc.
///
/// This is a complex parser that handles cases where multiplication is implied
/// without an explicit operator.
pub fn implicit_multiplication(input: &str) -> IResult<&str, Expression> {
    // Try to parse a number followed by a variable or function
    let (input, first) = alt((
        crate::parser::nom::core::numbers::number,
        crate::parser::nom::core::variables::variable,
    ))
    .parse(input)?;

    // Look for additional terms that should be multiplied
    let (input, rest) = nom::multi::many0(alt((
        crate::parser::nom::core::variables::variable,
        latex_command,
        delimited(
            delimited(multispace0, tag("("), multispace0),
            latex_expression,
            delimited(multispace0, tag(")"), multispace0),
        ),
    )))
    .parse(input)?;

    if rest.is_empty() {
        Ok((input, first))
    } else {
        let mut terms = vec![first];
        terms.extend(rest);
        Ok((input, Expression::mul(terms)))
    }
}

/// Parse LaTeX multiple integrals: \iint, \iiint, \oint
///
/// Handles double integrals, triple integrals, and contour integrals.
/// Examples: \iint_D f(x,y) dx dy, \iiint_V f(x,y,z) dx dy dz, \oint_C f(z) dz
fn latex_multiple_integrals(input: &str) -> IResult<&str, Expression> {
    alt((
        // Triple integral: \iiint
        map(
            preceded(
                tag("\\iiint"),
                tuple((
                    opt(preceded(
                        tag("_"),
                        delimited(tag("{"), expression, tag("}")),
                    )),
                    delimited(multispace0, expression, multispace0),
                    preceded(tag("d"), expression),
                    preceded(tag("d"), expression),
                    preceded(tag("d"), expression),
                )),
            ),
            |(domain, integrand, var1, var2, var3)| {
                Expression::function(
                    "triple_integral",
                    vec![
                        integrand,
                        var1,
                        var2,
                        var3,
                        domain.unwrap_or(Expression::symbol("V")),
                    ],
                )
            },
        ),
        // Double integral: \iint
        map(
            preceded(
                tag("\\iint"),
                tuple((
                    opt(preceded(
                        tag("_"),
                        delimited(tag("{"), expression, tag("}")),
                    )),
                    delimited(multispace0, expression, multispace0),
                    preceded(tag("d"), expression),
                    preceded(tag("d"), expression),
                )),
            ),
            |(domain, integrand, var1, var2)| {
                Expression::function(
                    "double_integral",
                    vec![
                        integrand,
                        var1,
                        var2,
                        domain.unwrap_or(Expression::symbol("D")),
                    ],
                )
            },
        ),
        // Contour integral: \oint
        map(
            preceded(
                tag("\\oint"),
                tuple((
                    opt(preceded(
                        tag("_"),
                        delimited(tag("{"), expression, tag("}")),
                    )),
                    delimited(multispace0, expression, multispace0),
                    preceded(tag("d"), expression),
                )),
            ),
            |(contour, integrand, var)| {
                Expression::function(
                    "contour_integral",
                    vec![integrand, var, contour.unwrap_or(Expression::symbol("C"))],
                )
            },
        ),
    ))
    .parse(input)
}

/// Parse LaTeX binomial coefficients: \binom{n}{k}, \choose
///
/// Handles both \binom{n}{k} and n \choose k syntax.
/// Examples: \binom{n}{k}, 5 \choose 3
fn latex_binomial(input: &str) -> IResult<&str, Expression> {
    alt((
        // \binom{n}{k} syntax
        map(
            preceded(
                tag("\\binom"),
                delimited(
                    tag("{"),
                    separated_pair(expression, tag("}{"), expression),
                    tag("}"),
                ),
            ),
            |(n, k)| Expression::function("binomial", vec![n, k]),
        ),
        // n \choose k syntax
        map(
            tuple((
                expression,
                delimited(multispace0, tag("\\choose"), multispace0),
                expression,
            )),
            |(n, _, k)| Expression::function("binomial", vec![n, k]),
        ),
    ))
    .parse(input)
}

/// Parse LaTeX factorials: n!, n!!
///
/// Handles both single and double factorials.
/// Examples: 5!, n!!, (n+1)!
fn latex_factorial(input: &str) -> IResult<&str, Expression> {
    alt((
        // Double factorial: n!!
        map(terminated(expression, tag("!!")), |expr| {
            Expression::function("double_factorial", vec![expr])
        }),
        // Single factorial: n!
        map(terminated(expression, tag("!")), |expr| {
            Expression::function("factorial", vec![expr])
        }),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Symbol;
    use nom::combinator::all_consuming;

    #[test]
    fn test_simple_latex_parsing() {
        // Test basic LaTeX parsing
        let result = latex_expression("\\frac{1}{2}");
        println!("Parsing \\frac{{1}}{{2}}: {:?}", result);

        let result = latex_expression("\\int x dx");
        println!("Parsing \\int x dx: {:?}", result);

        let result = latex_expression("\\cdot");
        println!("Parsing \\cdot: {:?}", result);

        // Test individual components
        println!("Testing latex_integral directly:");
        let result = latex_integral("\\int x dx");
        println!("latex_integral(\\int x dx): {:?}", result);

        println!("Testing latex_constants directly:");
        let result = latex_constants("\\int x dx");
        println!("latex_constants(\\int x dx): {:?}", result);

        let result = latex_constants("\\i");
        println!("latex_constants(\\i): {:?}", result);

        println!("Testing latex_command directly:");
        let result = latex_command("\\int x dx");
        println!("latex_command(\\int x dx): {:?}", result);
    }

    // Tests temporarily disabled during development
    // Will be re-enabled once basic parsing is working
}
