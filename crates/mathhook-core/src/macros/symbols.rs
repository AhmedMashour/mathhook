//! Symbol creation macros (now procedural)
//!
//! This module re-exports procedural macros from mathhook-macros.
//! The old declarative macros have been replaced with procedural macros for better error messages.

pub use mathhook_macros::{symbol, symbols};
