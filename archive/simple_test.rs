use mathhook_core::core::Expression;
use mathhook_core::functions::FunctionEvaluator;

fn main() {
    println!("🔍 SIMPLE TEST - WHAT ACTUALLY WORKS");

    let evaluator = FunctionEvaluator::new();

    // Test cos(0) - should be 1 but returns Unevaluated
    println!(
        "cos(0) = {:?}",
        evaluator.evaluate("cos", &[Expression::integer(0)])
    );

    // Test sin(0) - this works!
    println!(
        "sin(0) = {:?}",
        evaluator.evaluate("sin", &[Expression::integer(0)])
    );

    // Check registry
    let registry = &mathhook_core::functions::intelligence::UNIVERSAL_REGISTRY;
    println!("Has cos: {}", registry.has_intelligence("cos"));
    println!("Has sin: {}", registry.has_intelligence("sin"));
}
