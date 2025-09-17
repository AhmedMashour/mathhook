use mathhook_core::simplify::Simplify;
use mathhook_core::Expression;

fn main() {
    // sin(cos(0))
    let expr = Expression::function(
        "sin",
        vec![Expression::function("cos", vec![Expression::integer(0)])],
    );

    let result = expr.simplify();
    println!("Original: {:?}", expr);
    println!("Simplified: {:?}", result);
    println!("String: {}", result);
    println!("Contains 'sin': {}", result.to_string().contains("sin"));
    println!("Contains '1': {}", result.to_string().contains("1"));
}
