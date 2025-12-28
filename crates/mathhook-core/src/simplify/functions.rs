//! Function expression simplification
//!
//! Handles simplification of mathematical functions.
//! Implements standard mathematical function identities and special cases.
//!
//! ## Architecture
//!
//! Uses Function Intelligence System for automatic evaluation of all registered functions.
//! Hardcoded special cases remain for performance-critical elementary functions.

use super::Simplify;
use crate::core::expression::evaluation::evaluate_function_dispatch;
use crate::core::{Expression, Number};

/// Simplify function expressions using mathematical identities
///
/// ## Evaluation Strategy
///
/// 1. **Mathematical Identities**: Apply algebraic identities first (e.g., exp(log(x)) = x)
/// 2. **Fast Path**: Hardcoded special cases for performance-critical elementary functions
/// 3. **Universal Evaluation**: Direct dispatch for all registered functions
/// 4. **Symbolic Preservation**: Keep expressions symbolic when appropriate
/// 5. **Fallback**: Return unevaluated if no simplification possible
///
/// This hybrid approach provides:
/// - Maximum performance for common cases (sin, cos, exp, log)
/// - Automatic support for all registered functions (gamma, bessel, etc.)
/// - Extensibility for user-defined functions
/// - Correct handling of composition identities
pub fn simplify_function(name: &str, args: &[Expression]) -> Expression {
    if args.is_empty() {
        return Expression::function(name, vec![]);
    }

    // First simplify arguments
    let simplified_args: Vec<Expression> = args.iter().map(|arg| arg.simplify()).collect();

    // Apply mathematical identities before evaluation
    // exp(log(x)) = x and exp(ln(x)) = x
    if name == "exp" && simplified_args.len() == 1 {
        if let Expression::Function {
            name: inner_name,
            args: inner_args,
        } = &simplified_args[0]
        {
            if (inner_name.as_ref() == "log" || inner_name.as_ref() == "ln")
                && inner_args.len() == 1
            {
                return inner_args[0].clone();
            }
        }
    }

    // log(exp(x)) = x and ln(exp(x)) = x
    if (name == "log" || name == "ln") && simplified_args.len() == 1 {
        if let Expression::Function {
            name: inner_name,
            args: inner_args,
        } = &simplified_args[0]
        {
            if inner_name.as_ref() == "exp" && inner_args.len() == 1 {
                return inner_args[0].clone();
            }
        }
    }

    // Determine if we should keep the expression symbolic
    // Keep transcendental functions (sin, cos, tan, etc.) symbolic when applied to non-zero integers
    // This prevents sin(1) → 0.8414... (numeric), keeping sin(1) → sin(1) (symbolic)
    let should_keep_symbolic = matches!(
        name,
        "sin"
            | "cos"
            | "tan"
            | "cot"
            | "sec"
            | "csc"
            | "asin"
            | "acos"
            | "atan"
            | "asinh"
            | "acosh"
            | "atanh"
    ) && simplified_args.len() == 1
        && matches!(&simplified_args[0], Expression::Number(Number::Integer(n)) if *n != 0);

    // Try evaluation
    if let Some(result) = evaluate_function_dispatch(name, &simplified_args) {
        match &result {
            // If dispatch returns any Function, keep original name (function wasn't fully evaluated)
            // This preserves behavior like log(x) staying as log(x), not becoming log10(x)
            Expression::Function { .. } => Expression::function(name, simplified_args),
            // For transcendental functions, keep numeric results symbolic for non-zero integers
            Expression::Number(_) if should_keep_symbolic => {
                Expression::function(name, simplified_args)
            }
            // Return non-Function evaluated results (Symbol, Add, Mul, Number, etc.)
            _ => result,
        }
    } else {
        Expression::function(name, simplified_args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_trigonometric_simplification() {
        // sin(0) = 0
        let result = simplify_function("sin", &[expr!(0)]);
        assert_eq!(result, expr!(0));

        // cos(0) = 1
        let result = simplify_function("cos", &[expr!(0)]);
        assert_eq!(result, expr!(1));

        // tan(0) = 0
        let result = simplify_function("tan", &[expr!(0)]);
        assert_eq!(result, expr!(0));
    }

    #[test]
    fn test_exponential_simplification() {
        // exp(0) = 1
        let result = simplify_function("exp", &[expr!(0)]);
        assert_eq!(result, expr!(1));

        // ln(1) = 0
        let result = simplify_function("ln", &[expr!(1)]);
        assert_eq!(result, expr!(0));
    }

    #[test]
    fn test_sqrt_simplification() {
        // sqrt(0) = 0
        let result = simplify_function("sqrt", &[expr!(0)]);
        assert_eq!(result, expr!(0));

        // sqrt(4) = 2
        let result = simplify_function("sqrt", &[expr!(4)]);
        assert_eq!(result, expr!(2));
    }

    #[test]
    fn test_factorial_simplification() {
        // factorial(0) = 1
        let result = simplify_function("factorial", &[expr!(0)]);
        assert_eq!(result, expr!(1));

        // factorial(5) = 120
        let result = simplify_function("factorial", &[expr!(5)]);
        assert_eq!(result, expr!(120));
    }

    #[test]
    fn test_universal_evaluation_gamma() {
        // gamma(5) = 24 through direct dispatch
        let result = simplify_function("gamma", &[expr!(5)]);
        assert_eq!(result, expr!(24));

        // gamma(1) = 1
        let result = simplify_function("gamma", &[expr!(1)]);
        assert_eq!(result, expr!(1));
    }

    #[test]
    fn test_universal_evaluation_preserves_symbolic() {
        // gamma(x) should stay symbolic
        let result = simplify_function("gamma", &[expr!(x)]);

        // Result should be a Function expression (unevaluated)
        assert!(matches!(result, Expression::Function { .. }));
    }

    #[test]
    fn test_exp_log_identity() {
        let x = symbol!(x);

        let result = simplify_function(
            "exp",
            &[Expression::function(
                "log",
                vec![Expression::symbol(x.clone())],
            )],
        );

        assert_eq!(result, Expression::symbol(x));
    }

    #[test]
    fn test_log_exp_identity() {
        let x = symbol!(x);

        // log(exp(x)) should simplify to x
        let result = simplify_function(
            "log",
            &[Expression::function(
                "exp",
                vec![Expression::symbol(x.clone())],
            )],
        );

        assert_eq!(result, Expression::symbol(x));
    }

    #[test]
    fn test_exp_ln_identity() {
        let x = symbol!(x);

        let result = simplify_function(
            "exp",
            &[Expression::function(
                "ln",
                vec![Expression::symbol(x.clone())],
            )],
        );

        assert_eq!(result, Expression::symbol(x));
    }

    #[test]
    fn test_ln_exp_identity() {
        let x = symbol!(x);

        // ln(exp(x)) should simplify to x
        let result = simplify_function(
            "ln",
            &[Expression::function(
                "exp",
                vec![Expression::symbol(x.clone())],
            )],
        );

        assert_eq!(result, Expression::symbol(x));
    }

    #[test]
    fn test_function_composition_stays_symbolic() {
        // sin(cos(0)) should evaluate cos(0)=1 but keep sin(1) symbolic
        let result = simplify_function("sin", &[simplify_function("cos", &[expr!(0)])]);

        // Should be sin(1) (symbolic), not a float
        match result {
            Expression::Function { name, args } => {
                assert_eq!(name.as_ref(), "sin");
                assert_eq!(args.len(), 1);
                assert_eq!(args[0], expr!(1));
            }
            _ => panic!("Expected Function(sin, [1]), got {:?}", result),
        }
    }
}
