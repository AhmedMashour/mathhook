//! Expression Classification
//!
//! Classification of expression types for intelligent algorithm routing.
//! This provides a foundation for smart dispatch in polynomial operations,
//! simplification strategies, and solver selection.

use crate::core::Symbol;

/// Classification of expression type for algorithm routing
///
/// Used to select optimal algorithms based on expression structure.
/// The classification determines which specialized algorithms to use
/// for operations like GCD, factorization, and simplification.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::expression::classification::ExpressionClass;
/// use mathhook_core::core::polynomial::PolynomialClassification;
/// use mathhook_core::{expr, symbol};
///
/// let x = symbol!(x);
/// let poly = expr!(x ^ 2);
///
/// match poly.classify() {
///     ExpressionClass::UnivariatePolynomial { var, degree } => {
///         assert_eq!(degree, 2);
///     }
///     _ => panic!("Expected univariate polynomial"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionClass {
    /// Integer constant
    ///
    /// Pure integer values like 0, 1, -5, 42.
    /// Fast-path for integer arithmetic.
    Integer,

    /// Rational number
    ///
    /// Fractions like 1/2, 3/4, -7/11.
    /// Uses rational arithmetic algorithms.
    Rational,

    /// Univariate polynomial in one variable
    ///
    /// Polynomials like x^2 + 2x + 1, 3y^5 - y.
    /// Enables specialized univariate algorithms (Euclidean GCD, etc.).
    UnivariatePolynomial {
        /// The variable of the polynomial
        var: Symbol,
        /// The degree of the polynomial
        degree: i64,
    },

    /// Multivariate polynomial in multiple variables
    ///
    /// Polynomials like x^2 + xy + y^2, x*y*z + 1.
    /// Uses multivariate algorithms (Buchberger, etc.).
    MultivariatePolynomial {
        /// All variables in the polynomial
        vars: Vec<Symbol>,
        /// Total degree (sum of degrees in all variables)
        total_degree: i64,
    },

    /// Rational function (ratio of polynomials)
    ///
    /// Expressions like (x+1)/(x-1), 1/(x^2+1).
    /// Requires special handling for simplification.
    RationalFunction,

    /// Contains transcendental functions (sin, cos, exp, log)
    ///
    /// Expressions involving sin, cos, tan, exp, log, etc.
    /// Limited algebraic simplification possible.
    Transcendental,

    /// Symbolic expression that doesn't fit other categories
    ///
    /// General symbolic expressions that don't match
    /// any of the more specific classifications.
    Symbolic,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_class_eq() {
        assert_eq!(ExpressionClass::Integer, ExpressionClass::Integer);
        assert_eq!(ExpressionClass::Rational, ExpressionClass::Rational);
        assert_ne!(ExpressionClass::Integer, ExpressionClass::Rational);
    }

    #[test]
    fn test_expression_class_clone() {
        let class = ExpressionClass::UnivariatePolynomial {
            var: crate::symbol!(x),
            degree: 3,
        };
        let cloned = class.clone();
        assert_eq!(class, cloned);
    }

    #[test]
    fn test_expression_class_debug() {
        let class = ExpressionClass::Integer;
        let debug_str = format!("{:?}", class);
        assert!(debug_str.contains("Integer"));
    }
}
