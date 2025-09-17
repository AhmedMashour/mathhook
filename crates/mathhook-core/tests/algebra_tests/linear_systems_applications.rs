//! Real-world applications of linear system solvers
//!
//! These tests demonstrate practical uses of matrix solving in various domains:
//! - Electrical circuit analysis (Kirchhoff's laws)
//! - Network flow distribution
//! - Chemical equation balancing
//! - Structural engineering (truss equilibrium)
//! - Economic input-output models (Leontief)
//! - Mixture/blending problems
//! - Heat transfer (discretized PDEs)

use mathhook_core::core::{Expression, Number};
use mathhook_core::matrices::unified::Matrix;
use mathhook_core::simplify::Simplify;
use num_bigint::BigInt;

fn expr_equals_integer(expr: &Expression, value: i64) -> bool {
    let simplified = expr.clone().simplify();
    match simplified {
        Expression::Number(Number::Integer(i)) => i == value,
        Expression::Number(Number::Rational(r)) => {
            r.denom() == &BigInt::from(1) && r.numer() == &BigInt::from(value)
        }
        _ => false,
    }
}

/// Electrical Circuit Analysis using Kirchhoff's Laws
///
/// Circuit with 3 loops and resistors:
/// - Loop 1: 10V source, R1=2Ω, R2=4Ω
/// - Loop 2: R2=4Ω (shared), R3=3Ω
/// - Loop 3: R3=3Ω (shared), R4=1Ω, 5V source
///
/// Applying Kirchhoff's voltage law (KVL):
///   Loop 1: 2*I1 + 4*(I1-I2) = 10  →  6*I1 - 4*I2 = 10
///   Loop 2: 4*(I2-I1) + 3*(I2-I3) = 0  →  -4*I1 + 7*I2 - 3*I3 = 0
///   Loop 3: 3*(I3-I2) + 1*I3 = 5  →  -3*I2 + 4*I3 = 5
///
/// Solution: I1 = 2A, I2 = 1/2A, I3 = 13/8A
#[test]
fn test_electrical_circuit_kirchhoff() {
    // Coefficient matrix from KVL equations
    let a = Matrix::from_arrays([
        [6, -4, 0],  // 6*I1 - 4*I2 + 0*I3 = 10
        [-4, 7, -3], // -4*I1 + 7*I2 - 3*I3 = 0
        [0, -3, 4],  // 0*I1 - 3*I2 + 4*I3 = 5
    ]);
    let b = vec![
        Expression::integer(10),
        Expression::integer(0),
        Expression::integer(5),
    ];

    let currents = a.solve(&b).unwrap();
    assert_eq!(currents.len(), 3);

    // Verify solution satisfies all equations (Ax = b)
    // Equation 1: 6*I1 - 4*I2 = 10
    let eq1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(6), currents[0].clone()]),
        Expression::mul(vec![Expression::integer(-4), currents[1].clone()]),
    ])
    .simplify();
    assert!(
        expr_equals_integer(&eq1, 10),
        "Circuit equation 1 failed: 6*I1 - 4*I2 should equal 10, got {:?}",
        eq1
    );

    // Equation 2: -4*I1 + 7*I2 - 3*I3 = 0
    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(-4), currents[0].clone()]),
        Expression::mul(vec![Expression::integer(7), currents[1].clone()]),
        Expression::mul(vec![Expression::integer(-3), currents[2].clone()]),
    ])
    .simplify();
    assert!(
        expr_equals_integer(&eq2, 0),
        "Circuit equation 2 failed: -4*I1 + 7*I2 - 3*I3 should equal 0, got {:?}",
        eq2
    );

    // Equation 3: -3*I2 + 4*I3 = 5
    let eq3 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(-3), currents[1].clone()]),
        Expression::mul(vec![Expression::integer(4), currents[2].clone()]),
    ])
    .simplify();
    assert!(
        expr_equals_integer(&eq3, 5),
        "Circuit equation 3 failed: -3*I2 + 4*I3 should equal 5, got {:?}",
        eq3
    );
}

/// Network Flow Problem - Water distribution system
///
/// Material balance with splitting ratios:
///   Total flow: x + y + z = 100
///   Route A gets twice as much as route B: x = 2y
///   Route C gets same as route B: z = y
///
/// Solution: x = 50, y = 25, z = 25
#[test]
fn test_network_flow_distribution() {
    let a = Matrix::from_arrays([
        [1, 1, 1],  // x + y + z = 100 (total flow)
        [1, -2, 0], // x - 2y = 0 (x is twice y)
        [0, 1, -1], // y - z = 0 (y equals z)
    ]);
    let b = vec![
        Expression::integer(100),
        Expression::integer(0),
        Expression::integer(0),
    ];

    let flows = a.solve(&b).unwrap();
    assert_eq!(flows.len(), 3);

    // Verify total flow
    let total =
        Expression::add(vec![flows[0].clone(), flows[1].clone(), flows[2].clone()]).simplify();
    assert!(
        expr_equals_integer(&total, 100),
        "Total flow should be 100, got {:?}",
        total
    );

    // Verify exact solutions: x=50, y=25, z=25
    assert!(
        expr_equals_integer(&flows[0].clone().simplify(), 50),
        "x should be 50, got {:?}",
        flows[0].clone().simplify()
    );
    assert!(
        expr_equals_integer(&flows[1].clone().simplify(), 25),
        "y should be 25, got {:?}",
        flows[1].clone().simplify()
    );
    assert!(
        expr_equals_integer(&flows[2].clone().simplify(), 25),
        "z should be 25, got {:?}",
        flows[2].clone().simplify()
    );
}

/// Chemical Equation Balancing: Combustion of Propane
///
/// C3H8 + O2 → CO2 + H2O
///
/// Let: a*C3H8 + b*O2 → c*CO2 + d*H2O
///
/// Atom balance:
///   Carbon:  3a = c
///   Hydrogen: 8a = 2d  →  4a = d
///   Oxygen:  2b = 2c + d
///
/// Setting a = 1: c = 3, d = 4, then 2b = 6 + 4 = 10 → b = 5
/// So: C3H8 + 5O2 → 3CO2 + 4H2O
#[test]
fn test_chemical_equation_balancing() {
    // Coefficients for [b, c, d] with a=1 already substituted
    let a = Matrix::from_arrays([
        [0, 1, 0],   // c = 3 (from 3a - c = 0 with a=1)
        [0, 0, 1],   // d = 4 (from 4a - d = 0 with a=1)
        [2, -2, -1], // 2b - 2c - d = 0
    ]);
    let b = vec![
        Expression::integer(3), // c = 3
        Expression::integer(4), // d = 4
        Expression::integer(0), // oxygen balance
    ];

    let coeffs = a.solve(&b).unwrap();

    // b (O2 coefficient) should be 5
    assert!(
        expr_equals_integer(&coeffs[0].clone().simplify(), 5),
        "O2 coefficient should be 5, got {:?}",
        coeffs[0].clone().simplify()
    );
    // c (CO2 coefficient) should be 3
    assert!(
        expr_equals_integer(&coeffs[1].clone().simplify(), 3),
        "CO2 coefficient should be 3, got {:?}",
        coeffs[1].clone().simplify()
    );
    // d (H2O coefficient) should be 4
    assert!(
        expr_equals_integer(&coeffs[2].clone().simplify(), 4),
        "H2O coefficient should be 4, got {:?}",
        coeffs[2].clone().simplify()
    );
}

/// Structural Engineering: Force Equilibrium in a Truss
///
/// Simplified symmetric truss with integer solution:
///   F1 - F3 = 0 (symmetry)
///   F1 + F3 = 2 (load)
///   F2 = 0 (horizontal equilibrium for symmetric case)
#[test]
fn test_structural_truss_equilibrium() {
    let a = Matrix::from_arrays([
        [1, 0, -1], // F1 - F3 = 0 (symmetry)
        [1, 0, 1],  // F1 + F3 = 2 (vertical equilibrium)
        [0, 1, 0],  // F2 = 0 (horizontal equilibrium for symmetric case)
    ]);
    let b = vec![
        Expression::integer(0),
        Expression::integer(2),
        Expression::integer(0),
    ];

    let forces = a.solve(&b).unwrap();

    // F1 should equal 1
    assert!(
        expr_equals_integer(&forces[0].clone().simplify(), 1),
        "F1 should be 1, got {:?}",
        forces[0].clone().simplify()
    );
    // F2 should equal 0
    assert!(
        expr_equals_integer(&forces[1].clone().simplify(), 0),
        "F2 should be 0, got {:?}",
        forces[1].clone().simplify()
    );
    // F3 should equal 1
    assert!(
        expr_equals_integer(&forces[2].clone().simplify(), 1),
        "F3 should be 1, got {:?}",
        forces[2].clone().simplify()
    );
}

/// Leontief Input-Output Economic Model
///
/// Three-sector economy: Agriculture (A), Manufacturing (M), Services (S)
/// Leontief equation: (I - A)x = d
///
/// Using scaled integer coefficients: (10I - 10A) * x = 10*d
#[test]
fn test_leontief_economic_model() {
    // (10I - 10A) matrix with integer coefficients
    let a = Matrix::from_arrays([
        [9, -2, -1], // 10 - 1 = 9, -2, -1
        [-3, 8, -2], // -3, 10 - 2 = 8, -2
        [-1, -3, 7], // -1, -3, 10 - 3 = 7
    ]);
    // 10 * demand vector
    let b = vec![
        Expression::integer(100), // 10 * 10
        Expression::integer(150), // 10 * 15
        Expression::integer(200), // 10 * 20
    ];

    let production = a.solve(&b).unwrap();
    assert_eq!(production.len(), 3);

    // Verify (10I - 10A)x = 10d
    let eq1 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(9), production[0].clone()]),
        Expression::mul(vec![Expression::integer(-2), production[1].clone()]),
        Expression::mul(vec![Expression::integer(-1), production[2].clone()]),
    ])
    .simplify();
    assert!(
        expr_equals_integer(&eq1, 100),
        "Leontief eq1 failed: got {:?}, expected 100",
        eq1
    );

    let eq2 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(-3), production[0].clone()]),
        Expression::mul(vec![Expression::integer(8), production[1].clone()]),
        Expression::mul(vec![Expression::integer(-2), production[2].clone()]),
    ])
    .simplify();
    assert!(
        expr_equals_integer(&eq2, 150),
        "Leontief eq2 failed: got {:?}, expected 150",
        eq2
    );

    let eq3 = Expression::add(vec![
        Expression::mul(vec![Expression::integer(-1), production[0].clone()]),
        Expression::mul(vec![Expression::integer(-3), production[1].clone()]),
        Expression::mul(vec![Expression::integer(7), production[2].clone()]),
    ])
    .simplify();
    assert!(
        expr_equals_integer(&eq3, 200),
        "Leontief eq3 failed: got {:?}, expected 200",
        eq3
    );
}

/// Mixture Problem: Blending solutions to achieve target concentration
///
/// Three solutions with different concentrations of a chemical:
///   Solution A: 10% concentration, cost $2/liter
///   Solution B: 25% concentration, cost $4/liter
///   Solution C: 50% concentration, cost $7/liter
///
/// Target: 100 liters at 30% concentration, total cost $450
#[test]
fn test_mixture_blending_problem() {
    let a = Matrix::from_arrays([
        [1, 1, 1],    // volume constraint
        [10, 25, 50], // concentration (scaled by 100)
        [2, 4, 7],    // cost
    ]);
    let b = vec![
        Expression::integer(100),  // 100 liters total
        Expression::integer(3000), // 30% concentration * 100 liters * 100
        Expression::integer(450),  // $450 total cost
    ];

    let volumes = a.solve(&b).unwrap();
    assert_eq!(volumes.len(), 3);

    // Verify volume constraint
    let total_vol = Expression::add(vec![
        volumes[0].clone(),
        volumes[1].clone(),
        volumes[2].clone(),
    ])
    .simplify();
    assert!(
        expr_equals_integer(&total_vol, 100),
        "Total volume should be 100, got {:?}",
        total_vol
    );

    // Verify concentration constraint
    let total_chem = Expression::add(vec![
        Expression::mul(vec![Expression::integer(10), volumes[0].clone()]),
        Expression::mul(vec![Expression::integer(25), volumes[1].clone()]),
        Expression::mul(vec![Expression::integer(50), volumes[2].clone()]),
    ])
    .simplify();
    assert!(
        expr_equals_integer(&total_chem, 3000),
        "Total chemical (scaled) should be 3000, got {:?}",
        total_chem
    );

    // Verify cost constraint
    let total_cost = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), volumes[0].clone()]),
        Expression::mul(vec![Expression::integer(4), volumes[1].clone()]),
        Expression::mul(vec![Expression::integer(7), volumes[2].clone()]),
    ])
    .simplify();
    assert!(
        expr_equals_integer(&total_cost, 450),
        "Total cost should be $450, got {:?}",
        total_cost
    );
}

/// Heat Transfer: Steady-state temperature distribution
///
/// 1D heat conduction in a bar with 4 internal nodes:
/// Boundary: T_left = 100°C, T_right = 0°C
///
/// At steady state, each internal node temperature is average of neighbors.
/// This is a tridiagonal system - common in numerical PDEs
#[test]
fn test_heat_transfer_steady_state() {
    // Tridiagonal matrix from discretized heat equation
    let a = Matrix::from_arrays([
        [2, -1, 0, 0],  // 2*T1 - T2 = 100
        [-1, 2, -1, 0], // -T1 + 2*T2 - T3 = 0
        [0, -1, 2, -1], // -T2 + 2*T3 - T4 = 0
        [0, 0, -1, 2],  // -T3 + 2*T4 = 0
    ]);
    let b = vec![
        Expression::integer(100), // from T_left = 100
        Expression::integer(0),
        Expression::integer(0),
        Expression::integer(0), // from T_right = 0
    ];

    let temps = a.solve(&b).unwrap();
    assert_eq!(temps.len(), 4);

    // Expected temperatures: T1=80, T2=60, T3=40, T4=20 (linear gradient)
    assert!(
        expr_equals_integer(&temps[0].clone().simplify(), 80),
        "T1 should be 80°C, got {:?}",
        temps[0].clone().simplify()
    );
    assert!(
        expr_equals_integer(&temps[1].clone().simplify(), 60),
        "T2 should be 60°C, got {:?}",
        temps[1].clone().simplify()
    );
    assert!(
        expr_equals_integer(&temps[2].clone().simplify(), 40),
        "T3 should be 40°C, got {:?}",
        temps[2].clone().simplify()
    );
    assert!(
        expr_equals_integer(&temps[3].clone().simplify(), 20),
        "T4 should be 20°C, got {:?}",
        temps[3].clone().simplify()
    );
}
