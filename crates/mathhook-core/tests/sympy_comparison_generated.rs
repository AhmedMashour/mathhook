//! Generated test file for SymPy comparison
//! Auto-generated - do not edit manually

use mathhook_core::prelude::*;
use mathhook_core::calculus::Derivative;
use std::time::Instant;


#[test]
fn d_dx_x_2() {
    let var = symbol!(x);
    let expr_str = r#"x^2"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_x_3() {
    let var = symbol!(x);
    let expr_str = r#"x^3"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_x_4() {
    let var = symbol!(x);
    let expr_str = r#"x^4"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_sin_x() {
    let var = symbol!(x);
    let expr_str = r#"sin(x)"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_cos_x() {
    let var = symbol!(x);
    let expr_str = r#"cos(x)"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_tan_x() {
    let var = symbol!(x);
    let expr_str = r#"tan(x)"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_exp_x() {
    let var = symbol!(x);
    let expr_str = r#"exp(x)"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_log_x() {
    let var = symbol!(x);
    let expr_str = r#"log(x)"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_x_sin_x_product_rule() {
    let var = symbol!(x);
    let expr_str = r#"x*sin(x)"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_sin_x_x_quotient_rule() {
    let var = symbol!(x);
    let expr_str = r#"sin(x)/x"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_sin_x_2_chain_rule() {
    let var = symbol!(x);
    let expr_str = r#"sin(x^2)"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_x_2_2x_1() {
    let var = symbol!(x);
    let expr_str = r#"x^2 + 2*x + 1"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_x_2_exp_x() {
    let var = symbol!(x);
    let expr_str = r#"x^2*exp(x)"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_1_x() {
    let var = symbol!(x);
    let expr_str = r#"1/x"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}

#[test]
fn d_dx_sqrt_x() {
    let var = symbol!(x);
    let expr_str = r#"sqrt(x)"#;

    let expr = match parse!(expr_str) {
        Ok(e) => e,
        Err(_) => {
            eprintln!("PARSE_ERROR");
            return;
        }
    };


    let start = Instant::now();
    let result = expr.derivative(var);
    let elapsed = start.elapsed();

    println!("RESULT: {}", result);
    println!("TIME: {} ns", elapsed.as_nanos());
}
