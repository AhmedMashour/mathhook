//! High-performance lexer with comprehensive implicit multiplication
//!
//! This module provides ultra-fast implicit multiplication processing with
//! comprehensive support for LaTeX, Wolfram, and standard mathematical notation.
//!
//! Features:
//! - Multi-format support: LaTeX (\pi), Wolfram (\[Pi]), Standard (pi)
//! - Smart token recognition with O(1) HashMap lookups
//! - Intelligent operator and function call preservation
//! - Precomputed multiplication rules matrix for maximum speed

pub mod comprehensive_processor;
pub mod multiplication_rules;
pub mod rules;
pub mod standard_tokens;
pub mod token_maps;
pub mod tokens;
pub mod wolfram_tokens;

pub use comprehensive_processor::*;
pub use multiplication_rules::*;
pub use rules::*;
pub use standard_tokens::*;
pub use token_maps::*;
pub use tokens::*;
pub use wolfram_tokens::*;
