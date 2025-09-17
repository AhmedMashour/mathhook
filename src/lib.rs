//! MathHook - High-Performance Educational Computer Algebra System
//! 
//! A Rust-based computer algebra system focused on educational features
//! and competitive performance with systems like Symbolica.

pub mod core;
pub mod algebra;

// Re-export the prelude for easy access
pub mod prelude {
    pub use crate::core::*;
    pub use crate::algebra::*;
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