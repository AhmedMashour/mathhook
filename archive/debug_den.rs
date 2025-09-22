use mathhook_core::core::{Expression, Symbol};
use mathhook_core::calculus::limits::LimitMethods;

fn main() {
    let x = Symbol::new("x");
    let x_expr = Expression::symbol(x.clone());
    let point = Expression::integer(0);
    
    let den_at_point = LimitMethods::substitute_and_evaluate(&x_expr, &x, &point);
    println!("den_at_point: {:?}", den_at_point);
    println!("den_at_point.is_zero(): {}", den_at_point.is_zero());
}
