//! Symbol type for variables and identifiers

use serde::{Deserialize, Serialize};

/// Mathematical symbol/variable
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    /// Create a new symbol
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Symbol;
    ///
    /// let x = Symbol::new("x");
    /// let alpha = Symbol::new("α");
    /// ```
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }

    /// Get the symbol name
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Symbol;
    ///
    /// let x = Symbol::new("x");
    /// assert_eq!(x.name(), "x");
    /// ```
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<&str> for Symbol {
    fn from(name: &str) -> Self {
        Self::new(name)
    }
}

impl From<String> for Symbol {
    fn from(name: String) -> Self {
        Self { name }
    }
}
