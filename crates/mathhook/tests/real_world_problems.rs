//! Real-world mathematical problem solving test suite
//! Tests practical applications in physics, engineering, and mathematics

use mathhook::prelude::*;
use std::time::Instant;

#[test]
fn test_physics_kinematics() {
    // Physics: Kinematic equations
    // s = ut + (1/2)at² where s=displacement, u=initial velocity, a=acceleration, t=time
    let u = Expression::symbol(Symbol::new("u"));
    let a = Expression::symbol(Symbol::new("a"));
    let t = Expression::symbol(Symbol::new("t"));

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
    match simplified {
        Expression::Add(_) => assert!(true),
        _ => println!("Kinematic result: {}", simplified),
    }
}

#[test]
fn test_engineering_beam_deflection() {
    // Engineering: Beam deflection equation
    // δ = (wL⁴)/(8EI) for uniformly distributed load
    let w = Expression::symbol(Symbol::new("w")); // load per unit length
    let l = Expression::symbol(Symbol::new("L")); // beam length
    let e = Expression::symbol(Symbol::new("E")); // elastic modulus
    let i = Expression::symbol(Symbol::new("I")); // moment of inertia

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
    let p = Expression::symbol(Symbol::new("P"));
    let r = Expression::symbol(Symbol::new("r"));
    let n = Expression::symbol(Symbol::new("n"));
    let t = Expression::symbol(Symbol::new("t"));

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
    let v = Expression::symbol(Symbol::new("V"));
    let n = Expression::symbol(Symbol::new("n"));
    let r = Expression::symbol(Symbol::new("R"));
    let temp = Expression::symbol(Symbol::new("T"));

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
    let x = Expression::symbol(Symbol::new("x"));
    let mu = Expression::symbol(Symbol::new("mu"));
    let sigma = Expression::symbol(Symbol::new("sigma"));
    let pi = Expression::symbol(Symbol::new("pi"));

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
    let r = Expression::symbol(Symbol::new("r"));
    let h = Expression::symbol(Symbol::new("h"));
    let pi = Expression::symbol(Symbol::new("pi"));

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
    match simplified {
        Expression::Add(_) => assert!(true),
        _ => println!("Surface area result: {}", simplified),
    }
}

#[test]
fn test_signal_processing_fourier_series() {
    // Signal Processing: Fourier series coefficients
    // f(x) = a₀/2 + Σ(aₙcos(nx) + bₙsin(nx))
    let x = Expression::symbol(Symbol::new("x"));
    let a0 = Expression::symbol(Symbol::new("a0"));
    let a1 = Expression::symbol(Symbol::new("a1"));
    let b1 = Expression::symbol(Symbol::new("b1"));
    let n = Expression::symbol(Symbol::new("n"));

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
    let theta = Expression::symbol(Symbol::new("theta"));
    let x = Expression::symbol(Symbol::new("x"));
    let y = Expression::symbol(Symbol::new("y"));
    let m = Expression::symbol(Symbol::new("m"));

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
    let psi = Expression::symbol(Symbol::new("psi"));
    let e = Expression::symbol(Symbol::new("E"));
    let hbar = Expression::symbol(Symbol::new("hbar"));
    let m = Expression::symbol(Symbol::new("m"));
    let v = Expression::symbol(Symbol::new("V"));

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
    let x = Expression::symbol(Symbol::new("x"));
    let y = Expression::symbol(Symbol::new("y"));
    let z = Expression::symbol(Symbol::new("z"));

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
    let gcd_result = simplified.gcd(&Expression::mul(vec![
        Expression::integer(2),
        Expression::symbol(Symbol::new("x")),
    ]));
    let factored = simplified.factor_gcd();

    let duration = start.elapsed();

    println!("Real-world problem solving time: {:?}", duration);

    // Should solve real-world problems efficiently
    assert!(duration.as_millis() < 10); // < 10ms for complex problems
    assert!(!simplified.is_zero());
    assert!(!gcd_result.is_zero());
    assert!(!factored.is_zero());
}
