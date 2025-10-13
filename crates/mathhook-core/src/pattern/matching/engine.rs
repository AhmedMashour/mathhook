//! Pattern matching engine
//!
//! Implements the matching algorithms including commutative matching,
//! wildcard binding, and pattern replacement logic.

use crate::core::Expression;
use crate::pattern::matching::patterns::Pattern;
use std::collections::HashMap;

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
    ///         Pattern::wildcard("a"),
    ///         Pattern::Exact(Expression::symbol(x.clone()))
    ///     ]),
    ///     Pattern::wildcard("b")
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
    ///             args: vec![Pattern::wildcard("a")]
    ///         }),
    ///         Box::new(Pattern::Exact(Expression::integer(2)))
    ///     ),
    ///     Pattern::Pow(
    ///         Box::new(Pattern::Function {
    ///             name: "cos".to_string(),
    ///             args: vec![Pattern::wildcard("a")]
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
        if let Some(bindings) = self.matches(pattern) {
            apply_replacement(replacement, &bindings)
        } else {
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
        Pattern::Wildcard { name, constraints } => {
            if let Some(constraints) = constraints {
                if !constraints.is_satisfied_by(expr) {
                    return false;
                }
            }

            if let Some(existing) = bindings.get(name) {
                expr == existing
            } else {
                bindings.insert(name.clone(), expr.clone());
                true
            }
        }

        Pattern::Exact(pattern_expr) => expr == pattern_expr,

        Pattern::Add(pattern_terms) => {
            if let Expression::Add(expr_terms) = expr {
                match_commutative(expr_terms, pattern_terms, bindings)
            } else {
                false
            }
        }

        Pattern::Mul(pattern_factors) => {
            if let Expression::Mul(expr_factors) = expr {
                match_commutative(expr_factors, pattern_factors, bindings)
            } else {
                false
            }
        }

        Pattern::Pow(pattern_base, pattern_exp) => {
            if let Expression::Pow(expr_base, expr_exp) = expr {
                match_recursive(expr_base, pattern_base, bindings)
                    && match_recursive(expr_exp, pattern_exp, bindings)
            } else {
                false
            }
        }

        Pattern::Function { name, args } => {
            if let Expression::Function {
                name: expr_name,
                args: expr_args,
            } = expr
            {
                if expr_name != name {
                    return false;
                }

                if expr_args.len() != args.len() {
                    return false;
                }

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
    if pattern_items.is_empty() {
        return expr_items.is_empty();
    }

    if pattern_items.len() == 1 {
        if expr_items.len() == 1 {
            return match_recursive(&expr_items[0], &pattern_items[0], bindings);
        } else {
            return false;
        }
    }

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

        *bindings = backup_bindings;
    }

    if pattern_items.len() <= 6 {
        try_permutation_match(expr_items, pattern_items, bindings)
    } else {
        try_greedy_match(expr_items, pattern_items, bindings)
    }
}

/// Try all permutations of pattern items to find a match
fn try_permutation_match(
    expr_items: &[Expression],
    pattern_items: &[Pattern],
    bindings: &mut PatternMatches,
) -> bool {
    if expr_items.len() != pattern_items.len() {
        return false;
    }

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
        let backup_bindings = bindings.clone();
        for (expr_idx, &pattern_idx) in indices.iter().enumerate() {
            if !match_recursive(&expr_items[expr_idx], &pattern_items[pattern_idx], bindings) {
                *bindings = backup_bindings;
                return false;
            }
        }
        return true;
    }

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

    for pattern_item in pattern_items {
        let mut matched = false;
        for (expr_idx, expr_item) in expr_items.iter().enumerate() {
            if !used_expr[expr_idx] {
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
        Pattern::Wildcard { name, .. } => {
            bindings
                .get(name)
                .cloned()
                .unwrap_or_else(|| panic!("Unbound wildcard in replacement: {}", name))
        }

        Pattern::Exact(expr) => expr.clone(),

        Pattern::Add(terms) => {
            let new_terms: Vec<Expression> =
                terms.iter().map(|t| apply_replacement(t, bindings)).collect();
            Expression::Add(Box::new(new_terms))
        }

        Pattern::Mul(factors) => {
            let new_factors: Vec<Expression> = factors
                .iter()
                .map(|f| apply_replacement(f, bindings))
                .collect();
            Expression::Mul(Box::new(new_factors))
        }

        Pattern::Pow(base, exp) => {
            let new_base = apply_replacement(base, bindings);
            let new_exp = apply_replacement(exp, bindings);
            Expression::Pow(Box::new(new_base), Box::new(new_exp))
        }

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
    use crate::pattern::matching::patterns::Pattern;
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
        let expr = Expression::Add(Box::new(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(x.clone()),
        ]));

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
        let expr = Expression::add(vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())]);

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
        let expr = Expression::Add(Box::new(vec![
            Expression::symbol(x.clone()),
            Expression::integer(1),
        ]));

        let pattern = Pattern::Add(vec![Pattern::wildcard("a"), Pattern::wildcard("b")]);

        let replacement = Pattern::Add(vec![Pattern::wildcard("b"), Pattern::wildcard("a")]);

        let result = expr.replace(&pattern, &replacement);

        let expected = Expression::Add(Box::new(vec![
            Expression::integer(1),
            Expression::symbol(x.clone()),
        ]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_trig_identity_replacement() {
        let x = symbol!(x);
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

        let replacement = Pattern::Exact(Expression::integer(1));

        let result = expr.replace(&pattern, &replacement);
        assert_eq!(result, Expression::integer(1));
    }

    #[test]
    fn test_commutative_addition_matching() {
        let x = symbol!(x);
        let y = symbol!(y);
        let expr = Expression::add(vec![Expression::symbol(y.clone()), Expression::symbol(x.clone())]);

        let pattern = Pattern::Add(vec![Pattern::wildcard("a"), Pattern::wildcard("b")]);

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());

        if let Some(bindings) = matches {
            let a_val = bindings.get("a").unwrap();
            let b_val = bindings.get("b").unwrap();

            assert!(
                (a_val == &Expression::symbol(y.clone()) && b_val == &Expression::symbol(x.clone()))
                    || (a_val == &Expression::symbol(x.clone()) && b_val == &Expression::symbol(y.clone()))
            );
        }
    }

    #[test]
    fn test_commutative_multiplication_matching() {
        let x = symbol!(x);
        let expr = Expression::mul(vec![Expression::symbol(x.clone()), Expression::integer(3)]);

        let pattern = Pattern::Mul(vec![Pattern::wildcard("a"), Pattern::wildcard("b")]);

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());
    }

    #[test]
    fn test_wildcard_with_exclude() {
        let x = symbol!(x);
        let y = symbol!(y);

        let pattern = Pattern::wildcard_excluding("a", vec![Expression::symbol(x.clone())]);

        assert!(Expression::symbol(x.clone()).matches(&pattern).is_none());

        assert!(Expression::symbol(y.clone()).matches(&pattern).is_some());

        let expr_with_x = Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]);
        assert!(expr_with_x.matches(&pattern).is_none());
    }

    #[test]
    fn test_wildcard_with_property() {
        fn is_integer(expr: &Expression) -> bool {
            matches!(expr, Expression::Number(_))
        }

        let pattern = Pattern::wildcard_with_properties("n", vec![is_integer]);

        assert!(Expression::integer(42).matches(&pattern).is_some());

        let x = symbol!(x);
        assert!(Expression::symbol(x.clone()).matches(&pattern).is_none());
    }

    #[test]
    fn test_three_term_commutative_match() {
        let x = symbol!(x);
        let y = symbol!(y);
        let z = symbol!(z);

        let expr = Expression::add(vec![
            Expression::symbol(z.clone()),
            Expression::symbol(y.clone()),
            Expression::symbol(x.clone()),
        ]);

        let pattern = Pattern::Add(vec![
            Pattern::wildcard("a"),
            Pattern::wildcard("b"),
            Pattern::wildcard("c"),
        ]);

        let matches = expr.matches(&pattern);
        assert!(matches.is_some());
    }
}
