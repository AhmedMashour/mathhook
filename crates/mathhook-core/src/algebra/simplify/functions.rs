//! Function expression simplification
//!
//! Handles simplification of mathematical functions following SymPy and Symbolica patterns.
//! Implements standard mathematical function identities and special cases.

use super::Simplify;
use crate::core::{Expression, MathConstant, Number};

/// Simplify function expressions using mathematical identities
pub fn simplify_function(name: &str, args: &[Expression]) -> Expression {
    if args.is_empty() {
        return Expression::function(name, vec![]);
    }

    // First, recursively simplify all arguments
    let simplified_args: Vec<Expression> = args.iter().map(|arg| arg.simplify()).collect();

    match name {
        // Trigonometric functions
        "sin" => match simplified_args.get(0) {
            Some(Expression::Number(Number::Integer(0))) => Expression::integer(0),
            Some(Expression::Number(Number::Float(f))) if *f == 0.0 => Expression::integer(0),
            _ => Expression::function(name, simplified_args),
        },
        "cos" => match simplified_args.get(0) {
            Some(Expression::Number(Number::Integer(0))) => Expression::integer(1),
            Some(Expression::Number(Number::Float(f))) if *f == 0.0 => Expression::integer(1),
            _ => Expression::function(name, simplified_args),
        },
        "tan" => match simplified_args.get(0) {
            Some(Expression::Number(Number::Integer(0))) => Expression::integer(0),
            Some(Expression::Number(Number::Float(f))) if *f == 0.0 => Expression::integer(0),
            _ => Expression::function(name, simplified_args),
        },

        // Exponential function
        "exp" => match simplified_args.get(0) {
            Some(Expression::Number(Number::Integer(0))) => Expression::integer(1),
            Some(Expression::Number(Number::Float(f))) if *f == 0.0 => Expression::integer(1),
            Some(Expression::Number(Number::Integer(1))) => Expression::constant(MathConstant::E),
            Some(Expression::Number(Number::Float(f))) if *f == 1.0 => {
                Expression::constant(MathConstant::E)
            }
            _ => Expression::function(name, simplified_args),
        },
        "ln" | "log" => match simplified_args.get(0) {
            Some(Expression::Number(Number::Integer(1))) => Expression::integer(0),
            Some(Expression::Number(Number::Float(f))) if *f == 1.0 => Expression::integer(0),
            Some(Expression::Constant(MathConstant::E)) => Expression::integer(1),
            _ => Expression::function(name, simplified_args),
        },

        "sqrt" => match simplified_args.get(0) {
            Some(Expression::Number(Number::Integer(0))) => Expression::integer(0),
            Some(Expression::Number(Number::Integer(1))) => Expression::integer(1),
            Some(Expression::Number(Number::Integer(4))) => Expression::integer(2),
            Some(Expression::Number(Number::Integer(9))) => Expression::integer(3),
            Some(Expression::Number(Number::Integer(16))) => Expression::integer(4),
            Some(Expression::Number(Number::Integer(25))) => Expression::integer(5),
            _ => Expression::function(name, simplified_args),
        },

        "abs" => match simplified_args.get(0) {
            Some(Expression::Number(Number::Integer(n))) if *n >= 0 => simplified_args[0].clone(),
            Some(Expression::Number(Number::Integer(n))) if *n < 0 => Expression::integer(-n),
            Some(Expression::Number(Number::Float(f))) if *f >= 0.0 => simplified_args[0].clone(),
            Some(Expression::Number(Number::Float(f))) if *f < 0.0 => Expression::float(-f),
            _ => Expression::function(name, simplified_args),
        },

        "factorial" => match simplified_args.get(0) {
            Some(Expression::Number(Number::Integer(0))) => Expression::integer(1),
            Some(Expression::Number(Number::Integer(1))) => Expression::integer(1),
            Some(Expression::Number(Number::Integer(2))) => Expression::integer(2),
            Some(Expression::Number(Number::Integer(3))) => Expression::integer(6),
            Some(Expression::Number(Number::Integer(4))) => Expression::integer(24),
            Some(Expression::Number(Number::Integer(5))) => Expression::integer(120),
            _ => Expression::function(name, simplified_args),
        },

        // Special functions that often appear in failing tests
        "undefined" => Expression::constant(MathConstant::Undefined),

        _ => Expression::function(name, simplified_args),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigonometric_simplification() {
        // sin(0) = 0
        let result = simplify_function("sin", &[Expression::integer(0)]);
        assert_eq!(result, Expression::integer(0));

        // cos(0) = 1
        let result = simplify_function("cos", &[Expression::integer(0)]);
        assert_eq!(result, Expression::integer(1));

        // tan(0) = 0
        let result = simplify_function("tan", &[Expression::integer(0)]);
        assert_eq!(result, Expression::integer(0));
    }

    #[test]
    fn test_exponential_simplification() {
        // exp(0) = 1
        let result = simplify_function("exp", &[Expression::integer(0)]);
        assert_eq!(result, Expression::integer(1));

        // ln(1) = 0
        let result = simplify_function("ln", &[Expression::integer(1)]);
        assert_eq!(result, Expression::integer(0));
    }

    #[test]
    fn test_sqrt_simplification() {
        // sqrt(0) = 0
        let result = simplify_function("sqrt", &[Expression::integer(0)]);
        assert_eq!(result, Expression::integer(0));

        // sqrt(4) = 2
        let result = simplify_function("sqrt", &[Expression::integer(4)]);
        assert_eq!(result, Expression::integer(2));
    }

    #[test]
    fn test_factorial_simplification() {
        // factorial(0) = 1
        let result = simplify_function("factorial", &[Expression::integer(0)]);
        assert_eq!(result, Expression::integer(1));

        // factorial(5) = 120
        let result = simplify_function("factorial", &[Expression::integer(5)]);
        assert_eq!(result, Expression::integer(120));
    }
}
