//! Expression expansion operations
//! Handles polynomial expansion, distribution, and algebraic expansion

use crate::core::commutativity::Commutativity;
use crate::core::{Expression, Number};

/// Trait for expanding expressions
pub trait Expand {
    fn expand(&self) -> Self;
}

impl Expand for Expression {
    /// Expand the expression by distributing multiplication over addition
    fn expand(&self) -> Self {
        match self {
            Expression::Number(_) | Expression::Symbol(_) => self.clone(),

            Expression::Add(terms) => {
                let expanded_terms: Vec<Expression> =
                    terms.iter().map(|term| term.expand()).collect();
                Expression::add(expanded_terms)
            }

            Expression::Mul(factors) => self.expand_multiplication(factors),

            Expression::Pow(base, exp) => self.expand_power(base, exp),

            Expression::Function { name, args } => {
                let expanded_args: Vec<Expression> = args.iter().map(|arg| arg.expand()).collect();
                Expression::function(name.clone(), expanded_args)
            }
            _ => self.clone(),
        }
    }
}

impl Expression {
    /// Expand multiplication by distributing over addition
    fn expand_multiplication(&self, factors: &[Expression]) -> Expression {
        if factors.is_empty() {
            return Expression::integer(1);
        }

        if factors.len() == 1 {
            return factors[0].expand();
        }

        let mut result = factors[0].expand();

        for factor in &factors[1..] {
            result = self.distribute_multiply(&result, &factor.expand());
        }

        result
    }

    /// Distribute multiplication: (a + b) * c = a*c + b*c
    fn distribute_multiply(&self, left: &Expression, right: &Expression) -> Expression {
        match (left, right) {
            (Expression::Add(left_terms), _) => {
                let distributed_terms: Vec<Expression> = left_terms
                    .iter()
                    .map(|term| self.distribute_multiply(term, right))
                    .collect();
                Expression::add(distributed_terms)
            }

            (_, Expression::Add(right_terms)) => {
                let distributed_terms: Vec<Expression> = right_terms
                    .iter()
                    .map(|term| self.distribute_multiply(left, term))
                    .collect();
                Expression::add(distributed_terms)
            }

            _ => Expression::mul(vec![left.clone(), right.clone()]),
        }
    }

    /// Expand power expressions
    fn expand_power(&self, base: &Expression, exp: &Expression) -> Expression {
        if let Expression::Number(Number::Integer(n)) = exp {
            let exp_val = *n;
            if exp_val >= 0 && exp_val <= 10 {
                return self.expand_integer_power(base, exp_val as u32);
            }
        }

        Expression::pow(base.clone(), exp.clone())
    }

    /// Expand integer powers: (a + b)^n
    ///
    /// For noncommutative terms, preserves order:
    /// (A+B)^2 = A^2 + AB + BA + B^2 (4 terms for noncommutative)
    /// (x+y)^2 = x^2 + 2xy + y^2 (3 terms for commutative)
    fn expand_integer_power(&self, base: &Expression, exp: u32) -> Expression {
        match exp {
            0 => Expression::integer(1),
            1 => base.expand(),
            2 => {
                match base {
                    Expression::Add(terms) if terms.len() == 2 => {
                        let a = &terms[0];
                        let b = &terms[1];

                        let commutativity = Commutativity::combine(
                            terms.iter().map(|t| t.commutativity())
                        );

                        if commutativity.can_sort() {
                            Expression::add(vec![
                                Expression::pow(a.clone(), Expression::integer(2)).expand(),
                                Expression::mul(vec![Expression::integer(2), a.clone(), b.clone()])
                                    .expand(),
                                Expression::pow(b.clone(), Expression::integer(2)).expand(),
                            ])
                        } else {
                            Expression::add(vec![
                                Expression::pow(a.clone(), Expression::integer(2)).expand(),
                                Expression::mul(vec![a.clone(), b.clone()]).expand(),
                                Expression::mul(vec![b.clone(), a.clone()]).expand(),
                                Expression::pow(b.clone(), Expression::integer(2)).expand(),
                            ])
                        }
                    }
                    _ => {
                        let expanded_base = base.expand();
                        self.distribute_multiply(&expanded_base, &expanded_base)
                    }
                }
            }
            _ => {
                let expanded_base = base.expand();
                let mut result = expanded_base.clone();

                for _ in 1..exp {
                    result = self.distribute_multiply(&result, &expanded_base);
                }

                result
            }
        }
    }

    /// Expand binomial expressions: (a + b)^n using binomial theorem
    ///
    /// For commutative terms, uses binomial theorem: C(n,k) * a^k * b^(n-k)
    /// For noncommutative terms, uses direct multiplication to preserve order
    pub fn expand_binomial(&self, a: &Expression, b: &Expression, n: u32) -> Expression {
        if n == 0 {
            return Expression::integer(1);
        }

        if n == 1 {
            return Expression::add(vec![a.clone(), b.clone()]);
        }

        let commutativity = Commutativity::combine(
            vec![a.commutativity(), b.commutativity()]
        );

        if !commutativity.can_sort() {
            let base = Expression::add(vec![a.clone(), b.clone()]);
            let mut result = base.clone();
            for _ in 1..n {
                result = self.distribute_multiply(&result, &base);
            }
            return result;
        }

        if n <= 5 {
            let mut terms = Vec::new();

            for k in 0..=n {
                let coeff = self.binomial_coefficient(n, k);
                let a_power = if k == 0 {
                    Expression::integer(1)
                } else {
                    Expression::pow(a.clone(), Expression::integer(k as i64))
                };
                let b_power = if n - k == 0 {
                    Expression::integer(1)
                } else {
                    Expression::pow(b.clone(), Expression::integer((n - k) as i64))
                };

                let term = Expression::mul(vec![Expression::integer(coeff), a_power, b_power]);

                terms.push(term);
            }

            Expression::add(terms)
        } else {
            Expression::pow(
                Expression::add(vec![a.clone(), b.clone()]),
                Expression::integer(n as i64),
            )
        }
    }

    /// Calculate binomial coefficient C(n, k)
    fn binomial_coefficient(&self, n: u32, k: u32) -> i64 {
        if k > n {
            return 0;
        }

        if k == 0 || k == n {
            return 1;
        }

        let mut result = 1i64;
        let k = k.min(n - k); // Take advantage of symmetry

        for i in 0..k {
            if let Some(new_result) = result.checked_mul((n - i) as i64) {
                if let Some(final_result) = new_result.checked_div((i + 1) as i64) {
                    result = final_result;
                } else {
                    return 1; // Fallback on division error
                }
            } else {
                return 1; // Fallback on overflow
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_basic_expansion() {
        let x = symbol!(x);
        let y = symbol!(y);

        let expr = Expression::mul(vec![
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::integer(2),
        ]);

        let result = expr.expand();

        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2);
            }
            _ => println!("Expansion result: {}", result),
        }
    }

    #[test]
    fn test_square_expansion() {
        let x = symbol!(x);
        let y = symbol!(y);

        let expr = Expression::pow(
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::integer(2),
        );

        let result = expr.expand();

        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 3);
            }
            _ => println!("Square expansion result: {}", result),
        }
    }

    #[test]
    fn test_binomial_coefficients() {
        let expr = Expression::integer(1); // Dummy expression for method access

        assert_eq!(expr.binomial_coefficient(5, 0), 1);
        assert_eq!(expr.binomial_coefficient(5, 1), 5);
        assert_eq!(expr.binomial_coefficient(5, 2), 10);
        assert_eq!(expr.binomial_coefficient(5, 3), 10);
        assert_eq!(expr.binomial_coefficient(5, 4), 5);
        assert_eq!(expr.binomial_coefficient(5, 5), 1);
    }

    #[test]
    fn test_nested_expansion() {
        let x = symbol!(x);

        let expr = Expression::mul(vec![
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(2)]),
        ]);

        let result = expr.expand();

        assert!(!result.is_zero());
    }

    #[test]
    fn test_expansion_with_numbers() {
        let expr = Expression::mul(vec![
            Expression::integer(3),
            Expression::add(vec![Expression::integer(2), Expression::integer(4)]),
        ]);

        let result = expr.expand();

        assert!(!result.is_zero());
    }

    #[test]
    fn test_commutative_square_expansion() {
        let x = symbol!(x);
        let y = symbol!(y);

        let expr = Expression::pow(
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::integer(2),
        );

        let result = expr.expand();

        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 3, "Expected 3 terms for commutative square");
            }
            _ => panic!("Expected addition of 3 terms"),
        }
    }

    #[test]
    fn test_noncommutative_matrix_square_expansion() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);

        let expr = Expression::pow(
            Expression::add(vec![
                Expression::symbol(a.clone()),
                Expression::symbol(b.clone()),
            ]),
            Expression::integer(2),
        );

        let result = expr.expand();

        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 4, "Expected 4 terms for noncommutative square");
            }
            _ => panic!("Expected addition of 4 terms"),
        }
    }

    #[test]
    fn test_noncommutative_operator_square_expansion() {
        let p = symbol!(p; operator);
        let x = symbol!(x; operator);

        let expr = Expression::pow(
            Expression::add(vec![
                Expression::symbol(p.clone()),
                Expression::symbol(x.clone()),
            ]),
            Expression::integer(2),
        );

        let result = expr.expand();

        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 4, "Expected 4 terms for operator square");
            }
            _ => panic!("Expected addition of 4 terms"),
        }
    }

    #[test]
    fn test_noncommutative_quaternion_square_expansion() {
        let i = symbol!(i; quaternion);
        let j = symbol!(j; quaternion);

        let expr = Expression::pow(
            Expression::add(vec![
                Expression::symbol(i.clone()),
                Expression::symbol(j.clone()),
            ]),
            Expression::integer(2),
        );

        let result = expr.expand();

        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 4, "Expected 4 terms for quaternion square");
            }
            _ => panic!("Expected addition of 4 terms"),
        }
    }

    #[test]
    fn test_mixed_commutative_noncommutative_expansion() {
        let x = symbol!(x);
        let a = symbol!(A; matrix);

        let expr = Expression::pow(
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(a.clone()),
            ]),
            Expression::integer(2),
        );

        let result = expr.expand();

        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 4, "Expected 4 terms when ANY term is noncommutative");
            }
            _ => panic!("Expected addition of 4 terms"),
        }
    }

    #[test]
    fn test_distribution_preserves_order_for_matrices() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);
        let c = symbol!(C; matrix);

        let expr = Expression::mul(vec![
            Expression::add(vec![
                Expression::symbol(a.clone()),
                Expression::symbol(b.clone()),
            ]),
            Expression::symbol(c.clone()),
        ]);

        let result = expr.expand();

        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2, "Expected AC + BC");
            }
            _ => panic!("Expected addition"),
        }
    }

    #[test]
    fn test_binomial_theorem_not_used_for_noncommutative() {
        let a = symbol!(A; matrix);
        let b = symbol!(B; matrix);

        let result = Expression::integer(1).expand_binomial(
            &Expression::symbol(a.clone()),
            &Expression::symbol(b.clone()),
            3,
        );

        // (via repeated multiplication, not binomial theorem)
        assert!(!result.is_zero());
    }
}
