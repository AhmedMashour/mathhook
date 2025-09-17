use mathhook_core::{expr, Expression};

fn main() {
    let poly1 = expr!((x ^ 5) + (2 * (x ^ 4)) + (3 * (x ^ 3)) + (4 * (x ^ 2)) + (5 * x) + 6);

    println!("poly1: {}", poly1);

    // This mirrors the internal find_variables logic
    use std::collections::HashSet;

    fn collect_symbols(expr: &Expression, symbols: &mut HashSet<mathhook_core::Symbol>) {
        match expr {
            Expression::Symbol(s) => {
                symbols.insert(s.clone());
                println!("Found symbol: {:?}", s);
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
    collect_symbols(&poly1, &mut symbols);
    let vars: Vec<_> = symbols.into_iter().collect();

    println!("Number of variables: {}", vars.len());
    for v in &vars {
        println!("Variable: {:?}", v);
    }
}
