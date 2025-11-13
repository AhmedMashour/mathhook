use mathhook_core::algebra::groebner::{poly_reduce, MonomialOrder};
use mathhook_core::{symbol, Expression};

fn main() {
    let x = symbol!(x);
    let vars = vec![x.clone()];

    let poly = Expression::symbol(x.clone());
    let basis = [poly.clone()];
    let basis_refs: Vec<&Expression> = basis.iter().collect();

    println!("=== Single Reduction Step ===");
    println!("Poly: {}", poly);
    println!("Basis[0]: {}", basis[0]);

    let (reduced, changed) = poly_reduce(&poly, &basis_refs, &vars, &MonomialOrder::Lex);

    println!("Reduced: {}", reduced);
    println!("Changed: {}", changed);
    println!("Is zero? {}", reduced.is_zero());

    if !reduced.is_zero() && changed {
        println!("\n=== Second Reduction Step ===");
        let (reduced2, changed2) = poly_reduce(&reduced, &basis_refs, &vars, &MonomialOrder::Lex);
        println!("Reduced2: {}", reduced2);
        println!("Changed2: {}", changed2);
        println!("Is zero? {}", reduced2.is_zero());
    }
}
