//! Validation and error handling macros
//!
//! Comprehensive validation patterns for mathematical expressions,
//! matrices, and operations. These macros provide consistent error
//! handling and validation across the entire mathematical system.

/// Validation and error handling
///
/// This macro provides comprehensive validation patterns for mathematical
/// expressions, matrices, and operations with consistent error messages.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::validate;
/// use mathhook_core::{Expression, matrix::Matrix};
///
/// fn validate_expression(expr: &Expression) -> Result<(), String> {
///     validate!(expr: expr);
///     Ok(())
/// }
///
/// fn validate_square_matrix(matrix: &Matrix) -> Result<(), String> {
///     validate!(square_matrix: matrix);
///     Ok(())
/// }
///
/// fn validate_range(value: i32) -> Result<(), String> {
///     validate!(bounds: value, 0, 100);
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! validate {
    // Expression validation
    (expr: $expr:expr) => {
        if !$expr.is_valid_expression() {
            return Err(format!("Invalid expression: {}", $expr));
        }
    };

    // Matrix dimension validation
    (matrix_dims: $matrix:expr, $expected_rows:expr, $expected_cols:expr) => {
        let (rows, cols) = $matrix.dimensions();
        if rows != $expected_rows || cols != $expected_cols {
            return Err(format!(
                "Matrix dimension mismatch: expected {}x{}, got {}x{}",
                $expected_rows, $expected_cols, rows, cols
            ));
        }
    };

    // Square matrix validation
    (square_matrix: $matrix:expr) => {
        let (rows, cols) = $matrix.dimensions();
        if rows != cols {
            return Err(format!("Matrix must be square: got {}x{}", rows, cols));
        }
    };

    // Non-zero validation
    (non_zero: $expr:expr) => {
        if $expr.is_zero() {
            return Err("Expression cannot be zero".to_string());
        }
    };

    // Non-empty validation
    (non_empty: $collection:expr, $name:literal) => {
        if $collection.is_empty() {
            return Err(format!("{} cannot be empty", $name));
        }
    };

    // Symbol validation
    (symbol: $sym:expr) => {
        if $sym.name.is_empty() {
            return Err("Symbol name cannot be empty".to_string());
        }
    };

    // Bounds checking
    (bounds: $value:expr, $min:expr, $max:expr) => {
        if $value < $min || $value > $max {
            return Err(format!(
                "Value {} out of bounds [{}, {}]",
                $value, $min, $max
            ));
        }
    };

    // Type checking
    (type: $expr:expr, $expected_type:pat) => {
        match $expr {
            $expected_type => {}
            _ => {
                return Err(format!(
                    "Type mismatch: expected {}",
                    stringify!($expected_type)
                ))
            }
        }
    };

    // Positive number validation
    (positive: $value:expr) => {
        if $value <= 0 {
            return Err(format!("Value must be positive, got {}", $value));
        }
    };

    // Non-negative validation
    (non_negative: $value:expr) => {
        if $value < 0 {
            return Err(format!("Value must be non-negative, got {}", $value));
        }
    };

    // Matrix compatibility for operations
    (matrix_add_compatible: $a:expr, $b:expr) => {
        let (a_rows, a_cols) = $a.dimensions();
        let (b_rows, b_cols) = $b.dimensions();
        if a_rows != b_rows || a_cols != b_cols {
            return Err(format!(
                "Matrices not compatible for addition: {}x{} and {}x{}",
                a_rows, a_cols, b_rows, b_cols
            ));
        }
    };

    // Matrix multiplication compatibility
    (matrix_mul_compatible: $a:expr, $b:expr) => {
        let (a_rows, a_cols) = $a.dimensions();
        let (b_rows, b_cols) = $b.dimensions();
        if a_cols != b_rows {
            return Err(format!(
                "Matrices not compatible for multiplication: {}x{} and {}x{}",
                a_rows, a_cols, b_rows, b_cols
            ));
        }
    };

    // Vector length validation
    (vector_length: $vector:expr, $expected_length:expr) => {
        if $vector.len() != $expected_length {
            return Err(format!(
                "Vector length mismatch: expected {}, got {}",
                $expected_length,
                $vector.len()
            ));
        }
    };

    // Finite number validation
    (finite: $value:expr) => {
        if !$value.is_finite() {
            return Err(format!("Value must be finite, got {}", $value));
        }
    };

    // Range validation (inclusive)
    (range_inclusive: $value:expr, $min:expr, $max:expr) => {
        if $value < $min || $value > $max {
            return Err(format!(
                "Value {} not in range [{}, {}] (inclusive)",
                $value, $min, $max
            ));
        }
    };

    // Range validation (exclusive)
    (range_exclusive: $value:expr, $min:expr, $max:expr) => {
        if $value <= $min || $value >= $max {
            return Err(format!(
                "Value {} not in range ({}, {}) (exclusive)",
                $value, $min, $max
            ));
        }
    };
}

/// Error handling patterns
///
/// This macro provides common error handling patterns with consistent
/// error formatting and context information.
///
/// # Examples
///
/// ```rust
/// use mathhook_core::handle;
///
/// fn parse_number(s: &str) -> Result<i32, String> {
///     let result = s.parse::<i32>();
///     Ok(handle!(unwrap: result, "Failed to parse number"))
/// }
///
/// fn get_first_element<T>(vec: Vec<T>) -> Result<T, String> {
///     Ok(handle!(unwrap_option: vec.into_iter().next(), "Vector is empty"))
/// }
/// ```
#[macro_export]
macro_rules! handle {
    // Result unwrapping with context
    (unwrap: $result:expr, $context:literal) => {
        match $result {
            Ok(val) => val,
            Err(e) => return Err(format!("{}: {}", $context, e)),
        }
    };

    // Option unwrapping with context
    (unwrap_option: $option:expr, $context:literal) => {
        match $option {
            Some(val) => val,
            None => return Err($context.to_string()),
        }
    };

    // Try with fallback
    (try_or: $primary:expr, $fallback:expr) => {
        match $primary {
            Ok(val) => val,
            Err(_) => $fallback,
        }
    };

    // Parser error formatting
    (parse_error: $input:expr, $error:expr) => {
        $crate::ParseError::InvalidSyntax(format!("Failed to parse '{}': {}", $input, $error))
    };

    // Solver error formatting
    (solver_error: $equation:expr, $variable:expr, $error:expr) => {
        $crate::algebra::solvers::SolverError::InvalidEquation(format!(
            "Cannot solve {} for {}: {}",
            $equation, $variable, $error
        ))
    };

    // Mathematical error formatting
    (math_error: $operation:literal, $error:expr) => {
        format!("Mathematical error in {}: {}", $operation, $error)
    };

    // IO error formatting
    (io_error: $operation:literal, $error:expr) => {
        format!("IO error during {}: {}", $operation, $error)
    };

    // Conversion error formatting
    (conversion_error: $from_type:literal, $to_type:literal, $error:expr) => {
        format!(
            "Failed to convert from {} to {}: {}",
            $from_type, $to_type, $error
        )
    };

    // Timeout error
    (timeout_error: $operation:literal, $timeout:expr) => {
        format!("Operation '{}' timed out after {:?}", $operation, $timeout)
    };

    // Memory error
    (memory_error: $operation:literal) => {
        format!("Out of memory during operation: {}", $operation)
    };

    // Network error formatting
    (network_error: $operation:literal, $error:expr) => {
        format!("Network error during {}: {}", $operation, $error)
    };
}

#[cfg(test)]
mod tests {
    use crate::{matrix::Matrix, Expression, Symbol};

    #[test]
    fn test_validate_expr_valid() {
        fn validate_expression(expr: &Expression) -> Result<(), String> {
            validate!(expr: expr);
            Ok(())
        }

        let expr = Expression::integer(42);
        assert!(validate_expression(&expr).is_ok());
    }

    #[test]
    fn test_validate_bounds_valid() {
        fn validate_range(value: i32) -> Result<(), String> {
            validate!(bounds: value, 0, 100);
            Ok(())
        }

        assert!(validate_range(50).is_ok());
    }

    #[test]
    fn test_validate_bounds_invalid() {
        fn validate_range(value: i32) -> Result<(), String> {
            validate!(bounds: value, 0, 100);
            Ok(())
        }

        assert!(validate_range(150).is_err());
        assert!(validate_range(-10).is_err());
    }

    #[test]
    fn test_validate_positive_valid() {
        fn validate_positive(value: i32) -> Result<(), String> {
            validate!(positive: value);
            Ok(())
        }

        assert!(validate_positive(1).is_ok());
        assert!(validate_positive(100).is_ok());
    }

    #[test]
    fn test_validate_positive_invalid() {
        fn validate_positive(value: i32) -> Result<(), String> {
            validate!(positive: value);
            Ok(())
        }

        assert!(validate_positive(0).is_err());
        assert!(validate_positive(-1).is_err());
    }

    #[test]
    fn test_validate_non_negative_valid() {
        fn validate_non_negative(value: i32) -> Result<(), String> {
            validate!(non_negative: value);
            Ok(())
        }

        assert!(validate_non_negative(0).is_ok());
        assert!(validate_non_negative(1).is_ok());
    }

    #[test]
    fn test_validate_non_negative_invalid() {
        fn validate_non_negative(value: i32) -> Result<(), String> {
            validate!(non_negative: value);
            Ok(())
        }

        assert!(validate_non_negative(-1).is_err());
    }

    #[test]
    fn test_validate_non_empty_valid() {
        fn validate_non_empty(vec: &Vec<i32>) -> Result<(), String> {
            validate!(non_empty: vec, "vector");
            Ok(())
        }

        let vec = vec![1, 2, 3];
        assert!(validate_non_empty(&vec).is_ok());
    }

    #[test]
    fn test_validate_non_empty_invalid() {
        fn validate_non_empty(vec: &Vec<i32>) -> Result<(), String> {
            validate!(non_empty: vec, "vector");
            Ok(())
        }

        let vec: Vec<i32> = vec![];
        assert!(validate_non_empty(&vec).is_err());
    }

    #[test]
    fn test_handle_unwrap_ok() {
        fn test_unwrap() -> Result<i32, String> {
            let result: Result<i32, &str> = Ok(42);
            Ok(handle!(unwrap: result, "Test context"))
        }

        assert_eq!(test_unwrap().unwrap(), 42);
    }

    #[test]
    fn test_handle_unwrap_err() {
        fn test_unwrap() -> Result<i32, String> {
            let result: Result<i32, &str> = Err("test error");
            Ok(handle!(unwrap: result, "Test context"))
        }

        let err = test_unwrap().unwrap_err();
        assert!(err.contains("Test context"));
        assert!(err.contains("test error"));
    }

    #[test]
    fn test_handle_unwrap_option_some() {
        fn test_unwrap_option() -> Result<i32, String> {
            let option = Some(42);
            Ok(handle!(unwrap_option: option, "Option is None"))
        }

        assert_eq!(test_unwrap_option().unwrap(), 42);
    }

    #[test]
    fn test_handle_unwrap_option_none() {
        fn test_unwrap_option() -> Result<i32, String> {
            let option: Option<i32> = None;
            Ok(handle!(unwrap_option: option, "Option is None"))
        }

        let err = test_unwrap_option().unwrap_err();
        assert_eq!(err, "Option is None");
    }

    #[test]
    fn test_handle_try_or_ok() {
        let primary: Result<i32, &str> = Ok(42);
        let result = handle!(try_or: primary, 0);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_handle_try_or_err() {
        let primary: Result<i32, &str> = Err("error");
        let result = handle!(try_or: primary, 0);
        assert_eq!(result, 0);
    }
}
