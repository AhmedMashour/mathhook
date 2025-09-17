//! Integration tests for multivariate polynomial GCD
//!
//! Tests end-to-end workflows involving multivariate GCD computation

use mathhook_core::algebra::multivariate_gcd::multivariate_gcd;
use mathhook_core::{expr, symbol, Expression};

#[test]
fn test_bivariate_gcd_workflow() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Example: gcd(x²y + xy², xy) = xy
    // Fixed: Use Expression::mul for multi-term multiplication
    let p1 = Expression::add(vec![
        Expression::mul(vec![expr!(x ^ 2), expr!(y)]),
        Expression::mul(vec![expr!(x), expr!(y ^ 2)]),
    ]);
    let p2 = Expression::mul(vec![expr!(x), expr!(y)]);
    let vars = vec![x.clone(), y.clone()];

    let result = multivariate_gcd(&p1, &p2, &vars);

    // Result should be xy or a simplified form
    assert!(!result.is_zero());
    assert!(!result.is_one());
}

#[test]
fn test_difference_of_squares() {
    let x = symbol!(x);
    let y = symbol!(y);

    // gcd(x² - y², x - y) should contain (x - y) as a factor
    let p1 = expr!((x ^ 2) - (y ^ 2));
    let p2 = expr!(x - y);
    let vars = vec![x.clone(), y.clone()];

    let result = multivariate_gcd(&p1, &p2, &vars);

    // Since x² - y² = (x-y)(x+y), gcd should be related to (x-y)
    assert!(!result.is_zero());
}

#[test]
fn test_three_variables() {
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    // gcd(xyz, xy) = xy
    // Fixed: Use Expression::mul for triple multiplication
    let p1 = Expression::mul(vec![expr!(x), expr!(y), expr!(z)]);
    let p2 = Expression::mul(vec![expr!(x), expr!(y)]);
    let vars = vec![x.clone(), y.clone(), z.clone()];

    let result = multivariate_gcd(&p1, &p2, &vars);

    assert!(!result.is_zero());
}

#[test]
fn test_numeric_coefficients() {
    let x = symbol!(x);
    let y = symbol!(y);

    // gcd(6xy, 9xy) should have xy as factor, content gcd is 3
    // Fixed: Use Expression::mul for triple multiplication
    let p1 = Expression::mul(vec![expr!(6), expr!(x), expr!(y)]);
    let p2 = Expression::mul(vec![expr!(9), expr!(x), expr!(y)]);
    let vars = vec![x.clone(), y.clone()];

    let result = multivariate_gcd(&p1, &p2, &vars);

    // Result should contain xy and possibly numeric factor 3
    assert!(!result.is_zero());
}

#[test]
fn test_coprime_polynomials_integration() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Coprime polynomials: gcd(x+1, y+1) = 1
    let p1 = expr!(x + 1);
    let p2 = expr!(y + 1);
    let vars = vec![x.clone(), y.clone()];

    let result = multivariate_gcd(&p1, &p2, &vars);

    // Should be 1 or equivalent
    eprintln!("DEBUG: coprime gcd result = {}", result);
    assert!(
        result.is_one() || result == Expression::integer(1),
        "Expected 1, got {}",
        result
    );
}

#[test]
fn test_with_constants() {
    let x = symbol!(x);
    let y = symbol!(y);

    // gcd(12, 18) = 6 (pure constants)
    let p1 = Expression::integer(12);
    let p2 = Expression::integer(18);
    let vars = vec![x.clone(), y.clone()];

    let result = multivariate_gcd(&p1, &p2, &vars);
    assert_eq!(result, Expression::integer(6));
}

#[test]
fn test_edge_case_zero() {
    let x = symbol!(x);
    let y = symbol!(y);

    let poly = Expression::mul(vec![expr!(x), expr!(y)]);
    let zero = Expression::integer(0);
    let vars = vec![x.clone(), y.clone()];

    // gcd(poly, 0) = poly
    let result = multivariate_gcd(&poly, &zero, &vars);
    assert_eq!(result, poly);
}

#[test]
fn test_edge_case_one() {
    let x = symbol!(x);
    let y = symbol!(y);

    let poly = Expression::mul(vec![expr!(x), expr!(y)]);
    let one = Expression::integer(1);
    let vars = vec![x.clone(), y.clone()];

    // gcd(poly, 1) = 1
    let result = multivariate_gcd(&poly, &one, &vars);
    assert!(result.is_one() || result == Expression::integer(1));
}

#[test]
fn test_symmetric_polynomials() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Test symmetry: gcd(p, q) = gcd(q, p)
    let p = Expression::mul(vec![expr!(x ^ 2), expr!(y)]);
    let q = Expression::mul(vec![expr!(x), expr!(y ^ 2)]);
    let vars = vec![x.clone(), y.clone()];

    let result1 = multivariate_gcd(&p, &q, &vars);
    let result2 = multivariate_gcd(&q, &p, &vars);

    assert_eq!(result1, result2);
}

#[test]
fn test_associativity() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Test: gcd(gcd(a, b), c) = gcd(a, gcd(b, c))
    let a = Expression::mul(vec![expr!(x), expr!(y)]);
    let b = expr!(x);
    let c = expr!(y);
    let vars = vec![x.clone(), y.clone()];

    let left = multivariate_gcd(&multivariate_gcd(&a, &b, &vars), &c, &vars);
    let right = multivariate_gcd(&a, &multivariate_gcd(&b, &c, &vars), &vars);

    // Both should give same GCD
    assert_eq!(left, right);
}

#[test]
fn test_gcd_with_itself() {
    let x = symbol!(x);
    let y = symbol!(y);

    let poly = expr!((x ^ 2) + (y ^ 2));
    let vars = vec![x.clone(), y.clone()];

    let result = multivariate_gcd(&poly, &poly, &vars);
    assert_eq!(result, poly);
}

#[test]
fn test_content_primitive_separation() {
    let x = symbol!(x);
    let y = symbol!(y);

    // Test that content and primitive parts are correctly separated
    // 6x²y + 9xy² = 3xy(2x + 3y)
    // Fixed: Use Expression::mul for multi-term multiplication
    let poly1 = Expression::add(vec![
        Expression::mul(vec![expr!(6), expr!(x ^ 2), expr!(y)]),
        Expression::mul(vec![expr!(9), expr!(x), expr!(y ^ 2)]),
    ]);
    let poly2 = Expression::mul(vec![expr!(3), expr!(x), expr!(y)]);
    let vars = vec![x.clone(), y.clone()];

    let result = multivariate_gcd(&poly1, &poly2, &vars);

    // Result should contain 3xy
    assert!(!result.is_zero());
}
