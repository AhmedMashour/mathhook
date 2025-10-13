//! Pattern matching and substitution system for MathHook
//!
//! This module provides the fundamental pattern matching and substitution capabilities
//! that enable equation solving, transformation rules, and algebraic manipulation.
//!
//! # Core Functionality
//!
//! 1. **Basic Substitution**: Replace variables or subexpressions with values
//! 2. **Multiple Substitution**: Apply several substitutions simultaneously
//! 3. **Pattern Matching**: Match structural patterns with wildcards
//! 4. **Pattern Replacement**: Apply transformation rules
//!
//! # Examples
//!
//! ```
//! use mathhook_core::prelude::*;
//! use mathhook_core::pattern::Substitutable;
//!
//! let x = symbol!(x);
//! let expr = expr!(x + 1);
//!
//! // Basic substitution: replace x with 5
//! let result = expr.subs(&Expression::symbol(x), &Expression::integer(5));
//! assert_eq!(result, Expression::integer(6));
//! ```

pub mod matching;
pub mod substitution;

pub use matching::{Matchable, Pattern, PatternMatches};
pub use substitution::Substitutable;
