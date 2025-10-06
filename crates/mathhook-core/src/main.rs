use mathhook_core::parser::config::ParserConfig;
use mathhook_core::parser::Parser;
fn main() {
    println!("\n=== Fully Integrated Parser Demo ===\n");
    // Show the complete integrated parser in action
    test_integrated_parser();
}

fn test_integrated_parser() {
    let parser = Parser::new(ParserConfig {
        enable_implicit_multiplication: true,
    });

    println!("Testing fully integrated parser (Enhanced Lexer + LALRPOP Grammar):");

    let test_cases = vec![
        // Cases that should work with implicit multiplication
        ("3\\beta", "Should parse as: beta"),
        // ("2x", "Should parse as: 2 * x"),
        // ("2(x+1)", "Should parse as: 2 * (x + 1)"),
        // Cases that should work normally
        // ("x+y", "Should parse as: x + y"),
        // ("x^2", "Should parse as: x^2"),
        // ("2*x", "Should parse as: 2 * x (already explicit)"),
        // Cases that might not work yet (functions)
        // ("2sin(x)", "Should parse as: 2 * sin(x)"),
        // ("sin(x)cos(y)", "Should parse as: sin(x) * cos(y)"),
    ];

    for (input, description) in test_cases {
        println!("\nInput: '{}'", input);
        println!("  Expected: {}", description);

        match parser.parse(input) {
            Ok(expr) => {
                println!("  Result: ✅ {:?}", expr);
                println!("  Display: {}", expr);
            }
            Err(e) => {
                println!("  Result: ❌ {}", e);
            }
        }
    }
}
