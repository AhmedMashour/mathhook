//! 🧪 UNIVERSAL PARSER VALIDATION TESTS
//! Validates that our JSON test cases match the actual Expression enum structure
//! NOTE: Temporarily disabled due to parser module refactoring

/*
use mathhook_core::core::expression::RelationType;
use mathhook_core::prelude::*;
use serde_json;
use std::fs;

#[test]
fn test_expression_variants_completeness() {
    println!("🧪 Testing Expression enum completeness...");

    // Test all new expression variants can be created
    let x = Symbol::new("x");

    // Basic expressions (already working)
    let _basic = vec![
        Expression::integer(42),
        Expression::symbol(x.clone()),
        Expression::add(vec![Expression::symbol(x.clone()), Expression::integer(1)]),
        Expression::mul(vec![Expression::integer(2), Expression::symbol(x.clone())]),
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
    ];

    // NEW: Complex numbers
    let _complex = Expression::complex(Expression::integer(3), Expression::integer(4));
    println!("✅ Complex numbers: 3 + 4i");

    // NEW: Mathematical constants
    let _constants = vec![
        Expression::pi(),
        Expression::e(),
        Expression::i(),
        Expression::infinity(),
        Expression::constant(MathConstant::GoldenRatio),
    ];
    println!("✅ Mathematical constants: π, e, i, ∞, φ");

    // NEW: Matrices
    let _matrix = Expression::matrix(vec![
        vec![Expression::integer(1), Expression::integer(2)],
        vec![Expression::integer(3), Expression::integer(4)],
    ]);
    println!("✅ Matrices: 2x2 matrix");

    // NEW: Relations/Equations
    let _equation = Expression::equation(Expression::symbol(x.clone()), Expression::integer(5));
    let _inequality = Expression::relation(
        Expression::symbol(x.clone()),
        Expression::integer(0),
        RelationType::Greater,
    );
    println!("✅ Relations: x = 5, x > 0");

    // NEW: Sets and Intervals
    let _set = Expression::set(vec![
        Expression::integer(1),
        Expression::integer(2),
        Expression::integer(3),
    ]);
    let _interval = Expression::interval(
        Expression::integer(0),
        Expression::integer(1),
        true,  // start inclusive
        false, // end exclusive
    );
    println!("✅ Sets and intervals: {{1,2,3}}, [0,1)");

    // NEW: First-class calculus
    let _derivative = Expression::derivative(
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        x.clone(),
        1,
    );
    let _integral = Expression::integral(Expression::symbol(x.clone()), x.clone());
    let _definite_integral = Expression::definite_integral(
        Expression::symbol(x.clone()),
        x.clone(),
        Expression::integer(0),
        Expression::integer(1),
    );
    let _limit = Expression::limit(
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        x.clone(),
        Expression::integer(0),
    );
    let _sum = Expression::sum(
        Expression::symbol(Symbol::new("i")),
        Symbol::new("i"),
        Expression::integer(1),
        Expression::symbol(Symbol::new("n")),
    );
    let _product = Expression::product(
        Expression::symbol(Symbol::new("i")),
        Symbol::new("i"),
        Expression::integer(1),
        Expression::symbol(Symbol::new("n")),
    );
    println!("✅ First-class calculus: d/dx, ∫, lim, Σ, Π");

    // NEW: Piecewise functions
    let _piecewise = Expression::piecewise(
        vec![
            (
                Expression::relation(
                    Expression::symbol(x.clone()),
                    Expression::integer(0),
                    RelationType::Greater,
                ),
                Expression::symbol(x.clone()),
            ),
            (
                Expression::relation(
                    Expression::symbol(x.clone()),
                    Expression::integer(0),
                    RelationType::LessEqual,
                ),
                Expression::mul(vec![Expression::integer(-1), Expression::symbol(x.clone())]),
            ),
        ],
        None,
    );
    println!("✅ Piecewise functions: |x| definition");

    println!("🎉 ALL EXPRESSION VARIANTS SUCCESSFULLY CREATED!");
}

#[test]
fn test_json_test_cases_structure() {
    println!("🧪 Testing JSON test cases structure...");

    let json_content = fs::read_to_string("parser_test_cases.json")
        .expect("Failed to read parser_test_cases.json");

    let test_data: serde_json::Value =
        serde_json::from_str(&json_content).expect("Failed to parse JSON");

    // Validate structure
    assert!(test_data["metadata"].is_object());
    assert!(test_data["test_cases"].is_object());
    assert!(test_data["stress_tests"].is_object());

    let categories = test_data["metadata"]["categories"].as_array().unwrap();
    println!("📋 Test categories: {}", categories.len());

    let test_cases = test_data["test_cases"].as_object().unwrap();
    println!("🧪 Test case groups: {}", test_cases.len());

    // Count total test cases
    let mut total_tests = 0;
    for (category, tests) in test_cases {
        if let Some(latex_tests) = tests["latex"].as_array() {
            total_tests += latex_tests.len();
        }
        if let Some(wolfram_tests) = tests["wolfram"].as_array() {
            total_tests += wolfram_tests.len();
        }
        println!(
            "  📂 {}: {} test cases",
            category,
            tests["latex"].as_array().unwrap_or(&vec![]).len()
                + tests["wolfram"].as_array().unwrap_or(&vec![]).len()
        );
    }

    println!("🎯 Total test cases: {}", total_tests);
    assert!(total_tests > 100, "Should have extensive test coverage");

    println!("✅ JSON structure validation passed!");
}

#[test]
fn test_first_class_calculus_examples() {
    println!("🧪 Testing first-class calculus constructs...");

    // Test that we can create and display calculus expressions
    let x = Symbol::new("x");

    // Derivative: d/dx x²
    let derivative = Expression::derivative(
        Expression::pow(Expression::symbol(x.clone()), Expression::integer(2)),
        x.clone(),
        1,
    );
    println!("Derivative: {}", derivative);
    assert!(format!("{}", derivative).contains("d/dx"));

    // Integral: ∫ x dx
    let integral = Expression::integral(Expression::symbol(x.clone()), x.clone());
    println!("Integral: {}", integral);
    assert!(format!("{}", integral).contains("∫"));

    // Definite integral: ∫₀¹ x dx
    let def_integral = Expression::definite_integral(
        Expression::symbol(x.clone()),
        x.clone(),
        Expression::integer(0),
        Expression::integer(1),
    );
    println!("Definite integral: {}", def_integral);
    assert!(format!("{}", def_integral).contains("∫[0 to 1]"));

    // Limit: lim(x→0) sin(x)
    let limit = Expression::limit(
        Expression::function("sin", vec![Expression::symbol(x.clone())]),
        x.clone(),
        Expression::integer(0),
    );
    println!("Limit: {}", limit);
    assert!(format!("{}", limit).contains("lim"));

    // Sum: Σ(i=1 to n) i²
    let i = Symbol::new("i");
    let n = Symbol::new("n");
    let sum = Expression::sum(
        Expression::pow(Expression::symbol(i.clone()), Expression::integer(2)),
        i.clone(),
        Expression::integer(1),
        Expression::symbol(n.clone()),
    );
    println!("Sum: {}", sum);
    assert!(format!("{}", sum).contains("Σ"));

    println!("✅ First-class calculus constructs working!");
}

#[test]
fn test_mathematical_constants() {
    println!("🧪 Testing mathematical constants...");

    let constants = vec![
        (Expression::pi(), "π"),
        (Expression::e(), "e"),
        (Expression::i(), "i"),
        (Expression::infinity(), "∞"),
        (Expression::constant(MathConstant::GoldenRatio), "φ"),
        (Expression::constant(MathConstant::EulerGamma), "γ"),
    ];

    for (expr, expected_symbol) in constants {
        let display = format!("{}", expr);
        println!("Constant: {} → {}", expected_symbol, display);
        assert!(display.contains(expected_symbol));
    }

    println!("✅ Mathematical constants working!");
}

#[test]
fn test_complex_numbers() {
    println!("🧪 Testing complex numbers...");

    let complex = Expression::complex(Expression::integer(3), Expression::integer(4));
    println!("Complex: {}", complex);
    assert!(format!("{}", complex).contains("3"));
    assert!(format!("{}", complex).contains("4"));
    assert!(format!("{}", complex).contains("i"));

    println!("✅ Complex numbers working!");
}

#[test]
fn test_no_function_fallbacks() {
    println!("🧪 Verifying no Function(name) fallbacks in critical areas...");

    // These should NOT be Function calls
    let x = Symbol::new("x");

    // Derivative should be Derivative variant, not Function
    let deriv = Expression::derivative(Expression::symbol(x.clone()), x.clone(), 1);
    let deriv_debug = format!("{:?}", deriv);
    assert!(deriv_debug.contains("Derivative"));
    assert!(!deriv_debug.contains("Function"));
    println!(
        "✅ Derivative is first-class: {}",
        deriv_debug.chars().take(50).collect::<String>()
    );

    // Integral should be Integral variant, not Function
    let integral = Expression::integral(Expression::symbol(x.clone()), x.clone());
    let integral_debug = format!("{:?}", integral);
    assert!(integral_debug.contains("Integral"));
    assert!(!integral_debug.contains("Function"));
    println!(
        "✅ Integral is first-class: {}",
        integral_debug.chars().take(50).collect::<String>()
    );

    // Constants should be Constant variant, not Symbol
    let pi = Expression::pi();
    let pi_debug = format!("{:?}", pi);
    assert!(pi_debug.contains("Constant"));
    assert!(!pi_debug.contains("Symbol"));
    println!("✅ Pi is first-class: {}", pi_debug);

    println!("🎉 NO FUNCTION FALLBACKS - ALL FIRST CLASS!");
}
*/
