//! MathHook Book Test Library
//!
//! This is a minimal library that re-exports mathhook and mathhook-core
//! to make them available to mdbook's doctests.
//!
//! By having this lib.rs, all code examples in the book can use:
//! - `extern crate mathhook_book;`
//! - `use mathhook_core::prelude::*;` (because we have it as a dependency)
//! - `use mathhook::prelude::*;` (because we have it as a dependency)

// Re-export for convenience
pub use mathhook;
pub use mathhook_core;

// Also re-export preludes directly
pub mod prelude {
    pub use mathhook::prelude::*;
    pub use mathhook_core::prelude::*;
}
