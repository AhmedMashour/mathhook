//! Debug trace for GCD issue

use mathhook_core::core::polynomial::poly::IntPoly;
use mathhook_core::expr;

fn main() {
    let a = expr!(x + 1);
    let b = expr!(x + 2);

    println!("=== Input ===");
    println!("a = {:?}", a);
    println!("b = {:?}", b);

    // Check find_variables
    let vars = a.find_variables();
    println!("\n=== Variables ===");
    println!("vars = {:?}", vars);
    println!("vars.len() = {}", vars.len());

    if vars.len() == 1 {
        let var = &vars[0];
        println!("var = {:?}", var);

        // Check can_convert
        let can_a = IntPoly::can_convert(&a, var);
        let can_b = IntPoly::can_convert(&b, var);
        println!("\n=== can_convert ===");
        println!("can_convert(a, var) = {}", can_a);
        println!("can_convert(b, var) = {}", can_b);

        if can_a && can_b {
            // Try conversion
            let poly1 = IntPoly::try_from_expression(&a, var);
            let poly2 = IntPoly::try_from_expression(&b, var);

            println!("\n=== IntPoly conversion ===");
            println!("poly1 = {:?}", poly1);
            println!("poly2 = {:?}", poly2);

            if let (Some(p1), Some(p2)) = (poly1, poly2) {
                // Do GCD
                let gcd_poly = p1.gcd_i64(&p2).unwrap();
                println!("\n=== GCD ===");
                println!("gcd_poly = {:?}", gcd_poly);
                println!("gcd_poly.degree() = {:?}", gcd_poly.degree());
                println!("gcd_poly.leading_coeff() = {:?}", gcd_poly.leading_coeff());

                // Convert back
                let result = gcd_poly.to_expression(var);
                println!("\n=== to_expression result ===");
                println!("result = {:?}", result);
            }
        }
    }
}
