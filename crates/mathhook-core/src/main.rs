use mathhook_core::parser::{
    integrated_parser::IntegratedMathParser, lalrpop::grammar::mathematical,
    lexer_with_implicit_mul::ImplicitMultiplicationLexer,
};

fn main() {
    println!("=== MathHook Implicit Multiplication Demo ===\n");

    // Test 1: Show the enhanced lexer in action
    test_enhanced_lexer();

    println!("\n=== Fully Integrated Parser Demo ===\n");
    // Test 4: Show the complete integrated parser in action
    test_integrated_parser();
}

fn test_current_parser(value: &str) {
    println!("\nInput: '{}'", value);
    let parser = mathematical::ExpressionParser::new();

    match parser.parse(value) {
        Ok(expr) => println!("✅ Parsed: {:?}", expr),
        Err(e) => println!("❌ Error: {:?}", e),
    }
}
fn test_enhanced_lexer() {
    let test_cases = vec![
        "2x",           // Should insert * between 2 and x
        "xy",           // Should insert * between x and y
        "2sin(x)",      // Should insert * between 2 and sin
        "sin(x)cos(y)", // Should insert * between sin(x) and cos
        "2(x+1)",       // Should insert * between 2 and (
        "x+y",          // Should NOT insert * (different operator)
        "sin(x)",       // Should NOT insert * (function call)
    ];

    println!("Testing enhanced lexer (detects implicit multiplication):");

    for case in test_cases {
        println!("\nInput: '{}'", case);

        // Show original tokens
        println!("  Original: {}", case);

        // Show enhanced tokens
        let mut lexer = ImplicitMultiplicationLexer::new(case);
        let enhanced_tokens: Vec<_> = std::iter::from_fn(|| lexer.next_enhanced_token()).collect();

        // Convert to string format for easy testing
        let enhanced_string = tokens_to_string(&enhanced_tokens);
        println!("  Enhanced: {}", enhanced_string);

        // Show what LALRPOP would receive
        let mut lexer2 = ImplicitMultiplicationLexer::new(case);
        let lalrpop_tokens = lexer2.to_lalrpop_tokens();

        print!("  LALRPOP:  ");
        for (i, (_, token, _)) in lalrpop_tokens.iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{:?}", token);
        }
        println!();

        // Try parsing with LALRPOP (if tokens are compatible)
        if lalrpop_tokens
            .iter()
            .all(|(_, token, _)| is_supported_token(token))
        {
            println!("  Status:   ✅ Compatible with LALRPOP grammar");

            // Test the enhanced string directly with your LALRPOP parser!
            println!("  Testing:  '{}'", enhanced_string);
            let parser = mathematical::ExpressionParser::new();
            match parser.parse(&enhanced_string) {
                Ok(expr) => println!("  Result:   ✅ Parsed successfully: {:?}", expr),
                Err(e) => println!("  Result:   ❌ Parse error: {:?}", e),
            }
        } else {
            println!("  Status:   ⚠️  Contains unsupported tokens");
        }
    }
}

/// Check if a token is supported by your current LALRPOP grammar
fn is_supported_token(token: &mathhook_core::parser::lalrpop::lexer::tokens::Token) -> bool {
    use mathhook_core::parser::lalrpop::lexer::tokens::Token;

    matches!(
        token,
        Token::Plus
            | Token::Minus
            | Token::Star
            | Token::Slash
            | Token::Caret
            | Token::LParen
            | Token::RParen
            | Token::LBracket
            | Token::RBracket
            | Token::LBrace
            | Token::RBrace
            | Token::Comma
            | Token::Equals
            | Token::Number(_)
            | Token::Identifier(_)
            | Token::Pi
            | Token::E
            | Token::ImaginaryUnit
            | Token::Infinity // Add more tokens as supported by your grammar
    )
}

/// Convert enhanced tokens to a string that can be parsed by LALRPOP
fn tokens_to_string(
    tokens: &[(
        usize,
        mathhook_core::parser::lexer_with_implicit_mul::EnhancedToken,
        usize,
    )],
) -> String {
    let mut result = String::new();

    for (i, (_, token, _)) in tokens.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }

        match token {
            mathhook_core::parser::lexer_with_implicit_mul::EnhancedToken::Regular(t) => {
                result.push_str(&token_to_string(t));
            }
            mathhook_core::parser::lexer_with_implicit_mul::EnhancedToken::ImplicitMultiply => {
                result.push('*');
            }
        }
    }

    result
}

/// Convert a single token to its string representation
fn token_to_string(token: &mathhook_core::parser::lalrpop::lexer::tokens::Token) -> String {
    use mathhook_core::parser::lalrpop::lexer::tokens::Token;

    match token {
        Token::Plus => "+".to_string(),
        Token::Minus => "-".to_string(),
        Token::Star => "*".to_string(),
        Token::Slash => "/".to_string(),
        Token::Caret => "^".to_string(),
        Token::Equals => "=".to_string(),
        Token::LParen => "(".to_string(),
        Token::RParen => ")".to_string(),
        Token::LBracket => "[".to_string(),
        Token::RBracket => "]".to_string(),
        Token::LBrace => "{".to_string(),
        Token::RBrace => "}".to_string(),
        Token::Comma => ",".to_string(),
        Token::Exclamation => "!".to_string(),
        Token::Number(n) => n.to_string(),
        Token::Identifier(id) => id.to_string(),
        Token::Pi => "pi".to_string(),
        Token::E => "e".to_string(),
        Token::ImaginaryUnit => "i".to_string(),
        Token::Infinity => "infinity".to_string(),
        _ => format!("{:?}", token), // Fallback for other tokens
    }
}

fn test_integrated_parser() {
    let parser = IntegratedMathParser::new();

    println!("Testing fully integrated parser (Enhanced Lexer + LALRPOP Grammar):");

    let test_cases = vec![
        // Cases that should work with implicit multiplication
        ("2x", "Should parse as: 2 * x"),
        ("xy", "Should parse as: x * y"),
        ("2(x+1)", "Should parse as: 2 * (x + 1)"),
        // Cases that should work normally
        ("x+y", "Should parse as: x + y"),
        ("x^2", "Should parse as: x^2"),
        ("2*x", "Should parse as: 2 * x (already explicit)"),
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

    println!("\n=== Copy-Paste Test Strings ===");
    println!("You can copy these strings and test them manually:");

    let copy_paste_cases = vec!["2x", "xy", "abc", "2(x+1)", "(x+1)y"];

    for case in copy_paste_cases {
        let mut lexer = ImplicitMultiplicationLexer::new(case);
        let tokens = lexer.to_lalrpop_tokens();
        let enhanced_string = tokens_to_string(
            &tokens
                .iter()
                .map(|(s, t, e)| {
                    (
                        *s,
                        mathhook_core::parser::lexer_with_implicit_mul::EnhancedToken::Regular(
                            t.clone(),
                        ),
                        *e,
                    )
                })
                .collect::<Vec<_>>(),
        );

        println!("  '{}' -> '{}'", case, enhanced_string);
    }
}
