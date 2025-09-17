//! Polynomial Properties
//!
//! Core polynomial properties: degree, leading coefficient, content, primitive part.
//! These methods are cached using thread-local LRU caching for performance.

use super::classification::PolynomialClassification;
use crate::core::{Expression, Symbol};

mod content;
mod degree;
#[cfg(test)]
mod tests;

// Re-export for internal use within polynomial module
pub(crate) use content::{compute_content_impl, divide_by_integer};
pub(crate) use degree::{
    compute_leading_coefficient_impl, compute_total_degree_impl, degree_cached,
};

/// Trait for polynomial properties
///
/// Provides methods for computing polynomial properties such as degree,
/// leading coefficient, content, and primitive part. Results are cached
/// using thread-local LRU caching for performance.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::polynomial::{PolynomialClassification, PolynomialProperties};
/// use mathhook_core::{expr, symbol};
///
/// let x = symbol!(x);
/// let poly = expr!(x ^ 3);
///
/// assert_eq!(poly.degree(&x), Some(3));
/// ```
pub trait PolynomialProperties: PolynomialClassification {
    /// Compute degree with respect to a variable
    ///
    /// Returns the highest power of the variable in the polynomial.
    ///
    /// # Arguments
    ///
    /// * `var` - The variable to compute degree for
    ///
    /// # Returns
    ///
    /// `Some(degree)` if the expression is a polynomial in the variable,
    /// `None` if not a polynomial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::polynomial::PolynomialProperties;
    /// use mathhook_core::{expr, symbol};
    ///
    /// let x = symbol!(x);
    ///
    /// assert_eq!(expr!(x ^ 3).degree(&x), Some(3));
    /// assert_eq!(expr!(5).degree(&x), Some(0));
    /// ```
    fn degree(&self, var: &Symbol) -> Option<i64>;

    /// Compute total degree (sum of degrees in all variables)
    ///
    /// For multivariate polynomials, returns the maximum sum of exponents
    /// across all terms.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::polynomial::PolynomialProperties;
    /// use mathhook_core::{expr, symbol};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let poly = expr!(x * y);  // x*y has total degree 2
    ///
    /// assert_eq!(poly.total_degree(), Some(2));
    /// ```
    fn total_degree(&self) -> Option<i64>;

    /// Get leading coefficient with respect to a variable
    ///
    /// Returns the coefficient of the highest degree term.
    ///
    /// # Arguments
    ///
    /// * `var` - The variable to find leading coefficient for
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::polynomial::PolynomialProperties;
    /// use mathhook_core::{expr, symbol};
    ///
    /// let x = symbol!(x);
    /// let poly = expr!((5 * (x ^ 2)) + (3 * x) + 1);
    ///
    /// // Leading coefficient of 5x^2 + 3x + 1 is 5
    /// let lc = poly.leading_coefficient(&x);
    /// ```
    fn leading_coefficient(&self, var: &Symbol) -> Expression;

    /// Extract content (GCD of all coefficients)
    ///
    /// The content is the GCD of all numeric coefficients in the polynomial.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::polynomial::PolynomialProperties;
    /// use mathhook_core::{expr, symbol};
    ///
    /// let x = symbol!(x);
    /// let poly = expr!((6 * (x ^ 2)) + (9 * x) + 3);
    ///
    /// // Content of 6x^2 + 9x + 3 is 3
    /// let content = poly.content();
    /// ```
    fn content(&self) -> Expression;

    /// Extract primitive part (polynomial divided by content)
    ///
    /// The primitive part is the polynomial with content factored out.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::polynomial::PolynomialProperties;
    /// use mathhook_core::{expr, symbol};
    ///
    /// let x = symbol!(x);
    /// let poly = expr!((6 * (x ^ 2)) + (9 * x) + 3);
    ///
    /// // Primitive part of 6x^2 + 9x + 3 is 2x^2 + 3x + 1
    /// let pp = poly.primitive_part();
    /// ```
    fn primitive_part(&self) -> Expression;
}

impl PolynomialProperties for Expression {
    fn degree(&self, var: &Symbol) -> Option<i64> {
        degree_cached(self, var)
    }

    fn total_degree(&self) -> Option<i64> {
        let vars = self.polynomial_variables();
        if vars.is_empty() {
            return Some(0);
        }

        // For total degree, we need to find the maximum sum of degrees in any term
        compute_total_degree_impl(self, &vars)
    }

    fn leading_coefficient(&self, var: &Symbol) -> Expression {
        compute_leading_coefficient_impl(self, var)
    }

    fn content(&self) -> Expression {
        compute_content_impl(self)
    }

    fn primitive_part(&self) -> Expression {
        let content = self.content();
        if content.is_zero() || content == Expression::integer(1) {
            self.clone()
        } else {
            // Divide polynomial by content
            divide_by_integer(self, &content)
        }
    }
}
