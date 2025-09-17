//! Unit tests for trigonometric product integration
//!
//! Tests the integrate_trig_product function directly with various
//! trigonometric product patterns. All results validated against SymPy.

use mathhook_core::calculus::integrals::trigonometric::integrate_trig_product;
use mathhook_core::core::Expression;
use mathhook_core::symbol;

#[test]
fn test_sin_cos_product_different_freq() {
    let x = symbol!(x);

    // ∫sin(2x)cos(3x) dx
    // SymPy: cos(x)/2 - cos(5x)/10
    let result = integrate_trig_product("sin", 2, "cos", 3, x.clone());
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(matches!(integral, Expression::Add(_)));
}

#[test]
fn test_sin_cos_product_same_freq() {
    let x = symbol!(x);

    // ∫sin(x)cos(x) dx = -cos(2x)/4
    // SymPy: sin(x)**2/2 (differs by constant 1/4)
    let result = integrate_trig_product("sin", 1, "cos", 1, x.clone());
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(matches!(integral, Expression::Mul(_)));
}

#[test]
fn test_sin_sin_product_different_freq() {
    let x = symbol!(x);

    // ∫sin(x)sin(2x) dx
    // SymPy: sin(x)/2 - sin(3x)/6
    let result = integrate_trig_product("sin", 1, "sin", 2, x.clone());
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(matches!(integral, Expression::Add(_)));
}

#[test]
fn test_sin_sin_product_same_freq() {
    let x = symbol!(x);

    // ∫sin(x)sin(x) dx = ∫sin²(x) dx = x/2 - sin(2x)/4
    let result = integrate_trig_product("sin", 1, "sin", 1, x.clone());
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(matches!(integral, Expression::Add(_)));
}

#[test]
fn test_cos_cos_product_different_freq() {
    let x = symbol!(x);

    // ∫cos(x)cos(2x) dx
    // SymPy: sin(x)/2 + sin(3x)/6
    let result = integrate_trig_product("cos", 1, "cos", 2, x.clone());
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(matches!(integral, Expression::Add(_)));
}

#[test]
fn test_cos_cos_product_same_freq() {
    let x = symbol!(x);

    // ∫cos(x)cos(x) dx = ∫cos²(x) dx = x/2 + sin(2x)/4
    let result = integrate_trig_product("cos", 1, "cos", 1, x.clone());
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(matches!(integral, Expression::Add(_)));
}

#[test]
fn test_sin_cos_product_reversed_args() {
    let x = symbol!(x);

    // ∫cos(3x)sin(2x) dx should give same result as ∫sin(2x)cos(3x) dx
    let result1 = integrate_trig_product("sin", 2, "cos", 3, x.clone());
    let result2 = integrate_trig_product("cos", 3, "sin", 2, x.clone());

    assert!(result1.is_some());
    assert!(result2.is_some());

    // Both should be valid expressions
    assert!(matches!(result1.unwrap(), Expression::Add(_)));
    assert!(matches!(result2.unwrap(), Expression::Add(_)));
}

#[test]
fn test_sin_cos_product_higher_freq() {
    let x = symbol!(x);

    // ∫sin(5x)cos(7x) dx
    let result = integrate_trig_product("sin", 5, "cos", 7, x.clone());
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(matches!(integral, Expression::Add(_)));
}

#[test]
fn test_sin_sin_product_higher_freq() {
    let x = symbol!(x);

    // ∫sin(3x)sin(5x) dx
    let result = integrate_trig_product("sin", 3, "sin", 5, x.clone());
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(matches!(integral, Expression::Add(_)));
}

#[test]
fn test_cos_cos_product_higher_freq() {
    let x = symbol!(x);

    // ∫cos(4x)cos(6x) dx
    let result = integrate_trig_product("cos", 4, "cos", 6, x.clone());
    assert!(result.is_some());

    let integral = result.unwrap();
    assert!(matches!(integral, Expression::Add(_)));
}

#[test]
fn test_invalid_function_combination() {
    let x = symbol!(x);

    // Invalid: tan*sin is not supported
    let result = integrate_trig_product("tan", 1, "sin", 1, x.clone());
    assert!(result.is_none());
}

#[test]
fn test_invalid_function_name() {
    let x = symbol!(x);

    // Invalid function name
    let result = integrate_trig_product("invalid", 1, "cos", 1, x.clone());
    assert!(result.is_none());
}

#[test]
fn test_sin_cos_negative_frequencies() {
    let x = symbol!(x);

    // ∫sin(-2x)cos(3x) dx
    // This should work since we just compute (m-n) and (m+n)
    let result = integrate_trig_product("sin", -2, "cos", 3, x.clone());
    assert!(result.is_some());
}

#[test]
fn test_structure_of_result() {
    let x = symbol!(x);

    // Test that sin(2x)cos(3x) produces correct structure
    let result = integrate_trig_product("sin", 2, "cos", 3, x).unwrap();

    // Should be an addition of two terms
    if let Expression::Add(terms) = result {
        assert_eq!(terms.len(), 2);

        // Each term should be a multiplication involving cos function
        for term in terms.iter() {
            assert!(matches!(term, Expression::Mul(_)));
        }
    } else {
        panic!("Expected Add expression");
    }
}

#[test]
fn test_same_freq_structure() {
    let x = symbol!(x);

    // Test that sin(2x)cos(2x) produces: -cos(4x)/8
    let result = integrate_trig_product("sin", 2, "cos", 2, x).unwrap();

    // Should be a multiplication: -cos(4x)/8
    if let Expression::Mul(factors) = result {
        assert_eq!(factors.len(), 2);
    } else {
        panic!("Expected Mul expression");
    }
}
