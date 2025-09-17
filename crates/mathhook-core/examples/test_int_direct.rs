use mathhook_core::calculus::integrals::strategy::integrate_with_strategy;
use mathhook_core::{symbol, Expression};

fn main() {
    let x = symbol!(x);

    // ∫(1/x)*e^x dx
    let integrand = Expression::mul(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(-1)), // 1/x
        Expression::function("exp", vec![Expression::symbol(x.clone())]),        // e^x
    ]);

    println!("Integrating: {:?}", integrand);
    let result = integrate_with_strategy(&integrand, x.clone(), 0);
    println!("Result: {:?}", result);
}
