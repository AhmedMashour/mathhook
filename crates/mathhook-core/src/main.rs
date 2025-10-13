use mathhook_core::formatter::MathLanguage;
use mathhook_core::parser::config::ParserConfig;
use mathhook_core::parser::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    parse_error_category: Option<String>,

    // Formatting results
    formatted_latex: Option<String>,
    formatted_wolfram: Option<String>,

    // Validation results (strict)
    exact_string_match: Option<bool>, // Does formatted output EXACTLY match input?
    normalized_match: Option<bool>,   // Does it match after normalization (whitespace)?
    semantic_match: Option<bool>,     // Does reparsing yield same expression?
}

fn categorize_parse_error(error: &str) -> String {
    if error.contains("UnrecognizedToken") {
        "UnrecognizedToken".to_string()
    } else if error.contains("UnrecognizedEof") {
        "UnrecognizedEOF".to_string()
    } else if error.contains("ExtraToken") {
        "ExtraToken".to_string()
    } else if error.contains("InvalidToken") {
        "InvalidToken".to_string()
    } else if error.contains("\\text") {
        "MissingTextSupport".to_string()
    } else if error.contains("\\begin") || error.contains("\\end") {
        "MissingEnvironmentSupport".to_string()
    } else if error.contains("dx") {
        "DifferentialTokenization".to_string()
    } else {
        "Other".to_string()
    }
}

fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn test_explicit_cases() {
    let parser = Parser::new(ParserConfig {
        enable_implicit_multiplication: false,
    });

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
    let mut error_categories: HashMap<String, usize> = HashMap::new();

    for (idx, case) in test_cases.iter().enumerate() {
        if idx % 50 == 0 {
            println!("Progress: {}/{}", idx, test_cases.len());
        }

        let result = test_single_case(&parser, case);

        // Count error categories
        if let Some(ref cat) = result.parse_error_category {
            *error_categories.entry(cat.clone()).or_insert(0) += 1;
        }

        // Update stats
        if result.parse_success {
            stats.parse_success += 1;

            if case.language == "latex" {
                stats.latex_parse_success += 1;
                if result.exact_string_match == Some(true) {
                    stats.latex_exact_match += 1;
                }
                if result.normalized_match == Some(true) {
                    stats.latex_normalized_match += 1;
                }
                if result.semantic_match == Some(true) {
                    stats.latex_semantic_match += 1;
                }
            } else if case.language == "wolfram" {
                stats.wolfram_parse_success += 1;
                if result.exact_string_match == Some(true) {
                    stats.wolfram_exact_match += 1;
                }
                if result.normalized_match == Some(true) {
                    stats.wolfram_normalized_match += 1;
                }
                if result.semantic_match == Some(true) {
                    stats.wolfram_semantic_match += 1;
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

    println!("\n📝 LATEX ({} cases):", latex_total);
    println!(
        "  ✅ Parsed: {} ({:.1}%)",
        stats.latex_parse_success,
        100.0 * stats.latex_parse_success as f64 / latex_total as f64
    );
    println!(
        "  🎯 Exact string match: {} ({:.1}%)",
        stats.latex_exact_match,
        100.0 * stats.latex_exact_match as f64 / latex_total as f64
    );
    println!(
        "  📋 Normalized match: {} ({:.1}%)",
        stats.latex_normalized_match,
        100.0 * stats.latex_normalized_match as f64 / latex_total as f64
    );
    println!(
        "  🔄 Semantic match: {} ({:.1}%)",
        stats.latex_semantic_match,
        100.0 * stats.latex_semantic_match as f64 / latex_total as f64
    );

    println!("\n🅦 WOLFRAM ({} cases):", wolfram_total);
    println!(
        "  ✅ Parsed: {} ({:.1}%)",
        stats.wolfram_parse_success,
        100.0 * stats.wolfram_parse_success as f64 / wolfram_total as f64
    );
    println!(
        "  🎯 Exact string match: {} ({:.1}%)",
        stats.wolfram_exact_match,
        100.0 * stats.wolfram_exact_match as f64 / wolfram_total as f64
    );
    println!(
        "  📋 Normalized match: {} ({:.1}%)",
        stats.wolfram_normalized_match,
        100.0 * stats.wolfram_normalized_match as f64 / wolfram_total as f64
    );
    println!(
        "  🔄 Semantic match: {} ({:.1}%)",
        stats.wolfram_semantic_match,
        100.0 * stats.wolfram_semantic_match as f64 / wolfram_total as f64
    );

    println!("\n❌ ERROR CATEGORIES:");
    let mut categories: Vec<_> = error_categories.iter().collect();
    categories.sort_by_key(|(_, count)| std::cmp::Reverse(**count));
    for (category, count) in categories.iter().take(10) {
        println!("  {} : {} cases", category, count);
    }

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

            // Get the formatted output for the SAME language as input
            let formatted_same_lang = match case.language.as_str() {
                "latex" => formatted_latex.as_ref(),
                "wolfram" => formatted_wolfram.as_ref(),
                _ => None,
            };

            // Check 1: Exact string match
            let exact_string_match = formatted_same_lang.map(|formatted| formatted == &case.input);

            // Check 2: Normalized match (ignoring whitespace differences)
            let normalized_match = formatted_same_lang.map(|formatted| {
                normalize_whitespace(formatted) == normalize_whitespace(&case.input)
            });

            // Check 3: Semantic match (round-trip parsing)
            let semantic_match = formatted_same_lang.and_then(|formatted| {
                parser
                    .parse(formatted)
                    .ok()
                    .map(|reparsed| reparsed.to_string() == expr.to_string())
            });

            TestResult {
                id: case.id.clone(),
                input: case.input.clone(),
                language: case.language.clone(),
                category: case.category.clone(),
                description: case.description.clone(),
                parse_success: true,
                parse_error: None,
                parse_error_category: None,
                formatted_latex,
                formatted_wolfram,
                exact_string_match,
                normalized_match,
                semantic_match,
            }
        }
        Err(e) => {
            let error_str = format!("{:?}", e);
            let error_category = categorize_parse_error(&error_str);

            TestResult {
                id: case.id.clone(),
                input: case.input.clone(),
                language: case.language.clone(),
                category: case.category.clone(),
                description: case.description.clone(),
                parse_success: false,
                parse_error: Some(error_str),
                parse_error_category: Some(error_category),
                formatted_latex: None,
                formatted_wolfram: None,
                exact_string_match: None,
                normalized_match: None,
                semantic_match: None,
            }
        }
    }
}

#[derive(Default)]
struct TestStats {
    parse_success: usize,
    parse_failure: usize,
    latex_parse_success: usize,
    latex_exact_match: usize,      // Exact string match
    latex_normalized_match: usize, // Match after whitespace normalization
    latex_semantic_match: usize,   // Round-trip semantic match
    wolfram_parse_success: usize,
    wolfram_exact_match: usize,
    wolfram_normalized_match: usize,
    wolfram_semantic_match: usize,
}
