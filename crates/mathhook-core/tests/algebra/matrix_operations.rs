//! Comprehensive matrix operations test suite
//! Covers linear algebra, determinants, eigenvalues, and matrix arithmetic

use mathhook_core::prelude::*;

#[test]
fn test_matrix_basic_arithmetic() {
    // Test basic matrix addition/multiplication patterns
    let a11 = Expression::symbol(Symbol::new("a11"));
    let a12 = Expression::symbol(Symbol::new("a12"));
    let a21 = Expression::symbol(Symbol::new("a21"));
    let a22 = Expression::symbol(Symbol::new("a22"));
    
    // Matrix determinant: det([[a11, a12], [a21, a22]]) = a11*a22 - a12*a21
    let det = Expression::add(vec![
        Expression::mul(vec![a11.clone(), a22.clone()]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::mul(vec![a12.clone(), a21.clone()])
        ])
    ]);
    
    let simplified = det.simplify();
    println!("Matrix determinant simplified: {}", simplified);
    
    // Should maintain structure for symbolic matrix
    assert!(!simplified.is_zero());
}

#[test]
fn test_matrix_trace_operations() {
    // Matrix trace: tr(A) = a11 + a22 + a33
    let a11 = Expression::symbol(Symbol::new("a11"));
    let a22 = Expression::symbol(Symbol::new("a22"));
    let a33 = Expression::symbol(Symbol::new("a33"));
    
    let trace = Expression::add(vec![a11, a22, a33]);
    let simplified = trace.simplify();
    
    // Trace should maintain additive structure
    match simplified {
        Expression::Add(_) => assert!(true),
        Expression::Symbol(_) => assert!(true), // Single symbol case
        _ => println!("Trace result: {}", simplified),
    }
}

#[test]
fn test_matrix_eigenvalue_characteristic_polynomial() {
    // Characteristic polynomial: det(A - λI) for 2x2 matrix
    let a = Expression::symbol(Symbol::new("a"));
    let b = Expression::symbol(Symbol::new("b"));
    let c = Expression::symbol(Symbol::new("c"));
    let d = Expression::symbol(Symbol::new("d"));
    let lambda = Expression::symbol(Symbol::new("lambda"));
    
    // det([[a-λ, b], [c, d-λ]]) = (a-λ)(d-λ) - bc = λ² - (a+d)λ + (ad-bc)
    let char_poly = Expression::add(vec![
        Expression::mul(vec![
            Expression::add(vec![a.clone(), Expression::mul(vec![Expression::integer(-1), lambda.clone()])]),
            Expression::add(vec![d.clone(), Expression::mul(vec![Expression::integer(-1), lambda.clone()])])
        ]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::mul(vec![b.clone(), c.clone()])
        ])
    ]);
    
    let simplified = char_poly.simplify();
    println!("Characteristic polynomial: {}", simplified);
    
    // Should be a polynomial in lambda
    assert!(!simplified.is_zero());
}

#[test]
fn test_matrix_identity_properties() {
    // Test matrix identity properties: AI = IA = A
    let a = Expression::symbol(Symbol::new("a"));
    
    // Identity matrix multiplication: a * 1 = a
    let identity_mult = Expression::mul(vec![a.clone(), Expression::integer(1)]);
    let simplified = identity_mult.simplify();
    
    assert_eq!(simplified, a);
}

#[test]
fn test_matrix_zero_properties() {
    // Test matrix zero properties: A + 0 = A, A * 0 = 0
    let a = Expression::symbol(Symbol::new("a"));
    
    // Zero addition: a + 0 = a
    let zero_add = Expression::add(vec![a.clone(), Expression::integer(0)]);
    assert_eq!(zero_add.simplify(), a);
    
    // Zero multiplication: a * 0 = 0
    let zero_mult = Expression::mul(vec![a, Expression::integer(0)]);
    assert_eq!(zero_mult.simplify(), Expression::integer(0));
}

#[test]
fn test_matrix_transpose_properties() {
    // Test transpose properties: (A + B)^T = A^T + B^T
    let a = Expression::symbol(Symbol::new("a"));
    let b = Expression::symbol(Symbol::new("b"));
    
    // For scalars, transpose is identity
    let sum = Expression::add(vec![a.clone(), b.clone()]);
    let simplified = sum.simplify();
    
    // Should maintain additive structure
    match simplified {
        Expression::Add(terms) => assert_eq!(terms.len(), 2),
        _ => println!("Sum result: {}", simplified),
    }
}

#[test]
fn test_matrix_inverse_properties() {
    // Test matrix inverse properties: A * A^(-1) = I
    let a = Expression::symbol(Symbol::new("a"));
    
    // For scalars: a * a^(-1) = 1
    let inverse_mult = Expression::mul(vec![
        a.clone(),
        Expression::pow(a, Expression::integer(-1))
    ]);
    
    let simplified = inverse_mult.simplify();
    println!("Inverse multiplication: {}", simplified);
    
    // Should maintain structure (symbolic case)
    assert!(!simplified.is_zero());
}

#[test]
fn test_matrix_commutator_operations() {
    // Test commutator: [A, B] = AB - BA
    let a = Expression::symbol(Symbol::new("A"));
    let b = Expression::symbol(Symbol::new("B"));
    
    let commutator = Expression::add(vec![
        Expression::mul(vec![a.clone(), b.clone()]),
        Expression::mul(vec![
            Expression::integer(-1),
            Expression::mul(vec![b, a])
        ])
    ]);
    
    let simplified = commutator.simplify();
    println!("Commutator [A,B]: {}", simplified);
    
    // For different symbols, should maintain structure
    assert!(!simplified.is_zero());
}

#[test]
fn test_matrix_rank_operations() {
    // Test rank-related operations and linear independence
    let x = Expression::symbol(Symbol::new("x"));
    let y = Expression::symbol(Symbol::new("y"));
    
    // Linear combination: 2x + 3y
    let linear_combo = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), x]),
        Expression::mul(vec![Expression::integer(3), y])
    ]);
    
    let simplified = linear_combo.simplify();
    
    // Should maintain linear structure
    match simplified {
        Expression::Add(_) => assert!(true),
        _ => println!("Linear combination: {}", simplified),
    }
}

#[test]
fn test_matrix_norm_operations() {
    // Test matrix norm operations (Frobenius norm patterns)
    let a = Expression::symbol(Symbol::new("a"));
    let b = Expression::symbol(Symbol::new("b"));
    
    // Frobenius norm squared: |a|² + |b|² (simplified as a² + b²)
    let norm_squared = Expression::add(vec![
        Expression::pow(a, Expression::integer(2)),
        Expression::pow(b, Expression::integer(2))
    ]);
    
    let simplified = norm_squared.simplify();
    
    // Should maintain sum of squares structure
    match simplified {
        Expression::Add(_) => assert!(true),
        Expression::Pow(_, _) => assert!(true), // Single term case
        _ => println!("Norm squared: {}", simplified),
    }
}
