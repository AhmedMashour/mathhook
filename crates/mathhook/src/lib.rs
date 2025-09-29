//! MathHook: High-performance educational computer algebra system
//!
//! Created by Ahmed Mashhour
//!
//! MathHook is a modern symbolic mathematics library written in Rust,
//! featuring a hybrid API architecture that combines Expression-centric
//! method chaining with separate solver objects for complex operations.
//!
//! # Features
//!
//! - **Memory-optimized**: 32-byte Expression enum for maximum cache performance
//! - **Hybrid API**: Expression methods + separate solver objects
//! - **Multi-format parsing**: LaTeX, Wolfram Language, standard notation
//! - **Language bindings**: Python (PyO3) and Node.js (NAPI-RS) support
//! - **Educational focus**: Step-by-step explanations and teaching tools
//!
//! # Quick Start
//!
//! ```rust
//! use mathhook::prelude::*;
//!
//! // Expression-centric API (method chaining)
//! let expr = Expression::add(vec![
//!     Expression::integer(2),
//!     Expression::integer(3),
//! ]).simplify();
//!
//! // Solver object API (stateful operations)
//! let mut solver = MathSolver::new();
//! let equation = Expression::equation(
//!     Expression::symbol("x"),
//!     Expression::integer(5),
//! );
//! let result = solver.solve(&equation, &Symbol::new("x"));
//!
//! // Parser API (multi-format support)
//! let parser = MathParser::new();
//! let parsed = parser.parse("x^2 + 2*x + 1", MathLanguage::Standard)?;
//! # Ok::<(), mathhook_parser::ParseError>(())
//! ```

pub use mathhook_core as core;

pub use mathhook_core::{
    algebra::{Expand, Factor},
    parser::universal::UniversalParser,
    Expression, MathConstant, MathSolver, Number, Simplify, SolverConfig, SolverResult, Symbol,
};

pub use core::{MathLanguage, ParseError};

pub use num_bigint;
pub use num_rational;
pub use serde_json;

/// Convenience prelude for common imports
///
/// # Examples
///
/// ```rust
/// use mathhook::prelude::*;
///
/// let expr = Expression::symbol("x").pow(Expression::integer(2));
/// let simplified = expr.simplify();
/// ```
pub mod prelude {
    pub use crate::core::parser::{MathLanguage, ParseError};
    pub use crate::core::{
        algebra::{Expand, Factor},
        Expression, MathConstant, Number, Simplify, Symbol,
    };
    pub use crate::core::{MathSolver, SolverConfig, SolverResult};
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_hybrid_api_integration() {
        let expr = Expression::add(vec![Expression::integer(2), Expression::integer(3)]);
        let simplified = expr.simplify();

        match simplified {
            Expression::Number(Number::Integer(5)) => (),
            _ => panic!("Expected simplified result to be 5"),
        }
    }

    #[test]
    fn test_solver_integration() {
        let mut solver = MathSolver::new();
        let equation = Expression::equation(Expression::symbol("x"), Expression::integer(42));
        let result = solver.solve(&equation, &Symbol::new("x"));

        match result {
            SolverResult::Single(_) => (),
            _ => panic!("Expected single solution"),
        }
    }

    #[test]
    fn test_parser_integration() {
        let parser = UniversalParser::new();
        let result = parser.parse("42", MathLanguage::Standard);
        assert!(result.is_ok());
    }

    #[test]
    fn test_memory_optimization_preserved() {
        let size = std::mem::size_of::<Expression>();
        assert!(
            size <= 32,
            "Expression size should be ≤ 32 bytes, got {} bytes",
            size
        );
    }
}
