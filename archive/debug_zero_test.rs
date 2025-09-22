use mathhook_core::core::{Expression, Symbol};

fn main() {
    let x = Symbol::new("x");
    let diff = Expression::add(vec![
        Expression::symbol(x.clone()),
        Expression::mul(vec![Expression::integer(-1), Expression::symbol(x)]),
    ]);
    
    println!("Original expression: {:?}", diff);
    let simplified = diff.simplify();
    println!("Simplified expression: {:?}", simplified);
    
    match simplified {
        Expression::Number(mathhook_core::core::Number::Integer(0)) => println!("✓ Correctly simplified to 0"),
        _ => println!("✗ Did not simplify to 0"),
    }
}
