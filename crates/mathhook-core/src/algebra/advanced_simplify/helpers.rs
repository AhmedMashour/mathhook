//! Helper functions for advanced simplification

use crate::core::commutativity::Commutativity;
use crate::core::{Expression, Number};
use num_bigint::BigInt;
use num_traits::{One, Zero};
use num_traits::ToPrimitive;

impl Expression {
    /// Compute factorial for integer values
    pub(super) fn compute_factorial(&self, arg: &Expression) -> Expression {
        match arg {
            Expression::Number(Number::Integer(n)) => {
                if let Some(val) = n.to_i64() {
                    if val >= 0 && val <= 20 {
                        // Reasonable limit to prevent overflow
                        let factorial_result = self.factorial_i64(val as u64);
                        Expression::big_integer(factorial_result)
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
    pub(super) fn factorial_i64(&self, n: u64) -> BigInt {
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
    pub(super) fn simplify_log_function(&self, args: &[Expression]) -> Expression {
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

                // log(a*b) = log(a) + log(b) ONLY if commutative
                Expression::Mul(factors) => {
                    let commutativity = Commutativity::combine(
                        factors.iter().map(|f| f.commutativity())
                    );

                    if commutativity.can_sort() {
                        // Commutative: Can apply logarithm product rule
                        let log_terms: Vec<Expression> = factors
                            .iter()
                            .map(|f| Expression::function("log", vec![f.clone()]))
                            .collect();
                        Expression::add(log_terms)
                    } else {
                        // Noncommutative: Cannot split log(AB) into log(A) + log(B)
                        Expression::function("log", args.to_vec())
                    }
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
    pub(super) fn simplify_ln_function(&self, args: &[Expression]) -> Expression {
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
    pub(super) fn is_trig_function(&self, name: &str) -> bool {
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
    pub(super) fn simplify_trig_function(&self, name: &str, args: &[Expression]) -> Expression {
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
    pub(super) fn simplify_sqrt(&self, args: &[Expression]) -> Expression {
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
                            Expression::big_integer(sqrt_val)
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
    pub(super) fn simplify_abs(&self, args: &[Expression]) -> Expression {
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
    pub(super) fn simplify_exp(&self, args: &[Expression]) -> Expression {
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
    pub(super) fn simplify_gamma(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            match &args[0] {
                Expression::Number(Number::Integer(n)) => {
                    if let Some(val) = n.to_i64() {
                        if val > 0 && val <= 10 {
                            // Gamma(n) = (n-1)! for positive integers
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

    /// Compute integer square root if it exists
    pub(super) fn integer_sqrt(&self, n: &BigInt) -> Option<BigInt> {
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
}
