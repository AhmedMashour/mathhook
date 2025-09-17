use mathhook_core::{expr, Expression};
use std::collections::HashSet;

fn find_variables(expr: &Expression) -> Vec<mathhook_core::Symbol> {
    fn collect_symbols(expr: &Expression, symbols: &mut HashSet<mathhook_core::Symbol>) {
        match expr {
            Expression::Symbol(s) => {
                symbols.insert(s.clone());
            }
            Expression::Add(terms) | Expression::Mul(terms) => {
                for term in terms.iter() {
                    collect_symbols(term, symbols);
                }
            }
            Expression::Pow(base, exp) => {
                collect_symbols(base, symbols);
                collect_symbols(exp, symbols);
            }
            _ => {}
        }
    }

    let mut symbols = HashSet::new();
    collect_symbols(expr, &mut symbols);
    symbols.into_iter().collect()
}

fn main() {
    let poly1 = expr!((x ^ 5) + (2 * (x ^ 4)) + (3 * (x ^ 3)) + (4 * (x ^ 2)) + (5 * x) + 6);
    let poly2 =
        expr!((2 * (x ^ 5)) + (4 * (x ^ 4)) + (6 * (x ^ 3)) + (8 * (x ^ 2)) + (10 * x) + 12);

    println!("poly1: {}", poly1);
    println!("poly2: {}", poly2);

    let vars1 = find_variables(&poly1);
    let vars2 = find_variables(&poly2);

    println!("poly1 variables: {}", vars1.len());
    println!("poly2 variables: {}", vars2.len());

    // Union
    let mut all_vars = HashSet::new();
    for v in vars1 {
        all_vars.insert(v);
    }
    for v in vars2 {
        all_vars.insert(v);
    }
    println!("Total unique variables: {}", all_vars.len());
}
