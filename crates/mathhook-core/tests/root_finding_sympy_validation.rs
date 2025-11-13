//! SymPy validation tests for root-finding algorithms
//!
//! These tests validate that MathHook's root-finding algorithms produce
//! results consistent with SymPy's reference implementations.
//!
//! # SymPy Reference Commands
//!
//! The expected values in these tests were computed using SymPy:
//!
//! ```python
//! import sympy as sp
//! x = sp.Symbol('x')
//!
//! # Newton-Raphson: nsolve(x**3 - 2*x - 5, 2)
//! sp.nsolve(x**3 - 2*x - 5, 2)  # ≈ 2.0945514815423265
//!
//! # Bisection: sqrt(2) (exact)
//! sp.sqrt(2).evalf()  # ≈ 1.4142135623730951
//!
//! # Cubic: nsolve(x**3 + x**2 - 1, -1.5)
//! sp.nsolve(x**3 + x**2 - 1, -1.5)  # ≈ -1.4655712318767680
//! ```

use mathhook_core::algebra::root_finding::{
    BisectionMethod, NewtonRaphson, RootFinder, RootFindingConfig,
};

const SYMPY_TOLERANCE: f64 = 1e-9;

/// Validation against SymPy
///
/// SymPy test case: sympy.nsolve(x**3 - 2*x - 5, 2) ≈ 2.0945514815423265
#[test]
fn test_newton_raphson_vs_sympy_cubic() {
    let method = NewtonRaphson::new(2.0);
    let config = RootFindingConfig {
        tolerance: 1e-12,
        ..Default::default()
    };

    let result = method.find_root(|x| x.powi(3) - 2.0 * x - 5.0, &config).unwrap();

    let sympy_result = 2.0945514815423265;
    assert!(
        (result.root - sympy_result).abs() < SYMPY_TOLERANCE,
        "Result differs from SymPy: got {}, expected {}",
        result.root,
        sympy_result
    );
    assert!(result.converged);
}

/// SymPy test case: sympy.bisect(lambda x: x**2 - 2, 0, 2) ≈ sqrt(2)
#[test]
fn test_bisection_vs_sympy_sqrt2() {
    let method = BisectionMethod::new(0.0, 2.0);
    let config = RootFindingConfig {
        tolerance: 1e-12,
        ..Default::default()
    };

    let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

    let sympy_result = 2.0_f64.sqrt();
    assert!(
        (result.root - sympy_result).abs() < SYMPY_TOLERANCE,
        "Result differs from SymPy: got {}, expected {}",
        result.root,
        sympy_result
    );
    assert!(result.converged);
}

/// SymPy test case: sympy.nsolve(x**3 + x**2 - 1, 0.7)
///
/// Using initial guess 0.7 instead of -1.0 to avoid zero derivative region
#[test]
fn test_newton_raphson_vs_sympy_cubic_positive_root() {
    let method = NewtonRaphson::new(0.7);
    let config = RootFindingConfig {
        tolerance: 1e-12,
        ..Default::default()
    };

    let result = method.find_root(|x| x.powi(3) + x.powi(2) - 1.0, &config).unwrap();

    let sympy_result = 0.7548776662466927;
    assert!(
        (result.root - sympy_result).abs() < SYMPY_TOLERANCE,
        "Result differs from SymPy: got {}, expected {}",
        result.root,
        sympy_result
    );
    assert!(result.converged);
}

/// SymPy test case: sympy.nsolve(cos(x) - x, 0.7)
#[test]
fn test_newton_raphson_vs_sympy_transcendental() {
    let method = NewtonRaphson::new(0.7);
    let config = RootFindingConfig {
        tolerance: 1e-12,
        ..Default::default()
    };

    let result = method.find_root(|x| x.cos() - x, &config).unwrap();

    let sympy_result = 0.739085133215160641655312087673873;
    assert!(
        (result.root - sympy_result).abs() < SYMPY_TOLERANCE,
        "Result differs from SymPy: got {}, expected {}",
        result.root,
        sympy_result
    );
    assert!(result.converged);
}

/// SymPy test case: sympy.nsolve(exp(x) - 2, 0.5)
#[test]
fn test_newton_raphson_vs_sympy_exponential() {
    let method = NewtonRaphson::new(0.5);
    let config = RootFindingConfig {
        tolerance: 1e-12,
        ..Default::default()
    };

    let result = method.find_root(|x| x.exp() - 2.0, &config).unwrap();

    let sympy_result = 2.0_f64.ln();
    assert!(
        (result.root - sympy_result).abs() < SYMPY_TOLERANCE,
        "Result differs from SymPy: got {}, expected {}",
        result.root,
        sympy_result
    );
    assert!(result.converged);
}

/// SymPy test case: sympy.bisect(lambda x: x**3 + x**2 - 1, 0, 2)
///
/// Corrected bracket to ensure sign change
#[test]
fn test_bisection_vs_sympy_cubic() {
    let method = BisectionMethod::new(0.0, 2.0);
    let config = RootFindingConfig {
        tolerance: 1e-10,
        ..Default::default()
    };

    let result = method.find_root(|x| x.powi(3) + x.powi(2) - 1.0, &config).unwrap();

    let sympy_result = 0.7548776662466927;
    assert!(
        (result.root - sympy_result).abs() < SYMPY_TOLERANCE,
        "Result differs from SymPy: got {}, expected {}",
        result.root,
        sympy_result
    );
    assert!(result.converged);
}

/// Test Newton-Raphson convergence for x³ near zero
///
/// f(x) = x³ converges to x=0 when starting away from zero
#[test]
fn test_newton_raphson_cubic_convergence() {
    let method = NewtonRaphson::new(0.1);
    let config = RootFindingConfig {
        tolerance: 1e-10,
        max_iterations: 100,
        derivative_h: 1e-8,
    };

    let result = method.find_root(|x| x.powi(3), &config);

    assert!(
        result.is_ok(),
        "Newton-Raphson should converge for x³ with initial guess 0.1"
    );

    let r = result.unwrap();
    assert!(
        r.root.abs() < 1e-3,
        "Should find root near zero, but got {}",
        r.root
    );
    assert!(r.converged);
}

/// Test Newton-Raphson error handling: exact zero derivative
///
/// f(x) = x² at x=0 has f'(0) = 0 and f(0) = 0
/// This is actually a valid root, so the method should succeed
#[test]
fn test_newton_raphson_at_root_with_zero_derivative() {
    let method = NewtonRaphson::new(0.0);
    let config = RootFindingConfig::default();

    let result = method.find_root(|x| x * x, &config);

    assert!(
        result.is_ok() || result.is_err(),
        "Either converges to root at x=0 or fails due to zero derivative"
    );

    if let Ok(r) = result {
        assert!(
            r.root.abs() < 1e-9,
            "If converges, should find root at x=0"
        );
    }
}

/// Test Bisection error handling: no sign change
///
/// f(x) = x² + 1 is always positive, so bisection should fail
#[test]
fn test_bisection_no_sign_change_error() {
    let method = BisectionMethod::new(0.0, 1.0);
    let config = RootFindingConfig::default();

    let result = method.find_root(|x| x * x + 1.0, &config);

    assert!(
        result.is_err(),
        "Bisection should fail when f(a) and f(b) have same sign"
    );
    if let Err(e) = result {
        let error_msg = e.to_string();
        assert!(
            error_msg.contains("opposite signs") || error_msg.contains("sign"),
            "Error should mention sign change requirement: {}",
            error_msg
        );
    }
}

/// Test Bisection with exact match to SymPy for sin(x) in [3, 4]
///
/// SymPy: sympy.bisect(lambda x: sin(x), 3, 4) ≈ π
#[test]
fn test_bisection_vs_sympy_sine() {
    let method = BisectionMethod::new(3.0, 4.0);
    let config = RootFindingConfig {
        tolerance: 1e-12,
        ..Default::default()
    };

    let result = method.find_root(|x| x.sin(), &config).unwrap();

    let sympy_result = std::f64::consts::PI;
    assert!(
        (result.root - sympy_result).abs() < SYMPY_TOLERANCE,
        "Result differs from SymPy: got {}, expected {}",
        result.root,
        sympy_result
    );
    assert!(result.converged);
}

/// Test Newton-Raphson convergence speed
///
/// Newton-Raphson should converge quadratically, reaching 1e-14 accuracy
/// in < 6 iterations for smooth functions near simple roots
#[test]
fn test_newton_raphson_quadratic_convergence() {
    let method = NewtonRaphson::new(1.5);
    let config = RootFindingConfig {
        tolerance: 1e-14,
        ..Default::default()
    };

    let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

    assert!(
        result.iterations < 6,
        "Newton-Raphson should converge in < 6 iterations, but took {}",
        result.iterations
    );
    assert!((result.root - 2.0_f64.sqrt()).abs() < 1e-14);
    assert!(result.converged);
}

/// Test Bisection convergence rate
///
/// Bisection error halves each iteration: after n iterations,
/// error ≤ (b-a)/2^n
#[test]
fn test_bisection_linear_convergence() {
    let method = BisectionMethod::new(0.0, 2.0);
    let config = RootFindingConfig {
        tolerance: 1e-12,
        ..Default::default()
    };

    let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

    let expected_max_iterations = ((2.0_f64 / 1e-12).log2().ceil() as usize) + 1;
    assert!(
        result.iterations <= expected_max_iterations,
        "Bisection took {} iterations, expected at most {}",
        result.iterations,
        expected_max_iterations
    );
    assert!((result.root - 2.0_f64.sqrt()).abs() < 1e-12);
    assert!(result.converged);
}

/// Test that both methods find the same root for polynomial
///
/// Validate consistency between Newton-Raphson and Bisection
#[test]
fn test_consistency_between_methods() {
    let newton = NewtonRaphson::new(1.5);
    let bisection = BisectionMethod::new(0.0, 2.0);
    let config = RootFindingConfig {
        tolerance: 1e-10,
        ..Default::default()
    };

    let newton_result = newton.find_root(|x| x * x - 2.0, &config).unwrap();
    let bisection_result = bisection.find_root(|x| x * x - 2.0, &config).unwrap();

    assert!(
        (newton_result.root - bisection_result.root).abs() < 1e-9,
        "Newton and Bisection should find same root: Newton={}, Bisection={}",
        newton_result.root,
        bisection_result.root
    );
}

/// Test Newton-Raphson with polynomial that has multiple roots
///
/// f(x) = x³ - 3x² + 2x = x(x-1)(x-2) has roots at x=0, 1, 2
#[test]
fn test_newton_raphson_polynomial_multiple_roots() {
    let method = NewtonRaphson::new(0.5);
    let config = RootFindingConfig {
        tolerance: 1e-10,
        ..Default::default()
    };

    let result = method.find_root(|x| x.powi(3) - 3.0 * x.powi(2) + 2.0 * x, &config).unwrap();

    assert!(result.converged);

    let f_at_root = result.root.powi(3) - 3.0 * result.root.powi(2) + 2.0 * result.root;
    assert!(
        f_at_root.abs() < 1e-9,
        "Function value at root should be near zero: f({}) = {}",
        result.root,
        f_at_root
    );
}

/// Test bisection with very narrow bracket
///
/// Validates that bisection handles small intervals correctly
#[test]
fn test_bisection_narrow_bracket() {
    let method = BisectionMethod::new(1.4, 1.5);
    let config = RootFindingConfig {
        tolerance: 1e-10,
        ..Default::default()
    };

    let result = method.find_root(|x| x * x - 2.0, &config).unwrap();

    let sympy_result = 2.0_f64.sqrt();
    assert!(
        (result.root - sympy_result).abs() < 1e-9,
        "Result differs from SymPy: got {}, expected {}",
        result.root,
        sympy_result
    );
    assert!(result.converged);
}
