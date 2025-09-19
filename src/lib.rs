//! MathHook - High-Performance Educational Computer Algebra System
//!
//! A Rust-based computer algebra system focused on educational features
//! and competitive performance with systems like Symbolica.

pub mod algebra;
pub mod api;
pub mod core;
pub mod educational;
pub mod macros;
pub mod parsing;

// Re-export macros at crate root for ergonomic usage
pub use macros::*;

// Re-export the prelude for easy access
pub mod prelude {
    pub use crate::algebra::*;
    pub use crate::api::*;
    pub use crate::core::*;
    pub use crate::educational::*;
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_basic_functionality() {
        let x = Symbol::new("x");
        let expr = Expression::symbol(x) + Expression::integer(1);
        let simplified = expr.simplify();

        // Basic test to ensure the system is working
        assert!(matches!(simplified, Expression::Add(_)));
    }
}
