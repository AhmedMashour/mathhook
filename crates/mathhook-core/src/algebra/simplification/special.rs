//! Special Function Simplification Strategies
//!
//! Implements algebraic rewrite rules for special functions (gamma, factorial).

use super::strategy::SimplificationStrategy;
use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};

/// Gamma function simplification strategy
pub struct GammaSimplificationStrategy;

impl GammaSimplificationStrategy {
    fn factorial_i64(&self, n: u64) -> BigInt {
        if n <= 1 {
            BigInt::one()
        } else {
            let mut result = BigInt::one();
            for i in 2..=n {
                result *= BigInt::from(i);
            }
            result
        }
    }
}

impl SimplificationStrategy for GammaSimplificationStrategy {
    fn simplify(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            match &args[0] {
                Expression::Number(Number::Integer(n)) => {
                    if let Some(val) = n.to_i64() {
                        if val > 0 && val <= 10 {
                            let factorial_result = self.factorial_i64((val - 1) as u64);
                            Expression::big_integer(factorial_result)
                        } else {
                            Expression::function("gamma", args.to_vec())
                        }
                    } else {
                        Expression::function("gamma", args.to_vec())
                    }
                }
                _ => Expression::function("gamma", args.to_vec()),
            }
        } else {
            Expression::function("gamma", args.to_vec())
        }
    }

    fn applies_to(&self, args: &[Expression]) -> bool {
        args.len() == 1
    }

    fn name(&self) -> &str {
        "GammaSimplificationStrategy"
    }
}

/// Factorial function simplification strategy
///
/// Delegates to the official factorial implementation
pub struct FactorialSimplificationStrategy;

impl SimplificationStrategy for FactorialSimplificationStrategy {
    fn simplify(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            crate::functions::special::factorial(&args[0])
        } else {
            Expression::function("factorial", args.to_vec())
        }
    }

    fn applies_to(&self, args: &[Expression]) -> bool {
        args.len() == 1
    }

    fn name(&self) -> &str {
        "FactorialSimplificationStrategy"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr;

    #[test]
    fn test_gamma_of_one() {
        let strategy = GammaSimplificationStrategy;
        let result = strategy.simplify(&[expr!(1)]);
        assert_eq!(result, expr!(1));
    }

    #[test]
    fn test_gamma_of_two() {
        let strategy = GammaSimplificationStrategy;
        let result = strategy.simplify(&[expr!(2)]);
        assert_eq!(result, expr!(1));
    }

    #[test]
    fn test_gamma_of_three() {
        let strategy = GammaSimplificationStrategy;
        let result = strategy.simplify(&[expr!(3)]);
        assert_eq!(result, expr!(2));
    }

    #[test]
    fn test_gamma_of_four() {
        let strategy = GammaSimplificationStrategy;
        let result = strategy.simplify(&[expr!(4)]);
        assert_eq!(result, expr!(6));
    }

    #[test]
    fn test_gamma_of_five() {
        let strategy = GammaSimplificationStrategy;
        let result = strategy.simplify(&[expr!(5)]);
        assert_eq!(result, expr!(24));
    }

    #[test]
    fn test_factorial_of_zero() {
        let strategy = FactorialSimplificationStrategy;
        let result = strategy.simplify(&[expr!(0)]);
        assert_eq!(result, expr!(1));
    }

    #[test]
    fn test_factorial_of_one() {
        let strategy = FactorialSimplificationStrategy;
        let result = strategy.simplify(&[expr!(1)]);
        assert_eq!(result, expr!(1));
    }

    #[test]
    fn test_factorial_of_five() {
        let strategy = FactorialSimplificationStrategy;
        let result = strategy.simplify(&[expr!(5)]);
        assert_eq!(result, expr!(120));
    }

    #[test]
    fn test_factorial_of_ten() {
        let strategy = FactorialSimplificationStrategy;
        let result = strategy.simplify(&[expr!(10)]);
        assert_eq!(result, expr!(3628800));
    }
}
