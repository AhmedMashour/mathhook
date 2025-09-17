//! Complex number operation integration tests
//!
//! Tests for complex number support including:
//! - Complex arithmetic
//! - Polar form conversions
//! - Conjugate properties
//! - Complex predicates

use mathhook_core::{expr, symbol, ComplexOperations, Expression, MathConstant};

#[test]
fn test_complex_from_real_imag() {
    let z = Expression::complex(expr!(3), expr!(4));
    assert_eq!(z.real(), expr!(3));
    assert_eq!(z.imag(), expr!(4));
}

#[test]
fn test_complex_purely_real_extraction() {
    let z = Expression::complex(expr!(5), expr!(0));
    assert_eq!(z.real(), expr!(5));
    assert_eq!(z.imag(), expr!(0));
}

#[test]
fn test_complex_purely_imaginary_extraction() {
    let z = Expression::complex(expr!(0), expr!(3));
    assert_eq!(z.real(), expr!(0));
    assert_eq!(z.imag(), expr!(3));
}

#[test]
fn test_imaginary_unit_creation() {
    let i = Expression::i();
    match i {
        Expression::Constant(c) => {
            assert_eq!(c, MathConstant::I);
        }
        _ => panic!("Expected constant i"),
    }
}

#[test]
fn test_complex_addition() {
    let z1 = Expression::complex(expr!(3), expr!(4));
    let z2 = Expression::complex(expr!(1), expr!(2));
    let sum = z1.complex_add(&z2);

    if let Expression::Complex(data) = sum {
        assert_eq!(data.real, expr!(4));
        assert_eq!(data.imag, expr!(6));
    } else {
        panic!("Expected complex result");
    }
}

#[test]
fn test_complex_subtraction() {
    let z1 = Expression::complex(expr!(5), expr!(3));
    let z2 = Expression::complex(expr!(2), expr!(1));
    let diff = z1.complex_subtract(&z2);

    if let Expression::Complex(data) = diff {
        assert_eq!(data.real, expr!(3));
        assert_eq!(data.imag, expr!(2));
    } else {
        panic!("Expected complex result");
    }
}

#[test]
fn test_complex_multiplication() {
    let z1 = Expression::complex(expr!(3), expr!(4));
    let z2 = Expression::complex(expr!(1), expr!(2));
    let product = z1.complex_multiply(&z2);

    if let Expression::Complex(data) = product {
        assert_eq!(data.real, expr!(-5));
        assert_eq!(data.imag, expr!(10));
    } else {
        panic!("Expected complex result");
    }
}

#[test]
fn test_complex_division() {
    let z1 = Expression::complex(expr!(3), expr!(4));
    let z2 = Expression::complex(expr!(1), expr!(2));
    let quotient = z1.complex_divide(&z2);

    match quotient {
        Expression::Complex(_) => {}
        _ => panic!("Expected complex result"),
    }
}

#[test]
fn test_complex_conjugate() {
    let z = Expression::complex(expr!(3), expr!(4));
    let conj = z.conjugate();

    if let Expression::Complex(data) = conj {
        assert_eq!(data.real, expr!(3));
        assert_eq!(data.imag, expr!(-4));
    } else {
        panic!("Expected complex result");
    }
}

#[test]
fn test_complex_modulus() {
    let z = Expression::complex(expr!(3), expr!(4));
    let modulus = z.abs();

    match modulus {
        Expression::Function { .. } => {}
        _ => panic!("Expected function expression for abs"),
    }
}

#[test]
fn test_complex_argument() {
    let z = Expression::complex(expr!(1), expr!(1));
    let arg = z.arg();

    match arg {
        Expression::Function { .. } => {}
        _ => panic!("Expected function expression for arg"),
    }
}

#[test]
fn test_to_polar_form() {
    let z = Expression::complex(expr!(3), expr!(4));
    let (magnitude, angle) = z.to_polar();

    match magnitude {
        Expression::Function { .. } => {}
        _ => panic!("Expected function expression for magnitude"),
    }

    match angle {
        Expression::Function { .. } => {}
        _ => panic!("Expected function expression for angle"),
    }
}

#[test]
fn test_from_polar_form() {
    let magnitude = expr!(5);
    let angle = expr!(0);
    let z = Expression::from_polar(magnitude, angle);

    match z {
        Expression::Complex(_) => {}
        _ => panic!("Expected complex result from polar conversion"),
    }
}

#[test]
fn test_conjugate_twice_returns_original() {
    let z = Expression::complex(expr!(3), expr!(4));
    let conjugate = z.conjugate();
    let double_conjugate = conjugate.conjugate();

    if let Expression::Complex(data) = double_conjugate {
        assert_eq!(data.real, expr!(3));
        assert_eq!(data.imag, expr!(4));
    } else {
        panic!("Expected complex result");
    }
}

#[test]
fn test_z_times_conjugate() {
    let z = Expression::complex(expr!(3), expr!(4));
    let conj = z.conjugate();
    let product = z.complex_multiply(&conj);

    if let Expression::Complex(data) = product {
        assert_eq!(data.real, expr!(25));
        assert_eq!(data.imag, expr!(0));
    } else {
        panic!("Expected complex result");
    }
}

#[test]
fn test_is_real() {
    let z = Expression::complex(expr!(5), expr!(0));
    assert!(z.is_real());
}

#[test]
fn test_is_pure_imaginary() {
    let z = Expression::complex(expr!(0), expr!(5));
    assert!(z.is_pure_imaginary());
}

#[test]
fn test_is_imaginary_general() {
    let z = Expression::complex(expr!(3), expr!(4));
    assert!(z.is_imaginary());
    assert!(!z.is_pure_imaginary());
}

#[test]
fn test_complex_with_symbolic_parts() {
    let x = symbol!(x);
    let y = symbol!(y);

    let z = Expression::complex(Expression::symbol(x.clone()), Expression::symbol(y.clone()));

    assert_eq!(z.real(), Expression::symbol(x));
    assert_eq!(z.imag(), Expression::symbol(y));
}

#[test]
fn test_complex_addition_symbolic() {
    let x = symbol!(x);
    let y = symbol!(y);
    let a = symbol!(a);
    let b = symbol!(b);

    let z1 = Expression::complex(Expression::symbol(x), Expression::symbol(y));
    let z2 = Expression::complex(Expression::symbol(a), Expression::symbol(b));
    let result = z1.complex_add(&z2);

    if let Expression::Complex(data) = result {
        match (&data.real, &data.imag) {
            (Expression::Add(real_terms), Expression::Add(imag_terms)) => {
                assert_eq!(real_terms.len(), 2);
                assert_eq!(imag_terms.len(), 2);
            }
            _ => panic!("Expected addition expressions for real and imaginary parts"),
        }
    } else {
        panic!("Expected complex result");
    }
}

#[test]
fn test_complex_zero() {
    let z = Expression::complex(expr!(0), expr!(0));
    assert_eq!(z.real(), expr!(0));
    assert_eq!(z.imag(), expr!(0));
}

#[test]
fn test_complex_multiply_by_zero() {
    let z1 = Expression::complex(expr!(3), expr!(4));
    let z2 = Expression::complex(expr!(0), expr!(0));
    let result = z1.complex_multiply(&z2);

    if let Expression::Complex(data) = result {
        assert_eq!(data.real, expr!(0));
        assert_eq!(data.imag, expr!(0));
    } else {
        panic!("Expected complex result");
    }
}

#[test]
fn test_complex_multiply_by_i() {
    let z = Expression::complex(expr!(3), expr!(4));
    let i = Expression::complex(expr!(0), expr!(1));
    let result = z.complex_multiply(&i);

    if let Expression::Complex(data) = result {
        assert_eq!(data.real, expr!(-4));
        assert_eq!(data.imag, expr!(3));
    } else {
        panic!("Expected complex result");
    }
}

#[test]
fn test_complex_subtract_self() {
    let z1 = Expression::complex(expr!(3), expr!(4));
    let z2 = Expression::complex(expr!(3), expr!(4));
    let result = z1.complex_subtract(&z2);

    if let Expression::Complex(data) = result {
        assert_eq!(data.real, expr!(0));
        assert_eq!(data.imag, expr!(0));
    } else {
        panic!("Expected complex result");
    }
}

#[test]
fn test_simplify_complex_pure_real() {
    let z = Expression::complex(expr!(3), expr!(0));
    let result = Expression::simplify_complex(&z);
    assert_eq!(result, expr!(3));
}

#[test]
fn test_simplify_complex_pure_imaginary() {
    let z = Expression::complex(expr!(0), expr!(4));
    let result = Expression::simplify_complex(&z);
    assert_eq!(result, Expression::mul(vec![expr!(4), Expression::i()]));
}
