//! Logarithmic Function Simplification Strategies
//!
//! Implements algebraic rewrite rules for logarithmic functions (log, ln).

use super::strategy::SimplificationStrategy;
use crate::core::commutativity::Commutativity;
use crate::core::{Expression, Number};

/// Logarithm (base 10) simplification strategy
pub struct LogarithmSimplificationStrategy;

impl SimplificationStrategy for LogarithmSimplificationStrategy {
    fn simplify(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            match &args[0] {
                Expression::Number(Number::Integer(n)) if *n == 1 => Expression::integer(0),

                Expression::Number(Number::Integer(n)) if *n == 10 => Expression::integer(1),

                Expression::Pow(base, exp) => Expression::mul(vec![
                    exp.as_ref().clone(),
                    Expression::function("log", vec![base.as_ref().clone()]),
                ]),

                Expression::Mul(factors) => {
                    let commutativity =
                        Commutativity::combine(factors.iter().map(|f| f.commutativity()));

                    if commutativity.can_sort() {
                        let log_terms: Vec<Expression> = factors
                            .iter()
                            .map(|f| Expression::function("log", vec![f.clone()]))
                            .collect();
                        Expression::add(log_terms)
                    } else {
                        Expression::function("log", args.to_vec())
                    }
                }

                _ => Expression::function("log", args.to_vec()),
            }
        } else if args.len() == 2 {
            let x = &args[0];
            let base = &args[1];

            if x == base {
                Expression::integer(1)
            } else if x.is_one() {
                Expression::integer(0)
            } else {
                Expression::function("log", args.to_vec())
            }
        } else {
            Expression::function("log", args.to_vec())
        }
    }

    fn applies_to(&self, args: &[Expression]) -> bool {
        !args.is_empty() && args.len() <= 2
    }

    fn name(&self) -> &str {
        "LogarithmSimplificationStrategy"
    }
}

/// Natural logarithm (ln) simplification strategy
pub struct NaturalLogSimplificationStrategy;

impl SimplificationStrategy for NaturalLogSimplificationStrategy {
    fn simplify(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            match &args[0] {
                Expression::Number(Number::Integer(n)) if *n == 1 => Expression::integer(0),

                Expression::Function {
                    name,
                    args: inner_args,
                } if name == "exp" && inner_args.len() == 1 => inner_args[0].clone(),

                Expression::Pow(base, exp) => Expression::mul(vec![
                    exp.as_ref().clone(),
                    Expression::function("ln", vec![base.as_ref().clone()]),
                ]),

                _ => Expression::function("ln", args.to_vec()),
            }
        } else {
            Expression::function("ln", args.to_vec())
        }
    }

    fn applies_to(&self, args: &[Expression]) -> bool {
        args.len() == 1
    }

    fn name(&self) -> &str {
        "NaturalLogSimplificationStrategy"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_log_of_one() {
        let strategy = LogarithmSimplificationStrategy;
        let result = strategy.simplify(&[expr!(1)]);
        assert_eq!(result, expr!(0));
    }

    #[test]
    fn test_log_of_ten() {
        let strategy = LogarithmSimplificationStrategy;
        let result = strategy.simplify(&[expr!(10)]);
        assert_eq!(result, expr!(1));
    }

    #[test]
    fn test_log_power_rule() {
        let strategy = LogarithmSimplificationStrategy;
        let result = strategy.simplify(&[expr!(x ^ 2)]);

        if let Expression::Mul(terms) = result {
            assert_eq!(terms.len(), 2);
        } else {
            panic!("Expected multiplication");
        }
    }

    #[test]
    fn test_log_with_base() {
        let strategy = LogarithmSimplificationStrategy;
        let x = symbol!(x);
        let result = strategy.simplify(&[x.clone().into(), x.into()]);
        assert_eq!(result, expr!(1));
    }

    #[test]
    fn test_ln_of_one() {
        let strategy = NaturalLogSimplificationStrategy;
        let result = strategy.simplify(&[expr!(1)]);
        assert_eq!(result, expr!(0));
    }

    #[test]
    fn test_ln_of_exp() {
        let strategy = NaturalLogSimplificationStrategy;
        let x = symbol!(x);
        let exp_x = Expression::function("exp", vec![x.clone().into()]);
        let result = strategy.simplify(&[exp_x]);
        assert_eq!(result, x.into());
    }

    #[test]
    fn test_ln_power_rule() {
        let strategy = NaturalLogSimplificationStrategy;
        let result = strategy.simplify(&[expr!(x ^ 3)]);

        if let Expression::Mul(terms) = result {
            assert_eq!(terms.len(), 2);
        } else {
            panic!("Expected multiplication");
        }
    }
}
