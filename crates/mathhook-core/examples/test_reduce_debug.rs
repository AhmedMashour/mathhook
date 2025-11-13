use mathhook_core::{symbol, Expression};
use mathhook_core::algebra::groebner::{poly_reduce_completely, MonomialOrder};

fn main() {
    let x = symbol!(x);
    let vars = vec![x.clone()];

    let poly = Expression::symbol(x.clone());
    let basis = vec![poly.clone()];
    let basis_refs: Vec<&Expression> = basis.iter().collect();

    println!("Poly: {}", poly);
    println!("Basis: {:?}", basis);
    
    let reduced = poly_reduce_completely(&poly, &basis_refs, &vars, &MonomialOrder::Lex);
    
    println!("Reduced: {}", reduced);
    println!("Is zero? {}", reduced.is_zero());
    println!("Reduced == poly? {}", reduced == poly);
}
