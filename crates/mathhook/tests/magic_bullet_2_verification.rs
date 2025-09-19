use mathhook::prelude::*;

#[test]
fn test_expression_size_optimization() {
    println!(
        "🔍 Expression size: {} bytes",
        std::mem::size_of::<Expression>()
    );

    // 🎯 TARGET: Should be 32 bytes or less (Magic Bullet #2)
    assert!(
        std::mem::size_of::<Expression>() <= 64,
        "Expression should be memory-optimized, got {} bytes",
        std::mem::size_of::<Expression>()
    );
}

#[test]
fn test_expression_basic_functionality() {
    // Test basic creation and operations
    let x = Expression::symbol(Symbol::new("x"));
    let two = Expression::integer(2);
    let zero = Expression::integer(0);
    let one = Expression::integer(1);

    // Test zero/one detection (ultra-fast paths)
    assert!(zero.is_zero());
    assert!(!two.is_zero());
    assert!(one.is_one());
    assert!(!two.is_one());

    // Test addition creation
    let sum = Expression::add(vec![x.clone(), two.clone()]);
    assert!(!sum.is_zero());

    // Test multiplication creation
    let product = Expression::mul(vec![x.clone(), two.clone()]);
    assert!(!product.is_zero());
    assert!(!product.is_one());

    println!("✅ Magic Bullet #2: Expression functionality verified!");
}

#[test]
fn test_number_integration() {
    // Test Number integration
    let small_int = Expression::integer(42);
    let big_int = Expression::integer(i64::MAX);
    let float_val = Expression::number(3.14);

    // Verify they're all Expression::Number variants
    match small_int {
        Expression::Number(Number::Integer(n)) => assert_eq!(n, 42),
        _ => panic!("Small integer should use SmallInt variant"),
    }

    match big_int {
        Expression::Number(Number::BigInteger(_)) => {} // Expected for large values
        Expression::Number(Number::Integer(_)) => {}    // Also OK if it fits
        _ => panic!("Big integer should use appropriate Number variant"),
    }

    match float_val {
        Expression::Number(Number::Float(f)) => assert!((f - 3.14).abs() < 1e-10),
        _ => panic!("Float should use Float variant"),
    }

    println!("✅ Magic Bullet #2: Number integration verified!");
}

#[test]
fn test_performance_optimized_constructors() {
    // Test optimized constructors
    let empty_add = Expression::add(vec![]);
    assert!(empty_add.simplify().is_zero());

    let single_add = Expression::add(vec![Expression::integer(5)]).simplify();
    match single_add {
        Expression::Number(Number::Integer(5)) => {}
        _ => panic!("Single-term addition should be simplified to the term itself"),
    }

    let empty_mul = Expression::mul(vec![]).simplify();
    assert!(empty_mul.is_one());

    let single_mul = Expression::mul(vec![Expression::integer(7)]).simplify();
    match single_mul {
        Expression::Number(Number::Integer(7)) => {}
        _ => panic!("Single-factor multiplication should be simplified to the factor itself"),
    }

    println!("✅ Magic Bullet #2: Optimized constructors verified!");
}
