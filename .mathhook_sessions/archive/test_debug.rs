use mathhook_core::core::{Expression, Number};
use num_bigint::BigInt;
use num_rational::BigRational;

fn main() {
    let one = Expression::integer(1);
    let one_third = Expression::rational(1, 3);
    let neg_one_third = Expression::mul(vec![Expression::integer(-1), one_third]);
    let one_minus_one_third = Expression::add(vec![one, neg_one_third]);
    
    println!("1 - 1/3 = {:?}", one_minus_one_third);
    println!("Simplified: {:?}", one_minus_one_third.simplify());
    
    let power_neg_one = Expression::pow(one_minus_one_third.simplify(), Expression::integer(-1));
    println!("(1 - 1/3)^(-1) = {:?}", power_neg_one);
    println!("Simplified: {:?}", power_neg_one.simplify());
}
