//! Generated function bindings using mathhook-macros for Node.js
//!
//! This module demonstrates code generation for Node.js bindings, eliminating
//! duplication while maintaining zero-cost abstractions.

// Generate Node.js bindings for trig functions using the macro
mathhook_macros::generate_nodejs_binding!(sin_macro_generated);
mathhook_macros::generate_nodejs_binding!(cos_macro_generated);
mathhook_macros::generate_nodejs_binding!(tan_macro_generated);

#[cfg(test)]
mod tests {
    #[test]
    fn test_macro_expansion_compiles() {
        // This test verifies that the macro expands to valid Rust code
        // The macros generate three Node.js functions:
        // - sin_macro_generated
        // - cos_macro_generated
        // - tan_macro_generated
        println!("✅ Macro-generated Node.js functions compiled successfully");
    }
}
