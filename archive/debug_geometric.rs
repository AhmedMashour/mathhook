use mathhook_core::core::{Expression, Symbol};
use mathhook_core::algebra::simplify::Simplify;

fn main() {
    // Create the ratio: 1 * 2^(-1)
    let ratio = Expression::mul(vec![
        Expression::integer(1),
        Expression::pow(Expression::integer(2), Expression::integer(-1)),
    ]);
    
    println!("Original ratio: {:?}", ratio);
    let simplified_ratio = ratio.simplify();
    println!("Simplified ratio: {:?}", simplified_ratio);
    
    // Test (1/2)^3
    let ratio_cubed = Expression::pow(simplified_ratio.clone(), Expression::integer(3));
    println!("Ratio cubed: {:?}", ratio_cubed);
    let simplified_cubed = ratio_cubed.simplify();
    println!("Simplified cubed: {:?}", simplified_cubed);
    
    // Test 1 - (1/2)^3
    let one_minus_cubed = Expression::add(vec![
        Expression::integer(1),
        Expression::mul(vec![Expression::integer(-1), simplified_cubed.clone()]),
    ]);
    println!("1 - cubed: {:?}", one_minus_cubed);
    let simplified_one_minus_cubed = one_minus_cubed.simplify();
    println!("Simplified 1 - cubed: {:?}", simplified_one_minus_cubed);
}
