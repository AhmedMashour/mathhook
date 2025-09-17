//! Elementary Function Simplification Strategies
//!
//! Implements algebraic rewrite rules for elementary functions (sqrt, abs, exp).

use super::strategy::SimplificationStrategy;
use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};

/// Square root simplification strategy
pub struct SqrtSimplificationStrategy;

impl SqrtSimplificationStrategy {
    fn integer_sqrt(&self, n: &BigInt) -> Option<BigInt> {
        if n < &BigInt::zero() {
            return None;
        }

        if let Some(val) = n.to_i64() {
            let sqrt_val = (val as f64).sqrt() as i64;
            let sqrt_bigint = BigInt::from(sqrt_val);

            if &(&sqrt_bigint * &sqrt_bigint) == n {
                Some(sqrt_bigint)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl SimplificationStrategy for SqrtSimplificationStrategy {
    fn simplify(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            match &args[0] {
                Expression::Number(Number::Integer(n)) => {
                    if n.is_zero() {
                        Expression::integer(0)
                    } else if *n == 1 {
                        Expression::integer(1)
                    } else if let Some(sqrt_val) = self.integer_sqrt(&BigInt::from(*n)) {
                        Expression::big_integer(sqrt_val)
                    } else {
                        Expression::function("sqrt", args.to_vec())
                    }
                }

                Expression::Pow(base, exp) => {
                    if exp.as_ref() == &Expression::integer(2) {
                        base.as_ref().clone()
                    } else {
                        Expression::function("sqrt", args.to_vec())
                    }
                }

                _ => Expression::function("sqrt", args.to_vec()),
            }
        } else {
            Expression::function("sqrt", args.to_vec())
        }
    }

    fn applies_to(&self, args: &[Expression]) -> bool {
        args.len() == 1
    }

    fn name(&self) -> &str {
        "SqrtSimplificationStrategy"
    }
}

/// Absolute value simplification strategy
pub struct AbsSimplificationStrategy;

impl SimplificationStrategy for AbsSimplificationStrategy {
    fn simplify(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            match &args[0] {
                Expression::Number(Number::Integer(n)) => Expression::integer(n.abs()),
                Expression::Number(Number::Float(f)) => Expression::number(Number::float(f.abs())),
                _ => Expression::function("abs", args.to_vec()),
            }
        } else {
            Expression::function("abs", args.to_vec())
        }
    }

    fn applies_to(&self, args: &[Expression]) -> bool {
        args.len() == 1
    }

    fn name(&self) -> &str {
        "AbsSimplificationStrategy"
    }
}

/// Exponential function simplification strategy
pub struct ExpSimplificationStrategy;

impl SimplificationStrategy for ExpSimplificationStrategy {
    fn simplify(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            match &args[0] {
                Expression::Number(Number::Integer(n)) if n.is_zero() => Expression::integer(1),

                Expression::Function {
                    name,
                    args: inner_args,
                } if name == "ln" && inner_args.len() == 1 => inner_args[0].clone(),

                _ => Expression::function("exp", args.to_vec()),
            }
        } else {
            Expression::function("exp", args.to_vec())
        }
    }

    fn applies_to(&self, args: &[Expression]) -> bool {
        args.len() == 1
    }

    fn name(&self) -> &str {
        "ExpSimplificationStrategy"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_sqrt_of_zero() {
        let strategy = SqrtSimplificationStrategy;
        let result = strategy.simplify(&[expr!(0)]);
        assert_eq!(result, expr!(0));
    }

    #[test]
    fn test_sqrt_of_one() {
        let strategy = SqrtSimplificationStrategy;
        let result = strategy.simplify(&[expr!(1)]);
        assert_eq!(result, expr!(1));
    }

    #[test]
    fn test_sqrt_of_four() {
        let strategy = SqrtSimplificationStrategy;
        let result = strategy.simplify(&[expr!(4)]);
        assert_eq!(result, expr!(2));
    }

    #[test]
    fn test_sqrt_of_nine() {
        let strategy = SqrtSimplificationStrategy;
        let result = strategy.simplify(&[expr!(9)]);
        assert_eq!(result, expr!(3));
    }

    #[test]
    fn test_sqrt_of_power() {
        let strategy = SqrtSimplificationStrategy;
        let x = symbol!(x);
        let result = strategy.simplify(&[expr!(x ^ 2)]);
        assert_eq!(result, x.into());
    }

    #[test]
    fn test_abs_of_positive_integer() {
        let strategy = AbsSimplificationStrategy;
        let result = strategy.simplify(&[expr!(5)]);
        assert_eq!(result, expr!(5));
    }

    #[test]
    fn test_abs_of_negative_integer() {
        let strategy = AbsSimplificationStrategy;
        let result = strategy.simplify(&[expr!(-5)]);
        assert_eq!(result, expr!(5));
    }

    #[test]
    fn test_exp_of_zero() {
        let strategy = ExpSimplificationStrategy;
        let result = strategy.simplify(&[expr!(0)]);
        assert_eq!(result, expr!(1));
    }

    #[test]
    fn test_exp_of_ln() {
        let strategy = ExpSimplificationStrategy;
        let x = symbol!(x);
        let ln_x = Expression::function("ln", vec![x.clone().into()]);
        let result = strategy.simplify(&[ln_x]);
        assert_eq!(result, x.into());
    }
}
