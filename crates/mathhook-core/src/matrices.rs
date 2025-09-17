//! Matrix operations and linear algebra
//!
//! This module provides comprehensive matrix functionality including:
//! - Memory-optimized matrix types (identity, diagonal, sparse, etc.)
//! - Unified matrix interface with zero-cost abstractions
//! - Mathematical operations (add, multiply, transpose, inverse, etc.)
//! - Matrix decompositions (LU, QR, Cholesky, SVD)
//! - Eigenvalue and eigenvector computation
//! - Integration with the Expression system
//!
//! ## Module Structure
//!
//! - `types` - Matrix data structures and type definitions
//! - `unified` - Core Matrix enum and basic operations
//! - `operations` - High-level operations for Expression integration
//! - `decomposition` - Matrix decomposition algorithms (LU, QR, Cholesky, SVD)
//! - `eigenvalues` - Eigenvalue computation and matrix functions

pub mod decomposition;
pub mod eigenvalues;
pub mod operations;
pub mod types;
pub mod unified;

// Test modules

pub mod inverse_tests;

// Re-exports for clean API
pub use decomposition::MatrixDecomposition;
pub use eigenvalues::EigenOperations;
pub use operations::MatrixOperations;
pub use types::*;
pub use unified::{CoreMatrixOps, Matrix};
