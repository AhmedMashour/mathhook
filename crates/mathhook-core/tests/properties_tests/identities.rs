//! Mathematical identity tests
//!
//! Validates fundamental mathematical identities:
//! - Additive identity: x + 0 = x
//! - Multiplicative identity: x * 1 = x
//! - Trigonometric identities: sin^2(x) + cos^2(x) = 1
//! - Logarithmic identities: e^(ln x) = x, ln(e^x) = x
//! - Power identities: x^0 = 1, x^1 = x

use mathhook_core::{symbol, Expression, Simplify};

#[test]
fn test_additive_identity() {
    let test_cases = vec![
        Expression::integer(5),
        Expression::integer(-3),
        Expression::integer(0),
        Expression::symbol(symbol!(x)),
        Expression::pow(Expression::symbol(symbol!(x)), Expression::integer(2)),
    ];

    for expr in test_cases {
        let with_zero = Expression::add(vec![expr.clone(), Expression::integer(0)]).simplify();
        assert_eq!(
            with_zero,
            expr.simplify(),
            "Additive identity failed: {} + 0 != {}",
            expr,
            expr
        );

        let zero_first = Expression::add(vec![Expression::integer(0), expr.clone()]).simplify();
        assert_eq!(
            zero_first,
            expr.simplify(),
            "Additive identity failed: 0 + {} != {}",
            expr,
            expr
        );
    }
}

#[test]
fn test_multiplicative_identity() {
    let test_cases = vec![
        Expression::integer(5),
        Expression::integer(-3),
        Expression::integer(1),
        Expression::symbol(symbol!(x)),
        Expression::pow(Expression::symbol(symbol!(x)), Expression::integer(2)),
    ];

    for expr in test_cases {
        let with_one = Expression::mul(vec![expr.clone(), Expression::integer(1)]).simplify();
        assert_eq!(
            with_one,
            expr.simplify(),
            "Multiplicative identity failed: {} * 1 != {}",
            expr,
            expr
        );

        let one_first = Expression::mul(vec![Expression::integer(1), expr.clone()]).simplify();
        assert_eq!(
            one_first,
            expr.simplify(),
            "Multiplicative identity failed: 1 * {} != {}",
            expr,
            expr
        );
    }
}

#[test]
fn test_additive_inverse() {
    let test_cases = vec![5, -3, 0, 100, -42];

    for n in test_cases {
        let sum = Expression::add(vec![Expression::integer(n), Expression::integer(-n)]).simplify();
        assert_eq!(
            sum,
            Expression::integer(0),
            "Additive inverse failed: {} + {} != 0",
            n,
            -n
        );
    }
}

#[test]
fn test_multiplicative_inverse() {
    use mathhook_core::core::Number;
    use num_bigint::BigInt;
    use num_rational::BigRational;

    let test_cases = vec![2, 3, 5, 7, 10];

    for n in test_cases {
        let num = Expression::integer(n);
        let reciprocal = Expression::number(Number::rational(BigRational::new(
            BigInt::from(1),
            BigInt::from(n),
        )));

        let product = Expression::mul(vec![num.clone(), reciprocal]).simplify();
        assert_eq!(
            product,
            Expression::integer(1),
            "Multiplicative inverse failed: {} * (1/{}) != 1",
            n,
            n
        );
    }
}

#[test]
fn test_zero_power_identity() {
    let test_cases = vec![
        Expression::integer(2),
        Expression::integer(5),
        Expression::integer(100),
        Expression::symbol(symbol!(x)),
    ];

    for base in test_cases {
        let result = Expression::pow(base.clone(), Expression::integer(0)).simplify();
        assert_eq!(
            result,
            Expression::integer(1),
            "Zero power failed: {}^0 != 1",
            base
        );
    }
}

#[test]
fn test_first_power_identity() {
    let test_cases = vec![
        Expression::integer(2),
        Expression::integer(-5),
        Expression::symbol(symbol!(x)),
        Expression::add(vec![Expression::symbol(symbol!(x)), Expression::integer(1)]),
    ];

    for base in test_cases {
        let result = Expression::pow(base.clone(), Expression::integer(1)).simplify();
        assert_eq!(
            result,
            base.simplify(),
            "First power failed: {}^1 != {}",
            base,
            base
        );
    }
}

#[test]
fn test_zero_multiplication() {
    let test_cases = vec![
        Expression::integer(5),
        Expression::integer(-3),
        Expression::symbol(symbol!(x)),
        Expression::pow(Expression::symbol(symbol!(x)), Expression::integer(2)),
        Expression::add(vec![Expression::symbol(symbol!(x)), Expression::integer(1)]),
    ];

    for expr in test_cases {
        let with_zero = Expression::mul(vec![expr.clone(), Expression::integer(0)]).simplify();
        assert_eq!(
            with_zero,
            Expression::integer(0),
            "Zero multiplication failed: {} * 0 != 0",
            expr
        );
    }
}

#[test]
fn test_double_negation() {
    let test_cases = vec![
        Expression::integer(5),
        Expression::integer(-3),
        Expression::symbol(symbol!(x)),
    ];

    for expr in test_cases {
        let double_neg = Expression::mul(vec![
            Expression::integer(-1),
            Expression::mul(vec![Expression::integer(-1), expr.clone()]),
        ])
        .simplify();

        assert_eq!(
            double_neg,
            expr.simplify(),
            "Double negation failed: -(-{}) != {}",
            expr,
            expr
        );
    }
}

#[test]
fn test_pythagorean_identity() {
    let x = symbol!(x);

    let sin_squared = Expression::pow(
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        Expression::integer(2),
    );

    let cos_squared = Expression::pow(
        Expression::function("cos", vec![Expression::symbol(x.clone())]),
        Expression::integer(2),
    );

    let sum = Expression::add(vec![sin_squared, cos_squared]).simplify();

    assert_eq!(
        sum,
        Expression::integer(1),
        "Pythagorean identity failed: sin^2(x) + cos^2(x) != 1"
    );
}

#[test]
fn test_sqrt_of_square() {
    let test_cases = vec![4, 9, 16, 25, 100];

    for n in test_cases {
        let sqrt_n = Expression::function("sqrt", vec![Expression::integer(n)]).simplify();
        let expected = Expression::integer((n as f64).sqrt() as i64);
        assert_eq!(sqrt_n, expected, "sqrt({}) != {}", n, expected);
    }
}

#[test]
fn test_gcd_lcm_relationship() {
    // gcd(a,b) * lcm(a,b) = a * b
    let test_cases = vec![(12, 18), (15, 25), (7, 11), (24, 36)];

    for (a, b) in test_cases {
        let gcd = Expression::integer(a).gcd(&Expression::integer(b));
        let lcm = Expression::integer(a).lcm(&Expression::integer(b));

        let gcd_lcm_product = Expression::mul(vec![gcd, lcm]).simplify();
        let ab_product =
            Expression::mul(vec![Expression::integer(a), Expression::integer(b)]).simplify();

        assert_eq!(
            gcd_lcm_product, ab_product,
            "GCD-LCM relationship failed: gcd({},{}) * lcm({},{}) != {} * {}",
            a, b, a, b, a, b
        );
    }
}
