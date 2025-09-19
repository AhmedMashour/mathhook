use mathhook::prelude::*;

fn main() {
    // Test step by step
    println!("Testing: x^2-x-6=9");

    // Try a simpler equation first
    println!("Testing simpler: x^2=4");
    match convenience::solve("x^2=4") {
        Ok(result) => println!("✅ x^2=4 solution: {:?}", result),
        Err(e) => println!("❌ x^2=4 error: {:?}", e),
    }

    // Test what preprocessing does to our equation
    println!("Testing preprocessing on: x^2-x-6=9");
    let test_input = "x^2-x-6=9";

    // Manual preprocessing steps
    let step1 = test_input.replace(" ", "");
    println!("After space removal: '{}'", step1);

    // Test if the issue is in implicit multiplication
    println!("Testing simpler case: x-1=0");
    match convenience::solve("x-1=0") {
        Ok(result) => println!("✅ x-1=0 solution: {:?}", result),
        Err(e) => println!("❌ x-1=0 error: {:?}", e),
    }

    // Step 1: Test parsing
    match Expression::parse_latex("x^2-x-6=9") {
        Ok(parsed) => {
            println!("✅ Parsing successful: {:?}", parsed);

            // Step 2: Test analysis
            match convenience::analyze("x^2-x-6=9") {
                Ok(eq_type) => {
                    println!("✅ Analysis successful: {:?}", eq_type);

                    // Step 3: Test solving with steps to see more detail
                    match convenience::solve_with_steps("x^2-x-6=9") {
                        Ok((result, steps)) => {
                            println!("✅ Solution: {:?}", result);
                            println!("✅ Steps: {:?}", steps);
                        }
                        Err(e) => {
                            println!("❌ Solve error: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Analysis error: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Parsing error: {:?}", e);
        }
    }

    // Expected: x = (1 ± √61)/2 ≈ 4.405, -3.405
    println!("Expected: x = (1 + √61)/2 ≈ 4.405, x = (1 - √61)/2 ≈ -3.405");
}
