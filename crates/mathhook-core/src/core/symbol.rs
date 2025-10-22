//! Symbol type for variables and identifiers

use crate::core::commutativity::Commutativity;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Type of symbol (determines commutativity)
///
/// Symbols can represent different mathematical objects with different algebraic properties.
/// The symbol type determines whether operations involving this symbol are commutative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolType {
    /// Scalar variable (default) - commutative
    ///
    /// Examples: x, y, z, theta
    /// Properties: x*y = y*x
    Scalar,

    /// Matrix variable - noncommutative
    ///
    /// Examples: A, B, M (typically uppercase)
    /// Properties: A*B ≠ B*A in general
    Matrix,

    /// Quantum operator - noncommutative
    ///
    /// Examples: x, p, H (position, momentum, Hamiltonian)
    /// Properties: [x,p] = xp - px ≠ 0
    Operator,

    /// Quaternion - noncommutative
    ///
    /// Examples: i, j, k
    /// Properties: ij = k, ji = -k
    Quaternion,
}

impl Default for SymbolType {
    fn default() -> Self {
        SymbolType::Scalar
    }
}

/// Global symbol interning cache to avoid duplicate Arc allocations
static SYMBOL_CACHE: Mutex<Option<HashMap<String, Arc<str>>>> = Mutex::new(None);

/// Mathematical symbol/variable with efficient string sharing
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    pub name: Arc<str>,
    symbol_type: SymbolType,
}

impl Symbol {
    /// Create a new scalar symbol (default behavior, backward compatible)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::Symbol;
    ///
    /// let x = Symbol::new("x");
    /// let alpha = Symbol::new("α");
    /// ```
    #[inline]
    pub fn new<S: AsRef<str>>(name: S) -> Self {
        Self::scalar(name)
    }

    /// Create a scalar symbol (commutative)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::core::symbol::Symbol;
    /// use mathhook_core::core::commutativity::Commutativity;
    ///
    /// let x = Symbol::scalar("x");
    /// assert_eq!(x.commutativity(), Commutativity::Commutative);
    /// ```
    pub fn scalar<S: AsRef<str>>(name: S) -> Self {
        let name_str = name.as_ref();

        let interned_name = match name_str {
            "x" | "y" | "z" | "a" | "b" | "c" | "t" | "n" | "i" | "j" | "k" => {
                match name_str {
                    "x" => {
                        static X_SYMBOL: std::sync::OnceLock<Arc<str>> =
                            std::sync::OnceLock::new();
                        X_SYMBOL.get_or_init(|| "x".into()).clone()
                    }
                    "y" => {
                        static Y_SYMBOL: std::sync::OnceLock<Arc<str>> =
                            std::sync::OnceLock::new();
                        Y_SYMBOL.get_or_init(|| "y".into()).clone()
                    }
                    "z" => {
                        static Z_SYMBOL: std::sync::OnceLock<Arc<str>> =
                            std::sync::OnceLock::new();
                        Z_SYMBOL.get_or_init(|| "z".into()).clone()
                    }
                    _ => Self::intern_symbol(name_str),
                }
            }
            _ => Self::intern_symbol(name_str),
        };

        Self {
            name: interned_name,
            symbol_type: SymbolType::Scalar,
        }
    }

    /// Create a matrix symbol (noncommutative)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::core::symbol::Symbol;
    /// use mathhook_core::core::commutativity::Commutativity;
    ///
    /// let A = Symbol::matrix("A");
    /// assert_eq!(A.commutativity(), Commutativity::Noncommutative);
    /// ```
    pub fn matrix<S: AsRef<str>>(name: S) -> Self {
        let name_str = name.as_ref();
        Self {
            name: Self::intern_symbol(name_str),
            symbol_type: SymbolType::Matrix,
        }
    }

    /// Create an operator symbol (noncommutative)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::core::symbol::Symbol;
    /// use mathhook_core::core::commutativity::Commutativity;
    ///
    /// let p = Symbol::operator("p");
    /// assert_eq!(p.commutativity(), Commutativity::Noncommutative);
    /// ```
    pub fn operator<S: AsRef<str>>(name: S) -> Self {
        let name_str = name.as_ref();
        Self {
            name: Self::intern_symbol(name_str),
            symbol_type: SymbolType::Operator,
        }
    }

    /// Create a quaternion symbol (noncommutative)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::core::symbol::Symbol;
    /// use mathhook_core::core::commutativity::Commutativity;
    ///
    /// let i = Symbol::quaternion("i");
    /// assert_eq!(i.commutativity(), Commutativity::Noncommutative);
    /// ```
    pub fn quaternion<S: AsRef<str>>(name: S) -> Self {
        let name_str = name.as_ref();
        Self {
            name: Self::intern_symbol(name_str),
            symbol_type: SymbolType::Quaternion,
        }
    }

    /// Internal method to intern symbols using the global cache
    fn intern_symbol(name: &str) -> Arc<str> {
        let mut cache_guard = SYMBOL_CACHE
            .lock()
            .expect("BUG: Symbol cache lock poisoned - indicates panic during symbol interning in another thread");
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
    #[inline]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the type of this symbol
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::core::symbol::{Symbol, SymbolType};
    ///
    /// let x = Symbol::scalar("x");
    /// assert_eq!(x.symbol_type(), SymbolType::Scalar);
    ///
    /// let A = Symbol::matrix("A");
    /// assert_eq!(A.symbol_type(), SymbolType::Matrix);
    /// ```
    #[inline]
    pub fn symbol_type(&self) -> SymbolType {
        self.symbol_type
    }

    /// Get commutativity of this symbol
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::core::symbol::Symbol;
    /// use mathhook_core::core::commutativity::Commutativity;
    ///
    /// let x = Symbol::scalar("x");
    /// assert_eq!(x.commutativity(), Commutativity::Commutative);
    ///
    /// let A = Symbol::matrix("A");
    /// assert_eq!(A.commutativity(), Commutativity::Noncommutative);
    /// ```
    #[inline]
    pub fn commutativity(&self) -> Commutativity {
        match self.symbol_type {
            SymbolType::Scalar => Commutativity::Commutative,
            SymbolType::Matrix | SymbolType::Operator | SymbolType::Quaternion => {
                Commutativity::Noncommutative
            }
        }
    }
}

impl Serialize for Symbol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Symbol", 2)?;
        state.serialize_field("name", &*self.name)?;
        state.serialize_field("symbol_type", &self.symbol_type)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Symbol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};

        struct SymbolVisitor;

        impl<'de> Visitor<'de> for SymbolVisitor {
            type Value = Symbol;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Symbol struct or string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Symbol, E>
            where
                E: de::Error,
            {
                Ok(Symbol::new(value))
            }

            fn visit_map<M>(self, mut map: M) -> Result<Symbol, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut name: Option<String> = None;
                let mut symbol_type: Option<SymbolType> = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "name" => {
                            name = Some(map.next_value()?);
                        }
                        "symbol_type" => {
                            symbol_type = Some(map.next_value()?);
                        }
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let symbol_type = symbol_type.unwrap_or_default();

                let interned_name = Symbol::intern_symbol(&name);
                Ok(Symbol {
                    name: interned_name,
                    symbol_type,
                })
            }
        }

        deserializer.deserialize_any(SymbolVisitor)
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
mod symbol_type_tests {
    use super::*;

    #[test]
    fn test_scalar_is_commutative() {
        let x = Symbol::scalar("x");
        assert_eq!(x.symbol_type(), SymbolType::Scalar);
        assert_eq!(x.commutativity(), Commutativity::Commutative);
    }

    #[test]
    fn test_matrix_is_noncommutative() {
        let a = Symbol::matrix("A");
        assert_eq!(a.symbol_type(), SymbolType::Matrix);
        assert_eq!(a.commutativity(), Commutativity::Noncommutative);
    }

    #[test]
    fn test_operator_is_noncommutative() {
        let p = Symbol::operator("p");
        assert_eq!(p.symbol_type(), SymbolType::Operator);
        assert_eq!(p.commutativity(), Commutativity::Noncommutative);
    }

    #[test]
    fn test_quaternion_is_noncommutative() {
        let i = Symbol::quaternion("i");
        assert_eq!(i.symbol_type(), SymbolType::Quaternion);
        assert_eq!(i.commutativity(), Commutativity::Noncommutative);
    }

    #[test]
    fn test_default_symbol_type_is_scalar() {
        assert_eq!(SymbolType::default(), SymbolType::Scalar);
    }

    #[test]
    fn test_backward_compatibility() {
        let x = Symbol::new("x");
        assert_eq!(x.symbol_type(), SymbolType::Scalar);
        assert_eq!(x.commutativity(), Commutativity::Commutative);
    }
}
