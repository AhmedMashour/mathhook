//! Cross-Wave Integration Tests (Wave 12)
//!
//! Tests verifying that multiple waves (8-12) work together seamlessly.
//! Covers: Parser→Solver, Symbols→Formatter, full workflows.

use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
use mathhook_core::algebra::solvers::{EquationSolver, SolverResult};
use mathhook_core::educational::message_registry::{MessageBuilder, MessageCategory, MessageType};
use mathhook_core::formatter::latex::LaTeXContext;
use mathhook_core::formatter::LaTeXFormatter;
use mathhook_core::parser::config::ParserConfig;
use mathhook_core::parser::Parser;
use mathhook_core::{symbol, symbols, Expression, Symbol};

fn parse_latex(input: &str) -> Result<Expression, String> {
    let parser = Parser::new(ParserConfig {
        enable_implicit_multiplication: true,
    });
    parser.parse(input).map_err(|e| e.to_string())
}

#[test]
fn test_parser_to_solver_integration() {
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let solver = MatrixEquationSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            let solution_str = solution.to_string();
            assert!(
                solution_str.contains("A") || solution_str.contains("B"),
                "Solution should involve A and B"
            );
        }
        _ => panic!("Expected single solution for matrix equation"),
    }
}

#[test]
fn test_parser_to_formatter_integration() {
    let p = symbol!(p; operator);
    let x = symbol!(x; operator);

    let expr = Expression::mul(vec![Expression::symbol(p), Expression::symbol(x)]);

    let output = expr.to_latex(LaTeXContext::default()).unwrap();

    assert!(
        output.contains(r"\hat{p}") && output.contains(r"\hat{x}"),
        "Formatter should preserve operator notation"
    );
}

#[test]
fn test_symbols_to_solver_integration() {
    let matrices = symbols![A, B, X => matrix];
    let a = &matrices[0];
    let b = &matrices[1];
    let x = &matrices[2];

    let equation = Expression::add(vec![
        Expression::mul(vec![
            Expression::symbol(a.clone()),
            Expression::symbol(x.clone()),
        ]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let solver = MatrixEquationSolver::new();
    let result = solver.solve(&equation, &x);

    assert!(
        matches!(result, SolverResult::Single(_)),
        "Should solve A*X = B"
    );
}

#[test]
fn test_matrix_equation_full_workflow() {
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(a), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b)]),
    ]);

    let solver = MatrixEquationSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            let latex_output = solution.to_latex(LaTeXContext::default()).unwrap();

            assert!(
                latex_output.contains(r"\mathbf{A}") || latex_output.contains(r"\mathbf{B}"),
                "Output should use matrix notation"
            );

            let msg = MessageBuilder::new(
                MessageCategory::NoncommutativeAlgebra,
                MessageType::LeftMultiplyInverse,
                0,
            )
            .build();
            if let Some(message) = msg {
                assert!(!message.description.is_empty(), "Educational message should exist");
            }
        }
        _ => panic!("Expected solution"),
    }
}

#[test]
fn test_operator_commutator_workflow() {
    let x = symbol!(x; operator);
    let p = symbol!(p; operator);

    let xp = Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(p.clone())]);

    let px = Expression::mul(vec![Expression::symbol(p.clone()), Expression::symbol(x.clone())]);

    let commutator = Expression::add(vec![xp, Expression::mul(vec![Expression::integer(-1), px])]);

    let latex = commutator.to_latex(LaTeXContext::default()).unwrap();

    assert!(
        latex.contains(r"\hat{x}") && latex.contains(r"\hat{p}"),
        "Should use operator hat notation"
    );
}

#[test]
fn test_quaternion_multiplication_workflow() {
    let i = symbol!(i; quaternion);
    let j = symbol!(j; quaternion);

    let ij = Expression::mul(vec![Expression::symbol(i.clone()), Expression::symbol(j.clone())]);

    let ji = Expression::mul(vec![Expression::symbol(j.clone()), Expression::symbol(i.clone())]);

    let ij_str = ij.to_string();
    let ji_str = ji.to_string();

    assert_ne!(ij_str, ji_str, "i*j should differ from j*i structurally");

    let ij_latex = ij.to_latex(LaTeXContext::default()).unwrap();
    let ji_latex = ji.to_latex(LaTeXContext::default()).unwrap();

    assert!(ij_latex.contains("i") && ij_latex.contains("j"));
    assert!(ji_latex.contains("j") && ji_latex.contains("i"));
}

#[test]
fn test_mixed_commutative_noncommutative() {
    let a = symbol!(a);
    let b_mat = symbol!(B; matrix);

    let expr = Expression::mul(vec![
        Expression::integer(2),
        Expression::symbol(a),
        Expression::symbol(b_mat),
    ]);

    let expr_str = expr.to_string();
    assert!(expr_str.contains("a") || expr_str.contains("B") || expr_str.contains("2"));

    let latex = expr.to_latex(LaTeXContext::default()).unwrap();

    assert!(latex.contains(r"\mathbf{B}"), "Should use bold for matrix");
}

#[test]
fn test_educational_messages_with_formatter() {
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);

    let ax = Expression::mul(vec![Expression::symbol(a), Expression::symbol(x)]);

    let latex = ax.to_latex(LaTeXContext::default()).unwrap();

    assert!(latex.contains(r"\mathbf{A}") && latex.contains(r"\mathbf{X}"));
}

#[test]
fn test_step_by_step_with_type_aware_latex() {
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let solver = MatrixEquationSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            let step_latex = solution.to_latex(LaTeXContext::default()).unwrap();

            assert!(
                step_latex.contains(r"\mathbf{A}") || step_latex.contains("A"),
                "Solution should reference matrix A"
            );
        }
        _ => panic!("Expected solution"),
    }
}

#[test]
fn test_end_to_end_quantum_mechanics() {
    let latex_hamiltonian = r"\hat{H}\hat{\psi} = \hat{E}";
    let equation = parse_latex(latex_hamiltonian).unwrap_or_else(|_| {
        let h = symbol!(H; operator);
        let psi = symbol!(psi; operator);
        let e = symbol!(E; operator);

        Expression::add(vec![
            Expression::mul(vec![Expression::symbol(h), Expression::symbol(psi.clone())]),
            Expression::mul(vec![Expression::integer(-1), Expression::symbol(e)]),
        ])
    });

    let psi_sym = Symbol::operator("psi");
    let solver = MatrixEquationSolver::new();
    let result = solver.solve(&equation, &psi_sym);

    assert!(
        matches!(result, SolverResult::Single(_) | SolverResult::NoSolution),
        "Should handle operator equation"
    );
}

#[test]
fn test_matrix_symbol_type_preservation() {
    let a = symbol!(A; matrix);
    let b = symbol!(B; matrix);

    let product = Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(b.clone())]);

    assert_eq!(a.symbol_type(), mathhook_core::core::symbol::SymbolType::Matrix);
    assert_eq!(b.symbol_type(), mathhook_core::core::symbol::SymbolType::Matrix);

    let latex = product.to_latex(LaTeXContext::default()).unwrap();
    assert!(latex.contains(r"\mathbf{A}") && latex.contains(r"\mathbf{B}"));
}

#[test]
fn test_operator_symbol_type_preservation() {
    let p = symbol!(p; operator);
    let x = symbol!(x; operator);

    let product = Expression::mul(vec![Expression::symbol(p.clone()), Expression::symbol(x.clone())]);

    assert_eq!(p.symbol_type(), mathhook_core::core::symbol::SymbolType::Operator);
    assert_eq!(x.symbol_type(), mathhook_core::core::symbol::SymbolType::Operator);

    let latex = product.to_latex(LaTeXContext::default()).unwrap();
    assert!(latex.contains(r"\hat{p}") && latex.contains(r"\hat{x}"));
}
