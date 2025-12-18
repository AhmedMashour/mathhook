//! Integration tests for the mathhook_fn macro
//!
//! This module validates that the export macro system generates correct NAPI bindings.
//!
//! Test Coverage:
//! - Function export with various arities
//! - Option return types
//! - Vec parameters
//! - String parameters
//! - Boolean returns
//! - Float parameters
//!
//! These tests verify compilation only. Runtime tests are in Node.js test suite.
//!
//! Note: Expression parameter tests are in the main codebase with the actual
//! JsExpression wrapper implementation to avoid NAPI trait resolution issues.

use mathhook_macros::mathhook_fn;

// ============================================================================
// Test 1: Simple Function Export
// ============================================================================

/// Double an integer value
#[mathhook_fn(skip_python)]
pub fn test_double_fn(x: i64) -> i64 {
    x * 2
}

// ============================================================================
// Test 2: Function returning Option
// ============================================================================

/// Test optional return type
#[mathhook_fn(skip_python)]
pub fn test_optional_result(x: i64) -> Option<i64> {
    if x > 0 {
        Some(x * 2)
    } else {
        None
    }
}

// ============================================================================
// Test 3: Function with multiple parameters
// ============================================================================

/// Test multi-parameter function
#[mathhook_fn(skip_python)]
pub fn test_add_numbers(a: i64, b: i64, c: i64) -> i64 {
    a + b + c
}

// ============================================================================
// Test 4: Skip-bindings flags
// ============================================================================

/// This function should only generate Node.js bindings (skip Python)
#[mathhook_fn(skip_python)]
pub fn test_nodejs_only(x: i64) -> i64 {
    x + 1
}

// ============================================================================
// Test 5: Custom naming
// ============================================================================

/// Test function with custom Node.js name
#[mathhook_fn(nodejs_name = "custom_nodejs_name", skip_python)]
pub fn test_naming_fn(value: i64) -> i64 {
    value * 3
}

// ============================================================================
// Test 6: Function with Vec parameter
// ============================================================================

/// Sum all elements in a vector
#[mathhook_fn(skip_python)]
pub fn test_sum_vec(values: Vec<i64>) -> i64 {
    values.iter().sum()
}

// ============================================================================
// Test 7: Function with String parameter
// ============================================================================

/// Uppercase a string value
#[mathhook_fn(skip_python)]
pub fn test_uppercase(text: String) -> String {
    text.to_uppercase()
}

// ============================================================================
// Test 8: Function with boolean return
// ============================================================================

/// Check if number is positive
#[mathhook_fn(skip_python)]
pub fn test_is_positive(x: i64) -> bool {
    x > 0
}

// ============================================================================
// Test 9: Function with float parameter
// ============================================================================

/// Double a float value
#[mathhook_fn(skip_python)]
pub fn test_double_float(x: f64) -> f64 {
    x * 2.0
}

// ============================================================================
// Test 10: Function with no parameters
// ============================================================================

/// Get the answer to life, the universe, and everything
#[mathhook_fn(skip_python)]
pub fn test_no_params() -> i64 {
    42
}

// ============================================================================
// Module registration is handled automatically by NAPI
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_fn_works() {
        assert_eq!(test_double_fn(21), 42);
    }

    #[test]
    fn test_optional_result_some() {
        assert_eq!(test_optional_result(5), Some(10));
    }

    #[test]
    fn test_optional_result_none() {
        assert_eq!(test_optional_result(-5), None);
    }

    #[test]
    fn test_add_numbers_works() {
        assert_eq!(test_add_numbers(1, 2, 3), 6);
    }

    #[test]
    fn test_nodejs_only_works() {
        assert_eq!(test_nodejs_only(5), 6);
    }

    #[test]
    fn test_naming_fn_works() {
        assert_eq!(test_naming_fn(4), 12);
    }

    #[test]
    fn test_sum_vec_works() {
        assert_eq!(test_sum_vec(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_uppercase_works() {
        assert_eq!(test_uppercase("hello".to_string()), "HELLO");
    }

    #[test]
    fn test_is_positive_true() {
        assert!(test_is_positive(5));
    }

    #[test]
    fn test_is_positive_false() {
        assert!(!test_is_positive(-5));
    }

    #[test]
    fn test_double_float_works() {
        let pi = std::f64::consts::PI;
        let tau = std::f64::consts::TAU;
        assert!((test_double_float(pi) - tau).abs() < 1e-10);
    }

    #[test]
    fn test_no_params_works() {
        assert_eq!(test_no_params(), 42);
    }
}
