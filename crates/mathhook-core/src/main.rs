use mathhook_core::formatter::MathLanguage;
use mathhook_core::parser::config::ParserConfig;
use mathhook_core::parser::Parser;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write};

fn main() {
    println!("\n=== Comprehensive Parser & Formatter Testing ===\n");
    test_explicit_cases();
}

#[derive(Debug, Deserialize)]
struct TestCase {
    id: String,
    language: String,
    category: String,
    input: String,
    expected_expr: String,
    description: String,
}

#[derive(Debug, Serialize)]
struct TestResult {
    id: String,
    input: String,
    language: String,
    category: String,
    description: String,

    // Parsing results
    parse_success: bool,
    parse_error: Option<String>,

    // Formatting results
    formatted_latex: Option<String>,
    formatted_wolfram: Option<String>,

    // Validation results
    input_language_format_match: Option<bool>, // Does formatted output in SAME language as input match?
    latex_round_trip: Option<bool>,
    wolfram_round_trip: Option<bool>,
}

fn test_explicit_cases() {
    let parser = Parser::new(ParserConfig {
        enable_implicit_multiplication: false,
    });

    // Read explicit test cases (without implicit multiplication)
    let test_cases_str = read_to_string("./crates/mathhook-core/tests/parsing/cases_explicit.json")
        .expect("Failed to read cases_explicit.json");
    let test_cases: Vec<TestCase> =
        serde_json::from_str(&test_cases_str).expect("Failed to parse cases_explicit.json");

    println!(
        "Testing {} explicit cases (no implicit multiplication)\n",
        test_cases.len()
    );

    let mut results = Vec::new();
    let mut stats = TestStats::default();

    for (idx, case) in test_cases.iter().enumerate() {
        if idx % 50 == 0 {
            println!("Progress: {}/{}", idx, test_cases.len());
        }

        let result = test_single_case(&parser, case);

        // Update stats
        if result.parse_success {
            stats.parse_success += 1;

            if case.language == "latex" {
                stats.latex_parse_success += 1;
                if result.input_language_format_match == Some(true) {
                    stats.latex_format_match_success += 1;
                }
                if result.latex_round_trip == Some(true) {
                    stats.latex_round_trip_success += 1;
                }
            } else if case.language == "wolfram" {
                stats.wolfram_parse_success += 1;
                if result.input_language_format_match == Some(true) {
                    stats.wolfram_format_match_success += 1;
                }
                if result.wolfram_round_trip == Some(true) {
                    stats.wolfram_round_trip_success += 1;
                }
            }
        } else {
            stats.parse_failure += 1;
        }

        results.push(result);
    }

    // Write results
    let output_path = "./crates/mathhook-core/tests/parsing/test_results_explicit.json";
    write(output_path, serde_json::to_string_pretty(&results).unwrap())
        .expect("Failed to write results");

    // Print summary
    println!("\n{}", "=".repeat(70));
    println!("TEST SUMMARY");
    println!("{}", "=".repeat(70));

    let total = test_cases.len();
    println!("\n📊 PARSING:");
    println!("  Total cases: {}", total);
    println!(
        "  ✅ Parsed successfully: {} ({:.1}%)",
        stats.parse_success,
        100.0 * stats.parse_success as f64 / total as f64
    );
    println!(
        "  ❌ Parse failures: {} ({:.1}%)",
        stats.parse_failure,
        100.0 * stats.parse_failure as f64 / total as f64
    );

    let latex_total = test_cases.iter().filter(|c| c.language == "latex").count();
    let wolfram_total = test_cases
        .iter()
        .filter(|c| c.language == "wolfram")
        .count();

    println!("\n📝 LATEX:");
    println!("  Total LaTeX cases: {}", latex_total);
    println!(
        "  ✅ Parsed: {} ({:.1}%)",
        stats.latex_parse_success,
        100.0 * stats.latex_parse_success as f64 / latex_total as f64
    );
    println!(
        "  📋 Format matches input: {} ({:.1}%)",
        stats.latex_format_match_success,
        100.0 * stats.latex_format_match_success as f64 / latex_total as f64
    );
    println!(
        "  🔄 Cross-language round-trip: {} ({:.1}%)",
        stats.latex_round_trip_success,
        100.0 * stats.latex_round_trip_success as f64 / latex_total as f64
    );

    println!("\n🅦 WOLFRAM:");
    println!("  Total Wolfram cases: {}", wolfram_total);
    println!(
        "  ✅ Parsed: {} ({:.1}%)",
        stats.wolfram_parse_success,
        100.0 * stats.wolfram_parse_success as f64 / wolfram_total as f64
    );
    println!(
        "  📋 Format matches input: {} ({:.1}%)",
        stats.wolfram_format_match_success,
        100.0 * stats.wolfram_format_match_success as f64 / wolfram_total as f64
    );
    println!(
        "  🔄 Cross-language round-trip: {} ({:.1}%)",
        stats.wolfram_round_trip_success,
        100.0 * stats.wolfram_round_trip_success as f64 / wolfram_total as f64
    );

    println!("\n💾 Results written to: {}", output_path);
    println!("{}", "=".repeat(70));
}

fn test_single_case(parser: &Parser, case: &TestCase) -> TestResult {
    let parse_result = parser.parse(&case.input);

    match parse_result {
        Ok(expr) => {
            // Successfully parsed - now test formatting
            let formatted_latex = expr.format_as(MathLanguage::LaTeX).ok();
            let formatted_wolfram = expr.format_as(MathLanguage::Wolfram).ok();

            // Test round-trip: Parse → Format → Parse again
            let latex_round_trip = formatted_latex.as_ref().and_then(|latex_str| {
                parser.parse(latex_str).ok().map(|reparsed| {
                    // Compare expressions (should be equal or canonical form)
                    reparsed.to_string() == expr.to_string()
                })
            });

            let wolfram_round_trip = formatted_wolfram.as_ref().and_then(|wolfram_str| {
                parser
                    .parse(wolfram_str)
                    .ok()
                    .map(|reparsed| reparsed.to_string() == expr.to_string())
            });

            // Validate: Does formatted output in SAME language as input match?
            let input_language_format_match = match case.language.as_str() {
                "latex" => latex_round_trip,
                "wolfram" => wolfram_round_trip,
                _ => None,
            };

            TestResult {
                id: case.id.clone(),
                input: case.input.clone(),
                language: case.language.clone(),
                category: case.category.clone(),
                description: case.description.clone(),
                parse_success: true,
                parse_error: None,
                formatted_latex,
                formatted_wolfram,
                input_language_format_match,
                latex_round_trip,
                wolfram_round_trip,
            }
        }
        Err(e) => {
            // Parse failed
            TestResult {
                id: case.id.clone(),
                input: case.input.clone(),
                language: case.language.clone(),
                category: case.category.clone(),
                description: case.description.clone(),
                parse_success: false,
                parse_error: Some(format!("{:?}", e)),
                formatted_latex: None,
                formatted_wolfram: None,
                input_language_format_match: None,
                latex_round_trip: None,
                wolfram_round_trip: None,
            }
        }
    }
}

#[derive(Default)]
struct TestStats {
    parse_success: usize,
    parse_failure: usize,
    latex_parse_success: usize,
    latex_format_match_success: usize, // LaTeX input → LaTeX format matches
    latex_round_trip_success: usize,
    wolfram_parse_success: usize,
    wolfram_format_match_success: usize, // Wolfram input → Wolfram format matches
    wolfram_round_trip_success: usize,
}
