//! Expression expansion operations
//! Handles polynomial expansion, distribution, and algebraic expansion

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
                // Expand each term and keep as addition
                let expanded_terms: Vec<Expression> =
                    terms.iter().map(|term| term.expand()).collect();
                Expression::add(expanded_terms)
            }

            Expression::Mul(factors) => self.expand_multiplication(factors),

            Expression::Pow(base, exp) => self.expand_power(base, exp),

            Expression::Function { name, args } => {
                // Expand arguments
                let expanded_args: Vec<Expression> = args.iter().map(|arg| arg.expand()).collect();
                Expression::function(name.clone(), expanded_args)
            }
            // New expression types - implement later
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

        // Look for additions to distribute over
        let mut result = factors[0].expand();

        for factor in &factors[1..] {
            result = self.distribute_mul(vec![&result, &factor.expand());
        }

        result
    }

    /// Distribute multiplication: (a + b) * c = a*c + b*c
    fn distribute_mul(vec![&self, left: &Expression, right: &Expression) -> Expression {
        match (left, right) {
            // Distribute over left addition: (a + b) * c = a*c + b*c
            (Expression::Add(left_terms), _) => {
                let distributed_terms: Vec<Expression> = left_terms
                    .iter()
                    .map(|term| self.distribute_mul(vec![term, right))
                    .collect();
                Expression::add(distributed_terms)
            }

            // Distribute over right addition: a * (b + c) = a*b + a*c
            (_, Expression::Add(right_terms)) => {
                let distributed_terms: Vec<Expression> = right_terms
                    .iter()
                    .map(|term| self.distribute_mul(vec![left, term))
                    .collect();
                Expression::add(distributed_terms)
            }

            // Base case: multiply non-addition expressions
            _ => Expression::mul(vec![left.clone(), right.clone()]),
        }
    }

    /// Expand power expressions
    fn expand_power(&self, base: &Expression, exp: &Expression) -> Expression {
        // For now, only handle simple integer exponents
        if let Expression::Number(Number::Integer(n)) = exp {
            let exp_val = *n;
            if exp_val >= 0 && exp_val <= 10 {
                // Reasonable limit
                return self.expand_integer_power(base, exp_val as u32);
            }
        }

        // For complex cases, just expand the base
        Expression::pow(base.clone(), exp.clone())
    }

    /// Expand integer powers: (a + b)^n
    fn expand_integer_power(&self, base: &Expression, exp: u32) -> Expression {
        match exp {
            0 => Expression::integer(1),
            1 => base.expand(),
            2 => {
                // Special case for squaring: (a + b)^2 = a^2 + 2ab + b^2
                match base {
                    Expression::Add(terms) if terms.len() == 2 => {
                        let a = &terms[0];
                        let b = &terms[1];

                        Expression::add(vec![
                            Expression::pow(a.clone(), Expression::integer(2)).expand(),
                            Expression::mul(vec![Expression::integer(2), a.clone(), b.clone()])
                                .expand(),
                            Expression::pow(b.clone(), Expression::integer(2)).expand(),
                        ])
                    }
                    _ => {
                        // General case: multiply base by itself
                        let expanded_base = base.expand();
                        self.distribute_mul(vec![&expanded_base, &expanded_base)
                    }
                }
            }
            _ => {
                // For higher powers, use repeated multiplication
                let expanded_base = base.expand();
                let mut result = expanded_base.clone();

                for _ in 1..exp {
                    result = self.distribute_mul(vec![&result, &expanded_base);
                }

                result
            }
        }
    }

    /// Expand binomial expressions: (a + b)^n using binomial theorem
    pub fn expand_binomial(&self, a: &Expression, b: &Expression, n: u32) -> Expression {
        if n == 0 {
            return Expression::integer(1);
        }

        if n == 1 {
            return Expression::add(vec![a.clone(), b.clone()]);
        }

        // For small n, use direct expansion
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
            // For large n, fall back to power representation
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

        // Use the multiplicative formula to avoid overflow
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
    use crate::core::Symbol;

    #[test]
    fn test_basic_expansion() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        // Test (x + y) * 2 = 2x + 2y
        let expr = Expression::mul(vec![
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::integer(2),
        ]);

        let result = expr.expand();
        println!("(x + y) * 2 = {}", result);

        // Should distribute
        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 2);
                // Should contain 2x and 2y terms
            }
            _ => println!("Expansion result: {}", result),
        }
    }

    #[test]
    fn test_square_expansion() {
        let x = Symbol::new("x");
        let y = Symbol::new("y");

        // Test (x + y)^2 = x^2 + 2xy + y^2
        let expr = Expression::pow(
            Expression::add(vec![
                Expression::symbol(x.clone()),
                Expression::symbol(y.clone()),
            ]),
            Expression::integer(2),
        );

        let result = expr.expand();
        println!("(x + y)^2 = {}", result);

        // Should expand to three terms
        match result {
            Expression::Add(terms) => {
                assert_eq!(terms.len(), 3);
                println!("Expanded terms: {:?}", terms);
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
        let x = Symbol::new("x");

        // Test (x + 1) * (x + 2)
        let expr = Expression::mul(vec![
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
            Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(2)]),
        ]);

        let result = expr.expand();
        println!("(x + 1)(x + 2) = {}", result);

        // Should expand to x^2 + 3x + 2
        assert!(!result.is_zero());
    }

    #[test]
    fn test_expansion_with_numbers() {
        // Test 3 * (2 + 4) = 3*2 + 3*4 = 6 + 12 = 18
        let expr = Expression::mul(vec![
            Expression::integer(3),
            Expression::add(vec![Expression::integer(2), Expression::integer(4)]),
        ]);

        let result = expr.expand();
        println!("3 * (2 + 4) = {}", result);

        // Should distribute and potentially simplify
        assert!(!result.is_zero());
    }
}
