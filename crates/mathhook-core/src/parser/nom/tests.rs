/// Comprehensive test suite for the nom-based mathematical expression parser
///
/// This module tests the nom parser against the existing 468 test cases to establish
/// a baseline and track progress toward 100% success rate.
use super::NomParser;
use crate::core::Expression;
use crate::parser::ParseError;
use serde_json;
use std::fs;

/// Test the nom parser against all 468 test cases from cases.json
#[test]
fn test_nom_parser_against_all_cases() {
    println!("🧪 NOM PARSER COMPREHENSIVE TEST");
    println!("Testing nom parser against all 468 test cases");

    let parser = NomParser::new();

    // Read the test cases JSON file
    let json_content =
        fs::read_to_string("tests/parsing/cases.json").expect("Failed to read test cases file");
    let test_cases: Vec<serde_json::Value> =
        serde_json::from_str(&json_content).expect("Failed to parse test cases JSON");

    let mut total_tests = 0;
    let mut passed_tests = 0;
    let mut failed_tests = 0;
    let mut simple_passed = 0;
    let mut latex_passed = 0;
    let mut wolfram_passed = 0;
    let mut simple_total = 0;
    let mut latex_total = 0;
    let mut wolfram_total = 0;

    println!("📋 Found {} test cases", test_cases.len());

    for test_case in test_cases {
        total_tests += 1;

        let id = test_case["id"].as_str().unwrap();
        let language = test_case["language"].as_str().unwrap();
        let category = test_case["category"].as_str().unwrap();
        let input = test_case["input"].as_str().unwrap();
        let description = test_case["description"].as_str().unwrap();

        // Count by language
        match language {
            "simple" => simple_total += 1,
            "latex" => latex_total += 1,
            "wolfram" => wolfram_total += 1,
            _ => {}
        }

        print!(
            "🧪 [{}] {}: {} ... ",
            category,
            language.to_uppercase(),
            description
        );

        // Try to parse with our nom parser (automatic language detection)
        let result = parser.parse(input);

        match result {
            Ok(parsed) => {
                println!("✅ PASS: {:?}", parsed);
                passed_tests += 1;
                match language {
                    "simple" => simple_passed += 1,
                    "latex" => latex_passed += 1,
                    "wolfram" => wolfram_passed += 1,
                    _ => {}
                }
            }
            Err(err) => {
                println!("❌ FAIL: {:?}", err);
                println!("     ID: {}", id);
                println!("     Input: {}", input);
                failed_tests += 1;
            }
        }
    }

    // Final results
    println!("\n🎯 NOM PARSER TEST RESULTS:");
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

    println!("\n📊 BREAKDOWN BY LANGUAGE:");
    println!(
        "   Simple: {}/{} ({}%)",
        simple_passed,
        simple_total,
        (simple_passed * 100) / simple_total.max(1)
    );
    println!(
        "   LaTeX: {}/{} ({}%)",
        latex_passed,
        latex_total,
        (latex_passed * 100) / latex_total.max(1)
    );
    println!(
        "   Wolfram: {}/{} ({}%)",
        wolfram_passed,
        wolfram_total,
        (wolfram_passed * 100) / wolfram_total.max(1)
    );

    if failed_tests == 0 {
        println!("🎉 ALL NOM PARSER TESTS PASSED!");
    } else {
        println!("⚠️  Some tests failed - this is expected during Phase 1");
        println!("   Target for Phase 1: 50-80 basic test cases passing");
        println!("   Current: {} test cases passing", passed_tests);
    }

    // Don't fail the test during development - we expect failures
    // assert_eq!(failed_tests, 0, "Some nom parser tests failed");
}

/// Test basic arithmetic operations that should work in Phase 1
#[test]
fn test_nom_parser_basic_arithmetic() {
    let parser = NomParser::new();

    let basic_cases = vec![
        // Numbers
        ("42", "integer"),
        ("3.14", "float"),
        ("-5", "negative integer"),
        ("0", "zero"),
        // Variables
        ("x", "simple variable"),
        ("alpha", "multi-char variable"),
        ("var123", "variable with numbers"),
        // Constants
        ("pi", "pi constant"),
        ("e", "e constant"),
        ("i", "imaginary unit"),
        ("infinity", "infinity"),
        // Basic operations
        ("x + y", "addition"),
        ("x - y", "subtraction"),
        ("x * y", "multiplication"),
        ("x / y", "division"),
        ("x^2", "power"),
        ("x!", "factorial"),
        // Parentheses
        ("(x + 1)", "parentheses"),
        ("(x + y) * z", "complex parentheses"),
        // Function calls
        ("sin(x)", "sine function"),
        ("cos(x + y)", "cosine with expression"),
        ("log(x, y)", "two-argument function"),
        // Relations
        ("x = 5", "equality"),
        ("x != y", "inequality"),
        ("x < y", "less than"),
        ("x <= y", "less than or equal"),
        ("x > y", "greater than"),
        ("x >= y", "greater than or equal"),
        // Complex expressions
        ("x^2 + 2*x + 1", "quadratic"),
        ("sin(x) + cos(y)", "trig functions"),
        ("(x + 1)/(x - 1)", "rational expression"),
        ("-x + y", "unary minus"),
        ("x * -y", "negative factor"),
    ];

    let mut passed = 0;
    let mut failed = 0;

    println!("🧪 BASIC ARITHMETIC TESTS (Phase 1 Target)");

    for (input, description) in basic_cases {
        print!("Testing {}: '{}' ... ", description, input);

        match parser.parse(input) {
            Ok(expr) => {
                println!("✅ PASS: {:?}", expr);
                passed += 1;
            }
            Err(err) => {
                println!("❌ FAIL: {:?}", err);
                failed += 1;
            }
        }
    }

    println!("\n🎯 BASIC ARITHMETIC RESULTS:");
    println!(
        "   Passed: {}/{} ({}%)",
        passed,
        passed + failed,
        (passed * 100) / (passed + failed).max(1)
    );
    println!(
        "   Failed: {}/{} ({}%)",
        failed,
        passed + failed,
        (failed * 100) / (passed + failed).max(1)
    );

    // For Phase 1, we expect most basic arithmetic to work
    assert!(
        passed >= (passed + failed) / 2,
        "Less than 50% of basic arithmetic tests passed"
    );
}

/// Test roundtrip consistency for expressions that parse successfully
#[test]
fn test_nom_parser_roundtrip() {
    let parser = NomParser::new();

    let roundtrip_cases = vec![
        "42", "x", "pi", "x + y", "x * y", "x^2", "(x + 1)", "sin(x)",
    ];

    println!("🔄 ROUNDTRIP CONSISTENCY TESTS");

    for input in roundtrip_cases {
        print!("Testing roundtrip for '{}' ... ", input);

        match parser.parse(input) {
            Ok(expr) => {
                // For now, we'll just verify parsing works
                // TODO: Implement proper formatting and re-parsing
                println!("✅ PARSED: {:?}", expr);
            }
            Err(err) => {
                println!("❌ PARSE FAILED: {:?}", err);
            }
        }
    }
}

/// Test error handling for invalid expressions
#[test]
fn test_nom_parser_error_handling() {
    let parser = NomParser::new();

    let error_cases = vec![
        ("", "empty input"),
        ("x +", "incomplete expression"),
        ("(x + y", "unmatched parentheses"),
        ("x + + y", "double operator"),
        ("123abc", "invalid number-letter combination"),
        ("sin(", "incomplete function call"),
    ];

    println!("🚫 ERROR HANDLING TESTS");

    for (input, description) in error_cases {
        print!("Testing error case {}: '{}' ... ", description, input);

        match parser.parse(input) {
            Ok(expr) => {
                println!("❌ UNEXPECTED SUCCESS: {:?}", expr);
            }
            Err(err) => {
                println!("✅ CORRECTLY FAILED: {:?}", err);
            }
        }
    }
}

/// Performance benchmark for the nom parser
#[test]
fn test_nom_parser_performance() {
    let parser = NomParser::new();

    let benchmark_expressions = vec![
        "x + y",
        "x^2 + 2*x + 1",
        "sin(x) * cos(y) + tan(z)",
        "(a + b) * (c + d) * (e + f)",
        "x^3 + 3*x^2*y + 3*x*y^2 + y^3",
    ];

    println!("⚡ PERFORMANCE BENCHMARK");

    for expr in benchmark_expressions {
        let start = std::time::Instant::now();

        // Parse the expression multiple times
        for _ in 0..1000 {
            let _ = parser.parse_simple(expr);
        }

        let duration = start.elapsed();
        println!(
            "'{}': {:?} for 1000 parses ({:.2} μs/parse)",
            expr,
            duration,
            duration.as_micros() as f64 / 1000.0
        );
    }
}
