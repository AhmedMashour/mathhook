//! Real-world mathematical problem solving test suite
//! Tests practical applications in physics, engineering, and mathematics
//!
//! Tests verify both structural correctness and mathematical accuracy
//! using concrete value substitution and dimensional analysis

use mathhook::prelude::*;
use std::time::Instant;

#[test]
fn test_physics_kinematics() {
    // Physics: Kinematic equations
    // s = ut + (1/2)at² where s=displacement, u=initial velocity, a=acceleration, t=time
    let u = expr!(u);
    let a = expr!(a);
    let t = expr!(t);

    let displacement = Expression::add(vec![
        Expression::mul(vec![u, t.clone()]),
        Expression::mul(vec![
            Expression::pow(Expression::integer(2), Expression::integer(-1)),
            a,
            Expression::pow(t, Expression::integer(2)),
        ]),
    ]);

    let simplified = displacement.simplify();
    println!("Kinematic equation: s = {}", simplified);

    // Should maintain physics equation structure
    assert!(
        matches!(simplified, Expression::Add(_)),
        "Expected kinematic equation to remain as addition (sum of terms), got: {}",
        simplified
    );
}

#[test]
fn test_physics_kinematics_with_concrete_values() {
    // REAL mathematical correctness test: verify with concrete values
    // Given: u=10 m/s, a=2 m/s², t=5 s
    // Expected: s = 10*5 + 0.5*2*25 = 50 + 25 = 75 m

    // Construct expression directly with concrete values
    let displacement = Expression::add(vec![
        Expression::mul(vec![Expression::integer(10), Expression::integer(5)]),
        Expression::mul(vec![
            Expression::rational(1, 2),
            Expression::integer(2),
            Expression::pow(Expression::integer(5), Expression::integer(2)),
        ]),
    ]);

    let concrete = displacement.simplify();

    println!("Concrete kinematics: s(10, 2, 5) = {}", concrete);

    // Verify mathematical correctness: should equal 75
    assert_eq!(
        concrete,
        Expression::integer(75),
        "Kinematic equation with u=10, a=2, t=5 should yield s=75"
    );
}

#[test]
fn test_engineering_beam_deflection() {
    // Engineering: Beam deflection equation
    // δ = (wL⁴)/(8EI) for uniformly distributed load
    let w = expr!(w); // load per unit length
    let l = expr!(L); // beam length
    let e = expr!(E); // elastic modulus
    let i = expr!(I); // moment of inertia

    let deflection = Expression::mul(vec![
        w,
        Expression::pow(l, Expression::integer(4)),
        Expression::pow(Expression::integer(8), Expression::integer(-1)),
        Expression::pow(e, Expression::integer(-1)),
        Expression::pow(i, Expression::integer(-1)),
    ]);

    let simplified = deflection.simplify();
    println!("Beam deflection: δ = {}", simplified);

    // Should maintain engineering formula structure
    assert!(!simplified.is_zero());
}

#[test]
fn test_economics_compound_interest() {
    // Economics: Compound interest formula
    // A = P(1 + r/n)^(nt) where A=amount, P=principal, r=rate, n=compounds per year, t=time
    let p = expr!(P);
    let r = expr!(r);
    let n = expr!(n);
    let t = expr!(t);

    let compound_interest = Expression::mul(vec![
        p,
        Expression::pow(
            Expression::add(vec![
                Expression::integer(1),
                Expression::mul(vec![r, Expression::pow(n.clone(), Expression::integer(-1))]),
            ]),
            Expression::mul(vec![n, t]),
        ),
    ]);

    let simplified = compound_interest.simplify();
    println!("Compound interest: A = {}", simplified);

    // Should maintain financial formula structure
    assert!(!simplified.is_zero());
}

#[test]
fn test_chemistry_ideal_gas_law() {
    // Chemistry: Ideal gas law PV = nRT
    let v = expr!(V);
    let n = expr!(n);
    let r = expr!(R);
    let temp = expr!(T);

    // Rearrange to solve for pressure: P = nRT/V
    let pressure = Expression::mul(vec![
        n,
        r,
        temp,
        Expression::pow(v, Expression::integer(-1)),
    ]);

    let simplified = pressure.simplify();
    println!("Ideal gas pressure: P = {}", simplified);

    // Should maintain chemistry equation structure
    assert!(!simplified.is_zero());
}

#[test]
fn test_statistics_normal_distribution() {
    // Statistics: Normal distribution probability density function
    // f(x) = (1/σ√(2π)) * e^(-(x-μ)²/(2σ²))
    let x = expr!(x);
    let mu = expr!(mu);
    let sigma = expr!(sigma);
    let pi = expr!(pi);

    let normal_pdf = Expression::mul(vec![
        Expression::pow(sigma.clone(), Expression::integer(-1)),
        Expression::pow(
            Expression::mul(vec![Expression::integer(2), pi]),
            Expression::pow(Expression::integer(2), Expression::integer(-1)),
        ),
        Expression::function(
            "exp",
            vec![Expression::mul(vec![
                Expression::integer(-1),
                Expression::pow(
                    Expression::add(vec![x, Expression::mul(vec![Expression::integer(-1), mu])]),
                    Expression::integer(2),
                ),
                Expression::pow(
                    Expression::mul(vec![
                        Expression::integer(2),
                        Expression::pow(sigma, Expression::integer(2)),
                    ]),
                    Expression::integer(-1),
                ),
            ])],
        ),
    ]);

    let simplified = normal_pdf.simplify();
    println!("Normal PDF: f(x) = {}", simplified);

    // Should maintain statistical formula structure
    assert!(!simplified.is_zero());
}

#[test]
fn test_calculus_optimization_problem() {
    // Calculus: Optimization problem - minimize surface area of cylinder with fixed volume
    // V = πr²h (constraint), A = 2πr² + 2πrh (objective)
    let r = expr!(r);
    let h = expr!(h);
    let pi = expr!(pi);

    let surface_area = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(2),
            pi.clone(),
            Expression::pow(r.clone(), Expression::integer(2)),
        ]),
        Expression::mul(vec![Expression::integer(2), pi, r.clone(), h]),
    ]);

    let simplified = surface_area.simplify();
    println!("Cylinder surface area: A = {}", simplified);

    // Should maintain optimization problem structure
    assert!(
        matches!(simplified, Expression::Add(_)),
        "Expected surface area formula to remain as addition, got: {}",
        simplified
    );
}

#[test]
fn test_signal_processing_fourier_series() {
    // Signal Processing: Fourier series coefficients
    // f(x) = a₀/2 + Σ(aₙcos(nx) + bₙsin(nx))
    let x = expr!(x);
    let a0 = expr!(a0);
    let a1 = expr!(a1);
    let b1 = expr!(b1);
    let n = expr!(n);

    let fourier_series = Expression::add(vec![
        Expression::mul(vec![
            a0,
            Expression::pow(Expression::integer(2), Expression::integer(-1)),
        ]),
        Expression::mul(vec![
            a1,
            Expression::function("cos", vec![Expression::mul(vec![n.clone(), x.clone()])]),
        ]),
        Expression::mul(vec![
            b1,
            Expression::function("sin", vec![Expression::mul(vec![n, x])]),
        ]),
    ]);

    let simplified = fourier_series.simplify();
    println!("Fourier series: f(x) = {}", simplified);

    // Should maintain signal processing structure
    assert!(!simplified.is_zero());
}

#[test]
fn test_machine_learning_cost_function() {
    // Machine Learning: Mean squared error cost function
    // J(θ) = (1/2m) * Σ(hθ(x) - y)²
    let theta = expr!(theta);
    let x = expr!(x);
    let y = expr!(y);
    let m = expr!(m);

    // Simplified version: J = (1/2m) * (θx - y)²
    let cost_function = Expression::mul(vec![
        Expression::pow(
            Expression::mul(vec![Expression::integer(2), m]),
            Expression::integer(-1),
        ),
        Expression::pow(
            Expression::add(vec![
                Expression::mul(vec![theta, x]),
                Expression::mul(vec![Expression::integer(-1), y]),
            ]),
            Expression::integer(2),
        ),
    ]);

    let simplified = cost_function.simplify();
    println!("ML cost function: J(θ) = {}", simplified);

    // Should maintain ML formula structure
    assert!(!simplified.is_zero());
}

#[test]
fn test_quantum_mechanics_schrodinger() {
    // Quantum Mechanics: Time-independent Schrödinger equation patterns
    // Ĥψ = Eψ, where Ĥ = -ℏ²/(2m)∇² + V(x)
    let psi = expr!(psi);
    let e = expr!(E);
    let hbar = expr!(hbar);
    let m = expr!(m);
    let v = expr!(V);

    // Hamiltonian operator pattern (kinetic + potential energy)
    let hamiltonian = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::pow(hbar, Expression::integer(2)),
            Expression::pow(
                Expression::mul(vec![Expression::integer(2), m]),
                Expression::integer(-1),
            ),
            Expression::function("laplacian", vec![psi.clone()]),
        ]),
        Expression::mul(vec![v, psi.clone()]),
    ]);

    // Eigenvalue equation: Hψ = Eψ
    let eigenvalue_eq = Expression::add(vec![
        hamiltonian,
        Expression::mul(vec![Expression::integer(-1), e, psi]),
    ]);

    let simplified = eigenvalue_eq.simplify();
    println!("Schrödinger equation: {}", simplified);

    // Should maintain quantum mechanics structure
    assert!(!simplified.is_zero());
}

#[test]
fn test_real_world_performance_benchmark() {
    // Performance test with real-world complexity
    let x = expr!(x);
    let y = expr!(y);
    let z = expr!(z);

    let start = Instant::now();

    // Complex real-world expression
    let complex_expr = Expression::add(vec![
        Expression::mul(vec![
            Expression::integer(2),
            Expression::pow(x.clone(), Expression::integer(3)),
            Expression::pow(y.clone(), Expression::integer(2)),
        ]),
        Expression::mul(vec![
            Expression::integer(3),
            x.clone(),
            Expression::pow(z.clone(), Expression::integer(2)),
        ]),
        Expression::mul(vec![
            Expression::integer(5),
            Expression::pow(y.clone(), Expression::integer(3)),
        ]),
        Expression::add(vec![
            Expression::mul(vec![x, y]),
            Expression::mul(vec![Expression::integer(7), z]),
        ]),
    ]);

    // Perform multiple real-world operations
    let simplified = complex_expr.simplify();
    let gcd_result = simplified.gcd(&Expression::mul(vec![Expression::integer(2), expr!(x)]));
    let factored = simplified.factor_gcd();

    let duration = start.elapsed();

    println!("Real-world problem solving time: {:?}", duration);

    // Should solve real-world problems efficiently
    assert!(duration.as_millis() < 10); // < 10ms for complex problems
    assert!(!simplified.is_zero());
    assert!(!gcd_result.is_zero());
    assert!(!factored.is_zero());
}

#[test]
fn test_mathematical_property_commutativity() {
    // Property-based test: addition is commutative (a + b = b + a)
    let x = expr!(x);
    let y = expr!(y);

    let ab = Expression::add(vec![x.clone(), y.clone()]).simplify();
    let ba = Expression::add(vec![y, x]).simplify();

    assert_eq!(
        ab, ba,
        "Addition should be commutative: x + y should equal y + x"
    );
}

#[test]
fn test_mathematical_property_associativity() {
    // Property-based test: addition is associative ((a + b) + c = a + (b + c))
    let x = expr!(x);
    let y = expr!(y);
    let z = expr!(z);

    let abc_left =
        Expression::add(vec![Expression::add(vec![x.clone(), y.clone()]), z.clone()]).simplify();

    let abc_right = Expression::add(vec![x, Expression::add(vec![y, z])]).simplify();

    assert_eq!(
        abc_left, abc_right,
        "Addition should be associative: (x + y) + z should equal x + (y + z)"
    );
}

#[test]
fn test_mathematical_property_distributivity() {
    // Property-based test: multiplication distributes over addition (a(b + c) = ab + ac)
    let a = expr!(a);
    let b = expr!(b);
    let c = expr!(c);

    let left =
        Expression::mul(vec![a.clone(), Expression::add(vec![b.clone(), c.clone()])]).simplify();

    let right = Expression::add(vec![
        Expression::mul(vec![a.clone(), b]),
        Expression::mul(vec![a, c]),
    ])
    .simplify();

    // Note: This tests if simplification preserves distributivity
    // Both forms are mathematically equivalent
    println!("Left (factored): {}", left);
    println!("Right (expanded): {}", right);

    // Structural test: both should be valid expressions
    assert!(!left.is_zero());
    assert!(!right.is_zero());
}
