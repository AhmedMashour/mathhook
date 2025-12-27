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
//! let x = symbol!(x);
//! let equation = Expression::equation(
//!     Expression::symbol("x"),
//!     Expression::integer(5),
//! );
//! let result = solver.solve(&equation, &x);
//!
//! // Parser API (multi-format support)
//! let parser = Parser::new(&ParserConfig::default());
//! let parsed = parser.parse("x^2 + 2*x + 1")?;
//! # Ok::<(), ParseError>(())
//! ```
pub use mathhook_core as core;
pub use mathhook_core::{Expression, MathSolver, Number, SolverResult, Symbol};
pub use mathhook_core::{MathError, ParseError};
/// Convenience prelude for common imports
///
/// This is the recommended way to use MathHook. Import everything you need with:
/// ```rust
/// use mathhook::prelude::*;
/// ```
///
/// # Examples
///
/// ```rust
/// use mathhook::prelude::*;
///
/// // Macro-first philosophy
/// let x = symbol!(x);
/// let expr = expr!(x ^ 2);  // x squared
/// let simplified = expr.simplify();
///
/// // Solver API
/// let mut solver = MathSolver::new();
/// let equation = Expression::equation(Expression::symbol(x.clone()), expr!(5));
/// let solutions = solver.solve(&equation, &x);
/// ```
pub mod prelude {
    pub use mathhook_core::prelude::*;
    pub use num_bigint;
    pub use num_rational;
}
#[cfg(test)]
mod tests {
    use super::prelude::*;
    #[test]
    fn test_hybrid_api_integration() {
        let expr = Expression::add(vec![Expression::integer(2), Expression::integer(3)]);
        let simplified = expr.simplify();
        match simplified {
            Expression::Number(Number::Integer(5)) => {}
            _ => panic!("Expected simplified result to be 5"),
        }
    }
    #[test]
    fn test_solver_integration() {
        let solver = MathSolver::new();
        let x = symbol!(x);
        let equation = Expression::equation(Expression::symbol("x"), Expression::integer(42));
        let result = solver.solve(&equation, &x);
        match result {
            SolverResult::Single(_) => {}
            _ => panic!("Expected single solution"),
        }
    }
    #[test]
    fn test_parser_integration() {
        let parser = Parser::new(&ParserConfig::default());
        let result = parser.parse("42");
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
