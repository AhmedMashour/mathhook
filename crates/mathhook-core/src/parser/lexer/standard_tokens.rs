//! Standard mathematical notation token classification
//!
//! This module contains optimized HashMap for standard mathematical notation
//! like pi, e, sin, cos, alpha, beta, etc.

use super::token_maps::TokenType;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Standard token classification map for O(1) lookups
pub static STANDARD_TOKEN_MAP: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // Standard constants
    map.insert("pi", TokenType::Constant);
    map.insert("e", TokenType::Constant);
    map.insert("i", TokenType::Constant);
    map.insert("infinity", TokenType::Constant);
    map.insert("phi", TokenType::Constant);
    map.insert("gamma", TokenType::Constant);

    // Standard functions
    map.insert("sin", TokenType::Function);
    map.insert("cos", TokenType::Function);
    map.insert("tan", TokenType::Function);
    map.insert("ln", TokenType::Function);
    map.insert("log", TokenType::Function);
    map.insert("sqrt", TokenType::Function);
    map.insert("exp", TokenType::Function);
    map.insert("abs", TokenType::Function);
    map.insert("floor", TokenType::Function);
    map.insert("ceil", TokenType::Function);
    map.insert("round", TokenType::Function);
    map.insert("max", TokenType::Function);
    map.insert("min", TokenType::Function);
    map.insert("gcd", TokenType::Function);
    map.insert("lcm", TokenType::Function);

    // Greek symbol names (common in math)
    map.insert("alpha", TokenType::GreekSymbol);
    map.insert("beta", TokenType::GreekSymbol);
    map.insert("gamma", TokenType::GreekSymbol);
    map.insert("delta", TokenType::GreekSymbol);
    map.insert("epsilon", TokenType::GreekSymbol);
    map.insert("theta", TokenType::GreekSymbol);
    map.insert("lambda", TokenType::GreekSymbol);
    map.insert("mu", TokenType::GreekSymbol);
    map.insert("sigma", TokenType::GreekSymbol);
    map.insert("omega", TokenType::GreekSymbol);

    map
});
