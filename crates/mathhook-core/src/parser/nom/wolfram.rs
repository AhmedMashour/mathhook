/// Wolfram Language mathematical notation parsing using nom combinators
///
/// This module implements high-performance parsing for Wolfram Language expressions,
/// focusing on the characteristic bracket syntax: Function[arg1, arg2, ...]
///
/// Architecture priorities:
/// - Performance: Optimized parsers with minimal allocations
/// - Readability: Clear, self-documenting parser combinators
/// - Maintainability: Modular design with focused responsibilities
/// - Memory efficiency: Zero-copy parsing where possible
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric0, multispace0},
    combinator::recognize,
    multi::separated_list0,
    sequence::{delimited, pair},
    IResult, Parser,
};

use crate::core::Expression;
use crate::parser::nom::core::expression;
use crate::parser::nom::shared::constants::parse_wolfram_constants;

/// Parse Wolfram mathematical expressions
///
/// Entry point for Wolfram Language parsing. Handles the characteristic
/// bracket syntax and converts to our internal Expression representation.
pub fn wolfram_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        wolfram_function,        // PRIORITY: Parse functions first: Sin[x], Exp[x]
        wolfram_rule,            // Parse rules: x -> 0
        wolfram_list,            // Parse lists/sets: {1, 2, 3}
        parse_wolfram_constants, // Parse Wolfram constants: Pi, E, I, GoldenRatio, etc.
        expression,              // Fall back to regular expression parsing
    ))
    .parse(input)
}

/// Parse Wolfram function calls with bracket syntax
///
/// Handles: Function[arg1, arg2, ...]
/// Examples: Sin[x], Times[x, y], Power[x, 2]
fn wolfram_function(input: &str) -> IResult<&str, Expression> {
    let (input, func_name) = delimited(
        multispace0,
        recognize(pair(alpha1, alphanumeric0)),
        multispace0,
    )
    .parse(input)?;

    let (input, args) = delimited(
        delimited(multispace0, tag("["), multispace0),
        separated_list0(
            delimited(multispace0, tag(","), multispace0),
            wolfram_expression,
        ),
        delimited(multispace0, tag("]"), multispace0),
    )
    .parse(input)?;

    // Convert Wolfram function names to our internal representation
    let expr = match func_name.to_lowercase().as_str() {
        // Arithmetic operations
        "plus" => {
            if args.len() >= 2 {
                Expression::add(args)
            } else {
                Expression::function(func_name, args)
            }
        }
        "times" => {
            if args.len() >= 2 {
                Expression::mul(args)
            } else {
                Expression::function(func_name, args)
            }
        }
        "power" => {
            if args.len() == 2 {
                Expression::pow(args[0].clone(), args[1].clone())
            } else {
                Expression::function(func_name, args)
            }
        }

        // Trigonometric functions
        "sin" | "Sin" => Expression::function("sin", args),
        "cos" | "Cos" => Expression::function("cos", args),
        "tan" | "Tan" => Expression::function("tan", args),
        "sec" | "Sec" => Expression::function("sec", args),
        "csc" | "Csc" => Expression::function("csc", args),
        "cot" | "Cot" => Expression::function("cot", args),

        // Inverse trigonometric functions
        "arcsin" | "ArcSin" => Expression::function("arcsin", args),
        "arccos" | "ArcCos" => Expression::function("arccos", args),
        "arctan" | "ArcTan" => Expression::function("arctan", args),

        // Hyperbolic functions
        "sinh" => Expression::function("sinh", args),
        "cosh" => Expression::function("cosh", args),
        "tanh" => Expression::function("tanh", args),

        // Logarithmic and exponential functions
        "log" => Expression::function("log", args),

        // Subscript and superscript functions
        "subscript" => {
            if args.len() == 2 {
                Expression::function("subscript", args)
            } else {
                Expression::function("subscript", args)
            }
        }
        "superscript" => {
            if args.len() == 2 {
                Expression::pow(args[0].clone(), args[1].clone())
            } else {
                Expression::function("superscript", args)
            }
        }
        "ln" | "Ln" => Expression::function("ln", args),
        "exp" | "Exp" => Expression::function("exp", args),
        "sqrt" => {
            if args.len() == 1 {
                Expression::pow(args[0].clone(), Expression::rational(1, 2))
            } else {
                Expression::function("sqrt", args)
            }
        }

        // Special mathematical functions
        "abs" => Expression::function("abs", args),
        "sign" => Expression::function("sign", args),
        "floor" => Expression::function("floor", args),
        "ceiling" => Expression::function("ceiling", args),
        "round" => Expression::function("round", args),
        "factorial" => Expression::function("factorial", args),

        // Calculus functions
        "d" | "D" => Expression::function("derivative", args),
        "integrate" | "Integrate" => Expression::function("integrate", args),
        "limit" | "Limit" => Expression::function("limit", args),
        "sum" | "Sum" => Expression::function("sum", args),
        "product" | "Product" => Expression::function("product", args),

        // Linear algebra functions
        "det" => Expression::function("determinant", args),
        "transpose" => Expression::function("transpose", args),
        "inverse" => Expression::function("inverse", args),
        "eigenvalues" => Expression::function("eigenvalues", args),
        "eigenvectors" => Expression::function("eigenvectors", args),

        // Number theory functions
        "gcd" => Expression::function("gcd", args),
        "lcm" => Expression::function("lcm", args),
        "mod" => Expression::function("mod", args),
        "prime" => Expression::function("prime", args),
        "factor" => Expression::function("factor", args),

        // Statistical functions
        "mean" => Expression::function("mean", args),
        "median" => Expression::function("median", args),
        "variance" => Expression::function("variance", args),
        "standarddeviation" => Expression::function("std", args),

        // Complex number functions
        "re" => Expression::function("real", args),
        "im" => Expression::function("imaginary", args),
        "conjugate" => Expression::function("conjugate", args),
        "arg" => Expression::function("argument", args),

        // Special Functions (Bessel, Legendre, etc.)
        "besselj" => Expression::function("bessel_j", args),
        "bessely" => Expression::function("bessel_y", args),
        "besseli" => Expression::function("bessel_i", args),
        "besselk" => Expression::function("bessel_k", args),
        "hankelh1" => Expression::function("hankel_h1", args),
        "hankelh2" => Expression::function("hankel_h2", args),
        "legendrep" => Expression::function("legendre_p", args),
        "legendreq" => Expression::function("legendre_q", args),
        "hermiteh" => Expression::function("hermite_h", args),
        "laguerrel" => Expression::function("laguerre_l", args),
        "chebyshevt" => Expression::function("chebyshev_t", args),
        "chebyshevu" => Expression::function("chebyshev_u", args),

        // Hypergeometric Functions
        "hypergeometric0f1" => Expression::function("hypergeometric_0f1", args),
        "hypergeometric1f1" => Expression::function("hypergeometric_1f1", args),
        "hypergeometric2f1" => Expression::function("hypergeometric_2f1", args),
        "hypergeometricpfq" => Expression::function("hypergeometric_pfq", args),
        "appellf1" => Expression::function("appell_f1", args),
        "meijerg" => Expression::function("meijer_g", args),

        // Zeta and Related Functions
        "zeta" => Expression::function("zeta", args),
        "hurwitzzeta" => Expression::function("hurwitz_zeta", args),
        "polylog" => Expression::function("polylog", args),
        "lerchphi" => Expression::function("lerch_phi", args),
        "stieltjesgamma" => Expression::function("stieltjes_gamma", args),
        "dirichleteta" => Expression::function("dirichlet_eta", args),

        // Number Theory Functions
        "jacobisymbol" => Expression::function("jacobi_symbol", args),
        "kroneckersymbol" => Expression::function("kronecker_symbol", args),
        "mobius" => Expression::function("mobius", args),
        "eulerphi" => Expression::function("euler_phi", args),
        "carmichaellambda" => Expression::function("carmichael_lambda", args),
        "divisorsigma" => Expression::function("divisor_sigma", args),
        "primenu" => Expression::function("prime_nu", args),
        "primeomega" => Expression::function("prime_omega", args),

        // Probability & Statistics
        "erf" => Expression::function("erf", args),
        "erfc" => Expression::function("erfc", args),
        "erfi" => Expression::function("erfi", args),
        "gamma" => Expression::function("gamma", args),
        "loggamma" => Expression::function("log_gamma", args),
        "polygamma" => Expression::function("polygamma", args),
        "beta" => Expression::function("beta", args),
        "logbeta" => Expression::function("log_beta", args),
        "binomial" => Expression::function("binomial", args),
        "multinomial" => Expression::function("multinomial", args),

        // Distributions
        "normaldistribution" => Expression::function("normal_distribution", args),
        "uniformdistribution" => Expression::function("uniform_distribution", args),
        "exponentialdistribution" => Expression::function("exponential_distribution", args),
        "poissondistribution" => Expression::function("poisson_distribution", args),
        "binomialdistribution" => Expression::function("binomial_distribution", args),
        "gammadistribution" => Expression::function("gamma_distribution", args),
        "betadistribution" => Expression::function("beta_distribution", args),
        "studenttdistribution" => Expression::function("student_t_distribution", args),
        "chisquaredistribution" => Expression::function("chi_square_distribution", args),
        "fdistribution" => Expression::function("f_distribution", args),

        // Transforms
        "fouriertransform" => Expression::function("fourier_transform", args),
        "inversefouriertransform" => Expression::function("inverse_fourier_transform", args),
        "laplacetransform" => Expression::function("laplace_transform", args),
        "inverselaplacetransform" => Expression::function("inverse_laplace_transform", args),
        "ztransform" => Expression::function("z_transform", args),
        "inverseztransform" => Expression::function("inverse_z_transform", args),
        "convolution" => Expression::function("convolution", args),

        // Optimization
        "minimize" => Expression::function("minimize", args),
        "maximize" => Expression::function("maximize", args),
        "argmin" => Expression::function("argmin", args),
        "argmax" => Expression::function("argmax", args),
        "findminimum" => Expression::function("find_minimum", args),
        "findmaximum" => Expression::function("find_maximum", args),

        // Advanced Calculus
        "grad" => Expression::function("gradient", args),
        "div" => Expression::function("divergence", args),
        "curl" => Expression::function("curl", args),
        "laplacian" => Expression::function("laplacian", args),

        // Spherical Functions
        "sphericalharmonicy" => Expression::function("spherical_harmonic_y", args),
        "sphericalbesselj" => Expression::function("spherical_bessel_j", args),
        "sphericalbessely" => Expression::function("spherical_bessel_y", args),

        // Elliptic Functions
        "elliptick" => Expression::function("elliptic_k", args),
        "elliptice" => Expression::function("elliptic_e", args),
        "ellipticpi" => Expression::function("elliptic_pi", args),
        "jacobisn" => Expression::function("jacobi_sn", args),
        "jacobicn" => Expression::function("jacobi_cn", args),
        "jacobidn" => Expression::function("jacobi_dn", args),

        // Polynomial Functions
        "cyclotomicpolynomial" => Expression::function("cyclotomic_polynomial", args),
        "minimalpolynomial" => Expression::function("minimal_polynomial", args),
        "resultant" => Expression::function("resultant", args),
        "discriminant" => Expression::function("discriminant", args),
        "polynomialgcd" => Expression::function("polynomial_gcd", args),
        "polynomiallcm" => Expression::function("polynomial_lcm", args),

        // Continued Fractions
        "continuedfraction" => Expression::function("continued_fraction", args),
        "convergents" => Expression::function("convergents", args),

        // Default: preserve original function name
        _ => Expression::function(func_name, args),
    };

    Ok((input, expr))
}

/// Parse Wolfram lists and sets: {1, 2, 3}, {{1, 2}, {3, 4}}
///
/// Handles both simple lists and nested structures like matrices.
/// Examples: {1, 2, 3}, {{a, b}, {c, d}}, {x, y, z}
fn wolfram_list(input: &str) -> IResult<&str, Expression> {
    let (input, _) = delimited(multispace0, tag("{"), multispace0).parse(input)?;

    // Parse list elements separated by commas
    // Use alt to try wolfram-specific parsing first, then fall back to basic parsers
    let (input, elements) = separated_list0(
        delimited(multispace0, tag(","), multispace0),
        alt((
            wolfram_rule,                                  // Parse rules: x -> 0
            wolfram_list,                                  // Parse nested lists: {1, 2}
            parse_wolfram_constants,                       // Parse Wolfram constants: Pi, E, I
            wolfram_function,                              // Parse functions: Sin[x]
            crate::parser::nom::core::numbers::number,     // Parse numbers directly
            crate::parser::nom::core::variables::variable, // Parse variables directly
        )),
    )
    .parse(input)?;

    let (input, _) = delimited(multispace0, tag("}"), multispace0).parse(input)?;

    // Check if this is a matrix (list of lists)
    let is_matrix = elements
        .iter()
        .all(|elem| matches!(elem, Expression::Function { name, .. } if name == "list"));

    if is_matrix && !elements.is_empty() {
        // Convert to matrix representation
        Ok((input, Expression::function("matrix", elements)))
    } else {
        // Regular list
        Ok((input, Expression::function("list", elements)))
    }
}

/// Parse Wolfram rules: x -> value
///
/// Handles patterns like: x -> 0, Sin[x] -> 1, etc.
/// Used in limits, substitutions, and other Wolfram constructs.
fn wolfram_rule(input: &str) -> IResult<&str, Expression> {
    let (input, left) = alt((wolfram_function, wolfram_list, expression)).parse(input)?;
    let (input, _) = delimited(multispace0, tag("->"), multispace0).parse(input)?;
    let (input, right) = wolfram_expression.parse(input)?;

    Ok((input, Expression::function("rule", vec![left, right])))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Symbol;

    #[test]
    fn test_wolfram_arithmetic_functions() {
        // Test Plus[x, y]
        let result = wolfram_function("Plus[x, y]").unwrap().1;
        let expected = Expression::add(vec![
            Expression::symbol(Symbol::new("x")),
            Expression::symbol(Symbol::new("y")),
        ]);
        assert_eq!(result, expected);

        // Test Times[2, x]
        let result = wolfram_function("Times[2, x]").unwrap().1;
        let expected = Expression::mul(vec![
            Expression::integer(2),
            Expression::symbol(Symbol::new("x")),
        ]);
        assert_eq!(result, expected);

        // Test Power[x, 2]
        let result = wolfram_function("Power[x, 2]").unwrap().1;
        let expected =
            Expression::pow(Expression::symbol(Symbol::new("x")), Expression::integer(2));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_wolfram_trig_functions() {
        // Test Sin[x]
        let result = wolfram_function("Sin[x]").unwrap().1;
        let expected = Expression::function("sin", vec![Expression::symbol(Symbol::new("x"))]);
        assert_eq!(result, expected);

        // Test Cos[Pi/2]
        let result = wolfram_function("Cos[Pi/2]").unwrap().1;
        let pi_half = Expression::mul(vec![
            Expression::pi(),
            Expression::pow(Expression::integer(2), Expression::integer(-1)),
        ]);
        let expected = Expression::function("cos", vec![pi_half]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_wolfram_sqrt_function() {
        // Test Sqrt[x]
        let result = wolfram_function("Sqrt[x]").unwrap().1;
        let expected = Expression::pow(
            Expression::symbol(Symbol::new("x")),
            Expression::rational(1, 2),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_wolfram_nested_functions() {
        // Test Sin[Cos[x]]
        let result = wolfram_function("Sin[Cos[x]]").unwrap().1;
        let inner = Expression::function("cos", vec![Expression::symbol(Symbol::new("x"))]);
        let expected = Expression::function("sin", vec![inner]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_wolfram_multiple_args() {
        // Test Log[x, 10] (logarithm base 10)
        let result = wolfram_function("Log[x, 10]").unwrap().1;
        let expected = Expression::function(
            "log",
            vec![
                Expression::symbol(Symbol::new("x")),
                Expression::integer(10),
            ],
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_wolfram_expressions() {
        // Test Power[Sin[x], 2]
        let result = wolfram_function("Power[Sin[x], 2]").unwrap().1;
        let sin_x = Expression::function("sin", vec![Expression::symbol(Symbol::new("x"))]);
        let expected = Expression::pow(sin_x, Expression::integer(2));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_wolfram_list_syntax() {
        // Test {1, 2, 3}
        let result = wolfram_list("{1, 2, 3}").unwrap().1;
        let expected = Expression::function(
            "list",
            vec![
                Expression::integer(1),
                Expression::integer(2),
                Expression::integer(3),
            ],
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_wolfram_matrix_syntax() {
        // Test {{1, 2}, {3, 4}}
        let result = wolfram_list("{{1, 2}, {3, 4}}").unwrap().1;
        let row1 =
            Expression::function("list", vec![Expression::integer(1), Expression::integer(2)]);
        let row2 =
            Expression::function("list", vec![Expression::integer(3), Expression::integer(4)]);
        let expected = Expression::function("matrix", vec![row1, row2]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_wolfram_functions() {
        // Test BesselJ function
        let result = wolfram_function("BesselJ[0, x]").unwrap().1;
        let expected = Expression::function(
            "bessel_j",
            vec![Expression::integer(0), Expression::symbol(Symbol::new("x"))],
        );
        assert_eq!(result, expected);

        // Test NormalDistribution
        let result = wolfram_function("NormalDistribution[μ, σ]").unwrap().1;
        let expected = Expression::function(
            "normal_distribution",
            vec![
                Expression::symbol(Symbol::new("μ")),
                Expression::symbol(Symbol::new("σ")),
            ],
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_wolfram_rule_syntax() {
        // Test x -> 0
        let result = wolfram_rule("x -> 0").unwrap().1;
        let expected = Expression::function(
            "rule",
            vec![Expression::symbol(Symbol::new("x")), Expression::integer(0)],
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_wolfram_expressions() {
        // Test function with list argument: Integrate[x, {x, 0, 1}]
        let result = wolfram_function("Integrate[x, {x, 0, 1}]").unwrap().1;
        let list_arg = Expression::function(
            "list",
            vec![
                Expression::symbol(Symbol::new("x")),
                Expression::integer(0),
                Expression::integer(1),
            ],
        );
        let expected = Expression::function(
            "integrate",
            vec![Expression::symbol(Symbol::new("x")), list_arg],
        );
        assert_eq!(result, expected);
    }
}
