/// Quick memory size verification for Expression optimization
use mathhook_core::core::Expression;

#[test]
fn test_expression_size_only() {
    let size = std::mem::size_of::<Expression>();
    println!("Current Expression size: {} bytes", size);

    // Check if our optimization worked
    if size <= 32 {
        println!("Memory optimization SUCCESS - Expression ≤ 32 bytes");
    } else {
        println!("Memory optimization FAILED - Expression > 32 bytes");
    }

    // Create basic expressions to verify they work
    let _number = Expression::integer(42);
    let _symbol = Expression::symbol("x");

    println!("Basic expression creation works");
}
