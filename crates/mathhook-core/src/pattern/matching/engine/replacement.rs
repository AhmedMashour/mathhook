//! Pattern replacement logic
//!
//! Applies replacement patterns with wildcard bindings from matches.

use super::PatternMatches;
use crate::core::Expression;
use crate::pattern::matching::patterns::Pattern;

/// Apply a replacement pattern with bindings from a match
pub(super) fn apply_replacement(replacement: &Pattern, bindings: &PatternMatches) -> Expression {
    match replacement {
        Pattern::Wildcard { name, .. } => bindings
            .get(name)
            .cloned()
            .unwrap_or_else(|| panic!("Unbound wildcard in replacement: {}", name)),

        Pattern::Exact(expr) => expr.clone(),

        Pattern::Add(terms) => {
            let new_terms: Vec<Expression> = terms
                .iter()
                .map(|t| apply_replacement(t, bindings))
                .collect();
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
            let new_args: Vec<Expression> = args
                .iter()
                .map(|a| apply_replacement(a, bindings))
                .collect();
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
    use crate::pattern::matching::engine::Matchable;
    use crate::pattern::matching::patterns::Pattern;
    use crate::prelude::*;

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
}
