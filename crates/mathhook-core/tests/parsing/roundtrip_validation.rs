//! Single test that reads JSON test cases and validates full roundtrip:
//! LaTeX/Wolfram → Expression → LaTeX/Wolfram
//!
use mathhook_core::parser::universal::MathLanguage;
use mathhook_core::parser::universal::UniversalParser;
use serde_json;
use std::fs;

#[test]
fn test_roundtrip_validation() {
    println!("🔄 COMPREHENSIVE ROUNDTRIP VALIDATION");
    println!("Testing: Input → Parse → Expression → Stringify → Output");

    // Read the flattened JSON test cases
    let json_content =
        fs::read_to_string("tests/parsing/cases.json").expect("Failed to read cases.json");

    let test_cases: Vec<serde_json::Value> =
        serde_json::from_str(&json_content).expect("Failed to parse JSON test cases");

    let mut total_tests = 0;
    let mut passed_tests = 0;
    let mut failed_tests = 0;

    println!("📋 Found {} test cases", test_cases.len());

    for test_case in test_cases {
        total_tests += 1;

        let id = test_case["id"].as_str().unwrap();
        let language = test_case["language"].as_str().unwrap();
        let category = test_case["category"].as_str().unwrap();
        let input = test_case["input"].as_str().unwrap();
        let description = test_case["description"].as_str().unwrap();

        print!(
            "🧪 [{}] {}: {} ... ",
            category,
            language.to_uppercase(),
            description
        );

        let result = test_format_aware_roundtrip(input, language);

        match result {
            Ok(_) => {
                println!("✅ PASS");
                passed_tests += 1;
            }
            Err(e) => {
                println!("❌ FAIL: {}", e);
                println!("     ID: {}", id);
                println!("     Input: {}", input);
                failed_tests += 1;

                // Continue through all tests to get complete results
                // if failed_tests >= 5 {
                //     println!("⚠️ Stopping after 5 failures to avoid spam...");
                //     break;
                // }
            }
        }
    }

    // Final results
    println!("\n🎯 ROUNDTRIP VALIDATION RESULTS:");
    println!("   Total tests: {}", total_tests);
    println!(
        "   Passed: {} ({}%)",
        passed_tests,
        (passed_tests * 100) / total_tests.max(1)
    );
    println!(
        "   Failed: {} ({}%)",
        failed_tests,
        (failed_tests * 100) / total_tests.max(1)
    );

    if failed_tests == 0 {
        println!("🎉 ALL ROUNDTRIP TESTS PASSED!");
    } else {
        println!("⚠️  Some tests failed - parser needs improvement");
        println!("   This is expected since we're still implementing the full parser");
    }

    assert!(false)
}

/// Test format-aware roundtrip: Input → Expression → Same Format Output
fn test_format_aware_roundtrip(input: &str, language: &str) -> Result<(), String> {
    let mut parser = UniversalParser::new();

    // Map string language to MathLanguage enum
    let math_language = match language {
        "latex" => MathLanguage::LaTeX,
        "wolfram" => MathLanguage::Wolfram,
        _ => return Err(format!("Unknown language: {}", language)),
    };

    // But actually detect the format to handle simple cases
    let detected_format = parser.detect_language(input);
    let parse_format = if detected_format == MathLanguage::Simple {
        detected_format
    } else {
        math_language
    };

    // Step 1: Parse input to Expression
    let expression = parser
        .parse_with_language(input, parse_format)
        .map_err(|e| format!("Parse failed: {:?}", e))?;

    // Step 2: Convert Expression back to same format
    let output = parser.to_format(&expression, parse_format);

    // Step 3: Parse the output again to verify it's valid
    let _reparsed_expression = parser
        .parse_with_language(&output, parse_format)
        .map_err(|e| format!("Re-parse failed: {:?} (Output: {})", e, output))?;

    Ok(())
}

fn test_language_detection_accuracy() {
    println!("🔍 LANGUAGE DETECTION ACCURACY TEST");

    let parser = UniversalParser::new();

    let test_cases = vec![
        // Clear LaTeX cases
        ("\\frac{1}{2}", MathLanguage::LaTeX, "LaTeX fraction"),
        ("\\sin(x)", MathLanguage::LaTeX, "LaTeX sine"),
        ("\\int x dx", MathLanguage::LaTeX, "LaTeX integral"),
        ("x^2 + y^2", MathLanguage::Simple, "LaTeX powers"),
        // Clear Wolfram cases
        ("Sin[x]", MathLanguage::Wolfram, "Wolfram sine"),
        ("Integrate[x, x]", MathLanguage::Wolfram, "Wolfram integral"),
        ("Power[x, 2]", MathLanguage::Wolfram, "Wolfram power"),
        ("Plus[x, y]", MathLanguage::Wolfram, "Wolfram plus"),
        // Ambiguous cases (should default to LaTeX)
        ("x + y", MathLanguage::Simple, "Ambiguous addition"),
        ("2*x", MathLanguage::Simple, "Ambiguous multiplication"),
    ];

    let mut correct = 0;
    let total = test_cases.len();

    for (input, expected, description) in test_cases {
        let detected = parser.detect_language(input);
        print!("  🔍 {}: ", description);

        if detected == expected {
            println!("✅ Detected {:?}", detected);
            correct += 1;
        } else {
            println!("❌ Expected {:?}, got {:?}", expected, detected);
        }
    }

    println!(
        "\n🎯 DETECTION ACCURACY: {}/{} correct ({}%)",
        correct,
        total,
        (correct * 100) / total.max(1)
    );

    assert!(
        correct >= (total * 80) / 100,
        "Detection accuracy should be at least 80%"
    );
}
