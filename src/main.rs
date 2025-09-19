use mathhook::{calculus, const_expr, expr, parse, symbol, to_format};

fn main() {
    println!("MATHHOOK MACRO DEMONSTRATION");
    println!("Making mathematical expressions delightfully easy!\n");

    // ========== SYMBOL CREATION ==========
    println!("Symbol Creation with symbol! macro:");

    let x = symbol!(x);
    let (a, b, c) = symbol!(a, b, c);
    let alpha = symbol!("α");

    println!("   Single symbol:     {:?}", x);
    println!("   Multiple symbols:  {:?}, {:?}, {:?}", a, b, c);
    println!("   Unicode symbol:    {:?}", alpha);

    // ========== BASIC EXPRESSION CREATION ==========
    println!("\nBasic Expression Creation with expr! macro:");

    let x = expr!(x);
    let number = expr!(42);
    let simple_add = expr!(x + 1);
    let simple_mul = expr!(2 * x);
    let power = expr!(x ^ 2);
    let function = expr!(sin(x));

    println!("   Symbol:     {:?}", x);
    println!("   Number:     {:?}", number);
    println!("   Addition:   {:?}", simple_add);
    println!("   Multiply:   {:?}", simple_mul);
    println!("   Power:      {:?}", power);
    println!("   Function:   {:?}", function);

    // ========== MATHEMATICAL CONSTANTS ==========
    println!("\nMathematical Constants with const_expr! macro:");

    let pi = const_expr!(pi);
    let e = const_expr!(e);
    let i = const_expr!(i);
    let infinity = const_expr!(infinity);

    println!("   π: {:?}", pi);
    println!("   e: {:?}", e);
    println!("   i: {:?}", i);
    println!("   ∞: {:?}", infinity);

    // ========== PARSING MACROS ==========
    println!("\nParsing with parse! macro:");

    let auto_parsed = parse!("x^2 + 2*x + 1").unwrap();
    let latex_parsed = parse!(latex: "\\frac{x}{y}").unwrap();
    let wolfram_parsed = parse!(wolfram: "Times[x, y]").unwrap();
    let simple_parsed = parse!(simple: "sin(x)").unwrap();

    println!("   Auto:     {:?}", auto_parsed);
    println!("   LaTeX:    {:?}", latex_parsed);
    println!("   Wolfram:  {:?}", wolfram_parsed);
    println!("   Simple:   {:?}", simple_parsed);

    // ========== FORMAT CONVERSION ==========
    println!("\nFormat Conversion with to_format! macro:");

    let example_expr = expr!(x ^ 2);

    let simple_output = to_format!(simple: example_expr);
    let latex_output = to_format!(latex: example_expr);
    let wolfram_output = to_format!(wolfram: example_expr);

    println!("   Expression: {:?}", example_expr);
    println!("   Simple:     {}", simple_output);
    println!("   LaTeX:      {}", latex_output);
    println!("   Wolfram:    {}", wolfram_output);

    // ========== CALCULUS MACROS ==========
    println!("\nCalculus with calculus! macro:");

    let f = expr!(x ^ 2);
    let derivative = calculus!(derivative: f.clone(), x);
    let integral = calculus!(integral: f.clone(), x);

    println!("   Function:   {:?}", f);
    println!("   Derivative: {:?}", derivative);
    println!("   Integral:   {:?}", integral);

    // ========== PRACTICAL EXAMPLE ==========
    println!("\nPractical Example - Using Parse for Complex Expressions:");

    // For complex expressions, parsing is often more convenient than macros
    let quadratic = parse!("a*x^2 + b*x + c").unwrap();
    let fraction = parse!("(x + 1)/(x - 1)").unwrap();
    let trig = parse!("sin(x) + cos(x)").unwrap();

    println!("   Quadratic: {:?}", quadratic);
    println!("   Fraction:  {:?}", fraction);
    println!("   Trig:      {:?}", trig);

    println!("\n   Format outputs for quadratic:");
    println!("   Simple:  {}", to_format!(simple: quadratic));
    println!("   LaTeX:   {}", to_format!(latex: quadratic));
    println!("   Wolfram: {}", to_format!(wolfram: quadratic));

    // ========== COMPOSITION EXAMPLE ==========
    println!("\nComposition Example - Combining Macros:");

    let (_x_sym, _y_sym) = symbol!(x, y);
    let pi_const = const_expr!(pi);
    let base = expr!(2 * x);

    // Build complex expression using composition
    let composed = base + pi_const * expr!(y ^ 2);
    println!("   Composed: {:?}", composed);

    println!("\nMacros provide ergonomic building blocks!");
    println!("For complex expressions, parsing is often more convenient!");
}
