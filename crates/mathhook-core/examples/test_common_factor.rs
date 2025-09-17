use mathhook_core::{expr, Expression};

// Copy of find_common_factor logic for testing
fn test_find_common_factor(expr1: &Expression, expr2: &Expression) {
    match (expr1, expr2) {
        (Expression::Mul(_factors1), Expression::Mul(factors2)) => {
            println!("Both are Mul");
            println!("factors1: {:?}", _factors1.len());
            println!("factors2: {:?}", factors2.len());
        }
        (Expression::Mul(factors), single) | (single, Expression::Mul(factors)) => {
            println!("One Mul, one single");
            println!("factors: {:?}", factors.len());
            println!("single: {}", single);
        }
        _ => {
            println!("Neither is Mul");
            println!("expr1 == expr2: {}", expr1 == expr2);
        }
    }
}

fn main() {
    let poly1 = expr!((x ^ 5) + (2 * (x ^ 4)) + (3 * (x ^ 3)) + (4 * (x ^ 2)) + (5 * x) + 6);
    let poly2 =
        expr!((2 * (x ^ 5)) + (4 * (x ^ 4)) + (6 * (x ^ 3)) + (8 * (x ^ 2)) + (10 * x) + 12);

    println!("poly1: {}", poly1);
    println!("poly2: {}", poly2);

    test_find_common_factor(&poly1, &poly2);
}
