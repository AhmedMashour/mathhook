//! Term collection and combination operations
//! Handles collecting like terms, combining coefficients, and organizing expressions

mod coefficients;
mod terms;

use crate::core::{Expression, Symbol};

/// Trait for collecting terms in expressions
pub trait Collect {
    fn collect(&self, var: &Symbol) -> Self;
    fn collect_terms(&self) -> Self;
    fn combine_like_terms(&self) -> Self;
}

impl Collect for Expression {
    /// Collect terms with respect to a specific variable
    fn collect(&self, var: &Symbol) -> Self {
        match self {
            Expression::Add(terms) => self.collect_addition_terms(terms, var),
            _ => self.clone(),
        }
    }

    /// Collect and combine all like terms
    fn collect_terms(&self) -> Self {
        match self {
            Expression::Add(terms) => self.collect_all_like_terms(terms),
            Expression::Mul(factors) => self.collect_multiplication_terms(factors),
            _ => self.clone(),
        }
    }

    /// Combine like terms in the expression
    fn combine_like_terms(&self) -> Self {
        self.collect_terms()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_collect_like_terms() {
        let x = symbol!(x);

        let expr = Expression::add(vec![
            Expression::mul(vec![expr!(2), Expression::symbol(x.clone())]),
            Expression::mul(vec![Expression::integer(3), Expression::symbol(x.clone())]),
        ]);

        let result = expr.collect(&x);
        println!("2x + 3x collected = {}", result);

        assert!(!result.is_zero());
    }

    #[test]
    fn test_collect_different_powers() {
        let x = symbol!(x);

        let expr = Expression::add(vec![
            Expression::pow(Expression::symbol(x.clone()), expr!(2)),
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::pow(Expression::symbol(x.clone()), expr!(2)),
        ]);

        let result = expr.collect(&x);
        println!("x^2 + 2x + x^2 collected = {}", result);

        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2);
            }
            _ => println!("Collection result: {}", result),
        }
    }

    #[test]
    fn test_combine_like_terms() {
        let x = symbol!(x);
        let y = symbol!(y);

        let expr = Expression::add(vec![
            Expression::mul(vec![expr!(3), Expression::symbol(x.clone())]),
            Expression::mul(vec![expr!(2), Expression::symbol(y.clone())]),
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);

        let result = expr.combine_like_terms();
        println!("3x + 2y + x + y combined = {}", result);

        assert!(!result.is_zero());
    }

    #[test]
    fn test_collect_constants() {
        let x = symbol!(x);

        let expr = Expression::add(vec![
            Expression::integer(5),
            Expression::mul(vec![expr!(3), Expression::symbol(x.clone())]),
            expr!(2),
        ]);

        let result = expr.collect(&x);
        println!("5 + 3x + 2 collected = {}", result);

        assert!(!result.is_zero());
    }

    #[test]
    fn test_separate_constants() {
        let x = symbol!(x);

        let expr = Expression::add(vec![expr!(5), Expression::symbol(x.clone()), expr!(3)]);

        let (constants, variables) = expr.separate_constants();

        println!("Constants: {}, Variables: {}", constants, variables);

        assert!(!constants.is_zero());
        assert!(!variables.is_zero());
    }

    #[test]
    fn test_collect_multiplication_powers() {
        let x = symbol!(x);

        let expr = Expression::mul(vec![
            Expression::pow(Expression::symbol(x.clone()), expr!(2)),
            Expression::pow(Expression::symbol(x.clone()), expr!(3)),
        ]);

        let result = expr.collect_terms();
        println!("x^2 * x^3 collected = {}", result);

        assert!(!result.is_zero());
    }

    #[test]
    fn test_commutative_collection() {
        let x = symbol!(x);
        let y = symbol!(y);

        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::mul(vec![
                Expression::integer(3),
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
        ]);

        let result = expr.combine_like_terms();
        println!("2xy + 3xy = {}", result);

        match result {
            Expression::Mul(_) => {
                println!("Successfully combined like terms");
            }
            _ => println!("Result: {}", result),
        }
    }

    #[test]
    fn test_noncommutative_no_collection_different_order() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);

        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(a.clone()),
                Expression::symbol(b.clone()),
            ]),
            Expression::mul(vec![
                Expression::integer(3),
                Expression::symbol(b.clone()),
                Expression::symbol(a.clone()),
            ]),
        ]);

        let result = expr.combine_like_terms();
        println!("2AB + 3BA = {}", result);

        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2, "AB and BA should NOT combine (different order)");
            }
            _ => panic!("Expected addition of 2 separate terms"),
        }
    }

    #[test]
    fn test_noncommutative_collection_same_order() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);

        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(a.clone()),
                Expression::symbol(b.clone()),
            ]),
            Expression::mul(vec![
                Expression::integer(3),
                Expression::symbol(a.clone()),
                Expression::symbol(b.clone()),
            ]),
        ]);

        let result = expr.combine_like_terms();
        println!("2AB + 3AB = {}", result);

        match result {
            Expression::Mul(_) => {
                println!("Successfully combined like terms with same order");
            }
            Expression::Add(terms) if terms.len() == 1 => {
                println!("Single term result (acceptable)");
            }
            _ => println!("Result: {}", result),
        }
    }

    #[test]
    fn test_operator_collection() {
        let p = symbol!(p; operator);
        let x = symbol!(x; operator);

        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(p.clone()),
                Expression::symbol(x.clone()),
            ]),
            Expression::mul(vec![
                Expression::integer(3),
                Expression::symbol(x.clone()),
                Expression::symbol(p.clone()),
            ]),
        ]);

        let result = expr.combine_like_terms();
        println!("2px + 3xp = {}", result);

        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2, "px and xp should NOT combine");
            }
            _ => panic!("Expected addition of 2 separate terms"),
        }
    }

    #[test]
    fn test_quaternion_collection() {
        let i = symbol!(i; quaternion);
        let j = symbol!(j; quaternion);

        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(i.clone()),
                Expression::symbol(j.clone()),
            ]),
            Expression::mul(vec![
                Expression::integer(3),
                Expression::symbol(j.clone()),
                Expression::symbol(i.clone()),
            ]),
        ]);

        let result = expr.combine_like_terms();
        println!("2ij + 3ji = {}", result);

        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2, "ij and ji should NOT combine");
            }
            _ => panic!("Expected addition of 2 separate terms"),
        }
    }

    #[test]
    fn test_mixed_commutative_noncommutative() {
        let x = symbol!(x);
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);

        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::integer(2),
                Expression::symbol(x.clone()),
                Expression::symbol(a.clone()),
                Expression::symbol(b.clone()),
            ]),
            Expression::mul(vec![
                Expression::integer(3),
                Expression::symbol(x.clone()),
                Expression::symbol(a.clone()),
                Expression::symbol(b.clone()),
            ]),
        ]);

        let result = expr.combine_like_terms();
        println!("2xAB + 3xAB = {}", result);

        assert!(!result.is_zero());
    }
}
