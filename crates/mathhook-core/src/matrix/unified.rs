//! Unified matrix system with zero-cost abstractions
//!
//! This module provides a single `Matrix` type that can represent all special
//! matrix types while maintaining optimal memory usage and performance.

use crate::matrix::types::*;
use serde::{Deserialize, Serialize};

mod construction;
mod operations;
mod decomposition;

pub use operations::CoreMatrixOps;

/// Unified matrix type that can represent any matrix efficiently
///
/// This enum uses zero-cost abstractions to provide a single interface
/// for all matrix types while maintaining optimal memory usage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Matrix {
    /// Regular dense matrix: O(n²) memory
    Dense(MatrixData),

    /// Identity matrix: O(1) memory
    Identity(IdentityMatrixData),

    /// Zero matrix: O(1) memory
    Zero(ZeroMatrixData),

    /// Diagonal matrix: O(n) memory
    Diagonal(DiagonalMatrixData),

    /// Scalar matrix: O(1) memory
    Scalar(ScalarMatrixData),

    /// Upper triangular: O(n²/2) memory
    UpperTriangular(UpperTriangularMatrixData),

    /// Lower triangular: O(n²/2) memory
    LowerTriangular(LowerTriangularMatrixData),

    /// Symmetric matrix: O(n²/2) memory
    Symmetric(SymmetricMatrixData),

    /// Permutation matrix: O(n) memory
    Permutation(PermutationMatrixData),
}
