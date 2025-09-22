use mathhook_core::core::{Expression, Symbol};
use mathhook_core::algebra::simplify::Simplify;
use mathhook_core::calculus::limits::{Limits, LimitMethods};

fn main() {
    let x = Symbol::new("x");
    let sin_x = Expression::function("sin", vec![Expression::symbol(x.clone())]);
    let x_expr = Expression::symbol(x.clone());
    let point = Expression::integer(0);
    
    println!("Original sin(x): {:?}", sin_x);
    println!("Original x: {:?}", x_expr);
    println!("Point: {:?}", point);
    
    // Test substitution
    let sin_at_0 = LimitMethods::substitute_and_evaluate(&sin_x, &x, &point);
    let x_at_0 = LimitMethods::substitute_and_evaluate(&x_expr, &x, &point);
    
    println!("sin(0): {:?}", sin_at_0);
    println!("x at 0: {:?}", x_at_0);
    
    println!("sin(0) is_zero: {}", sin_at_0.is_zero());
    println!("x at 0 is_zero: {}", x_at_0.is_zero());
    
    // Test the full expression
    let expr = Expression::mul(vec![
        sin_x.clone(),
        Expression::pow(x_expr.clone(), Expression::integer(-1)),
    ]);
    
    println!("Full expr: {:?}", expr);
    let result = expr.limit(&x, &point);
    println!("Limit result: {:?}", result);
}
