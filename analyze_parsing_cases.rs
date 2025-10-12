use mathhook_core::parser::Parser;
use mathhook_core::Expression;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct TestCase {
    id: String,
    language: String,
    category: String,
    input: String,
    expected_expr: String,
    description: String,
}

#[derive(Debug)]
struct AnalysisResult {
    total: usize,
    success: usize,
    failed: usize,
    failures_by_category: HashMap<String, Vec<String>>,
    failures_by_language: HashMap<String, usize>,
}

fn main() {
    let content = fs::read_to_string("crates/mathhook-core/tests/parsing/cases.json")
        .expect("Failed to read cases.json");

    let cases: Vec<TestCase> = serde_json::from_str(&content)
        .expect("Failed to parse JSON");

    println!("Total test cases: {}", cases.len());
    println!("\n========== PARSING ANALYSIS ==========\n");

    let mut result = AnalysisResult {
        total: cases.len(),
        success: 0,
        failed: 0,
        failures_by_category: HashMap::new(),
        failures_by_language: HashMap::new(),
    };

    for case in &cases {
        let parser = match case.language.as_str() {
            "latex" => Parser::latex(),
            "wolfram" => Parser::wolfram(),
            _ => Parser::standard(),
        };

        match parser.parse(&case.input) {
            Ok(_expr) => {
                result.success += 1;
            }
            Err(e) => {
                result.failed += 1;

                // Track by category
                result.failures_by_category
                    .entry(case.category.clone())
                    .or_insert_with(Vec::new)
                    .push(case.id.clone());

                // Track by language
                *result.failures_by_language
                    .entry(case.language.clone())
                    .or_insert(0) += 1;

                println!("FAILED: {} [{}] [{}]", case.id, case.language, case.category);
                println!("  Input: {}", case.input);
                println!("  Error: {:?}", e);
                println!();
            }
        }
    }

    println!("\n========== SUMMARY ==========");
    println!("Total: {}", result.total);
    println!("Success: {} ({:.1}%)", result.success, (result.success as f64 / result.total as f64) * 100.0);
    println!("Failed: {} ({:.1}%)", result.failed, (result.failed as f64 / result.total as f64) * 100.0);

    println!("\n========== FAILURES BY LANGUAGE ==========");
    for (lang, count) in &result.failures_by_language {
        println!("{}: {} failures", lang, count);
    }

    println!("\n========== FAILURES BY CATEGORY ==========");
    let mut sorted_categories: Vec<_> = result.failures_by_category.iter().collect();
    sorted_categories.sort_by_key(|(_, v)| -(v.len() as i32));

    for (category, ids) in sorted_categories.iter().take(20) {
        println!("{}: {} failures", category, ids.len());
    }

    println!("\n========== TOP 10 MOST PROBLEMATIC CATEGORIES ==========");
    for (category, ids) in sorted_categories.iter().take(10) {
        println!("\n{}:", category);
        for id in ids.iter().take(3) {
            if let Some(case) = cases.iter().find(|c| &c.id == id) {
                println!("  - {} [{}]: {}", case.id, case.language, case.input);
            }
        }
        if ids.len() > 3 {
            println!("  ... and {} more", ids.len() - 3);
        }
    }
}
