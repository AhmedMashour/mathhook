//! Constants for mathematical function names and optimization
//!
//! This module provides compile-time constants for function names and common
//! mathematical values to improve readability, maintainability, and performance.

use crate::core::Expression;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Mathematical function names (compile-time optimization)
pub mod function_names {
    // Basic trigonometric functions
    pub const SIN: &str = "sin";
    pub const COS: &str = "cos";
    pub const TAN: &str = "tan";

    // Hyperbolic functions
    pub const SINH: &str = "sinh";
    pub const COSH: &str = "cosh";
    pub const TANH: &str = "tanh";
    pub const SECH: &str = "sech";
    pub const CSCH: &str = "csch";
    pub const COTH: &str = "coth";

    // Inverse trigonometric functions
    pub const ARCSIN: &str = "arcsin";
    pub const ARCCOS: &str = "arccos";
    pub const ARCTAN: &str = "arctan";
    pub const ARCSEC: &str = "arcsec";
    pub const ARCCSC: &str = "arccsc";
    pub const ARCCOT: &str = "arccot";

    // Extended trigonometric functions
    pub const SEC: &str = "sec";
    pub const CSC: &str = "csc";
    pub const COT: &str = "cot";

    // Logarithmic functions
    pub const LN: &str = "ln";
    pub const LOG: &str = "log";
    pub const LOG10: &str = "log10";
    pub const LOG2: &str = "log2";

    // Other basic functions
    pub const SQRT: &str = "sqrt";
    pub const ABS: &str = "abs";
    pub const EXP: &str = "exp";

    // Factorial functions
    pub const FACTORIAL: &str = "factorial";
    pub const DOUBLE_FACTORIAL: &str = "double_factorial";

    // Special functions
    pub const GAMMA: &str = "gamma";
    pub const BESSEL_J: &str = "bessel_j";
    pub const BESSEL_Y: &str = "bessel_y";
    pub const BESSEL_I: &str = "bessel_i";
    pub const BESSEL_K: &str = "bessel_k";
    pub const LEGENDRE_P: &str = "legendre_p";
    pub const LEGENDRE_Q: &str = "legendre_q";
    pub const HERMITE: &str = "hermite";
    pub const HERMITE_PHYSICIST: &str = "hermite_physicist";
    pub const LAGUERRE: &str = "laguerre";
    pub const LAGUERRE_ASSOCIATED: &str = "laguerre_associated";
    pub const CHEBYSHEV_FIRST: &str = "chebyshev_first";
    pub const CHEBYSHEV_SECOND: &str = "chebyshev_second";

    // Calculus functions
    pub const DERIVATIVE: &str = "derivative";
    pub const INTEGRAL: &str = "integral";
    pub const LIMIT: &str = "limit";
    pub const SUM: &str = "sum";
    pub const PRODUCT: &str = "product";

    // Text functions
    pub const TEXT_RE: &str = "text_re";
    pub const TEXT_IM: &str = "text_im";
    pub const TEXT_QR: &str = "text_qr";
    pub const TEXT_PRIM_ROOT: &str = "text_primroot";

    // Complex analysis
    pub const CONJUGATE: &str = "conjugate";
    pub const REAL: &str = "real";
    pub const IMAG: &str = "imag";
    pub const ARG: &str = "arg";

    // Utility Functions
    pub const SIGN: &str = "sign";
    pub const FLOOR: &str = "floor";
    pub const CEILING: &str = "ceiling";
    pub const ROUND: &str = "round";
    pub const MAX: &str = "max";
    pub const MIN: &str = "min";

    // Note: Matrix operations (det, trace, rank, inverse, transpose, eigenvalues, eigenvectors)
    // are now handled as MethodCall expressions, not function constants

    // Statistical Functions
    pub const MEAN: &str = "mean";
    pub const MEDIAN: &str = "median";
    pub const VARIANCE: &str = "variance";
    pub const STD: &str = "std";

    // Number Theory Functions (additional)
    pub const GCD_CAPS: &str = "gcd";
    pub const LCM_CAPS: &str = "lcm";
    pub const BINOMIAL: &str = "binomial";

    // Number theory
    pub const CYCLOTOMIC_POLYNOMIAL: &str = "cyclotomic_polynomial";
    pub const MINIMAL_POLYNOMIAL: &str = "minimal_polynomial";
    pub const GROEBNER_BASIS: &str = "groebner_basis";
    pub const RESULTANT: &str = "resultant";
    pub const DISCRIMINANT: &str = "discriminant";
    pub const POLYNOMIAL_GCD: &str = "polynomial_gcd";
    pub const RIEMANN_ZETA: &str = "riemann_zeta";
    pub const RIEMANN_SIEGEL_THETA: &str = "riemann_siegel_theta";
    pub const MOEBIUS_MU: &str = "moebius_mu";
    pub const EULER_PHI: &str = "euler_phi";
    pub const PRIME_PI: &str = "prime_pi";
}

/// Common mathematical values (lazy initialization for performance)
pub mod math_constants {
    use super::*;

    pub static ZERO: Lazy<Expression> = Lazy::new(|| Expression::integer(0));
    pub static ONE: Lazy<Expression> = Lazy::new(|| Expression::integer(1));
    pub static MINUS_ONE: Lazy<Expression> = Lazy::new(|| Expression::integer(-1));
    pub static TWO: Lazy<Expression> = Lazy::new(|| Expression::integer(2));
    pub static HALF: Lazy<Expression> = Lazy::new(|| Expression::rational(1, 2));
    pub static PI: Lazy<Expression> = Lazy::new(|| Expression::pi());
    pub static E: Lazy<Expression> = Lazy::new(|| Expression::e());
    pub static I: Lazy<Expression> = Lazy::new(|| Expression::i());
}

/// Special function name mapping for indexed functions
///
/// Used for functions like J_n(x), P_l^m(x) where the base name needs
/// to be mapped to the appropriate function name.
pub static SPECIAL_FUNCTION_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Bessel functions
    map.insert("J", function_names::BESSEL_J);
    map.insert("Y", function_names::BESSEL_Y);
    map.insert("I", function_names::BESSEL_I);
    map.insert("K", function_names::BESSEL_K);

    // Legendre functions
    map.insert("P", function_names::LEGENDRE_P);
    map.insert("Q", function_names::LEGENDRE_Q);

    // Hermite polynomials
    map.insert("H", function_names::HERMITE);
    map.insert("He", function_names::HERMITE_PHYSICIST);

    // Laguerre polynomials
    map.insert("L", function_names::LAGUERRE);
    map.insert("La", function_names::LAGUERRE_ASSOCIATED);

    // Chebyshev polynomials
    map.insert("T", function_names::CHEBYSHEV_FIRST);
    map.insert("U", function_names::CHEBYSHEV_SECOND);

    map
});

/// Wolfram function name mapping for readability
///
/// Maps Wolfram Language function names to our internal function names.
/// Uses HashMap for complex lookups where readability matters more than
/// micro-performance optimizations.
pub static WOLFRAM_FUNCTION_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Basic functions
    map.insert("Sin", function_names::SIN);
    map.insert("Cos", function_names::COS);
    map.insert("Tan", function_names::TAN);
    map.insert("Log", function_names::LN);
    map.insert("Sqrt", function_names::SQRT);
    map.insert("Exp", function_names::EXP);

    // Hyperbolic functions
    map.insert("Sinh", function_names::SINH);
    map.insert("Cosh", function_names::COSH);
    map.insert("Tanh", function_names::TANH);

    // Inverse functions
    map.insert("ArcSin", function_names::ARCSIN);
    map.insert("ArcCos", function_names::ARCCOS);
    map.insert("ArcTan", function_names::ARCTAN);

    // Special functions
    map.insert("BesselJ", function_names::BESSEL_J);
    map.insert("BesselY", function_names::BESSEL_Y);
    map.insert("BesselI", function_names::BESSEL_I);
    map.insert("BesselK", function_names::BESSEL_K);
    map.insert("LegendreP", function_names::LEGENDRE_P);
    map.insert("LegendreQ", function_names::LEGENDRE_Q);
    map.insert("HermiteH", function_names::HERMITE);
    map.insert("LaguerreL", function_names::LAGUERRE);
    map.insert("ChebyshevT", function_names::CHEBYSHEV_FIRST);
    map.insert("ChebyshevU", function_names::CHEBYSHEV_SECOND);

    // Advanced functions
    map.insert(
        "CyclotomicPolynomial",
        function_names::CYCLOTOMIC_POLYNOMIAL,
    );
    map.insert("MinimalPolynomial", function_names::MINIMAL_POLYNOMIAL);
    map.insert("GroebnerBasis", function_names::GROEBNER_BASIS);
    map.insert("Resultant", function_names::RESULTANT);
    map.insert("Discriminant", function_names::DISCRIMINANT);
    map.insert("PolynomialGCD", function_names::POLYNOMIAL_GCD);
    map.insert("RiemannSiegelTheta", function_names::RIEMANN_ZETA);
    map.insert("MoebiusMu", function_names::MOEBIUS_MU);
    map.insert("EulerPhi", function_names::EULER_PHI);
    map.insert("PrimePi", function_names::PRIME_PI);

    // Extended trigonometric functions
    map.insert("Sec", function_names::SEC);
    map.insert("Csc", function_names::CSC);
    map.insert("Cot", function_names::COT);

    // Extended hyperbolic functions
    map.insert("Sech", function_names::SECH);
    map.insert("Csch", function_names::CSCH);
    map.insert("Coth", function_names::COTH);

    // Extended inverse functions
    map.insert("ArcSec", function_names::ARCSEC);
    map.insert("ArcCsc", function_names::ARCCSC);
    map.insert("ArcCot", function_names::ARCCOT);

    // Utility functions
    map.insert("Abs", function_names::ABS);
    map.insert("Sign", function_names::SIGN);
    map.insert("Max", function_names::MAX);
    map.insert("Min", function_names::MIN);
    map.insert("Floor", function_names::FLOOR);
    map.insert("Ceiling", function_names::CEILING);
    map.insert("Round", function_names::ROUND);

    // Complex functions
    map.insert("Re", function_names::REAL);
    map.insert("Im", function_names::IMAG);
    map.insert("Conjugate", function_names::CONJUGATE);
    map.insert("Arg", function_names::ARG);

    // Note: Matrix operations (Det, Tr, Inverse, Transpose, Eigenvalues, Eigenvectors)
    // are handled as matrix methods, not simple functions

    // Number theory functions
    map.insert("GCD", function_names::GCD_CAPS);
    map.insert("LCM", function_names::LCM_CAPS);
    map.insert("Factorial", function_names::FACTORIAL);
    map.insert("Binomial", function_names::BINOMIAL);

    // Statistical functions
    map.insert("Mean", function_names::MEAN);
    map.insert("Median", function_names::MEDIAN);
    map.insert("Variance", function_names::VARIANCE);
    map.insert("StandardDeviation", function_names::STD);

    map
});

/// Standard function name mapping for simple functions like sin(x), cos(x)
///
/// Note: LaTeX functions (\sin, \cos) are handled by explicit tokens in grammar
/// Note: Wolfram functions (Sin, Cos) use WOLFRAM_FUNCTION_MAP
pub static STANDARD_FUNCTION_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Basic trigonometric functions
    map.insert("sin", function_names::SIN);
    map.insert("cos", function_names::COS);
    map.insert("tan", function_names::TAN);
    map.insert("sec", function_names::SEC);
    map.insert("csc", function_names::CSC);
    map.insert("cot", function_names::COT);

    // Hyperbolic functions
    map.insert("sinh", function_names::SINH);
    map.insert("cosh", function_names::COSH);
    map.insert("tanh", function_names::TANH);
    map.insert("sech", function_names::SECH);
    map.insert("csch", function_names::CSCH);
    map.insert("coth", function_names::COTH);

    // Inverse trigonometric functions
    map.insert("arcsin", function_names::ARCSIN);
    map.insert("arccos", function_names::ARCCOS);
    map.insert("arctan", function_names::ARCTAN);
    map.insert("arcsec", function_names::ARCSEC);
    map.insert("arccsc", function_names::ARCCSC);
    map.insert("arccot", function_names::ARCCOT);
    map.insert("asin", function_names::ARCSIN); // Alternative names
    map.insert("acos", function_names::ARCCOS);
    map.insert("atan", function_names::ARCTAN);

    // Logarithmic functions
    map.insert("ln", function_names::LN);
    map.insert("log", function_names::LOG);
    map.insert("log10", function_names::LOG10);
    map.insert("log2", function_names::LOG2);

    // Other basic functions
    map.insert("sqrt", function_names::SQRT);
    map.insert("abs", function_names::ABS);
    map.insert("exp", function_names::EXP);
    map.insert("sign", function_names::SIGN);
    map.insert("floor", function_names::FLOOR);
    map.insert("ceiling", function_names::CEILING);
    map.insert("round", function_names::ROUND);
    map.insert("max", function_names::MAX);
    map.insert("min", function_names::MIN);

    // Special functions
    map.insert("gamma", function_names::GAMMA);
    map.insert("factorial", function_names::FACTORIAL);

    // Complex functions
    map.insert("real", function_names::REAL);
    map.insert("imag", function_names::IMAG);
    map.insert("conj", function_names::CONJUGATE);
    map.insert("arg", function_names::ARG);

    // Note: Matrix operations (det, trace, rank) are handled as matrix methods, not simple functions

    // Statistical functions
    map.insert("mean", function_names::MEAN);
    map.insert("median", function_names::MEDIAN);
    map.insert("var", function_names::VARIANCE);
    map.insert("std", function_names::STD);

    // Number theory functions
    map.insert("gcd", function_names::GCD_CAPS);
    map.insert("lcm", function_names::LCM_CAPS);

    map
});

/// Efficient function name lookup with fallback
///
/// # Examples
///
/// ```rust
/// use mathhook_core::parser::constants::resolve_wolfram_function;
///
/// assert_eq!(resolve_wolfram_function("Sin"), Some("sin"));
/// assert_eq!(resolve_wolfram_function("UnknownFunction"), None);
/// ```
pub fn resolve_wolfram_function(name: &str) -> Option<&'static str> {
    WOLFRAM_FUNCTION_MAP.get(name).copied()
}

/// Convert PascalCase function names to snake_case
///
/// This provides a flexible, generic way to convert Wolfram function names
/// (like "BesselJ", "ArcSin") to consistent snake_case format.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::parser::constants::pascal_to_snake_case;
///
/// assert_eq!(pascal_to_snake_case("Sin"), "sin");
/// assert_eq!(pascal_to_snake_case("BesselJ"), "bessel_j");
/// assert_eq!(pascal_to_snake_case("ArcSin"), "arc_sin");
/// assert_eq!(pascal_to_snake_case("DiracDelta"), "dirac_delta");
/// ```
pub fn pascal_to_snake_case(name: &str) -> String {
    let mut result = String::with_capacity(name.len() + 4); // Reserve extra space for underscores
    let mut chars = name.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            // Add underscore before uppercase letters (except the first character)
            if !result.is_empty() {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap());
        } else {
            result.push(ch);
        }
    }

    result
}

/// Resolve special function name for indexed functions
///
/// # Examples
///
/// ```rust
/// use mathhook_core::parser::constants::resolve_special_function;
///
/// assert_eq!(resolve_special_function("J"), Some("bessel_j"));
/// assert_eq!(resolve_special_function("P"), Some("legendre_p"));
/// assert_eq!(resolve_special_function("Unknown"), None);
/// ```
pub fn resolve_special_function(name: &str) -> Option<&'static str> {
    SPECIAL_FUNCTION_MAP.get(name).copied()
}

/// Resolve standard function name (for simple functions like sin(x))
///
/// # Examples
///
/// ```rust
/// use mathhook_core::parser::constants::resolve_standard_function;
///
/// assert_eq!(resolve_standard_function("sin"), Some("sin"));
/// assert_eq!(resolve_standard_function("asin"), Some("arcsin"));
/// assert_eq!(resolve_standard_function("unknown"), None);
/// ```
pub fn resolve_standard_function(name: &str) -> Option<&'static str> {
    STANDARD_FUNCTION_MAP.get(name).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wolfram_function_resolution() {
        assert_eq!(resolve_wolfram_function("Sin"), Some(function_names::SIN));
        assert_eq!(resolve_wolfram_function("Cos"), Some(function_names::COS));
        assert_eq!(
            resolve_wolfram_function("BesselJ"),
            Some(function_names::BESSEL_J)
        );
        assert_eq!(resolve_wolfram_function("UnknownFunction"), None);
    }

    #[test]
    fn test_pascal_to_snake_case() {
        // Basic cases
        assert_eq!(pascal_to_snake_case("Sin"), "sin");
        assert_eq!(pascal_to_snake_case("Cos"), "cos");

        // Multi-word cases
        assert_eq!(pascal_to_snake_case("BesselJ"), "bessel_j");
        assert_eq!(pascal_to_snake_case("ArcSin"), "arc_sin");
        assert_eq!(pascal_to_snake_case("DiracDelta"), "dirac_delta");
        assert_eq!(pascal_to_snake_case("HeavisideTheta"), "heaviside_theta");

        // Complex cases
        assert_eq!(
            pascal_to_snake_case("Hypergeometric2F1"),
            "hypergeometric2_f1"
        );
        assert_eq!(pascal_to_snake_case("LegendreP"), "legendre_p");

        // Edge cases
        assert_eq!(pascal_to_snake_case("A"), "a");
        assert_eq!(pascal_to_snake_case("AB"), "a_b");
        assert_eq!(pascal_to_snake_case("ABC"), "a_b_c");
    }

    #[test]
    fn test_special_function_resolution() {
        assert_eq!(
            resolve_special_function("J"),
            Some(function_names::BESSEL_J)
        );
        assert_eq!(
            resolve_special_function("P"),
            Some(function_names::LEGENDRE_P)
        );
        assert_eq!(resolve_special_function("H"), Some(function_names::HERMITE));
        assert_eq!(resolve_special_function("Unknown"), None);
    }

    #[test]
    fn test_math_constants_initialization() {
        // Test that lazy constants can be accessed without panicking
        let _zero = &*math_constants::ZERO;
        let _one = &*math_constants::ONE;
        let _pi = &*math_constants::PI;
    }
}
