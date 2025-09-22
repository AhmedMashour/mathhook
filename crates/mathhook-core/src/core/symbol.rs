//! Symbol type for variables and identifiers

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Arc;

/// Mathematical symbol/variable with efficient string sharing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub name: Arc<str>,
}

impl Symbol {
    /// Create a new symbol with string interning
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Symbol;
    ///
    /// let x = Symbol::new("x");
    /// let alpha = Symbol::new("α");
    /// ```
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self {
            name: name.as_ref().into(),
        }
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

impl Serialize for Symbol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.name.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Symbol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let name = String::deserialize(deserializer)?;
        Ok(Symbol::new(name))
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
