#![allow(dead_code)]
//! Lazy-clone helper for FFI boundaries
//!
//! Minimizes allocations by deferring clones until necessary.
//!
//! Strategy:
//! - Borrowed variant: From PyExpression.inner / JsExpression.inner - no clone yet
//! - Owned variant: From int/float/string conversion - already owned
//!
//! The clone only happens when calling `into_owned_if_needed()`, which is
//! typically right before passing to the Rust core function.

/// Helper type for deferred cloning at FFI boundary
///
/// Minimizes memory allocations by cloning only when necessary.
///
/// Generic over any cloneable type T (typically Expression).
///
/// # Examples
///
/// ```ignore
/// // Borrowed from PyExpression - no clone yet
/// let ref_borrowed = ExprRef::Borrowed(&py_expr.inner);
///
/// // Owned from conversion - no extra clone
/// let ref_owned = ExprRef::Owned(Expression::integer(42));
///
/// // Clone only when needed
/// let expr = ref_borrowed.into_owned_if_needed();
/// ```
#[derive(Debug)]
pub enum ExprRef<'a, T: Clone> {
    /// Reference to an existing value - will clone when needed
    Borrowed(&'a T),
    /// Already owned value - no clone needed
    Owned(T),
}

impl<'a, T: Clone> ExprRef<'a, T> {
    /// Convert to owned value, cloning only if borrowed
    ///
    /// - `Borrowed`: Clones the referenced value
    /// - `Owned`: Moves the value (no clone)
    #[inline]
    pub fn into_owned_if_needed(self) -> T {
        match self {
            Self::Borrowed(e) => e.clone(),
            Self::Owned(e) => e,
        }
    }

    /// Create from a borrowed value
    #[inline]
    pub fn borrowed(value: &'a T) -> Self {
        Self::Borrowed(value)
    }

    /// Create from an owned value
    #[inline]
    pub fn owned(value: T) -> Self {
        Self::Owned(value)
    }

    /// Get a reference to the value without cloning
    #[inline]
    pub fn as_ref(&self) -> &T {
        match self {
            Self::Borrowed(e) => e,
            Self::Owned(e) => e,
        }
    }

    /// Check if this reference is borrowed
    #[inline]
    pub fn is_borrowed(&self) -> bool {
        matches!(self, Self::Borrowed(_))
    }

    /// Check if this reference is owned
    #[inline]
    pub fn is_owned(&self) -> bool {
        matches!(self, Self::Owned(_))
    }
}

impl<'a, T: Clone> From<&'a T> for ExprRef<'a, T> {
    #[inline]
    fn from(value: &'a T) -> Self {
        Self::Borrowed(value)
    }
}

impl<'a, T: Clone> From<T> for ExprRef<'a, T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::Owned(value)
    }
}

impl<'a, T: Clone> AsRef<T> for ExprRef<'a, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        match self {
            Self::Borrowed(e) => e,
            Self::Owned(e) => e,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestValue(i32);

    #[test]
    fn test_borrowed_variant() {
        let value = TestValue(42);
        let value_ref = ExprRef::borrowed(&value);

        assert!(value_ref.is_borrowed());
        assert!(!value_ref.is_owned());
        assert_eq!(value_ref.as_ref(), &value);

        let owned = value_ref.into_owned_if_needed();
        assert_eq!(owned, value);
    }

    #[test]
    fn test_owned_variant() {
        let value = TestValue(42);
        let value_ref = ExprRef::owned(value.clone());

        assert!(value_ref.is_owned());
        assert!(!value_ref.is_borrowed());
        assert_eq!(value_ref.as_ref(), &value);

        let owned = value_ref.into_owned_if_needed();
        assert_eq!(owned, value);
    }

    #[test]
    fn test_from_reference() {
        let value = TestValue(42);
        let value_ref: ExprRef<TestValue> = (&value).into();

        assert!(value_ref.is_borrowed());
    }

    #[test]
    fn test_from_owned() {
        let value = TestValue(42);
        let value_ref: ExprRef<TestValue> = value.clone().into();

        assert!(value_ref.is_owned());
    }

    #[test]
    fn test_as_ref_trait() {
        let value = TestValue(42);
        let value_ref = ExprRef::borrowed(&value);

        let ref_via_trait: &TestValue = value_ref.as_ref();
        assert_eq!(ref_via_trait, &value);
    }

    #[test]
    fn test_memory_efficiency() {
        let value = TestValue(42);

        let borrowed = ExprRef::borrowed(&value);

        let owned = borrowed.into_owned_if_needed();
        assert_eq!(owned, value);
    }
}
