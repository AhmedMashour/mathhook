//! Commutative matching algorithms
//!
//! Handles pattern matching for commutative operations (Add, Mul) with
//! permutation-based and greedy heuristic matching strategies.

use crate::core::Expression;
use crate::pattern::matching::patterns::Pattern;
use super::PatternMatches;
use super::core::match_recursive;

/// Match commutative operations (Add, Mul)
///
/// Tries to match expression terms/factors against pattern terms/factors
/// considering all possible orderings. This is essential for algebraic
/// pattern matching since addition and multiplication are commutative.
pub(super) fn match_commutative(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::matching::engine::Matchable;
    use crate::pattern::matching::patterns::Pattern;
    use crate::prelude::*;

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
