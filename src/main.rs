use mathhook::prelude::*;

fn main() {
    let x = Symbol::new("x");
    let expr = Expression::symbol(x) + Expression::integer(1);
    let simplified = expr.simplify();
    println!("{:?}", simplified);
}
