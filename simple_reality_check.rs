//! 🔬 SIMPLE REALITY CHECK
//!
//! Direct test of what MathHook can actually do right now.

use mathhook_core::*;
use std::time::Instant;

fn main() {
    println!("🔬 MATHHOOK SIMPLE REALITY CHECK");
    println!("=================================\n");

    // Test 1: Basic Expression System
    println!("📝 TEST 1: BASIC EXPRESSIONS");
    let x = Expression::symbol(Symbol::new("x"));
    let expr = Expression::add(vec![
        Expression::mul(vec![Expression::integer(2), x.clone()]),
        Expression::integer(1),
    ]);
    println!("✅ Expression creation: {}", expr);

    // Test 2: Function Intelligence Registry
    println!("\n🧠 TEST 2: FUNCTION INTELLIGENCE");
    test_function_registry();

    // Test 3: Mathematical Operations
    println!("\n🧮 TEST 3: ACTUAL MATH OPERATIONS");
    test_real_math();

    // Test 4: Performance Reality
    println!("\n⚡ TEST 4: PERFORMANCE REALITY");
    test_performance_reality();

    // Test 5: What's Actually Implemented vs Parsing Only
    println!("\n🎯 TEST 5: IMPLEMENTATION REALITY");
    test_implementation_reality();

    println!("\n🏁 REALITY CHECK COMPLETE");
}

fn test_function_registry() {
    // Test the Universal Registry
    use mathhook_core::functions::intelligence::UNIVERSAL_REGISTRY;

    println!("Testing Universal Function Registry...");

    // Test basic queries
    let has_sin = UNIVERSAL_REGISTRY.has_intelligence("sin");
    let has_cos = UNIVERSAL_REGISTRY.has_intelligence("cos");
    let has_gamma = UNIVERSAL_REGISTRY.has_intelligence("gamma");
    let has_fake = UNIVERSAL_REGISTRY.has_intelligence("nonexistent_function");

    println!("✅ Registry queries:");
    println!("  - sin: {}", has_sin);
    println!("  - cos: {}", has_cos);
    println!("  - gamma: {}", has_gamma);
    println!("  - fake function: {}", has_fake);

    // Test property access
    if let Some(sin_props) = UNIVERSAL_REGISTRY.get_properties("sin") {
        println!("✅ sin properties found:");
        println!("  - has derivative: {}", sin_props.has_derivative());
        println!("  - special values: {}", sin_props.special_value_count());
    } else {
        println!("❌ sin properties: NOT FOUND");
    }

    // Test educational explanations
    let explanation =
        UNIVERSAL_REGISTRY.explain_function("sin", &[Expression::symbol(Symbol::new("x"))]);
    println!("✅ Educational explanations: {} steps", explanation.len());
    if !explanation.is_empty() {
        println!("  - First step: {}", explanation[0].title);
    }
}

fn test_real_math() {
    println!("Testing actual mathematical computations...");

    // Test basic arithmetic that should work
    let a = Expression::integer(12);
    let b = Expression::integer(8);

    // Test GCD (this should actually work)
    let gcd_result = a.gcd(&b);
    println!("✅ GCD computation: gcd(12, 8) = {}", gcd_result);

    // Test LCM (this should actually work)
    let lcm_result = a.lcm(&b);
    println!("✅ LCM computation: lcm(12, 8) = {}", lcm_result);

    // Test simplification
    let complex_expr = Expression::add(vec![
        Expression::integer(5),
        Expression::integer(3),
        Expression::integer(-2),
    ]);
    let simplified = complex_expr.simplify();
    println!("✅ Simplification: {} → {}", complex_expr, simplified);

    // Test if simplification actually computed the result
    if let Expression::Number(Number::Integer(result)) = simplified {
        if result == 6 {
            println!("✅ Simplification ACTUALLY WORKS: 5+3-2 = 6");
        } else {
            println!("❌ Simplification WRONG: got {}, expected 6", result);
        }
    } else {
        println!("❌ Simplification DOESN'T COMPUTE: still symbolic");
    }
}

fn test_performance_reality() {
    println!("Testing performance claims...");

    // Test simplification performance
    let start = Instant::now();
    let iterations = 100_000;

    for _ in 0..iterations {
        let expr = Expression::add(vec![Expression::integer(2), Expression::integer(3)]);
        let _ = expr.simplify();
    }

    let duration = start.elapsed();
    let ops_per_sec = iterations as f64 / duration.as_secs_f64();

    println!("✅ Simplification performance: {:.0} ops/sec", ops_per_sec);

    if ops_per_sec > 1_000_000.0 {
        println!("✅ PERFORMANCE CLAIM VERIFIED: >1M ops/sec");
    } else if ops_per_sec > 100_000.0 {
        println!(
            "⚠️  PERFORMANCE DECENT: {:.0} ops/sec (not claimed 7M+)",
            ops_per_sec
        );
    } else {
        println!("❌ PERFORMANCE POOR: {:.0} ops/sec", ops_per_sec);
    }

    // Test SIMD evaluation
    match std::panic::catch_unwind(|| {
        use mathhook_core::functions::FunctionEvaluator;
        let evaluator = FunctionEvaluator::new();

        let test_values: Vec<f64> = vec![
            0.0, 0.5236, 1.0472, 1.5708, 2.0944, 2.618, 3.1416, 3.665, 4.189, 4.712,
        ]; // 0, π/6, π/3, π/2, 2π/3, 5π/6, π, 7π/6, 4π/3, 3π/2 (10 values > 8 threshold)

        if let Some(results) = evaluator.evaluate_bulk_f64("sin", &test_values) {
            println!("  - sin(0) ≈ {:.6}", results[0]);
            println!("  - sin(π/6) ≈ {:.6}", results[1]);
            println!("  - sin(π/2) ≈ {:.6}", results[3]);
            println!("  - sin(π) ≈ {:.6}", results[6]);

            // Verify mathematical correctness for special values
            if results[0].abs() < 1e-10  // sin(0) = 0
                && (results[3] - 1.0).abs() < 1e-10  // sin(π/2) = 1
                && results[6].abs() < 1e-10
            // sin(π) = 0
            {
                println!("✅ SIMD MATHEMATICALLY CORRECT");
            } else {
                println!("❌ SIMD MATHEMATICALLY WRONG");
            }
        } else {
            println!("❌ SIMD evaluation: DOESN'T WORK");
        }
    }) {
        Ok(_) => {}
        Err(_) => println!("❌ SIMD system: COMPLETELY BROKEN"),
    }
}

fn test_implementation_reality() {
    println!("Testing what's actually implemented vs just parsing...");

    // Test 1: Number Theory - Can we actually compute or just parse?
    println!("\n🔢 NUMBER THEORY REALITY:");

    // GCD should work (we know this exists)
    let gcd_works = Expression::integer(48).gcd(&Expression::integer(18));
    println!("✅ GCD actually computes: gcd(48, 18) = {}", gcd_works);

    // Test modular arithmetic
    let mod_expr =
        Expression::function("mod", vec![Expression::integer(17), Expression::integer(5)]);
    println!(
        "⚠️  Modular arithmetic: Can create expression {}, but can it compute?",
        mod_expr
    );

    // Test 2: Special Functions - Can we evaluate or just create expressions?
    println!("\n⭐ SPECIAL FUNCTIONS REALITY:");

    let sin_zero = Expression::function("sin", vec![Expression::integer(0)]);
    let gamma_one = Expression::function("gamma", vec![Expression::integer(1)]);

    println!(
        "⚠️  Special functions: Can create {} and {}",
        sin_zero, gamma_one
    );
    println!("⚠️  But can they evaluate to actual values? Testing...");

    // Try to evaluate using the function evaluator
    match std::panic::catch_unwind(|| {
        use mathhook_core::functions::FunctionEvaluator;
        let evaluator = FunctionEvaluator::new();

        let sin_result = evaluator.evaluate("sin", &[Expression::integer(0)]);
        match sin_result {
            mathhook_core::functions::EvaluationResult::Exact(expr) => {
                println!("✅ sin(0) evaluates to: {}", expr);
            }
            mathhook_core::functions::EvaluationResult::Numerical(num) => {
                println!("✅ sin(0) evaluates numerically to: {}", num);
            }
            mathhook_core::functions::EvaluationResult::Unevaluated => {
                println!("❌ sin(0) stays unevaluated");
            }
            mathhook_core::functions::EvaluationResult::Error(e) => {
                println!("❌ sin(0) evaluation error: {}", e);
            }
        }
    }) {
        Ok(_) => {}
        Err(_) => println!("❌ Function evaluation system: BROKEN"),
    }

    // Test 3: Calculus - Can we actually differentiate or just create expressions?
    println!("\n∫ CALCULUS REALITY:");

    let sin_x = Expression::function("sin", vec![Expression::symbol(Symbol::new("x"))]);

    match std::panic::catch_unwind(|| {
        use mathhook_core::calculus::derivatives::Derivative;
        let derivative = sin_x.derivative(Symbol::new("x"));
        println!("✅ Derivative computation: d/dx sin(x) = {}", derivative);

        // Check if it's actually cos(x) or just a derivative expression
        if let Expression::Function { name, .. } = derivative {
            if name == "cos" {
                println!("✅ DERIVATIVE ACTUALLY COMPUTED: sin'(x) = cos(x)");
            } else {
                println!("❌ DERIVATIVE NOT COMPUTED: got function '{}'", name);
            }
        } else {
            println!("❌ DERIVATIVE NOT COMPUTED: got {:?}", derivative);
        }
    }) {
        Ok(_) => {}
        Err(_) => println!("❌ Calculus system: BROKEN"),
    }

    // Test 4: Integration
    match std::panic::catch_unwind(|| {
        use mathhook_core::calculus::integrals::Integration;
        let cos_x = Expression::function("cos", vec![Expression::symbol(Symbol::new("x"))]);
        let integral = cos_x.integrate(Symbol::new("x"));
        println!("✅ Integration: ∫cos(x)dx = {}", integral);

        // Check if it actually computed sin(x) + C
        if let Expression::Function { name, .. } = integral {
            if name == "sin" {
                println!("✅ INTEGRATION ACTUALLY COMPUTED: ∫cos(x)dx = sin(x)");
            } else {
                println!("❌ INTEGRATION NOT COMPUTED: got function '{}'", name);
            }
        } else {
            println!("❌ INTEGRATION NOT COMPUTED: got {:?}", integral);
        }
    }) {
        Ok(_) => {}
        Err(_) => println!("❌ Integration system: BROKEN"),
    }

    // Test 5: Matrix Operations
    println!("\n🔲 MATRIX REALITY:");

    match std::panic::catch_unwind(|| {
        let matrix = Expression::matrix(vec![
            vec![Expression::integer(1), Expression::integer(2)],
            vec![Expression::integer(3), Expression::integer(4)],
        ]);
        println!("✅ Matrix creation: {}", matrix);

        // Test determinant computation
        let det_expr = Expression::method_call(matrix.clone(), "det".to_string(), vec![]);
        println!("⚠️  Determinant expression: {}", det_expr);

        // Try to evaluate the determinant
        let det_result = det_expr.evaluate_method_call();
        println!("✅ Determinant evaluation: {}", det_result);

        // Check if it computed the actual determinant (should be -2)
        if let Expression::Number(Number::Integer(det_val)) = det_result {
            if det_val == -2 {
                println!("✅ MATRIX DETERMINANT ACTUALLY COMPUTED: det = -2");
            } else {
                println!("❌ MATRIX DETERMINANT WRONG: got {}, expected -2", det_val);
            }
        } else {
            println!("❌ MATRIX DETERMINANT NOT COMPUTED: still symbolic");
        }
    }) {
        Ok(_) => {}
        Err(_) => println!("❌ Matrix system: BROKEN"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // This will be run by cargo test
        main();
    }
}
