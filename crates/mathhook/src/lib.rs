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
pub use mathhook_parser as parser;

pub use mathhook_core::{
    algebra::{Expand, Factor, Simplify},
    Expression, MathConstant, MathSolver, Number, SolverConfig, SolverResult, Symbol,
};

pub use mathhook_parser::{MathFormatter, MathLanguage, MathParser, ParseError};

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
    pub use crate::core::{
        algebra::{Expand, Factor, Simplify},
        Expression, MathConstant, Number, Symbol,
    };
    pub use crate::core::{MathSolver, SolverConfig, SolverResult};
    pub use crate::parser::{MathFormatter, MathLanguage, MathParser, ParseError};
}

/// Macro support for ergonomic expression creation
pub mod macros {
    /// Create mathematical expressions using natural syntax
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook::expr;
    ///
    /// let result = expr!(x^2 + 2*x + 1);
    /// ```
    #[macro_export]
    macro_rules! expr {
        ($var:ident) => {
            $crate::Expression::symbol(stringify!($var))
        };
        ($num:literal) => {
            $crate::Expression::integer($num)
        };
        ($left:tt + $right:tt) => {
            $crate::Expression::add(vec![expr!($left), expr!($right)])
        };
        ($left:tt * $right:tt) => {
            $crate::Expression::multiply(expr!($left), expr!($right))
        };
        ($base:tt ^ $exp:tt) => {
            $crate::Expression::pow(expr!($base), expr!($exp))
        };
    }

    /// Create symbols easily
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook::symbol;
    ///
    /// let x = symbol!(x);
    /// let alpha = symbol!(α);
    /// ```
    #[macro_export]
    macro_rules! symbol {
        ($name:ident) => {
            $crate::Symbol::new(stringify!($name))
        };
    }

    /// Parse expressions from strings
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mathhook::parse;
    ///
    /// let expr = parse!("x + 2", standard);
    /// ```
    #[macro_export]
    macro_rules! parse {
        ($input:expr, $lang:ident) => {{
            let parser = $crate::MathParser::new();
            parser.parse($input, $crate::MathLanguage::$lang)
        }};
    }
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
        let parser = MathParser::new();
        let result = parser.parse("42", MathLanguage::Standard);
        assert!(result.is_ok());
    }

    #[test]
    fn test_macro_support() {
        use crate::{expr, symbol};

        let x = symbol!(x);
        assert_eq!(x.name, "x");

        let two = expr!(2);
        match two {
            Expression::Number(Number::Integer(2)) => (),
            _ => panic!("Expected integer 2"),
        }
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
