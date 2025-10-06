use mathhook_core::core::Expression;
use mathhook_core::functions::FunctionEvaluator;

fn main() {
    println!("🔍 DEBUGGING PATTERN MATCHING");

    let evaluator = FunctionEvaluator::new();

    // Create the exact expressions
    let zero_expr = Expression::integer(0);
    let one_expr = Expression::integer(1);

    println!("Zero expression: {:?}", zero_expr);
    println!("One expression: {:?}", one_expr);

    // Test cos(0)
    let cos_result = evaluator.evaluate("cos", &[zero_expr.clone()]);
    println!("cos(0) result: {:?}", cos_result);

    // Test sin(0) - this works
    let sin_result = evaluator.evaluate("sin", &[zero_expr.clone()]);
    println!("sin(0) result: {:?}", sin_result);

    // Check if expressions are equal
    println!(
        "zero_expr == Expression::integer(0): {}",
        zero_expr == Expression::integer(0)
    );
    println!(
        "one_expr == Expression::integer(1): {}",
        one_expr == Expression::integer(1)
    );
}
