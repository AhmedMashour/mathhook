//! Associativity property tests
//!
//! Validates that associative operations satisfy (a + b) + c = a + (b + c)
//! and (a * b) * c = a * (b * c).

use mathhook_core::{symbol, Expression, Simplify};

#[test]
fn test_integer_addition_associativity() {
    let test_cases = vec![(1, 2, 3), (0, 5, 10), (-3, 7, 2), (100, -50, 25)];

    for (a, b, c) in test_cases {
        let left = Expression::add(vec![
            Expression::add(vec![Expression::integer(a), Expression::integer(b)]),
            Expression::integer(c),
        ])
        .simplify();

        let right = Expression::add(vec![
            Expression::integer(a),
            Expression::add(vec![Expression::integer(b), Expression::integer(c)]),
        ])
        .simplify();

        assert_eq!(
            left, right,
            "Addition associativity failed: ({} + {}) + {} != {} + ({} + {})",
            a, b, c, a, b, c
        );
    }
}

#[test]
fn test_integer_multiplication_associativity() {
    let test_cases = vec![(2, 3, 4), (1, 5, 10), (-2, 3, 4), (2, -3, -4)];

    for (a, b, c) in test_cases {
        let left = Expression::mul(vec![
            Expression::mul(vec![Expression::integer(a), Expression::integer(b)]),
            Expression::integer(c),
        ])
        .simplify();

        let right = Expression::mul(vec![
            Expression::integer(a),
            Expression::mul(vec![Expression::integer(b), Expression::integer(c)]),
        ])
        .simplify();

        assert_eq!(
            left, right,
            "Multiplication associativity failed: ({} * {}) * {} != {} * ({} * {})",
            a, b, c, a, b, c
        );
    }
}

#[test]
fn test_symbolic_addition_associativity() {
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    let left = Expression::add(vec![
        Expression::add(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]),
        Expression::symbol(z.clone()),
    ])
    .simplify();

    let right = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::add(vec![
            Expression::symbol(y.clone()),
            Expression::symbol(z.clone()),
        ]),
    ])
    .simplify();

    assert_eq!(
        left, right,
        "Symbolic addition associativity failed: (x + y) + z != x + (y + z)"
    );
}

#[test]
fn test_symbolic_multiplication_associativity() {
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    let left = Expression::mul(vec![
        Expression::mul(vec![
            Expression::symbol(x.clone()),
            Expression::symbol(y.clone()),
        ]),
        Expression::symbol(z.clone()),
    ])
    .simplify();

    let right = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![
            Expression::symbol(y.clone()),
            Expression::symbol(z.clone()),
        ]),
    ])
    .simplify();

    assert_eq!(
        left, right,
        "Symbolic multiplication associativity failed: (x * y) * z != x * (y * z)"
    );
}

#[test]
fn test_mixed_associativity() {
    let x = symbol!(x);

    let left = Expression::add(vec![
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(2)]),
        Expression::integer(3),
    ])
    .simplify();

    let right = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::add(vec![Expression::integer(2), Expression::integer(3)]),
    ])
    .simplify();

    assert_eq!(
        left, right,
        "Mixed associativity failed: (x + 2) + 3 != x + (2 + 3)"
    );
}

#[test]
fn test_power_associativity_note() {
    // Note: Exponentiation is NOT associative: (2^3)^4 != 2^(3^4)
    // (2^3)^4 = 8^4 = 4096
    // 2^(3^4) = 2^81 = huge number
    // This test documents this mathematical fact

    let left = Expression::pow(
        Expression::pow(Expression::integer(2), Expression::integer(3)),
        Expression::integer(2),
    )
    .simplify();

    let right = Expression::pow(
        Expression::integer(2),
        Expression::pow(Expression::integer(3), Expression::integer(2)),
    )
    .simplify();

    // These should NOT be equal - documenting non-associativity
    assert_ne!(
        left, right,
        "Exponentiation should NOT be associative: (2^3)^2 should != 2^(3^2)"
    );
}
