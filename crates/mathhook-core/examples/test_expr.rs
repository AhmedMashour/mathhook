//! Debug test for expression structure

use mathhook_core::{symbol, Expression};

fn main() {
    let x = symbol!(x);
    let numerator = Expression::symbol(x.clone());
    let denominator = Expression::symbol(x.clone());
    let expr = Expression::mul(vec![
        numerator,
        Expression::pow(denominator, Expression::integer(-1)),
    ]);
    println!("expr = {:?}", expr);

    if let Expression::Mul(factors) = &expr {
        println!("factors.len() = {}", factors.len());
        for (i, f) in factors.iter().enumerate() {
            println!("  factors[{}] = {:?}", i, f);
            if let Expression::Pow(base, exp) = f {
                println!("    Pow: base={:?}, exp={:?}", base, exp);
                println!(
                    "    exp == Integer(-1): {}",
                    **exp == Expression::integer(-1)
                );
            }
        }
    } else {
        println!("NOT a Mul expression! It's: {:?}", expr);
    }
}
