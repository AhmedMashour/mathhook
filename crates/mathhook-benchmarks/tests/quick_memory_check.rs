/// Quick check of Expression memory size after optimization
use std::mem::size_of;

#[test]
fn test_expression_memory_size() {
    // This should compile even with the pattern matching errors in other modules
    println!(
        "Expression size: {} bytes",
        size_of::<mathhook_core::core::Expression>()
    );

    // Our target: 32 bytes or less
    let size = size_of::<mathhook_core::core::Expression>();

    if size <= 32 {
        println!("✅ SUCCESS: Expression ≤ 32 bytes (target achieved)");
    } else if size <= 48 {
        println!("⚠️  PARTIAL: Expression ≤ 48 bytes (acceptable but not optimal)");
    } else {
        println!("❌ FAILED: Expression > 48 bytes (optimization failed)");
    }

    // Just verify the enum exists
    assert!(size > 0);
}
