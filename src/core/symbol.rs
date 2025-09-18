//! Symbol representation for algebraic variables

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents an algebraic symbol (variable)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    /// Create a new symbol with the given name
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }

    /// Get the name of the symbol
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<&str> for Symbol {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl From<String> for Symbol {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_creation() {
        let x = Symbol::new("x");
        assert_eq!(x.name(), "x");

        let y = Symbol::from("y");
        assert_eq!(y.name(), "y");

        let z: Symbol = "z".into();
        assert_eq!(z.name(), "z");
    }

    #[test]
    fn test_symbol_display() {
        let x = Symbol::new("x");
        assert_eq!(format!("{}", x), "x");
    }

    #[test]
    fn test_symbol_equality() {
        let x1 = Symbol::new("x");
        let x2 = Symbol::new("x");
        let y = Symbol::new("y");

        assert_eq!(x1, x2);
        assert_ne!(x1, y);
    }
}
