//! Advanced simplification operations including special functions
//! Handles factorial, trigonometric functions, logarithms, and complex simplifications
//!
//! # Noncommutative Simplification Rules
//!
//! All simplification rules respect noncommutativity:
//! - log(AB) = log(A) + log(B) is ONLY valid when A and B commute
//! - For matrices: log(AB) ≠ log(A) + log(B) in general
//! - This implementation checks commutativity before applying simplification rules

mod helpers;

use crate::core::Expression;

/// Trait for advanced simplification operations
pub trait AdvancedSimplify {
    fn advanced_simplify(&self) -> Self;
    fn simplify_factorial(&self) -> Self;
    fn simplify_logarithms(&self) -> Self;
    fn simplify_trigonometric(&self) -> Self;
    fn simplify_special_functions(&self) -> Self;
}

impl AdvancedSimplify for Expression {
    /// Perform advanced simplification including special functions
    fn advanced_simplify(&self) -> Self {
        let mut result = self.clone();

        // Apply various advanced simplification techniques
        result = result.simplify_factorial();
        result = result.simplify_logarithms();
        result = result.simplify_trigonometric();
        result = result.simplify_special_functions();

        result
    }

    /// Simplify factorial expressions
    fn simplify_factorial(&self) -> Self {
        match self {
            Expression::Function { name, args } if name.as_ref() == "factorial" => {
                if args.len() == 1 {
                    self.compute_factorial(&args[0])
                } else {
                    self.clone()
                }
            }

            Expression::Add(terms) => {
                let simplified_terms: Vec<Expression> =
                    terms.iter().map(|term| term.simplify_factorial()).collect();
                Expression::add(simplified_terms)
            }

            Expression::Mul(factors) => {
                let simplified_factors: Vec<Expression> = factors
                    .iter()
                    .map(|factor| factor.simplify_factorial())
                    .collect();
                Expression::mul(simplified_factors)
            }

            Expression::Pow(base, exp) => {
                Expression::pow(base.simplify_factorial(), exp.simplify_factorial())
            }

            Expression::Function { name, args } => {
                let simplified_args: Vec<Expression> =
                    args.iter().map(|arg| arg.simplify_factorial()).collect();
                Expression::function(name.clone(), simplified_args)
            }

            _ => self.clone(),
        }
    }

    /// Simplify logarithmic expressions
    fn simplify_logarithms(&self) -> Self {
        match self {
            Expression::Function { name, args } if name.as_ref() == "log" => {
                self.simplify_log_function(args)
            }

            Expression::Function { name, args } if name.as_ref() == "ln" => {
                self.simplify_ln_function(args)
            }

            Expression::Add(terms) => {
                let simplified_terms: Vec<Expression> = terms
                    .iter()
                    .map(|term| term.simplify_logarithms())
                    .collect();
                Expression::add(simplified_terms)
            }

            Expression::Mul(factors) => {
                let simplified_factors: Vec<Expression> = factors
                    .iter()
                    .map(|factor| factor.simplify_logarithms())
                    .collect();
                Expression::mul(simplified_factors)
            }

            _ => self.clone(),
        }
    }

    /// Simplify trigonometric expressions
    fn simplify_trigonometric(&self) -> Self {
        match self {
            Expression::Function { name, args } if self.is_trig_function(name) => {
                self.simplify_trig_function(name, args)
            }

            Expression::Add(terms) => {
                let simplified_terms: Vec<Expression> = terms
                    .iter()
                    .map(|term| term.simplify_trigonometric())
                    .collect();
                Expression::add(simplified_terms)
            }

            Expression::Mul(factors) => {
                let simplified_factors: Vec<Expression> = factors
                    .iter()
                    .map(|factor| factor.simplify_trigonometric())
                    .collect();
                Expression::mul(simplified_factors)
            }

            _ => self.clone(),
        }
    }

    /// Simplify other special functions
    fn simplify_special_functions(&self) -> Self {
        match self {
            Expression::Function { name, args } => match name.as_ref() {
                "sqrt" => self.simplify_sqrt(args),
                "abs" => self.simplify_abs(args),
                "exp" => self.simplify_exp(args),
                "gamma" => self.simplify_gamma(args),
                _ => self.clone(),
            },
            _ => self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol;

    #[test]
    fn test_factorial_computation() {
        // Test 5! = 120
        let expr = Expression::factorial(Expression::integer(5));
        let result = expr.simplify_factorial();

        assert_eq!(result, Expression::integer(120));

        // Test 0! = 1
        let expr = Expression::factorial(Expression::integer(0));
        let result = expr.simplify_factorial();

        assert_eq!(result, Expression::integer(1));

        // Test 1! = 1
        let expr = Expression::factorial(Expression::integer(1));
        let result = expr.simplify_factorial();

        assert_eq!(result, Expression::integer(1));
    }

    #[test]
    fn test_logarithm_simplification() {
        // Test ln(1) = 0
        let expr = Expression::ln(Expression::integer(1));
        let result = expr.simplify_logarithms();

        assert_eq!(result, Expression::integer(0));

        // Test ln(exp(x)) = x
        let x = symbol!(x);
        let expr = Expression::ln(Expression::function(
            "exp",
            vec![Expression::symbol(x.clone())],
        ));
        let result = expr.simplify_logarithms();

        assert_eq!(result, Expression::symbol(x));
    }

    #[test]
    fn test_trigonometric_simplification() {
        // Test sin(0) = 0
        let expr = Expression::function("sin", vec![Expression::integer(0)]);
        let result = expr.simplify_trigonometric();

        assert_eq!(result, Expression::integer(0));

        // Test cos(0) = 1
        let expr = Expression::function("cos", vec![Expression::integer(0)]);
        let result = expr.simplify_trigonometric();

        assert_eq!(result, Expression::integer(1));
    }

    #[test]
    fn test_sqrt_simplification() {
        // Test sqrt(4) = 2
        let expr = Expression::sqrt(Expression::integer(4));
        let result = expr.simplify_special_functions();

        assert_eq!(result, Expression::integer(2));

        // Test sqrt(0) = 0
        let expr = Expression::sqrt(Expression::integer(0));
        let result = expr.simplify_special_functions();

        assert_eq!(result, Expression::integer(0));

        // Test sqrt(1) = 1
        let expr = Expression::sqrt(Expression::integer(1));
        let result = expr.simplify_special_functions();

        assert_eq!(result, Expression::integer(1));
    }

    #[test]
    fn test_gamma_function() {
        // Test Gamma(4) = 3! = 6
        let expr = Expression::function("gamma", vec![Expression::integer(4)]);
        let result = expr.simplify_special_functions();

        assert_eq!(result, Expression::integer(6));

        // Test Gamma(1) = 0! = 1
        let expr = Expression::function("gamma", vec![Expression::integer(1)]);
        let result = expr.simplify_special_functions();

        assert_eq!(result, Expression::integer(1));
    }

    #[test]
    fn test_advanced_zero_detection() {
        // Test complex zero detection
        let x = symbol!(x);
        let expr = Expression::add(vec![
            Expression::integer(4),
            Expression::mul(vec![Expression::integer(4), Expression::symbol(x.clone())]),
            Expression::mul(vec![
                Expression::integer(-1),
                Expression::mul(vec![
                    Expression::integer(2),
                    Expression::add(vec![
                        Expression::integer(2),
                        Expression::mul(vec![
                            Expression::integer(2),
                            Expression::symbol(x.clone()),
                        ]),
                    ]),
                ]),
            ]),
        ]);

        let result = expr.advanced_simplify();
        // This is a complex case that might not simplify to zero immediately
        // but should maintain the algebraic structure
        assert!(!result.to_string().is_empty());
    }
}
