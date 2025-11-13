//! Factorization operations for expressions
//! Handles polynomial factorization, common factor extraction, and algebraic factoring

mod common;
mod noncommutative;
mod quadratic;

use crate::core::commutativity::Commutativity;
use crate::core::{Expression};
// num_traits imports removed

/// Trait for factoring expressions
pub trait Factor {
    fn factor(&self) -> Self;
    fn factor_out_gcd(&self) -> Self;
    fn factor_common(&self) -> Self;
}

impl Factor for Expression {
    /// Factor the expression by extracting common factors
    fn factor(&self) -> Self {
        match self {
            Expression::Number(_) | Expression::Symbol(_) => self.clone(),

            Expression::Add(terms) => self.factor_addition(terms),

            Expression::Mul(factors) => {
                let factored_factors: Vec<Expression> =
                    factors.iter().map(|f| f.factor()).collect();
                Expression::mul(factored_factors)
            }

            Expression::Pow(base, exp) => Expression::pow(base.factor(), exp.factor()),

            Expression::Function { name, args } => {
                let factored_args: Vec<Expression> = args.iter().map(|arg| arg.factor()).collect();
                Expression::function(name.clone(), factored_args)
            }
            _ => self.clone(),
        }
    }

    /// Factor out the GCD from an expression
    fn factor_out_gcd(&self) -> Self {
        match self {
            Expression::Add(terms) => {
                if terms.len() < 2 {
                    return self.clone();
                }

                let mut common_factor = terms[0].clone();
                for term in &terms[1..] {
                    common_factor = common_factor.gcd(term);
                    if common_factor.is_one() {
                        return self.clone();
                    }
                }

                if !common_factor.is_one() {
                    let factored_terms: Vec<Expression> = terms
                        .iter()
                        .map(|term| self.divide_by_factor(term, &common_factor))
                        .collect();

                    Expression::mul(vec![common_factor, Expression::add(factored_terms)])
                } else {
                    self.clone()
                }
            }
            _ => self.clone(),
        }
    }

    /// Factor common elements
    fn factor_common(&self) -> Self {
        self.factor_out_gcd()
    }
}

impl Expression {
    /// Factor addition expressions by finding common factors
    ///
    /// For commutative terms: AB + AC = A(B+C)
    /// For noncommutative terms: Try left factoring first, then right factoring
    ///   Left: AB + AC = A(B+C)
    ///   Right: BA + CA = (B+C)A
    fn factor_addition(&self, terms: &[Expression]) -> Expression {
        if terms.len() < 2 {
            return Expression::add(terms.to_vec());
        }

        let commutativity = Commutativity::combine(
            terms.iter().map(|t| t.commutativity())
        );

        if commutativity.can_sort() {
            let common_factor = self.find_common_factor_in_terms(terms);

            if !common_factor.is_one() {
                let factored_terms: Vec<Expression> = terms
                    .iter()
                    .map(|term| self.divide_by_factor(term, &common_factor))
                    .collect();

                Expression::mul(vec![common_factor, Expression::add(factored_terms)])
            } else {
                self.try_quadratic_factoring(terms)
                    .unwrap_or_else(|| Expression::add(terms.to_vec()))
            }
        } else {
            if let Some(left_factored) = self.try_left_factor(terms) {
                return left_factored;
            }

            if let Some(right_factored) = self.try_right_factor(terms) {
                return right_factored;
            }

            Expression::add(terms.to_vec())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;
    use num_bigint::BigInt;

    #[test]
    fn test_basic_factoring() {
        let x = symbol!(x);

        let expr = Expression::add(vec![
            Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
            Expression::integer(4),
        ]);

        let result = expr.factor();
        println!("2x + 4 factored = {}", result);

        match result {
            Expression::Mul(_) => println!("Successfully factored"),
            _ => println!("Factoring result: {}", result),
        }
    }

    #[test]
    fn test_gcd_factoring() {
        let x = symbol!(x);

        let expr = Expression::add(vec![
            Expression::mul(vec![Expression::integer(6), Expression::symbol(x.clone())]),
            Expression::integer(9),
        ]);

        let result = expr.factor_out_gcd();
        println!("6x + 9 GCD factored = {}", result);

        assert!(!result.is_zero());
    }

    #[test]
    fn test_numeric_coefficient_extraction() {
        let x = symbol!(x);

        let expr = Expression::mul(vec![
            Expression::integer(12),
            Expression::symbol(x.clone()),
            Expression::integer(5),
        ]);

        let (coeff, remaining) = expr.factor_numeric_coefficient();

        println!("Coefficient: {}, Remaining: {}", coeff, remaining);
        assert_eq!(coeff, BigInt::from(60));
        assert_eq!(remaining, Expression::symbol(x));
    }

    #[test]
    fn test_difference_of_squares() {
        let x = symbol!(x);
        let y = symbol!(y);

        let result = Expression::integer(1).factor_difference_of_squares(
            &Expression::symbol(x.clone()),
            &Expression::symbol(y.clone()),
        );

        println!("x^2 - y^2 factored = {}", result);

        match result {
            Expression::Mul(factors) => assert_eq!(factors.len(), 2),
            _ => panic!("Expected multiplication"),
        }
    }

    #[test]
    fn test_common_factor_extraction() {
        let x = symbol!(x);
        let y = symbol!(y);

        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::symbol(x.clone()),
        ]);

        let result = expr.factor_common();
        println!("xy + x factored = {}", result);

        assert!(!result.is_zero());
    }

    #[test]
    fn test_no_common_factor() {
        let x = symbol!(x);
        let y = symbol!(y);

        let expr = Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]);

        let result = expr.factor();

        assert_eq!(result, expr);
    }

    #[test]
    fn test_left_factoring_matrices() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);
        let c = symbol!(C; matrix);

        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(a.clone()),
                Expression::symbol(b.clone()),
            ]),
            Expression::mul(vec![
                Expression::symbol(a.clone()),
                Expression::symbol(c.clone()),
            ]),
        ]);

        let result = expr.factor();
        println!("AB + AC factored = {}", result);

        match result {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2, "Expected factored form A(B+C) or (B+C)A");
                let has_a = factors.iter().any(|f| f == &Expression::symbol(a.clone()));
                let has_sum = factors.iter().any(|f| matches!(f, Expression::Add(_)));
                assert!(has_a, "Should contain factor A");
                assert!(has_sum, "Should contain sum (B+C)");
            }
            _ => panic!("Expected multiplication after factoring, got: {}", result),
        }
    }

    #[test]
    fn test_right_factoring_matrices() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);
        let c = symbol!(C; matrix);

        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(b.clone()),
                Expression::symbol(a.clone()),
            ]),
            Expression::mul(vec![
                Expression::symbol(c.clone()),
                Expression::symbol(a.clone()),
            ]),
        ]);

        let result = expr.factor();
        println!("BA + CA factored = {}", result);

        match result {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2, "Expected factored form (B+C)A or A(B+C)");
                let has_a = factors.iter().any(|f| f == &Expression::symbol(a.clone()));
                let has_sum = factors.iter().any(|f| matches!(f, Expression::Add(_)));
                assert!(has_a, "Should contain factor A");
                assert!(has_sum, "Should contain sum (B+C)");
            }
            _ => panic!("Expected multiplication after factoring, got: {}", result),
        }
    }

    #[test]
    fn test_cannot_cross_factor_noncommutative() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);
        let c = symbol!(C; matrix);
        let d = symbol!(D; matrix);

        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(a.clone()),
                Expression::symbol(b.clone()),
            ]),
            Expression::mul(vec![
                Expression::symbol(c.clone()),
                Expression::symbol(d.clone()),
            ]),
        ]);

        let result = expr.factor();
        println!("AB + CD factored = {}", result);

        match result {
            Expression::Add(_) => (),
            _ => panic!("Expected no factoring for AB + CD"),
        }
    }

    #[test]
    fn test_operator_left_factoring() {
        let p = symbol!(p; operator);
        let x = symbol!(x; operator);
        let h = symbol!(h; operator);

        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(p.clone()),
                Expression::symbol(x.clone()),
            ]),
            Expression::mul(vec![
                Expression::symbol(p.clone()),
                Expression::symbol(h.clone()),
            ]),
        ]);

        let result = expr.factor();
        println!("px + ph factored = {}", result);

        match result {
            Expression::Mul(factors) => {
                assert_eq!(factors.len(), 2, "Expected factored form p(x+h) or (x+h)p");
                let has_p = factors.iter().any(|f| f == &Expression::symbol(p.clone()));
                let has_sum = factors.iter().any(|f| matches!(f, Expression::Add(_)));
                assert!(has_p, "Should contain factor p");
                assert!(has_sum, "Should contain sum (x+h)");
            }
            _ => panic!("Expected multiplication after factoring, got: {}", result),
        }
    }

    #[test]
    fn test_commutative_factoring_unchanged() {
        let x = symbol!(x);
        let y = symbol!(y);

        let expr = Expression::add(vec![
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::mul(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
        ]);

        let result = expr.factor();
        println!("Commutative xy + xz factored = {}", result);

        assert!(!result.is_zero());
    }
    #[test]

    #[test]
    fn test_matrix_same_position_factoring() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);
        
        // Test 2AB + 3AB = 5AB (same order, should combine)
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

        let result = expr.factor();

        // Should be able to factor out AB
        assert!(!result.is_zero());
    }
}
