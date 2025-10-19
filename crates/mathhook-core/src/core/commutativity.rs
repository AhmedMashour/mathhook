//! Commutativity tracking for expressions
//!
//! Supports noncommutative algebra (matrices, operators, quaternions) while
//! maintaining default commutative behavior for scalars.
//!
//! Commutativity is computed on-demand from symbol types, not stored in expressions.

/// Commutativity of an expression or operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Commutativity {
    /// Operation is commutative (a*b = b*a)
    /// Examples: scalar multiplication, addition
    Commutative,

    /// Operation is noncommutative (a*b may not equal b*a)
    /// Examples: matrix multiplication, operator multiplication, quaternion multiplication
    Noncommutative,
}

impl Commutativity {
    /// Can factors be sorted during canonicalization?
    ///
    /// Returns true only if commutativity is guaranteed.
    pub fn can_sort(self) -> bool {
        matches!(self, Commutativity::Commutative)
    }

    /// Combine commutativity of multiple factors
    ///
    /// Rule: If ANY factor is noncommutative, the entire product is noncommutative.
    /// Only if ALL factors are commutative is the product commutative.
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::core::commutativity::Commutativity;
    ///
    /// // All commutative → result commutative
    /// let factors = vec![Commutativity::Commutative, Commutativity::Commutative];
    /// assert_eq!(Commutativity::combine(factors), Commutativity::Commutative);
    ///
    /// // Any noncommutative → result noncommutative
    /// let factors = vec![Commutativity::Commutative, Commutativity::Noncommutative];
    /// assert_eq!(Commutativity::combine(factors), Commutativity::Noncommutative);
    /// ```
    pub fn combine<I>(factors: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        for comm in factors {
            if comm == Commutativity::Noncommutative {
                return Commutativity::Noncommutative;
            }
        }
        Commutativity::Commutative
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commutativity_can_sort() {
        assert!(Commutativity::Commutative.can_sort());
        assert!(!Commutativity::Noncommutative.can_sort());
    }

    #[test]
    fn test_combine_all_commutative() {
        let factors = vec![Commutativity::Commutative, Commutativity::Commutative];
        assert_eq!(Commutativity::combine(factors), Commutativity::Commutative);
    }

    #[test]
    fn test_combine_any_noncommutative() {
        let factors = vec![
            Commutativity::Commutative,
            Commutativity::Noncommutative,
        ];
        assert_eq!(
            Commutativity::combine(factors),
            Commutativity::Noncommutative
        );
    }

    #[test]
    fn test_combine_all_noncommutative() {
        let factors = vec![
            Commutativity::Noncommutative,
            Commutativity::Noncommutative,
        ];
        assert_eq!(
            Commutativity::combine(factors),
            Commutativity::Noncommutative
        );
    }

    #[test]
    fn test_combine_empty() {
        let factors: Vec<Commutativity> = vec![];
        assert_eq!(Commutativity::combine(factors), Commutativity::Commutative);
    }
}
