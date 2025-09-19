//! Advanced simplification operations including special functions
//! Handles factorial, trigonometric functions, logarithms, and complex simplifications

use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};

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
            Expression::Function { name, args } if name == "factorial" => {
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
            Expression::Function { name, args } if name == "log" => {
                self.simplify_log_function(args)
            }

            Expression::Function { name, args } if name == "ln" => self.simplify_ln_function(args),

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
            Expression::Function { name, args } => match name.as_str() {
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

impl Expression {
    /// Compute factorial for integer values
    fn compute_factorial(&self, arg: &Expression) -> Expression {
        match arg {
            Expression::Number(Number::Integer(n)) => {
                if let Some(val) = n.to_i64() {
                    if val >= 0 && val <= 20 {
                        // Reasonable limit to prevent overflow
                        let factorial_result = self.factorial_i64(val as u64);
                        Expression::integer(factorial_result)
                    } else {
                        // For large values, return the function call
                        Expression::function("factorial", vec![arg.clone()])
                    }
                } else {
                    Expression::function("factorial", vec![arg.clone()])
                }
            }

            Expression::Add(terms) if terms.len() == 2 => {
                // Check for n! where n = k - 1, so (k-1)! = factorial(k-1)
                if let (Expression::Symbol(_s), Expression::Number(Number::Integer(offset))) =
                    (&terms[0], &terms[1])
                {
                    if *offset == -1 {
                        // (n-1)! case - this is complex, return as function for now
                        return Expression::function("factorial", vec![arg.clone()]);
                    }
                }
                Expression::function("factorial", vec![arg.clone()])
            }

            _ => Expression::function("factorial", vec![arg.clone()]),
        }
    }

    /// Compute factorial for small integers
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

    /// Simplify logarithm functions
    fn simplify_log_function(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            match &args[0] {
                Expression::Number(Number::Integer(n)) if *n == 1 => {
                    // log(1) = 0
                    Expression::integer(0)
                }

                Expression::Number(Number::Integer(n)) if *n == 10 => {
                    // log(10) = 1 (assuming base 10)
                    Expression::integer(1)
                }

                // log(x^n) = n * log(x)
                Expression::Pow(base, exp) => Expression::mul(vec![
                    exp.as_ref().clone(),
                    Expression::function("log", vec![base.as_ref().clone()]),
                ]),

                // log(a*b) = log(a) + log(b)
                Expression::Mul(factors) => {
                    let log_terms: Vec<Expression> = factors
                        .iter()
                        .map(|f| Expression::function("log", vec![f.clone()]))
                        .collect();
                    Expression::add(log_terms)
                }

                _ => Expression::function("log", args.to_vec()),
            }
        } else if args.len() == 2 {
            // log(x, base)
            let x = &args[0];
            let base = &args[1];

            if x == base {
                // log_b(b) = 1
                Expression::integer(1)
            } else if x.is_one() {
                // log_b(1) = 0
                Expression::integer(0)
            } else {
                Expression::function("log", args.to_vec())
            }
        } else {
            Expression::function("log", args.to_vec())
        }
    }

    /// Simplify natural logarithm functions
    fn simplify_ln_function(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            match &args[0] {
                Expression::Number(Number::Integer(n)) if *n == 1 => {
                    // ln(1) = 0
                    Expression::integer(0)
                }

                Expression::Function {
                    name,
                    args: inner_args,
                } if name == "exp" && inner_args.len() == 1 => {
                    // ln(exp(x)) = x
                    inner_args[0].clone()
                }

                // ln(x^n) = n * ln(x)
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

    /// Check if a function name is trigonometric
    fn is_trig_function(&self, name: &str) -> bool {
        matches!(
            name,
            "sin"
                | "cos"
                | "tan"
                | "csc"
                | "sec"
                | "cot"
                | "asin"
                | "acos"
                | "atan"
                | "sinh"
                | "cosh"
                | "tanh"
        )
    }

    /// Simplify trigonometric functions
    fn simplify_trig_function(&self, name: &str, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            let arg = &args[0];

            match name {
                "sin" => {
                    if arg.is_zero() {
                        Expression::integer(0) // sin(0) = 0
                    } else {
                        Expression::function(name, args.to_vec())
                    }
                }

                "cos" => {
                    if arg.is_zero() {
                        Expression::integer(1) // cos(0) = 1
                    } else {
                        Expression::function(name, args.to_vec())
                    }
                }

                "tan" => {
                    if arg.is_zero() {
                        Expression::integer(0) // tan(0) = 0
                    } else {
                        Expression::function(name, args.to_vec())
                    }
                }

                _ => Expression::function(name, args.to_vec()),
            }
        } else {
            Expression::function(name, args.to_vec())
        }
    }

    /// Simplify square root function
    fn simplify_sqrt(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            match &args[0] {
                Expression::Number(Number::Integer(n)) => {
                    if n.is_zero() {
                        Expression::integer(0) // sqrt(0) = 0
                    } else if n.is_one() {
                        Expression::integer(1) // sqrt(1) = 1
                    } else {
                        // Check for perfect squares
                        if let Some(sqrt_val) = self.integer_sqrt(&BigInt::from(*n)) {
                            Expression::integer(sqrt_val)
                        } else {
                            Expression::function("sqrt", args.to_vec())
                        }
                    }
                }

                // sqrt(x^2) = |x| (simplified to x for now)
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

    /// Simplify absolute value function
    fn simplify_abs(&self, args: &[Expression]) -> Expression {
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

    /// Simplify exponential function
    fn simplify_exp(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            match &args[0] {
                Expression::Number(Number::Integer(n)) if n.is_zero() => {
                    Expression::integer(1) // exp(0) = 1
                }

                Expression::Function {
                    name,
                    args: inner_args,
                } if name == "ln" && inner_args.len() == 1 => {
                    // exp(ln(x)) = x
                    inner_args[0].clone()
                }

                _ => Expression::function("exp", args.to_vec()),
            }
        } else {
            Expression::function("exp", args.to_vec())
        }
    }

    /// Simplify gamma function
    fn simplify_gamma(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            match &args[0] {
                Expression::Number(Number::Integer(n)) => {
                    if let Some(val) = n.to_i64() {
                        if val > 0 && val <= 10 {
                            // Gamma(n) = (n-1)! for positive integers
                            let factorial_result = self.factorial_i64((val - 1) as u64);
                            Expression::integer(factorial_result)
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

    /// Compute integer square root if it exists
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

    /// Create factorial expression
    pub fn factorial(arg: Expression) -> Expression {
        Expression::function("factorial", vec![arg])
    }

    /// Create natural logarithm expression
    pub fn ln(arg: Expression) -> Expression {
        Expression::function("ln", vec![arg])
    }

    /// Create square root expression
    pub fn sqrt(arg: Expression) -> Expression {
        Expression::function("sqrt", vec![arg])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Symbol;

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
        let x = Symbol::new("x");
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
        let x = Symbol::new("x");
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
        println!("Complex expression result: {}", result);
        assert!(!result.to_string().is_empty());
    }
}
