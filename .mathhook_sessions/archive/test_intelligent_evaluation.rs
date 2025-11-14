//! 🧠 Test Intelligent Function Evaluation System

use mathhook_core::*;

fn main() {
    println!("🧠 TESTING INTELLIGENT FUNCTION EVALUATION");
    println!("==========================================\n");

    // Test 1: Function Intelligence Integration
    println!("📝 TEST 1: FUNCTION INTELLIGENCE INTEGRATION");
    test_intelligence_integration();

    // Test 2: Number Theory Functions
    println!("\n🔢 TEST 2: NUMBER THEORY FUNCTIONS");
    test_number_theory_functions();

    // Test 3: Mathematical Properties Usage
    println!("\n📊 TEST 3: MATHEMATICAL PROPERTIES USAGE");
    test_properties_usage();

    println!("\n🏁 INTELLIGENT EVALUATION TEST COMPLETE");
}

fn test_intelligence_integration() {
    use mathhook_core::functions::intelligence::UNIVERSAL_REGISTRY;

    println!("Testing Function Intelligence Registry integration...");

    // Check if number theory functions are now registered
    let has_mod = UNIVERSAL_REGISTRY.has_intelligence("mod");
    let has_mod_pow = UNIVERSAL_REGISTRY.has_intelligence("mod_pow");
    let has_gcd = UNIVERSAL_REGISTRY.has_intelligence("gcd");
    let has_lcm = UNIVERSAL_REGISTRY.has_intelligence("lcm");

    println!("✅ Function Intelligence Registry:");
    println!("  - mod: {}", has_mod);
    println!("  - mod_pow: {}", has_mod_pow);
    println!("  - gcd: {}", has_gcd);
    println!("  - lcm: {}", has_lcm);

    // Test properties access
    if let Some(mod_props) = UNIVERSAL_REGISTRY.get_properties("mod") {
        println!("✅ mod properties found:");
        println!("  - has derivative: {}", mod_props.has_derivative());
        println!("  - special values: {}", mod_props.special_value_count());
    } else {
        println!("❌ mod properties not found");
    }
}

fn test_number_theory_functions() {
    let evaluator = FunctionEvaluator::new();

    println!("Testing number theory function evaluation...");

    // Test modular reduction
    let mod_result = evaluator.evaluate("mod", &[Expression::integer(17), Expression::integer(5)]);
    match mod_result {
        EvaluationResult::Exact(expr) => {
            println!("✅ mod(17, 5) = {}", expr);
            if expr == Expression::integer(2) {
                println!("✅ MODULAR REDUCTION WORKS CORRECTLY");
            } else {
                println!("❌ MODULAR REDUCTION INCORRECT: got {}, expected 2", expr);
            }
        }
        EvaluationResult::Error(e) => println!("❌ mod evaluation error: {}", e),
        _ => println!("⚠️  mod evaluation unevaluated"),
    }

    // Test modular exponentiation
    let mod_pow_result = evaluator.evaluate(
        "mod_pow",
        &[
            Expression::integer(2),
            Expression::integer(10),
            Expression::integer(1000),
        ],
    );
    match mod_pow_result {
        EvaluationResult::Exact(expr) => {
            println!("✅ mod_pow(2, 10, 1000) = {}", expr);
            if expr == Expression::integer(24) {
                println!("✅ MODULAR EXPONENTIATION WORKS CORRECTLY");
            } else {
                println!(
                    "❌ MODULAR EXPONENTIATION INCORRECT: got {}, expected 24",
                    expr
                );
            }
        }
        EvaluationResult::Error(e) => println!("❌ mod_pow evaluation error: {}", e),
        _ => println!("⚠️  mod_pow evaluation unevaluated"),
    }

    // Test existing GCD (should still work)
    let gcd_result = evaluator.evaluate("gcd", &[Expression::integer(48), Expression::integer(18)]);
    match gcd_result {
        EvaluationResult::Exact(expr) => {
            println!("✅ gcd(48, 18) = {}", expr);
            if expr == Expression::integer(6) {
                println!("✅ GCD STILL WORKS WITH INTELLIGENT SYSTEM");
            } else {
                println!("❌ GCD BROKEN: got {}, expected 6", expr);
            }
        }
        _ => println!("❌ GCD evaluation failed"),
    }
}

fn test_properties_usage() {
    let evaluator = FunctionEvaluator::new();

    println!("Testing mathematical properties usage...");

    // Test special value evaluation using Function Intelligence
    let sin_zero = evaluator.evaluate("sin", &[Expression::integer(0)]);
    match sin_zero {
        EvaluationResult::Exact(expr) => {
            println!(
                "✅ sin(0) = {} (using Function Intelligence special values)",
                expr
            );
            if expr == Expression::integer(0) {
                println!("✅ FUNCTION INTELLIGENCE SPECIAL VALUES WORK");
            }
        }
        _ => println!("⚠️  sin(0) not evaluated using special values"),
    }

    // Test numerical evaluation fallback
    let pi_over_4 = Expression::mul(vec![Expression::pi(), Expression::rational(1, 4)]);
    let sin_pi_4 = evaluator.evaluate("sin", &[pi_over_4]);
    match sin_pi_4 {
        EvaluationResult::Numerical(val) => {
            println!("✅ sin(π/4) ≈ {:.6} (numerical fallback)", val);
            if (val - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-10 {
                println!("✅ NUMERICAL FALLBACK WORKS CORRECTLY");
            }
        }
        _ => println!("⚠️  sin(π/4) numerical evaluation failed"),
    }
}
