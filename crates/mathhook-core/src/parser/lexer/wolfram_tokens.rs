//! Wolfram Language token classification
//!
//! This module contains optimized HashMap for Wolfram notation tokens
//! like \[Alpha], \[Pi], etc.

use super::token_maps::TokenType;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Wolfram token classification map for O(1) lookups
pub static WOLFRAM_TOKEN_MAP: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // Wolfram Constants
    map.insert("\\[Pi]", TokenType::Constant);
    map.insert("\\[E]", TokenType::Constant);
    map.insert("\\[I]", TokenType::Constant);
    map.insert("\\[Infinity]", TokenType::Constant);
    map.insert("\\[EulerGamma]", TokenType::Constant);

    // Wolfram Greek Symbols
    map.insert("\\[Alpha]", TokenType::GreekSymbol);
    map.insert("\\[Beta]", TokenType::GreekSymbol);
    map.insert("\\[Delta]", TokenType::GreekSymbol);
    map.insert("\\[Epsilon]", TokenType::GreekSymbol);
    map.insert("\\[Zeta]", TokenType::GreekSymbol);
    map.insert("\\[Eta]", TokenType::GreekSymbol);
    map.insert("\\[Theta]", TokenType::GreekSymbol);
    map.insert("\\[Iota]", TokenType::GreekSymbol);
    map.insert("\\[Kappa]", TokenType::GreekSymbol);
    map.insert("\\[Lambda]", TokenType::GreekSymbol);
    map.insert("\\[Mu]", TokenType::GreekSymbol);
    map.insert("\\[Nu]", TokenType::GreekSymbol);
    map.insert("\\[Xi]", TokenType::GreekSymbol);
    map.insert("\\[Omicron]", TokenType::GreekSymbol);
    map.insert("\\[Rho]", TokenType::GreekSymbol);
    map.insert("\\[Sigma]", TokenType::GreekSymbol);
    map.insert("\\[Tau]", TokenType::GreekSymbol);
    map.insert("\\[Upsilon]", TokenType::GreekSymbol);
    map.insert("\\[Chi]", TokenType::GreekSymbol);
    map.insert("\\[Psi]", TokenType::GreekSymbol);
    map.insert("\\[Omega]", TokenType::GreekSymbol);

    // Wolfram Functions (for implicit multiplication)
    // Basic Mathematical Functions
    map.insert("Sin", TokenType::Function);
    map.insert("Cos", TokenType::Function);
    map.insert("Tan", TokenType::Function);
    map.insert("Sec", TokenType::Function);
    map.insert("Csc", TokenType::Function);
    map.insert("Cot", TokenType::Function);

    // Hyperbolic Functions
    map.insert("Sinh", TokenType::Function);
    map.insert("Cosh", TokenType::Function);
    map.insert("Tanh", TokenType::Function);
    map.insert("Sech", TokenType::Function);
    map.insert("Csch", TokenType::Function);
    map.insert("Coth", TokenType::Function);

    // Inverse Trigonometric Functions
    map.insert("ArcSin", TokenType::Function);
    map.insert("ArcCos", TokenType::Function);
    map.insert("ArcTan", TokenType::Function);
    map.insert("ArcSec", TokenType::Function);
    map.insert("ArcCsc", TokenType::Function);
    map.insert("ArcCot", TokenType::Function);

    // Logarithmic and Exponential Functions
    map.insert("Log", TokenType::Function);
    map.insert("Ln", TokenType::Function);
    map.insert("Log10", TokenType::Function);
    map.insert("Log2", TokenType::Function);
    map.insert("Exp", TokenType::Function);
    map.insert("Sqrt", TokenType::Function);

    // Basic Utility Functions
    map.insert("Abs", TokenType::Function);
    map.insert("Sign", TokenType::Function);
    map.insert("Max", TokenType::Function);
    map.insert("Min", TokenType::Function);
    map.insert("Floor", TokenType::Function);
    map.insert("Ceiling", TokenType::Function);
    map.insert("Round", TokenType::Function);

    // Complex Functions
    map.insert("Re", TokenType::Function);
    map.insert("Im", TokenType::Function);
    map.insert("Conjugate", TokenType::Function);
    map.insert("Arg", TokenType::Function);

    // Special Functions
    map.insert("Gamma", TokenType::Function);
    map.insert("BesselJ", TokenType::Function);
    map.insert("BesselY", TokenType::Function);
    map.insert("BesselI", TokenType::Function);
    map.insert("BesselK", TokenType::Function);
    map.insert("LegendreP", TokenType::Function);
    map.insert("LegendreQ", TokenType::Function);
    map.insert("HermiteH", TokenType::Function);
    map.insert("LaguerreL", TokenType::Function);
    map.insert("ChebyshevT", TokenType::Function);
    map.insert("ChebyshevU", TokenType::Function);

    // Calculus Functions (these create special expressions, not simple functions)
    // Note: D, Integrate, Limit, Sum, Product should be handled specially in grammar
    // They are NOT classified as Function type for implicit multiplication

    // Note: Matrix operations (Det, Tr, Inverse, Transpose, Eigenvalues, Eigenvectors)
    // are handled as matrix methods, not simple functions

    // Number Theory Functions
    map.insert("GCD", TokenType::Function);
    map.insert("LCM", TokenType::Function);
    map.insert("Factorial", TokenType::Function);
    map.insert("Binomial", TokenType::Function);
    map.insert("EulerPhi", TokenType::Function);
    map.insert("PrimePi", TokenType::Function);
    map.insert("MoebiusMu", TokenType::Function);

    // Polynomial Functions
    map.insert("PolynomialGCD", TokenType::Function);
    map.insert("Resultant", TokenType::Function);
    map.insert("Discriminant", TokenType::Function);
    map.insert("CyclotomicPolynomial", TokenType::Function);
    map.insert("MinimalPolynomial", TokenType::Function);
    map.insert("GroebnerBasis", TokenType::Function);

    // Statistical Functions
    map.insert("Mean", TokenType::Function);
    map.insert("Median", TokenType::Function);
    map.insert("Variance", TokenType::Function);
    map.insert("StandardDeviation", TokenType::Function);

    map
});
