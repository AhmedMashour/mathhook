//! Enhanced lexer with implicit multiplication detection
//!
//! This lexer extends the standard mathematical lexer to detect
//! adjacent terms that should be implicitly multiplied and inserts
//! multiplication tokens automatically.
pub mod impilict;
pub mod single_char;
pub mod tokens;

pub use impilict::*;
pub use single_char::*;
pub use tokens::*;
