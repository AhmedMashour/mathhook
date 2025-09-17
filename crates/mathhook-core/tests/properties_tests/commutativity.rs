//! Commutativity property tests
//!
//! Validates that commutative operations satisfy a + b = b + a and a * b = b * a.
//! These tests ensure mathematical correctness through fundamental algebraic properties.

use mathhook_core::{symbol, Expression, Simplify};

#[test]
fn test_integer_addition_commutativity() {
    let test_cases = vec![(1, 2), (0, 5), (-3, 7), (100, -50), (0, 0)];

    for (a, b) in test_cases {
        let ab = Expression::add(vec![Expression::integer(a), Expression::integer(b)]).simplify();
        let ba = Expression::add(vec![Expression::integer(b), Expression::integer(a)]).simplify();

        assert_eq!(
            ab, ba,
            "Addition commutativity failed: {} + {} != {} + {}",
            a, b, b, a
        );
    }
}

#[test]
fn test_integer_multiplication_commutativity() {
    let test_cases = vec![(2, 3), (0, 5), (-3, 7), (1, 100), (-2, -3)];

    for (a, b) in test_cases {
        let ab = Expression::mul(vec![Expression::integer(a), Expression::integer(b)]).simplify();
        let ba = Expression::mul(vec![Expression::integer(b), Expression::integer(a)]).simplify();

        assert_eq!(
            ab, ba,
            "Multiplication commutativity failed: {} * {} != {} * {}",
            a, b, b, a
        );
    }
}

#[test]
fn test_symbolic_addition_commutativity() {
    let x = symbol!(x);
    let y = symbol!(y);

    let xy = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
    ])
    .simplify();

    let yx = Expression::add(vec![
        Expression::symbol(y.clone()),
        Expression::symbol(x.clone()),
    ])
    .simplify();

    assert_eq!(
        xy, yx,
        "Symbolic addition commutativity failed: x + y != y + x"
    );
}

#[test]
fn test_symbolic_multiplication_commutativity() {
    let x = symbol!(x);
    let y = symbol!(y);

    let xy = Expression::mul(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
    ])
    .simplify();

    let yx = Expression::mul(vec![
        Expression::symbol(y.clone()),
        Expression::symbol(x.clone()),
    ])
    .simplify();

    assert_eq!(
        xy, yx,
        "Symbolic multiplication commutativity failed: x * y != y * x"
    );
}

#[test]
fn test_mixed_addition_commutativity() {
    let x = symbol!(x);

    let cases = vec![
        (Expression::symbol(x.clone()), Expression::integer(5)),
        (Expression::integer(3), Expression::symbol(x.clone())),
        (
            Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
            Expression::symbol(x.clone()),
        ),
    ];

    for (a, b) in cases {
        let ab = Expression::add(vec![a.clone(), b.clone()]).simplify();
        let ba = Expression::add(vec![b.clone(), a.clone()]).simplify();

        assert_eq!(ab, ba, "Mixed addition commutativity failed");
    }
}

#[test]
fn test_multi_term_addition_commutativity() {
    let x = symbol!(x);
    let y = symbol!(y);
    let z = symbol!(z);

    let xyz = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(z.clone()),
    ])
    .simplify();

    let zyx = Expression::add(vec![
        Expression::symbol(z.clone()),
        Expression::symbol(y.clone()),
        Expression::symbol(x.clone()),
    ])
    .simplify();

    let yxz = Expression::add(vec![
        Expression::symbol(y.clone()),
        Expression::symbol(x.clone()),
        Expression::symbol(z.clone()),
    ])
    .simplify();

    assert_eq!(
        xyz, zyx,
        "Multi-term addition commutativity failed: x+y+z != z+y+x"
    );
    assert_eq!(
        xyz, yxz,
        "Multi-term addition commutativity failed: x+y+z != y+x+z"
    );
}

#[test]
fn test_gcd_commutativity() {
    let test_cases = vec![(12, 18), (100, 25), (7, 11), (0, 5), (24, 36)];

    for (a, b) in test_cases {
        let gcd_ab = Expression::integer(a).gcd(&Expression::integer(b));
        let gcd_ba = Expression::integer(b).gcd(&Expression::integer(a));

        assert_eq!(
            gcd_ab, gcd_ba,
            "GCD commutativity failed: gcd({}, {}) != gcd({}, {})",
            a, b, b, a
        );
    }
}

#[test]
fn test_lcm_commutativity() {
    let test_cases = vec![(4, 6), (12, 18), (7, 11), (15, 25)];

    for (a, b) in test_cases {
        let lcm_ab = Expression::integer(a).lcm(&Expression::integer(b));
        let lcm_ba = Expression::integer(b).lcm(&Expression::integer(a));

        assert_eq!(
            lcm_ab, lcm_ba,
            "LCM commutativity failed: lcm({}, {}) != lcm({}, {})",
            a, b, b, a
        );
    }
}
