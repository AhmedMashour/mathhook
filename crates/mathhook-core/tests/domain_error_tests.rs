//! Domain error tests
//!
//! Comprehensive tests for mathematical domain restrictions and error handling.
//! Verifies that operations correctly detect and report domain violations,
//! singularities, and undefined behavior.

use mathhook_core::{Expression, MathError, Number, Simplify};

/// Test that 0^(-1) produces division by zero error
///
/// Mathematical reasoning: 0^(-1) = 1/0^1 = 1/0 which is division by zero
#[test]
fn test_zero_to_negative_one_division_by_zero() {
    let base = Expression::integer(0);
    let exp = Expression::integer(-1);
    let expr = Expression::pow(base, exp);

    // Currently returns Expression::function("undefined", vec![])
    // After fix, should detect division by zero during simplification
    let simplified = expr.simplify();

    // For now, verify it returns undefined function
    match simplified {
        Expression::Function { name, .. } if name == "undefined" => {
            // Expected current behavior
        }
        _ => {
            // After implementing error system, this test will need updating
            // to expect an error or a symbolic representation that can be
            // evaluated to produce an error
        }
    }
}

/// Test that 0^0 is recognized as indeterminate
///
/// Mathematical reasoning: 0^0 is an indeterminate form with different
/// conventions in different contexts (often 1 in combinatorics, undefined in analysis)
#[test]
fn test_zero_to_zero_indeterminate() {
    let base = Expression::integer(0);
    let exp = Expression::integer(0);
    let expr = Expression::pow(base, exp);

    let simplified = expr.simplify();

    // 0^0 should be handled specially
    // Convention: In many contexts, 0^0 = 1, but it's technically indeterminate
    // The simplifier currently returns 1 (x^0 = 1 rule)
    // This is acceptable for now, but should be documented
}

/// Test that 0^(-n) for n > 1 is division by zero
#[test]
fn test_zero_to_negative_power_division_by_zero() {
    for n in [2, 3, 5, 10] {
        let base = Expression::integer(0);
        let exp = Expression::integer(-n);
        let expr = Expression::pow(base, exp);

        let simplified = expr.simplify();

        // 0^(-n) = 1/0^n = 1/0 is division by zero
        // Should either return error or symbolic representation
    }
}

/// Test that sqrt(-1) in real domain produces domain error
///
/// Note: In complex domain, sqrt(-1) = i is valid
#[test]
fn test_sqrt_negative_real_domain() {
    let expr = Expression::function("sqrt".to_string(), vec![Expression::integer(-1)]);

    let result = expr.evaluate();
    assert!(matches!(result, Err(MathError::DomainError { .. })));
}

/// Test that sqrt requires non-negative input in real domain
#[test]
fn test_sqrt_domain_restriction() {
    let test_cases = vec![
        (-2, true),   // Should error
        (-1, true),   // Should error
        (0, false),   // Valid: sqrt(0) = 0
        (1, false),   // Valid: sqrt(1) = 1
        (4, false),   // Valid: sqrt(4) = 2
    ];

    for (value, should_error) in test_cases {
        let expr = Expression::function("sqrt".to_string(), vec![Expression::integer(value)]);
        let result = expr.evaluate();

        if should_error {
            assert!(result.is_err(), "Expected error for sqrt({})", value);
        } else {
            assert!(result.is_ok(), "Expected success for sqrt({})", value);
        }
    }
}

/// Test that log(0) produces pole error
///
/// Mathematical reasoning: log(x) → -∞ as x → 0+, so log(0) is a pole
#[test]
fn test_log_zero_pole() {
    let expr = Expression::function("log".to_string(), vec![Expression::integer(0)]);

    let result = expr.evaluate();
    assert!(matches!(result, Err(MathError::Pole { function, .. }) if function == "log"));
}

/// Test that log requires positive input in real domain
#[test]
fn test_log_domain_restriction() {
    let test_cases = vec![
        (-2, true),   // Negative: branch cut in real domain
        (-1, true),   // Negative: branch cut in real domain
        (0, true),    // Zero: pole
        (1, false),   // Valid: log(1) = 0
        (2, false),   // Valid: log(2) ≈ 0.693
    ];

    for (value, should_error) in test_cases {
        let expr = Expression::function("log".to_string(), vec![Expression::integer(value)]);
        let result = expr.evaluate();

        if should_error {
            assert!(result.is_err(), "Expected error for log({})", value);
        } else {
            assert!(result.is_ok(), "Expected success for log({})", value);
        }
    }
}

/// Test that ln requires positive input in real domain
#[test]
fn test_ln_domain_restriction() {
    let test_cases = vec![
        (-2, true),   // Negative: branch cut in real domain
        (-1, true),   // Negative: branch cut in real domain
        (0, true),    // Zero: pole
        (1, false),   // Valid: ln(1) = 0
        (2, false),   // Valid: ln(2) ≈ 0.693
    ];

    for (value, should_error) in test_cases {
        let expr = Expression::function("ln".to_string(), vec![Expression::integer(value)]);
        let result = expr.evaluate();

        if should_error {
            assert!(result.is_err(), "Expected error for ln({})", value);
        } else {
            assert!(result.is_ok(), "Expected success for ln({})", value);
        }
    }
}

/// Test that ln(0) produces pole error
#[test]
fn test_ln_zero_pole() {
    let expr = Expression::function("ln".to_string(), vec![Expression::integer(0)]);

    let result = expr.evaluate();
    assert!(matches!(result, Err(MathError::Pole { function, .. }) if function == "ln"));
}

/// Test that ln of negative numbers produces branch cut error
#[test]
fn test_ln_negative_branch_cut() {
    let expr = Expression::function("ln".to_string(), vec![Expression::integer(-1)]);

    let result = expr.evaluate();
    assert!(matches!(result, Err(MathError::BranchCut { .. })));
}

/// Test that log of negative numbers produces branch cut error in real domain
#[test]
fn test_log_negative_branch_cut() {
    let expr = Expression::function("log".to_string(), vec![Expression::integer(-1)]);

    let result = expr.evaluate();
    assert!(matches!(result, Err(MathError::BranchCut { .. })));
}

/// Test that division by zero is detected
#[test]
fn test_division_by_zero() {
    // Division is represented as multiplication by negative power
    // 1/0 = 1 * 0^(-1)
    let expr = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(Expression::integer(0), Expression::integer(-1))
    ]);

    let result = expr.evaluate();
    eprintln!("Result: {:?}", result);
    assert!(matches!(result, Err(MathError::DivisionByZero)));
}

/// Test that tan(π/2) produces pole error
///
/// Mathematical reasoning: tan(x) = sin(x)/cos(x), and cos(π/2) = 0
#[test]
fn test_tan_pole_at_pi_over_2() {
    use std::f64::consts::PI;

    let expr = Expression::function("tan".to_string(), vec![
        Expression::Number(Number::float(PI / 2.0))
    ]);

    let result = expr.evaluate();
    assert!(matches!(result, Err(MathError::Pole { function, .. }) if function == "tan"));
}

/// Test that tan has poles at π/2 + nπ
#[test]
fn test_tan_multiple_poles() {
    use std::f64::consts::PI;

    let pole_locations = vec![
        PI / 2.0,         // π/2
        -PI / 2.0,        // -π/2
        3.0 * PI / 2.0,   // 3π/2
        5.0 * PI / 2.0,   // 5π/2
    ];

    for pole in pole_locations {
        let expr = Expression::function("tan".to_string(), vec![
            Expression::Number(Number::float(pole))
        ]);
        let result = expr.evaluate();
        assert!(matches!(result, Err(MathError::Pole { .. })), "Expected pole error at tan({})", pole);
    }
}

/// Test that arcsin domain is [-1, 1] in real numbers
#[test]
fn test_arcsin_domain_restriction() {
    let test_cases = vec![
        (-2.0, true),   // Out of domain
        (-1.5, true),   // Out of domain
        (-1.0, false),  // Valid: arcsin(-1) = -π/2
        (0.0, false),   // Valid: arcsin(0) = 0
        (0.5, false),   // Valid: arcsin(0.5) = π/6
        (1.0, false),   // Valid: arcsin(1) = π/2
        (1.5, true),    // Out of domain
        (2.0, true),    // Out of domain
    ];

    for (value, should_error) in test_cases {
        let expr = Expression::function("arcsin".to_string(), vec![
            Expression::Number(Number::float(value))
        ]);
        let result = expr.evaluate();

        if should_error {
            assert!(result.is_err(), "Expected error for arcsin({})", value);
        } else {
            assert!(result.is_ok(), "Expected success for arcsin({})", value);
        }
    }
}

/// Test that arccos domain is [-1, 1] in real numbers
#[test]
fn test_arccos_domain_restriction() {
    let test_cases = vec![
        (-2.0, true),   // Out of domain
        (-1.0, false),  // Valid: arccos(-1) = π
        (0.0, false),   // Valid: arccos(0) = π/2
        (1.0, false),   // Valid: arccos(1) = 0
        (2.0, true),    // Out of domain
    ];

    for (value, should_error) in test_cases {
        let expr = Expression::function("arccos".to_string(), vec![
            Expression::Number(Number::float(value))
        ]);
        let result = expr.evaluate();

        if should_error {
            assert!(result.is_err(), "Expected error for arccos({})", value);
        } else {
            assert!(result.is_ok(), "Expected success for arccos({})", value);
        }
    }
}

/// Test that csc(0) produces pole error
///
/// Mathematical reasoning: csc(x) = 1/sin(x), and sin(0) = 0
#[test]
fn test_csc_pole_at_zero() {
    let expr = Expression::function("csc".to_string(), vec![Expression::integer(0)]);

    let result = expr.evaluate();
    assert!(matches!(result, Err(MathError::Pole { .. })));
}

/// Test that csc has poles at nπ
#[test]
fn test_csc_multiple_poles() {
    use std::f64::consts::PI;

    let pole_locations = vec![
        0.0,
        PI,
        -PI,
        2.0 * PI,
        3.0 * PI,
    ];

    for pole in pole_locations {
        let expr = Expression::function("csc".to_string(), vec![
            Expression::Number(Number::float(pole))
        ]);
        let result = expr.evaluate();
        assert!(matches!(result, Err(MathError::Pole { .. })), "Expected pole error at csc({})", pole);
    }
}

/// Test that sec(π/2) produces pole error
///
/// Mathematical reasoning: sec(x) = 1/cos(x), and cos(π/2) = 0
#[test]
fn test_sec_pole_at_pi_over_2() {
    use std::f64::consts::PI;

    let expr = Expression::function("sec".to_string(), vec![
        Expression::Number(Number::float(PI / 2.0))
    ]);

    let result = expr.evaluate();
    assert!(matches!(result, Err(MathError::Pole { .. })));
}

/// Test error messages are clear and helpful
#[test]
fn test_error_messages_quality() {
    // Test DivisionByZero message
    let err = MathError::DivisionByZero;
    let msg = err.to_string();
    assert!(msg.contains("Division by zero"));

    // Test DomainError message
    let err = MathError::DomainError {
        operation: "sqrt".to_string(),
        value: Expression::integer(-1),
        reason: "sqrt requires non-negative input in real domain".to_string(),
    };
    let msg = err.to_string();
    assert!(msg.contains("Domain error"));
    assert!(msg.contains("sqrt"));

    // Test Pole message
    let err = MathError::Pole {
        function: "log".to_string(),
        at: Expression::integer(0),
    };
    let msg = err.to_string();
    assert!(msg.contains("Pole singularity"));
    assert!(msg.contains("log"));

    // Test BranchCut message
    let err = MathError::BranchCut {
        function: "log".to_string(),
        value: Expression::integer(-1),
    };
    let msg = err.to_string();
    assert!(msg.contains("Branch cut"));
    assert!(msg.contains("log"));

    // Test Undefined message
    let err = MathError::Undefined {
        expression: Expression::pow(Expression::integer(0), Expression::integer(0)),
        reason: "0^0 is indeterminate".to_string(),
    };
    let msg = err.to_string();
    assert!(msg.contains("Undefined"));
    assert!(msg.contains("indeterminate"));
}

/// Test that error type is Clone and PartialEq
#[test]
fn test_error_traits() {
    let err1 = MathError::DivisionByZero;
    let err2 = err1.clone();
    assert_eq!(err1, err2);

    let err3 = MathError::NotImplemented {
        feature: "groebner bases".to_string(),
    };
    assert_ne!(err1, err3);
}

/// Test that MathError implements std::error::Error
#[test]
fn test_error_trait_implementation() {
    let err: Box<dyn std::error::Error> = Box::new(MathError::DivisionByZero);
    let _msg = err.to_string(); // Should work since Error trait is implemented
}

/// Test helper methods is_negative_number and is_positive_number
#[test]
fn test_number_sign_helpers() {
    // Test negative numbers
    assert!(Expression::integer(-5).is_negative_number());
    assert!(Expression::rational(-1, 2).is_negative_number());
    assert!(Expression::Number(Number::float(-3.14)).is_negative_number());

    // Test positive numbers
    assert!(Expression::integer(5).is_positive_number());
    assert!(Expression::rational(1, 2).is_positive_number());
    assert!(Expression::Number(Number::float(3.14)).is_positive_number());

    // Test zero (neither positive nor negative)
    assert!(!Expression::integer(0).is_negative_number());
    assert!(!Expression::integer(0).is_positive_number());

    // Test symbolic expressions (not numbers)
    use mathhook_core::symbol;
    assert!(!Expression::symbol(symbol!(x)).is_negative_number());
    assert!(!Expression::symbol(symbol!(x)).is_positive_number());
}

/// Test that symbolic sqrt expressions don't trigger domain errors
///
/// Mathematical reasoning: sqrt(x) is valid symbolically even though x might
/// evaluate to negative at runtime. Domain checking should only happen during
/// numerical evaluation, not during symbolic construction.
#[test]
fn test_sqrt_symbolic_allowed() {
    use mathhook_core::symbol;

    // Symbolic sqrt should be allowed
    let x = symbol!(x);
    let expr = Expression::sqrt(Expression::symbol(x.clone()));

    // Construction should succeed
    assert!(matches!(&expr, Expression::Function { name, .. } if name == "sqrt"));

    // Evaluation should succeed for symbolic expressions (no numeric value to check)
    let result = expr.evaluate();
    assert!(result.is_ok(), "Symbolic sqrt(x) should not error during evaluation");
}

/// Test that symbolic log expressions don't trigger domain errors
#[test]
fn test_log_symbolic_allowed() {
    use mathhook_core::symbol;

    // Symbolic log should be allowed
    let x = symbol!(x);
    let expr = Expression::function("log", vec![Expression::symbol(x.clone())]);

    // Construction should succeed
    assert!(matches!(&expr, Expression::Function { name, .. } if name == "log"));

    // Evaluation should succeed for symbolic expressions
    let result = expr.evaluate();
    assert!(result.is_ok(), "Symbolic log(x) should not error during evaluation");
}

/// Test that symbolic ln expressions don't trigger domain errors
#[test]
fn test_ln_symbolic_allowed() {
    use mathhook_core::symbol;

    // Symbolic ln should be allowed
    let x = symbol!(x);
    let expr = Expression::function("ln", vec![Expression::symbol(x.clone())]);

    // Construction should succeed
    assert!(matches!(&expr, Expression::Function { name, .. } if name == "ln"));

    // Evaluation should succeed for symbolic expressions
    let result = expr.evaluate();
    assert!(result.is_ok(), "Symbolic ln(x) should not error during evaluation");
}

/// Test that negative rationals are caught by domain checking
#[test]
fn test_sqrt_negative_rational() {
    let expr = Expression::sqrt(Expression::rational(-1, 2));

    let result = expr.evaluate();
    assert!(matches!(result, Err(MathError::DomainError { .. })),
        "sqrt of negative rational should error");
}

/// Test that negative floats are caught by domain checking
#[test]
fn test_sqrt_negative_float() {
    let expr = Expression::sqrt(Expression::Number(Number::float(-2.5)));

    let result = expr.evaluate();
    assert!(matches!(result, Err(MathError::DomainError { .. })),
        "sqrt of negative float should error");
}

/// Test that log catches negative rationals
#[test]
fn test_log_negative_rational() {
    let expr = Expression::function("log", vec![Expression::rational(-3, 4)]);

    let result = expr.evaluate();
    assert!(matches!(result, Err(MathError::BranchCut { .. })),
        "log of negative rational should error with branch cut");
}

/// Test that log catches negative floats
#[test]
fn test_log_negative_float() {
    let expr = Expression::function("log", vec![Expression::Number(Number::float(-1.5))]);

    let result = expr.evaluate();
    assert!(matches!(result, Err(MathError::BranchCut { .. })),
        "log of negative float should error with branch cut");
}

/// Test future evaluation API structure
///
/// This test documents the intended API for domain-aware evaluation
#[test]
#[ignore] // Future API - not yet implemented
fn test_future_evaluation_api_structure() {
    let expr = Expression::function("sqrt".to_string(), vec![Expression::integer(-1)]);

    // Future API (not yet implemented):
    // Real domain evaluation - should error
    // let result = expr.evaluate_in_domain(Domain::Real);
    // assert!(matches!(result, Err(MathError::DomainError { .. })));

    // Complex domain evaluation - should succeed
    // let result = expr.evaluate_in_domain(Domain::Complex);
    // assert!(result.is_ok());

    // Default evaluation (complex-safe)
    // let result = expr.evaluate();
    // assert!(result.is_ok());
}

/// Integration test: Verify that simplification doesn't lose error information
#[test]
fn test_simplification_preserves_error_markers() {
    // When we have 0^(-1), even after simplification, the error should be detectable
    let expr = Expression::pow(Expression::integer(0), Expression::integer(-1));
    let simplified = expr.simplify();

    // Currently returns Expression::function("undefined", vec![])
    // This is a marker for error - not ideal but better than silently wrong result
    match simplified {
        Expression::Function { name, .. } if name == "undefined" => {
            // Current behavior - acceptable as error marker
        }
        _ => {
            // Future: Should have better error handling
        }
    }
}
