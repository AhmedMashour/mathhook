//! Shared mathematical functions for LaTeX and Wolfram parsers
//!
//! This module provides a unified set of mathematical functions that can be used
//! by both LaTeX and Wolfram parsers, ensuring consistency and avoiding duplication.

use crate::core::Expression;

/// Mathematical functions that are shared between LaTeX and Wolfram
#[derive(Debug, Clone, PartialEq)]
pub enum MathFunction {
    // Trigonometric functions
    Sin,
    Cos,
    Tan,
    Sec,
    Csc,
    Cot,
    ArcSin,
    ArcCos,
    ArcTan,
    ArcSec,
    ArcCsc,
    ArcCot,

    // Hyperbolic functions
    Sinh,
    Cosh,
    Tanh,
    Sech,
    Csch,
    Coth,
    ArcSinh,
    ArcCosh,
    ArcTanh,
    ArcSech,
    ArcCsch,
    ArcCoth,

    // Logarithmic functions
    Log,
    Ln,
    Log10,
    Log2,

    // Exponential functions
    Exp,
    Power,

    // Root functions
    Sqrt,
    CubeRoot,
    NthRoot,

    // Special functions
    Gamma,
    Beta,
    Factorial,
    DoubleFactorial,
    Binomial,
    Choose,

    // Calculus
    Integrate,
    Sum,
    Product,
    Limit,
    Derivative,
    Partial,

    // Complex functions
    Abs,
    Arg,
    Re,
    Im,
    Conjugate,

    // Bessel functions
    BesselJ,
    BesselY,
    BesselI,
    BesselK,

    // Hypergeometric functions
    Hypergeometric2F1,
    HypergeometricPFQ,

    // Zeta functions
    Zeta,
    Polylog,
    DirichletEta,

    // Error functions
    Erf,
    Erfc,
    ErfInv,
}

impl MathFunction {
    /// Convert function call to Expression
    pub fn to_expression(&self, args: Vec<Expression>) -> Expression {
        let name = match self {
            // Trigonometric functions
            MathFunction::Sin => "sin",
            MathFunction::Cos => "cos",
            MathFunction::Tan => "tan",
            MathFunction::Sec => "sec",
            MathFunction::Csc => "csc",
            MathFunction::Cot => "cot",
            MathFunction::ArcSin => "arcsin",
            MathFunction::ArcCos => "arccos",
            MathFunction::ArcTan => "arctan",
            MathFunction::ArcSec => "arcsec",
            MathFunction::ArcCsc => "arccsc",
            MathFunction::ArcCot => "arccot",

            // Hyperbolic functions
            MathFunction::Sinh => "sinh",
            MathFunction::Cosh => "cosh",
            MathFunction::Tanh => "tanh",
            MathFunction::Sech => "sech",
            MathFunction::Csch => "csch",
            MathFunction::Coth => "coth",
            MathFunction::ArcSinh => "arcsinh",
            MathFunction::ArcCosh => "arccosh",
            MathFunction::ArcTanh => "arctanh",
            MathFunction::ArcSech => "arcsech",
            MathFunction::ArcCsch => "arccsch",
            MathFunction::ArcCoth => "arccoth",

            // Logarithmic functions
            MathFunction::Log => "log",
            MathFunction::Ln => "ln",
            MathFunction::Log10 => "log10",
            MathFunction::Log2 => "log2",

            // Exponential functions
            MathFunction::Exp => "exp",
            MathFunction::Power => "pow",

            // Root functions
            MathFunction::Sqrt => "sqrt",
            MathFunction::CubeRoot => "cbrt",
            MathFunction::NthRoot => "nthroot",

            // Special functions
            MathFunction::Gamma => "gamma",
            MathFunction::Beta => "beta",
            MathFunction::Factorial => "factorial",
            MathFunction::DoubleFactorial => "double_factorial",
            MathFunction::Binomial => "binomial",
            MathFunction::Choose => "binomial", // Same as binomial

            // Calculus
            MathFunction::Integrate => "integrate",
            MathFunction::Sum => "sum",
            MathFunction::Product => "product",
            MathFunction::Limit => "limit",
            MathFunction::Derivative => "derivative",
            MathFunction::Partial => "partial",

            // Complex functions
            MathFunction::Abs => "abs",
            MathFunction::Arg => "arg",
            MathFunction::Re => "re",
            MathFunction::Im => "im",
            MathFunction::Conjugate => "conjugate",

            // Bessel functions
            MathFunction::BesselJ => "bessel_j",
            MathFunction::BesselY => "bessel_y",
            MathFunction::BesselI => "bessel_i",
            MathFunction::BesselK => "bessel_k",

            // Hypergeometric functions
            MathFunction::Hypergeometric2F1 => "hypergeometric_2f1",
            MathFunction::HypergeometricPFQ => "hypergeometric_pfq",

            // Zeta functions
            MathFunction::Zeta => "zeta",
            MathFunction::Polylog => "polylog",
            MathFunction::DirichletEta => "dirichlet_eta",

            // Error functions
            MathFunction::Erf => "erf",
            MathFunction::Erfc => "erfc",
            MathFunction::ErfInv => "erf_inv",
        };

        // Handle special cases
        match self {
            MathFunction::Power if args.len() == 2 => {
                Expression::pow(args[0].clone(), args[1].clone())
            }
            MathFunction::Sqrt if args.len() == 1 => {
                Expression::pow(args[0].clone(), Expression::rational(1, 2))
            }
            _ => Expression::function(name, args),
        }
    }

    /// Get LaTeX representation
    pub fn latex_name(&self) -> &'static str {
        match self {
            // Trigonometric functions
            MathFunction::Sin => "\\sin",
            MathFunction::Cos => "\\cos",
            MathFunction::Tan => "\\tan",
            MathFunction::Sec => "\\sec",
            MathFunction::Csc => "\\csc",
            MathFunction::Cot => "\\cot",
            MathFunction::ArcSin => "\\arcsin",
            MathFunction::ArcCos => "\\arccos",
            MathFunction::ArcTan => "\\arctan",
            MathFunction::ArcSec => "\\arcsec",
            MathFunction::ArcCsc => "\\arccsc",
            MathFunction::ArcCot => "\\arccot",

            // Hyperbolic functions
            MathFunction::Sinh => "\\sinh",
            MathFunction::Cosh => "\\cosh",
            MathFunction::Tanh => "\\tanh",
            MathFunction::Sech => "\\sech",
            MathFunction::Csch => "\\csch",
            MathFunction::Coth => "\\coth",
            MathFunction::ArcSinh => "\\text{arcsinh}",
            MathFunction::ArcCosh => "\\text{arccosh}",
            MathFunction::ArcTanh => "\\text{arctanh}",
            MathFunction::ArcSech => "\\text{arcsech}",
            MathFunction::ArcCsch => "\\text{arccsch}",
            MathFunction::ArcCoth => "\\text{arccoth}",

            // Logarithmic functions
            MathFunction::Log => "\\log",
            MathFunction::Ln => "\\ln",
            MathFunction::Log10 => "\\log_{10}",
            MathFunction::Log2 => "\\log_2",

            // Exponential functions
            MathFunction::Exp => "\\exp",
            MathFunction::Power => "^",

            // Root functions
            MathFunction::Sqrt => "\\sqrt",
            MathFunction::CubeRoot => "\\sqrt[3]",
            MathFunction::NthRoot => "\\sqrt[n]",

            // Special functions
            MathFunction::Gamma => "\\Gamma",
            MathFunction::Beta => "\\text{B}",
            MathFunction::Factorial => "!",
            MathFunction::DoubleFactorial => "!!",
            MathFunction::Binomial => "\\binom",
            MathFunction::Choose => "\\choose",

            // Calculus
            MathFunction::Integrate => "\\int",
            MathFunction::Sum => "\\sum",
            MathFunction::Product => "\\prod",
            MathFunction::Limit => "\\lim",
            MathFunction::Derivative => "\\frac{d}{dx}",
            MathFunction::Partial => "\\frac{\\partial}{\\partial x}",

            // Complex functions
            MathFunction::Abs => "\\left|\\cdot\\right|",
            MathFunction::Arg => "\\arg",
            MathFunction::Re => "\\text{Re}",
            MathFunction::Im => "\\text{Im}",
            MathFunction::Conjugate => "\\overline",

            // Bessel functions
            MathFunction::BesselJ => "J",
            MathFunction::BesselY => "Y",
            MathFunction::BesselI => "I",
            MathFunction::BesselK => "K",

            // Hypergeometric functions
            MathFunction::Hypergeometric2F1 => "\\,_2F_1",
            MathFunction::HypergeometricPFQ => "\\,_pF_q",

            // Zeta functions
            MathFunction::Zeta => "\\zeta",
            MathFunction::Polylog => "\\text{Li}",
            MathFunction::DirichletEta => "\\eta",

            // Error functions
            MathFunction::Erf => "\\text{erf}",
            MathFunction::Erfc => "\\text{erfc}",
            MathFunction::ErfInv => "\\text{erf}^{-1}",
        }
    }

    /// Get Wolfram representation
    pub fn wolfram_name(&self) -> &'static str {
        match self {
            // Trigonometric functions
            MathFunction::Sin => "Sin",
            MathFunction::Cos => "Cos",
            MathFunction::Tan => "Tan",
            MathFunction::Sec => "Sec",
            MathFunction::Csc => "Csc",
            MathFunction::Cot => "Cot",
            MathFunction::ArcSin => "ArcSin",
            MathFunction::ArcCos => "ArcCos",
            MathFunction::ArcTan => "ArcTan",
            MathFunction::ArcSec => "ArcSec",
            MathFunction::ArcCsc => "ArcCsc",
            MathFunction::ArcCot => "ArcCot",

            // Hyperbolic functions
            MathFunction::Sinh => "Sinh",
            MathFunction::Cosh => "Cosh",
            MathFunction::Tanh => "Tanh",
            MathFunction::Sech => "Sech",
            MathFunction::Csch => "Csch",
            MathFunction::Coth => "Coth",
            MathFunction::ArcSinh => "ArcSinh",
            MathFunction::ArcCosh => "ArcCosh",
            MathFunction::ArcTanh => "ArcTanh",
            MathFunction::ArcSech => "ArcSech",
            MathFunction::ArcCsch => "ArcCsch",
            MathFunction::ArcCoth => "ArcCoth",

            // Logarithmic functions
            MathFunction::Log => "Log",
            MathFunction::Ln => "Log",
            MathFunction::Log10 => "Log10",
            MathFunction::Log2 => "Log2",

            // Exponential functions
            MathFunction::Exp => "Exp",
            MathFunction::Power => "Power",

            // Root functions
            MathFunction::Sqrt => "Sqrt",
            MathFunction::CubeRoot => "CubeRoot",
            MathFunction::NthRoot => "Surd",

            // Special functions
            MathFunction::Gamma => "Gamma",
            MathFunction::Beta => "Beta",
            MathFunction::Factorial => "Factorial",
            MathFunction::DoubleFactorial => "Factorial2",
            MathFunction::Binomial => "Binomial",
            MathFunction::Choose => "Binomial",

            // Calculus
            MathFunction::Integrate => "Integrate",
            MathFunction::Sum => "Sum",
            MathFunction::Product => "Product",
            MathFunction::Limit => "Limit",
            MathFunction::Derivative => "D",
            MathFunction::Partial => "D",

            // Complex functions
            MathFunction::Abs => "Abs",
            MathFunction::Arg => "Arg",
            MathFunction::Re => "Re",
            MathFunction::Im => "Im",
            MathFunction::Conjugate => "Conjugate",

            // Bessel functions
            MathFunction::BesselJ => "BesselJ",
            MathFunction::BesselY => "BesselY",
            MathFunction::BesselI => "BesselI",
            MathFunction::BesselK => "BesselK",

            // Hypergeometric functions
            MathFunction::Hypergeometric2F1 => "Hypergeometric2F1",
            MathFunction::HypergeometricPFQ => "HypergeometricPFQ",

            // Zeta functions
            MathFunction::Zeta => "Zeta",
            MathFunction::Polylog => "PolyLog",
            MathFunction::DirichletEta => "DirichletEta",

            // Error functions
            MathFunction::Erf => "Erf",
            MathFunction::Erfc => "Erfc",
            MathFunction::ErfInv => "InverseErf",
        }
    }
}
