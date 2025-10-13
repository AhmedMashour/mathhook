use mathhook_core::parser::{config::ParserConfig, Parser};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    id: String,
    language: String,
    category: String,
    input: String,
    description: String,
}

#[test]
fn analyze_all_parsing_cases() {
    let content =
        fs::read_to_string("tests/parsing/cases.json").expect("Failed to read cases.json");

    let cases: Vec<TestCase> = serde_json::from_str(&content).expect("Failed to parse JSON");

    println!("\n========== PARSING ANALYSIS ==========");
    println!("Total test cases: {}\n", cases.len());

    let mut total = 0;
    let mut success = 0;
    let mut failed = 0;
    let mut failures_by_category: HashMap<String, Vec<String>> = HashMap::new();
    let mut failures_by_language: HashMap<String, usize> = HashMap::new();

    for case in &cases {
        total += 1;

        // Create parser with implicit multiplication enabled
        let parser = Parser::new(ParserConfig {
            enable_implicit_multiplication: true,
        });

        match parser.parse(&case.input) {
            Ok(_expr) => {
                success += 1;
            }
            Err(e) => {
                failed += 1;

                failures_by_category
                    .entry(case.category.clone())
                    .or_insert_with(Vec::new)
                    .push(case.id.clone());

                *failures_by_language
                    .entry(case.language.clone())
                    .or_insert(0) += 1;

                println!(
                    "❌ FAILED: {} [{}] [{}]",
                    case.id, case.language, case.category
                );
                println!("   Input: {}", case.input);
                println!("   Error: {:?}\n", e);
            }
        }
    }

    println!("\n========== SUMMARY ==========");
    println!("Total: {}", total);
    println!(
        "Success: {} ({:.1}%)",
        success,
        (success as f64 / total as f64) * 100.0
    );
    println!(
        "Failed: {} ({:.1}%)",
        failed,
        (failed as f64 / total as f64) * 100.0
    );

    println!("\n========== FAILURES BY LANGUAGE ==========");
    for (lang, count) in &failures_by_language {
        println!("{}: {} failures", lang, count);
    }

    println!("\n========== FAILURES BY CATEGORY (Top 20) ==========");
    let mut sorted_categories: Vec<_> = failures_by_category.iter().collect();
    sorted_categories.sort_by_key(|(_, v)| -(v.len() as i32));

    for (category, ids) in sorted_categories.iter().take(20) {
        println!("{}: {} failures", category, ids.len());
    }

    println!("\n========== TOP 10 MOST PROBLEMATIC CATEGORIES WITH EXAMPLES ==========");
    for (category, ids) in sorted_categories.iter().take(10) {
        println!("\n{}:", category);
        for id in ids.iter().take(3) {
            if let Some(case) = cases.iter().find(|c| &c.id == id) {
                println!("  - [{}] {}: \"{}\"", case.language, case.id, case.input);
            }
        }
        if ids.len() > 3 {
            println!("  ... and {} more", ids.len() - 3);
        }
    }

    // Don't actually fail the test - just report
    println!("\n========== Analysis complete! ==========\n");
}
