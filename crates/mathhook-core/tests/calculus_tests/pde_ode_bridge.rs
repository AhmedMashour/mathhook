//! Test to verify PDE → ODE bridge is working
//!
//! This test proves that the method of characteristics actually calls the ODE solver
//! and produces meaningful results.

use mathhook_core::calculus::pde::method_of_characteristics::solve_characteristic_odes;
use mathhook_core::core::Expression;

#[test]
fn test_pde_to_ode_bridge_transport_equation() {
    // Transport equation: ∂u/∂t + c·∂u/∂x = 0
    // Characteristic ODEs: dx/dt = c, du/dt = 0
    // For c = 1: dx/dt = 1, du/dt = 0

    let char_eqs = vec![
        Expression::integer(1), // dx/ds = 1
        Expression::integer(1), // dy/ds = 1 (or dt/ds = 1)
        Expression::integer(0), // du/ds = 0
    ];

    let initial_conditions = vec![0.0, 0.0, 1.0]; // x₀=0, y₀=0, u₀=1
    let s_end = 2.0;
    let step_size = 0.1;

    let result = solve_characteristic_odes(&char_eqs, &initial_conditions, s_end, step_size);

    assert!(result.is_ok(), "ODE solver should succeed");

    let solution = result.unwrap();
    assert!(!solution.is_empty(), "Solution should not be empty");

    // Verify structure: each point is (s, [x, y, u])
    assert_eq!(
        solution[0].1.len(),
        3,
        "Each solution point should have 3 components"
    );

    // Verify initial conditions
    let (s0, state0) = &solution[0];
    assert_eq!(*s0, 0.0, "First point should be at s=0");
    assert!((state0[0] - 0.0).abs() < 1e-10, "x(0) should be 0");
    assert!((state0[1] - 0.0).abs() < 1e-10, "y(0) should be 0");
    assert!((state0[2] - 1.0).abs() < 1e-10, "u(0) should be 1");

    // Verify final values (approximately)
    let (s_final, state_final) = solution.last().unwrap();
    assert!(
        (*s_final - s_end).abs() < 0.15,
        "Final s should be near s_end"
    );

    // For dx/ds = 1, x(s) = x₀ + s = 0 + s = s
    assert!(
        (state_final[0] - *s_final).abs() < 0.5,
        "x(s) ≈ s for dx/ds = 1"
    );

    // For du/ds = 0, u(s) = u₀ = 1 (constant)
    assert!(
        (state_final[2] - 1.0).abs() < 1e-6,
        "u(s) should remain constant"
    );
}

#[test]
fn test_pde_to_ode_bridge_with_different_coefficients() {
    // Test with different coefficients
    // dx/ds = 2, dy/ds = 3, du/ds = 0

    let char_eqs = vec![
        Expression::integer(2), // dx/ds = 2
        Expression::integer(3), // dy/ds = 3
        Expression::integer(0), // du/ds = 0
    ];

    let initial_conditions = vec![1.0, 2.0, 5.0]; // x₀=1, y₀=2, u₀=5
    let s_end = 1.0;
    let step_size = 0.1;

    let result = solve_characteristic_odes(&char_eqs, &initial_conditions, s_end, step_size);

    assert!(result.is_ok(), "ODE solver should succeed");

    let solution = result.unwrap();
    assert!(!solution.is_empty(), "Solution should not be empty");

    // Verify solution structure
    for (s, state) in &solution {
        assert!(state.len() == 3, "Each state should have 3 components");
        assert!(
            *s >= 0.0 && *s <= s_end + step_size,
            "s should be in valid range"
        );
    }
}

#[test]
fn test_bridge_error_handling_wrong_equation_count() {
    // Test error handling: wrong number of equations
    let char_eqs = vec![
        Expression::integer(1),
        Expression::integer(1),
        // Missing third equation
    ];

    let initial_conditions = vec![0.0, 0.0, 1.0];

    let result = solve_characteristic_odes(&char_eqs, &initial_conditions, 1.0, 0.1);

    assert!(result.is_err(), "Should error with wrong equation count");
}

#[test]
fn test_bridge_error_handling_wrong_ic_count() {
    // Test error handling: wrong number of initial conditions
    let char_eqs = vec![
        Expression::integer(1),
        Expression::integer(1),
        Expression::integer(0),
    ];

    let initial_conditions = vec![0.0, 0.0]; // Missing u₀

    let result = solve_characteristic_odes(&char_eqs, &initial_conditions, 1.0, 0.1);

    assert!(result.is_err(), "Should error with wrong IC count");
}
