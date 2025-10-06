use mathhook_core::educational::enhanced_steps::FormatContext;
use mathhook_core::formatter::ExpressionFormatter;
use mathhook_core::formatter::FormattingContext;
use mathhook_core::parser::config::ParserConfig;
use mathhook_core::parser::Parser;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::fs::write;
fn main() {
    println!("\n=== Fully Integrated Parser Demo ===\n");
    // Show the complete integrated parser in action
    // test_integrated_parser();
    test_json_test_cases();
}

// fn test_integrated_parser() {
//     let parser = Parser::new(ParserConfig {
//         enable_implicit_multiplication: true,
//     });

//     println!("Testing fully integrated parser (Enhanced Lexer + LALRPOP Grammar):");

//     let test_cases = vec![
//         // Cases that should work with implicit multiplication
//         ("3\\beta", "Should parse as: beta"),
//         // ("2x", "Should parse as: 2 * x"),
//         // ("2(x+1)", "Should parse as: 2 * (x + 1)"),
//         // Cases that should work normally
//         // ("x+y", "Should parse as: x + y"),
//         // ("x^2", "Should parse as: x^2"),
//         // ("2*x", "Should parse as: 2 * x (already explicit)"),
//         // Cases that might not work yet (functions)
//         // ("2sin(x)", "Should parse as: 2 * sin(x)"),
//         // ("sin(x)cos(y)", "Should parse as: sin(x) * cos(y)"),
//     ];

//     for (input, description) in test_cases {
//         println!("\nInput: '{}'", input);
//         println!("  Expected: {}", description);

//         match parser.parse(input) {
//             Ok(expr) => {
//                 println!("  Result: ✅ {:?}", expr);
//                 println!("  Display: {}", expr);
//             }
//             Err(e) => {
//                 println!("  Result: ❌ {}", e);
//             }
//         }
//     }
// }

fn test_json_test_cases() {
    let parser = Parser::new(ParserConfig {
        enable_implicit_multiplication: false,
    });

    let test_cases = read_to_string("./crates/mathhook-core/tests/parsing/cases.json")
        .expect("Failed to read cases.json");
    let cases: serde_json::Value = serde_json::from_str(&test_cases).unwrap();

    assert!(cases.is_array());

    let mut responses: Vec<HashMap<String, String>> = vec![];
    for case in cases.as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let expected = case["expected_expr"].as_str().unwrap();
        let result = parser.parse(input);
        match result {
            Ok(expr) => {
                responses.push(HashMap::from([
                    ("input".to_string(), String::from(input)),
                    ("expected".to_string(), String::from(expected)),
                    ("received".to_string(), expr.format().unwrap()),
                ]));
            }
            Err(e) => {
                // responses.push(HashMap::from([
                //     ("input".to_string(), input),
                //     ("expected".to_string(), expected),
                //     ("success".to_string(), "false"),
                // ]));
            }
        }
    }

    let file_path = "./crates/mathhook-core/tests/parsing/output_3.json";
    write(file_path, serde_json::to_string_pretty(&responses).unwrap()).unwrap();

    println!("Output written to {}", file_path);
}
