//! Token classification rules for implicit multiplication
//!
//! This module contains the static data and rules that determine when
//! implicit multiplication should be inserted between tokens.

use std::collections::HashSet;
use std::sync::LazyLock;

/// Categories of mathematical tokens for implicit multiplication logic
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenCategory {
    Number,
    Identifier,
    Constant,
    GreekSymbol,
    Function,
    LeftParen,
    RightParen,
    Operator,
    Other,
}

/// Mathematical constants that should trigger implicit multiplication
pub static CONSTANTS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        // Basic constants (as token variants)
        "PI",
        "E_CONST",
        "I_CONST",
        "INFINITY",
        "PHI",
        "GOLDEN_RATIO",
        "EULER_GAMMA",
        "GAMMA_CONST",
        "UNDEFINED",
        // LaTeX constants
        "LATEX_PI",
        "LATEX_PHI",
        "LATEX_VARPHI",
        "LATEX_INFTY",
        "LATEX_EULER_GAMMA",
        "LATEX_GAMMA", // Gamma function
        // Wolfram constants/functions that act like constants
        "WOLFRAM_GAMMA", // Gamma function
    ])
});

/// Greek symbols that should trigger implicit multiplication
pub static GREEK_SYMBOLS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        // LaTeX Greek symbols
        "LATEX_ALPHA",
        "LATEX_BETA",
        "LATEX_DELTA",
        "LATEX_EPSILON",
        "LATEX_ZETA",
        "LATEX_ETA",
        "LATEX_THETA",
        "LATEX_IOTA",
        "LATEX_KAPPA",
        "LATEX_LAMBDA",
        "LATEX_MU",
        "LATEX_NU",
        "LATEX_XI",
        "LATEX_OMICRON",
        "LATEX_RHO",
        "LATEX_SIGMA",
        "LATEX_TAU",
        "LATEX_UPSILON",
        "LATEX_CHI",
        "LATEX_PSI",
        "LATEX_OMEGA",
        // Wolfram Greek symbols
        "WOLFRAM_ALPHA",
        "WOLFRAM_BETA",
        "WOLFRAM_DELTA",
        "WOLFRAM_EPSILON",
        "WOLFRAM_ZETA",
        "WOLFRAM_ETA",
        "WOLFRAM_THETA",
        "WOLFRAM_IOTA",
        "WOLFRAM_KAPPA",
        "WOLFRAM_LAMBDA",
        "WOLFRAM_MU",
        "WOLFRAM_NU",
        "WOLFRAM_XI",
        "WOLFRAM_OMICRON",
        "WOLFRAM_RHO",
        "WOLFRAM_SIGMA",
        "WOLFRAM_TAU",
        "WOLFRAM_UPSILON",
        "WOLFRAM_CHI",
        "WOLFRAM_PSI",
        "WOLFRAM_OMEGA",
        // Note: WOLFRAM_GAMMA is in CONSTANTS as it's the Gamma function
    ])
});

/// Mathematical functions that should NOT trigger implicit multiplication when followed by parentheses
pub static FUNCTIONS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "sin", "cos", "tan", "sec", "csc", "cot", "sinh", "cosh", "tanh", "sech", "csch", "coth",
        "arcsin", "arccos", "arctan", "arcsec", "arccsc", "arccot", "asin", "acos", "atan", "asec",
        "acsc", "acot", "log", "ln", "exp", "sqrt", "abs", "floor", "ceil", "round", "sign", "max",
        "min", "gcd", "lcm", "gamma", "beta", "zeta", "erf", "erfc", "J", "Y", "I",
        "K", // Bessel functions
        "P", "Q", "L", "H", // Legendre, Hermite functions
        "F", "G", "U", "M", "W", // Hypergeometric, Whittaker functions
    ])
});

/// Rules for when implicit multiplication should be inserted
pub static IMPLICIT_MUL_RULES: LazyLock<HashSet<(TokenCategory, TokenCategory)>> =
    LazyLock::new(|| {
        HashSet::from([
            // Number followed by anything multiplicative
            (TokenCategory::Number, TokenCategory::Identifier),
            (TokenCategory::Number, TokenCategory::Constant),
            (TokenCategory::Number, TokenCategory::GreekSymbol),
            (TokenCategory::Number, TokenCategory::LeftParen),
            // Identifier followed by anything multiplicative
            (TokenCategory::Identifier, TokenCategory::Identifier),
            (TokenCategory::Identifier, TokenCategory::Constant),
            (TokenCategory::Identifier, TokenCategory::GreekSymbol),
            // Constants followed by anything multiplicative
            (TokenCategory::Constant, TokenCategory::Identifier),
            (TokenCategory::Constant, TokenCategory::Constant),
            (TokenCategory::Constant, TokenCategory::GreekSymbol),
            (TokenCategory::Constant, TokenCategory::LeftParen),
            // Greek symbols followed by anything multiplicative
            (TokenCategory::GreekSymbol, TokenCategory::Identifier),
            (TokenCategory::GreekSymbol, TokenCategory::Constant),
            (TokenCategory::GreekSymbol, TokenCategory::GreekSymbol),
            (TokenCategory::GreekSymbol, TokenCategory::LeftParen),
            // Right parentheses followed by anything multiplicative
            (TokenCategory::RightParen, TokenCategory::Identifier),
            (TokenCategory::RightParen, TokenCategory::Constant),
            (TokenCategory::RightParen, TokenCategory::GreekSymbol),
            (TokenCategory::RightParen, TokenCategory::LeftParen),
        ])
    });
