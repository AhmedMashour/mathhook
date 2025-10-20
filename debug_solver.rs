use mathhook_core::prelude::*;
use mathhook_core::algebra::solvers::{EquationSolver, LinearSolver};

fn main() {
    println!("=== Debugging LinearSolver Regression ===\n");

    // Test 1: Simple linear (PASSING in sympy_validation)
    println!("Test 1: x = 5");
    let x = symbol!(x);
    let equation1 = Expression::equation(Expression::symbol(x.clone()), expr!(5));
    println!("  Equation: {:?}", equation1);

    // Test 2: Linear with coefficient (FAILING in sympy_validation)
    println!("\nTest 2: 2*x = 10");
    let x = symbol!(x);
    let equation2 = Expression::equation(expr!(2 * x), expr!(10));
    println!("  Equation: {:?}", equation2);

    // Now let's test what the LinearSolver does with these
    println!("\n=== Testing LinearSolver directly ===\n");

    let linear_solver = LinearSolver::new();

    // For the tests to use LinearSolver, they need to convert the equation to LHS = 0 form
    // Let's test that
    println!("Test 1 with LinearSolver:");
    // x - 5 = 0
    let eq1_lhs = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::integer(5)])
    ]);
    println!("  Equation (LHS=0 form): {:?} = 0", eq1_lhs);
    let result1 = linear_solver.solve(&eq1_lhs, &x);
    println!("  Result: {:?}", result1);

    println!("\nTest 2 with LinearSolver:");
    // 2*x - 10 = 0
    let eq2_lhs = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::integer(10)])
    ]);
    println!("  Equation (LHS=0 form): {:?} = 0", eq2_lhs);
    let result2 = linear_solver.solve(&eq2_lhs, &x);
    println!("  Result: {:?}", result2);

    // Test the check_commutativity function on these equations
    println!("\n=== Testing check_commutativity ===\n");
    // We can't call it directly as it's private, but we can infer from behavior
}
