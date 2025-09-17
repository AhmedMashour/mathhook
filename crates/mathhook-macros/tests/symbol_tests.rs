//! Comprehensive tests for symbol!() and symbols!() macros

use mathhook_core::Symbol;
use mathhook_macros::{symbol, symbols};

// ============================================================================
// symbol!() Basic Functionality (10 tests)
// ============================================================================

#[test]
fn test_symbol_scalar_identifier() {
    let s = symbol!(x);
    assert_eq!(s, Symbol::scalar("x"));
}

#[test]
fn test_symbol_scalar_string_literal() {
    let s = symbol!("y");
    assert_eq!(s, Symbol::scalar("y"));
}

#[test]
fn test_symbol_matrix_identifier() {
    let s = symbol!(A; matrix);
    assert_eq!(s, Symbol::matrix("A"));
}

#[test]
fn test_symbol_matrix_string_literal() {
    let s = symbol!("B"; matrix);
    assert_eq!(s, Symbol::matrix("B"));
}

#[test]
fn test_symbol_operator_identifier() {
    let s = symbol!(p; operator);
    assert_eq!(s, Symbol::operator("p"));
}

#[test]
fn test_symbol_operator_string_literal() {
    let s = symbol!("x_op"; operator);
    assert_eq!(s, Symbol::operator("x_op"));
}

#[test]
fn test_symbol_quaternion_identifier() {
    let s = symbol!(i; quaternion);
    assert_eq!(s, Symbol::quaternion("i"));
}

#[test]
fn test_symbol_quaternion_string_literal() {
    let s = symbol!("j"; quaternion);
    assert_eq!(s, Symbol::quaternion("j"));
}

#[test]
fn test_symbol_identifier_with_underscore() {
    let s = symbol!(alpha_1);
    assert_eq!(s, Symbol::scalar("alpha_1"));
}

#[test]
fn test_symbol_identifier_with_number() {
    let s = symbol!(x2);
    assert_eq!(s, Symbol::scalar("x2"));
}

// ============================================================================
// symbols!() Basic Functionality (10 tests)
// ============================================================================

#[test]
fn test_symbols_single_scalar() {
    let syms = symbols![x];
    assert_eq!(syms, vec![Symbol::scalar("x")]);
}

#[test]
fn test_symbols_multiple_scalars() {
    let syms = symbols![x, y, z];
    assert_eq!(
        syms,
        vec![
            Symbol::scalar("x"),
            Symbol::scalar("y"),
            Symbol::scalar("z")
        ]
    );
}

#[test]
fn test_symbols_single_matrix() {
    let syms = symbols![A => matrix];
    assert_eq!(syms, vec![Symbol::matrix("A")]);
}

#[test]
fn test_symbols_multiple_matrices() {
    let syms = symbols![A, B, C => matrix];
    assert_eq!(
        syms,
        vec![
            Symbol::matrix("A"),
            Symbol::matrix("B"),
            Symbol::matrix("C")
        ]
    );
}

#[test]
fn test_symbols_single_operator() {
    let syms = symbols![p => operator];
    assert_eq!(syms, vec![Symbol::operator("p")]);
}

#[test]
fn test_symbols_multiple_operators() {
    let syms = symbols![p, x, H => operator];
    assert_eq!(
        syms,
        vec![
            Symbol::operator("p"),
            Symbol::operator("x"),
            Symbol::operator("H")
        ]
    );
}

#[test]
fn test_symbols_single_quaternion() {
    let syms = symbols![i => quaternion];
    assert_eq!(syms, vec![Symbol::quaternion("i")]);
}

#[test]
fn test_symbols_multiple_quaternions() {
    let syms = symbols![i, j, k => quaternion];
    assert_eq!(
        syms,
        vec![
            Symbol::quaternion("i"),
            Symbol::quaternion("j"),
            Symbol::quaternion("k")
        ]
    );
}

#[test]
fn test_symbols_vector_length() {
    let syms = symbols![a, b, c, d, e];
    assert_eq!(syms.len(), 5);
}

#[test]
fn test_symbols_can_be_iterated() {
    let syms = symbols![x, y, z];
    let names: Vec<&str> = syms.iter().map(|s| s.name()).collect();
    assert_eq!(names, vec!["x", "y", "z"]);
}

// ============================================================================
// Edge Cases (10 tests)
// ============================================================================

#[test]
fn test_symbol_name_with_unicode() {
    let s = symbol!("θ");
    assert_eq!(s, Symbol::scalar("θ"));
}

#[test]
fn test_symbol_name_greek_letter() {
    let s = symbol!("alpha");
    assert_eq!(s, Symbol::scalar("alpha"));
}

#[test]
fn test_symbols_two_elements() {
    let syms = symbols![a, b];
    assert_eq!(syms.len(), 2);
}

#[test]
fn test_symbols_ten_elements() {
    let syms = symbols![a, b, c, d, e, f, g, h, i, j];
    assert_eq!(syms.len(), 10);
}

#[test]
fn test_symbol_scalar_default() {
    let default_scalar = symbol!(x);
    let explicit_scalar = symbol!(x; scalar);
    assert_eq!(default_scalar, explicit_scalar);
}

#[test]
fn test_symbols_scalar_default() {
    let default_syms = symbols![x, y];
    let explicit_syms = symbols![x, y => scalar];
    assert_eq!(default_syms, explicit_syms);
}

#[test]
fn test_symbol_common_physics_symbols() {
    let mass = symbol!(m);
    let velocity = symbol!(v);
    let force = symbol!(F);
    assert_eq!(mass, Symbol::scalar("m"));
    assert_eq!(velocity, Symbol::scalar("v"));
    assert_eq!(force, Symbol::scalar("F"));
}

#[test]
fn test_symbol_quantum_operators() {
    let position = symbol!(x; operator);
    let momentum = symbol!(p; operator);
    let hamiltonian = symbol!(H; operator);
    assert_eq!(position, Symbol::operator("x"));
    assert_eq!(momentum, Symbol::operator("p"));
    assert_eq!(hamiltonian, Symbol::operator("H"));
}

#[test]
fn test_symbol_quaternion_basis() {
    let i = symbol!(i; quaternion);
    let j = symbol!(j; quaternion);
    let k = symbol!(k; quaternion);
    assert_eq!(i, Symbol::quaternion("i"));
    assert_eq!(j, Symbol::quaternion("j"));
    assert_eq!(k, Symbol::quaternion("k"));
}

#[test]
fn test_symbols_mixed_case() {
    let syms = symbols![x, Y, Z];
    assert_eq!(
        syms,
        vec![
            Symbol::scalar("x"),
            Symbol::scalar("Y"),
            Symbol::scalar("Z")
        ]
    );
}
