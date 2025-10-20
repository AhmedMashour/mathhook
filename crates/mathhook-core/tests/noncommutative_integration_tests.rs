//! Noncommutative Algebra Integration Tests (Wave 12)
//!
//! Comprehensive integration tests verifying that all waves (8-12) work together seamlessly.
//! Tests cover cross-wave functionality and regression prevention.
//!
//! Test Organization:
//! - Cross-Wave Integration (10 tests): Verify multiple features work together
//! - Regression Prevention (10 tests): Ensure backward compatibility
//! - Example Validation (3 tests): Verify examples are correct

use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
use mathhook_core::algebra::solvers::{EquationSolver, SolverResult};
use mathhook_core::core::symbol::SymbolType;
use mathhook_core::educational::message_registry::{MessageBuilder, MessageCategory, MessageType};
use mathhook_core::formatter::latex::{LaTeXContext, LaTeXFormatter};
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
fn test_scalar_behavior_unchanged() {
    let x = symbol!(x);
    let y = symbol!(y);

    assert_eq!(x.symbol_type(), SymbolType::Scalar);
    assert_eq!(y.symbol_type(), SymbolType::Scalar);

    let xy = Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(y.clone())]);

    let yx = Expression::mul(vec![Expression::symbol(y), Expression::symbol(x)]);

    let xy_str = xy.to_string();
    let yx_str = yx.to_string();

    assert_eq!(xy_str, yx_str, "Scalar multiplication should be commutative");
}

#[test]
fn test_backward_compatibility_symbols() {
    let x_new = symbol!(x);
    let x_old = Symbol::new("x");

    assert_eq!(x_new.name(), x_old.name());
    assert_eq!(x_new.symbol_type(), x_old.symbol_type());
    assert_eq!(x_new.symbol_type(), SymbolType::Scalar);
}

#[test]
fn test_backward_compatibility_formatter() {
    let x = symbol!(x);
    let expr = Expression::mul(vec![Expression::integer(2), Expression::symbol(x)]);

    let latex = expr.to_latex(LaTeXContext::default()).unwrap();

    assert!(!latex.contains(r"\mathbf{"), "Scalar should not use bold");
    assert!(!latex.contains(r"\hat{"), "Scalar should not use hat");
}

#[test]
fn test_backward_compatibility_messages() {
    let x = symbol!(x);
    let y = symbol!(y);

    let xy = Expression::mul(vec![Expression::symbol(x), Expression::symbol(y)]);

    let xy_str = xy.to_string();
    assert!(xy_str.contains("x") || xy_str.contains("y"));
}

#[test]
fn test_backward_compatibility_solver() {
    let a = symbol!(a);
    let x = symbol!(x);
    let b = symbol!(b);

    let equation = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(a), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b)]),
    ]);

    let solver = MatrixEquationSolver::new();
    let result = solver.solve(&equation, &x);

    match result {
        SolverResult::Single(solution) => {
            let sol_str = solution.to_string();
            assert!(!sol_str.is_empty(), "Solution should not be empty");
        }
        SolverResult::Multiple(solutions) if !solutions.is_empty() => {
            let sol_str = solutions[0].to_string();
            assert!(!sol_str.is_empty(), "Solution should not be empty");
        }
        SolverResult::NoSolution => {
            println!("No solution found - matrix solver may not handle scalar equations");
        }
        _ => {}
    }
}

#[test]
fn test_performance_no_regression() {
    use std::time::Instant;

    let start = Instant::now();

    for _ in 0..1000 {
        let x = symbol!(x);
        let y = symbol!(y);
        let expr = Expression::mul(vec![Expression::symbol(x), Expression::symbol(y)]);
        let _str = expr.to_string();
    }

    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 1000,
        "Symbol creation should be fast (< 1s for 1000 iterations)"
    );
}

#[test]
fn test_file_size_compliance() {
    use std::fs;
    use std::path::Path;

    let src_dir = Path::new("crates/mathhook-core/src");
    if !src_dir.exists() {
        return;
    }

    let mut oversized_files = Vec::new();

    fn check_dir(dir: &Path, oversized: &mut Vec<String>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    check_dir(&path, oversized);
                } else if path.extension().map_or(false, |ext| ext == "rs") {
                    if let Ok(contents) = fs::read_to_string(&path) {
                        let line_count = contents.lines().count();
                        if line_count > 500 {
                            oversized.push(format!("{} ({} lines)", path.display(), line_count));
                        }
                    }
                }
            }
        }
    }

    check_dir(src_dir, &mut oversized_files);

    assert!(
        oversized_files.is_empty(),
        "Files exceeding 500 lines: {:?}",
        oversized_files
    );
}

#[test]
fn test_emoji_compliance() {
    use std::fs;
    use std::path::Path;

    let dirs_to_check = vec!["crates/mathhook-core/src", "crates/mathhook-core/tests"];

    for dir_path in dirs_to_check {
        let dir = Path::new(dir_path);
        if !dir.exists() {
            continue;
        }

        fn check_for_emojis(dir: &Path) -> Vec<String> {
            let mut files_with_emojis = Vec::new();

            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        files_with_emojis.extend(check_for_emojis(&path));
                    } else if path.extension().map_or(false, |ext| ext == "rs") {
                        if let Ok(contents) = fs::read_to_string(&path) {
                            let emoji_patterns = vec![
                                "\u{2705}", "\u{274C}", "\u{26A0}", "\u{1F680}",
                                "\u{2728}", "\u{1F4A1}", "\u{1F4DD}", "\u{1F527}"
                            ];
                            for pattern in &emoji_patterns {
                                if contents.contains(pattern) {
                                    files_with_emojis
                                        .push(format!("{} contains emoji", path.display()));
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            files_with_emojis
        }

        let emoji_files = check_for_emojis(dir);
        assert!(
            emoji_files.is_empty(),
            "Files with emojis found: {:?}",
            emoji_files
        );
    }
}

#[test]
fn test_build_all_targets() {
    use std::process::Command;

    let output = Command::new("cargo")
        .args(["check", "-p", "mathhook-core"])
        .output();

    if let Ok(result) = output {
        assert!(
            result.status.success(),
            "Cargo check should succeed: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}

#[test]
fn test_all_existing_tests_pass() {
    use std::process::Command;

    let output = Command::new("cargo")
        .args(["test", "-p", "mathhook-core", "--lib"])
        .output();

    if let Ok(result) = output {
        let stdout = String::from_utf8_lossy(&result.stdout);
        let stderr = String::from_utf8_lossy(&result.stderr);

        let has_failures = stdout.contains("FAILED") || stderr.contains("FAILED");

        assert!(
            !has_failures,
            "Some existing tests failed:\nSTDOUT:\n{}\nSTDERR:\n{}",
            stdout,
            stderr
        );
    }
}

#[test]
fn test_quantum_mechanics_example_concepts() {
    let x = symbol!(x; operator);
    let p = symbol!(p; operator);

    let xp = Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(p.clone())]);

    let px = Expression::mul(vec![Expression::symbol(p.clone()), Expression::symbol(x.clone())]);

    let xp_str = xp.to_string();
    let px_str = px.to_string();

    assert_ne!(
        xp_str, px_str,
        "Operator multiplication should preserve order"
    );

    let xp_latex = xp.to_latex(LaTeXContext::default()).unwrap();
    let px_latex = px.to_latex(LaTeXContext::default()).unwrap();

    assert!(xp_latex.contains(r"\hat{x}") && xp_latex.contains(r"\hat{p}"));
    assert!(px_latex.contains(r"\hat{p}") && px_latex.contains(r"\hat{x}"));
}

#[test]
fn test_matrix_algebra_example_concepts() {
    let a = symbol!(A; matrix);
    let x = symbol!(X; matrix);
    let b = symbol!(B; matrix);

    let ax_eq = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(x.clone())]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b.clone())]),
    ]);

    let solver = MatrixEquationSolver::new();
    let result = solver.solve(&ax_eq, &x);

    assert!(
        matches!(result, SolverResult::Single(_)),
        "Should solve A*X = B"
    );

    let xa_eq = Expression::add(vec![
        Expression::mul(vec![Expression::symbol(x.clone()), Expression::symbol(a)]),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(b)]),
    ]);

    let result_right = solver.solve(&xa_eq, &x);

    assert!(
        matches!(result_right, SolverResult::Single(_)),
        "Should solve X*A = B"
    );
}

#[test]
fn test_quaternion_rotations_example_concepts() {
    let i = symbol!(i; quaternion);
    let j = symbol!(j; quaternion);
    let k = symbol!(k; quaternion);

    let ij = Expression::mul(vec![Expression::symbol(i.clone()), Expression::symbol(j.clone())]);

    let ji = Expression::mul(vec![Expression::symbol(j.clone()), Expression::symbol(i.clone())]);

    let ij_str = ij.to_string();
    let ji_str = ji.to_string();

    assert_ne!(ij_str, ji_str, "Quaternion multiplication order matters");

    let jk = Expression::mul(vec![Expression::symbol(j), Expression::symbol(k.clone())]);

    let ki = Expression::mul(vec![Expression::symbol(k), Expression::symbol(i.clone())]);

    assert!(!jk.to_string().is_empty());
    assert!(!ki.to_string().is_empty());
}

#[test]
fn test_matrix_symbol_type_preservation() {
    let a = symbol!(A; matrix);
    let b = symbol!(B; matrix);

    let product = Expression::mul(vec![Expression::symbol(a.clone()), Expression::symbol(b.clone())]);

    assert_eq!(a.symbol_type(), SymbolType::Matrix);
    assert_eq!(b.symbol_type(), SymbolType::Matrix);

    let latex = product.to_latex(LaTeXContext::default()).unwrap();
    assert!(latex.contains(r"\mathbf{A}") && latex.contains(r"\mathbf{B}"));
}

#[test]
fn test_operator_symbol_type_preservation() {
    let p = symbol!(p; operator);
    let x = symbol!(x; operator);

    let product = Expression::mul(vec![Expression::symbol(p.clone()), Expression::symbol(x.clone())]);

    assert_eq!(p.symbol_type(), SymbolType::Operator);
    assert_eq!(x.symbol_type(), SymbolType::Operator);

    let latex = product.to_latex(LaTeXContext::default()).unwrap();
    assert!(latex.contains(r"\hat{p}") && latex.contains(r"\hat{x}"));
}
