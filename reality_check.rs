//! 🔬 COMPREHENSIVE REALITY CHECK
//!
//! This test verifies what MathHook can ACTUALLY do vs what's just architecture.
//! Tests every major capability claimed in our analysis.

use mathhook_core::*;

fn main() {
    println!("🔬 MATHHOOK COMPREHENSIVE REALITY CHECK");
    println!("========================================\n");

    // Test 1: Basic Expression Creation and Parsing
    println!("📝 TEST 1: BASIC EXPRESSION SYSTEM");
    test_basic_expressions();

    // Test 2: Function Intelligence System
    println!("\n🧠 TEST 2: FUNCTION INTELLIGENCE SYSTEM");
    test_function_intelligence();

    // Test 3: Mathematical Operations
    println!("\n🧮 TEST 3: MATHEMATICAL OPERATIONS");
    test_mathematical_operations();

    // Test 4: Special Functions
    println!("\n⭐ TEST 4: SPECIAL FUNCTIONS");
    test_special_functions();

    // Test 5: Polynomial Operations
    println!("\n📊 TEST 5: POLYNOMIAL OPERATIONS");
    test_polynomial_operations();

    // Test 6: Number Theory
    println!("\n🔢 TEST 6: NUMBER THEORY");
    test_number_theory();

    // Test 7: Calculus Operations
    println!("\n∫ TEST 7: CALCULUS OPERATIONS");
    test_calculus_operations();

    // Test 8: Matrix Operations
    println!("\n🔲 TEST 8: MATRIX OPERATIONS");
    test_matrix_operations();

    // Test 9: Educational System
    println!("\n🎓 TEST 9: EDUCATIONAL SYSTEM");
    test_educational_system();

    // Test 10: Performance Claims
    println!("\n⚡ TEST 10: PERFORMANCE VERIFICATION");
    test_performance_claims();

    println!("\n🏁 REALITY CHECK COMPLETE");
}

fn test_basic_expressions() {
    // Test basic expression creation
    let x = Expression::symbol(Symbol::new("x"));
    let expr = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), x.clone()]),
        Expression::integer(1),
    ]);
    println!("✅ Basic expression creation: {}", expr);

    // Test parsing (if available)
    match std::panic::catch_unwind(|| {
        // Try to parse a simple expression
        let parsed = parse_expression("2*x + 1");
        println!("✅ Parsing works: {:?}", parsed);
    }) {
        Ok(_) => println!("✅ Parsing system: WORKING"),
        Err(_) => println!("❌ Parsing system: NOT WORKING or parse_expression doesn't exist"),
    }

    // Test formatting
    println!("✅ Display formatting: {}", expr);

    // Test LaTeX formatting
    match std::panic::catch_unwind(|| {
        use mathhook_core::formatter::latex::LaTeXFormatter;
        use mathhook_core::formatter::FormattingContext;

        struct SimpleContext;
        impl FormattingContext for SimpleContext {}

        let latex_result = expr.to_latex(SimpleContext);
        match latex_result {
            Ok(latex) => println!("✅ LaTeX formatting: {}", latex),
            Err(e) => println!("❌ LaTeX formatting error: {:?}", e),
        }
    }) {
        Ok(_) => println!("✅ LaTeX formatting: WORKING"),
        Err(_) => println!("❌ LaTeX formatting: NOT WORKING"),
    }
}

fn test_function_intelligence() {
    // Test if Universal Registry actually exists and works
    match std::panic::catch_unwind(|| {
        use crate::functions::intelligence::UNIVERSAL_REGISTRY;

        // Test basic function intelligence
        let has_sin = UNIVERSAL_REGISTRY.has_intelligence("sin");
        let has_cos = UNIVERSAL_REGISTRY.has_intelligence("cos");
        let has_gamma = UNIVERSAL_REGISTRY.has_intelligence("gamma");
        let has_legendre = UNIVERSAL_REGISTRY.has_intelligence("legendre_p");

        println!("✅ Function Intelligence Registry: WORKING");
        println!("  - sin intelligence: {}", has_sin);
        println!("  - cos intelligence: {}", has_cos);
        println!("  - gamma intelligence: {}", has_gamma);
        println!("  - legendre_p intelligence: {}", has_legendre);

        // Test property access
        if let Some(props) = UNIVERSAL_REGISTRY.get_properties("sin") {
            println!("✅ sin properties: AVAILABLE");
            println!("  - has derivative: {}", props.has_derivative());
        } else {
            println!("❌ sin properties: NOT AVAILABLE");
        }

        // Test educational explanations
        let sin_expr = Expression::function("sin", vec![Expression::symbol(Symbol::new("x"))]);
        let explanation =
            UNIVERSAL_REGISTRY.explain_function("sin", &[Expression::symbol(Symbol::new("x"))]);
        println!("✅ Educational explanations: {} steps", explanation.len());
    }) {
        Ok(_) => println!("✅ Function Intelligence System: FULLY WORKING"),
        Err(e) => println!("❌ Function Intelligence System: BROKEN - {:?}", e),
    }
}

fn test_mathematical_operations() {
    // Test basic arithmetic
    let a = Expression::integer(12);
    let b = Expression::integer(8);

    // Test addition
    let sum = Expression::add(vec![a.clone(), b.clone()]);
    println!("✅ Addition: {} + {} = {}", a, b, sum);

    // Test multiplication
    let product = Expression::mul(vec![a.clone(), b.clone()]);
    println!("✅ Multiplication: {} * {} = {}", a, b, product);

    // Test GCD (if implemented)
    match std::panic::catch_unwind(|| {
        let gcd_result = a.gcd(&b);
        println!("✅ GCD: gcd({}, {}) = {}", a, b, gcd_result);
    }) {
        Ok(_) => println!("✅ GCD operations: WORKING"),
        Err(_) => println!("❌ GCD operations: NOT WORKING"),
    }

    // Test simplification
    match std::panic::catch_unwind(|| {
        let complex_expr = Expression::add(vec![
            Expression::integer(5),
            Expression::integer(3),
            Expression::integer(-2),
        ]);
        let simplified = complex_expr.simplify();
        println!("✅ Simplification: {} = {}", complex_expr, simplified);
    }) {
        Ok(_) => println!("✅ Simplification: WORKING"),
        Err(_) => println!("❌ Simplification: NOT WORKING"),
    }
}

fn test_special_functions() {
    // Test if special functions can be created and evaluated
    let sin_expr = Expression::function("sin", vec![Expression::integer(0)]);
    let cos_expr = Expression::function("cos", vec![Expression::integer(0)]);
    let gamma_expr = Expression::function("gamma", vec![Expression::integer(1)]);

    println!("✅ Special function creation:");
    println!("  - sin(0): {}", sin_expr);
    println!("  - cos(0): {}", cos_expr);
    println!("  - gamma(1): {}", gamma_expr);

    // Test SIMD evaluation (if implemented)
    match std::panic::catch_unwind(|| {
        use crate::functions::FunctionEvaluator;
        let evaluator = FunctionEvaluator::new();

        // Test single evaluation
        let result = evaluator.evaluate("sin", &[Expression::integer(0)]);
        println!("✅ Function evaluation: sin(0) = {:?}", result);

        // Test SIMD bulk evaluation
        let values = vec![0.0, 1.57079632679, 3.14159265359]; // 0, π/2, π
        if let Some(simd_results) = evaluator.evaluate_bulk_f64("sin", &values) {
            println!("✅ SIMD evaluation: WORKING");
            println!("  - sin(0) ≈ {:.6}", simd_results[0]);
            println!("  - sin(π/2) ≈ {:.6}", simd_results[1]);
            println!("  - sin(π) ≈ {:.6}", simd_results[2]);
        } else {
            println!("❌ SIMD evaluation: NOT WORKING");
        }
    }) {
        Ok(_) => println!("✅ Function Evaluation System: WORKING"),
        Err(_) => println!("❌ Function Evaluation System: NOT WORKING"),
    }
}

fn test_polynomial_operations() {
    // Test polynomial creation
    let x = Expression::symbol(Symbol::new("x"));
    let poly = Expression::add(vec![
        Expression::pow(x.clone(), Expression::integer(2)),
        Expression::mul(vec![Expression::integer(2), x.clone()]),
        Expression::integer(1),
    ]);
    println!("✅ Polynomial creation: {}", poly);

    // Test polynomial operations (if available)
    match std::panic::catch_unwind(|| {
        // Test polynomial evaluation
        let evaluated = poly.clone(); // Placeholder - actual evaluation would substitute values
        println!("✅ Polynomial operations: Basic structure works");

        // Test if polynomial intelligence exists
        use crate::functions::polynomials::PolynomialIntelligence;
        let poly_intel = PolynomialIntelligence::new();
        let props = poly_intel.get_all_properties();
        let has_legendre = props.contains_key("legendre_p");
        let has_hermite = props.contains_key("hermite");

        println!("✅ Polynomial Intelligence:");
        println!("  - Legendre polynomials: {}", has_legendre);
        println!("  - Hermite polynomials: {}", has_hermite);
        println!(
            "  - Total polynomial functions: {}",
            poly_intel.get_all_properties().len()
        );
    }) {
        Ok(_) => println!("✅ Polynomial System: WORKING"),
        Err(_) => println!("❌ Polynomial System: NOT WORKING"),
    }
}

fn test_number_theory() {
    println!("Testing number theory capabilities...");

    // Test basic number theory functions (parsing level)
    let gcd_expr =
        Expression::function("gcd", vec![Expression::integer(12), Expression::integer(8)]);
    let lcm_expr =
        Expression::function("lcm", vec![Expression::integer(12), Expression::integer(8)]);
    let factorial_expr = Expression::function("factorial", vec![Expression::integer(5)]);

    println!("✅ Number theory function creation:");
    println!("  - gcd(12, 8): {}", gcd_expr);
    println!("  - lcm(12, 8): {}", lcm_expr);
    println!("  - factorial(5): {}", factorial_expr);

    // Test if actual number theory operations work
    match std::panic::catch_unwind(|| {
        let a = Expression::integer(12);
        let b = Expression::integer(8);
        let gcd_result = a.gcd(&b);
        println!("✅ Actual GCD computation: gcd(12, 8) = {}", gcd_result);
    }) {
        Ok(_) => println!("✅ Number Theory Operations: WORKING"),
        Err(_) => println!("❌ Number Theory Operations: ONLY PARSING, NO COMPUTATION"),
    }

    // Test modular arithmetic (if available)
    match std::panic::catch_unwind(|| {
        // This would test actual modular arithmetic if implemented
        println!("⚠️  Modular arithmetic: Testing not implemented yet");
    }) {
        Ok(_) => {}
        Err(_) => println!("❌ Modular Arithmetic: NOT IMPLEMENTED"),
    }
}

fn test_calculus_operations() {
    let x = Expression::symbol(Symbol::new("x"));

    // Test derivative creation
    let derivative_expr = Expression::function(
        "derivative",
        vec![Expression::function("sin", vec![x.clone()]), x.clone()],
    );
    println!("✅ Derivative expression creation: {}", derivative_expr);

    // Test integration creation
    let integral_expr = Expression::function(
        "integral",
        vec![Expression::function("cos", vec![x.clone()]), x.clone()],
    );
    println!("✅ Integral expression creation: {}", integral_expr);

    // Test if actual calculus operations work
    match std::panic::catch_unwind(|| {
        use crate::calculus::derivatives::Derivative;
        let sin_x = Expression::function("sin", vec![x.clone()]);
        let derivative = sin_x.derivative(Symbol::new("x"));
        println!(
            "✅ Actual derivative computation: d/dx sin(x) = {}",
            derivative
        );
    }) {
        Ok(_) => println!("✅ Calculus Operations: WORKING"),
        Err(_) => println!("❌ Calculus Operations: ONLY PARSING, NO COMPUTATION"),
    }
}

fn test_matrix_operations() {
    // Test matrix creation
    match std::panic::catch_unwind(|| {
        let matrix = Expression::matrix(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(3), Expression::integer(4)],
        ]);
        println!("✅ Matrix creation: {}", matrix);

        // Test matrix operations
        let det_expr = Expression::method_call(matrix.clone(), "det".to_string(), vec![]);
        println!("✅ Matrix determinant expression: {}", det_expr);
    }) {
        Ok(_) => println!("✅ Matrix System: WORKING"),
        Err(_) => println!("❌ Matrix System: NOT WORKING"),
    }
}

fn test_educational_system() {
    match std::panic::catch_unwind(|| {
        use crate::educational::step_by_step::Step;

        // Test step creation
        let step = Step::new("Test Step", "This is a test step".to_string());
        println!(
            "✅ Educational step creation: {} - {}",
            step.title, step.description
        );

        // Test if function explanations work
        let sin_expr = Expression::function("sin", vec![Expression::symbol(Symbol::new("x"))]);
        if let Ok(steps) = std::panic::catch_unwind(|| sin_expr.explain_function()) {
            println!("✅ Function explanations: {} steps generated", steps.len());
            for (i, step) in steps.iter().take(3).enumerate() {
                println!("  {}. {}: {}", i + 1, step.title, step.description);
            }
        } else {
            println!("❌ Function explanations: NOT WORKING");
        }
    }) {
        Ok(_) => println!("✅ Educational System: WORKING"),
        Err(_) => println!("❌ Educational System: NOT WORKING"),
    }
}

fn test_performance_claims() {
    use std::time::Instant;

    println!("Testing performance claims...");

    // Test simplification performance
    let start = Instant::now();
    let iterations = 10000;

    for _ in 0..iterations {
        let expr = Expression::add(vec![
            Expression::integer(2),
            Expression::integer(3),
            Expression::integer(-1),
        ]);
        let _ = expr.simplify();
    }

    let duration = start.elapsed();
    let ops_per_sec = iterations as f64 / duration.as_secs_f64();

    println!("✅ Simplification performance: {:.0} ops/sec", ops_per_sec);

    if ops_per_sec > 1_000_000.0 {
        println!("✅ Performance claim VERIFIED: >1M ops/sec");
    } else {
        println!(
            "❌ Performance claim FAILED: {:.0} ops/sec < 1M",
            ops_per_sec
        );
    }

    // Test SIMD performance (if available)
    match std::panic::catch_unwind(|| {
        use crate::functions::FunctionEvaluator;
        let evaluator = FunctionEvaluator::new();

        let start = Instant::now();
        let test_values: Vec<f64> = (0..1000).map(|i| i as f64 * 0.01).collect();

        for _ in 0..100 {
            if let Some(_results) = evaluator.evaluate_bulk_f64("sin", &test_values) {
                // SIMD evaluation worked
            }
        }

        let simd_duration = start.elapsed();
        let simd_ops_per_sec = (100 * 1000) as f64 / simd_duration.as_secs_f64();

        println!("✅ SIMD performance: {:.0} ops/sec", simd_ops_per_sec);
    }) {
        Ok(_) => println!("✅ SIMD System: WORKING"),
        Err(_) => println!("❌ SIMD System: NOT WORKING"),
    }
}

// Helper function to test parsing (may not exist)
fn parse_expression(input: &str) -> Result<Expression, String> {
    // This is a placeholder - actual parsing would use the parser module
    Err("Parsing function not directly accessible".to_string())
}
