//! Noncommutative Algebra Examples
//!
//! This module demonstrates real-world applications of MathHook's noncommutative
//! algebra features across three domains:
//!
//! 1. Quantum Mechanics - Operator algebra and commutator relations
//! 2. Matrix Algebra - Linear systems with left and right division
//! 3. Quaternion Rotations - 3D rotations using quaternion algebra
//!
//! Each example shows:
//! - How to create symbols with appropriate types using symbol! macro
//! - How order matters in noncommutative algebra
//! - How to use educational features for step-by-step explanations
//! - How to get LaTeX output with proper notation

use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
use mathhook_core::algebra::solvers::{EquationSolver, SolverResult};
use mathhook_core::educational::message_registry::{MessageBuilder, MessageCategory, MessageType};
use mathhook_core::formatter::latex::{LaTeXContext, LaTeXFormatter};
use mathhook_core::{symbol, Expression};

fn main() {
    println!("==============================================");
    println!("MathHook Noncommutative Algebra Examples");
    println!("==============================================\n");

    example_quantum_mechanics();
    println!("\n");

    example_matrix_algebra();
    println!("\n");

    example_quaternion_rotations();
    println!("\n");

    println!("==============================================");
    println!("All examples completed successfully!");
    println!("==============================================");
}

/// Example 1: Quantum Mechanics - Operator Algebra
///
/// Demonstrates operator algebra in quantum mechanics, showing how position
/// and momentum operators don't commute, leading to the Heisenberg uncertainty principle.
///
/// Educational Value:
/// - Shows students why [x, p] = iℏ (canonical commutation relation)
/// - Demonstrates how operator order matters in quantum mechanics
/// - Illustrates solving eigenvalue equations (H*ψ = E*ψ)
///
/// Example Output:
/// ```text
/// Commutator [x, p] = xp - px
/// LaTeX: [\hat{x}, \hat{p}] = \hat{x}\hat{p} - \hat{p}\hat{x}
/// Educational: Order matters because operators represent physical measurements
/// ```
fn example_quantum_mechanics() {
    println!("----------------------------------------------");
    println!("EXAMPLE 1: Quantum Mechanics - Operator Algebra");
    println!("----------------------------------------------");

    println!("\nPart A: Canonical Commutation Relations");
    println!("========================================\n");

    let x = symbol!(x; operator);
    let p = symbol!(p; operator);

    println!("Position operator: x (type: operator)");
    println!("Momentum operator: p (type: operator)");
    println!();

    let xp = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(p.clone()),
    ]);

    let px = Expression::mul(vec![
        Expression::symbol(p.clone()),
        Expression::symbol(x.clone()),
    ]);

    println!("Order matters in quantum mechanics:");
    println!("  x*p = (position then momentum)");
    println!("  p*x = (momentum then position)");
    println!();

    let commutator = Expression::add(vec![
        xp.clone(),
        Expression::mul(vec![Expression::integer(-1), px.clone()]),
    ]);

    let commutator_latex = commutator.to_latex(LaTeXContext::default()).unwrap();

    println!("Commutator [x, p] = xp - px");
    println!("LaTeX: {}", commutator_latex);
    println!();

    println!("Educational Explanation:");
    println!("------------------------");
    println!("In quantum mechanics, operators represent physical observables.");
    println!("The commutator [x, p] measures how much the order of measurements matters.");
    println!("The canonical commutation relation [x, p] = iℏ leads to the");
    println!("Heisenberg uncertainty principle: Δx·Δp ≥ ℏ/2");

    println!("\nPart B: Hamiltonian Eigenvalue Equation");
    println!("========================================\n");

    let h = symbol!(H; operator);
    let psi = symbol!(psi; operator);
    let e_val = symbol!(E; operator);

    println!("Hamiltonian operator: H");
    println!("Wave function: ψ");
    println!("Energy eigenvalue: E");
    println!();

    let h_psi = Expression::mul(vec![
        Expression::symbol(h.clone()),
        Expression::symbol(psi.clone()),
    ]);

    let eigenvalue_eq = Expression::add(vec![
        h_psi.clone(),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::symbol(e_val.clone()),
        ]),
    ]);

    println!("Time-independent Schrödinger equation:");
    println!("  H*ψ = E");
    println!();

    let solver = MatrixEquationSolver::new();
    let result = solver.solve(&eigenvalue_eq, &psi);

    match result {
        SolverResult::Single(solution) => {
            let solution_latex = solution.to_latex(LaTeXContext::default()).unwrap();
            println!("Solution for ψ:");
            println!("  ψ = H^(-1)*E");
            println!("  LaTeX: {}", solution_latex);
            println!();
            println!("Educational Note:");
            println!("This is the formal solution. In practice, we solve for energy");
            println!("eigenvalues E given a specific Hamiltonian H.");
        }
        _ => println!("Formal solution requires specific Hamiltonian"),
    }

    println!("\nKey Takeaway:");
    println!("-------------");
    println!("Operator algebra is fundamental to quantum mechanics.");
    println!("The noncommutative nature of operators reflects the physical");
    println!("reality that measurement order affects quantum states.");
}

/// Example 2: Matrix Algebra - Linear Systems
///
/// Demonstrates solving matrix equations using left and right division.
/// Shows how A*X = B (left division) differs from X*A = B (right division).
///
/// Educational Value:
/// - Clarifies the difference between left and right matrix multiplication
/// - Shows when matrix order matters in linear algebra
/// - Illustrates practical applications in solving systems of equations
///
/// Example Output:
/// ```text
/// Left division: A*X = B → X = A^(-1)*B
/// Right division: X*A = B → X = B*A^(-1)
/// LaTeX: \mathbf{A}\mathbf{X} = \mathbf{B}
/// ```
fn example_matrix_algebra() {
    println!("----------------------------------------------");
    println!("EXAMPLE 2: Matrix Algebra - Linear Systems");
    println!("----------------------------------------------");

    println!("\nPart A: Left Division (A*X = B)");
    println!("================================\n");

    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    println!("Matrix symbols (type: matrix):");
    println!("  A: Coefficient matrix");
    println!("  X: Unknown matrix");
    println!("  B: Right-hand side");
    println!();

    let ax_eq = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(x.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let equation_latex = ax_eq.to_latex(LaTeXContext::default()).unwrap();

    println!("Equation: A*X = B");
    println!("LaTeX: {}", equation_latex);
    println!();

    let solver = MatrixEquationSolver::new();
    let result = solver.solve(&ax_eq, &x);

    match result {
        SolverResult::Single(solution) => {
            let solution_latex = solution.to_latex(LaTeXContext::default()).unwrap();
            println!("Solution:");
            println!("  X = A^(-1)*B");
            println!("  LaTeX: {}", solution_latex);
            println!();
            println!("Educational Explanation:");
            println!("------------------------");
            println!("In left division, we multiply both sides by A^(-1) on the LEFT:");
            println!("  A*X = B");
            println!("  A^(-1)*(A*X) = A^(-1)*B");
            println!("  (A^(-1)*A)*X = A^(-1)*B");
            println!("  I*X = A^(-1)*B");
            println!("  X = A^(-1)*B");
            println!();
            println!("Note: Order matters! We must multiply on the left to cancel A.");
        }
        _ => println!("Solution requires invertible matrix A"),
    }

    println!("\nPart B: Right Division (X*A = B)");
    println!("=================================\n");

    let xa_eq = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(a.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let xa_eq_latex = xa_eq.to_latex(LaTeXContext::default()).unwrap();
    println!("Equation: X*A = B");
    println!("LaTeX: {}", xa_eq_latex);
    println!();

    let result_right = solver.solve(&xa_eq, &x);

    match result_right {
        SolverResult::Single(solution) => {
            let solution_latex = solution.to_latex(LaTeXContext::default()).unwrap();
            println!("Solution:");
            println!("  X = B*A^(-1)");
            println!("  LaTeX: {}", solution_latex);
            println!();
            println!("Educational Explanation:");
            println!("------------------------");
            println!("In right division, we multiply both sides by A^(-1) on the RIGHT:");
            println!("  X*A = B");
            println!("  (X*A)*A^(-1) = B*A^(-1)");
            println!("  X*(A*A^(-1)) = B*A^(-1)");
            println!("  X*I = B*A^(-1)");
            println!("  X = B*A^(-1)");
            println!();
            println!("Note: We must multiply on the right to cancel A correctly.");
        }
        _ => println!("Solution requires invertible matrix A"),
    }

    println!("\nPart C: Comparison - Why Order Matters");
    println!("=======================================\n");

    println!("Left division:  A*X = B  →  X = A^(-1)*B");
    println!("Right division: X*A = B  →  X = B*A^(-1)");
    println!();
    println!("These give DIFFERENT solutions because matrix multiplication");
    println!("is not commutative: A^(-1)*B ≠ B*A^(-1) in general");
    println!();

    let msg = MessageBuilder::new(
        MessageCategory::NoncommutativeAlgebra,
        MessageType::NoncommutativeWarning,
        0,
    )
    .with_substitution("symbol", "A")
    .with_substitution("symbol_type", "Matrix")
    .with_substitution("other", "B")
    .build();

    if let Some(message) = msg {
        println!("Educational Message:");
        println!("{}", message.description);
        println!();
    }

    println!("Key Takeaway:");
    println!("-------------");
    println!("Matrix equation solving requires careful attention to order.");
    println!("The solver automatically determines whether to use left or right");
    println!("division based on the equation structure.");
}

/// Example 3: Quaternion Rotations - 3D Graphics
///
/// Demonstrates quaternion algebra for 3D rotations, showing how
/// quaternion multiplication order affects rotation results.
///
/// Educational Value:
/// - Explains quaternion basis: i, j, k with i² = j² = k² = ijk = -1
/// - Shows multiplication rules: i*j = k, j*i = -k (order matters!)
/// - Illustrates rotation formula: v' = q*v*conj(q)
///
/// Example Output:
/// ```text
/// Quaternion multiplication: i*j = k, but j*i = -k
/// Order determines rotation direction (left-handed vs right-handed)
/// Rotation formula: v' = q*v*conj(q)
/// ```
fn example_quaternion_rotations() {
    println!("----------------------------------------------");
    println!("EXAMPLE 3: Quaternion Rotations - 3D Graphics");
    println!("----------------------------------------------");

    println!("\nPart A: Quaternion Basis Elements");
    println!("==================================\n");

    let i = symbol!(i; quaternion);
    let j = symbol!(j; quaternion);

    println!("Quaternion basis (type: quaternion):");
    println!("  i: First imaginary unit");
    println!("  j: Second imaginary unit");
    println!("  k: Third imaginary unit");
    println!();

    println!("Fundamental quaternion relations:");
    println!("  i² = j² = k² = -1");
    println!("  ijk = -1");
    println!();

    println!("Part B: Multiplication Rules - Order Matters!");
    println!("==============================================\n");

    let ij = Expression::mul(vec![
        Expression::symbol(i.clone()),
        Expression::symbol(j.clone()),
    ]);

    let ji = Expression::mul(vec![
        Expression::symbol(j.clone()),
        Expression::symbol(i.clone()),
    ]);

    println!("Forward multiplication: i*j");
    println!("  Result: k (by definition)");
    println!("  LaTeX: {}", ij.to_latex(LaTeXContext::default()).unwrap());
    println!();

    println!("Reverse multiplication: j*i");
    println!("  Result: -k (opposite sign!)");
    println!("  LaTeX: {}", ji.to_latex(LaTeXContext::default()).unwrap());
    println!();

    println!("Educational Explanation:");
    println!("------------------------");
    println!("The quaternion multiplication rules follow a right-hand rule:");
    println!("  i*j = k   (thumb: i, index: j, middle: k)");
    println!("  j*k = i   (cyclic permutation)");
    println!("  k*i = j   (cyclic permutation)");
    println!();
    println!("Reversing the order negates the result:");
    println!("  j*i = -k  (opposite direction)");
    println!("  k*j = -i");
    println!("  i*k = -j");
    println!();
    println!("This noncommutativity is essential for representing 3D rotations!");

    println!("\nPart C: Rotation Formula");
    println!("=========================\n");

    println!("Rotation quaternion: q");
    println!("Vector to rotate: v (represented as quaternion)");
    println!();

    println!("Quaternion rotation formula:");
    println!("  v' = q*v*conj(q)");
    println!();
    println!("Where:");
    println!("  q: Unit quaternion representing rotation");
    println!("  v: Pure quaternion (vector with zero scalar part)");
    println!("  conj(q): Quaternion conjugate of q");
    println!("  v': Rotated vector");
    println!();

    println!("Example: Rotate vector by 90° around z-axis");
    println!("  q = cos(45°) + sin(45°)*k");
    println!("  q ≈ 0.707 + 0.707k");
    println!();
    println!("  For v = i (x-axis unit vector):");
    println!("  v' = q*i*conj(q) = j (y-axis unit vector)");
    println!();

    println!("Educational Explanation:");
    println!("------------------------");
    println!("Quaternion rotations have several advantages:");
    println!("1. No gimbal lock (unlike Euler angles)");
    println!("2. Smooth interpolation (SLERP)");
    println!("3. Compact representation (4 numbers vs 9 for matrix)");
    println!("4. Efficient composition (quaternion multiplication)");
    println!();
    println!("The formula v' = q*v*conj(q) rotates v by the angle");
    println!("and axis encoded in q. The order is critical:");
    println!("  q on the left, conj(q) on the right");
    println!();
    println!("Reversing to conj(q)*v*q would rotate in the opposite direction!");

    println!("\nPart D: Solving for Rotation Quaternion");
    println!("========================================\n");

    let q_var = symbol!(Q; quaternion);
    let v_in = symbol!(V_in; quaternion);
    let v_out = symbol!(V_out; quaternion);

    println!("Given: Input vector V_in and desired output vector V_out");
    println!("Find: Rotation quaternion Q such that V_out = Q*V_in*conj(Q)");
    println!();

    let rotation_eq = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(q_var.clone()),
            Expression::symbol(v_in.clone()),
        ]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::symbol(v_out.clone()),
        ]),
    ]);

    let rotation_latex = rotation_eq.to_latex(LaTeXContext::default()).unwrap();
    println!("Simplified equation (assuming unit quaternions): Q*V_in = V_out");
    println!("LaTeX: {}", rotation_latex);
    println!();

    let solver = MatrixEquationSolver::new();
    let result = solver.solve(&rotation_eq, &q_var);

    match result {
        SolverResult::Single(solution) => {
            let solution_latex = solution.to_latex(LaTeXContext::default()).unwrap();
            println!("Formal solution:");
            println!("  Q = V_out*V_in^(-1)");
            println!("  LaTeX: {}", solution_latex);
            println!();
            println!("In practice, quaternion rotations are typically computed");
            println!("from axis-angle representation or by interpolating between");
            println!("known orientations.");
        }
        _ => println!("Full rotation quaternion requires additional constraints"),
    }

    println!("\nKey Takeaway:");
    println!("-------------");
    println!("Quaternions provide a powerful noncommutative algebra for 3D rotations.");
    println!("Understanding that i*j ≠ j*i is essential for correctly applying");
    println!("rotation formulas in computer graphics, robotics, and aerospace.");
}
