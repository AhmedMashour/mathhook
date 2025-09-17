use mathhook_core::{expr, symbol};

fn main() {
    let _x = symbol!(x);
    let a = expr!(x + 1);
    let b = expr!(x + 2);

    println!("a = {:?}", a);
    println!("b = {:?}", b);

    let result = a.gcd(&b);
    println!("GCD result: {:?}", result);
}
