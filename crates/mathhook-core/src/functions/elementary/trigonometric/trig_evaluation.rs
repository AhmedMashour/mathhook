//! Trigonometric function evaluation
//!
//! Provides numerical and symbolic evaluation for circular trigonometric functions.
//! Handles special values, periodicity, and numerical computation.

use crate::core::expression::Expression;
use crate::core::number::Number;
use num_bigint::BigInt;
use num_traits::ToPrimitive;

/// Evaluates sine function
pub fn sin(arg: &Expression) -> Expression {
    if let Some(exact) = try_exact_sin(arg) {
        return exact;
    }

    if let Some(num_val) = try_numeric_sin(arg) {
        return num_val;
    }

    Expression::function("sin", vec![arg.clone()])
}

/// Evaluates cosine function
pub fn cos(arg: &Expression) -> Expression {
    if let Some(exact) = try_exact_cos(arg) {
        return exact;
    }

    if let Some(num_val) = try_numeric_cos(arg) {
        return num_val;
    }

    Expression::function("cos", vec![arg.clone()])
}

/// Evaluates tangent function
pub fn tan(arg: &Expression) -> Expression {
    if let Some(exact) = try_exact_tan(arg) {
        return exact;
    }

    if let Some(num_val) = try_numeric_tan(arg) {
        return num_val;
    }

    Expression::function("tan", vec![arg.clone()])
}

/// Wrapper function pointer for sin (matches ElementaryProperties signature)
pub fn sin_evaluator(args: &[Expression]) -> Expression {
    if args.len() == 1 {
        sin(&args[0])
    } else {
        Expression::function("sin", args.to_vec())
    }
}

/// Wrapper function pointer for cos (matches ElementaryProperties signature)
pub fn cos_evaluator(args: &[Expression]) -> Expression {
    if args.len() == 1 {
        cos(&args[0])
    } else {
        Expression::function("cos", args.to_vec())
    }
}

/// Wrapper function pointer for tan (matches ElementaryProperties signature)
pub fn tan_evaluator(args: &[Expression]) -> Expression {
    if args.len() == 1 {
        tan(&args[0])
    } else {
        Expression::function("tan", args.to_vec())
    }
}

fn try_exact_sin(arg: &Expression) -> Option<Expression> {
    if arg.is_zero() {
        return Some(Expression::integer(0));
    }

    if matches!(
        arg,
        Expression::Constant(crate::core::constants::MathConstant::Pi)
    ) {
        return Some(Expression::integer(0));
    }

    if is_pi_over_2(arg) {
        return Some(Expression::integer(1));
    }

    if is_pi_over_6(arg) {
        return Some(Expression::rational(1, 2));
    }

    if is_pi_over_4(arg) {
        return Some(Expression::mul(vec![
            Expression::pow(Expression::integer(2), Expression::rational(1, 2)),
            Expression::rational(1, 2),
        ]));
    }

    if is_pi_over_3(arg) {
        return Some(Expression::mul(vec![
            Expression::pow(Expression::integer(3), Expression::rational(1, 2)),
            Expression::rational(1, 2),
        ]));
    }

    None
}

fn try_exact_cos(arg: &Expression) -> Option<Expression> {
    if arg.is_zero() {
        return Some(Expression::integer(1));
    }

    if matches!(
        arg,
        Expression::Constant(crate::core::constants::MathConstant::Pi)
    ) {
        return Some(Expression::integer(-1));
    }

    if is_pi_over_2(arg) {
        return Some(Expression::integer(0));
    }

    if is_pi_over_6(arg) {
        return Some(Expression::mul(vec![
            Expression::pow(Expression::integer(3), Expression::rational(1, 2)),
            Expression::rational(1, 2),
        ]));
    }

    if is_pi_over_4(arg) {
        return Some(Expression::mul(vec![
            Expression::pow(Expression::integer(2), Expression::rational(1, 2)),
            Expression::rational(1, 2),
        ]));
    }

    if is_pi_over_3(arg) {
        return Some(Expression::rational(1, 2));
    }

    None
}

fn try_exact_tan(arg: &Expression) -> Option<Expression> {
    if arg.is_zero() {
        return Some(Expression::integer(0));
    }

    if matches!(
        arg,
        Expression::Constant(crate::core::constants::MathConstant::Pi)
    ) {
        return Some(Expression::integer(0));
    }

    if is_pi_over_2(arg) {
        return Some(Expression::function("tan", vec![arg.clone()]));
    }

    if is_pi_over_4(arg) {
        return Some(Expression::integer(1));
    }

    if is_pi_over_6(arg) {
        return Some(Expression::pow(
            Expression::integer(3),
            Expression::rational(-1, 2),
        ));
    }

    if is_pi_over_3(arg) {
        return Some(Expression::pow(
            Expression::integer(3),
            Expression::rational(1, 2),
        ));
    }

    None
}

fn try_numeric_sin(arg: &Expression) -> Option<Expression> {
    match arg {
        Expression::Number(Number::Integer(n)) => Some(Expression::float((*n as f64).sin())),
        Expression::Number(Number::Rational(r)) => {
            let val = r.as_ref().to_f64().unwrap();
            Some(Expression::float(val.sin()))
        }
        Expression::Number(Number::Float(f)) => Some(Expression::float(f.sin())),
        _ => None,
    }
}

fn try_numeric_cos(arg: &Expression) -> Option<Expression> {
    match arg {
        Expression::Number(Number::Integer(n)) => Some(Expression::float((*n as f64).cos())),
        Expression::Number(Number::Rational(r)) => {
            let val = r.as_ref().to_f64().unwrap();
            Some(Expression::float(val.cos()))
        }
        Expression::Number(Number::Float(f)) => Some(Expression::float(f.cos())),
        _ => None,
    }
}

fn try_numeric_tan(arg: &Expression) -> Option<Expression> {
    match arg {
        Expression::Number(Number::Integer(n)) => Some(Expression::float((*n as f64).tan())),
        Expression::Number(Number::Rational(r)) => {
            let val = r.as_ref().to_f64().unwrap();
            Some(Expression::float(val.tan()))
        }
        Expression::Number(Number::Float(f)) => Some(Expression::float(f.tan())),
        _ => None,
    }
}

fn is_pi_over_2(expr: &Expression) -> bool {
    if let Expression::Mul(terms) = expr {
        if terms.len() == 2 {
            let (rational_idx, _pi_idx) = if matches!(
                &terms[0],
                Expression::Constant(crate::core::constants::MathConstant::Pi)
            ) {
                (1, 0)
            } else if matches!(
                &terms[1],
                Expression::Constant(crate::core::constants::MathConstant::Pi)
            ) {
                (0, 1)
            } else {
                return false;
            };

            if let Expression::Number(Number::Rational(r)) = &terms[rational_idx] {
                return r.numer() == &BigInt::from(1) && r.denom() == &BigInt::from(2);
            }
        }
    }
    false
}

fn is_pi_over_3(expr: &Expression) -> bool {
    if let Expression::Mul(terms) = expr {
        if terms.len() == 2 {
            let (rational_idx, _pi_idx) = if matches!(
                &terms[0],
                Expression::Constant(crate::core::constants::MathConstant::Pi)
            ) {
                (1, 0)
            } else if matches!(
                &terms[1],
                Expression::Constant(crate::core::constants::MathConstant::Pi)
            ) {
                (0, 1)
            } else {
                return false;
            };

            if let Expression::Number(Number::Rational(r)) = &terms[rational_idx] {
                return r.numer() == &BigInt::from(1) && r.denom() == &BigInt::from(3);
            }
        }
    }
    false
}

fn is_pi_over_4(expr: &Expression) -> bool {
    if let Expression::Mul(terms) = expr {
        if terms.len() == 2 {
            let (rational_idx, _pi_idx) = if matches!(
                &terms[0],
                Expression::Constant(crate::core::constants::MathConstant::Pi)
            ) {
                (1, 0)
            } else if matches!(
                &terms[1],
                Expression::Constant(crate::core::constants::MathConstant::Pi)
            ) {
                (0, 1)
            } else {
                return false;
            };

            if let Expression::Number(Number::Rational(r)) = &terms[rational_idx] {
                return r.numer() == &BigInt::from(1) && r.denom() == &BigInt::from(4);
            }
        }
    }
    false
}

fn is_pi_over_6(expr: &Expression) -> bool {
    if let Expression::Mul(terms) = expr {
        if terms.len() == 2 {
            let (rational_idx, _pi_idx) = if matches!(
                &terms[0],
                Expression::Constant(crate::core::constants::MathConstant::Pi)
            ) {
                (1, 0)
            } else if matches!(
                &terms[1],
                Expression::Constant(crate::core::constants::MathConstant::Pi)
            ) {
                (0, 1)
            } else {
                return false;
            };

            if let Expression::Number(Number::Rational(r)) = &terms[rational_idx] {
                return r.numer() == &BigInt::from(1) && r.denom() == &BigInt::from(6);
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sin_of_zero_returns_zero() {
        let zero = Expression::integer(0);
        let result = sin(&zero);

        // Debug: print what we got
        println!("Input: {:?}", zero);
        println!("Result: {:?}", result);

        // The result should be Expression::integer(0), not a function wrapper
        assert!(
            matches!(result, Expression::Number(Number::Integer(0))),
            "Expected sin(0) = 0 (integer), got: {:?}",
            result
        );
    }

    #[test]
    fn test_sin_of_one_returns_float() {
        let one = Expression::integer(1);
        let result = sin(&one);

        println!("Input: {:?}", one);
        println!("Result: {:?}", result);

        // The result should be a float (0.8414...)
        match &result {
            Expression::Number(Number::Float(f)) => {
                assert!(
                    (*f - 0.8414709848078965).abs() < 1e-10,
                    "Expected sin(1) ~ 0.841, got: {}",
                    f
                );
            }
            _ => panic!("Expected sin(1) to return Float, got: {:?}", result),
        }
    }

    #[test]
    fn test_cos_of_zero_returns_one() {
        let zero = Expression::integer(0);
        let result = cos(&zero);

        println!("Input: {:?}", zero);
        println!("Result: {:?}", result);

        // cos(0) should return 1
        assert!(
            matches!(result, Expression::Number(Number::Integer(1))),
            "Expected cos(0) = 1 (integer), got: {:?}",
            result
        );
    }
}
