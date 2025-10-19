use mathhook_core::algebra::gcd::PolynomialGcd;
use mathhook_core::{expr, symbol, Expression};

fn main() {
    println!("Polynomial Division API Examples");
    println!("=================================\n");

    example_1_simple_division();
    example_2_division_with_remainder();
    example_3_factored_polynomial();
    example_4_higher_degree();
    example_5_convenience_methods();
    example_6_constant_divisor();
    example_7_identical_polynomials();
}

fn example_1_simple_division() {
    println!("Example 1: Simple Division (Exact)");
    println!("-----------------------------------");

    let x = symbol!(x);
    let dividend = expr!((x^2) - 1);
    let divisor = expr!(x - 1);

    let (quotient, remainder) = dividend.div_polynomial(&divisor, &x);

    println!("Dividend:  {}", dividend);
    println!("Divisor:   {}", divisor);
    println!("Quotient:  {}", quotient);
    println!("Remainder: {}", remainder);
    println!();
    println!("Verification: ({}) = ({})({}) + ({})", dividend, divisor, quotient, remainder);
    println!();
}

fn example_2_division_with_remainder() {
    println!("Example 2: Division with Non-Zero Remainder");
    println!("--------------------------------------------");

    let x = symbol!(x);
    let dividend = expr!((x^2) + 1);
    let divisor = expr!(x - 1);

    let (quotient, remainder) = dividend.div_polynomial(&divisor, &x);

    println!("Dividend:  {}", dividend);
    println!("Divisor:   {}", divisor);
    println!("Quotient:  {}", quotient);
    println!("Remainder: {}", remainder);
    println!();
    println!("Division identity: ({}) = ({})({}) + ({})", dividend, divisor, quotient, remainder);
    println!();
}

fn example_3_factored_polynomial() {
    println!("Example 3: Dividing Factored Polynomial");
    println!("----------------------------------------");

    let x = symbol!(x);
    let dividend = expr!(add: (x^2), (3*x), 2);
    let divisor = expr!(x + 1);

    let (quotient, remainder) = dividend.div_polynomial(&divisor, &x);

    println!("Dividend:  {} (factors as (x+1)(x+2))", dividend);
    println!("Divisor:   {}", divisor);
    println!("Quotient:  {}", quotient);
    println!("Remainder: {}", remainder);
    println!();
    println!("The dividend factors as (x+1)(x+2), so dividing by (x+1) gives (x+2)");
    println!();
}

fn example_4_higher_degree() {
    println!("Example 4: Higher Degree Division");
    println!("----------------------------------");

    let x = symbol!(x);
    let dividend = Expression::add(vec![
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(3)),
        Expression::mul(vec![Expression::integer(2), Expression::pow(Expression::symbol(x.clone()), Expression::integer(2))]),
        Expression::mul(vec![Expression::integer(-5), Expression::symbol(x.clone())]),
        Expression::integer(-6),
    ]);
    let divisor = expr!(x - 2);

    let (quotient, remainder) = dividend.div_polynomial(&divisor, &x);

    println!("Dividend:  {}", dividend);
    println!("Divisor:   {}", divisor);
    println!("Quotient:  {}", quotient);
    println!("Remainder: {}", remainder);
    println!();
}

fn example_5_convenience_methods() {
    println!("Example 5: Using Convenience Methods");
    println!("-------------------------------------");

    let x = symbol!(x);
    let dividend = expr!((x^3) - 1);
    let divisor = expr!(x - 1);

    println!("Dividend: {}", dividend);
    println!("Divisor:  {}", divisor);
    println!();

    let quotient_only = dividend.quo_polynomial(&divisor, &x);
    println!("Using quo_polynomial() for quotient only: {}", quotient_only);

    let dividend2 = expr!((x^2) + 5);
    let divisor2 = expr!(x + 2);
    let remainder_only = dividend2.rem_polynomial(&divisor2, &x);
    println!("Using rem_polynomial() for remainder only: {}", remainder_only);
    println!("  (from dividing {} by {})", dividend2, divisor2);
    println!();
}

fn example_6_constant_divisor() {
    println!("Example 6: Division by Constant");
    println!("--------------------------------");

    let x = symbol!(x);
    let dividend = expr!(add: (2*(x^2)), (4*x), 6);
    let divisor = Expression::integer(2);

    let (quotient, remainder) = dividend.div_polynomial(&divisor, &x);

    println!("Dividend:  {}", dividend);
    println!("Divisor:   {}", divisor);
    println!("Quotient:  {}", quotient);
    println!("Remainder: {}", remainder);
    println!();
    println!("Dividing by a constant distributes to all terms");
    println!();
}

fn example_7_identical_polynomials() {
    println!("Example 7: Dividing Polynomial by Itself");
    println!("-----------------------------------------");

    let x = symbol!(x);
    let polynomial = expr!(add: (x^2), (5*x), 3);

    let (quotient, remainder) = polynomial.div_polynomial(&polynomial, &x);

    println!("Polynomial: {}", polynomial);
    println!("Dividing by itself:");
    println!("Quotient:  {}", quotient);
    println!("Remainder: {}", remainder);
    println!();
    println!("Any polynomial divided by itself gives quotient 1 and remainder 0");
    println!();
}
