//! Trigonometric Function Simplification Strategies
//!
//! Implements algebraic rewrite rules for trigonometric functions (sin, cos, tan, etc.).

use super::strategy::SimplificationStrategy;
use crate::core::Expression;

/// Sine function simplification strategy
pub struct SinSimplificationStrategy;

impl SimplificationStrategy for SinSimplificationStrategy {
    fn simplify(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            let arg = &args[0];

            if arg.is_zero() {
                Expression::integer(0)
            } else {
                Expression::function("sin", args.to_vec())
            }
        } else {
            Expression::function("sin", args.to_vec())
        }
    }

    fn applies_to(&self, args: &[Expression]) -> bool {
        args.len() == 1
    }

    fn name(&self) -> &str {
        "SinSimplificationStrategy"
    }
}

/// Cosine function simplification strategy
pub struct CosSimplificationStrategy;

impl SimplificationStrategy for CosSimplificationStrategy {
    fn simplify(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            let arg = &args[0];

            if arg.is_zero() {
                Expression::integer(1)
            } else {
                Expression::function("cos", args.to_vec())
            }
        } else {
            Expression::function("cos", args.to_vec())
        }
    }

    fn applies_to(&self, args: &[Expression]) -> bool {
        args.len() == 1
    }

    fn name(&self) -> &str {
        "CosSimplificationStrategy"
    }
}

/// Tangent function simplification strategy
pub struct TanSimplificationStrategy;

impl SimplificationStrategy for TanSimplificationStrategy {
    fn simplify(&self, args: &[Expression]) -> Expression {
        if args.len() == 1 {
            let arg = &args[0];

            if arg.is_zero() {
                Expression::integer(0)
            } else {
                Expression::function("tan", args.to_vec())
            }
        } else {
            Expression::function("tan", args.to_vec())
        }
    }

    fn applies_to(&self, args: &[Expression]) -> bool {
        args.len() == 1
    }

    fn name(&self) -> &str {
        "TanSimplificationStrategy"
    }
}

/// Generic trigonometric simplification strategy
///
/// Handles csc, sec, cot, asin, acos, atan, sinh, cosh, tanh
pub struct GenericTrigSimplificationStrategy {
    function_name: String,
}

impl GenericTrigSimplificationStrategy {
    pub fn new(function_name: &str) -> Self {
        Self {
            function_name: function_name.to_owned(),
        }
    }
}

impl SimplificationStrategy for GenericTrigSimplificationStrategy {
    fn simplify(&self, args: &[Expression]) -> Expression {
        Expression::function(&self.function_name, args.to_vec())
    }

    fn applies_to(&self, args: &[Expression]) -> bool {
        args.len() == 1
    }

    fn name(&self) -> &str {
        &self.function_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr, symbol};

    #[test]
    fn test_sin_of_zero() {
        let strategy = SinSimplificationStrategy;
        let result = strategy.simplify(&[expr!(0)]);
        assert_eq!(result, expr!(0));
    }

    #[test]
    fn test_sin_of_x() {
        let strategy = SinSimplificationStrategy;
        let x = symbol!(x);
        let result = strategy.simplify(&[x.clone().into()]);

        if let Expression::Function { name, args } = result {
            assert_eq!(name, "sin");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], x.into());
        } else {
            panic!("Expected function call");
        }
    }

    #[test]
    fn test_cos_of_zero() {
        let strategy = CosSimplificationStrategy;
        let result = strategy.simplify(&[expr!(0)]);
        assert_eq!(result, expr!(1));
    }

    #[test]
    fn test_cos_of_x() {
        let strategy = CosSimplificationStrategy;
        let x = symbol!(x);
        let result = strategy.simplify(&[x.clone().into()]);

        if let Expression::Function { name, args } = result {
            assert_eq!(name, "cos");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], x.into());
        } else {
            panic!("Expected function call");
        }
    }

    #[test]
    fn test_tan_of_zero() {
        let strategy = TanSimplificationStrategy;
        let result = strategy.simplify(&[expr!(0)]);
        assert_eq!(result, expr!(0));
    }

    #[test]
    fn test_tan_of_x() {
        let strategy = TanSimplificationStrategy;
        let x = symbol!(x);
        let result = strategy.simplify(&[x.clone().into()]);

        if let Expression::Function { name, args } = result {
            assert_eq!(name, "tan");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], x.into());
        } else {
            panic!("Expected function call");
        }
    }

    #[test]
    fn test_generic_trig() {
        let strategy = GenericTrigSimplificationStrategy::new("sinh");
        let x = symbol!(x);
        let result = strategy.simplify(&[x.clone().into()]);

        if let Expression::Function { name, args } = result {
            assert_eq!(name, "sinh");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], x.into());
        } else {
            panic!("Expected function call");
        }
    }
}
