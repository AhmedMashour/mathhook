use mathhook_core::algebra::solvers::{EquationSolver, PolynomialSolver, SolverResult};
use mathhook_core::core::Expression;
use mathhook_core::symbol;

fn main() {
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::integer(-8),
    ]);

    println!("Equation: {:?}", equation);

    let solver = PolynomialSolver::new();
    let result = solver.solve(&equation, &x);

    println!("Result: {:?}", result);
}
