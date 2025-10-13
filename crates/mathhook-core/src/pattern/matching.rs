//! Pattern matching infrastructure for structural matching
//!
//! Provides pattern matching with wildcards and constraints for
//! transformation rules and algebraic manipulation.

use crate::core::{Expression, Symbol};
use std::collections::HashMap;

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
        // Compare exclude lists, but properties cannot be compared
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
        // Check exclude list - expression must not equal any excluded expression
        for excluded in &self.exclude {
            if expr == excluded {
                return false;
            }
        }

        // Check if expression contains any excluded subexpression
        for excluded in &self.exclude {
            if contains_subexpression(expr, excluded) {
                return false;
            }
        }

        // Check all property predicates
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
    ///     Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
    ///     Expression::integer(1)
    /// ]);
    ///
    /// // Pattern: a*x + b
    /// let pattern = Pattern::Add(vec![
    ///     Pattern::Mul(vec![
    ///         Pattern::Wildcard("a".to_string()),
    ///         Pattern::Exact(Expression::symbol(x.clone()))
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
    ///         Expression::function("sin".to_string(), vec![Expression::symbol(x.clone())]),
    ///         Expression::integer(2)
    ///     ),
    ///     Expression::pow(
    ///         Expression::function("cos".to_string(), vec![Expression::symbol(x.clone())]),
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
        // Wildcard - match anything that satisfies constraints and bind
        Pattern::Wildcard { name, constraints } => {
            // Check constraints first
            if let Some(constraints) = constraints {
                if !constraints.is_satisfied_by(expr) {
                    return false;
                }
            }

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

        // Match addition with commutative support
        Pattern::Add(pattern_terms) => {
            if let Expression::Add(expr_terms) = expr {
                match_commutative(expr_terms, pattern_terms, bindings)
            } else {
                false
            }
        }

        // Match multiplication with commutative support
        Pattern::Mul(pattern_factors) => {
            if let Expression::Mul(expr_factors) = expr {
                match_commutative(expr_factors, pattern_factors, bindings)
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

/// Match commutative operations (Add, Mul)
///
/// Tries to match expression terms/factors against pattern terms/factors
/// considering all possible orderings. This is essential for algebraic
/// pattern matching since addition and multiplication are commutative.
fn match_commutative(
    expr_items: &[Expression],
    pattern_items: &[Pattern],
    bindings: &mut PatternMatches,
) -> bool {
    // Special case: empty patterns
    if pattern_items.is_empty() {
        return expr_items.is_empty();
    }

    // Special case: single item (no ordering to try)
    if pattern_items.len() == 1 {
        if expr_items.len() == 1 {
            return match_recursive(&expr_items[0], &pattern_items[0], bindings);
        } else {
            return false;
        }
    }

    // Try simple ordered match first (common case optimization)
    if expr_items.len() == pattern_items.len() {
        let backup_bindings = bindings.clone();
        let mut ordered_match = true;

        for (expr_item, pattern_item) in expr_items.iter().zip(pattern_items.iter()) {
            if !match_recursive(expr_item, pattern_item, bindings) {
                ordered_match = false;
                break;
            }
        }

        if ordered_match {
            return true;
        }

        // Restore bindings if ordered match failed
        *bindings = backup_bindings;
    }

    // For small number of items, try permutations
    // For larger numbers, use heuristic matching
    if pattern_items.len() <= 6 {
        try_permutation_match(expr_items, pattern_items, bindings)
    } else {
        // For large patterns, use greedy heuristic matching
        try_greedy_match(expr_items, pattern_items, bindings)
    }
}

/// Try all permutations of pattern items to find a match
fn try_permutation_match(
    expr_items: &[Expression],
    pattern_items: &[Pattern],
    bindings: &mut PatternMatches,
) -> bool {
    use std::collections::HashSet;

    if expr_items.len() != pattern_items.len() {
        return false;
    }

    // Generate permutations and try each one
    let indices: Vec<usize> = (0..pattern_items.len()).collect();
    try_permutations(&indices, 0, expr_items, pattern_items, bindings)
}

/// Recursive permutation generator and matcher
fn try_permutations(
    indices: &[usize],
    start: usize,
    expr_items: &[Expression],
    pattern_items: &[Pattern],
    bindings: &mut PatternMatches,
) -> bool {
    if start == indices.len() {
        // Try this permutation
        let backup_bindings = bindings.clone();
        for (expr_idx, &pattern_idx) in indices.iter().enumerate() {
            if !match_recursive(&expr_items[expr_idx], &pattern_items[pattern_idx], bindings) {
                *bindings = backup_bindings;
                return false;
            }
        }
        return true;
    }

    // Generate permutations by swapping
    for i in start..indices.len() {
        let mut perm = indices.to_vec();
        perm.swap(start, i);
        if try_permutations(&perm, start + 1, expr_items, pattern_items, bindings) {
            return true;
        }
    }

    false
}

/// Greedy heuristic matching for large commutative patterns
fn try_greedy_match(
    expr_items: &[Expression],
    pattern_items: &[Pattern],
    bindings: &mut PatternMatches,
) -> bool {
    if expr_items.len() != pattern_items.len() {
        return false;
    }

    let mut used_expr: Vec<bool> = vec![false; expr_items.len()];
    let backup_bindings = bindings.clone();

    // Try to match each pattern greedily
    for pattern_item in pattern_items {
        let mut matched = false;
        for (expr_idx, expr_item) in expr_items.iter().enumerate() {
            if !used_expr[expr_idx] {
                let test_bindings = bindings.clone();
                let mut temp_bindings = bindings.clone();
                if match_recursive(expr_item, pattern_item, &mut temp_bindings) {
                    *bindings = temp_bindings;
                    used_expr[expr_idx] = true;
                    matched = true;
                    break;
                }
            }
        }

        if !matched {
            *bindings = backup_bindings;
            return false;
        }
    }

    true
}

/// Apply a replacement pattern with bindings from a match
fn apply_replacement(replacement: &Pattern, bindings: &PatternMatches) -> Expression {
    match replacement {
        // Wildcard - substitute with binding (constraints don't matter for replacement)
        Pattern::Wildcard { name, .. } => {
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
    fn test_wildcard_pattern_matches() {
        let expr = Expression::integer(42);
        let pattern = Pattern::wildcard("x");

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
        let expr = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);

        let pattern = Pattern::Add(vec![Pattern::wildcard("a"), Pattern::wildcard("b")]);

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());

        if let Some(bindings) = matches {
            // Commutative matching - bindings can be in either order
            let a_val = bindings.get("a").unwrap();
            let b_val = bindings.get("b").unwrap();

            assert!(
                (a_val == &Expression::symbol(x.clone()) && b_val == &Expression::integer(1))
                    || (a_val == &Expression::integer(1) && b_val == &Expression::symbol(x.clone()))
            );
        }
    }

    #[test]
    fn test_multiplication_pattern() {
        let x = symbol!(x);
        let expr = Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]);

        let pattern = Pattern::Mul(vec![
            Pattern::Exact(Expression::integer(2)),
            Pattern::wildcard("x"),
        ]);

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());

        if let Some(bindings) = matches {
            assert_eq!(bindings.get("x"), Some(&Expression::symbol(x.clone())));
        }
    }

    #[test]
    fn test_power_pattern() {
        let x = symbol!(x);
        let expr = Expression::pow(Expression::symbol(x.clone()), Expression::integer(2));

        let pattern = Pattern::Pow(
            Box::new(Pattern::wildcard("base")),
            Box::new(Pattern::Exact(Expression::integer(2))),
        );

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());

        if let Some(bindings) = matches {
            assert_eq!(bindings.get("base"), Some(&Expression::symbol(x.clone())));
        }
    }

    #[test]
    fn test_function_pattern() {
        let x = symbol!(x);
        let expr = Expression::function("sin".to_string(), vec![Expression::symbol(x.clone())]);

        let pattern = Pattern::Function {
            name: "sin".to_string(),
            args: vec![Pattern::wildcard("arg")],
        };

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());

        if let Some(bindings) = matches {
            assert_eq!(bindings.get("arg"), Some(&Expression::symbol(x.clone())));
        }
    }

    #[test]
    fn test_wildcard_consistency() {
        let x = symbol!(x);
        // x + x (same variable twice)
        let expr = Expression::add(vec![Expression::symbol(x.clone()), Expression::symbol(x.clone())]);

        // Pattern: a + a (same wildcard twice - should match when both are same)
        let pattern = Pattern::Add(vec![Pattern::wildcard("a"), Pattern::wildcard("a")]);

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());

        if let Some(bindings) = matches {
            assert_eq!(bindings.get("a"), Some(&Expression::symbol(x.clone())));
        }
    }

    #[test]
    fn test_wildcard_inconsistency() {
        let x = symbol!(x);
        let y = symbol!(y);
        // x + y (different variables)
        let expr = Expression::add(vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())]);

        // Pattern: a + a (same wildcard twice - should NOT match when different)
        let pattern = Pattern::Add(vec![Pattern::wildcard("a"), Pattern::wildcard("a")]);

        assert!(expr.matches(&pattern).is_none());
    }

    #[test]
    fn test_simple_replacement() {
        let x = symbol!(x);
        let expr = Expression::symbol(x.clone());

        let pattern = Pattern::wildcard("x");
        let replacement = Pattern::Exact(Expression::integer(5));

        let result = expr.replace(&pattern, &replacement);
        assert_eq!(result, Expression::integer(5));
    }

    #[test]
    fn test_replacement_in_addition() {
        let x = symbol!(x);
        // x + 1
        let expr = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);

        // Pattern: a + b -> b + a (swap)
        let pattern = Pattern::Add(vec![Pattern::wildcard("a"), Pattern::wildcard("b")]);

        let replacement = Pattern::Add(vec![Pattern::wildcard("b"), Pattern::wildcard("a")]);

        let result = expr.replace(&pattern, &replacement);

        // Should be 1 + x
        let expected = Expression::add(vec![Expression::integer(1), Expression::symbol(x.clone())]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_trig_identity_replacement() {
        let x = symbol!(x);
        // sin(x)^2 + cos(x)^2
        let expr = Expression::add(vec![
            Expression::pow(
                Expression::function("sin".to_string(), vec![Expression::symbol(x.clone())]),
                Expression::integer(2),
            ),
            Expression::pow(
                Expression::function("cos".to_string(), vec![Expression::symbol(x.clone())]),
                Expression::integer(2),
            ),
        ]);

        // Pattern: sin(a)^2 + cos(a)^2
        let pattern = Pattern::Add(vec![
            Pattern::Pow(
                Box::new(Pattern::Function {
                    name: "sin".to_string(),
                    args: vec![Pattern::wildcard("a")],
                }),
                Box::new(Pattern::Exact(Expression::integer(2))),
            ),
            Pattern::Pow(
                Box::new(Pattern::Function {
                    name: "cos".to_string(),
                    args: vec![Pattern::wildcard("a")],
                }),
                Box::new(Pattern::Exact(Expression::integer(2))),
            ),
        ]);

        // Replacement: 1
        let replacement = Pattern::Exact(Expression::integer(1));

        let result = expr.replace(&pattern, &replacement);
        assert_eq!(result, Expression::integer(1));
    }

    // NEW TESTS FOR ENHANCED FUNCTIONALITY

    #[test]
    fn test_commutative_addition_matching() {
        let x = symbol!(x);
        let y = symbol!(y);
        // y + x (reversed order)
        let expr = Expression::add(vec![Expression::symbol(y.clone()), Expression::symbol(x.clone())]);

        // Pattern: a + b (should match despite different order)
        let pattern = Pattern::Add(vec![Pattern::wildcard("a"), Pattern::wildcard("b")]);

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());

        if let Some(bindings) = matches {
            // Should bind either way (commutative)
            let a_val = bindings.get("a").unwrap();
            let b_val = bindings.get("b").unwrap();

            // Either (a=y, b=x) or (a=x, b=y)
            assert!(
                (a_val == &Expression::symbol(y.clone()) && b_val == &Expression::symbol(x.clone()))
                    || (a_val == &Expression::symbol(x.clone()) && b_val == &Expression::symbol(y.clone()))
            );
        }
    }

    #[test]
    fn test_commutative_multiplication_matching() {
        let x = symbol!(x);
        // x * 3 (reversed from pattern 3 * x)
        let expr = Expression::mul(vec![Expression::symbol(x.clone()), Expression::integer(3)]);

        // Pattern: a * b
        let pattern = Pattern::Mul(vec![Pattern::wildcard("a"), Pattern::wildcard("b")]);

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());
    }

    #[test]
    fn test_wildcard_with_exclude() {
        let x = symbol!(x);
        let y = symbol!(y);

        // Pattern: wildcard 'a' that excludes x
        let pattern = Pattern::wildcard_excluding("a", vec![Expression::symbol(x.clone())]);

        // Should not match x
        assert!(Expression::symbol(x.clone()).matches(&pattern).is_none());

        // Should match y
        assert!(Expression::symbol(y.clone()).matches(&pattern).is_some());

        // Should not match expressions containing x
        let expr_with_x = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
        assert!(expr_with_x.matches(&pattern).is_none());
    }

    #[test]
    fn test_wildcard_with_property() {
        // Property: only match integers
        fn is_integer(expr: &Expression) -> bool {
            matches!(expr, Expression::Number(_))
        }

        let pattern = Pattern::wildcard_with_properties("n", vec![is_integer]);

        // Should match integer
        assert!(Expression::integer(42).matches(&pattern).is_some());

        // Should not match symbol
        let x = symbol!(x);
        assert!(Expression::symbol(x.clone()).matches(&pattern).is_none());
    }

    #[test]
    fn test_three_term_commutative_match() {
        let x = symbol!(x);
        let y = symbol!(y);
        let z = symbol!(z);

        // z + y + x (different order)
        let expr = Expression::add(vec![
            Expression::symbol(z.clone()),
            Expression::symbol(y.clone()),
            Expression::symbol(x.clone()),
        ]);

        // Pattern: a + b + c
        let pattern = Pattern::Add(vec![
            Pattern::wildcard("a"),
            Pattern::wildcard("b"),
            Pattern::wildcard("c"),
        ]);

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());
    }
}
