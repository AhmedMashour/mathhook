use mathhook_core::prelude::*;

fn main() {
    // Test simple multiplication: 1 * 3 should be 3
    let mult = Expression::mul(vec![Expression::integer(1), Expression::integer(3)]);
    let simplified_mult = mult.simplify();
    println!("1 * 3 = {:?}", simplified_mult);

    // Test the failing case: 1*3 + 2 should be 5
    let expr = Expression::add(vec![
        Expression::mul(vec![Expression::integer(1), Expression::integer(3)]),
        Expression::integer(2),
    ]);
    let simplified = expr.simplify();
    println!("1*3 + 2 = {:?}", simplified);

    // Test what the test expects
    let expected = Expression::integer(5);
    println!("Expected: {:?}", expected);
    println!("Match: {}", simplified == expected);
}
