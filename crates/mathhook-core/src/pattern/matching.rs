//! Pattern matching infrastructure for structural matching
//!
//! Provides pattern matching with wildcards and constraints for
//! transformation rules and algebraic manipulation.

use crate::core::{Expression, Symbol};
use std::collections::HashMap;

/// A pattern that can match against expressions
///
/// Patterns support wildcards and structural matching to enable
/// transformation rules and equation manipulation.
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Match any expression and bind to a name
    Wildcard(String),

    /// Match a specific expression exactly
    Exact(Expression),

    /// Match addition with pattern terms
    Add(Vec<Pattern>),

    /// Match multiplication with pattern factors
    Mul(Vec<Pattern>),

    /// Match power with pattern base and exponent
    Pow(Box<Pattern>, Box<Pattern>),

    /// Match function call with pattern arguments
    Function { name: String, args: Vec<Pattern> },
}

/// Result of pattern matching containing variable bindings
pub type PatternMatches = HashMap<String, Expression>;

/// Trait for types that support pattern matching
pub trait Matchable {
    /// Match this expression against a pattern
    ///
    /// Returns bindings for wildcard names if the match succeeds,
    /// or None if the pattern doesn't match.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to match against
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::prelude::*;
    /// use mathhook_core::pattern::{Pattern, Matchable};
    ///
    /// let x = symbol!(x);
    /// let expr = Expression::add(vec![
    ///     Expression::mul(vec![Expression::integer(2), Expression::symbol(x)]),
    ///     Expression::integer(1)
    /// ]);
    ///
    /// // Pattern: a*x + b
    /// let pattern = Pattern::Add(vec![
    ///     Pattern::Mul(vec![
    ///         Pattern::Wildcard("a".to_string()),
    ///         Pattern::Exact(Expression::symbol(x))
    ///     ]),
    ///     Pattern::Wildcard("b".to_string())
    /// ]);
    ///
    /// let matches = expr.matches(&pattern);
    /// assert!(matches.is_some());
    ///
    /// if let Some(bindings) = matches {
    ///     assert_eq!(bindings.get("a"), Some(&Expression::integer(2)));
    ///     assert_eq!(bindings.get("b"), Some(&Expression::integer(1)));
    /// }
    /// ```
    fn matches(&self, pattern: &Pattern) -> Option<PatternMatches>;

    /// Replace all occurrences of a pattern with a replacement expression
    ///
    /// Uses pattern matching to find matches and applies the replacement,
    /// substituting wildcards with their matched values.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to match
    /// * `replacement` - The replacement pattern (can contain wildcards from match)
    ///
    /// # Examples
    ///
    /// ```
    /// use mathhook_core::prelude::*;
    /// use mathhook_core::pattern::{Pattern, Matchable};
    ///
    /// let x = symbol!(x);
    /// // sin(x)^2 + cos(x)^2
    /// let expr = Expression::add(vec![
    ///     Expression::pow(
    ///         Expression::function("sin".to_string(), vec![Expression::symbol(x)]),
    ///         Expression::integer(2)
    ///     ),
    ///     Expression::pow(
    ///         Expression::function("cos".to_string(), vec![Expression::symbol(x)]),
    ///         Expression::integer(2)
    ///     )
    /// ]);
    ///
    /// // Pattern: sin(a)^2 + cos(a)^2
    /// let pattern = Pattern::Add(vec![
    ///     Pattern::Pow(
    ///         Box::new(Pattern::Function {
    ///             name: "sin".to_string(),
    ///             args: vec![Pattern::Wildcard("a".to_string())]
    ///         }),
    ///         Box::new(Pattern::Exact(Expression::integer(2)))
    ///     ),
    ///     Pattern::Pow(
    ///         Box::new(Pattern::Function {
    ///             name: "cos".to_string(),
    ///             args: vec![Pattern::Wildcard("a".to_string())]
    ///         }),
    ///         Box::new(Pattern::Exact(Expression::integer(2)))
    ///     )
    /// ]);
    ///
    /// // Replacement: 1
    /// let replacement = Pattern::Exact(Expression::integer(1));
    ///
    /// let result = expr.replace(&pattern, &replacement);
    /// assert_eq!(result, Expression::integer(1));
    /// ```
    fn replace(&self, pattern: &Pattern, replacement: &Pattern) -> Expression;
}

impl Matchable for Expression {
    fn matches(&self, pattern: &Pattern) -> Option<PatternMatches> {
        let mut bindings = HashMap::new();
        if match_recursive(self, pattern, &mut bindings) {
            Some(bindings)
        } else {
            None
        }
    }

    fn replace(&self, pattern: &Pattern, replacement: &Pattern) -> Expression {
        // Try to match the pattern
        if let Some(bindings) = self.matches(pattern) {
            // Apply the replacement with the bindings
            apply_replacement(replacement, &bindings)
        } else {
            // No match - recursively try to replace in subexpressions
            match self {
                Expression::Add(terms) => {
                    let new_terms: Vec<Expression> =
                        terms.iter().map(|t| t.replace(pattern, replacement)).collect();
                    Expression::Add(Box::new(new_terms))
                }

                Expression::Mul(factors) => {
                    let new_factors: Vec<Expression> = factors
                        .iter()
                        .map(|f| f.replace(pattern, replacement))
                        .collect();
                    Expression::Mul(Box::new(new_factors))
                }

                Expression::Pow(base, exp) => {
                    let new_base = base.replace(pattern, replacement);
                    let new_exp = exp.replace(pattern, replacement);
                    Expression::Pow(Box::new(new_base), Box::new(new_exp))
                }

                Expression::Function { name, args } => {
                    let new_args: Vec<Expression> =
                        args.iter().map(|a| a.replace(pattern, replacement)).collect();
                    Expression::Function {
                        name: name.clone(),
                        args: Box::new(new_args),
                    }
                }

                // For other types, return as-is (no replacement possible)
                _ => self.clone(),
            }
        }
    }
}

/// Recursive helper for pattern matching
///
/// Attempts to match an expression against a pattern, accumulating
/// wildcard bindings in the provided HashMap.
fn match_recursive(
    expr: &Expression,
    pattern: &Pattern,
    bindings: &mut PatternMatches,
) -> bool {
    match pattern {
        // Wildcard - match anything and bind
        Pattern::Wildcard(name) => {
            // Check if this wildcard was already bound
            if let Some(existing) = bindings.get(name) {
                // Must match the existing binding
                expr == existing
            } else {
                // Bind the wildcard
                bindings.insert(name.clone(), expr.clone());
                true
            }
        }

        // Exact match
        Pattern::Exact(pattern_expr) => expr == pattern_expr,

        // Match addition
        Pattern::Add(pattern_terms) => {
            if let Expression::Add(expr_terms) = expr {
                // Both must have same number of terms
                if expr_terms.len() != pattern_terms.len() {
                    return false;
                }

                // Try to match each term
                for (expr_term, pattern_term) in expr_terms.iter().zip(pattern_terms.iter()) {
                    if !match_recursive(expr_term, pattern_term, bindings) {
                        return false;
                    }
                }

                true
            } else {
                false
            }
        }

        // Match multiplication
        Pattern::Mul(pattern_factors) => {
            if let Expression::Mul(expr_factors) = expr {
                // Both must have same number of factors
                if expr_factors.len() != pattern_factors.len() {
                    return false;
                }

                // Try to match each factor
                for (expr_factor, pattern_factor) in
                    expr_factors.iter().zip(pattern_factors.iter())
                {
                    if !match_recursive(expr_factor, pattern_factor, bindings) {
                        return false;
                    }
                }

                true
            } else {
                false
            }
        }

        // Match power
        Pattern::Pow(pattern_base, pattern_exp) => {
            if let Expression::Pow(expr_base, expr_exp) = expr {
                match_recursive(expr_base, pattern_base, bindings)
                    && match_recursive(expr_exp, pattern_exp, bindings)
            } else {
                false
            }
        }

        // Match function
        Pattern::Function { name, args } => {
            if let Expression::Function {
                name: expr_name,
                args: expr_args,
            } = expr
            {
                // Names must match
                if expr_name != name {
                    return false;
                }

                // Number of arguments must match
                if expr_args.len() != args.len() {
                    return false;
                }

                // Try to match each argument
                for (expr_arg, pattern_arg) in expr_args.iter().zip(args.iter()) {
                    if !match_recursive(expr_arg, pattern_arg, bindings) {
                        return false;
                    }
                }

                true
            } else {
                false
            }
        }
    }
}

/// Apply a replacement pattern with bindings from a match
fn apply_replacement(replacement: &Pattern, bindings: &PatternMatches) -> Expression {
    match replacement {
        // Wildcard - substitute with binding
        Pattern::Wildcard(name) => {
            bindings
                .get(name)
                .cloned()
                .unwrap_or_else(|| panic!("Unbound wildcard in replacement: {}", name))
        }

        // Exact - return the expression
        Pattern::Exact(expr) => expr.clone(),

        // Reconstruct add
        Pattern::Add(terms) => {
            let new_terms: Vec<Expression> =
                terms.iter().map(|t| apply_replacement(t, bindings)).collect();
            Expression::Add(Box::new(new_terms))
        }

        // Reconstruct mul
        Pattern::Mul(factors) => {
            let new_factors: Vec<Expression> = factors
                .iter()
                .map(|f| apply_replacement(f, bindings))
                .collect();
            Expression::Mul(Box::new(new_factors))
        }

        // Reconstruct pow
        Pattern::Pow(base, exp) => {
            let new_base = apply_replacement(base, bindings);
            let new_exp = apply_replacement(exp, bindings);
            Expression::Pow(Box::new(new_base), Box::new(new_exp))
        }

        // Reconstruct function
        Pattern::Function { name, args } => {
            let new_args: Vec<Expression> =
                args.iter().map(|a| apply_replacement(a, bindings)).collect();
            Expression::Function {
                name: name.clone(),
                args: Box::new(new_args),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_wildcard_pattern_matches() {
        let expr = Expression::integer(42);
        let pattern = Pattern::Wildcard("x".to_string());

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());

        if let Some(bindings) = matches {
            assert_eq!(bindings.get("x"), Some(&Expression::integer(42)));
        }
    }

    #[test]
    fn test_exact_pattern_matches() {
        let expr = Expression::integer(42);
        let pattern = Pattern::Exact(Expression::integer(42));

        assert!(expr.matches(&pattern).is_some());
    }

    #[test]
    fn test_exact_pattern_no_match() {
        let expr = Expression::integer(42);
        let pattern = Pattern::Exact(Expression::integer(43));

        assert!(expr.matches(&pattern).is_none());
    }

    #[test]
    fn test_addition_pattern() {
        let x = symbol!(x);
        let expr = Expression::add(vec![Expression::symbol(x), Expression::integer(1)]);

        let pattern = Pattern::Add(vec![
            Pattern::Wildcard("a".to_string()),
            Pattern::Wildcard("b".to_string()),
        ]);

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());

        if let Some(bindings) = matches {
            assert_eq!(bindings.get("a"), Some(&Expression::symbol(x)));
            assert_eq!(bindings.get("b"), Some(&Expression::integer(1)));
        }
    }

    #[test]
    fn test_multiplication_pattern() {
        let x = symbol!(x);
        let expr = Expression::mul(vec![Expression::integer(2), Expression::symbol(x)]);

        let pattern = Pattern::Mul(vec![
            Pattern::Exact(Expression::integer(2)),
            Pattern::Wildcard("x".to_string()),
        ]);

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());

        if let Some(bindings) = matches {
            assert_eq!(bindings.get("x"), Some(&Expression::symbol(x)));
        }
    }

    #[test]
    fn test_power_pattern() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x), Expression::integer(2));

        let pattern = Pattern::Pow(
            Box::new(Pattern::Wildcard("base".to_string())),
            Box::new(Pattern::Exact(Expression::integer(2))),
        );

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());

        if let Some(bindings) = matches {
            assert_eq!(bindings.get("base"), Some(&Expression::symbol(x)));
        }
    }

    #[test]
    fn test_function_pattern() {
        let x = symbol!(x);
        let expr = Expression::function("sin".to_string(), vec![Expression::symbol(x)]);

        let pattern = Pattern::Function {
            name: "sin".to_string(),
            args: vec![Pattern::Wildcard("arg".to_string())],
        };

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());

        if let Some(bindings) = matches {
            assert_eq!(bindings.get("arg"), Some(&Expression::symbol(x)));
        }
    }

    #[test]
    fn test_wildcard_consistency() {
        let x = symbol!(x);
        // x + x (same variable twice)
        let expr = Expression::add(vec![Expression::symbol(x), Expression::symbol(x)]);

        // Pattern: a + a (same wildcard twice - should match when both are same)
        let pattern = Pattern::Add(vec![
            Pattern::Wildcard("a".to_string()),
            Pattern::Wildcard("a".to_string()),
        ]);

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());

        if let Some(bindings) = matches {
            assert_eq!(bindings.get("a"), Some(&Expression::symbol(x)));
        }
    }

    #[test]
    fn test_wildcard_inconsistency() {
        let x = symbol!(x);
        let y = symbol!(y);
        // x + y (different variables)
        let expr = Expression::add(vec![Expression::symbol(x), Expression::symbol(y)]);

        // Pattern: a + a (same wildcard twice - should NOT match when different)
        let pattern = Pattern::Add(vec![
            Pattern::Wildcard("a".to_string()),
            Pattern::Wildcard("a".to_string()),
        ]);

        assert!(expr.matches(&pattern).is_none());
    }

    #[test]
    fn test_simple_replacement() {
        let x = symbol!(x);
        let expr = Expression::symbol(x);

        let pattern = Pattern::Wildcard("x".to_string());
        let replacement = Pattern::Exact(Expression::integer(5));

        let result = expr.replace(&pattern, &replacement);
        assert_eq!(result, Expression::integer(5));
    }

    #[test]
    fn test_replacement_in_addition() {
        let x = symbol!(x);
        // x + 1
        let expr = Expression::add(vec![Expression::symbol(x), Expression::integer(1)]);

        // Pattern: a + b -> b + a (swap)
        let pattern = Pattern::Add(vec![
            Pattern::Wildcard("a".to_string()),
            Pattern::Wildcard("b".to_string()),
        ]);

        let replacement = Pattern::Add(vec![
            Pattern::Wildcard("b".to_string()),
            Pattern::Wildcard("a".to_string()),
        ]);

        let result = expr.replace(&pattern, &replacement);

        // Should be 1 + x
        let expected = Expression::add(vec![Expression::integer(1), Expression::symbol(x)]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_trig_identity_replacement() {
        let x = symbol!(x);
        // sin(x)^2 + cos(x)^2
        let expr = Expression::add(vec![
            Expression::pow(
                Expression::function("sin".to_string(), vec![Expression::symbol(x)]),
                Expression::integer(2),
            ),
            Expression::pow(
                Expression::function("cos".to_string(), vec![Expression::symbol(x)]),
                Expression::integer(2),
            ),
        ]);

        // Pattern: sin(a)^2 + cos(a)^2
        let pattern = Pattern::Add(vec![
            Pattern::Pow(
                Box::new(Pattern::Function {
                    name: "sin".to_string(),
                    args: vec![Pattern::Wildcard("a".to_string())],
                }),
                Box::new(Pattern::Exact(Expression::integer(2))),
            ),
            Pattern::Pow(
                Box::new(Pattern::Function {
                    name: "cos".to_string(),
                    args: vec![Pattern::Wildcard("a".to_string())],
                }),
                Box::new(Pattern::Exact(Expression::integer(2))),
            ),
        ]);

        // Replacement: 1
        let replacement = Pattern::Exact(Expression::integer(1));

        let result = expr.replace(&pattern, &replacement);
        assert_eq!(result, Expression::integer(1));
    }
}
