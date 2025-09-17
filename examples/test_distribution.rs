use mathhook_core::expr::Expression;
use mathhook_core::core::Number;
use mathhook_core::symbol;

fn main() {
    let x = symbol!(x);

    // Create: Mul([-1, Add([1, x])])
    let inner_add = Expression::add(vec![Expression::integer(1), Expression::symbol(x.clone())]);
    println!("Inner add: {:?}", inner_add);

    let mul = Expression::mul(vec![Expression::integer(-1), inner_add]);
    println!("Mul: {:?}", mul);

    // Now test the pattern
    match &mul {
        Expression::Mul(factors) if factors.len() == 2 => {
            println!("Matched Mul with 2 factors");
            println!("Factor 0: {:?}", &factors[0]);
            println!("Factor 1: {:?}", &factors[1]);

            if let (Expression::Number(coeff), Expression::Add(add_terms)) =
                (&factors[0], &factors[1])
            {
                println!("Matched Number and Add pattern!");
                println!("Coeff: {:?}", coeff);
                println!("Add terms: {:?}", add_terms);
            } else {
                println!("Did NOT match Number and Add pattern");
            }
        }
        _ => println!("Did not match Mul pattern"),
    }

    println!("\n--- Now testing the full expr from the test ---");
    let expr = Expression::add(vec![Expression::mul(vec![
        Expression::integer(-1),
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
    ])]);
    println!("Expr: {:?}", expr);

    let simplified = expr.simplify();
    println!("Simplified: {:?}", simplified);
}
