use mathhook_core::core::{Expression, MathConstant};
use mathhook_core::functions::FunctionEvaluator;

fn main() {
    println!("🔍 DEBUGGING COS SPECIAL VALUES");

    let evaluator = FunctionEvaluator::new();

    // Test cos(0) - should be 1
    println!("Testing cos(0):");
    let result = evaluator.evaluate("cos", &[Expression::integer(0)]);
    println!("Result: {:?}", result);

    // Check if cos has intelligence
    let registry = &mathhook_core::functions::intelligence::UNIVERSAL_REGISTRY;
    println!("Cos has intelligence: {}", registry.has_intelligence("cos"));

    // Check if we can get cos properties
    if let Some(properties) = registry.get_properties("cos") {
        println!("Cos properties found!");

        // Try direct evaluation
        let direct_result = properties.evaluate("cos", &[Expression::integer(0)]);
        println!("Direct properties evaluation: {:?}", direct_result);
    } else {
        println!("Cos properties NOT found!");
    }
}
