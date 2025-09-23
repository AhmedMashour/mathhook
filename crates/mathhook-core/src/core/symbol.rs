//! Symbol type for variables and identifiers

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Global symbol interning cache to avoid duplicate Arc allocations
static SYMBOL_CACHE: Mutex<Option<HashMap<String, Arc<str>>>> = Mutex::new(None);

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
        let name_str = name.as_ref();

        // Fast path for common single-character symbols
        let interned_name = match name_str {
            "x" | "y" | "z" | "a" | "b" | "c" | "t" | "n" | "i" | "j" | "k" => {
                // Use a static Arc for very common symbols to avoid cache lookup
                match name_str {
                    "x" => {
                        static X_SYMBOL: std::sync::OnceLock<Arc<str>> = std::sync::OnceLock::new();
                        X_SYMBOL.get_or_init(|| "x".into()).clone()
                    }
                    "y" => {
                        static Y_SYMBOL: std::sync::OnceLock<Arc<str>> = std::sync::OnceLock::new();
                        Y_SYMBOL.get_or_init(|| "y".into()).clone()
                    }
                    "z" => {
                        static Z_SYMBOL: std::sync::OnceLock<Arc<str>> = std::sync::OnceLock::new();
                        Z_SYMBOL.get_or_init(|| "z".into()).clone()
                    }
                    _ => {
                        // Fall back to cache for other common symbols
                        Self::intern_symbol(name_str)
                    }
                }
            }
            _ => {
                // Use cache for all other symbols
                Self::intern_symbol(name_str)
            }
        };

        Self {
            name: interned_name,
        }
    }

    /// Internal method to intern symbols using the global cache
    fn intern_symbol(name: &str) -> Arc<str> {
        let mut cache_guard = SYMBOL_CACHE.lock().unwrap();
        let cache = cache_guard.get_or_insert_with(HashMap::new);

        if let Some(existing) = cache.get(name) {
            existing.clone()
        } else {
            let arc_str: Arc<str> = name.into();
            cache.insert(name.to_string(), arc_str.clone());
            arc_str
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
