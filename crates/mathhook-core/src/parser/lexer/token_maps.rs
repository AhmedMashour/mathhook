//! Ultra-fast token classification maps
//!
//! This module contains optimized HashMap-based token lookups for maximum
//! performance in implicit multiplication processing.

use std::collections::HashMap;
use std::sync::LazyLock;

/// Token types for classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Number = 0,
    Identifier = 1,
    Constant = 2,
    GreekSymbol = 3,
    Function = 4,
    LeftParen = 5,
    RightParen = 6,
    Operator = 7,
    LaTeXCommand = 8,
    Other = 9,
}

/// LaTeX token classification map for O(1) lookups
pub static LATEX_TOKEN_MAP: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    // LaTeX Constants
    map.insert("\\pi", TokenType::Constant);
    map.insert("\\phi", TokenType::Constant);
    map.insert("\\varphi", TokenType::Constant);
    map.insert("\\infty", TokenType::Constant);
    map.insert("\\EulerGamma", TokenType::Constant);
    map.insert("\\gamma", TokenType::Constant);

    // LaTeX Greek Symbols
    map.insert("\\alpha", TokenType::GreekSymbol);
    map.insert("\\beta", TokenType::GreekSymbol);
    map.insert("\\delta", TokenType::GreekSymbol);
    map.insert("\\epsilon", TokenType::GreekSymbol);
    map.insert("\\zeta", TokenType::GreekSymbol);
    map.insert("\\eta", TokenType::GreekSymbol);
    map.insert("\\theta", TokenType::GreekSymbol);
    map.insert("\\iota", TokenType::GreekSymbol);
    map.insert("\\kappa", TokenType::GreekSymbol);
    map.insert("\\lambda", TokenType::GreekSymbol);
    map.insert("\\mu", TokenType::GreekSymbol);
    map.insert("\\nu", TokenType::GreekSymbol);
    map.insert("\\xi", TokenType::GreekSymbol);
    map.insert("\\omicron", TokenType::GreekSymbol);
    map.insert("\\rho", TokenType::GreekSymbol);
    map.insert("\\sigma", TokenType::GreekSymbol);
    map.insert("\\tau", TokenType::GreekSymbol);
    map.insert("\\upsilon", TokenType::GreekSymbol);
    map.insert("\\chi", TokenType::GreekSymbol);
    map.insert("\\psi", TokenType::GreekSymbol);
    map.insert("\\omega", TokenType::GreekSymbol);

    // LaTeX Functions
    map.insert("\\sin", TokenType::Function);
    map.insert("\\cos", TokenType::Function);
    map.insert("\\tan", TokenType::Function);
    map.insert("\\ln", TokenType::Function);
    map.insert("\\log", TokenType::Function);
    map.insert("\\sqrt", TokenType::Function);
    map.insert("\\arcsin", TokenType::Function);
    map.insert("\\arccos", TokenType::Function);
    map.insert("\\arctan", TokenType::Function);
    map.insert("\\sinh", TokenType::Function);
    map.insert("\\cosh", TokenType::Function);
    map.insert("\\tanh", TokenType::Function);
    map.insert("\\sec", TokenType::Function);
    map.insert("\\csc", TokenType::Function);
    map.insert("\\cot", TokenType::Function);
    map.insert("\\det", TokenType::Function);
    map.insert("\\max", TokenType::Function);
    map.insert("\\min", TokenType::Function);
    map.insert("\\sup", TokenType::Function);
    map.insert("\\inf", TokenType::Function);
    map.insert("\\gcd", TokenType::Function);
    map.insert("\\lcm", TokenType::Function);

    // LaTeX Operators (should NOT trigger implicit multiplication)
    map.insert("\\cdot", TokenType::Operator);
    map.insert("\\times", TokenType::Operator);
    map.insert("\\div", TokenType::Operator);
    map.insert("\\pm", TokenType::Operator);
    map.insert("\\mp", TokenType::Operator);
    map.insert("\\leq", TokenType::Operator);
    map.insert("\\geq", TokenType::Operator);
    map.insert("\\neq", TokenType::Operator);
    map.insert("\\equiv", TokenType::Operator);
    map.insert("\\approx", TokenType::Operator);

    map
});
