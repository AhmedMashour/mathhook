//! Matrix construction macros
//!
//! Memory-optimized matrix construction with specialized types.
//! These macros provide ergonomic creation of various matrix types
//! while maintaining optimal memory usage and performance.

/// Memory-optimized matrix construction
///
/// This macro provides ergonomic matrix creation with automatic
/// optimization to specialized matrix types when possible.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::matrix;
/// use mathhook_core::{Expression, matrix::Matrix};
///
/// // Dense matrix from nested arrays
/// let dense = matrix!(dense: [
///     [expr!(1), expr!(2)],
///     [expr!(3), expr!(4)]
/// ]);
///
/// // Identity matrix
/// let identity = matrix!(identity: 3);
/// assert_eq!(identity.dimensions(), (3, 3));
///
/// // Zero matrix
/// let zero = matrix!(zero: 2, 3);
/// assert_eq!(zero.dimensions(), (2, 3));
///
/// // Diagonal matrix
/// let diagonal = matrix!(diagonal: [expr!(1), expr!(2), expr!(3)]);
/// assert_eq!(diagonal.dimensions(), (3, 3));
///
/// // Scalar matrix (c*I)
/// let scalar = matrix!(scalar: 3, expr!(5));
/// assert_eq!(scalar.dimensions(), (3, 3));
/// ```
#[macro_export]
macro_rules! matrix {
    // Dense matrix from nested arrays
    (dense: [$([$($elem:expr),* $(,)?]),* $(,)?]) => {
        $crate::matrix::Matrix::dense(vec![$(vec![$($elem),*]),*])
    };

    // Identity matrix - O(1) memory
    (identity: $size:expr) => {
        $crate::matrix::Matrix::identity($size)
    };

    // Zero matrix - O(1) memory
    (zero: $rows:expr, $cols:expr) => {
        $crate::matrix::Matrix::zero($rows, $cols)
    };

    // Diagonal matrix - O(n) memory
    (diagonal: [$($elem:expr),* $(,)?]) => {
        $crate::matrix::Matrix::diagonal(vec![$($elem),*])
    };

    // Scalar matrix - O(1) memory
    (scalar: $size:expr, $value:expr) => {
        $crate::matrix::Matrix::scalar($size, $value)
    };

    // Upper triangular matrix - O(n²/2) memory
    (upper_triangular: $size:expr, [$($elem:expr),* $(,)?]) => {
        $crate::matrix::Matrix::upper_triangular($size, vec![$($elem),*])
    };

    // Lower triangular matrix - O(n²/2) memory
    (lower_triangular: $size:expr, [$($elem:expr),* $(,)?]) => {
        $crate::matrix::Matrix::lower_triangular($size, vec![$($elem),*])
    };

    // Symmetric matrix - O(n²/2) memory
    (symmetric: $size:expr, [$($elem:expr),* $(,)?]) => {
        $crate::matrix::Matrix::symmetric($size, vec![$($elem),*])
    };

    // Permutation matrix - O(n) memory
    (permutation: [$($perm:expr),* $(,)?]) => {
        $crate::matrix::Matrix::permutation(vec![$($perm),*])
    };

    // From flat vector (row-major order)
    (from_flat: $rows:expr, $cols:expr, [$($elem:expr),* $(,)?]) => {
        $crate::matrix::Matrix::from_flat($rows, $cols, vec![$($elem),*])
    };

    // From arrays (compile-time size)
    (from_arrays: $arrays:expr) => {
        $crate::matrix::Matrix::from_arrays($arrays)
    };
}

#[cfg(test)]
mod tests {
    use crate::{matrix::Matrix, Expression};

    #[test]
    fn test_matrix_identity() {
        let identity = matrix!(identity: 3);
        assert_eq!(identity.dimensions(), (3, 3));
        assert!(identity.is_identity());
    }

    #[test]
    fn test_matrix_zero() {
        let zero = matrix!(zero: 2, 3);
        assert_eq!(zero.dimensions(), (2, 3));
        assert!(zero.is_zero());
    }

    #[test]
    fn test_matrix_diagonal() {
        let diagonal = matrix!(diagonal: [
            Expression::integer(1),
            Expression::integer(2),
            Expression::integer(3)
        ]);
        assert_eq!(diagonal.dimensions(), (3, 3));
        assert!(diagonal.is_diagonal());
    }

    #[test]
    fn test_matrix_scalar() {
        let scalar = matrix!(scalar: 3, Expression::integer(5));
        assert_eq!(scalar.dimensions(), (3, 3));
    }

    #[test]
    fn test_matrix_dense() {
        let dense = matrix!(dense: [
            [Expression::integer(1), Expression::integer(2)],
            [Expression::integer(3), Expression::integer(4)]
        ]);
        assert_eq!(dense.dimensions(), (2, 2));
    }

    #[test]
    fn test_matrix_permutation() {
        let perm = matrix!(permutation: [2, 0, 1]);
        assert_eq!(perm.dimensions(), (3, 3));
    }

    #[test]
    fn test_matrix_from_flat() {
        let flat = matrix!(from_flat: 2, 2, [
            Expression::integer(1),
            Expression::integer(2),
            Expression::integer(3),
            Expression::integer(4)
        ]);
        assert_eq!(flat.dimensions(), (2, 2));
    }
}
