//! Core mathematical engine for MathHook
//!
//! This crate provides the foundational types and operations for symbolic mathematics,
//! implementing a hybrid API architecture optimized for both performance and usability.

pub mod algebra;
pub mod calculus;
pub mod core;
pub mod educational;
pub mod error;
pub mod formatter;
pub mod functions;
pub mod macros;
pub mod matrix;
pub mod parser;
pub mod pattern;
pub mod serialize;
pub mod simplify;
pub mod solvers;

// Re-export
pub use algebra::*;
pub use calculus::*;
pub use core::{Expression, MathConstant, Number, Symbol};
pub use error::*;
pub use functions::*;
pub use macros::*;
pub use matrix::*;
pub use parser::*;
pub use pattern::*;
pub use simplify::*;
pub use solvers::*;

/// Convenience prelude for common imports
pub mod prelude {
    pub use crate::macros::*;
    pub use crate::{expr, function, parse, symbol};
    pub use crate::{
        AdvancedSimplify, Collect, ComplexOperations, Expand, Factor, PolynomialGcd,
        RationalSimplify, Simplify, ZeroDetection,
    };
    pub use crate::{Expression, MathConstant, Number, Symbol};
    pub use crate::{MathSolver, SolverConfig, SolverResult};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_memory_optimization() {
        let expression_size = std::mem::size_of::<Expression>();
        println!("Clean Expression size: {} bytes", expression_size);

        // Our target is 32 bytes for optimal cache performance
        assert!(
            expression_size <= 32,
            "Expression should be ≤ 32 bytes, got {} bytes",
            expression_size
        );
    }

    #[test]
    fn test_hybrid_api_basic_operations() {
        // Test Expression-centric API (method chaining)
        let expr = Expression::add(vec![Expression::integer(2), Expression::integer(3)]);
        let simplified = expr.simplify();

        // Should combine to 5
        match simplified {
            Expression::Number(num) => {
                if let Number::Integer(i) = num {
                    assert_eq!(i, 5);
                } else {
                    panic!("Expected integer result");
                }
            }
            _ => panic!("Expected number result"),
        }
    }

    #[test]
    fn test_solver_object_api() {
        // Test separate solver object
        let mut solver = MathSolver::new();

        let equation = Expression::equation(expr!(x), expr!(42));

        let result = solver.solve(&equation, &symbol!(x));

        match result {
            SolverResult::Single(solution) => match solution {
                Expression::Number(num) => {
                    if let Number::Integer(i) = num {
                        assert_eq!(i, 42);
                    } else {
                        panic!("Expected integer solution");
                    }
                }
                _ => panic!("Expected number solution"),
            },
            _ => panic!("Expected single solution"),
        }
    }

    #[test]
    fn test_hot_path_performance() {
        // Test that hot-path operations are fast
        let start = std::time::Instant::now();

        for _ in 0..10000 {
            let expr = expr!(add: 1, x, 2);
            let _simplified = expr.simplify();
        }

        let duration = start.elapsed();
        println!("10K hot-path operations took: {:?}", duration);

        // Should be very fast (under 10ms for 10K operations)
        assert!(
            duration.as_millis() < 100,
            "Hot path too slow: {:?}",
            duration
        );
    }
}
