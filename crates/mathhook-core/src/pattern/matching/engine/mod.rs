//! Pattern matching engine
//!
//! Implements the matching algorithms including commutative matching,
//! wildcard binding, and pattern replacement logic.

mod commutative;
mod core;
mod replacement;

pub use self::commutative::*;
pub use self::core::*;
pub use self::replacement::*;

use crate::core::Expression;
use std::collections::HashMap;

/// Result of pattern matching containing variable bindings
pub type PatternMatches = HashMap<String, Expression>;
