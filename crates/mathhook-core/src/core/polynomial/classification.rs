//! Polynomial Classification
//!
//! Automatic detection of polynomial structure for intelligent routing.
//! The `PolynomialClassification` trait provides methods for checking
//! polynomial structure and classifying expressions.

use crate::core::expression::ExpressionClass;
use crate::core::polynomial::poly::IntPoly;
use crate::core::{Expression, Number, Symbol};

/// Trait for polynomial classification
///
/// Provides automatic detection of polynomial structure.
/// Implemented for `Expression` to enable classification-based
/// algorithm routing.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::core::polynomial::PolynomialClassification;
/// use mathhook_core::core::ExpressionClass;
/// use mathhook_core::{expr, symbol};
///
/// let x = symbol!(x);
/// let poly = expr!(x ^ 2 + x + 1);
///
/// assert!(poly.is_polynomial());
/// assert_eq!(poly.polynomial_variables().len(), 1);
///
/// match poly.classify() {
///     ExpressionClass::UnivariatePolynomial { degree, .. } => {
///         assert_eq!(degree, 2);
///     }
///     _ => panic!("Expected univariate polynomial"),
/// }
/// ```
pub trait PolynomialClassification {
    /// Check if expression is a valid polynomial
    ///
    /// A polynomial is an expression composed only of:
    /// - Constants (integers, rationals)
    /// - Symbols (variables)
    /// - Addition and subtraction
    /// - Multiplication
    /// - Non-negative integer powers
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::polynomial::PolynomialClassification;
    /// use mathhook_core::{expr, symbol};
    ///
    /// let x = symbol!(x);
    ///
    /// // Polynomials
    /// assert!(expr!(x ^ 2).is_polynomial());
    /// assert!(expr!(x + 1).is_polynomial());
    /// assert!(expr!(3 * x).is_polynomial());
    ///
    /// // Not polynomials
    /// assert!(!expr!(sin(x)).is_polynomial());
    /// ```
    fn is_polynomial(&self) -> bool;

    /// Check if polynomial in specific variables
    ///
    /// Returns true if the expression is a polynomial when treating
    /// only the given variables as indeterminates (others are constants).
    ///
    /// # Arguments
    ///
    /// * `vars` - The variables to treat as indeterminates
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::polynomial::PolynomialClassification;
    /// use mathhook_core::{expr, symbol};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let poly = expr!(x * y);
    ///
    /// assert!(poly.is_polynomial_in(&[x.clone()]));
    /// assert!(poly.is_polynomial_in(&[y.clone()]));
    /// assert!(poly.is_polynomial_in(&[x.clone(), y.clone()]));
    /// ```
    fn is_polynomial_in(&self, vars: &[Symbol]) -> bool;

    /// Get polynomial variables (empty if not polynomial)
    ///
    /// Returns all symbols that appear in the expression.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::polynomial::PolynomialClassification;
    /// use mathhook_core::{expr, symbol};
    ///
    /// let x = symbol!(x);
    /// let y = symbol!(y);
    /// let poly = expr!(x + y);
    ///
    /// let vars = poly.polynomial_variables();
    /// assert_eq!(vars.len(), 2);
    /// ```
    fn polynomial_variables(&self) -> Vec<Symbol>;

    /// Classify expression type for routing
    ///
    /// Returns the classification that determines which algorithm to use.
    /// This enables intelligent dispatch to specialized algorithms.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::polynomial::PolynomialClassification;
    /// use mathhook_core::core::ExpressionClass;
    /// use mathhook_core::{expr, symbol};
    ///
    /// let x = symbol!(x);
    ///
    /// // Integer classification
    /// assert_eq!(expr!(5).classify(), ExpressionClass::Integer);
    ///
    /// // Univariate polynomial
    /// match expr!(x ^ 2).classify() {
    ///     ExpressionClass::UnivariatePolynomial { degree, .. } => {
    ///         assert_eq!(degree, 2);
    ///     }
    ///     _ => panic!("Expected univariate polynomial"),
    /// }
    /// ```
    fn classify(&self) -> ExpressionClass;

    /// Check if expression can be represented as IntPoly
    ///
    /// Returns true if the expression is a univariate polynomial
    /// with integer coefficients only. This is a fast heuristic check.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::polynomial::PolynomialClassification;
    /// use mathhook_core::{expr, symbol};
    ///
    /// let x = symbol!(x);
    ///
    /// assert!(expr!(x ^ 2 + 2 * x + 1).is_intpoly_compatible());
    /// assert!(!expr!(1.5 * x + 2).is_intpoly_compatible());
    /// ```
    fn is_intpoly_compatible(&self) -> bool;

    /// Try to convert to IntPoly
    ///
    /// Returns IntPoly and variable if expression is a univariate
    /// integer polynomial, None otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook_core::core::polynomial::PolynomialClassification;
    /// use mathhook_core::{expr, symbol};
    ///
    /// let x = symbol!(x);
    /// let poly_expr = expr!(x ^ 2 + 2 * x + 3);
    ///
    /// if let Some((intpoly, var)) = poly_expr.try_as_intpoly() {
    ///     assert_eq!(var, x);
    ///     assert_eq!(intpoly.degree(), Some(2));
    /// }
    /// ```
    fn try_as_intpoly(&self) -> Option<(IntPoly, Symbol)>;
}

impl PolynomialClassification for Expression {
    fn is_polynomial(&self) -> bool {
        is_polynomial_impl(self)
    }

    fn is_polynomial_in(&self, vars: &[Symbol]) -> bool {
        is_polynomial_in_impl(self, vars)
    }

    fn polynomial_variables(&self) -> Vec<Symbol> {
        collect_polynomial_variables(self)
    }

    fn classify(&self) -> ExpressionClass {
        classify_impl(self)
    }

    fn is_intpoly_compatible(&self) -> bool {
        let vars = self.polynomial_variables();
        if vars.len() != 1 {
            return false;
        }
        has_only_integer_coefficients(self)
    }

    fn try_as_intpoly(&self) -> Option<(IntPoly, Symbol)> {
        let vars = self.polynomial_variables();
        if vars.len() != 1 {
            return None;
        }
        let var = &vars[0];
        IntPoly::try_from_expression(self, var).map(|poly| (poly, var.clone()))
    }
}

/// Extract integer value from expression if it's an integer
fn extract_integer(expr: &Expression) -> Option<i64> {
    match expr {
        Expression::Number(Number::Integer(n)) => Some(*n),
        _ => None,
    }
}

/// Check if expression is a rational number
fn is_rational(expr: &Expression) -> bool {
    matches!(expr, Expression::Number(Number::Rational(_)))
}

/// Check if expression is a polynomial (no transcendental functions, positive powers only)
fn is_polynomial_impl(expr: &Expression) -> bool {
    match expr {
        Expression::Number(_) => true,
        Expression::Symbol(_) => true,
        Expression::Add(terms) | Expression::Mul(terms) => terms.iter().all(is_polynomial_impl),
        Expression::Pow(base, exp) => {
            if !is_polynomial_impl(base) {
                return false;
            }
            if let Some(n) = extract_integer(exp) {
                n >= 0
            } else {
                false
            }
        }
        Expression::Function { .. } => false,
        _ => false,
    }
}

/// Check if expression is a polynomial in specific variables
fn is_polynomial_in_impl(expr: &Expression, vars: &[Symbol]) -> bool {
    match expr {
        Expression::Number(_) => true,
        Expression::Symbol(_s) => true,
        Expression::Add(terms) | Expression::Mul(terms) => {
            terms.iter().all(|t| is_polynomial_in_impl(t, vars))
        }
        Expression::Pow(base, exp) => {
            if !is_polynomial_in_impl(base, vars) {
                return false;
            }
            if let Some(n) = extract_integer(exp) {
                n >= 0
            } else {
                let exp_vars = collect_polynomial_variables(exp);
                !exp_vars.iter().any(|v| vars.contains(v))
            }
        }
        Expression::Function { .. } => false,
        _ => false,
    }
}

/// Collect all variables from a polynomial expression
fn collect_polynomial_variables(expr: &Expression) -> Vec<Symbol> {
    use std::collections::HashSet;
    let mut vars = HashSet::new();
    collect_vars_impl(expr, &mut vars);
    vars.into_iter().collect()
}

fn collect_vars_impl(expr: &Expression, vars: &mut std::collections::HashSet<Symbol>) {
    match expr {
        Expression::Symbol(s) => {
            vars.insert(s.clone());
        }
        Expression::Add(terms) | Expression::Mul(terms) => {
            for term in terms.iter() {
                collect_vars_impl(term, vars);
            }
        }
        Expression::Pow(base, exp) => {
            collect_vars_impl(base, vars);
            collect_vars_impl(exp, vars);
        }
        _ => {}
    }
}

/// Classify expression for algorithm routing
fn classify_impl(expr: &Expression) -> ExpressionClass {
    if extract_integer(expr).is_some() {
        return ExpressionClass::Integer;
    }

    if !is_polynomial_impl(expr) {
        if contains_transcendental(expr) {
            return ExpressionClass::Transcendental;
        }
        return ExpressionClass::Symbolic;
    }

    let vars = collect_polynomial_variables(expr);

    match vars.len() {
        0 => {
            if is_rational(expr) {
                ExpressionClass::Rational
            } else {
                ExpressionClass::Integer
            }
        }
        1 => {
            let var = vars.into_iter().next().unwrap();
            let degree = compute_degree(expr, &var).unwrap_or(0);
            ExpressionClass::UnivariatePolynomial { var, degree }
        }
        _ => {
            let total_degree = vars.iter().filter_map(|v| compute_degree(expr, v)).sum();
            ExpressionClass::MultivariatePolynomial { vars, total_degree }
        }
    }
}

/// Check if expression contains transcendental functions
fn contains_transcendental(expr: &Expression) -> bool {
    match expr {
        Expression::Function { name, .. } => {
            let transcendental_fns = [
                "sin", "cos", "tan", "cot", "sec", "csc", "sinh", "cosh", "tanh", "exp", "log",
                "ln", "arcsin", "arccos", "arctan",
            ];
            transcendental_fns.contains(&name.as_str())
        }
        Expression::Add(terms) | Expression::Mul(terms) => {
            terms.iter().any(contains_transcendental)
        }
        Expression::Pow(base, exp) => contains_transcendental(base) || contains_transcendental(exp),
        _ => false,
    }
}

/// Compute degree of polynomial with respect to a variable
fn compute_degree(expr: &Expression, var: &Symbol) -> Option<i64> {
    match expr {
        Expression::Number(_) => Some(0),
        Expression::Symbol(s) => {
            if s == var {
                Some(1)
            } else {
                Some(0)
            }
        }
        Expression::Add(terms) => terms.iter().filter_map(|t| compute_degree(t, var)).max(),
        Expression::Mul(terms) => {
            let degrees: Option<Vec<i64>> = terms.iter().map(|t| compute_degree(t, var)).collect();
            degrees.map(|ds| ds.into_iter().sum())
        }
        Expression::Pow(base, exp) => {
            let base_deg = compute_degree(base, var)?;
            let exp_val = extract_integer(exp)?;
            Some(base_deg * exp_val)
        }
        _ => None,
    }
}

/// Check if expression has only integer coefficients
fn has_only_integer_coefficients(expr: &Expression) -> bool {
    match expr {
        Expression::Number(Number::Integer(_)) => true,
        Expression::Symbol(_) => true,
        Expression::Add(terms) | Expression::Mul(terms) => {
            terms.iter().all(has_only_integer_coefficients)
        }
        Expression::Pow(base, exp) => {
            has_only_integer_coefficients(base)
                && matches!(exp.as_ref(), Expression::Number(Number::Integer(n)) if *n >= 0)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_is_polynomial() {
        let x = symbol!(x);

        assert!(Expression::integer(5).is_polynomial());
        assert!(Expression::symbol(x.clone()).is_polynomial());

        let poly = expr!(x + 1);
        assert!(poly.is_polynomial());

        let poly2 = expr!(x ^ 2);
        assert!(poly2.is_polynomial());
    }

    #[test]
    fn test_classify_integer() {
        let five = Expression::integer(5);
        assert_eq!(five.classify(), ExpressionClass::Integer);
    }

    #[test]
    fn test_classify_univariate() {
        let x = symbol!(x);
        let poly = expr!(x ^ 2);

        match poly.classify() {
            ExpressionClass::UnivariatePolynomial { var, degree } => {
                assert_eq!(var, x);
                assert_eq!(degree, 2);
            }
            other => panic!("Expected UnivariatePolynomial, got {:?}", other),
        }
    }

    #[test]
    fn test_polynomial_variables() {
        let x = symbol!(x);
        let y = symbol!(y);

        let poly = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);

        let vars = poly.polynomial_variables();
        assert_eq!(vars.len(), 2);
        assert!(vars.contains(&x));
        assert!(vars.contains(&y));
    }

    #[test]
    fn test_is_polynomial_in() {
        let x = symbol!(x);
        let y = symbol!(y);
        let poly = expr!(x * y);

        assert!(poly.is_polynomial_in(std::slice::from_ref(&x)));
        assert!(poly.is_polynomial_in(std::slice::from_ref(&y)));
        assert!(poly.is_polynomial_in(&[x.clone(), y.clone()]));
    }

    #[test]
    fn test_classify_multivariate() {
        let x = symbol!(x);
        let y = symbol!(y);
        let poly = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);

        match poly.classify() {
            ExpressionClass::MultivariatePolynomial { vars, .. } => {
                assert_eq!(vars.len(), 2);
                assert!(vars.contains(&x));
                assert!(vars.contains(&y));
            }
            other => panic!("Expected MultivariatePolynomial, got {:?}", other),
        }
    }

    #[test]
    fn test_classify_transcendental() {
        let x = symbol!(x);
        let expr = Expression::function("sin", vec![Expression::symbol(x)]);

        assert_eq!(expr.classify(), ExpressionClass::Transcendental);
    }

    #[test]
    fn test_is_intpoly_compatible() {
        assert!(expr!(2 * x + 3).is_intpoly_compatible());
        assert!(expr!(x ^ 2 + 2 * x + 1).is_intpoly_compatible());

        assert!(!expr!(x + y).is_intpoly_compatible());

        assert!(!expr!(1.5 * x + 2).is_intpoly_compatible());

        assert!(!expr!(x ^ (-1)).is_intpoly_compatible());
    }

    #[test]
    fn test_try_as_intpoly() {
        let x = symbol!(x);
        let poly_expr = expr!(x ^ 2 + 2 * x + 3);

        let result = poly_expr.try_as_intpoly();
        assert!(result.is_some());

        let (intpoly, var) = result.unwrap();
        assert_eq!(var, x);
        assert_eq!(intpoly.degree(), Some(2));
        assert_eq!(intpoly.coefficients(), &[3, 2, 1]);
    }
}
