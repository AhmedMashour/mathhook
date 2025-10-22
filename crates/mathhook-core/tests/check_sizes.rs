use mathhook_core::core::Expression;
use mathhook_core::core::Symbol;
use mathhook_core::core::Number;

#[test]
fn check_type_sizes() {
    let expr_size = std::mem::size_of::<Expression>();
    let symbol_size = std::mem::size_of::<Symbol>();
    let number_size = std::mem::size_of::<Number>();
    let box_expr_size = std::mem::size_of::<Box<Expression>>();
    let vec_expr_size = std::mem::size_of::<Vec<Expression>>();

    println!("\n=== Type Sizes ===");
    println!("Expression size: {} bytes (target: 32)", expr_size);
    println!("Symbol size: {} bytes", symbol_size);
    println!("Number size: {} bytes (target: 16)", number_size);
    println!("Box<Expression> size: {} bytes", box_expr_size);
    println!("Vec<Expression> size: {} bytes", vec_expr_size);
    println!("=================\n");

    // Warnings if sizes exceed targets
    if expr_size > 32 {
        println!("⚠️  WARNING: Expression size ({} bytes) exceeds 32-byte target!", expr_size);
        println!("   This will hurt cache performance!");
    }

    if number_size != 16 {
        println!("⚠️  WARNING: Number size ({} bytes) is not 16 bytes!", number_size);
    }

    // Always passes, just for printing
    assert!(true);
}
