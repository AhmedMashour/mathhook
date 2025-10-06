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

    map
});
