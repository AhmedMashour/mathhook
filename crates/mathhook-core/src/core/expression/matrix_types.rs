//! Matrix data types with memory-optimized storage
//!
//! This module contains all matrix-related data structures designed for
//! maximum memory efficiency and performance. Each special matrix type
//! stores only the minimum required data.

use super::Expression;
use serde::{Deserialize, Serialize};

/// Regular matrix data
/// Memory: 24 + n*m * 24+ bytes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatrixData {
    pub rows: Vec<Vec<Expression>>,
}

/// Identity matrix data
/// Memory: 8 bytes (vs n² * 24+ bytes for regular matrix)
///
/// Represents an n×n matrix with 1s on the diagonal and 0s elsewhere.
/// Extremely memory efficient for large identity matrices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IdentityMatrixData {
    pub size: usize,
}

/// Zero matrix data
/// Memory: 16 bytes (vs n*m * 24+ bytes for regular matrix)
///
/// Represents an n×m matrix with all elements equal to zero.
/// Massive memory savings for large zero matrices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZeroMatrixData {
    pub rows: usize,
    pub cols: usize,
}

/// Diagonal matrix data
/// Memory: 24 + n * 24+ bytes (vs n² * 24+ bytes for regular matrix)
///
/// Stores only the diagonal elements. Off-diagonal elements are implicitly zero.
/// Memory savings: ~50% for typical cases, up to 99%+ for large sparse matrices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiagonalMatrixData {
    pub diagonal_elements: Vec<Expression>,
}

/// Scalar matrix data (diagonal with equal elements)
/// Memory: 32 bytes (vs n² * 24+ bytes for regular matrix)
///
/// Represents a diagonal matrix where all diagonal elements are equal.
/// Extremely efficient for matrices like 5*I, -2*I, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScalarMatrixData {
    pub size: usize,
    pub scalar_value: Expression,
}

/// Upper triangular matrix data
/// Memory: 8 + (n*(n+1)/2) * 24+ bytes (vs n² * 24+ bytes)
///
/// Stores only the upper triangle elements (including diagonal).
/// Memory savings: ~50% for triangular matrices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpperTriangularMatrixData {
    pub size: usize,
    /// Elements stored row by row, upper triangle only
    /// Index mapping: element[i][j] -> elements[i*(2*n-i-1)/2 + (j-i)]
    pub elements: Vec<Expression>,
}

/// Lower triangular matrix data
/// Memory: 8 + (n*(n+1)/2) * 24+ bytes (vs n² * 24+ bytes)
///
/// Stores only the lower triangle elements (including diagonal).
/// Memory savings: ~50% for triangular matrices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LowerTriangularMatrixData {
    pub size: usize,
    /// Elements stored row by row, lower triangle only
    /// Index mapping: element[i][j] -> elements[i*(i+1)/2 + j]
    pub elements: Vec<Expression>,
}

/// Symmetric matrix data
/// Memory: 8 + (n*(n+1)/2) * 24+ bytes (vs n² * 24+ bytes)
///
/// Stores only the upper triangle since A[i][j] = A[j][i].
/// Memory savings: ~50% for symmetric matrices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymmetricMatrixData {
    pub size: usize,
    /// Elements stored as upper triangle only (A[i][j] = A[j][i])
    /// Index mapping: element[i][j] -> elements[min(i,j)*(2*n-min(i,j)-1)/2 + abs(i-j)]
    pub elements: Vec<Expression>,
}

/// Permutation matrix data
/// Memory: 8 + n * 8 bytes (vs n² * 24+ bytes for regular matrix)
///
/// Represents a matrix with exactly one 1 in each row and column.
/// Massive memory savings: from O(n²) to O(n).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermutationMatrixData {
    /// Permutation vector: permutation[i] = j means row i has 1 in column j
    /// All other elements are implicitly 0
    pub permutation: Vec<usize>,
}

impl UpperTriangularMatrixData {
    /// Get element at position (i, j)
    /// Returns zero for lower triangle elements
    pub fn get_element(&self, i: usize, j: usize) -> Option<&Expression> {
        if i > j || i >= self.size || j >= self.size {
            None // Lower triangle or out of bounds
        } else {
            let index = i * (2 * self.size - i - 1) / 2 + (j - i);
            self.elements.get(index)
        }
    }
}

impl LowerTriangularMatrixData {
    /// Get element at position (i, j)
    /// Returns zero for upper triangle elements
    pub fn get_element(&self, i: usize, j: usize) -> Option<&Expression> {
        if i < j || i >= self.size || j >= self.size {
            None // Upper triangle or out of bounds
        } else {
            let index = i * (i + 1) / 2 + j;
            self.elements.get(index)
        }
    }
}

impl SymmetricMatrixData {
    /// Get element at position (i, j)
    /// Uses symmetry property: A[i][j] = A[j][i]
    pub fn get_element(&self, i: usize, j: usize) -> Option<&Expression> {
        if i >= self.size || j >= self.size {
            None // Out of bounds
        } else {
            let (row, col) = if i <= j { (i, j) } else { (j, i) };
            let index = row * (2 * self.size - row - 1) / 2 + (col - row);
            self.elements.get(index)
        }
    }
}

impl PermutationMatrixData {
    /// Get element at position (i, j)
    /// Returns 1 if this is the permuted position, 0 otherwise
    pub fn get_element(&self, i: usize, j: usize) -> i64 {
        if i >= self.permutation.len() {
            0 // Out of bounds
        } else if self.permutation[i] == j {
            1 // This is the permuted position
        } else {
            0 // All other positions are zero
        }
    }

    /// Check if this is a valid permutation
    pub fn is_valid(&self) -> bool {
        let n = self.permutation.len();
        let mut seen = vec![false; n];

        for &j in &self.permutation {
            if j >= n || seen[j] {
                return false; // Out of range or duplicate
            }
            seen[j] = true;
        }

        seen.iter().all(|&x| x) // All positions must be covered
    }
}
