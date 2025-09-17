/// Tests for fraction simplification in linear equation solver
///
/// This test suite verifies that the linear solver correctly simplifies
/// fractional solutions to their lowest terms and represents integer
/// solutions as integers rather than fractions.
use mathhook_core::algebra::solvers::{linear::LinearSolver, EquationSolver, SolverResult};
use mathhook_core::core::{Expression, Number};
use mathhook_core::symbol;
use num_bigint::BigInt;

#[test]
fn test_fraction_already_simplified() {
    // Test: 2x = 3 should give x = 3/2 (already in lowest terms)
    let solver = LinearSolver::new_fast();
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::integer(-3),
    ]);

    match solver.solve(&equation, &x) {
        SolverResult::Single(solution) => {
            if let Expression::Number(Number::Rational(r)) = solution {
                assert_eq!(r.numer(), &BigInt::from(3));
                assert_eq!(r.denom(), &BigInt::from(2));
            } else {
                panic!("Expected rational number, got: {:?}", solution);
            }
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_fraction_needs_simplification() {
    // Test: 4x = 6 should give x = 3/2 (simplified from 6/4)
    let solver = LinearSolver::new_fast();
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(4), Expression::symbol(x.clone())]),
        Expression::integer(-6),
    ]);

    match solver.solve(&equation, &x) {
        SolverResult::Single(solution) => {
            if let Expression::Number(Number::Rational(r)) = solution {
                // Should be 3/2, not 6/4
                assert_eq!(r.numer(), &BigInt::from(3));
                assert_eq!(r.denom(), &BigInt::from(2));
            } else {
                panic!("Expected rational number, got: {:?}", solution);
            }
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_integer_solution_not_fraction() {
    // Test: 5x = 10 should give x = 2 (integer, not 10/5)
    let solver = LinearSolver::new_fast();
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(5), Expression::symbol(x.clone())]),
        Expression::integer(-10),
    ]);

    match solver.solve(&equation, &x) {
        SolverResult::Single(solution) => {
            if let Expression::Number(Number::Integer(i)) = solution {
                assert_eq!(i, 2);
            } else {
                panic!("Expected integer number, got: {:?}", solution);
            }
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_large_fraction_simplification() {
    // Test: 12x = 18 should give x = 3/2 (simplified from 18/12)
    let solver = LinearSolver::new_fast();
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(12), Expression::symbol(x.clone())]),
        Expression::integer(-18),
    ]);

    match solver.solve(&equation, &x) {
        SolverResult::Single(solution) => {
            if let Expression::Number(Number::Rational(r)) = solution {
                // Should be 3/2, not 18/12 or 9/6 or 6/4
                assert_eq!(r.numer(), &BigInt::from(3));
                assert_eq!(r.denom(), &BigInt::from(2));
            } else {
                panic!("Expected rational number, got: {:?}", solution);
            }
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_negative_fraction_simplification() {
    // Test: -4x = 6 should give x = -3/2 (simplified from -6/4)
    let solver = LinearSolver::new_fast();
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(-4), Expression::symbol(x.clone())]),
        Expression::integer(-6),
    ]);

    match solver.solve(&equation, &x) {
        SolverResult::Single(solution) => {
            if let Expression::Number(Number::Rational(r)) = solution {
                // Should be -3/2, not -6/4
                assert_eq!(r.numer(), &BigInt::from(-3));
                assert_eq!(r.denom(), &BigInt::from(2));
            } else {
                panic!("Expected rational number, got: {:?}", solution);
            }
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_coprime_numerator_denominator() {
    // Test: 7x = 11 should give x = 11/7 (already coprime)
    let solver = LinearSolver::new_fast();
    let x = symbol!(x);
    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::integer(7), Expression::symbol(x.clone())]),
        Expression::integer(-11),
    ]);

    match solver.solve(&equation, &x) {
        SolverResult::Single(solution) => {
            if let Expression::Number(Number::Rational(r)) = solution {
                assert_eq!(r.numer(), &BigInt::from(11));
                assert_eq!(r.denom(), &BigInt::from(7));
            } else {
                panic!("Expected rational number, got: {:?}", solution);
            }
        }
        _ => panic!("Expected single solution"),
    }
}

#[test]
fn test_zero_solution_as_integer() {
    // Test: 5x = 0 should give x = 0 (integer zero, not 0/5)
    let solver = LinearSolver::new_fast();
    let x = symbol!(x);
    let equation = Expression::mul(vec![Expression::integer(5), Expression::symbol(x.clone())]);

    match solver.solve(&equation, &x) {
        SolverResult::Single(solution) => {
            if let Expression::Number(Number::Integer(i)) = solution {
                assert_eq!(i, 0);
            } else {
                panic!("Expected integer zero, got: {:?}", solution);
            }
        }
        _ => panic!("Expected single solution"),
    }
}
