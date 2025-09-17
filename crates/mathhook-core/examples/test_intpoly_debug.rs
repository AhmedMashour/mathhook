use mathhook_core::{core::polynomial::IntPoly, expr, symbol};

fn main() {
    let x = symbol!(x);

    let poly1 = expr!((x ^ 5) + (2 * (x ^ 4)) + (3 * (x ^ 3)) + (4 * (x ^ 2)) + (5 * x) + 6);
    let poly2 =
        expr!((2 * (x ^ 5)) + (4 * (x ^ 4)) + (6 * (x ^ 3)) + (8 * (x ^ 2)) + (10 * x) + 12);

    println!("poly1: {}", poly1);
    println!("poly2: {}", poly2);

    // Check if can convert
    println!("Can convert poly1: {}", IntPoly::can_convert(&poly1, &x));
    println!("Can convert poly2: {}", IntPoly::can_convert(&poly2, &x));

    // Try conversion
    if let Some(ip1) = IntPoly::try_from_expression(&poly1, &x) {
        println!("IntPoly1: {}", ip1);
        if let Some(ip2) = IntPoly::try_from_expression(&poly2, &x) {
            println!("IntPoly2: {}", ip2);
            let gcd_poly = ip1.gcd(&ip2).unwrap();
            println!("IntPoly GCD: {}", gcd_poly);
            println!("Back to Expression: {}", gcd_poly.to_expression(&x));
        }
    } else {
        println!("Could not convert poly1");
    }

    // Expression-based GCD
    let result = poly1.gcd(&poly2);
    println!("Expression GCD result: {}", result);
}
