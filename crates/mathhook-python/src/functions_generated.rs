//! Macro-generated Python function bindings
//!
//! This module demonstrates all macro variants for different function arities:
//! - Unary: sin, cos, tan
//! - Binary: pow_macro, atan2_macro
//! - Variadic: add_macro, mul_macro
//! - Constants: pi, e
//!
//! These are used for testing and validation before full migration.

// Unary functions (original prototype)
mathhook_macros::generate_python_binding!(sin_macro_generated);
mathhook_macros::generate_python_binding!(cos_macro_generated);
mathhook_macros::generate_python_binding!(tan_macro_generated);

// Binary functions (Phase 1 extension)
mathhook_macros::generate_python_binary_binding!(pow_macro);
mathhook_macros::generate_python_binary_binding!(atan2_macro);

// Variadic functions (Phase 1 extension)
mathhook_macros::generate_python_variadic_binding!(add_macro);
mathhook_macros::generate_python_variadic_binding!(mul_macro);

// Constant functions (Phase 1 extension)
mathhook_macros::generate_python_constant_binding!(pi);
mathhook_macros::generate_python_constant_binding!(e);
