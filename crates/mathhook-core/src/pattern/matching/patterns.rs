//! Pattern types and constraint definitions
//!
//! Provides the core pattern types for structural matching, including
//! wildcards with constraints and exact matching patterns.

use crate::core::Expression;

/// Constraints for wildcard pattern matching
///
/// Provides fine-grained control over what expressions a wildcard can match.
#[derive(Debug, Clone)]
pub struct WildcardConstraints {
    /// Expressions that cannot be matched (e.g., specific variables to exclude)
    pub exclude: Vec<Expression>,

    /// Predicates that must return true for a match to succeed
    /// Common examples: is_integer, is_positive, is_polynomial_in(x)
    pub properties: Vec<fn(&Expression) -> bool>,
}

impl PartialEq for WildcardConstraints {
    fn eq(&self, other: &Self) -> bool {
        self.exclude == other.exclude && self.properties.len() == other.properties.len()
    }
}

impl WildcardConstraints {
    /// Create constraints with excluded expressions
    pub fn with_exclude(exclude: Vec<Expression>) -> Self {
        Self {
            exclude,
            properties: Vec::new(),
        }
    }

    /// Create constraints with property predicates
    pub fn with_properties(properties: Vec<fn(&Expression) -> bool>) -> Self {
        Self {
            exclude: Vec::new(),
            properties,
        }
    }

    /// Check if an expression satisfies all constraints
    pub fn is_satisfied_by(&self, expr: &Expression) -> bool {
        for excluded in &self.exclude {
            if expr == excluded {
                return false;
            }
        }

        for excluded in &self.exclude {
            if contains_subexpression(expr, excluded) {
                return false;
            }
        }

        for property in &self.properties {
            if !property(expr) {
                return false;
            }
        }

        true
    }
}

/// Check if an expression contains a subexpression
fn contains_subexpression(expr: &Expression, sub: &Expression) -> bool {
    if expr == sub {
        return true;
    }

    match expr {
        Expression::Add(terms) | Expression::Mul(terms) | Expression::Set(terms) => {
            terms.iter().any(|t| contains_subexpression(t, sub))
        }
        Expression::Pow(base, exp) => {
            contains_subexpression(base, sub) || contains_subexpression(exp, sub)
        }
        Expression::Function { args, .. } => args.iter().any(|a| contains_subexpression(a, sub)),
        Expression::Complex(data) => {
            contains_subexpression(&data.real, sub) || contains_subexpression(&data.imag, sub)
        }
        _ => false,
    }
}

/// A pattern that can match against expressions
///
/// Patterns support wildcards and structural matching to enable
/// transformation rules and equation manipulation.
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Match any expression and bind to a name, optionally with constraints
    Wildcard {
        name: String,
        constraints: Option<WildcardConstraints>,
    },

    /// Match a specific expression exactly
    Exact(Expression),

    /// Match addition with pattern terms (supports commutative matching)
    Add(Vec<Pattern>),

    /// Match multiplication with pattern factors (supports commutative matching)
    Mul(Vec<Pattern>),

    /// Match power with pattern base and exponent
    Pow(Box<Pattern>, Box<Pattern>),

    /// Match function call with pattern arguments
    Function { name: String, args: Vec<Pattern> },
}

impl Pattern {
    /// Create a simple wildcard pattern without constraints
    pub fn wildcard(name: impl Into<String>) -> Self {
        Pattern::Wildcard {
            name: name.into(),
            constraints: None,
        }
    }

    /// Create a wildcard pattern with exclude constraints
    pub fn wildcard_excluding(name: impl Into<String>, exclude: Vec<Expression>) -> Self {
        Pattern::Wildcard {
            name: name.into(),
            constraints: Some(WildcardConstraints::with_exclude(exclude)),
        }
    }

    /// Create a wildcard pattern with property constraints
    pub fn wildcard_with_properties(
        name: impl Into<String>,
        properties: Vec<fn(&Expression) -> bool>,
    ) -> Self {
        Pattern::Wildcard {
            name: name.into(),
            constraints: Some(WildcardConstraints::with_properties(properties)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_wildcard_constraints_exclude() {
        let x = symbol!(x);
        let constraints = WildcardConstraints::with_exclude(vec![Expression::symbol(x.clone())]);

        assert!(!constraints.is_satisfied_by(&Expression::symbol(x.clone())));
        assert!(constraints.is_satisfied_by(&Expression::integer(42)));
    }

    #[test]
    fn test_wildcard_constraints_exclude_subexpression() {
        let x = symbol!(x);
        let constraints = WildcardConstraints::with_exclude(vec![Expression::symbol(x.clone())]);

        let expr_with_x =
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
        assert!(!constraints.is_satisfied_by(&expr_with_x));
    }

    #[test]
    fn test_wildcard_constraints_properties() {
        fn is_integer(expr: &Expression) -> bool {
            matches!(expr, Expression::Number(_))
        }

        let constraints = WildcardConstraints::with_properties(vec![is_integer]);

        assert!(constraints.is_satisfied_by(&Expression::integer(42)));

        let x = symbol!(x);
        assert!(!constraints.is_satisfied_by(&Expression::symbol(x)));
    }

    #[test]
    fn test_pattern_wildcard_construction() {
        let pattern = Pattern::wildcard("x");
        match pattern {
            Pattern::Wildcard { name, constraints } => {
                assert_eq!(name.as_str(), "x");
                assert!(constraints.is_none());
            }
            _ => panic!("Expected Wildcard pattern"),
        }
    }

    #[test]
    fn test_pattern_wildcard_excluding_construction() {
        let x = symbol!(x);
        let pattern = Pattern::wildcard_excluding("a", vec![Expression::symbol(x.clone())]);

        match pattern {
            Pattern::Wildcard { name, constraints } => {
                assert_eq!(name.as_str(), "a");
                assert!(constraints.is_some());
            }
            _ => panic!("Expected Wildcard pattern"),
        }
    }

    #[test]
    fn test_pattern_wildcard_with_properties_construction() {
        fn is_integer(expr: &Expression) -> bool {
            matches!(expr, Expression::Number(_))
        }

        let pattern = Pattern::wildcard_with_properties("n", vec![is_integer]);

        match pattern {
            Pattern::Wildcard { name, constraints } => {
                assert_eq!(name.as_str(), "n");
                assert!(constraints.is_some());
            }
            _ => panic!("Expected Wildcard pattern"),
        }
    }

    #[test]
    fn test_contains_subexpression_direct() {
        let x = symbol!(x);
        let expr = Expression::symbol(x.clone());
        assert!(contains_subexpression(&expr, &expr));
    }

    #[test]
    fn test_contains_subexpression_in_add() {
        let x = symbol!(x);
        let expr = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
        assert!(contains_subexpression(
            &expr,
            &Expression::symbol(x.clone())
        ));
        assert!(contains_subexpression(&expr, &Expression::integer(1)));
    }

    #[test]
    fn test_contains_subexpression_in_mul() {
        let x = symbol!(x);
        let expr = Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);
        assert!(contains_subexpression(
            &expr,
            &Expression::symbol(x.clone())
        ));
    }

    #[test]
    fn test_contains_subexpression_in_pow() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));
        assert!(contains_subexpression(
            &expr,
            &Expression::symbol(x.clone())
        ));
        assert!(contains_subexpression(&expr, &Expression::integer(2)));
    }

    #[test]
    fn test_contains_subexpression_in_function() {
        let x = symbol!(x);
        let expr = Expression::function("sin".to_string(), vec![Expression::symbol(x.clone())]);
        assert!(contains_subexpression(
            &expr,
            &Expression::symbol(x.clone())
        ));
    }

    #[test]
    fn test_contains_subexpression_not_found() {
        let x = symbol!(x);
        let y = symbol!(y);
        let expr = Expression::symbol(x);
        assert!(!contains_subexpression(&expr, &Expression::symbol(y)));
    }
}
