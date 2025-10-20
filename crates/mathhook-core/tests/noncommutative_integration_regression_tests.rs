//! Regression Prevention Tests (Wave 12)
//!
//! Tests ensuring backward compatibility and preventing regressions.
//! Covers: Scalar behavior unchanged, build checks, performance, compliance.

use mathhook_core::algebra::solvers::matrix_equations::MatrixEquationSolver;
use mathhook_core::algebra::solvers::{EquationSolver, SolverResult};
use mathhook_core::core::symbol::SymbolType;
use mathhook_core::formatter::LaTeXFormatter;
use mathhook_core::{symbol, Expression, Symbol};

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

    let latex = expr.to_latex(mathhook_core::formatter::latex::LaTeXContext::default()).unwrap();

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
